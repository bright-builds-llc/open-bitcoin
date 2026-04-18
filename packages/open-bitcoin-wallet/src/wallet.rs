use core::cmp::Ordering;

use secp256k1::{Message, Secp256k1};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ScriptExecutionData, SigHashType, SigVersion, TransactionInputContext,
    TransactionValidationContext, legacy_sighash, segwit_v0_sighash, taproot_sighash,
};
use open_bitcoin_mempool::FeeRate;
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionOutput,
};

use crate::WalletError;
use crate::address::{
    Address, AddressNetwork, PrivateKey, public_key_bytes, push_data,
    taproot_output_key_from_private_key,
};
use crate::descriptor::{DescriptorRecord, DescriptorRole, SingleKeyDescriptor};

mod build;
mod scan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletSnapshot {
    pub network: AddressNetwork,
    pub descriptors: Vec<DescriptorRecord>,
    pub utxos: Vec<WalletUtxo>,
    pub next_descriptor_id: u32,
    pub maybe_tip_height: Option<u32>,
    pub maybe_tip_median_time_past: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletUtxo {
    pub descriptor_id: u32,
    pub outpoint: OutPoint,
    pub output: TransactionOutput,
    pub created_height: u32,
    pub created_median_time_past: i64,
    pub is_coinbase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletBalance {
    pub total: Amount,
    pub spendable: Amount,
    pub immature: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipient {
    pub script_pubkey: ScriptBuf,
    pub value: Amount,
}

impl Recipient {
    pub fn from_address(address: &Address, value: Amount) -> Self {
        Self {
            script_pubkey: address.script_pubkey.clone(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildRequest {
    pub recipients: Vec<Recipient>,
    pub fee_rate: FeeRate,
    pub maybe_change_descriptor_id: Option<u32>,
    pub maybe_lock_time: Option<u32>,
    pub enable_rbf: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltTransaction {
    pub transaction: Transaction,
    pub selected_inputs: Vec<WalletUtxo>,
    pub fee: Amount,
    pub change_output_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallet {
    network: AddressNetwork,
    descriptors: Vec<DescriptorRecord>,
    utxos: Vec<WalletUtxo>,
    next_descriptor_id: u32,
    maybe_tip_height: Option<u32>,
    maybe_tip_median_time_past: Option<i64>,
}

impl Wallet {
    pub fn new(network: AddressNetwork) -> Self {
        Self {
            network,
            descriptors: Vec::new(),
            utxos: Vec::new(),
            next_descriptor_id: 0,
            maybe_tip_height: None,
            maybe_tip_median_time_past: None,
        }
    }

    pub fn from_snapshot(snapshot: WalletSnapshot) -> Self {
        Self {
            network: snapshot.network,
            descriptors: snapshot.descriptors,
            utxos: snapshot.utxos,
            next_descriptor_id: snapshot.next_descriptor_id,
            maybe_tip_height: snapshot.maybe_tip_height,
            maybe_tip_median_time_past: snapshot.maybe_tip_median_time_past,
        }
    }

    pub fn snapshot(&self) -> WalletSnapshot {
        WalletSnapshot {
            network: self.network,
            descriptors: self.descriptors.clone(),
            utxos: self.utxos.clone(),
            next_descriptor_id: self.next_descriptor_id,
            maybe_tip_height: self.maybe_tip_height,
            maybe_tip_median_time_past: self.maybe_tip_median_time_past,
        }
    }

    pub fn network(&self) -> AddressNetwork {
        self.network
    }

    pub fn descriptors(&self) -> &[DescriptorRecord] {
        &self.descriptors
    }

    pub fn utxos(&self) -> &[WalletUtxo] {
        &self.utxos
    }

    pub fn import_descriptor(
        &mut self,
        label: impl Into<String>,
        role: DescriptorRole,
        descriptor_text: &str,
    ) -> Result<u32, WalletError> {
        let label = label.into();
        if self.descriptors.iter().any(|record| record.label == label) {
            return Err(WalletError::DuplicateLabel(label));
        }

        let descriptor = SingleKeyDescriptor::parse(descriptor_text, self.network)?;
        let id = self.next_descriptor_id;
        self.next_descriptor_id += 1;
        self.descriptors.push(DescriptorRecord {
            id,
            label,
            role,
            original_text: descriptor_text.to_string(),
            descriptor,
        });
        self.descriptors.sort_by_key(|record| record.id);
        Ok(id)
    }

    pub fn address_for_descriptor(&self, descriptor_id: u32) -> Result<Address, WalletError> {
        let record = self
            .descriptor(descriptor_id)
            .ok_or(WalletError::UnknownDescriptor(descriptor_id))?;
        record.descriptor.address(self.network)
    }

    pub fn default_receive_address(&self) -> Result<Address, WalletError> {
        let Some(record) = self
            .descriptors
            .iter()
            .find(|descriptor| descriptor.role == DescriptorRole::External)
        else {
            return Err(WalletError::ChangeDescriptorRequired);
        };
        record.descriptor.address(self.network)
    }

    pub fn default_change_address(&self) -> Result<Address, WalletError> {
        let Some(record) = self
            .descriptors
            .iter()
            .find(|descriptor| descriptor.role == DescriptorRole::Internal)
        else {
            return Err(WalletError::ChangeDescriptorRequired);
        };
        record.descriptor.address(self.network)
    }

    pub fn rescan_chainstate(&mut self, snapshot: &ChainstateSnapshot) -> Result<(), WalletError> {
        scan::rescan_chainstate(self, snapshot)
    }

    pub fn balance(&self, coinbase_maturity: u32) -> Result<WalletBalance, WalletError> {
        scan::balance(self, coinbase_maturity)
    }

    pub fn build_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, WalletError> {
        build::build_transaction(self, request, coinbase_maturity)
    }

    pub fn sign_transaction(&self, built: &BuiltTransaction) -> Result<Transaction, WalletError> {
        let mut transaction = built.transaction.clone();
        let input_contexts = self.input_contexts_for(built)?;
        let validation_context = self.validation_context(&input_contexts);
        let precomputed = validation_context.precompute(&transaction)?;

        for (input_index, utxo) in built.selected_inputs.iter().enumerate() {
            let descriptor = self
                .descriptor(utxo.descriptor_id)
                .ok_or(WalletError::UnknownDescriptor(utxo.descriptor_id))?;
            match &descriptor.descriptor {
                SingleKeyDescriptor::Pkh(key) => {
                    let private_key = key.private_key().ok_or_else(|| {
                        WalletError::MissingSigningKey(descriptor.original_text.clone())
                    })?;
                    let script_code = descriptor.descriptor.script_pubkey()?;
                    let sighash =
                        legacy_sighash(&script_code, &transaction, input_index, SigHashType::ALL);
                    let signature = sign_ecdsa_low_s(private_key, &sighash.to_byte_array())?;
                    let public_key = key.public_key();
                    let public_key_bytes = public_key_bytes(&public_key, key.is_compressed());
                    let script_sig =
                        push_script(&[signature.as_slice(), public_key_bytes.as_slice()])?;
                    transaction.inputs[input_index].script_sig = script_sig;
                }
                SingleKeyDescriptor::ShWpkh(key) | SingleKeyDescriptor::Wpkh(key) => {
                    let private_key = key.private_key().ok_or_else(|| {
                        WalletError::MissingSigningKey(descriptor.original_text.clone())
                    })?;
                    let public_key = key.public_key();
                    let script_code = p2pkh_script(&public_key)?;
                    let input_context = input_contexts[input_index].clone();
                    let sighash = segwit_v0_sighash(
                        &script_code,
                        &transaction,
                        input_index,
                        &input_context,
                        SigHashType::ALL,
                        &precomputed,
                    );
                    let signature = sign_ecdsa_low_s(private_key, &sighash.to_byte_array())?;
                    let public_key_bytes = public_key_bytes(&public_key, key.is_compressed());
                    if let Some(redeem_script) = descriptor.descriptor.redeem_script()? {
                        transaction.inputs[input_index].script_sig =
                            push_script(&[redeem_script.as_bytes()])?;
                    }
                    transaction.inputs[input_index].witness =
                        ScriptWitness::new(vec![signature, public_key_bytes]);
                }
                SingleKeyDescriptor::Tr(key) => {
                    let private_key = key.private_key().ok_or_else(|| {
                        WalletError::MissingSigningKey(descriptor.original_text.clone())
                    })?;
                    let secp = Secp256k1::new();
                    let (keypair, _output_key) = taproot_output_key_from_private_key(private_key)?;
                    let sighash = taproot_sighash(
                        &ScriptExecutionData::default(),
                        &transaction,
                        input_index,
                        SigHashType::DEFAULT,
                        SigVersion::Taproot,
                        &validation_context,
                    )
                    .expect("taproot key-path sighash should exist for built transactions");
                    let message = Message::from_digest(sighash.to_byte_array());
                    let signature = secp.sign_schnorr_no_aux_rand(message.as_ref(), &keypair);
                    transaction.inputs[input_index].witness =
                        ScriptWitness::new(vec![signature.as_ref().to_vec()]);
                }
            }
        }

        Ok(transaction)
    }

    pub fn build_and_sign(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, WalletError> {
        let built = self.build_transaction(request, coinbase_maturity)?;
        let signed = self.sign_transaction(&built)?;

        Ok(BuiltTransaction {
            transaction: signed,
            ..built
        })
    }

    pub fn input_contexts_for(
        &self,
        built: &BuiltTransaction,
    ) -> Result<Vec<TransactionInputContext>, WalletError> {
        build::input_contexts_for(self, built)
    }

    fn validation_context(
        &self,
        input_contexts: &[TransactionInputContext],
    ) -> TransactionValidationContext {
        build::validation_context(self, input_contexts)
    }

    fn descriptor(&self, descriptor_id: u32) -> Option<&DescriptorRecord> {
        self.descriptors
            .iter()
            .find(|descriptor| descriptor.id == descriptor_id)
    }

    #[cfg(test)]
    fn estimate_vsize(
        &self,
        selected_inputs: &[WalletUtxo],
        recipients: &[Recipient],
        maybe_change_output: Option<&TransactionOutput>,
        request: &BuildRequest,
    ) -> Result<usize, WalletError> {
        build::estimate_vsize(
            self,
            selected_inputs,
            recipients,
            maybe_change_output,
            request,
        )
    }

    fn is_spendable(&self, utxo: &WalletUtxo, coinbase_maturity: u32) -> bool {
        scan::is_spendable(self, utxo, coinbase_maturity)
    }

    fn spend_height(&self) -> u32 {
        scan::spend_height(self)
    }
}

fn amount_from_sats(sats: i64) -> Result<Amount, WalletError> {
    build::amount_from_sats(sats)
}

fn compare_wallet_utxos(left: &WalletUtxo, right: &WalletUtxo) -> Ordering {
    build::compare_wallet_utxos(left, right)
}

fn sign_ecdsa_low_s(private_key: &PrivateKey, digest: &[u8; 32]) -> Result<Vec<u8>, WalletError> {
    let secp = Secp256k1::new();
    let message = Message::from_digest(*digest);
    let mut signature = secp.sign_ecdsa(message, private_key.secret_key());
    signature.normalize_s();
    let mut encoded = signature.serialize_der().as_ref().to_vec();
    encoded.push(SigHashType::ALL.raw() as u8);
    Ok(encoded)
}

fn push_script(pushes: &[&[u8]]) -> Result<ScriptBuf, WalletError> {
    let mut bytes = Vec::new();
    for push in pushes {
        bytes.extend_from_slice(&push_data(push));
    }
    Ok(ScriptBuf::from_bytes(bytes)?)
}

fn p2pkh_script(public_key: &secp256k1::PublicKey) -> Result<ScriptBuf, WalletError> {
    crate::address::p2pkh_script(public_key)
}

fn standard_wallet_verify_flags() -> open_bitcoin_consensus::ScriptVerifyFlags {
    open_bitcoin_consensus::ScriptVerifyFlags::P2SH
        | open_bitcoin_consensus::ScriptVerifyFlags::STRICTENC
        | open_bitcoin_consensus::ScriptVerifyFlags::DERSIG
        | open_bitcoin_consensus::ScriptVerifyFlags::LOW_S
        | open_bitcoin_consensus::ScriptVerifyFlags::NULLDUMMY
        | open_bitcoin_consensus::ScriptVerifyFlags::SIGPUSHONLY
        | open_bitcoin_consensus::ScriptVerifyFlags::MINIMALDATA
        | open_bitcoin_consensus::ScriptVerifyFlags::CLEANSTACK
        | open_bitcoin_consensus::ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | open_bitcoin_consensus::ScriptVerifyFlags::CHECKSEQUENCEVERIFY
        | open_bitcoin_consensus::ScriptVerifyFlags::WITNESS
        | open_bitcoin_consensus::ScriptVerifyFlags::MINIMALIF
        | open_bitcoin_consensus::ScriptVerifyFlags::NULLFAIL
        | open_bitcoin_consensus::ScriptVerifyFlags::WITNESS_PUBKEYTYPE
        | open_bitcoin_consensus::ScriptVerifyFlags::TAPROOT
}

#[cfg(test)]
mod tests;

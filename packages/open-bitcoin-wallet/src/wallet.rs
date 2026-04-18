use core::cmp::Ordering;

use secp256k1::{Message, Secp256k1};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptExecutionData, SigHashType, SigVersion, TransactionInputContext,
    TransactionValidationContext, legacy_sighash, segwit_v0_sighash, taproot_sighash,
};
use open_bitcoin_mempool::{FeeRate, dust_threshold_sats, transaction_weight_and_virtual_size};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

use crate::WalletError;
use crate::address::{
    Address, AddressNetwork, PrivateKey, public_key_bytes, push_data,
    taproot_output_key_from_private_key,
};
use crate::descriptor::{DescriptorRecord, DescriptorRole, SingleKeyDescriptor};

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
        let descriptor_scripts = self
            .descriptors
            .iter()
            .map(|record| Ok((record.id, record.descriptor.script_pubkey()?)))
            .collect::<Result<Vec<_>, WalletError>>()?;

        let mut utxos = Vec::new();
        for (outpoint, coin) in &snapshot.utxos {
            let Some((descriptor_id, _script_pubkey)) = descriptor_scripts
                .iter()
                .find(|(_, script_pubkey)| *script_pubkey == coin.output.script_pubkey)
            else {
                continue;
            };
            utxos.push(WalletUtxo {
                descriptor_id: *descriptor_id,
                outpoint: outpoint.clone(),
                output: coin.output.clone(),
                created_height: coin.created_height,
                created_median_time_past: coin.created_median_time_past,
                is_coinbase: coin.is_coinbase,
            });
        }

        utxos.sort_by(compare_wallet_utxos);
        self.utxos = utxos;
        self.maybe_tip_height = snapshot.tip().map(|tip| tip.height);
        self.maybe_tip_median_time_past = snapshot.tip().map(|tip| tip.median_time_past);
        Ok(())
    }

    pub fn balance(&self, coinbase_maturity: u32) -> Result<WalletBalance, WalletError> {
        let spend_height = self.spend_height();
        let mut total_sats = 0_i64;
        let mut spendable_sats = 0_i64;
        let mut immature_sats = 0_i64;

        for utxo in &self.utxos {
            let value = utxo.output.value.to_sats();
            total_sats += value;
            if utxo.is_coinbase
                && spend_height < utxo.created_height.saturating_add(coinbase_maturity)
            {
                immature_sats += value;
            } else {
                spendable_sats += value;
            }
        }

        Ok(WalletBalance {
            total: amount_from_sats(total_sats)?,
            spendable: amount_from_sats(spendable_sats)?,
            immature: amount_from_sats(immature_sats)?,
        })
    }

    pub fn build_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, WalletError> {
        if request.recipients.is_empty() {
            return Err(WalletError::NoRecipients);
        }

        let mut spendable = self
            .utxos
            .iter()
            .filter(|utxo| self.is_spendable(utxo, coinbase_maturity))
            .cloned()
            .collect::<Vec<_>>();
        if spendable.is_empty() {
            return Err(WalletError::NoSpendableCoins);
        }
        spendable
            .sort_by(|left, right| compare_effective_value(self, request.fee_rate, left, right));

        let recipients_total = request
            .recipients
            .iter()
            .fold(0_i64, |sum, recipient| sum + recipient.value.to_sats());
        let maybe_change_descriptor =
            self.resolve_change_descriptor(request.maybe_change_descriptor_id);
        let maybe_change_script = maybe_change_descriptor
            .map(|descriptor| descriptor.descriptor.script_pubkey())
            .transpose()?;

        let mut selected = Vec::new();
        let mut available_sats = 0_i64;
        for utxo in spendable {
            available_sats += utxo.output.value.to_sats();
            selected.push(utxo);

            let no_change_vsize =
                self.estimate_vsize(&selected, &request.recipients, None, request)?;
            let no_change_fee = request.fee_rate.fee_for_virtual_size(no_change_vsize);
            if available_sats < recipients_total + no_change_fee {
                continue;
            }

            let maybe_change_output = if let Some(change_script) = &maybe_change_script {
                let placeholder = TransactionOutput {
                    value: amount_from_sats(1)?,
                    script_pubkey: change_script.clone(),
                };
                let with_change_vsize_result = self.estimate_change_vsize(
                    &selected,
                    &request.recipients,
                    &placeholder,
                    request,
                );
                let with_change_vsize = with_change_vsize_result?;
                let change_fee = request.fee_rate.fee_for_virtual_size(with_change_vsize);
                let change_sats = available_sats - recipients_total - change_fee;
                if change_sats > 0 {
                    let candidate_output = TransactionOutput {
                        value: amount_from_sats(change_sats)?,
                        script_pubkey: change_script.clone(),
                    };
                    if change_sats > dust_threshold_sats(&candidate_output) {
                        Some(candidate_output)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let mut outputs = request
                .recipients
                .iter()
                .map(|recipient| TransactionOutput {
                    value: recipient.value,
                    script_pubkey: recipient.script_pubkey.clone(),
                })
                .collect::<Vec<_>>();
            let change_output_index = if let Some(change_output) = maybe_change_output.clone() {
                outputs.push(change_output);
                Some(outputs.len() - 1)
            } else {
                None
            };

            let transaction = Transaction {
                version: 2,
                inputs: selected
                    .iter()
                    .map(|utxo| TransactionInput {
                        previous_output: utxo.outpoint.clone(),
                        script_sig: ScriptBuf::default(),
                        sequence: if request.enable_rbf {
                            TransactionInput::MAX_SEQUENCE_NONFINAL
                        } else {
                            TransactionInput::SEQUENCE_FINAL
                        },
                        witness: ScriptWitness::default(),
                    })
                    .collect(),
                outputs,
                lock_time: request
                    .maybe_lock_time
                    .unwrap_or_else(|| self.maybe_tip_height.unwrap_or(0)),
            };
            let inputs_total = selected
                .iter()
                .fold(0_i64, |sum, utxo| sum + utxo.output.value.to_sats());
            let outputs_total = transaction
                .outputs
                .iter()
                .fold(0_i64, |sum, output| sum + output.value.to_sats());
            let fee_sats = inputs_total - outputs_total;
            if change_output_index.is_none()
                && maybe_change_script.is_none()
                && fee_sats > no_change_fee
            {
                return Err(WalletError::ChangeDescriptorRequired);
            }

            return Ok(BuiltTransaction {
                transaction,
                selected_inputs: selected,
                fee: amount_from_sats(fee_sats)?,
                change_output_index,
            });
        }

        Err(WalletError::InsufficientFunds {
            needed_sats: recipients_total,
            available_sats,
        })
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
        built
            .selected_inputs
            .iter()
            .map(|utxo| {
                Ok(TransactionInputContext {
                    spent_output: open_bitcoin_consensus::SpentOutput {
                        value: utxo.output.value,
                        script_pubkey: utxo.output.script_pubkey.clone(),
                        is_coinbase: utxo.is_coinbase,
                    },
                    created_height: utxo.created_height,
                    created_median_time_past: utxo.created_median_time_past,
                })
            })
            .collect()
    }

    fn validation_context(
        &self,
        input_contexts: &[TransactionInputContext],
    ) -> TransactionValidationContext {
        TransactionValidationContext {
            inputs: input_contexts.to_vec(),
            spend_height: self.spend_height(),
            block_time: self.maybe_tip_median_time_past.unwrap_or(0),
            median_time_past: self.maybe_tip_median_time_past.unwrap_or(0),
            verify_flags: standard_wallet_verify_flags(),
            consensus_params: ConsensusParams::default(),
        }
    }

    fn descriptor(&self, descriptor_id: u32) -> Option<&DescriptorRecord> {
        self.descriptors
            .iter()
            .find(|descriptor| descriptor.id == descriptor_id)
    }

    fn resolve_change_descriptor(
        &self,
        maybe_descriptor_id: Option<u32>,
    ) -> Option<&DescriptorRecord> {
        maybe_descriptor_id
            .and_then(|descriptor_id| self.descriptor(descriptor_id))
            .or_else(|| {
                self.descriptors
                    .iter()
                    .find(|descriptor| descriptor.role == DescriptorRole::Internal)
            })
    }

    fn estimate_vsize(
        &self,
        selected_inputs: &[WalletUtxo],
        recipients: &[Recipient],
        maybe_change_output: Option<&TransactionOutput>,
        request: &BuildRequest,
    ) -> Result<usize, WalletError> {
        let outputs = recipients
            .iter()
            .map(|recipient| TransactionOutput {
                value: recipient.value,
                script_pubkey: recipient.script_pubkey.clone(),
            })
            .chain(maybe_change_output.into_iter().cloned())
            .collect::<Vec<_>>();
        let transaction = Transaction {
            version: 2,
            inputs: selected_inputs
                .iter()
                .map(|utxo| self.placeholder_input(utxo, request.enable_rbf))
                .collect::<Result<Vec<_>, WalletError>>()?,
            outputs,
            lock_time: request
                .maybe_lock_time
                .unwrap_or_else(|| self.maybe_tip_height.unwrap_or(0)),
        };

        Ok(transaction_weight_and_virtual_size(&transaction).1)
    }

    fn estimate_change_vsize(
        &self,
        selected_inputs: &[WalletUtxo],
        recipients: &[Recipient],
        change_output: &TransactionOutput,
        request: &BuildRequest,
    ) -> Result<usize, WalletError> {
        self.estimate_vsize(selected_inputs, recipients, Some(change_output), request)
    }

    fn placeholder_input(
        &self,
        utxo: &WalletUtxo,
        enable_rbf: bool,
    ) -> Result<TransactionInput, WalletError> {
        let descriptor = self
            .descriptor(utxo.descriptor_id)
            .ok_or(WalletError::UnknownDescriptor(utxo.descriptor_id))?;
        let sequence = if enable_rbf {
            TransactionInput::MAX_SEQUENCE_NONFINAL
        } else {
            TransactionInput::SEQUENCE_FINAL
        };

        let (script_sig, witness) = match &descriptor.descriptor {
            SingleKeyDescriptor::Pkh(key) => {
                let pubkey_len = if key.is_compressed() { 33 } else { 65 };
                let signature = vec![0_u8; 73];
                let public_key = vec![0_u8; pubkey_len];
                (
                    push_script(&[signature.as_slice(), public_key.as_slice()])?,
                    ScriptWitness::default(),
                )
            }
            SingleKeyDescriptor::ShWpkh(_) => {
                let redeem_script = descriptor
                    .descriptor
                    .redeem_script()?
                    .expect("nested segwit always has redeem script");
                (
                    push_script(&[redeem_script.as_bytes()])?,
                    ScriptWitness::new(vec![vec![0_u8; 73], vec![0_u8; 33]]),
                )
            }
            SingleKeyDescriptor::Wpkh(_) => (
                ScriptBuf::default(),
                ScriptWitness::new(vec![vec![0_u8; 73], vec![0_u8; 33]]),
            ),
            SingleKeyDescriptor::Tr(_) => (
                ScriptBuf::default(),
                ScriptWitness::new(vec![vec![0_u8; 64]]),
            ),
        };

        Ok(TransactionInput {
            previous_output: utxo.outpoint.clone(),
            script_sig,
            sequence,
            witness,
        })
    }

    fn is_spendable(&self, utxo: &WalletUtxo, coinbase_maturity: u32) -> bool {
        let Some(descriptor) = self.descriptor(utxo.descriptor_id) else {
            return false;
        };
        if !descriptor.descriptor.can_sign() {
            return false;
        }
        if utxo.is_coinbase
            && self.spend_height() < utxo.created_height.saturating_add(coinbase_maturity)
        {
            return false;
        }

        true
    }

    fn spend_height(&self) -> u32 {
        self.maybe_tip_height
            .map_or(0, |height| height.saturating_add(1))
    }
}

fn amount_from_sats(sats: i64) -> Result<Amount, WalletError> {
    Ok(Amount::from_sats(sats)?)
}

fn compare_wallet_utxos(left: &WalletUtxo, right: &WalletUtxo) -> Ordering {
    left.created_height
        .cmp(&right.created_height)
        .then_with(|| {
            left.outpoint
                .txid
                .as_bytes()
                .cmp(right.outpoint.txid.as_bytes())
        })
        .then_with(|| left.outpoint.vout.cmp(&right.outpoint.vout))
}

fn compare_effective_value(
    wallet: &Wallet,
    fee_rate: FeeRate,
    left: &WalletUtxo,
    right: &WalletUtxo,
) -> Ordering {
    let left_effective = left.output.value.to_sats()
        - fee_rate.fee_for_virtual_size(
            wallet
                .descriptor(left.descriptor_id)
                .map(|record| record.descriptor.estimated_input_vbytes())
                .unwrap_or(0),
        );
    let right_effective = right.output.value.to_sats()
        - fee_rate.fee_for_virtual_size(
            wallet
                .descriptor(right.descriptor_id)
                .map(|record| record.descriptor.estimated_input_vbytes())
                .unwrap_or(0),
        );

    right_effective
        .cmp(&left_effective)
        .then_with(|| {
            right
                .output
                .value
                .to_sats()
                .cmp(&left.output.value.to_sats())
        })
        .then_with(|| {
            left.outpoint
                .txid
                .as_bytes()
                .cmp(right.outpoint.txid.as_bytes())
        })
        .then_with(|| left.outpoint.vout.cmp(&right.outpoint.vout))
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
mod tests {
    use open_bitcoin_chainstate::{ChainPosition, ChainstateSnapshot, Coin};
    use open_bitcoin_consensus::{TransactionValidationContext, validate_transaction_with_context};
    use open_bitcoin_mempool::validate_standard_transaction;
    use open_bitcoin_primitives::{
        BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid,
    };

    use super::{
        BuildRequest, BuiltTransaction, Recipient, Wallet, WalletSnapshot, WalletUtxo,
        amount_from_sats, standard_wallet_verify_flags,
    };
    use crate::WalletError;
    use crate::address::AddressNetwork;
    use crate::descriptor::DescriptorRole;

    fn sample_tip(height: u32) -> ChainPosition {
        ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: Default::default(),
                time: 1_700_000_000 + height,
                bits: 0x207f_ffff,
                nonce: 1,
            },
            height,
            1,
            i64::from(1_700_000_000 + height),
        )
    }

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
    }

    fn wallet_with_descriptors() -> Wallet {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor(
                "receive",
                DescriptorRole::External,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("receive descriptor");
        wallet
            .import_descriptor(
                "change",
                DescriptorRole::Internal,
                "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            )
            .expect("change descriptor");
        wallet
    }

    fn funded_snapshot(wallet: &Wallet) -> ChainstateSnapshot {
        let receive_script = wallet
            .default_receive_address()
            .expect("receive address")
            .script_pubkey;
        let mut utxos = std::collections::HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(75_000).expect("amount"),
                    script_pubkey: receive_script,
                },
                is_coinbase: false,
                created_height: 9,
                created_median_time_past: 1_700_000_009,
            },
        );

        ChainstateSnapshot::new(vec![sample_tip(10)], utxos, Default::default())
    }

    #[test]
    fn rescan_populates_wallet_balance_from_matching_chainstate_outputs() {
        let mut wallet = wallet_with_descriptors();
        wallet
            .rescan_chainstate(&funded_snapshot(&wallet))
            .expect("rescan");
        let balance = wallet.balance(100).expect("balance");

        assert_eq!(wallet.utxos().len(), 1);
        assert_eq!(balance.total.to_sats(), 75_000);
        assert_eq!(balance.spendable.to_sats(), 75_000);
        assert_eq!(balance.immature.to_sats(), 0);
    }

    #[test]
    fn build_and_sign_produces_a_standard_spendable_transaction() {
        let mut wallet = wallet_with_descriptors();
        wallet
            .rescan_chainstate(&funded_snapshot(&wallet))
            .expect("rescan");
        let recipient = Recipient::from_address(
            &wallet
                .default_change_address()
                .expect("standard change address"),
            amount_from_sats(30_000).expect("amount"),
        );
        let built = wallet
            .build_and_sign(
                &BuildRequest {
                    recipients: vec![recipient],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(2000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect("build and sign");
        let input_contexts = wallet.input_contexts_for(&built).expect("input contexts");
        let validation_context = TransactionValidationContext {
            inputs: input_contexts.clone(),
            spend_height: 11,
            block_time: 1_700_000_010,
            median_time_past: 1_700_000_010,
            verify_flags: standard_wallet_verify_flags(),
            consensus_params: open_bitcoin_consensus::ConsensusParams::default(),
        };

        validate_transaction_with_context(&built.transaction, &validation_context)
            .expect("signed transaction should validate");
        validate_standard_transaction(
            &built.transaction,
            &input_contexts,
            &open_bitcoin_mempool::PolicyConfig::default(),
            open_bitcoin_mempool::transaction_weight_and_virtual_size(&built.transaction).0,
            open_bitcoin_mempool::transaction_sigops_cost(&built.transaction, &input_contexts)
                .expect("sigops"),
        )
        .expect("standard policy");
    }

    #[test]
    fn legacy_descriptor_signing_populates_script_sig() {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        let receive_id = wallet
            .import_descriptor(
                "legacy",
                DescriptorRole::External,
                "pkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("legacy descriptor");
        wallet
            .import_descriptor(
                "legacy-change",
                DescriptorRole::Internal,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("legacy change descriptor");
        let receive_script = wallet
            .address_for_descriptor(receive_id)
            .expect("address")
            .script_pubkey;
        let mut utxos = std::collections::HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([8_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(60_000).expect("amount"),
                    script_pubkey: receive_script.clone(),
                },
                is_coinbase: false,
                created_height: 5,
                created_median_time_past: 1_700_000_005,
            },
        );
        wallet
            .rescan_chainstate(&ChainstateSnapshot::new(
                vec![sample_tip(6)],
                utxos,
                Default::default(),
            ))
            .expect("rescan");
        let built = wallet
            .build_and_sign(
                &BuildRequest {
                    recipients: vec![Recipient {
                        script_pubkey: script(&[0x51]),
                        value: amount_from_sats(20_000).expect("amount"),
                    }],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1500),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect("legacy spend");

        assert!(!built.transaction.inputs[0].script_sig.is_empty());
        assert!(built.transaction.inputs[0].witness.is_empty());
    }

    #[test]
    fn watch_only_outputs_do_not_count_as_spendable() {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor(
                "watch",
                DescriptorRole::External,
                "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)",
            )
            .expect("watch-only descriptor");
        let snapshot = funded_snapshot(&wallet_with_descriptors());
        wallet.rescan_chainstate(&snapshot).expect("rescan");
        let error = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient {
                        script_pubkey: script(&[0x51]),
                        value: amount_from_sats(10_000).expect("amount"),
                    }],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect_err("watch-only wallet cannot spend");

        assert_eq!(error, WalletError::NoSpendableCoins);
    }

    #[test]
    fn coinbase_outputs_stay_immature_until_the_maturity_window_passes() {
        let mut wallet = wallet_with_descriptors();
        let receive_script = wallet
            .default_receive_address()
            .expect("receive")
            .script_pubkey;
        let mut utxos = std::collections::HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([9_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(50_000).expect("amount"),
                    script_pubkey: receive_script,
                },
                is_coinbase: true,
                created_height: 15,
                created_median_time_past: 1_700_000_015,
            },
        );
        wallet
            .rescan_chainstate(&ChainstateSnapshot::new(
                vec![sample_tip(20)],
                utxos,
                Default::default(),
            ))
            .expect("rescan");

        let balance = wallet.balance(100).expect("balance");

        assert_eq!(balance.spendable.to_sats(), 0);
        assert_eq!(balance.immature.to_sats(), 50_000);
    }

    #[test]
    fn build_transaction_requires_change_descriptor_for_changeful_spends() {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor(
                "receive",
                DescriptorRole::External,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("receive descriptor");
        wallet
            .rescan_chainstate(&funded_snapshot(&wallet_with_descriptors()))
            .expect("rescan");
        let error = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient {
                        script_pubkey: script(&[0x51]),
                        value: amount_from_sats(10_000).expect("amount"),
                    }],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect_err("change descriptor is required");

        assert_eq!(error, WalletError::ChangeDescriptorRequired);
    }

    #[test]
    fn duplicate_labels_and_unspendable_snapshots_are_rejected() {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor(
                "receive",
                DescriptorRole::External,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("receive descriptor");
        assert_eq!(
            wallet
                .import_descriptor(
                    "receive",
                    DescriptorRole::Internal,
                    "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
                )
                .expect_err("duplicate label"),
            WalletError::DuplicateLabel("receive".to_string())
        );

        let watch_only = Wallet::from_snapshot(WalletSnapshot {
            network: AddressNetwork::Regtest,
            descriptors: vec![crate::descriptor::DescriptorRecord {
                id: 0,
                label: "watch".to_string(),
                role: DescriptorRole::External,
                original_text:
                    "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                        .to_string(),
                descriptor: crate::descriptor::SingleKeyDescriptor::parse(
                    "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
                    AddressNetwork::Regtest,
                )
                .expect("watch descriptor"),
            }],
            utxos: vec![WalletUtxo {
                descriptor_id: 0,
                outpoint: OutPoint {
                    txid: Txid::from_byte_array([3_u8; 32]),
                    vout: 0,
                },
                output: TransactionOutput {
                    value: amount_from_sats(5_000).expect("amount"),
                    script_pubkey: crate::descriptor::SingleKeyDescriptor::parse(
                        "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
                        AddressNetwork::Regtest,
                    )
                    .expect("descriptor")
                    .script_pubkey()
                    .expect("script"),
                },
                created_height: 1,
                created_median_time_past: 1,
                is_coinbase: false,
            }],
            next_descriptor_id: 1,
            maybe_tip_height: Some(2),
            maybe_tip_median_time_past: Some(2),
        });
        let unknown_descriptor = Wallet::from_snapshot(WalletSnapshot {
            network: AddressNetwork::Regtest,
            descriptors: Vec::new(),
            utxos: vec![WalletUtxo {
                descriptor_id: 99,
                outpoint: OutPoint {
                    txid: Txid::from_byte_array([4_u8; 32]),
                    vout: 0,
                },
                output: TransactionOutput {
                    value: amount_from_sats(5_000).expect("amount"),
                    script_pubkey: script(&[0x51]),
                },
                created_height: 1,
                created_median_time_past: 1,
                is_coinbase: false,
            }],
            next_descriptor_id: 0,
            maybe_tip_height: Some(2),
            maybe_tip_median_time_past: Some(2),
        });
        let immature_coinbase = Wallet::from_snapshot(WalletSnapshot {
            network: AddressNetwork::Regtest,
            descriptors: wallet.descriptors().to_vec(),
            utxos: vec![WalletUtxo {
                descriptor_id: 0,
                outpoint: OutPoint {
                    txid: Txid::from_byte_array([5_u8; 32]),
                    vout: 0,
                },
                output: TransactionOutput {
                    value: amount_from_sats(5_000).expect("amount"),
                    script_pubkey: wallet
                        .default_receive_address()
                        .expect("receive")
                        .script_pubkey,
                },
                created_height: 10,
                created_median_time_past: 10,
                is_coinbase: true,
            }],
            next_descriptor_id: 1,
            maybe_tip_height: Some(10),
            maybe_tip_median_time_past: Some(10),
        });

        for wallet in [watch_only, unknown_descriptor, immature_coinbase] {
            assert_eq!(
                wallet
                    .build_transaction(
                        &BuildRequest {
                            recipients: vec![Recipient {
                                script_pubkey: script(&[0x51]),
                                value: amount_from_sats(1_000).expect("amount"),
                            }],
                            fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                            maybe_change_descriptor_id: None,
                            maybe_lock_time: None,
                            enable_rbf: true,
                        },
                        100,
                    )
                    .expect_err("unspendable snapshot"),
                WalletError::NoSpendableCoins
            );
        }
    }

    #[test]
    fn snapshot_round_trips_state_and_address_accessors() {
        let mut wallet = wallet_with_descriptors();
        wallet
            .rescan_chainstate(&funded_snapshot(&wallet))
            .expect("rescan");
        let snapshot = wallet.snapshot();
        let restored = Wallet::from_snapshot(snapshot.clone());

        assert_eq!(wallet.network(), AddressNetwork::Regtest);
        assert_eq!(restored.network(), AddressNetwork::Regtest);
        assert_eq!(snapshot.descriptors.len(), 2);
        assert_eq!(restored.descriptors().len(), 2);
        assert_eq!(restored.utxos().len(), 1);
        assert!(restored.address_for_descriptor(0).is_ok());
        assert!(restored.default_receive_address().is_ok());
        assert!(restored.default_change_address().is_ok());
        assert_eq!(
            restored
                .address_for_descriptor(42)
                .expect_err("missing descriptor"),
            WalletError::UnknownDescriptor(42)
        );
    }

    #[test]
    fn wallet_reports_missing_roles_and_basic_build_errors() {
        let wallet = Wallet::new(AddressNetwork::Regtest);
        assert_eq!(
            wallet
                .default_receive_address()
                .expect_err("missing external"),
            WalletError::ChangeDescriptorRequired
        );
        assert_eq!(
            wallet
                .default_change_address()
                .expect_err("missing internal"),
            WalletError::ChangeDescriptorRequired
        );
        assert_eq!(
            wallet
                .build_transaction(
                    &BuildRequest {
                        recipients: Vec::new(),
                        fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                        maybe_change_descriptor_id: None,
                        maybe_lock_time: None,
                        enable_rbf: true,
                    },
                    100,
                )
                .expect_err("no recipients"),
            WalletError::NoRecipients
        );
    }

    #[test]
    fn build_transaction_reports_insufficient_funds_and_uses_snapshot_sorting() {
        let mut wallet = wallet_with_descriptors();
        let receive_script = wallet
            .default_receive_address()
            .expect("receive")
            .script_pubkey;
        let mut utxos = std::collections::HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([2_u8; 32]),
                vout: 1,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(10_000).expect("amount"),
                    script_pubkey: receive_script.clone(),
                },
                is_coinbase: false,
                created_height: 7,
                created_median_time_past: 1_700_000_007,
            },
        );
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([1_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(40_000).expect("amount"),
                    script_pubkey: receive_script,
                },
                is_coinbase: false,
                created_height: 6,
                created_median_time_past: 1_700_000_006,
            },
        );
        wallet
            .rescan_chainstate(&ChainstateSnapshot::new(
                vec![sample_tip(10)],
                utxos,
                Default::default(),
            ))
            .expect("rescan");
        let build = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient {
                        script_pubkey: wallet
                            .default_change_address()
                            .expect("change")
                            .script_pubkey,
                        value: amount_from_sats(20_000).expect("amount"),
                    }],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: Some(99),
                    enable_rbf: true,
                },
                100,
            )
            .expect("build");
        let insufficient = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient {
                        script_pubkey: script(&[0x51]),
                        value: amount_from_sats(1_000_000).expect("amount"),
                    }],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect_err("insufficient");

        assert_eq!(wallet.utxos()[0].created_height, 6);
        assert_eq!(
            build.selected_inputs[0].outpoint.txid,
            Txid::from_byte_array([1_u8; 32])
        );
        assert_eq!(build.transaction.lock_time, 99);
        assert!(insufficient.to_string().contains("insufficient funds"));
    }

    #[test]
    fn nested_segwit_and_taproot_signing_cover_remaining_descriptor_paths() {
        let mut nested = Wallet::new(AddressNetwork::Regtest);
        nested
            .import_descriptor(
                "nested-receive",
                DescriptorRole::External,
                "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            )
            .expect("nested receive");
        nested
            .import_descriptor(
                "nested-change",
                DescriptorRole::Internal,
                "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            )
            .expect("nested change");
        nested
            .rescan_chainstate(&funded_snapshot(&nested))
            .expect("nested rescan");
        let nested_spend = nested
            .build_and_sign(
                &BuildRequest {
                    recipients: vec![Recipient::from_address(
                        &nested.default_change_address().expect("change"),
                        amount_from_sats(20_000).expect("amount"),
                    )],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1200),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect("nested spend");

        assert!(!nested_spend.transaction.inputs[0].script_sig.is_empty());
        assert_eq!(nested_spend.transaction.inputs[0].witness.stack().len(), 2);

        let mut taproot = Wallet::new(AddressNetwork::Regtest);
        taproot
            .import_descriptor(
                "taproot-receive",
                DescriptorRole::External,
                "tr(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("taproot receive");
        taproot
            .import_descriptor(
                "taproot-change",
                DescriptorRole::Internal,
                "tr(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("taproot change");
        taproot
            .rescan_chainstate(&funded_snapshot(&taproot))
            .expect("taproot rescan");
        let taproot_spend = taproot
            .build_and_sign(
                &BuildRequest {
                    recipients: vec![Recipient::from_address(
                        &taproot.default_change_address().expect("change"),
                        amount_from_sats(20_000).expect("amount"),
                    )],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1200),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect("taproot spend");

        assert!(taproot_spend.transaction.inputs[0].script_sig.is_empty());
        assert_eq!(taproot_spend.transaction.inputs[0].witness.stack().len(), 1);
    }

    #[test]
    fn final_sequence_and_dust_change_paths_are_covered() {
        let mut wallet = wallet_with_descriptors();
        wallet
            .rescan_chainstate(&funded_snapshot(&wallet))
            .expect("rescan");
        let built = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient::from_address(
                        &wallet.default_change_address().expect("change"),
                        amount_from_sats(74_800).expect("amount"),
                    )],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: Some(33),
                    enable_rbf: false,
                },
                100,
            )
            .expect("build without change");
        let no_capacity_for_change = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient::from_address(
                        &wallet.default_change_address().expect("change"),
                        amount_from_sats(74_860).expect("amount"),
                    )],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: Some(34),
                    enable_rbf: false,
                },
                100,
            )
            .expect("build with no room for change");

        assert_eq!(built.change_output_index, None);
        assert_eq!(
            built.transaction.inputs[0].sequence,
            TransactionInput::SEQUENCE_FINAL
        );
        assert_eq!(built.transaction.lock_time, 33);
        assert_eq!(no_capacity_for_change.change_output_index, None);
    }

    #[test]
    fn change_outputs_and_sort_tiebreakers_are_explicit() {
        let mut wallet = wallet_with_descriptors();
        wallet
            .rescan_chainstate(&funded_snapshot(&wallet))
            .expect("rescan");
        let with_change = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient::from_address(
                        &wallet.default_change_address().expect("change"),
                        amount_from_sats(30_000).expect("amount"),
                    )],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect("changeful spend");

        assert!(with_change.change_output_index.is_some());
        assert!(
            wallet
                .estimate_vsize(
                    &with_change.selected_inputs,
                    &[Recipient::from_address(
                        &wallet.default_change_address().expect("change"),
                        amount_from_sats(30_000).expect("amount"),
                    )],
                    Some(&TransactionOutput {
                        value: amount_from_sats(1).expect("amount"),
                        script_pubkey: wallet
                            .default_change_address()
                            .expect("change")
                            .script_pubkey,
                    }),
                    &BuildRequest {
                        recipients: vec![Recipient::from_address(
                            &wallet.default_change_address().expect("change"),
                            amount_from_sats(30_000).expect("amount"),
                        )],
                        fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                        maybe_change_descriptor_id: None,
                        maybe_lock_time: None,
                        enable_rbf: true,
                    },
                )
                .expect("estimate with change")
                > 0
        );

        let receive_script = wallet
            .default_receive_address()
            .expect("receive")
            .script_pubkey;
        let mut equal_snapshot = std::collections::HashMap::new();
        equal_snapshot.insert(
            OutPoint {
                txid: Txid::from_byte_array([9_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(15_000).expect("amount"),
                    script_pubkey: receive_script.clone(),
                },
                is_coinbase: false,
                created_height: 3,
                created_median_time_past: 3,
            },
        );
        equal_snapshot.insert(
            OutPoint {
                txid: Txid::from_byte_array([1_u8; 32]),
                vout: 1,
            },
            Coin {
                output: TransactionOutput {
                    value: amount_from_sats(15_000).expect("amount"),
                    script_pubkey: receive_script,
                },
                is_coinbase: false,
                created_height: 3,
                created_median_time_past: 3,
            },
        );
        wallet
            .rescan_chainstate(&ChainstateSnapshot::new(
                vec![sample_tip(4)],
                equal_snapshot,
                Default::default(),
            ))
            .expect("rescan equal snapshot");
        let equal_build = wallet
            .build_transaction(
                &BuildRequest {
                    recipients: vec![Recipient {
                        script_pubkey: script(&[0x51]),
                        value: amount_from_sats(10_000).expect("amount"),
                    }],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect("equal-value build");

        assert_eq!(
            wallet.utxos()[0].outpoint.txid,
            Txid::from_byte_array([1_u8; 32])
        );
        assert_eq!(
            equal_build.selected_inputs[0].outpoint.txid,
            Txid::from_byte_array([1_u8; 32])
        );
    }

    #[test]
    fn signing_reports_missing_private_keys_and_watch_only_paths() {
        let watch_descriptor = crate::descriptor::SingleKeyDescriptor::parse(
            "pkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
            AddressNetwork::Regtest,
        )
        .expect("watch descriptor");
        let watch_utxo = WalletUtxo {
            descriptor_id: 0,
            outpoint: OutPoint {
                txid: Txid::from_byte_array([6_u8; 32]),
                vout: 0,
            },
            output: TransactionOutput {
                value: amount_from_sats(5_000).expect("amount"),
                script_pubkey: watch_descriptor.script_pubkey().expect("script"),
            },
            created_height: 1,
            created_median_time_past: 1,
            is_coinbase: false,
        };
        let watch_wallet = Wallet::from_snapshot(WalletSnapshot {
            network: AddressNetwork::Regtest,
            descriptors: vec![crate::descriptor::DescriptorRecord {
                id: 0,
                label: "watch".to_string(),
                role: DescriptorRole::External,
                original_text:
                    "pkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                        .to_string(),
                descriptor: watch_descriptor,
            }],
            utxos: vec![watch_utxo.clone()],
            next_descriptor_id: 1,
            maybe_tip_height: Some(2),
            maybe_tip_median_time_past: Some(2),
        });
        let watch_built = BuiltTransaction {
            transaction: Transaction {
                version: 2,
                inputs: vec![TransactionInput {
                    previous_output: watch_utxo.outpoint.clone(),
                    script_sig: ScriptBuf::default(),
                    sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: amount_from_sats(4_000).expect("amount"),
                    script_pubkey: script(&[0x51]),
                }],
                lock_time: 0,
            },
            selected_inputs: vec![watch_utxo],
            fee: amount_from_sats(1_000).expect("amount"),
            change_output_index: None,
        };
        assert!(
            watch_wallet
                .sign_transaction(&watch_built)
                .expect_err("missing legacy key")
                .to_string()
                .contains("descriptor cannot sign")
        );

        let witness_watch_descriptor = crate::descriptor::SingleKeyDescriptor::parse(
            "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
            AddressNetwork::Regtest,
        )
        .expect("wpkh watch-only");
        let witness_watch = Wallet::from_snapshot(WalletSnapshot {
            network: AddressNetwork::Regtest,
            descriptors: vec![crate::descriptor::DescriptorRecord {
                id: 0,
                label: "wpkh-watch".to_string(),
                role: DescriptorRole::External,
                original_text:
                    "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                        .to_string(),
                descriptor: witness_watch_descriptor.clone(),
            }],
            utxos: vec![WalletUtxo {
                descriptor_id: 0,
                outpoint: OutPoint {
                    txid: Txid::from_byte_array([8_u8; 32]),
                    vout: 0,
                },
                output: TransactionOutput {
                    value: amount_from_sats(5_000).expect("amount"),
                    script_pubkey: witness_watch_descriptor.script_pubkey().expect("script"),
                },
                created_height: 1,
                created_median_time_past: 1,
                is_coinbase: false,
            }],
            next_descriptor_id: 1,
            maybe_tip_height: Some(2),
            maybe_tip_median_time_past: Some(2),
        });
        let witness_watch_built = BuiltTransaction {
            transaction: Transaction {
                version: 2,
                inputs: vec![TransactionInput {
                    previous_output: OutPoint {
                        txid: Txid::from_byte_array([8_u8; 32]),
                        vout: 0,
                    },
                    script_sig: ScriptBuf::default(),
                    sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: amount_from_sats(4_000).expect("amount"),
                    script_pubkey: script(&[0x51]),
                }],
                lock_time: 0,
            },
            selected_inputs: witness_watch.utxos().to_vec(),
            fee: amount_from_sats(1_000).expect("amount"),
            change_output_index: None,
        };
        assert!(
            witness_watch
                .sign_transaction(&witness_watch_built)
                .expect_err("missing segwit key")
                .to_string()
                .contains("descriptor cannot sign")
        );

        let taproot_watch_descriptor = crate::descriptor::SingleKeyDescriptor::parse(
            "tr(4d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
            AddressNetwork::Regtest,
        )
        .expect("taproot watch-only descriptor");
        let taproot_utxo = WalletUtxo {
            descriptor_id: 0,
            outpoint: OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            output: TransactionOutput {
                value: amount_from_sats(5_000).expect("amount"),
                script_pubkey: taproot_watch_descriptor.script_pubkey().expect("script"),
            },
            created_height: 1,
            created_median_time_past: 1,
            is_coinbase: false,
        };
        let taproot_watch = Wallet::from_snapshot(WalletSnapshot {
            network: AddressNetwork::Regtest,
            descriptors: vec![crate::descriptor::DescriptorRecord {
                id: 0,
                label: "taproot-watch".to_string(),
                role: DescriptorRole::External,
                original_text:
                    "tr(4d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                        .to_string(),
                descriptor: taproot_watch_descriptor,
            }],
            utxos: vec![taproot_utxo.clone()],
            next_descriptor_id: 1,
            maybe_tip_height: Some(2),
            maybe_tip_median_time_past: Some(2),
        });
        let taproot_built = BuiltTransaction {
            transaction: Transaction {
                version: 2,
                inputs: vec![TransactionInput {
                    previous_output: taproot_utxo.outpoint.clone(),
                    script_sig: ScriptBuf::default(),
                    sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: amount_from_sats(4_000).expect("amount"),
                    script_pubkey: script(&[0x51]),
                }],
                lock_time: 0,
            },
            selected_inputs: vec![taproot_utxo],
            fee: amount_from_sats(1_000).expect("amount"),
            change_output_index: None,
        };

        assert!(
            taproot_watch
                .sign_transaction(&taproot_built)
                .expect_err("missing taproot key")
                .to_string()
                .contains("descriptor cannot sign")
        );
    }
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use core::cmp::Ordering;

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{TransactionInputContext, TransactionValidationContext};
use open_bitcoin_mempool::FeeRate;
use open_bitcoin_primitives::{Amount, OutPoint, ScriptBuf, Transaction, TransactionOutput};

use crate::WalletError;
use crate::address::{Address, AddressNetwork};
use crate::descriptor::{DescriptorRecord, DescriptorRole, SingleKeyDescriptor};

mod build;
mod scan;
mod sign;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeeEstimateMode {
    Unset,
    Economical,
    Conservative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeEstimateRequest {
    pub conf_target: u16,
    pub mode: FeeEstimateMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeeSelection {
    Explicit(FeeRate),
    Estimate(FeeEstimateRequest),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangePolicy {
    Automatic,
    ChangeForbidden,
    FixedDescriptor(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendIntent {
    pub recipients: Vec<Recipient>,
    pub fee_selection: FeeSelection,
    pub change_policy: ChangePolicy,
    pub maybe_lock_time: Option<u32>,
    pub enable_rbf: bool,
    pub maybe_fee_ceiling_sats: Option<i64>,
}

impl SendIntent {
    pub fn new(
        recipients: Vec<Recipient>,
        fee_selection: FeeSelection,
        change_policy: ChangePolicy,
        maybe_lock_time: Option<u32>,
        enable_rbf: bool,
        maybe_fee_ceiling_sats: Option<i64>,
    ) -> Result<Self, WalletError> {
        if recipients.is_empty() {
            return Err(WalletError::NoRecipients);
        }
        if let FeeSelection::Estimate(request) = fee_selection
            && request.conf_target == 0
        {
            return Err(WalletError::InvalidEstimateRequest(
                "conf_target must be at least 1".to_string(),
            ));
        }
        if let Some(ceiling_sats) = maybe_fee_ceiling_sats
            && ceiling_sats <= 0
        {
            return Err(WalletError::FeeCeilingExceeded {
                fee_sats: 0,
                ceiling_sats,
            });
        }

        Ok(Self {
            recipients,
            fee_selection,
            change_policy,
            maybe_lock_time,
            enable_rbf,
            maybe_fee_ceiling_sats,
        })
    }

    pub fn into_build_request(
        &self,
        maybe_resolved_estimate: Option<FeeRate>,
    ) -> Result<BuildRequest, WalletError> {
        let fee_rate = match self.fee_selection {
            FeeSelection::Explicit(fee_rate) => fee_rate,
            FeeSelection::Estimate(_) => {
                let Some(fee_rate) = maybe_resolved_estimate else {
                    return Err(WalletError::EstimatorUnavailable(
                        "estimate_mode requires a resolved fee rate".to_string(),
                    ));
                };
                fee_rate
            }
        };

        let maybe_change_descriptor_id = match self.change_policy {
            ChangePolicy::Automatic => None,
            ChangePolicy::ChangeForbidden => None,
            ChangePolicy::FixedDescriptor(descriptor_id) => Some(descriptor_id),
        };

        Ok(BuildRequest {
            recipients: self.recipients.clone(),
            fee_rate,
            maybe_change_descriptor_id,
            maybe_lock_time: self.maybe_lock_time,
            enable_rbf: self.enable_rbf,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletRescanState {
    Fresh {
        scanned_through_height: u32,
        target_height: u32,
    },
    Partial {
        scanned_through_height: u32,
        target_height: u32,
    },
    Scanning {
        next_height: u32,
        target_height: u32,
    },
}

impl WalletRescanState {
    pub fn from_progress(
        maybe_scanned_through_height: Option<u32>,
        maybe_target_height: Option<u32>,
        maybe_next_height: Option<u32>,
        is_scanning: bool,
    ) -> Result<Self, WalletError> {
        scan::rescan_state_from_progress(
            maybe_scanned_through_height,
            maybe_target_height,
            maybe_next_height,
            is_scanning,
        )
    }
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
            original_text: descriptor.storage_text(),
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

    pub fn allocate_receive_address(&mut self) -> Result<Address, WalletError> {
        self.allocate_address_for_role(DescriptorRole::External)
    }

    pub fn allocate_change_address(&mut self) -> Result<Address, WalletError> {
        self.allocate_address_for_role(DescriptorRole::Internal)
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
        sign::sign_transaction(self, built)
    }

    pub fn build_and_sign(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, WalletError> {
        sign::build_and_sign(self, request, coinbase_maturity)
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

    fn allocate_address_for_role(&mut self, role: DescriptorRole) -> Result<Address, WalletError> {
        let Some(record) = self
            .descriptors
            .iter_mut()
            .find(|descriptor| descriptor.role == role)
        else {
            return Err(WalletError::UnsupportedAddressRole(match role {
                DescriptorRole::External => "external".to_string(),
                DescriptorRole::Internal => "internal".to_string(),
            }));
        };
        let index = record.descriptor.advance_next_index(role)?;
        let address = record.descriptor.address_at(self.network, index)?;
        record.original_text = record.descriptor.storage_text();
        Ok(address)
    }
}

fn amount_from_sats(sats: i64) -> Result<Amount, WalletError> {
    build::amount_from_sats(sats)
}

fn compare_wallet_utxos(left: &WalletUtxo, right: &WalletUtxo) -> Ordering {
    build::compare_wallet_utxos(left, right)
}

fn push_script(pushes: &[&[u8]]) -> Result<ScriptBuf, WalletError> {
    sign::push_script(pushes)
}

fn standard_wallet_verify_flags() -> open_bitcoin_consensus::ScriptVerifyFlags {
    sign::standard_wallet_verify_flags()
}

#[cfg(test)]
mod tests;

// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_core::{
    primitives::{BlockHash, OutPoint},
    wallet::{
        AddressNetwork, DescriptorRecord, DescriptorRole, SingleKeyDescriptor, WalletSnapshot,
        WalletUtxo,
    },
};
use serde::{Deserialize, Serialize};

use super::{OutPointDto, TransactionOutputDto, corruption, decode_versioned, encode_versioned};
use crate::{
    SelectedWalletRecord, StorageError, StorageNamespace, WalletRegistrySnapshot,
    WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WalletDescriptorDto {
    id: u32,
    label: String,
    role: DescriptorRoleDto,
    original_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DescriptorRoleDto {
    External,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum AddressNetworkDto {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WalletUtxoDto {
    descriptor_id: u32,
    outpoint: OutPointDto,
    output: TransactionOutputDto,
    created_height: u32,
    created_median_time_past: i64,
    is_coinbase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WalletSnapshotDto {
    network: AddressNetworkDto,
    descriptors: Vec<WalletDescriptorDto>,
    utxos: Vec<WalletUtxoDto>,
    next_descriptor_id: u32,
    maybe_tip_height: Option<u32>,
    maybe_tip_median_time_past: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WalletRegistrySnapshotDto {
    wallet_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SelectedWalletRecordDto {
    wallet_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WalletRescanFreshnessDto {
    Fresh,
    Partial,
    Scanning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WalletRescanJobStateDto {
    Pending,
    Scanning,
    Complete,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WalletRescanJobDto {
    wallet_name: String,
    target_tip_hash: [u8; 32],
    target_tip_height: u32,
    next_height: u32,
    maybe_scanned_through_height: Option<u32>,
    maybe_tip_median_time_past: Option<i64>,
    freshness: WalletRescanFreshnessDto,
    state: WalletRescanJobStateDto,
    maybe_error: Option<String>,
}

pub(crate) fn encode_wallet_snapshot(snapshot: &WalletSnapshot) -> Result<Vec<u8>, StorageError> {
    encode_versioned(StorageNamespace::Wallet, &WalletSnapshotDto::from(snapshot))
}

pub(crate) fn decode_wallet_snapshot(bytes: &[u8]) -> Result<WalletSnapshot, StorageError> {
    let dto: WalletSnapshotDto = decode_versioned(StorageNamespace::Wallet, bytes)?;
    dto.try_into()
}

pub(crate) fn encode_wallet_registry_snapshot(
    snapshot: &WalletRegistrySnapshot,
) -> Result<Vec<u8>, StorageError> {
    encode_versioned(
        StorageNamespace::Wallet,
        &WalletRegistrySnapshotDto::from(snapshot),
    )
}

pub(crate) fn decode_wallet_registry_snapshot(
    bytes: &[u8],
) -> Result<WalletRegistrySnapshot, StorageError> {
    let dto: WalletRegistrySnapshotDto = decode_versioned(StorageNamespace::Wallet, bytes)?;
    Ok(dto.into())
}

pub(crate) fn encode_selected_wallet(
    record: &SelectedWalletRecord,
) -> Result<Vec<u8>, StorageError> {
    encode_versioned(
        StorageNamespace::Wallet,
        &SelectedWalletRecordDto::from(record),
    )
}

pub(crate) fn decode_selected_wallet(bytes: &[u8]) -> Result<SelectedWalletRecord, StorageError> {
    let dto: SelectedWalletRecordDto = decode_versioned(StorageNamespace::Wallet, bytes)?;
    Ok(dto.into())
}

pub(crate) fn encode_wallet_rescan_job(job: &WalletRescanJob) -> Result<Vec<u8>, StorageError> {
    encode_versioned(StorageNamespace::Wallet, &WalletRescanJobDto::from(job))
}

pub(crate) fn decode_wallet_rescan_job(bytes: &[u8]) -> Result<WalletRescanJob, StorageError> {
    let dto: WalletRescanJobDto = decode_versioned(StorageNamespace::Wallet, bytes)?;
    dto.try_into()
}

impl From<DescriptorRole> for DescriptorRoleDto {
    fn from(role: DescriptorRole) -> Self {
        match role {
            DescriptorRole::External => Self::External,
            DescriptorRole::Internal => Self::Internal,
        }
    }
}

impl From<DescriptorRoleDto> for DescriptorRole {
    fn from(role: DescriptorRoleDto) -> Self {
        match role {
            DescriptorRoleDto::External => Self::External,
            DescriptorRoleDto::Internal => Self::Internal,
        }
    }
}

impl From<AddressNetwork> for AddressNetworkDto {
    fn from(network: AddressNetwork) -> Self {
        match network {
            AddressNetwork::Mainnet => Self::Mainnet,
            AddressNetwork::Testnet => Self::Testnet,
            AddressNetwork::Signet => Self::Signet,
            AddressNetwork::Regtest => Self::Regtest,
        }
    }
}

impl From<AddressNetworkDto> for AddressNetwork {
    fn from(network: AddressNetworkDto) -> Self {
        match network {
            AddressNetworkDto::Mainnet => Self::Mainnet,
            AddressNetworkDto::Testnet => Self::Testnet,
            AddressNetworkDto::Signet => Self::Signet,
            AddressNetworkDto::Regtest => Self::Regtest,
        }
    }
}

impl From<&DescriptorRecord> for WalletDescriptorDto {
    fn from(record: &DescriptorRecord) -> Self {
        Self {
            id: record.id,
            label: record.label.clone(),
            role: DescriptorRoleDto::from(record.role),
            original_text: record.original_text.clone(),
        }
    }
}

impl WalletDescriptorDto {
    fn into_record(self, network: AddressNetwork) -> Result<DescriptorRecord, StorageError> {
        let descriptor = SingleKeyDescriptor::parse(
            &normalize_descriptor_storage_text(&self.original_text),
            network,
        )
        .map_err(|error| corruption(StorageNamespace::Wallet, error))?;

        Ok(DescriptorRecord {
            id: self.id,
            label: self.label,
            role: DescriptorRole::from(self.role),
            original_text: self.original_text,
            descriptor,
        })
    }
}

fn normalize_descriptor_storage_text(text: &str) -> String {
    text.replace("#ob::", "#ob:")
}

impl From<&WalletUtxo> for WalletUtxoDto {
    fn from(utxo: &WalletUtxo) -> Self {
        Self {
            descriptor_id: utxo.descriptor_id,
            outpoint: OutPointDto::from(&utxo.outpoint),
            output: TransactionOutputDto::from(&utxo.output),
            created_height: utxo.created_height,
            created_median_time_past: utxo.created_median_time_past,
            is_coinbase: utxo.is_coinbase,
        }
    }
}

impl TryFrom<WalletUtxoDto> for WalletUtxo {
    type Error = StorageError;

    fn try_from(dto: WalletUtxoDto) -> Result<Self, Self::Error> {
        Ok(Self {
            descriptor_id: dto.descriptor_id,
            outpoint: OutPoint::from(dto.outpoint),
            output: dto.output.try_into()?,
            created_height: dto.created_height,
            created_median_time_past: dto.created_median_time_past,
            is_coinbase: dto.is_coinbase,
        })
    }
}

impl From<&WalletSnapshot> for WalletSnapshotDto {
    fn from(snapshot: &WalletSnapshot) -> Self {
        Self {
            network: AddressNetworkDto::from(snapshot.network),
            descriptors: snapshot
                .descriptors
                .iter()
                .map(WalletDescriptorDto::from)
                .collect(),
            utxos: snapshot.utxos.iter().map(WalletUtxoDto::from).collect(),
            next_descriptor_id: snapshot.next_descriptor_id,
            maybe_tip_height: snapshot.maybe_tip_height,
            maybe_tip_median_time_past: snapshot.maybe_tip_median_time_past,
        }
    }
}

impl TryFrom<WalletSnapshotDto> for WalletSnapshot {
    type Error = StorageError;

    fn try_from(dto: WalletSnapshotDto) -> Result<Self, Self::Error> {
        let network = AddressNetwork::from(dto.network);
        let descriptors = dto
            .descriptors
            .into_iter()
            .map(|descriptor| descriptor.into_record(network))
            .collect::<Result<Vec<_>, _>>()?;
        let utxos = dto
            .utxos
            .into_iter()
            .map(WalletUtxo::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            network,
            descriptors,
            utxos,
            next_descriptor_id: dto.next_descriptor_id,
            maybe_tip_height: dto.maybe_tip_height,
            maybe_tip_median_time_past: dto.maybe_tip_median_time_past,
        })
    }
}

impl From<&WalletRegistrySnapshot> for WalletRegistrySnapshotDto {
    fn from(snapshot: &WalletRegistrySnapshot) -> Self {
        Self {
            wallet_names: snapshot.wallet_names.clone(),
        }
    }
}

impl From<WalletRegistrySnapshotDto> for WalletRegistrySnapshot {
    fn from(snapshot: WalletRegistrySnapshotDto) -> Self {
        WalletRegistrySnapshot::new(snapshot.wallet_names)
    }
}

impl From<&SelectedWalletRecord> for SelectedWalletRecordDto {
    fn from(record: &SelectedWalletRecord) -> Self {
        Self {
            wallet_name: record.wallet_name.clone(),
        }
    }
}

impl From<SelectedWalletRecordDto> for SelectedWalletRecord {
    fn from(record: SelectedWalletRecordDto) -> Self {
        Self {
            wallet_name: record.wallet_name,
        }
    }
}

impl From<WalletRescanFreshness> for WalletRescanFreshnessDto {
    fn from(value: WalletRescanFreshness) -> Self {
        match value {
            WalletRescanFreshness::Fresh => Self::Fresh,
            WalletRescanFreshness::Partial => Self::Partial,
            WalletRescanFreshness::Scanning => Self::Scanning,
        }
    }
}

impl From<WalletRescanFreshnessDto> for WalletRescanFreshness {
    fn from(value: WalletRescanFreshnessDto) -> Self {
        match value {
            WalletRescanFreshnessDto::Fresh => Self::Fresh,
            WalletRescanFreshnessDto::Partial => Self::Partial,
            WalletRescanFreshnessDto::Scanning => Self::Scanning,
        }
    }
}

impl From<WalletRescanJobState> for WalletRescanJobStateDto {
    fn from(value: WalletRescanJobState) -> Self {
        match value {
            WalletRescanJobState::Pending => Self::Pending,
            WalletRescanJobState::Scanning => Self::Scanning,
            WalletRescanJobState::Complete => Self::Complete,
            WalletRescanJobState::Failed => Self::Failed,
        }
    }
}

impl From<WalletRescanJobStateDto> for WalletRescanJobState {
    fn from(value: WalletRescanJobStateDto) -> Self {
        match value {
            WalletRescanJobStateDto::Pending => Self::Pending,
            WalletRescanJobStateDto::Scanning => Self::Scanning,
            WalletRescanJobStateDto::Complete => Self::Complete,
            WalletRescanJobStateDto::Failed => Self::Failed,
        }
    }
}

impl From<&WalletRescanJob> for WalletRescanJobDto {
    fn from(job: &WalletRescanJob) -> Self {
        Self {
            wallet_name: job.wallet_name.clone(),
            target_tip_hash: job.target_tip_hash.to_byte_array(),
            target_tip_height: job.target_tip_height,
            next_height: job.next_height,
            maybe_scanned_through_height: job.maybe_scanned_through_height,
            maybe_tip_median_time_past: job.maybe_tip_median_time_past,
            freshness: WalletRescanFreshnessDto::from(job.freshness),
            state: WalletRescanJobStateDto::from(job.state),
            maybe_error: job.maybe_error.clone(),
        }
    }
}

impl TryFrom<WalletRescanJobDto> for WalletRescanJob {
    type Error = StorageError;

    fn try_from(dto: WalletRescanJobDto) -> Result<Self, Self::Error> {
        Ok(Self {
            wallet_name: dto.wallet_name,
            target_tip_hash: BlockHash::from_slice(&dto.target_tip_hash)
                .map_err(|error| corruption(StorageNamespace::Wallet, error))?,
            target_tip_height: dto.target_tip_height,
            next_height: dto.next_height,
            maybe_scanned_through_height: dto.maybe_scanned_through_height,
            maybe_tip_median_time_past: dto.maybe_tip_median_time_past,
            freshness: WalletRescanFreshness::from(dto.freshness),
            state: WalletRescanJobState::from(dto.state),
            maybe_error: dto.maybe_error,
        })
    }
}

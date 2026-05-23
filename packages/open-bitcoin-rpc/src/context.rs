// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_node::MemoryChainstateStore;
use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::wallet::AddressNetwork;
use std::{sync::mpsc, time::Duration};

use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, ManagedPeerNetwork, PersistMode, RuntimeMetadata,
};

use crate::{RpcFailure, RpcFailureKind};

mod network;
mod rescan;
#[cfg(test)]
mod tests;
mod wallet_state;

pub use rescan::{WalletFreshnessKind, WalletFreshnessView, WalletRescanExecution};
use wallet_state::WalletState;

pub struct ManagedRpcContext {
    chain: AddressNetwork,
    consensus_params: ConsensusParams,
    verify_flags: ScriptVerifyFlags,
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    maybe_durable_sync_state: Option<DurableSyncState>,
    maybe_daemon_sync_control: Option<DaemonSyncControl>,
    wallet_state: WalletState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonSyncControlAction {
    Status,
    Pause,
    Resume,
}

#[derive(Debug)]
pub struct DaemonSyncControlRequest {
    action: DaemonSyncControlAction,
    response_sender: mpsc::Sender<Result<RuntimeMetadata, String>>,
}

impl DaemonSyncControlRequest {
    pub const fn action(&self) -> DaemonSyncControlAction {
        self.action
    }

    pub fn respond(self, result: Result<RuntimeMetadata, String>) {
        let _ = self.response_sender.send(result);
    }
}

#[derive(Debug)]
pub struct DaemonSyncControlReceiver {
    receiver: mpsc::Receiver<DaemonSyncControlRequest>,
}

impl DaemonSyncControlReceiver {
    pub fn try_recv(&self) -> Result<DaemonSyncControlRequest, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn recv_timeout(
        &self,
        timeout: Duration,
    ) -> Result<DaemonSyncControlRequest, mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonSyncControlError {
    message: String,
}

impl DaemonSyncControlError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl core::fmt::Display for DaemonSyncControlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DaemonSyncControlError {}

#[derive(Clone)]
pub struct DaemonSyncControl {
    backend: DaemonSyncControlBackend,
}

#[derive(Clone)]
enum DaemonSyncControlBackend {
    Channel {
        sender: mpsc::Sender<DaemonSyncControlRequest>,
        response_timeout: Duration,
    },
    Store {
        store: FjallNodeStore,
        persist_mode: PersistMode,
    },
}

impl DaemonSyncControl {
    pub fn store_backed(store: FjallNodeStore, persist_mode: PersistMode) -> Self {
        Self {
            backend: DaemonSyncControlBackend::Store {
                store,
                persist_mode,
            },
        }
    }

    pub fn channel() -> (Self, DaemonSyncControlReceiver) {
        let (sender, receiver) = mpsc::channel();
        (
            Self {
                backend: DaemonSyncControlBackend::Channel {
                    sender,
                    response_timeout: Duration::from_secs(30),
                },
            },
            DaemonSyncControlReceiver { receiver },
        )
    }

    pub fn status(&self) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        self.request(DaemonSyncControlAction::Status)
    }

    pub fn pause(&self) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        self.request(DaemonSyncControlAction::Pause)
    }

    pub fn resume(&self) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        self.request(DaemonSyncControlAction::Resume)
    }

    fn request(
        &self,
        action: DaemonSyncControlAction,
    ) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        match &self.backend {
            DaemonSyncControlBackend::Channel {
                sender,
                response_timeout,
            } => request_daemon_sync_channel(sender, *response_timeout, action),
            DaemonSyncControlBackend::Store {
                store,
                persist_mode,
            } => request_daemon_sync_store(store, *persist_mode, action),
        }
    }
}

fn request_daemon_sync_channel(
    sender: &mpsc::Sender<DaemonSyncControlRequest>,
    response_timeout: Duration,
    action: DaemonSyncControlAction,
) -> Result<RuntimeMetadata, DaemonSyncControlError> {
    let (response_sender, response_receiver) = mpsc::channel();
    sender
        .send(DaemonSyncControlRequest {
            action,
            response_sender,
        })
        .map_err(|_| DaemonSyncControlError::new("daemon sync control is unavailable"))?;
    response_receiver
        .recv_timeout(response_timeout)
        .map_err(|_| DaemonSyncControlError::new("daemon sync control timed out"))?
        .map_err(DaemonSyncControlError::new)
}

fn request_daemon_sync_store(
    store: &FjallNodeStore,
    persist_mode: PersistMode,
    action: DaemonSyncControlAction,
) -> Result<RuntimeMetadata, DaemonSyncControlError> {
    let mut metadata = load_daemon_sync_metadata(store)?;
    match action {
        DaemonSyncControlAction::Status => Ok(metadata),
        DaemonSyncControlAction::Pause => {
            metadata.sync_control.paused = true;
            save_daemon_sync_metadata(store, persist_mode, &metadata)?;
            Ok(metadata)
        }
        DaemonSyncControlAction::Resume => {
            metadata.sync_control.paused = false;
            save_daemon_sync_metadata(store, persist_mode, &metadata)?;
            Ok(metadata)
        }
    }
}

fn load_daemon_sync_metadata(
    store: &FjallNodeStore,
) -> Result<RuntimeMetadata, DaemonSyncControlError> {
    store
        .load_runtime_metadata()
        .map_err(|error| DaemonSyncControlError::new(error.to_string()))
        .map(|maybe_metadata| maybe_metadata.unwrap_or_default())
}

fn save_daemon_sync_metadata(
    store: &FjallNodeStore,
    persist_mode: PersistMode,
    metadata: &RuntimeMetadata,
) -> Result<(), DaemonSyncControlError> {
    store
        .save_runtime_metadata(metadata, persist_mode)
        .map_err(|error| DaemonSyncControlError::new(error.to_string()))
}

impl core::fmt::Debug for ManagedRpcContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let wallet_mode = match &self.wallet_state {
            WalletState::Local(_) => "local",
            WalletState::DurableNamedRegistry { .. } => "durable",
        };
        f.debug_struct("ManagedRpcContext")
            .field("chain", &self.chain)
            .field("consensus_params", &self.consensus_params)
            .field("verify_flags", &self.verify_flags)
            .field(
                "has_durable_sync_state",
                &self.maybe_durable_sync_state.is_some(),
            )
            .field(
                "has_daemon_sync_control",
                &self.maybe_daemon_sync_control.is_some(),
            )
            .field("wallet_mode", &wallet_mode)
            .finish()
    }
}

impl ManagedRpcContext {
    pub fn set_daemon_sync_control(&mut self, control: DaemonSyncControl) {
        self.maybe_daemon_sync_control = Some(control);
    }

    pub fn daemon_sync_status(&self) -> Result<RuntimeMetadata, RpcFailure> {
        self.daemon_sync_control()?
            .status()
            .map_err(daemon_sync_control_error_to_failure)
    }

    pub fn daemon_sync_pause(&self) -> Result<RuntimeMetadata, RpcFailure> {
        self.daemon_sync_control()?
            .pause()
            .map_err(daemon_sync_control_error_to_failure)
    }

    pub fn daemon_sync_resume(&self) -> Result<RuntimeMetadata, RpcFailure> {
        self.daemon_sync_control()?
            .resume()
            .map_err(daemon_sync_control_error_to_failure)
    }

    fn daemon_sync_control(&self) -> Result<&DaemonSyncControl, RpcFailure> {
        self.maybe_daemon_sync_control.as_ref().ok_or_else(|| {
            RpcFailure::new(
                RpcFailureKind::ClientNotConnected,
                Some(crate::RpcErrorDetail::new(
                    crate::RpcErrorCode::ClientNotConnected,
                    "daemon sync control is unavailable",
                )),
            )
        })
    }
}

fn daemon_sync_control_error_to_failure(error: DaemonSyncControlError) -> RpcFailure {
    RpcFailure::new(
        RpcFailureKind::ClientNotConnected,
        Some(crate::RpcErrorDetail::new(
            crate::RpcErrorCode::ClientNotConnected,
            error.to_string(),
        )),
    )
}

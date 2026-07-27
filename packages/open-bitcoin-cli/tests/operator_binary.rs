// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, PersistMode, RuntimeMetadata, WalletRegistry,
    core::wallet::{AddressNetwork, DescriptorRole, Wallet},
    status::{
        BestKnownTipSource, BestKnownTipStatus, FieldAvailability, NoProgressDiagnosis, PeerCounts,
        PeerStatus, PeerTipAgreement, PeerTipAgreementStatus, StayCurrentStatus,
        SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState,
        SyncProgress, SyncProgressSignal, SyncResourcePressure, SyncStatus, SyncStopReasonStatus,
        TipFreshnessStatus, inbound_status_unavailable,
    },
};
use serde_json::{Value, json};

#[path = "operator_binary/process_fixtures.rs"]
mod process_fixtures;
use process_fixtures::*;
#[path = "operator_binary/sync_fixtures.rs"]
mod sync_fixtures;
use sync_fixtures::*;
#[path = "operator_binary/compatibility_wallet.rs"]
mod compatibility_wallet;
#[path = "operator_binary/dashboard_migration_support.rs"]
mod dashboard_migration_support;
#[path = "operator_binary/status_sync_soak.rs"]
mod status_sync_soak;
#[path = "operator_binary/support_recovery_live_smoke.rs"]
mod support_recovery_live_smoke;
#[path = "operator_binary/support_sync_soak.rs"]
mod support_sync_soak;

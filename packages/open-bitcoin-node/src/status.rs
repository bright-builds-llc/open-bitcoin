// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared operator status snapshot contracts.

use serde::{Deserialize, Serialize};

use crate::{LogStatus, MetricsStatus};

/// Explicit availability wrapper for status fields that may not be collectible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}

impl<T> FieldAvailability<T> {
    pub const fn available(value: T) -> Self {
        Self::Available(value)
    }

    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }
}

/// Daemon runtime state used by status, service, and dashboard consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeRuntimeState {
    Running,
    Stopped,
    Starting,
    Stopping,
    Unreachable,
    Unknown,
}

/// Node process status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeStatus {
    pub state: NodeRuntimeState,
    pub version: String,
}

/// Config and datadir status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigStatus {
    pub datadir: FieldAvailability<String>,
    pub config_paths: Vec<String>,
}

/// Service manager status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub manager: FieldAvailability<String>,
    pub installed: FieldAvailability<bool>,
    pub enabled: FieldAvailability<bool>,
    pub running: FieldAvailability<bool>,
}

/// Chain tip projection for status output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainTipStatus {
    pub height: u64,
    pub block_hash: String,
}

/// Sync progress projection for status output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncProgress {
    pub header_height: u64,
    pub block_height: u64,
    pub progress_ratio: f64,
    pub messages_processed: u64,
    pub headers_received: u64,
    pub blocks_received: u64,
}

/// Sync status fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncStatus {
    pub network: FieldAvailability<String>,
    pub chain_tip: FieldAvailability<ChainTipStatus>,
    pub sync_progress: FieldAvailability<SyncProgress>,
}

/// Peer count status details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerCounts {
    pub inbound: u32,
    pub outbound: u32,
}

/// Peer status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerStatus {
    pub peer_counts: FieldAvailability<PeerCounts>,
}

/// Mempool status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolStatus {
    pub transactions: FieldAvailability<u64>,
}

/// Wallet status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletStatus {
    pub trusted_balance_sats: FieldAvailability<u64>,
    pub freshness: FieldAvailability<WalletFreshness>,
    pub scan_progress: FieldAvailability<WalletScanProgress>,
}

/// Wallet completeness state relative to the durable node tip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletFreshness {
    Fresh,
    Stale,
    Partial,
    Scanning,
}

/// Wallet rescan progress surfaced to operator status consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletScanProgress {
    pub scanned_through_height: u32,
    pub target_tip_height: u32,
}

/// Recent operator health signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthSignal {
    pub level: HealthSignalLevel,
    pub source: String,
    pub message: String,
}

/// Severity of a health signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthSignalLevel {
    Info,
    Warn,
    Error,
}

/// Build metadata displayed in status and support output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildProvenance {
    pub version: String,
    pub commit: FieldAvailability<String>,
    pub build_time: FieldAvailability<String>,
    pub target: FieldAvailability<String>,
    pub profile: FieldAvailability<String>,
}

impl BuildProvenance {
    pub fn unavailable() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            commit: FieldAvailability::unavailable("commit unavailable"),
            build_time: FieldAvailability::unavailable("build time unavailable"),
            target: FieldAvailability::unavailable("target unavailable"),
            profile: FieldAvailability::unavailable("profile unavailable"),
        }
    }
}

/// Shared status snapshot consumed by CLI, service, dashboard, and support paths.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBitcoinStatusSnapshot {
    pub node: NodeStatus,
    pub config: ConfigStatus,
    pub service: ServiceStatus,
    pub sync: SyncStatus,
    pub peers: PeerStatus,
    pub mempool: MempoolStatus,
    pub wallet: WalletStatus,
    pub logs: LogStatus,
    pub metrics: MetricsStatus,
    pub health_signals: Vec<HealthSignal>,
    pub build: BuildProvenance,
}

#[cfg(test)]
mod tests {
    use super::{
        BuildProvenance, ChainTipStatus, ConfigStatus, FieldAvailability, HealthSignal,
        HealthSignalLevel, MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
        PeerCounts, PeerStatus, ServiceStatus, SyncProgress, SyncStatus, WalletFreshness,
        WalletScanProgress, WalletStatus,
    };
    use crate::{LogStatus, MetricsStatus};

    #[test]
    fn unavailable_field_serializes_with_reason() {
        // Arrange
        let value = FieldAvailability::<String>::unavailable("node stopped");

        // Act
        let encoded = serde_json::to_value(&value).expect("availability json");

        // Assert
        assert_eq!(encoded["state"], "unavailable");
        assert_eq!(encoded["value"]["reason"], "node stopped");
    }

    #[test]
    fn unavailable_build_provenance_keeps_missing_fields_visible() {
        // Arrange / Act
        let provenance = BuildProvenance::unavailable();
        let encoded = serde_json::to_value(provenance).expect("provenance json");

        // Assert
        assert_eq!(encoded["commit"]["state"], "unavailable");
        assert_eq!(encoded["build_time"]["state"], "unavailable");
        assert_eq!(encoded["target"]["state"], "unavailable");
    }

    #[test]
    fn stopped_node_snapshot_keeps_unavailable_live_fields_explicit() {
        // Arrange / Act
        let snapshot = stopped_snapshot();
        let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

        // Assert
        assert_eq!(snapshot.node.state, NodeRuntimeState::Stopped);
        assert_eq!(encoded["sync"]["network"]["state"], "unavailable");
        assert_eq!(encoded["sync"]["chain_tip"]["state"], "unavailable");
        assert_eq!(encoded["sync"]["sync_progress"]["state"], "unavailable");
        assert_eq!(encoded["peers"]["peer_counts"]["state"], "unavailable");
        assert_eq!(encoded["mempool"]["transactions"]["state"], "unavailable");
        assert_eq!(
            encoded["wallet"]["trusted_balance_sats"]["state"],
            "unavailable"
        );
        assert_eq!(encoded["wallet"]["freshness"]["state"], "unavailable");
        assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
        assert_eq!(encoded["config"]["datadir"]["state"], "available");
        assert_eq!(encoded["logs"]["retention"]["max_files"], 14);
        assert_eq!(
            encoded["metrics"]["retention"]["sample_interval_seconds"],
            30
        );
    }

    #[test]
    fn populated_snapshot_serializes_obs_01_fields() {
        // Arrange
        let snapshot = OpenBitcoinStatusSnapshot {
            node: NodeStatus {
                state: NodeRuntimeState::Running,
                version: "0.1.0".to_string(),
            },
            config: ConfigStatus {
                datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
                config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
            },
            service: ServiceStatus {
                manager: FieldAvailability::available("launchd".to_string()),
                installed: FieldAvailability::available(true),
                enabled: FieldAvailability::available(true),
                running: FieldAvailability::available(true),
            },
            sync: SyncStatus {
                network: FieldAvailability::available("mainnet".to_string()),
                chain_tip: FieldAvailability::available(ChainTipStatus {
                    height: 840_000,
                    block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                        .to_string(),
                }),
                sync_progress: FieldAvailability::available(SyncProgress {
                    header_height: 840_001,
                    block_height: 840_000,
                    progress_ratio: 0.99,
                    messages_processed: 12,
                    headers_received: 1,
                    blocks_received: 1,
                }),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::available(PeerCounts {
                    inbound: 0,
                    outbound: 8,
                }),
            },
            mempool: MempoolStatus {
                transactions: FieldAvailability::available(12),
            },
            wallet: WalletStatus {
                trusted_balance_sats: FieldAvailability::available(25_000),
                freshness: FieldAvailability::available(WalletFreshness::Fresh),
                scan_progress: FieldAvailability::unavailable("wallet already fresh"),
            },
            logs: LogStatus::default(),
            metrics: MetricsStatus::default(),
            health_signals: vec![HealthSignal {
                level: HealthSignalLevel::Info,
                source: "status".to_string(),
                message: "node healthy".to_string(),
            }],
            build: BuildProvenance::unavailable(),
        };

        // Act
        let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

        // Assert
        assert_eq!(encoded["config"]["datadir"]["state"], "available");
        assert_eq!(
            encoded["config"]["config_paths"][0],
            "/tmp/open-bitcoin/bitcoin.conf"
        );
        assert_eq!(encoded["sync"]["network"]["value"], "mainnet");
        assert_eq!(encoded["sync"]["chain_tip"]["value"]["height"], 840_000);
        assert_eq!(
            encoded["sync"]["sync_progress"]["value"]["header_height"],
            840_001
        );
        assert_eq!(encoded["peers"]["peer_counts"]["value"]["outbound"], 8);
        assert_eq!(encoded["wallet"]["freshness"]["value"], "fresh");
        assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
        assert_eq!(encoded["health_signals"][0]["message"], "node healthy");
    }

    #[test]
    fn wallet_freshness_states_serialize_distinctly_in_snapshot() {
        // Arrange
        let states = [
            (
                WalletFreshness::Fresh,
                FieldAvailability::unavailable("wallet already fresh"),
                "fresh",
            ),
            (
                WalletFreshness::Stale,
                FieldAvailability::unavailable("wallet scan not running"),
                "stale",
            ),
            (
                WalletFreshness::Partial,
                FieldAvailability::available(WalletScanProgress {
                    scanned_through_height: 40,
                    target_tip_height: 100,
                }),
                "partial",
            ),
            (
                WalletFreshness::Scanning,
                FieldAvailability::available(WalletScanProgress {
                    scanned_through_height: 60,
                    target_tip_height: 100,
                }),
                "scanning",
            ),
        ];

        // Act
        let encoded = states
            .into_iter()
            .map(|(freshness, scan_progress, expected)| {
                let mut snapshot = stopped_snapshot();
                snapshot.wallet = WalletStatus {
                    trusted_balance_sats: FieldAvailability::available(25_000),
                    freshness: FieldAvailability::available(freshness),
                    scan_progress,
                };
                let encoded = serde_json::to_value(snapshot).expect("snapshot json");
                (encoded, expected)
            })
            .collect::<Vec<_>>();

        // Assert
        assert_eq!(encoded[0].0["wallet"]["freshness"]["value"], encoded[0].1);
        assert_eq!(encoded[1].0["wallet"]["freshness"]["value"], encoded[1].1);
        assert_eq!(encoded[2].0["wallet"]["freshness"]["value"], encoded[2].1);
        assert_eq!(encoded[3].0["wallet"]["freshness"]["value"], encoded[3].1);
        assert_eq!(
            encoded[2].0["wallet"]["scan_progress"]["value"]["scanned_through_height"],
            40
        );
        assert_eq!(
            encoded[3].0["wallet"]["scan_progress"]["value"]["target_tip_height"],
            100
        );
    }

    fn stopped_snapshot() -> OpenBitcoinStatusSnapshot {
        let unavailable = "node stopped";
        OpenBitcoinStatusSnapshot {
            node: NodeStatus {
                state: NodeRuntimeState::Stopped,
                version: "0.1.0".to_string(),
            },
            config: ConfigStatus {
                datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
                config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
            },
            service: ServiceStatus {
                manager: FieldAvailability::unavailable("service manager not inspected"),
                installed: FieldAvailability::unavailable("service manager not inspected"),
                enabled: FieldAvailability::unavailable("service manager not inspected"),
                running: FieldAvailability::unavailable("service manager not inspected"),
            },
            sync: SyncStatus {
                network: FieldAvailability::unavailable(unavailable),
                chain_tip: FieldAvailability::unavailable(unavailable),
                sync_progress: FieldAvailability::unavailable(unavailable),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::unavailable(unavailable),
            },
            mempool: MempoolStatus {
                transactions: FieldAvailability::unavailable(unavailable),
            },
            wallet: WalletStatus {
                trusted_balance_sats: FieldAvailability::unavailable(unavailable),
                freshness: FieldAvailability::unavailable(unavailable),
                scan_progress: FieldAvailability::unavailable(unavailable),
            },
            logs: LogStatus::default(),
            metrics: MetricsStatus::default(),
            health_signals: Vec::new(),
            build: BuildProvenance::unavailable(),
        }
    }
}

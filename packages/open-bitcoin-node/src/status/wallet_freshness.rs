// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Wallet freshness facts moved out of the status snapshot root.

use serde::{Deserialize, Serialize};

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

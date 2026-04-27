use open_bitcoin_node::status::{
    FieldAvailability, WalletFreshness, WalletScanProgress, WalletStatus,
};
use open_bitcoin_rpc::method::{GetBlockchainInfoResponse, GetWalletInfoResponse};

use super::saturating_u64_to_u32;

pub(super) fn live_wallet_status(
    wallet_info: &GetWalletInfoResponse,
    blockchain_info: &GetBlockchainInfoResponse,
    trusted_balance_sats: u64,
) -> WalletStatus {
    let chain_tip_height = u64::from(blockchain_info.blocks);
    let maybe_wallet_tip_height = wallet_info.maybe_tip_height.map(u64::from);
    let freshness = wallet_freshness(maybe_wallet_tip_height, chain_tip_height);
    let scan_progress = wallet_scan_progress(freshness, maybe_wallet_tip_height, chain_tip_height);

    WalletStatus {
        trusted_balance_sats: FieldAvailability::available(trusted_balance_sats),
        freshness: FieldAvailability::available(freshness),
        scan_progress,
    }
}

fn wallet_freshness(
    maybe_wallet_tip_height: Option<u64>,
    chain_tip_height: u64,
) -> WalletFreshness {
    let Some(wallet_tip_height) = maybe_wallet_tip_height else {
        return WalletFreshness::Partial;
    };
    if wallet_tip_height >= chain_tip_height {
        return WalletFreshness::Fresh;
    }
    WalletFreshness::Stale
}

fn wallet_scan_progress(
    freshness: WalletFreshness,
    maybe_wallet_tip_height: Option<u64>,
    chain_tip_height: u64,
) -> FieldAvailability<WalletScanProgress> {
    match freshness {
        WalletFreshness::Fresh => FieldAvailability::unavailable("wallet already fresh"),
        WalletFreshness::Stale => FieldAvailability::unavailable("wallet scan not running"),
        WalletFreshness::Partial | WalletFreshness::Scanning => {
            let Some(wallet_tip_height) = maybe_wallet_tip_height else {
                return FieldAvailability::unavailable("wallet tip unavailable");
            };
            FieldAvailability::available(WalletScanProgress {
                scanned_through_height: saturating_u64_to_u32(wallet_tip_height),
                target_tip_height: saturating_u64_to_u32(chain_tip_height),
            })
        }
    }
}

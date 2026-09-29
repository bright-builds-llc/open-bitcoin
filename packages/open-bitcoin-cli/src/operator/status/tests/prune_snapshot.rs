// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn stopped_config_prune_550_unavailable_not_height_zero() {
    let mut with_target = status_input(Vec::new());
    with_target.request.include_live_rpc = false;
    let config = open_bitcoin_rpc::config::OpenBitcoinConfig {
        prune: 550,
        ..open_bitcoin_rpc::config::OpenBitcoinConfig::default()
    };
    with_target.config_resolution.maybe_open_bitcoin_config = Some(config);

    let snapshot = collect_status_snapshot(&with_target, None);

    assert!(snapshot.prune.pruned);
    assert_eq!(snapshot.prune.maybe_automatic_pruning, Some(true));
    assert_eq!(snapshot.prune.maybe_prune_target_size, Some(576_716_800));
    assert!(matches!(
        snapshot.prune.pruneheight,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(!matches!(
        snapshot.prune.pruneheight,
        FieldAvailability::Available(Some(0))
    ));
    assert!(matches!(
        snapshot.prune.locks,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.prune.manual_prune,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.prune.support_counts,
        FieldAvailability::Unavailable { .. }
    ));

    let mut missing = status_input(Vec::new());
    missing.request.include_live_rpc = false;
    missing.config_resolution.maybe_open_bitcoin_config = None;
    let missing_snapshot = collect_status_snapshot(&missing, None);
    assert!(!missing_snapshot.prune.pruned);
    assert_eq!(missing_snapshot.prune.maybe_automatic_pruning, None);
    assert_eq!(missing_snapshot.prune.maybe_prune_target_size, None);

    let mut disabled = status_input(Vec::new());
    disabled.request.include_live_rpc = false;
    let zero = open_bitcoin_rpc::config::OpenBitcoinConfig {
        prune: 0,
        ..open_bitcoin_rpc::config::OpenBitcoinConfig::default()
    };
    disabled.config_resolution.maybe_open_bitcoin_config = Some(zero);
    let disabled_snapshot = collect_status_snapshot(&disabled, None);
    assert!(!disabled_snapshot.prune.pruned);
    assert_eq!(disabled_snapshot.prune.maybe_automatic_pruning, None);
    assert_eq!(disabled_snapshot.prune.maybe_prune_target_size, None);
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid};
use open_bitcoin_primitives::{BlockHash, TransactionInput};

use super::{build_block, sample_chainstate_snapshot, spend_transaction, submit};
use crate::{
    AdmissionContext, BlockLifecycleContext, FeeRate, Mempool, MempoolAcceptanceTime,
    MempoolCapacity, MempoolEntryMetadata, MempoolOrigin, PolicyConfig, PolicyTime,
    ROLLING_FEE_HALFLIFE_SECONDS, RelayIntent, ReorgLifecycleContext, RollingMempoolFeeRate,
    recompute_resource_ledger,
};

const TEST_EXPIRY_HOURS: u64 = 1;
const SECONDS_PER_HOUR: i64 = 3_600;

fn submit_with_acceptance(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: open_bitcoin_primitives::Transaction,
    context: AdmissionContext,
) {
    mempool
        .accept_transaction_transition_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            context,
        )
        .expect("transition outcome");
}

fn known_context(accepted_at: PolicyTime) -> AdmissionContext {
    AdmissionContext::new(MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(accepted_at),
        MempoolOrigin::Local,
        RelayIntent::NotRequested,
    ))
}

fn assert_ledger_matches_oracle(mempool: &Mempool) {
    let oracle = recompute_resource_ledger(mempool.entries(), &mempool.spent_outpoints)
        .expect("oracle recomputation");
    assert_eq!(
        mempool.resource_ledger(),
        oracle,
        "cached resource ledger must match recompute_resource_ledger after committed transition"
    );
}

fn assert_no_dangling_children(mempool: &Mempool) {
    for entry in mempool.entries().values() {
        for parent_txid in &entry.parents {
            assert!(
                mempool.entry(parent_txid).is_some(),
                "dangling child retained after sustained-pressure transition"
            );
        }
        for child_txid in &entry.children {
            assert!(
                mempool.entry(child_txid).is_some(),
                "stale child index after sustained-pressure transition"
            );
        }
    }
}

fn assert_oracle_and_membership(mempool: &Mempool, expected_rolling: RollingMempoolFeeRate) {
    assert_ledger_matches_oracle(mempool);
    assert_no_dangling_children(mempool);
    assert_eq!(
        mempool.rolling_mempool_fee_rate(),
        expected_rolling,
        "rolling fee must match state-machine expectation after committed transition"
    );
}

fn two_entry_capacity(
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    coinbase_a: open_bitcoin_primitives::Txid,
    coinbase_b: open_bitcoin_primitives::Txid,
) -> usize {
    let mut probe = Mempool::default();
    submit(
        &mut probe,
        snapshot,
        spend_transaction(coinbase_a, 0, 499_999_000, TransactionInput::SEQUENCE_FINAL),
    )
    .expect("probe a");
    submit(
        &mut probe,
        snapshot,
        spend_transaction(coinbase_b, 0, 499_998_000, TransactionInput::SEQUENCE_FINAL),
    )
    .expect("probe b");
    probe.accounted_memory().as_usize()
}

#[test]
fn sustained_pressure_oracle_agrees_across_fill_trim_block_decay_expiry_refill_reorg() {
    // Arrange — hermetic PolicyTime / BlockLifecycleContext / ReorgLifecycleContext only.
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(6);
    let capacity_bytes = two_entry_capacity(&snapshot, coinbase_txids[4], coinbase_txids[5]);
    let accept_base = PolicyTime::new(1_000);
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(capacity_bytes),
        mempool_expiry_hours: TEST_EXPIRY_HOURS,
        ..PolicyConfig::default()
    });

    let keep = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_100,
        TransactionInput::SEQUENCE_FINAL,
    );
    let keep_txid = transaction_txid(&keep).expect("keep txid");
    let victim = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_400,
        TransactionInput::SEQUENCE_FINAL,
    );
    let victim_txid = transaction_txid(&victim).expect("victim txid");
    let pressure = spend_transaction(
        coinbase_txids[2],
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let pressure_txid = transaction_txid(&pressure).expect("pressure txid");

    // Act / Assert — 1) fill under small MempoolCapacity until trim fires (Pressure + bump)
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        keep.clone(),
        known_context(accept_base),
    );
    assert_oracle_and_membership(&mempool, RollingMempoolFeeRate::ZERO);

    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        victim,
        known_context(PolicyTime::new(accept_base.unix_seconds() + 10)),
    );
    assert_oracle_and_membership(&mempool, RollingMempoolFeeRate::ZERO);

    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        pressure.clone(),
        known_context(PolicyTime::new(accept_base.unix_seconds() + 20)),
    );
    assert!(
        mempool.entry(&victim_txid).is_none(),
        "lowest descendant-score package must be trimmed under accounted capacity"
    );
    assert!(mempool.entry(&keep_txid).is_some());
    assert!(mempool.entry(&pressure_txid).is_some());
    let rolling_after_trim = mempool.rolling_mempool_fee_rate();
    assert!(
        rolling_after_trim.fee_rate().sats_per_kvb() > 0,
        "pressure package removal must bump rolling floor"
    );
    assert_oracle_and_membership(&mempool, rolling_after_trim);

    // 2) connect block — remove confirmed pressure tx and open decay gate
    let connected_at = PolicyTime::new(accept_base.unix_seconds() + 100);
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 6, 499_999_000);
    block.transactions.push(pressure);
    mempool
        .remove_for_connected_block_transition(&block, BlockLifecycleContext::new(connected_at, 6))
        .expect("connected block lifecycle");
    assert!(mempool.entry(&pressure_txid).is_none());
    assert!(mempool.entry(&keep_txid).is_some());
    // Gate opened; no decay until PolicyTime advances past the update interval.
    assert_oracle_and_membership(&mempool, rolling_after_trim);

    // 3) advance PolicyTime → decay
    let after_halflife =
        PolicyTime::new(connected_at.unix_seconds() + ROLLING_FEE_HALFLIFE_SECONDS);
    let rolling_after_decay = mempool
        .materialize_rolling_fee_rate(after_halflife)
        .expect("revision remains available");
    let expected_after_decay = FeeRate::from_sats_per_kvb(
        (rolling_after_trim.fee_rate().sats_per_kvb() as f64 / 2.0).round() as i64,
    );
    assert_eq!(
        rolling_after_decay.fee_rate().sats_per_kvb(),
        expected_after_decay.sats_per_kvb(),
        "one default half-life at mid/high occupancy should halve the rolling floor"
    );
    assert_oracle_and_membership(&mempool, rolling_after_decay);

    // 4) expire aged Known entries (keep accepted at accept_base)
    let expire_now = PolicyTime::new(
        accept_base.unix_seconds() + TEST_EXPIRY_HOURS as i64 * SECONDS_PER_HOUR + 1,
    );
    mempool.expire(expire_now).expect("expire sweep");
    assert!(mempool.entry(&keep_txid).is_none());
    assert_oracle_and_membership(&mempool, rolling_after_decay);

    // 5) refill / admit replacements
    let refill = spend_transaction(
        coinbase_txids[3],
        0,
        499_996_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let refill_txid = transaction_txid(&refill).expect("refill txid");
    let refill_at = PolicyTime::new(expire_now.unix_seconds() + 5);
    submit_with_acceptance(&mut mempool, &snapshot, refill, known_context(refill_at));
    assert!(mempool.entry(&refill_txid).is_some());
    assert_oracle_and_membership(&mempool, rolling_after_decay);

    // 6) reorg lifecycle with explicit ReorgLifecycleContext time
    let reorg_context = ReorgLifecycleContext::new(PolicyTime::new(refill_at.unix_seconds() + 30));
    let reorg_tx = spend_transaction(
        coinbase_txids[4],
        0,
        499_995_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let reorg_txid = transaction_txid(&reorg_tx).expect("reorg txid");
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        reorg_tx,
        AdmissionContext::reorg(reorg_context.occurred_at),
    );
    assert!(mempool.entry(&reorg_txid).is_some());
    let reorg_entry = mempool.entry(&reorg_txid).expect("reorg entry");
    assert_eq!(reorg_entry.metadata.origin, MempoolOrigin::Reorg);
    assert_eq!(
        reorg_entry.metadata.accepted_at,
        MempoolAcceptanceTime::Known(reorg_context.occurred_at)
    );
    assert_oracle_and_membership(&mempool, rolling_after_decay);
}

#[test]
fn rolling_fee_restarts_at_zero_without_durability() {
    // Arrange — bump rolling via pressure, then simulate restart without persistence (D-15).
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let mut probe = Mempool::default();
    submit(
        &mut probe,
        &snapshot,
        spend_transaction(
            coinbase_txids[2],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("probe");
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(probe.accounted_memory().as_usize()),
        ..PolicyConfig::default()
    };
    let mut mempool = Mempool::new(config.clone());
    submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            0,
            499_999_200,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("low fee");
    submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[1],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("pressure admission");
    assert!(
        mempool.rolling_mempool_fee_rate().fee_rate().sats_per_kvb() > 0,
        "precondition: in-memory bump must raise rolling before restart"
    );

    // Act — freshly constructed Mempool simulates non-durable rolling recovery.
    let restarted = Mempool::new(config);

    // Assert
    assert_eq!(
        restarted.rolling_mempool_fee_rate(),
        RollingMempoolFeeRate::ZERO
    );
    assert_ledger_matches_oracle(&restarted);
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

//! The continuous lifecycle runs the actual private maintenance loop.

use super::super::{CheckpointWait, CoinsFlushError, coins_flush_worker_loop};
use crate::tests::filter_index::phase162::fixtures::{Phase162Fixture, Phase162Handoff};
use crate::tests::filter_index::phase162::restrictions;
use crate::tests::filter_index::phase162::retention::{self, SettledIdentity};
use open_bitcoin_node::core::chainstate::{CoinsView, IndexPrefix};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);

struct FixtureCleanup(std::path::PathBuf);

impl Drop for FixtureCleanup {
    fn drop(&mut self) {
        if self.0.exists()
            && let Err(error) = std::fs::remove_dir_all(&self.0)
        {
            eprintln!("phase162 datadir cleanup failed: {error}");
        }
    }
}

struct MaintenanceHarness {
    maybe_sender: Option<mpsc::SyncSender<CheckpointWait>>,
    parked: mpsc::Receiver<Duration>,
    maybe_join: Option<thread::JoinHandle<std::io::Result<()>>>,
}

impl MaintenanceHarness {
    fn start(fixture: &Phase162Fixture) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::sync_channel(1);
        let (parked_sender, parked) = mpsc::sync_channel(1);
        let handle = fixture.opened.network.clone();
        let store = fixture.store.clone();
        let join = thread::spawn(move || {
            let mut maybe_wait_error = None;
            let result = coins_flush_worker_loop(handle, store, |duration| {
                if let Err(error) = parked_sender.try_send(duration) {
                    maybe_wait_error = Some(std::io::Error::other(error));
                    return CheckpointWait::Shutdown;
                }
                match receiver.recv() {
                    Ok(event) => event,
                    Err(_) => CheckpointWait::Shutdown,
                }
            });
            result.map_err(std::io::Error::other)?;
            if let Some(error) = maybe_wait_error {
                return Err(error);
            }
            Ok(())
        });
        let worker = Self {
            maybe_sender: Some(sender),
            parked,
            maybe_join: Some(join),
        };
        worker.wait_parked()?;
        Ok(worker)
    }

    fn wait_parked(&self) -> std::io::Result<()> {
        let duration = self
            .parked
            .recv_timeout(HANDSHAKE_TIMEOUT)
            .map_err(std::io::Error::other)?;
        if duration != Duration::from_secs(1) {
            return Err(std::io::Error::other("unexpected production tick interval"));
        }
        Ok(())
    }

    fn tick(&self) -> std::io::Result<()> {
        self.maybe_sender
            .as_ref()
            .ok_or_else(|| std::io::Error::other("settled worker"))?
            .try_send(CheckpointWait::Elapsed)
            .map_err(std::io::Error::other)?;
        self.wait_parked()
    }

    fn settle(&mut self) -> std::io::Result<()> {
        let Some(join) = self.maybe_join.take() else {
            return Ok(());
        };
        let signal = self.maybe_sender.take().map_or(Ok(()), |sender| {
            sender
                .try_send(CheckpointWait::Shutdown)
                .map_err(std::io::Error::other)
        });
        let joined = join
            .join()
            .map_err(|_| std::io::Error::other("maintenance worker panicked"))?;
        // Join is attempted even when signaling failed; disconnect also requests Always.
        match (signal, joined) {
            (Err(signal), Err(joined)) => Err(std::io::Error::other(format!(
                "{signal}; settlement: {joined}"
            ))),
            (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
            (Ok(()), Ok(())) => Ok(()),
        }
    }

    fn shutdown(mut self) -> std::io::Result<()> {
        self.settle()
    }
}

impl Drop for MaintenanceHarness {
    fn drop(&mut self) {
        if let Err(error) = self.settle() {
            eprintln!("phase162 maintenance cleanup failed: {error}");
        }
    }
}

#[tokio::test]
async fn phase162_integrated_lifecycle() {
    // Arrange: one configured owner and one independently identified accepted history.
    let lifecycle_started = Instant::now();
    // Declaration order keeps cleanup after fixture/worker destruction on unwind.
    let cleanup;
    let fixture = Phase162Fixture::seed();
    cleanup = FixtureCleanup(fixture.path.clone());
    fixture.assert_initial_prefix();
    crate::tests::filter_index::phase162::rpc::assert_initial_sync(&fixture).await;
    let wallet = restrictions::fund_wallet(&fixture).await;
    retention::protected_request(&fixture).await;
    let worker = MaintenanceHarness::start(&fixture).expect("actual parked worker");
    let started = Instant::now();
    let tip = fixture
        .opened
        .network
        .maybe_chain_tip()
        .expect("tip")
        .expect("accepted tip");
    let mut previous = 7;
    let mut completed_ticks = 0;
    // Act / Assert: a turn is complete only after the loop reaches its next wait.
    for tick in 1..=256 {
        worker.tick().expect("bounded completed elapsed turn");
        completed_ticks = tick;
        let progress = fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("same owner observation")
            .expect("enabled owner");
        assert!(progress.best_block_height > previous);
        assert!(progress.best_block_height - previous <= 8);
        assert_eq!(
            fixture.opened.network.maybe_chain_tip().expect("fixed tip"),
            Some(tip.clone())
        );
        assert!(
            fixture
                .opened
                .network
                .peer_manager_snapshot()
                .expect("zero peers")
                .peer_ids()
                .is_empty()
        );
        assert_eq!(
            fixture
                .opened
                .network
                .network_info()
                .expect("network")
                .connected_peers,
            0
        );
        assert_eq!(
            fixture
                .store
                .coins_view()
                .best_block()
                .expect("genuine coins B"),
            Some(tip.block_hash)
        );
        let owner = fixture
            .store
            .load_prune_protection()
            .expect("durable protection")
            .maybe_owner()
            .expect("durable BASIC owner");
        if let IndexPrefix::Committed(safe) = owner.checkpoint().checkpoint().prefix() {
            assert!(safe.height() <= tip.height);
            assert!(safe.height() <= progress.best_block_height);
            assert_eq!(
                safe.block_hash(),
                fixture.provenance.receipts[safe.height() as usize].block_hash
            );
        }
        previous = progress.best_block_height;
        if progress.synced {
            assert_eq!(previous, 1000);
            break;
        }
    }
    assert_eq!(
        previous, 1000,
        "finite idle catch-up must reach the actual target"
    );
    assert!(completed_ticks < 256);
    let catch_up_ms = started.elapsed().as_millis();
    let owner = fixture
        .store
        .load_prune_protection()
        .expect("safe completed owner")
        .maybe_owner()
        .expect("saved BASIC owner");
    let IndexPrefix::Committed(safe) = owner.checkpoint().checkpoint().prefix() else {
        panic!("complete catch-up must earn a durable checkpoint")
    };
    assert_eq!(safe.height(), 1000);
    assert_eq!(safe.block_hash(), tip.block_hash);
    let open_bitcoin_node::BasicFilterQuery::Found(record) = fixture
        .opened
        .network
        .basic_filter_query(tip.block_hash)
        .expect("actual completed record")
    else {
        panic!("complete catch-up must expose its immutable tip record")
    };
    let expected = &fixture.provenance.oracle["cases"][1000];
    let hex = crate::tests::filter_index::phase162::hex;
    assert_eq!(expected["encoded"], hex(record.encoded_bytes()));
    assert_eq!(
        expected["hash_raw"],
        hex(record.identity().filter_hash().as_bytes())
    );
    assert_eq!(
        expected["header_raw"],
        hex(record.identity().filter_header().as_bytes())
    );
    let stale = crate::tests::filter_index::phase162::lifecycle::extend_and_reorg(&fixture, || {
        worker.tick().expect("actual lifecycle worker tick")
    });
    let tip = fixture
        .opened
        .network
        .maybe_chain_tip()
        .expect("extended tip")
        .expect("tip");
    assert_eq!(
        tip.height, 1002,
        "accepted lifecycle must cross the h1000 checkpoint"
    );
    // Downstream plans extend this same scenario while the worker remains parked.
    worker
        .shutdown()
        .expect("Always settlement and actual join");
    assert_eq!(
        fixture
            .store
            .coins_view()
            .best_block()
            .expect("settled coins B"),
        Some(tip.block_hash)
    );
    let seed_ms = fixture.provenance.seed_ms;
    let settled = SettledIdentity::capture(&fixture, &stale);
    wallet.assert_persisted(&fixture, false);
    let wallet = restrictions::checkpoint_wallet(&fixture, 8).await;
    retention::manual(&fixture, &settled).await;
    let fixture = retention::automatic(fixture, &settled);
    wallet.assert_persisted(&fixture, false);
    crate::tests::filter_index::phase162::rpc::assert_phase162_rpc(&fixture).await;
    settled.assert_unchanged(&fixture);
    let fixture =
        crate::tests::filter_index::phase162::inbound::assert_phase162_inbound(fixture, &settled)
            .await;
    let fixture = restrictions::assert_restrictions(fixture, &settled, &wallet).await;
    let handoff = failure_settlement_and_reopen(fixture.close());
    let fixture = Phase162Fixture::open(handoff);
    wallet.assert_persisted(&fixture, true);
    settled.assert_unchanged(&fixture);
    assert!(
        open_bitcoin_node::FjallNodeStore::open(&fixture.path).is_err(),
        "all daemon owners prevent concurrent open before outbound handoff"
    );
    let path = fixture.path.clone();
    let recipe_id = fixture.provenance.recipe_id.clone();
    let input_digest = fixture.provenance.ordered_input_digest.clone();
    let final_usage = fixture
        .store
        .retained_payload_usage(
            &fixture
                .opened
                .network
                .chainstate_snapshot()
                .expect("active chain")
                .active_chain,
        )
        .expect("actual final usage")
        .current_usage_bytes;
    let allocated_file_bytes = retention::allocated_bytes(&path);
    let handoff = fixture.close();
    let (handoff, outbound) =
        crate::tests::filter_index::phase162::outbound::assert_outbound(handoff);
    assert_eq!(handoff.path, path);
    assert_eq!(handoff.provenance.recipe_id, recipe_id);
    assert_eq!(handoff.provenance.ordered_input_digest, input_digest);
    let fixture = Phase162Fixture::open(handoff);
    settled.assert_unchanged(&fixture);
    wallet.assert_persisted(&fixture, true);
    for position in fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("final active chain")
        .active_chain
    {
        retention::assert_pair(&fixture, position.block_hash, position.height >= 715);
    }
    let handoff = fixture.close();
    drop(open_bitcoin_node::FjallNodeStore::open(&handoff.path).expect("all-handle reopen"));
    std::fs::remove_dir_all(&handoff.path).expect("closed datadir cleanup");
    assert!(!handoff.path.exists());
    drop(cleanup);
    eprintln!(
        "phase162 integrated lifecycle: recipe={recipe_id} ordered_digest_verified=true initial_blocks=1001 final_active_blocks=1003 accepted_receipts=1006 maturity=100 startup_height=7 completed_ticks={completed_ticks} seed_ms={seed_ms} catch_up_ms={catch_up_ms} lifecycle_ms={} configured_startup=true idle_zero_peers=true connected=true reorg=true manual_pairs=21 automatic_pairs=694 total_absent_pairs=715 retained_pairs=288 nonactive_codec_pressure_rows=584 pressure_value_bytes=578091088 target_mib=550 final_logical_usage_bytes={final_usage} target_attained=false allocated_file_bytes={allocated_file_bytes} authenticated_rpc=true inbound_exact=true body_wallet_restrictions=true outbound_requests={} outbound_physical_frames={} outbound_physical_bytes={} outbound_ms={} outbound_joined=true peers=0 charges=0 joined=true failure_joined=true same_path=true cold_reopened=true cleanup=true",
        lifecycle_started.elapsed().as_millis(),
        outbound.requests,
        outbound.frames,
        outbound.bytes,
        outbound.elapsed_ms
    );
}

fn failure_settlement_and_reopen(handoff: Phase162Handoff) -> Phase162Handoff {
    let reopened = Phase162Fixture::open(handoff);
    let tip = reopened
        .opened
        .network
        .maybe_chain_tip()
        .expect("recovered accepted tip")
        .expect("tip");
    let progress = reopened
        .opened
        .network
        .maybe_basic_index_summary()
        .expect("actual reopened owner")
        .expect("BASIC");
    assert!(progress.synced);
    assert_eq!(progress.best_block_height, 1002);
    // The existing disable boundary invalidates this live worker's old owner.
    let failed_worker = MaintenanceHarness::start(&reopened).expect("real failure-path worker");
    reopened
        .opened
        .network
        .disable_basic_filter_index()
        .expect("actual configured-owner invalidation");
    let saved = reopened
        .store
        .load_prune_protection()
        .expect("saved disabled prefix");
    failed_worker
        .tick()
        .expect("failure turn still yields to its next wait");
    let error = failed_worker
        .shutdown()
        .expect_err("retained BASIC error must survive Always and join");
    assert!(
        matches!(
            error
                .get_ref()
                .and_then(|cause| cause.downcast_ref::<CoinsFlushError>()),
            Some(CoinsFlushError::BasicFilter(_))
        ),
        "typed loop error must survive the harness"
    );
    assert_eq!(
        reopened
            .store
            .load_prune_protection()
            .expect("preserved disabled prefix"),
        saved
    );
    assert_eq!(
        reopened
            .store
            .coins_view()
            .best_block()
            .expect("failure Always coins B"),
        Some(tip.block_hash)
    );
    let handoff = reopened.close();
    let reopened = Phase162Fixture::open(handoff);
    let progress = reopened
        .opened
        .network
        .maybe_basic_index_summary()
        .expect("reopened after failure")
        .expect("BASIC owner restored");
    assert!(progress.synced);
    assert_eq!(progress.best_block_height, 1002);
    reopened.close()
}

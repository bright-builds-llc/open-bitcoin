// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Focused physical-write faults on the shared mature recipe, without pressure rows.

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::network::filter_transport::{FilterTransportDisposition, PreparedFilterResponse};
use crate::sync::tests::filter_index::phase162::{
    AcceptedRecipe, independent_oracle, oracle_input,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const PARTIAL: usize = 31;
static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Cleanup(PathBuf);
impl Drop for Cleanup {
    fn drop(&mut self) {
        drop(FjallNodeStore::open(&self.0).expect("all owners closed before raw reopen"));
        fs::remove_dir_all(&self.0).expect("owned write-fault cleanup");
        assert!(!self.0.exists());
    }
}

fn accepted(path: &Path) -> AcceptedRecipe {
    let fixture = AcceptedRecipe::at(path.to_owned(), 102, BasicFilterStartupMode::Enabled);
    fixture.finish();
    fixture.flush();
    let progress = fixture
        .runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("earned owner")
        .expect("enabled");
    assert_eq!(
        progress
            .maybe_safe_durable_endpoint()
            .expect("earned safe endpoint")
            .height(),
        101
    );
    assert_eq!(fixture.recipe.blocks[101].transactions.len(), 3);
    fixture
}

fn frames(fixture: &AcceptedRecipe) -> Vec<Vec<u8>> {
    let oracle = independent_oracle(&oracle_input(&fixture.recipe, None));
    (99..=101)
        .map(|height| {
            let row = oracle["wire"]
                .as_array()
                .expect("independent frames")
                .iter()
                .find(|row| row["name"] == format!("active-{height}-cfilter"))
                .expect("raw-derived row");
            let hex = row["expected"]["frame_hex"]
                .as_str()
                .expect("pinned Python frame");
            (0..hex.len())
                .step_by(2)
                .map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).expect("hex byte"))
                .collect()
        })
        .collect()
}

fn prepared(fixture: &AcceptedRecipe) -> PreparedFilterResponse {
    fixture
        .runtime
        .network
        .configure_basic_peer_serving(true)
        .expect("explicit serving grant");
    fixture
        .runtime
        .network
        .connect_outbound_peer(1, 50_000)
        .expect("own advertised capability");
    super::super::filter_index::peer_fixtures::complete_peer_handshake(&fixture.runtime, 1);
    let plans = fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 99,
                stop_hash: fixture.receipts[101].block_hash,
            }),
            50_000,
            fixture.recipe.flags,
            fixture.recipe.params,
        )
        .expect("actual request dispatch");
    let Item::PreparedFilterServe(response) = plans
        .inbound_response_plan
        .into_iter()
        .next()
        .expect("genuine accepted range")
    else {
        panic!("prepared filter response")
    };
    *response
}

#[derive(Clone, Copy, Debug)]
enum Case {
    Partial(usize),
    Timeout,
    Cancel,
    Epoch,
    Disconnect,
    Complete,
}

fn run(case: Case) {
    // Arrange: the cleanup guard precedes every database/session owner, including setup failures.
    let path = std::env::temp_dir().join(format!(
        "open-bitcoin-phase162-outbound-write-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    assert!(!path.exists(), "fresh owned path");
    let cleanup = Cleanup(path);
    let fixture = accepted(&cleanup.0);
    let expected = frames(&fixture);
    let response = prepared(&fixture);
    let pending = fixture
        .runtime
        .network
        .encode_pending_network_message(1, &WireNetworkMessage::Ping { nonce: 162_009 })
        .expect("real independent generic owner");
    let (mut session, mut socket, _) = loopback();
    let cancelled = Arc::new(AtomicBool::new(false));
    let writer_cancelled = cancelled.clone();
    let handle = fixture.runtime.network.clone();
    let mut calls = 0;
    session.control_writes(move |stream, bytes| {
        calls += 1;
        let snapshot = handle
            .basic_filter_transport_snapshot()
            .expect("live actual owner");
        assert_eq!(snapshot.requests, 1);
        assert_eq!(snapshot.active_frame_bytes, bytes.len() as u64);
        assert!(snapshot.suffix_bytes >= snapshot.active_frame_bytes);
        assert_eq!(
            (snapshot.generic_descriptors, snapshot.generic_pending_bytes),
            (1, 32)
        );
        handle
            .network_info()
            .expect("authority released during physical write");
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        stream.set_write_timeout(Some(
            deadline.saturating_duration_since(std::time::Instant::now()),
        ))?;
        match case {
            Case::Partial(position) if position == calls => {
                stream.write_all(&bytes[..PARTIAL])?;
                Err(io::Error::new(
                    io::ErrorKind::ConnectionReset,
                    "physical partial frame",
                ))
            }
            Case::Timeout => {
                stream.write_all(&bytes[..PARTIAL])?;
                Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "physical prefix then failed deadline",
                ))
            }
            Case::Cancel => {
                stream.write_all(bytes)?;
                writer_cancelled.store(true, Ordering::Release);
                Ok(())
            }
            Case::Epoch => {
                handle
                    .configure_basic_peer_serving(false)
                    .expect("actual epoch invalidation");
                handle
                    .configure_basic_peer_serving(true)
                    .expect("new grant cannot revive handed range");
                stream.write_all(bytes)
            }
            Case::Disconnect => {
                stream.shutdown(std::net::Shutdown::Write)?;
                stream.write_all(bytes)
            }
            _ => stream.write_all(bytes),
        }
    });
    // Act: the production executor alone acknowledges actual writer outcomes.
    let (result, prefix) = super::super::super::session::filter_serving::send_response(
        &fixture.runtime.network,
        &mut session,
        1,
        fixture.runtime.config.network.magic(),
        response,
        &mut || cancelled.load(Ordering::Acquire),
    );
    drop(session);
    let mut physical = Vec::new();
    let read = socket.read_to_end(&mut physical);
    drop(socket);
    let retained = fixture
        .runtime
        .network
        .basic_filter_transport_snapshot()
        .expect("independent generic owner retained");
    assert_eq!(
        (
            retained.requests,
            retained.active_frame_bytes,
            retained.suffix_bytes
        ),
        (0, 0, 0)
    );
    assert_eq!(
        (retained.generic_descriptors, retained.generic_pending_bytes),
        (1, 32)
    );
    drop(pending);
    fixture
        .runtime
        .network
        .disconnect_peer(1)
        .expect("actual peer lifecycle cleanup");
    let settled = fixture
        .runtime
        .network
        .basic_filter_transport_snapshot()
        .expect("settled ledger");
    // Assert: compare the entire wire prefix, including header and incomplete payload.
    read.expect("finite physical EOF");
    let prefix = prefix.expect("private complete-frame receipt");
    let complete = match case {
        Case::Complete => 3,
        Case::Partial(2) | Case::Cancel | Case::Epoch => 1,
        _ => 0,
    };
    let mut wanted: Vec<u8> = expected[..complete].concat();
    if matches!(case, Case::Partial(_) | Case::Timeout) {
        wanted.extend_from_slice(&expected[complete][..PARTIAL]);
    }
    assert_eq!(physical, wanted, "{case:?}: exact independent wire prefix");
    assert_eq!(prefix.progress.messages, complete as u64);
    assert_eq!(
        prefix.progress.bytes,
        expected[..complete].concat().len() as u64
    );
    assert_eq!(
        prefix.disposition == FilterTransportDisposition::CurrentRangeComplete,
        matches!(case, Case::Complete)
    );
    if matches!(case, Case::Cancel | Case::Epoch) {
        assert_eq!(
            prefix.disposition,
            FilterTransportDisposition::PhysicalPrefixInvalidated
        );
    }
    assert_eq!(
        result.is_ok(),
        matches!(case, Case::Cancel | Case::Complete),
        "{case:?}: {result:?}"
    );
    if matches!(case, Case::Partial(_) | Case::Timeout | Case::Disconnect) {
        assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    }
    assert_eq!(settled, FilterTransportSnapshot::default());
    eprintln!(
        "phase162 outbound {case:?}: physical={} complete_frames={complete} complete_bytes={} current_range={} resources=0",
        physical.len(),
        prefix.progress.bytes,
        matches!(case, Case::Complete)
    );
    drop(fixture);
    drop(cleanup);
}

#[test]
fn phase162_outbound_write_partial_first() {
    run(Case::Partial(1));
}
#[test]
fn phase162_outbound_write_partial_second() {
    run(Case::Partial(2));
}
#[test]
fn phase162_outbound_write_failed_deadline() {
    run(Case::Timeout);
}
#[test]
fn phase162_outbound_write_cancel_handed_frame() {
    run(Case::Cancel);
}
#[test]
fn phase162_outbound_write_epoch_handed_frame() {
    run(Case::Epoch);
}
#[test]
fn phase162_outbound_write_disconnect() {
    run(Case::Disconnect);
}
#[test]
fn phase162_outbound_write_complete_control() {
    run(Case::Complete);
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use std::{
    sync::{Arc, Barrier},
    thread,
};

use open_bitcoin_network::{
    BlockRelayActivationPolicy, PHASE94_MAX_PEER_QUEUED_MESSAGES, RelayActivationConfig,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    ManagedNetworkHandle, PeerIdentityAuthority, SyncRuntimeError,
    core::primitives::{Block, NetworkMagic},
    network::{
        AnnouncementPreparationOutcome, EffectAbort, EffectCompletion, PeerEmission,
        PeerEmissionReceipt, PeerEmissionWriteCapability, PeerOutboxSnapshot,
    },
    sync::AnnouncementOutboxRegistry,
};

use super::*;

const PEER_ID: u64 = 134_091;
const OTHER_PEER_ID: u64 = 134_092;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScriptedFailure {
    Encode,
    Rejected,
    Disconnected,
    Write,
}

struct ScriptedInboundExecutor {
    network: ManagedNetworkHandle,
    failure: ScriptedFailure,
    failure_on_call: usize,
    encode_calls: usize,
    write_calls: usize,
    completion_calls: Vec<EffectCompletion>,
    abort_calls: Vec<EffectAbort>,
    abort_attempts: usize,
    abort_should_fail: bool,
}

impl InboundEmissionExecutor for ScriptedInboundExecutor {
    fn encode(&mut self, message: &WireNetworkMessage) -> Result<Vec<u8>, ()> {
        self.encode_calls = self.encode_calls.saturating_add(1);
        if self.failure == ScriptedFailure::Encode && self.encode_calls == self.failure_on_call {
            return Err(());
        }
        message
            .encode_wire(NetworkMagic::MAINNET)
            .map_err(|_error| ())
    }

    async fn write(&mut self, _bytes: &[u8]) -> InboundEmissionWriteResult {
        self.write_calls = self.write_calls.saturating_add(1);
        if self.write_calls != self.failure_on_call {
            return InboundEmissionWriteResult::Written;
        }
        match self.failure {
            ScriptedFailure::Encode => InboundEmissionWriteResult::Written,
            ScriptedFailure::Rejected => InboundEmissionWriteResult::Rejected,
            ScriptedFailure::Disconnected => InboundEmissionWriteResult::Disconnected,
            ScriptedFailure::Write => InboundEmissionWriteResult::Failed,
        }
    }

    fn complete(&mut self, receipt: PeerEmissionReceipt) -> Result<(), ()> {
        let completion = self
            .network
            .complete_peer_emission(receipt)
            .map_err(|_error| ())?;
        self.completion_calls.push(completion);
        Ok(())
    }

    fn abort(&mut self, capability: PeerEmissionWriteCapability) -> Result<EffectAbort, ()> {
        self.abort_attempts = self.abort_attempts.saturating_add(1);
        if self.abort_should_fail {
            return Err(());
        }
        let abort = self
            .network
            .abort_peer_emission(capability)
            .map_err(|_error| ())?;
        self.abort_calls.push(abort);
        Ok(abort)
    }
}

fn network_fixture() -> ManagedNetworkHandle {
    let network = ManagedNetworkHandle::transient_runtime(
        NetworkMagic::MAINNET,
        8_333,
        RelayActivationConfig::default(),
        BlockRelayActivationPolicy::default(),
        true,
    );
    let mut context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &RuntimeConfig::default(),
        network.clone(),
        None,
    )
    .expect("compose scripted RPC network");
    for peer_id in [PEER_ID, OTHER_PEER_ID] {
        context
            .connect_outbound_peer(peer_id, 1)
            .expect("connect scripted peer");
        for message in [
            WireNetworkMessage::Version(VersionMessage::default()),
            WireNetworkMessage::Verack,
        ] {
            context
                .receive_network_message(peer_id, message, 1)
                .expect("complete scripted handshake");
        }
    }
    network
}

fn scripted_emissions(network: &ManagedNetworkHandle, peer_id: u64) -> Vec<PeerEmission> {
    let snapshots = [PeerOutboxSnapshot::new(
        peer_id,
        0,
        PHASE94_MAX_PEER_QUEUED_MESSAGES,
    )];
    (0..3)
        .map(|_| {
            let outcomes = network
                .prepare_block_announcements(&Block::default(), &snapshots)
                .expect("prepare scripted block announcement");
            match outcomes
                .into_iter()
                .find(|outcome| outcome.peer_id() == peer_id)
                .expect("one scripted target-peer outcome")
            {
                AnnouncementPreparationOutcome::Ready(emission) => *emission,
                other => panic!("scripted peer should produce a ready emission: {other:?}"),
            }
        })
        .collect()
}

async fn run_script(
    failure: ScriptedFailure,
    failure_on_call: usize,
) -> (InboundEmissionExecutionOutcome, ScriptedInboundExecutor) {
    let network = network_fixture();
    let emissions = scripted_emissions(&network, PEER_ID);
    let mut executor = ScriptedInboundExecutor {
        network,
        failure,
        failure_on_call,
        encode_calls: 0,
        write_calls: 0,
        completion_calls: Vec::new(),
        abort_calls: Vec::new(),
        abort_attempts: 0,
        abort_should_fail: false,
    };
    let outcome = execute_inbound_emissions(emissions, PEER_ID, &mut executor).await;
    (outcome, executor)
}

fn assert_pending_capacity_recovered(network: &ManagedNetworkHandle, peer_id: u64) {
    for _ in 0..PHASE94_MAX_PEER_QUEUED_MESSAGES {
        network
            .prepare_peer_relay_effect(peer_id)
            .expect("all pending peer slots should be reusable after fanout failure");
    }
}

fn assert_one_achieved_prefix(executor: &ScriptedInboundExecutor) {
    assert_eq!(executor.completion_calls, [EffectCompletion::Applied]);
    let evidence = serde_json::to_value(
        executor
            .network
            .block_relay_evidence_status()
            .expect("block relay evidence"),
    )
    .expect("serialize block relay evidence");
    assert_eq!(
        evidence["announcement"]["value"]["compact_inventory_fallback_count"],
        1
    );
}

#[tokio::test]
async fn phase134_rpc_successful_prefix_encode_failure_stops_before_second_write() {
    // Arrange
    let failure = ScriptedFailure::Encode;

    // Act
    let (outcome, executor) = run_script(failure, 2).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::EncodeFailed);
    assert_eq!(executor.encode_calls, 2);
    assert_eq!(executor.write_calls, 1);
    assert_one_achieved_prefix(&executor);
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted; 2]);
    assert_pending_capacity_recovered(&executor.network, PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_successful_prefix_rejection_stops_before_third_command() {
    // Arrange
    let failure = ScriptedFailure::Rejected;

    // Act
    let (outcome, executor) = run_script(failure, 2).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::Rejected);
    assert_eq!(executor.encode_calls, 2);
    assert_eq!(executor.write_calls, 2);
    assert_one_achieved_prefix(&executor);
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted; 2]);
    assert_pending_capacity_recovered(&executor.network, PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_successful_prefix_disconnect_stops_before_third_command() {
    // Arrange
    let failure = ScriptedFailure::Disconnected;

    // Act
    let (outcome, executor) = run_script(failure, 2).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::Disconnected);
    assert_eq!(executor.encode_calls, 2);
    assert_eq!(executor.write_calls, 2);
    assert_one_achieved_prefix(&executor);
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted; 2]);
    assert_pending_capacity_recovered(&executor.network, PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_successful_prefix_write_failure_stops_before_third_command() {
    // Arrange
    let failure = ScriptedFailure::Write;

    // Act
    let (outcome, executor) = run_script(failure, 2).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::WriteFailed);
    assert_eq!(executor.encode_calls, 2);
    assert_eq!(executor.write_calls, 2);
    assert_one_achieved_prefix(&executor);
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted; 2]);
    assert_pending_capacity_recovered(&executor.network, PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_first_write_failure_aborts_the_entire_batch() {
    // Arrange
    let failure = ScriptedFailure::Write;

    // Act
    let (outcome, executor) = run_script(failure, 1).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::WriteFailed);
    assert!(executor.completion_calls.is_empty());
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted; 3]);
    assert_pending_capacity_recovered(&executor.network, PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_last_write_failure_keeps_two_completed_prefix_items() {
    // Arrange
    let failure = ScriptedFailure::Write;

    // Act
    let (outcome, executor) = run_script(failure, 3).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::WriteFailed);
    assert_eq!(
        executor.completion_calls,
        [EffectCompletion::Applied, EffectCompletion::Applied]
    );
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted]);
    assert_pending_capacity_recovered(&executor.network, PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_target_mismatch_aborts_current_and_unsent_suffix() {
    // Arrange
    let network = network_fixture();
    let emissions = scripted_emissions(&network, OTHER_PEER_ID);
    let mut executor = ScriptedInboundExecutor {
        network,
        failure: ScriptedFailure::Write,
        failure_on_call: 1,
        encode_calls: 0,
        write_calls: 0,
        completion_calls: Vec::new(),
        abort_calls: Vec::new(),
        abort_attempts: 0,
        abort_should_fail: false,
    };

    // Act
    let outcome = execute_inbound_emissions(emissions, PEER_ID, &mut executor).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::TargetMismatch);
    assert_eq!(executor.encode_calls, 0);
    assert_eq!(executor.write_calls, 0);
    assert_eq!(executor.abort_calls, [EffectAbort::Aborted; 3]);
    assert_pending_capacity_recovered(&executor.network, OTHER_PEER_ID);
}

#[tokio::test]
async fn phase134_rpc_abort_failure_is_visible_and_attempts_every_suffix_item() {
    // Arrange
    let network = network_fixture();
    let emissions = scripted_emissions(&network, OTHER_PEER_ID);
    let mut executor = ScriptedInboundExecutor {
        network,
        failure: ScriptedFailure::Write,
        failure_on_call: 1,
        encode_calls: 0,
        write_calls: 0,
        completion_calls: Vec::new(),
        abort_calls: Vec::new(),
        abort_attempts: 0,
        abort_should_fail: true,
    };

    // Act
    let outcome = execute_inbound_emissions(emissions, PEER_ID, &mut executor).await;

    // Assert
    assert_eq!(outcome, InboundEmissionExecutionOutcome::AbortFailed);
    assert_eq!(executor.abort_attempts, 3);
}

#[test]
fn concurrent_inbound_and_outbound_sessions_have_distinct_scoped_outboxes() {
    // Arrange
    let authority = PeerIdentityAuthority::default();
    let network = network_fixture();
    let outboxes = AnnouncementOutboxRegistry::default();
    let barrier = Arc::new(Barrier::new(3));
    let inbound = spawn_registered_peer(&authority, &outboxes, &barrier);
    let outbound = spawn_registered_peer(&authority, &outboxes, &barrier);

    // Act
    barrier.wait();
    let inbound_peer_id = inbound.join().expect("join inbound allocation");
    let outbound_peer_id = outbound.join().expect("join outbound allocation");
    let duplicate_error = match outboxes.register_peer(outbound_peer_id) {
        Ok(_notification) => panic!("duplicate live peer registration must fail"),
        Err(error) => error,
    };
    outboxes
        .unregister_peer(&network, inbound_peer_id)
        .expect("unregister inbound peer");
    let snapshots = outboxes.snapshots().expect("outbox snapshots");

    // Assert
    assert_ne!(inbound_peer_id, outbound_peer_id);
    assert!(matches!(
        duplicate_error,
        SyncRuntimeError::Network { message } if message.contains("already registered")
    ));
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].peer_id(), outbound_peer_id);
    assert_eq!(snapshots[0].queued_messages(), 0);
}

fn spawn_registered_peer(
    authority: &PeerIdentityAuthority,
    outboxes: &AnnouncementOutboxRegistry,
    barrier: &Arc<Barrier>,
) -> thread::JoinHandle<u64> {
    let authority = authority.clone();
    let outboxes = outboxes.clone();
    let barrier = Arc::clone(barrier);
    thread::spawn(move || {
        barrier.wait();
        let peer_id = authority.allocate().expect("allocate peer");
        outboxes.register_peer(peer_id).expect("register peer");
        peer_id
    })
}

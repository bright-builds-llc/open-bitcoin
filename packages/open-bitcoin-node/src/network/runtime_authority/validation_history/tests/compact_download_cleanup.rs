// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp

use super::*;
use open_bitcoin_core::consensus::ScriptVerifyFlags;
use open_bitcoin_network::{VersionMessage, WireNetworkMessage};

fn block_row_key(hash: BlockHash) -> String {
    let mut key = String::from("block:");
    for byte in hash.as_bytes() {
        use std::fmt::Write;
        write!(key, "{byte:02x}").expect("infallible string write");
    }
    key
}

fn warm_accepted(command: &CanonicalSubmissionCommand, block: &Block) {
    accepted(command.submit(wire(block)).expect("genuine accepted body"));
    let hash = block_hash(&block.header);
    assert_eq!(
        command.store.load_block(hash).expect("retained row"),
        Some(block.clone())
    );
    command
        .network
        .read(|network| {
            assert_eq!(network.blocks_by_hash.get(&hash), Some(block));
            let facts = network
                .peer_manager()
                .header_store()
                .maybe_body_eligibility_binding(hash)
                .expect("fresh linkage")
                .facts();
            assert!(facts.available && !facts.failed);
        })
        .expect("warm accepted cache");
}

fn request_known_compact(command: &CanonicalSubmissionCommand, block: &Block, peer: u64) {
    let hash = block_hash(&block.header);
    command
        .network
        .mutate(|network| {
            network
                .peer_manager_mut()
                .set_block_relay_activation_policy(
                    open_bitcoin_network::BlockRelayActivationPolicy {
                        compact_relay: open_bitcoin_network::CompactRelayActivationConfig {
                            enabled: true,
                        },
                        ..Default::default()
                    },
                );
            network
                .connect_outbound_peer(peer, 0)
                .expect("fresh compact peer");
            for message in [
                WireNetworkMessage::Version(VersionMessage::default()),
                WireNetworkMessage::Verack,
                WireNetworkMessage::SendCompact(open_bitcoin_codec::SendCompactMessage {
                    announce: true,
                    version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
                }),
            ] {
                network
                    .receive_message(peer, message, 1, ScriptVerifyFlags::NONE, command.params)
                    .expect("real compact handshake");
            }
            let selector =
                open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, 1);
            let wtxid = open_bitcoin_core::consensus::transaction_wtxid(&block.transactions[0])
                .expect("actual coinbase wtxid");
            let payload = open_bitcoin_codec::CompactBlockPayload {
                header: block.header.clone(),
                nonce: 1,
                short_ids: vec![open_bitcoin_core::consensus::compact_short_id_for_wtxid(
                    selector, &wtxid,
                )],
                prefilled_transactions: vec![],
            };
            let result = network
                .receive_message(
                    peer,
                    WireNetworkMessage::CompactBlock(payload),
                    1,
                    ScriptVerifyFlags::NONE,
                    command.params,
                )
                .expect("real accepted-hash compact receipt");
            assert!(
                result
                    .outbound
                    .iter()
                    .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
                "actual incomplete compact must request missing transaction: {result:?}"
            );
            assert_eq!(
                network.peer_manager().block_request_peer_order(hash),
                vec![peer]
            );
            assert!(
                network
                    .peer_manager()
                    .compact_download_peer_state(peer)
                    .is_some_and(|state| state.in_flight.contains_key(&hash))
            );
        })
        .expect("held actual compact request");
}

fn request_state(
    command: &CanonicalSubmissionCommand,
    matching: BlockHash,
    unrelated: BlockHash,
    matching_peer: u64,
    unrelated_peer: u64,
) -> impl fmt::Debug + PartialEq + use<> {
    command
        .network
        .read(|network| {
            let peers = network.peer_manager();
            (
                peers.block_request_peer_order(matching),
                peers.block_request_peer_order(unrelated),
                peers.compact_download_peer_state(matching_peer).cloned(),
                peers.compact_download_peer_state(unrelated_peer).cloned(),
                network.block_relay_evidence_status().cleanup,
            )
        })
        .expect("held matching and unrelated request state")
}

fn request_order(command: &CanonicalSubmissionCommand, hash: BlockHash) -> Vec<u64> {
    command
        .network
        .read(|network| network.peer_manager().block_request_peer_order(hash))
        .expect("held ordered request state")
}

fn request(network: &mut SubmissionNetwork, peer: u64, hash: BlockHash, params: ConsensusParams) {
    network.connect_outbound_peer(peer, 0).expect("fresh peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage::default()),
        WireNetworkMessage::Verack,
    ] {
        network
            .receive_message(peer, message, 1, ScriptVerifyFlags::NONE, params)
            .expect("genuine handshake");
    }
    assert!(
        network
            .peer_manager_mut()
            .request_missing_blocks(peer, &[hash])
            .expect("actual ordered request")
            .is_some()
    );
}

fn request_pair(
    command: &CanonicalSubmissionCommand,
    matching: BlockHash,
    unrelated: BlockHash,
    matching_peer: u64,
    unrelated_peer: u64,
) {
    command
        .network
        .mutate(|network| {
            request(network, matching_peer, matching, command.params);
            request(network, unrelated_peer, unrelated, command.params);
        })
        .expect("real matching and unrelated requests");
}

#[test]
fn phase167_compact_download_cleanup_accepted_only_after_public_final_success() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let block = child(&genesis.header, 1);
    let hash = block_hash(&block.header);
    let unrelated = child(&genesis.header, 2);
    let unrelated_hash = block_hash(&unrelated.header);
    request_pair(&command, hash, unrelated_hash, 167_531, 167_533);
    command
        .network
        .mutate(|network| request(network, 167_532, hash, command.params))
        .expect("second matching request");
    assert_eq!(request_order(&command, hash), vec![167_531, 167_532]);

    // Act
    let outcome = command
        .submit(wire(&block))
        .expect("genuine final accepted success");
    let orders = (
        request_order(&command, hash),
        request_order(&command, unrelated_hash),
    );
    close(path, runtime, command);

    // Assert
    assert!(matches!(outcome, CanonicalSubmissionOutcome::Accepted(_)));
    assert!(
        orders.0.is_empty(),
        "matching retained requests survive public success: {:?}",
        orders.0
    );
    assert_eq!(orders.1, vec![167_533]);
}

#[test]
fn phase167_compact_download_cleanup_post_accept_compact_request_probe() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    warm_accepted(&command, &block);

    // Act
    request_known_compact(&command, &block, 167_539);
    let order = request_order(&command, block_hash(&block.header));
    close(path, runtime, command);

    // Assert
    assert_eq!(order, vec![167_539]);
}

#[test]
fn phase167_compact_download_cleanup_duplicate_uses_stored_body_with_changed_witness() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let hash = block_hash(&block.header);
    warm_accepted(&command, &block);
    request_known_compact(&command, &block, 167_541);
    let unrelated = BlockHash::from_byte_array([0x54; 32]);
    command
        .network
        .mutate(|network| request(network, 167_542, unrelated, command.params))
        .expect("unrelated genuine request");
    let mut altered = wire(&block);
    altered.transactions[0].inputs[0].witness =
        open_bitcoin_core::primitives::ScriptWitness::new(vec![vec![7]]);
    let before = accepted_state(&command);

    // Act
    let outcome = command.submit(altered).expect("valid retained duplicate");
    let orders = (
        request_order(&command, hash),
        request_order(&command, unrelated),
    );
    let after = accepted_state(&command);
    close(path, runtime, command);

    // Assert
    assert!(matches!(outcome, CanonicalSubmissionOutcome::Duplicate(actual) if actual == hash));
    assert!(orders.0.is_empty());
    assert_eq!(orders.1, vec![167_542]);
    assert_eq!(after, before);
}

#[test]
fn phase167_compact_download_cleanup_corrupt_stored_duplicate_preserves_post_tamper_state() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let hash = block_hash(&block.header);
    warm_accepted(&command, &block);
    request_known_compact(&command, &block, 167_551);
    let unrelated = BlockHash::from_byte_array([0x55; 32]);
    command
        .network
        .mutate(|network| request(network, 167_552, unrelated, command.params))
        .expect("unrelated genuine request");
    let key = block_row_key(hash);
    assert!(
        native_rows(&command.store)[crate::StorageNamespace::BlockIndex.as_str()]
            .contains_key(key.as_bytes())
    );
    command
        .store
        .write_raw_for_test(crate::StorageNamespace::BlockIndex, &key, vec![0xff])
        .expect("real corrupt BlockIndex row");
    assert!(command.store.load_block(hash).is_err());
    let before_rows = native_rows(&command.store);
    let before_state = accepted_state(&command);
    let before_requests = request_state(&command, hash, unrelated, 167_551, 167_552);

    // Act
    let error = command
        .submit(wire(&block))
        .expect_err("actual stored corruption must fail");
    let after_rows = native_rows(&command.store);
    let after_state = accepted_state(&command);
    let after_requests = request_state(&command, hash, unrelated, 167_551, 167_552);
    close(path, runtime, command);

    // Assert
    assert!(
        error.to_string().contains("duplicate retained body"),
        "{error}"
    );
    assert!(
        std::error::Error::source(&error)
            .and_then(|source| source.downcast_ref::<crate::StorageError>())
            .is_some()
    );
    assert_eq!(after_requests, before_requests);
    assert_eq!(after_rows, before_rows);
    assert_eq!(after_state, before_state);
}

#[test]
fn phase167_compact_download_cleanup_missing_stored_duplicate_preserves_post_removal_state() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let hash = block_hash(&block.header);
    warm_accepted(&command, &block);
    request_known_compact(&command, &block, 167_561);
    let unrelated = BlockHash::from_byte_array([0x56; 32]);
    command
        .network
        .mutate(|network| request(network, 167_562, unrelated, command.params))
        .expect("unrelated genuine request");
    let key = block_row_key(hash);
    let database = command.store.database();
    let index = database
        .keyspace(
            crate::StorageNamespace::BlockIndex.as_str(),
            fjall::KeyspaceCreateOptions::default,
        )
        .expect("existing BlockIndex namespace");
    index
        .remove(key.as_bytes())
        .expect("real block row removal");
    database
        .persist(fjall::PersistMode::SyncAll)
        .expect("durable removal");
    assert_eq!(command.store.load_block(hash).expect("removed row"), None);
    let before_rows = native_rows(&command.store);
    let before_state = accepted_state(&command);
    let before_requests = request_state(&command, hash, unrelated, 167_561, 167_562);

    // Act
    let error = command
        .submit(wire(&block))
        .expect_err("missing retained body must fail");
    let after_rows = native_rows(&command.store);
    let after_state = accepted_state(&command);
    let after_requests = request_state(&command, hash, unrelated, 167_561, 167_562);
    close(path, runtime, command);

    // Assert
    assert_eq!(error.to_string(), "duplicate retained body unavailable");
    assert!(std::error::Error::source(&error).is_none());
    assert_eq!(after_requests, before_requests);
    assert_eq!(after_rows, before_rows);
    assert_eq!(after_state, before_state);
}

#[test]
fn phase167_compact_download_cleanup_publication_faults_preserve_requests_after_absorption() {
    use crate::storage::fjall_store::validation_history::HistoryPublicationFault;
    for fault in [
        HistoryPublicationFault::BeforeCommit,
        HistoryPublicationFault::AfterCommit,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        let unrelated = BlockHash::from_byte_array([0x64; 32]);
        command
            .network
            .mutate(|network| {
                request(network, 167_641, hash, command.params);
                request(network, 167_642, hash, command.params);
                request(network, 167_643, unrelated, command.params);
            })
            .expect("real requests before public submit");
        let before_requests = request_state(&command, hash, unrelated, 167_641, 167_643);
        ABSORB_FAULT.set(Some(fault));

        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("actual publication fault");
        let after_requests = request_state(&command, hash, unrelated, 167_641, 167_643);
        let tip = command
            .network
            .maybe_chain_tip()
            .expect("actual chainstate")
            .expect("absorbed tip");
        let live_provenance = command
            .network
            .read(|network| network.chainstate().validation_provenance(hash))
            .expect("live owner")
            .expect("earned receipt");
        close(path, runtime, command);

        // Assert
        assert_eq!(error.stage, "acceptance/publication", "{fault:?}");
        assert!(std::error::Error::source(&error).is_some());
        assert_eq!(tip.block_hash, hash);
        assert_eq!(
            live_provenance,
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        assert_eq!(after_requests, before_requests);
    }
}

#[test]
fn phase167_compact_download_cleanup_basic_faults_preserve_requests_at_real_stage() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeBody,
        FilterPublicationFault::BeforeCoins,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(BasicFilterStartupMode::Enabled);
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        let unrelated = BlockHash::from_byte_array([0x65; 32]);
        request_pair(&command, hash, unrelated, 167_651, 167_652);
        let before_requests = request_state(&command, hash, unrelated, 167_651, 167_652);
        command.store.set_basic_filter_fault(fault);

        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("actual BASIC fault");
        let after_requests = request_state(&command, hash, unrelated, 167_651, 167_652);
        let tip = command
            .network
            .maybe_chain_tip()
            .expect("actual chainstate")
            .expect("tip");
        let retained = command
            .store
            .load_block(hash)
            .expect("actual retained row probe");
        close(path, runtime, command);

        // Assert
        assert!(
            std::error::Error::source(&error).is_some(),
            "{fault:?}: {error}"
        );
        assert_eq!(after_requests, before_requests);
        if fault == FilterPublicationFault::BeforeBody {
            assert_eq!(error.stage, "retained body");
            assert_eq!(tip.height, 0);
            assert_eq!(retained, None);
        } else {
            assert_eq!(error.stage, "accepted full flush");
            assert_eq!(tip.block_hash, hash);
            assert_eq!(retained, Some(block));
        }
    }
}

#[test]
fn phase167_compact_download_cleanup_after_basic_failure_keeps_requests_and_real_effects() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Enabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let hash = block_hash(&block.header);
    let unrelated = BlockHash::from_byte_array([0x66; 32]);
    request_pair(&command, hash, unrelated, 167_661, 167_662);
    let before_requests = request_state(&command, hash, unrelated, 167_661, 167_662);
    CHANGE_AFTER_BASIC.set(true);

    // Act
    let error = command
        .submit(wire(&block))
        .expect_err("actual achieved-inventory failure");
    let after_requests = request_state(&command, hash, unrelated, 167_661, 167_662);
    let tip = command
        .network
        .maybe_chain_tip()
        .expect("actual chainstate")
        .expect("accepted tip");
    let retained = command.store.load_block(hash).expect("actual retained row");
    let provenance = command
        .store
        .validation_provenance(hash)
        .expect("actual durable receipt");
    close(path, runtime, command);

    // Assert
    assert_eq!(error.stage, "accepted achieved inventory");
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(tip.block_hash, hash);
    assert_eq!(retained, Some(block));
    assert_eq!(
        provenance,
        crate::storage::validation_history::ValidationProvenance::ScriptsValid
    );
    assert_eq!(after_requests, before_requests);
}

#[test]
fn phase167_compact_download_cleanup_physical_inventory_refusal_keeps_requests() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let hash = block_hash(&block.header);
    let unrelated = BlockHash::from_byte_array([0x67; 32]);
    request_pair(&command, hash, unrelated, 167_671, 167_672);
    let before_requests = request_state(&command, hash, unrelated, 167_671, 167_672);
    let link = command
        .store
        .datadir()
        .join("phase167-plan51-physical-link");
    std::os::unix::fs::symlink("absent", &link).expect("owned disposable symlink");

    // Act
    let error = command
        .submit(wire(&block))
        .expect_err("actual physical inventory refusal");
    let after_requests = request_state(&command, hash, unrelated, 167_671, 167_672);
    let tip = command
        .network
        .maybe_chain_tip()
        .expect("chainstate")
        .expect("accepted tip");
    let retained = command.store.load_block(hash).expect("retained body");
    let provenance = command
        .store
        .validation_provenance(hash)
        .expect("durable receipt");
    close(path, runtime, command);

    // Assert
    assert_eq!(error.stage, "accepted physical inventory");
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(tip.block_hash, hash);
    assert_eq!(retained, Some(block));
    assert_eq!(
        provenance,
        crate::storage::validation_history::ValidationProvenance::ScriptsValid
    );
    assert_eq!(after_requests, before_requests);
}

#[test]
fn phase167_compact_download_cleanup_after_basic_header_advance_keeps_requests() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let hash = block_hash(&block.header);
    let unrelated = BlockHash::from_byte_array([0x68; 32]);
    request_pair(&command, hash, unrelated, 167_681, 167_682);
    let before_requests = request_state(&command, hash, unrelated, 167_681, 167_682);
    ADVANCE_AFTER_BASIC.set(true);

    // Act
    let error = command
        .submit(wire(&block))
        .expect_err("actual post-BASIC header advance");
    let after_requests = request_state(&command, hash, unrelated, 167_681, 167_682);
    let tip = command
        .network
        .maybe_chain_tip()
        .expect("chainstate")
        .expect("accepted tip");
    let retained = command.store.load_block(hash).expect("retained body");
    close(path, runtime, command);

    // Assert
    assert_eq!(error.stage, "canonical history reopen capacity unavailable");
    assert_eq!(tip.block_hash, hash);
    assert_eq!(retained, Some(block));
    assert_eq!(after_requests, before_requests);
}

#[test]
fn phase167_compact_download_cleanup_basic_backlog_projection_refusal_keeps_requests() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut parent = CanonicalChain::Regtest.spec().genesis_header();
    for height in 1..=9 {
        let previous = child(&parent, height);
        accepted(
            command
                .submit(wire(&previous))
                .expect("real accepted prefix"),
        );
        parent = previous.header;
    }
    let (runtime, command) = reopen(&path, runtime, command, BasicFilterStartupMode::Enabled);
    let block = child(&parent, 10);
    let hash = block_hash(&block.header);
    let unrelated = BlockHash::from_byte_array([0x69; 32]);
    admit_only(&command, std::slice::from_ref(&block));
    request_pair(&command, hash, unrelated, 167_691, 167_692);
    let before_requests = request_state(&command, hash, unrelated, 167_691, 167_692);
    let before_state = accepted_state(&command);
    let before_rows = native_rows(&command.store);

    // Act
    let error = command
        .submit(wire(&block))
        .expect_err("pre-effect BASIC backlog refusal");
    let after_requests = request_state(&command, hash, unrelated, 167_691, 167_692);
    let after_state = accepted_state(&command);
    let after_rows = native_rows(&command.store);
    let tip = command
        .network
        .maybe_chain_tip()
        .expect("chainstate")
        .expect("unchanged tip");
    let retained = command.store.load_block(hash).expect("retained body");
    close(path, runtime, command);

    // Assert
    assert_eq!(error.stage, "prepared BASIC resource projection");
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(tip.height, 9);
    assert_eq!(retained, None);
    assert_eq!(after_requests, before_requests);
    assert_eq!(after_state, before_state);
    assert_eq!(after_rows, before_rows);
}

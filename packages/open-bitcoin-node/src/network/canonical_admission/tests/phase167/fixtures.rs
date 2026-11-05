// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

pub(super) type Network = ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView>;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct AcceptedState {
    chain: open_bitcoin_core::chainstate::ChainstateSnapshot,
    maybe_coins_best: Option<BlockHash>,
    basic: String,
    maybe_basic_progress:
        Option<open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress>,
    provenance: Vec<(
        crate::storage::validation_history::ValidationProvenance,
        crate::storage::validation_history::ValidationProvenance,
    )>,
}

pub(super) fn with_owner(check: impl FnOnce(&mut Network, &FjallNodeStore, ConsensusParams)) {
    with_mode(crate::chainstate::BasicFilterStartupMode::Disabled, check);
}

pub(super) fn with_mode(
    mode: crate::chainstate::BasicFilterStartupMode,
    check: impl FnOnce(&mut Network, &FjallNodeStore, ConsensusParams),
) {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("unique disposable store");
    let dir = std::fs::canonicalize(std::env::temp_dir())
        .expect("physical temporary parent")
        .join(format!("phase167-ingress-{:x}", u128::from_le_bytes(nonce)));
    {
        let (store, recovered) =
            FjallNodeStore::open_canonical_genesis(&dir, CanonicalChain::Regtest)
                .expect("proved genesis")
                .into_parts()
                .expect("exclusive store");
        let (managed, proof) =
            ManagedChainstate::from_canonical_genesis(store.clone(), recovered, mode)
                .expect("actual canonical coins owner");
        let params =
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof);
        let mut network = ManagedPeerNetwork::from_canonical_genesis_initialized(
            managed,
            open_bitcoin_network::LocalPeerConfig {
                magic: open_bitcoin_core::primitives::NetworkMagic::from_bytes([
                    0xfa, 0xbf, 0xb5, 0xda,
                ]),
                ..Default::default()
            },
            16,
            &proof,
        )
        .expect("actual proved network owner");
        network
            .connect_outbound_peer(1, 0)
            .expect("socket-free peer");
        assert_eq!(network.relay_activation, Default::default());
        assert_eq!(network.block_relay_activation, Default::default());
        assert!(!network.inbound_serving_enabled);
        check(&mut network, &store, params);
    }
    std::fs::remove_dir_all(dir).expect("all store handles dropped");
}

pub(super) fn mined_child(parent: &BlockHeader, height: u8) -> Block {
    let coinbase_script = if height <= 16 {
        vec![0x50 + height, 0]
    } else {
        let mut number = vec![height];
        if height & 0x80 != 0 {
            number.push(0);
        }
        let mut script = vec![number.len() as u8];
        script.extend(number);
        script.push(0);
        script
    };
    let transactions = vec![Transaction {
        version: 1,
        lock_time: u32::from(height),
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(coinbase_script).expect("BIP34 height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("selected subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
    }];
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: block_hash(parent),
            merkle_root: block_merkle_root(&transactions)
                .expect("real merkle root")
                .0,
            time: parent.time + 1,
            bits: parent.bits,
            nonce: 0,
        },
        transactions,
    };
    mine(&mut block.header);
    block
}

pub(super) fn mine(header: &mut BlockHeader) {
    header.nonce = (0..100_000)
        .find(|nonce| {
            header.nonce = *nonce;
            open_bitcoin_core::consensus::check_block_header(header).is_ok()
        })
        .expect("bounded genuine regtest proof");
}

pub(super) fn admit(network: &mut Network, block: &Block, params: ConsensusParams) {
    network
        .handle_validated_headers(
            1,
            HeadersMessage {
                headers: vec![block.header.clone()],
            },
            i64::MIN,
            params,
        )
        .expect("genuine contextual header admission");
}

pub(super) fn accepted(network: &Network, store: &FjallNodeStore) -> AcceptedState {
    let state = network.chainstate.chainstate();
    let provenance: Vec<_> = state
        .active_chain()
        .iter()
        .map(|position| {
            (
                network
                    .chainstate
                    .validation_provenance(position.block_hash)
                    .expect("live provenance"),
                store
                    .validation_provenance(position.block_hash)
                    .expect("saved provenance"),
            )
        })
        .collect();
    AcceptedState {
        chain: network.chainstate_snapshot().expect("all coins/undo/work"),
        maybe_coins_best: state.coins_best_block().expect("coins best block"),
        basic: format!(
            "{:?}",
            store
                .maybe_basic_filter_state()
                .expect("processed/safe endpoints")
        ),
        maybe_basic_progress: network.chainstate.maybe_basic_index_progress(),
        provenance,
    }
}

pub(super) fn assert_unaccepted(
    network: &Network,
    store: &FjallNodeStore,
    block: &Block,
    before: &AcceptedState,
) {
    assert_eq!(&accepted(network, store), before);
    assert_ne!(
        network
            .chainstate
            .validation_provenance(block_hash(&block.header))
            .expect("live"),
        crate::storage::validation_history::ValidationProvenance::ScriptsValid
    );
    assert_ne!(
        store
            .validation_provenance(block_hash(&block.header))
            .expect("durable"),
        crate::storage::validation_history::ValidationProvenance::ScriptsValid
    );
}

pub(super) fn compact(block: &Block, partial: bool) -> open_bitcoin_codec::CompactBlockPayload {
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, 1);
    let wtxid =
        open_bitcoin_core::consensus::transaction_wtxid(&block.transactions[0]).expect("wtxid");
    open_bitcoin_codec::CompactBlockPayload {
        header: block.header.clone(),
        nonce: 1,
        short_ids: if partial {
            vec![open_bitcoin_core::consensus::compact_short_id_for_wtxid(
                selector, &wtxid,
            )]
        } else {
            vec![]
        },
        prefilled_transactions: if partial {
            vec![]
        } else {
            vec![open_bitcoin_codec::PrefilledTransaction {
                index_delta: 0,
                transaction: block.transactions[0].clone(),
            }]
        },
    }
}

pub(super) fn compact_peer(network: &mut Network, params: ConsensusParams) {
    // Explicit socket-free test transport; constructor activation defaults stay off.
    network.peer_manager.set_block_relay_activation_policy(
        open_bitcoin_network::BlockRelayActivationPolicy {
            compact_relay: open_bitcoin_network::CompactRelayActivationConfig { enabled: true },
            ..Default::default()
        },
    );
    for message in [
        WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
        WireNetworkMessage::Verack,
        WireNetworkMessage::SendCompact(open_bitcoin_codec::SendCompactMessage {
            announce: true,
            version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
        }),
    ] {
        network
            .receive_message(1, message, 1, ScriptVerifyFlags::NONE, params)
            .expect("local handshake");
    }
}

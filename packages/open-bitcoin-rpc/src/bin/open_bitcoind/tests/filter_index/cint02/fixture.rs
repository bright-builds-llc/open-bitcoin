// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};
use open_bitcoin_node::{
    BasicFilterQuery, BasicFilterRecordView, DurableSyncRuntime, FjallNodeStore, PersistMode,
    SyncNetwork, SyncRuntimeConfig,
    chainstate::BasicFilterStartupMode,
    core::{
        chainstate::{
            AnchoredBlock, FlushMode, FlushPolicyTime, ManualPruneInput, PruneMode,
            plan_manual_prune,
        },
        consensus::{
            CanonicalChain, ScriptVerifyFlags, block_hash, block_merkle_root,
            chain_spec::CanonicalGenesisProof, check_block_header,
        },
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput,
        },
    },
};
use std::path::{Path, PathBuf};

pub(super) const TIP: u32 = 1_002;
pub(super) const BOUNDARY: u32 = 714;
const ORIGINAL_TIP: u32 = TIP - 1;
const FORK: u32 = 998;

pub(super) struct Source(PathBuf);

impl Source {
    pub(super) fn new() -> Self {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned pruned fixture nonce");
        Self(
            std::env::temp_dir()
                .canonicalize()
                .expect("physical temporary directory")
                .join(format!("cint02-service-{:x}", u128::from_le_bytes(nonce))),
        )
    }

    pub(super) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        if self.0.exists() {
            std::fs::remove_dir_all(&self.0).expect("client and storage handles closed");
        }
    }
}

pub(super) struct Evidence {
    pub tip_hash: BlockHash,
    pub pruned: BasicFilterRecordView,
    pub retained: BasicFilterRecordView,
    pub retained_block: Block,
    pub active: BasicFilterRecordView,
    pub displaced: BasicFilterRecordView,
}

fn coinbase_height(height: u32) -> ScriptBuf {
    let mut payload = height.to_le_bytes().to_vec();
    while payload.last() == Some(&0) {
        payload.pop();
    }
    if payload.last().is_some_and(|byte| byte & 0x80 != 0) {
        payload.push(0);
    }
    let mut script = if height <= 16 {
        vec![0x50 + height as u8]
    } else {
        let mut push = vec![payload.len() as u8];
        push.extend(payload);
        push
    };
    script.push(0);
    ScriptBuf::from_bytes(script).expect("canonical height commitment")
}

fn mine(parent: &BlockHeader, height: u32, output_opcode: u8) -> Block {
    let transactions = vec![Transaction {
        version: 1,
        lock_time: height,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: coinbase_height(height),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000 >> (height / 150)).expect("Regtest subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![output_opcode]).expect("branch script"),
        }],
    }];
    let mut header = BlockHeader {
        version: 4,
        previous_block_hash: block_hash(parent),
        merkle_root: block_merkle_root(&transactions)
            .expect("coinbase commitment")
            .0,
        time: parent.time + 1,
        bits: parent.bits,
        nonce: 0,
    };
    header.nonce = (0..100_000)
        .find(|nonce| {
            header.nonce = *nonce;
            check_block_header(&header).is_ok()
        })
        .expect("bounded Regtest proof of work");
    Block {
        header,
        transactions,
    }
}

pub(super) fn build(source: &Source, prune: bool) -> Evidence {
    let spec = CanonicalChain::Regtest.spec();
    let genesis = spec.genesis_block();
    let proof = CanonicalGenesisProof::prove(spec, &genesis).expect("selected canonical genesis");
    let params =
        open_bitcoin_node::core::chainstate::engine::canonical_genesis::consensus_params(&proof);
    let opened = FjallNodeStore::open_canonical_genesis(source.path(), CanonicalChain::Regtest)
        .expect("fresh canonical store");
    let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            dns_seeds: Vec::new(),
            ..Default::default()
        },
        BasicFilterStartupMode::Enabled,
    )
    .expect("canonical managed runtime");
    let mut handle = runtime.network_handle();
    handle
        .connect_outbound_peer(1, 0)
        .expect("socket-free header ingress");
    handle
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("manual prune mode");
    let admit = |blocks: &[Block]| {
        handle
            .receive_message(
                1,
                WireNetworkMessage::Headers(HeadersMessage {
                    headers: blocks.iter().map(|block| block.header.clone()).collect(),
                }),
                i64::MIN,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("contextual header admission");
        for block in blocks {
            runtime
                .store()
                .save_block(block, PersistMode::Sync)
                .expect("durable serialized block");
        }
    };
    let mut blocks = vec![genesis];
    for height in 1..=ORIGINAL_TIP {
        let block = mine(&blocks.last().expect("parent").header, height, 0x51);
        admit(std::slice::from_ref(&block));
        handle
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("canonical acceptance");
        blocks.push(block);
    }
    checkpoint(&handle, ORIGINAL_TIP);
    let prior_status = runtime
        .durable_sync_state(
            open_bitcoin_node::SyncLifecycleState::Stopped,
            None,
            2_000_000_000,
        )
        .expect("genuine pre-reorg status");
    runtime
        .persist_durable_sync_state(prior_status)
        .expect("retain the earlier persisted tip across the reorg");
    let displaced = query(&handle, block_hash(&blocks[1000].header));
    let mut parent = blocks[FORK as usize].header.clone();
    let replacement = (FORK + 1..=TIP)
        .map(|height| {
            let block = mine(&parent, height, 0x52);
            parent = block.header.clone();
            block
        })
        .collect::<Vec<_>>();
    admit(&replacement);
    let headers = handle.header_entries().expect("earned branch work");
    let branch = replacement
        .into_iter()
        .map(|block| {
            let chain_work = headers
                .iter()
                .find(|entry| entry.block_hash == block_hash(&block.header))
                .expect("admitted replacement")
                .chain_work;
            AnchoredBlock { block, chain_work }
        })
        .collect::<Vec<_>>();
    let disconnect = blocks[(FORK + 1) as usize..]
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let transition = handle
        .reorg_to_branch(
            &disconnect,
            &branch,
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(i64::MIN)),
            ScriptVerifyFlags::NONE,
            params,
        )
        .expect("stronger canonical branch");
    assert_eq!(transition.disconnected.len(), 3);
    assert_eq!(transition.connected.len(), 4);
    checkpoint(&handle, TIP);
    let active = query(&handle, block_hash(&branch[1].block.header));
    assert_ne!(active.filter_header(), displaced.filter_header());
    let retained_block = blocks[(BOUNDARY + 1) as usize].clone();
    let evidence = Evidence {
        tip_hash: block_hash(&branch.last().expect("replacement tip").block.header),
        pruned: query(&handle, block_hash(&blocks[BOUNDARY as usize].header)),
        retained: query(&handle, block_hash(&retained_block.header)),
        retained_block,
        active,
        displaced,
    };
    if prune {
        let plan = plan_manual_prune(&ManualPruneInput {
            tip: TIP,
            prune_after_height: SyncNetwork::Regtest.prune_after_height(),
            mode: PruneMode::ManualOnly,
            target_height: BOUNDARY,
            maybe_present_heights: Some((1..=BOUNDARY).collect()),
            locks: handle.list_prune_locks().expect("owned prune locks"),
        })
        .expect("legal boundary plan");
        let committed = handle
            .flush_applying_prune_plan(
                FlushMode::Always,
                FlushPolicyTime::new(90_000),
                u64::MAX,
                &plan,
                &[],
            )
            .expect("actual paired deletion");
        assert_eq!(committed.deleted_block_hashes.len(), BOUNDARY as usize);
        assert_eq!(
            committed.deleted_block_hashes.last().copied(),
            Some(evidence.pruned.identity().block_hash())
        );
    }
    drop(handle);
    // Explicitly drop the owning runtime before returning the fixture handoff.
    drop(runtime);
    evidence
}

fn checkpoint(handle: &open_bitcoin_node::DurableNetworkHandle, tip: u32) {
    handle
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::new(u64::from(tip)),
            u64::MAX,
        )
        .expect("durable chainstate checkpoint");
    assert!((0..=tip).any(|_| {
        handle
            .drive_basic_filter_index_turn()
            .expect("durable BASIC turn")
            .maybe_accepted_lag
            == Some(0)
    }));
}

fn query(
    handle: &open_bitcoin_node::DurableNetworkHandle,
    hash: BlockHash,
) -> BasicFilterRecordView {
    let BasicFilterQuery::Found(record) = handle
        .basic_filter_query(hash)
        .expect("pre-prune BASIC query")
    else {
        panic!("accepted BASIC record missing for {hash:?}");
    };
    record
}

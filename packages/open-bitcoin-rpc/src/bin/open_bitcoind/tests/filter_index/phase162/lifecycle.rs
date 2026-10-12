// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Accepted extension and checkpoint-crossing replacement on the configured owner.

use super::{core_types, fixtures::Phase162Fixture, hex};
use core_types::{
    chainstate::{
        AnchoredBlock, BasicFilterInputs, ChainPosition, FlushMode, FlushPolicyTime,
        HistoricalBlockUndo, IndexPrefix,
    },
    consensus::block_hash,
    mempool::{PolicyTime, ReorgLifecycleContext},
    primitives::{Block, BlockHash},
};
use open_bitcoin_node::{BasicFilterQuery, BasicFilterRecordView, PersistMode};

pub(crate) fn record(fixture: &Phase162Fixture, hash: BlockHash) -> BasicFilterRecordView {
    let BasicFilterQuery::Found(record) = fixture
        .opened
        .network
        .basic_filter_query(hash)
        .expect("checked immutable query")
    else {
        panic!("accepted indexed record must exist")
    };
    record
}

pub(crate) fn safe_height(fixture: &Phase162Fixture) -> u32 {
    let owner = fixture
        .store
        .load_prune_protection()
        .expect("durable protection")
        .maybe_owner()
        .expect("BASIC owner");
    let IndexPrefix::Committed(endpoint) = owner.checkpoint().checkpoint().prefix() else {
        panic!("earned nonempty safe prefix")
    };
    endpoint.height()
}

pub(crate) fn flush(fixture: &Phase162Fixture, now: u64) {
    assert!(
        fixture
            .opened
            .network
            .flush_coins(FlushMode::Always, FlushPolicyTime::new(now), u64::MAX)
            .expect("ordinary real coins fence")
            .wrote_coins
    );
}

pub(crate) fn assert_oracle(
    fixture: &Phase162Fixture,
    record: &BasicFilterRecordView,
    case: usize,
) {
    let expected = &fixture.provenance.oracle["cases"][case];
    assert_eq!(expected["encoded"], hex(record.encoded_bytes()));
    assert_eq!(
        expected["hash_raw"],
        hex(record.identity().filter_hash().as_bytes())
    );
    assert_eq!(
        expected["header_raw"],
        hex(record.identity().filter_header().as_bytes())
    );
}

fn assert_inputs(
    fixture: &Phase162Fixture,
    block: &Block,
    position: &ChainPosition,
    expected: &[Vec<u8>],
) {
    let undo = fixture
        .store
        .load_undo(position.block_hash)
        .expect("real undo read")
        .expect("accepted durable undo");
    let inputs = BasicFilterInputs::from_historical(
        block,
        position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &undo,
        }),
    )
    .expect("normal historical and same-block provenance");
    assert_eq!(
        inputs
            .spent_scripts()
            .map(<[u8]>::to_vec)
            .collect::<Vec<_>>(),
        expected
    );
}

pub(crate) fn extend_and_reorg(
    fixture: &Phase162Fixture,
    mut tick: impl FnMut(),
) -> Vec<BasicFilterRecordView> {
    // Arrange: preserve original immutable rows before crossing the positive checkpoint.
    let mut stale: Vec<_> = fixture.provenance.receipts[999..]
        .iter()
        .map(|position| record(fixture, position.block_hash))
        .collect();
    let extension = fixture.recipe.fork(1000, 1, 0x51).remove(0);
    fixture
        .store
        .save_block(&extension, PersistMode::Sync)
        .expect("ordinary extension body");
    let receipt = fixture
        .opened
        .network
        .connect_local_block(&extension, fixture.recipe.flags, fixture.recipe.params)
        .expect("normal-maturity accepted h1001");
    assert_eq!(receipt.height, 1001);
    assert_eq!(receipt.block_hash, block_hash(&extension.header));
    assert_eq!(safe_height(fixture), 1000);
    tick();
    assert_eq!(
        fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("processed")
            .expect("owner")
            .best_block_height,
        1001
    );
    assert_eq!(
        safe_height(fixture),
        1000,
        "ahead processing cannot manufacture coins durability"
    );
    stale.push(record(fixture, receipt.block_hash));
    flush(fixture, 500_001);
    tick();
    assert_eq!(safe_height(fixture), 1001);
    assert_inputs(
        fixture,
        &extension,
        &receipt,
        &fixture
            .recipe
            .fork_spent_scripts(1000, std::slice::from_ref(&extension))[0],
    );

    // Act: replace exactly h999..1001 with genuinely accepted h999..1002.
    let branch = fixture.recipe.fork(998, 4, 0x62);
    let connected: Vec<_> = branch
        .iter()
        .enumerate()
        .map(|(offset, block)| AnchoredBlock {
            block: block.clone(),
            chain_work: 2000 + offset as u128,
        })
        .collect();
    for anchored in &connected {
        fixture
            .store
            .save_block(&anchored.block, PersistMode::Sync)
            .expect("received competing body");
    }
    let disconnect = vec![
        extension,
        fixture.recipe.blocks[1000].clone(),
        fixture.recipe.blocks[999].clone(),
    ];
    let transition = fixture
        .opened
        .network
        .reorg_to_branch(
            &disconnect,
            &connected,
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(500_002)),
            fixture.recipe.flags,
            fixture.recipe.params,
        )
        .expect("production accepted shallow reorg");
    assert_eq!(transition.disconnected.len(), 3);
    assert_eq!(transition.connected.len(), 4);
    for (offset, position) in transition.connected.iter().enumerate() {
        assert_eq!(position.height, 999 + offset as u32);
        assert_eq!(position.block_hash, block_hash(&branch[offset].header));
    }
    assert_eq!(
        fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("rewind")
            .expect("owner")
            .best_block_height,
        998
    );
    assert_eq!(safe_height(fixture), 998);
    tick();
    assert_eq!(
        fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("replacement processing")
            .expect("owner")
            .best_block_height,
        1002
    );
    assert_eq!(safe_height(fixture), 998);
    flush(fixture, 500_003);
    tick();

    // Assert: active and stale positive h1000 commitments come from raw independent inputs.
    assert_eq!(safe_height(fixture), 1002);
    let active = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("accepted replacement")
        .active_chain;
    assert_eq!(&active[..999], &fixture.provenance.receipts[..999]);
    assert_eq!(&active[999..], transition.connected);
    let spent = fixture.recipe.fork_spent_scripts(998, &branch);
    for (offset, position) in transition.connected.iter().enumerate() {
        let row = record(fixture, position.block_hash);
        assert_oracle(fixture, &row, 1001 + offset);
        assert_inputs(fixture, &branch[offset], position, &spent[offset]);
    }
    assert_oracle(fixture, &stale[1], 1000);
    assert_ne!(
        stale[1].filter_header(),
        record(fixture, transition.connected[1].block_hash).filter_header()
    );
    assert_ne!(stale[1].identity().block_hash(), active[1000].block_hash);
    for row in &stale {
        let retained = record(fixture, row.identity().block_hash());
        assert_eq!(retained.identity(), row.identity());
        assert_eq!(retained.encoded_bytes(), row.encoded_bytes());
    }
    eprintln!(
        "phase162 accepted lifecycle: extension=1001 disconnected=3 replacement=999..1002 ancestor=998 stale_rows=3 positive_checkpoint=1000 normal_maturity=100"
    );
    stale
}

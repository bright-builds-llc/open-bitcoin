// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

pub(crate) fn block(parent: BlockHash, height: u32, tag: u8) -> Block {
    let mut height_bytes = Vec::new();
    let mut remaining = height;
    while remaining > 0 {
        height_bytes.push(remaining as u8);
        remaining >>= 8;
    }
    if height_bytes.last().is_some_and(|byte| byte & 0x80 != 0) {
        height_bytes.push(0);
    }
    let mut coinbase_script = vec![height_bytes.len() as u8];
    coinbase_script.extend(height_bytes);
    coinbase_script.push(tag);
    let transactions = vec![Transaction {
        version: 1,
        lock_time: height,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(coinbase_script).expect("script"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("reward"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
    }];
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: parent,
            merkle_root: block_merkle_root(&transactions).expect("merkle").0,
            time: 1_000 + height * 100,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy POW");
    block
}

pub(super) fn fresh(
    name: &str,
    pressure: bool,
) -> (
    std::path::PathBuf,
    FjallNodeStore,
    ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
) {
    let dir = std::env::temp_dir().join(format!(
        "phase159-accept-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    let store = FjallNodeStore::open(&dir).expect("new store");
    let state = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        Vec::new(),
        Default::default(),
        None,
    );
    let manager = ManagedChainstate::from_recovered_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        state,
        FlushLifecycle::ready(
            FlushPolicyTime::from_unix_seconds(0),
            FlushPolicyTime::from_unix_seconds(0),
            0,
            pressure,
        ),
    )
    .expect("same recovered store");
    (dir, store, manager)
}

pub(super) fn accept(
    manager: &mut ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
    block: &Block,
    height: u32,
) -> Result<ChainPosition, ChainstateError> {
    manager.connect_block(
        block,
        u128::from(height) + 1,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    )
}

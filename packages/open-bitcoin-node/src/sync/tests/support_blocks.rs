// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

pub(super) fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

pub(super) fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

pub(super) fn build_block(previous_block_hash: BlockHash, height: u32) -> Block {
    let transactions = vec![coinbase_transaction(height, 50)];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

pub(super) fn build_branch_block(
    previous_block_hash: BlockHash,
    height: u32,
    time_offset: u32,
) -> Block {
    let mut block = build_block(previous_block_hash, height);
    block.header.time = block.header.time.saturating_add(time_offset);
    mine_header(&mut block);
    block
}

pub(super) fn getdata_block_hashes(messages: &[WireNetworkMessage]) -> Vec<BlockHash> {
    let mut hashes = Vec::new();
    for message in messages {
        let WireNetworkMessage::GetData(inventory) = message else {
            continue;
        };
        for item in &inventory.inventory {
            if matches!(
                item.inventory_type,
                InventoryType::Block | InventoryType::WitnessBlock
            ) {
                hashes.push(BlockHash::from(item.object_hash));
            }
        }
    }
    hashes
}

pub(super) fn notfound_for_block(block_hash: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash.into(),
    }]))
}

pub(super) fn block_hash_hex(block_hash: BlockHash) -> String {
    encode_hex(block_hash.as_bytes())
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

pub(super) fn save_best_chain_with_active_blocks(
    path: &Path,
    best_chain: &[(&Block, u32)],
    active_chain: &[(&Block, u32)],
) {
    save_chain_headers_snapshot_and_blocks(path, best_chain, active_chain, active_chain);
}

pub(super) fn save_chain_headers_snapshot_and_blocks(
    path: &Path,
    best_chain: &[(&Block, u32)],
    active_chain: &[(&Block, u32)],
    stored_blocks: &[(&Block, u32)],
) {
    let store = FjallNodeStore::open(path).expect("store");
    let header_entries = best_chain
        .iter()
        .map(|(block, height)| HeaderEntry {
            block_hash: block_hash(&block.header),
            header: block.header.clone(),
            height: *height,
            chain_work: u128::from(*height).saturating_add(1),
        })
        .collect::<Vec<_>>();
    store
        .save_header_entries(&header_entries, PersistMode::Sync)
        .expect("save best-chain headers");
    let active_positions = active_chain
        .iter()
        .map(|(block, height)| {
            ChainPosition::new(
                block.header.clone(),
                *height,
                u128::from(*height).saturating_add(1),
                i64::from(block.header.time),
            )
        })
        .collect::<Vec<_>>();
    store
        .save_chainstate_snapshot(
            &ChainstateSnapshot::new(active_positions, Default::default(), Default::default()),
            PersistMode::Sync,
        )
        .expect("save active chain snapshot");
    for (block, _) in stored_blocks {
        store
            .save_block(block, PersistMode::Sync)
            .expect("save stored block");
    }
}

pub(super) fn phase70_branch_blocks() -> (Block, Block, Block, Block, Block, Block) {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let branch_a_one = build_block(block_hash(&genesis.header), 1);
    let branch_a_two = build_block(block_hash(&branch_a_one.header), 2);
    let branch_b_one = build_branch_block(block_hash(&genesis.header), 1, 100);
    let branch_b_two = build_branch_block(block_hash(&branch_b_one.header), 2, 100);
    let branch_b_three = build_branch_block(block_hash(&branch_b_two.header), 3, 100);

    (
        genesis,
        branch_a_one,
        branch_a_two,
        branch_b_one,
        branch_b_two,
        branch_b_three,
    )
}

pub(super) fn phase70_save_reorg_ready_branch(
    path: &Path,
) -> (Block, Block, Block, Block, Block, Block) {
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    {
        let store = FjallNodeStore::open(path).expect("store");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 2,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    genesis.header.clone(),
                    branch_a_one.header.clone(),
                    branch_a_two.header.clone(),
                ],
            }),
            WireNetworkMessage::Block(genesis.clone()),
            WireNetworkMessage::Block(branch_a_one.clone()),
            WireNetworkMessage::Block(branch_a_two.clone()),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_a_two.header.time))
            .expect("initial branch sync");
    }

    {
        let store = FjallNodeStore::open(path).expect("reopen store for durable branch");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    branch_b_one.header.clone(),
                    branch_b_two.header.clone(),
                    branch_b_three.header.clone(),
                ],
            }),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_b_three.header.time))
            .expect("persist better branch headers");
        runtime
            .store()
            .save_block(&branch_b_one, PersistMode::Sync)
            .expect("save branch b one");
        runtime
            .store()
            .save_block(&branch_b_two, PersistMode::Sync)
            .expect("save branch b two");
        runtime
            .store()
            .save_block(&branch_b_three, PersistMode::Sync)
            .expect("save branch b three");
    }

    (
        genesis,
        branch_a_one,
        branch_a_two,
        branch_b_one,
        branch_b_two,
        branch_b_three,
    )
}

pub(super) fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    let mut header = BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_231_006_500 + nonce,
        bits: EASY_BITS,
        nonce,
    };
    let nonce = (0..=u32::MAX)
        .find(|candidate| {
            header.nonce = *candidate;
            check_block_header(&header).is_ok()
        })
        .expect("expected nonce at easy target");
    header.nonce = nonce;
    header
}

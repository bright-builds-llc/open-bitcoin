// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/net.cpp

use super::*;
use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CompactBlockPayload, PrefilledTransaction,
    SendCompactMessage, ShortId,
};
use open_bitcoin_core::primitives::*;
use open_bitcoin_network::WireNetworkMessage;
use open_bitcoin_network::{
    AddressAnnouncement, AddressList, CFCheckpt, CFHeaders, CFRequestKind, CFilter, GetCFCheckpt,
    GetCFHeaders, GetCFilters, HeadersMessage, InventoryList, VersionMessage,
};

fn transaction(witness: Vec<Vec<u8>>) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([7; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(witness),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 0,
    }
}

fn assert_serializer_size(message: WireNetworkMessage) -> WireSize {
    // Arrange / Act
    let measured = wire_size(&message).expect("measure");
    let payload = message.encode_payload().expect("payload");
    let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    // Assert
    assert_eq!(
        measured.payload,
        payload.len() as u64,
        "{}",
        message.command_name()
    );
    assert_eq!(
        measured.frame,
        wire.len() as u64,
        "{}",
        message.command_name()
    );
    assert!(measured.peak >= measured.frame);
    measured
}

#[test]
fn phase160_pending_encoding_tiny_exact_peaks() {
    // Arrange / Act
    let ping = wire_size(&WireNetworkMessage::Ping { nonce: 1 }).expect("ping");
    let verack = wire_size(&WireNetworkMessage::Verack).expect("verack");
    // Assert
    assert_eq!((ping.payload, ping.frame, ping.peak), (8, 32, 72));
    assert_eq!((verack.payload, verack.frame, verack.peak), (0, 24, 64));
}

#[test]
fn phase160_pending_encoding_every_encodable_variant_matches_serializer() {
    // Arrange
    let tx = transaction(vec![vec![1, 2], vec![]]);
    let hash = BlockHash::default();
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::default(),
    }]);
    let messages = vec![
        WireNetworkMessage::Version(VersionMessage {
            user_agent: "Δ🦀".into(),
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::WtxidRelay,
        WireNetworkMessage::SendHeaders,
        WireNetworkMessage::GetAddr,
        WireNetworkMessage::Ping { nonce: 1 },
        WireNetworkMessage::Pong { nonce: 2 },
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: 2,
        }),
        WireNetworkMessage::GetHeaders {
            locator: BlockLocator {
                block_hashes: vec![Hash32::default(); 3],
            },
            stop_hash: hash,
        },
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![BlockHeader::default(); 3],
        }),
        WireNetworkMessage::Addr(AddressList {
            addresses: vec![AddressAnnouncement {
                time_unix_seconds: 3,
                address: VersionMessage::default().sender,
            }],
        }),
        WireNetworkMessage::Inv(inventory.clone()),
        WireNetworkMessage::GetData(inventory.clone()),
        WireNetworkMessage::NotFound(inventory),
        WireNetworkMessage::Tx(tx.clone()),
        WireNetworkMessage::Block(Block {
            header: BlockHeader::default(),
            transactions: vec![tx.clone(); 2],
        }),
        WireNetworkMessage::CompactBlock(CompactBlockPayload {
            header: BlockHeader::default(),
            nonce: 2,
            short_ids: vec![ShortId::from_wire_bytes([1; 6]); 2],
            prefilled_transactions: vec![PrefilledTransaction {
                index_delta: 1,
                transaction: tx.clone(),
            }],
        }),
        WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
            block_hash: hash,
            index_deltas: vec![0, 252, 253],
        }),
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash: hash,
            transactions: vec![tx; 2],
        }),
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 4,
            stop_hash: hash,
        }),
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 4,
            stop_hash: hash,
        }),
        WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type: 0,
            stop_hash: hash,
        }),
        WireNetworkMessage::CFilter(CFilter {
            filter_type: 0,
            block_hash: hash,
            filter: vec![1; 253],
        }),
        WireNetworkMessage::CFHeaders(CFHeaders {
            filter_type: 0,
            stop_hash: hash,
            previous_filter_header: Hash32::default(),
            filter_hashes: vec![Hash32::default(); 253],
        }),
        WireNetworkMessage::CFCheckpt(CFCheckpt {
            filter_type: 0,
            stop_hash: hash,
            filter_headers: vec![Hash32::default(); 3],
        }),
    ];
    // Act / Assert
    assert_eq!(messages.len(), 25);
    for message in messages {
        assert_serializer_size(message);
    }
}

#[test]
fn phase160_pending_encoding_compact_size_boundaries_and_max_size() {
    // Arrange / Act / Assert
    for value in [0, 252, 253, 65_535, 65_536, open_bitcoin_codec::MAX_SIZE] {
        let mut bytes = Vec::new();
        open_bitcoin_codec::write_compact_size(&mut bytes, value).expect("codec");
        assert_eq!(compact(value), Ok(bytes.len() as u64));
    }
    assert_eq!(
        compact(open_bitcoin_codec::MAX_SIZE + 1),
        Err(Error::CompactSizeTooLarge)
    );
    assert_eq!(compact(u64::MAX), Err(Error::CompactSizeTooLarge));
    for len in [252, 253, 65_535, 65_536] {
        assert_serializer_size(WireNetworkMessage::Tx(transaction(vec![vec![1; len]])));
        assert_serializer_size(WireNetworkMessage::Version(VersionMessage {
            user_agent: "a".repeat(len),
            ..Default::default()
        }));
    }
}

#[test]
fn phase160_pending_encoding_witness_stack_activation_and_empty_vectors() {
    // Arrange
    let mut tx = transaction(vec![vec![]]);
    tx.inputs.push(transaction(Vec::new()).inputs.remove(0));
    // Act / Assert
    assert_serializer_size(WireNetworkMessage::Tx(tx));
    assert_serializer_size(WireNetworkMessage::Tx(Transaction::default()));
    assert_serializer_size(WireNetworkMessage::Block(Block::default()));
    assert_serializer_size(WireNetworkMessage::BlockTxn(BlockTransactions {
        block_hash: BlockHash::default(),
        transactions: Vec::new(),
    }));
    assert_serializer_size(WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
        block_hash: BlockHash::default(),
        index_deltas: Vec::new(),
    }));
}

#[test]
fn phase160_pending_encoding_existing_addr_and_inventory_limits() {
    // Arrange
    let address = AddressAnnouncement {
        time_unix_seconds: 1,
        address: VersionMessage::default().sender,
    };
    let inventory = InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: Hash32::default(),
    };
    // Act / Assert
    assert_serializer_size(WireNetworkMessage::Addr(AddressList {
        addresses: vec![address.clone(); 64],
    }));
    let excessive = WireNetworkMessage::Addr(AddressList {
        addresses: vec![address; 65],
    });
    assert_eq!(wire_size(&excessive), Err(Error::InvalidMessage));
    assert!(excessive.encode_payload().is_err());
    assert_serializer_size(WireNetworkMessage::Inv(InventoryList::new(vec![
        inventory
            .clone(
            );
        50_000
    ])));
    let excessive = WireNetworkMessage::GetData(InventoryList::new(vec![inventory; 50_001]));
    assert_eq!(wire_size(&excessive), Err(Error::InvalidMessage));
    assert!(excessive.encode_payload().is_err());
    // The outgoing encoder permits more than the incoming 2000-header limit.
    assert_serializer_size(WireNetworkMessage::Headers(HeadersMessage {
        headers: vec![BlockHeader::default(); 2001],
    }));
}

#[test]
fn phase160_pending_encoding_compact_scalar_validation_matches_codec() {
    // Arrange
    let valid = CompactBlockPayload {
        header: BlockHeader::default(),
        nonce: 0,
        short_ids: vec![ShortId::from_wire_bytes([1; 6])],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: transaction(Vec::new()),
        }],
    };
    let mut cases = Vec::new();
    let mut empty = valid.clone();
    empty.short_ids.clear();
    empty.prefilled_transactions.clear();
    cases.push(empty);
    let mut overflow = valid.clone();
    overflow.short_ids = vec![ShortId::from_wire_bytes([0; 6]); 65_535];
    cases.push(overflow);
    let mut out_of_bounds = valid.clone();
    out_of_bounds.prefilled_transactions[0].index_delta = 2;
    cases.push(out_of_bounds);
    let mut bad_delta = valid.clone();
    bad_delta.prefilled_transactions[0].index_delta = 65_536;
    cases.push(bad_delta);
    let mut null_tx = valid;
    null_tx.prefilled_transactions[0].transaction.inputs.clear();
    cases.push(null_tx);
    // Act / Assert
    for block in cases {
        let message = WireNetworkMessage::CompactBlock(block);
        assert!(wire_size(&message).is_err());
        assert!(message.encode_payload().is_err());
    }
    for deltas in [
        vec![65_535, 0],
        vec![u64::MAX],
        vec![open_bitcoin_codec::MAX_SIZE + 1],
    ] {
        let message = WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
            block_hash: BlockHash::default(),
            index_deltas: deltas,
        });
        assert!(wire_size(&message).is_err());
        assert!(message.encode_payload().is_err());
    }
    assert_serializer_size(WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
        block_hash: BlockHash::default(),
        index_deltas: vec![65_535],
    }));
}

#[test]
fn phase160_pending_encoding_checksum_padding_edges() {
    // Arrange / Act / Assert
    for (payload, padded) in [(55, 64), (56, 128), (63, 128), (64, 128)] {
        let message = WireNetworkMessage::CFilter(CFilter {
            filter_type: 0,
            block_hash: BlockHash::default(),
            filter: vec![0; payload - 34],
        });
        let measured = assert_serializer_size(message);
        assert_eq!(measured.payload, payload as u64);
        assert_eq!(
            measured.peak,
            (payload as u64 + padded).max(2 * payload as u64 + 24 + "cfilter".len() as u64)
        );
    }
}

#[test]
fn phase160_pending_encoding_receive_only_requests_rejected() {
    // Arrange / Act / Assert
    for kind in [
        CFRequestKind::Filters,
        CFRequestKind::Headers,
        CFRequestKind::Checkpoints,
    ] {
        let message = WireNetworkMessage::MalformedCFRequest(kind);
        assert_eq!(wire_size(&message), Err(Error::InvalidMessage));
        assert!(message.encode_payload().is_err());
    }
}

#[test]
fn phase160_pending_encoding_nested_final_transaction_peak_is_charged() {
    // Arrange
    let txs = vec![transaction(Vec::new()), transaction(vec![vec![1; 200_000]])];
    let final_len = open_bitcoin_codec::encode_transaction(
        &txs[1],
        open_bitcoin_codec::TransactionEncoding::WithWitness,
    )
    .expect("codec")
    .len() as u64;
    let prefix = 81
        + open_bitcoin_codec::encode_transaction(
            &txs[0],
            open_bitcoin_codec::TransactionEncoding::WithWitness,
        )
        .expect("codec")
        .len() as u64;
    // Act
    let message = WireNetworkMessage::Block(Block {
        header: BlockHeader::default(),
        transactions: txs,
    });
    let measured = assert_serializer_size(message);
    // Assert
    assert!(measured.peak >= prefix + 2 * final_len);
}

#[test]
fn phase160_pending_encoding_local_envelope_exact_and_one_over() {
    // Arrange
    let exact = WireNetworkMessage::Tx(transaction(vec![vec![1; 4_000_000 - 68]]));
    let over = WireNetworkMessage::Tx(transaction(vec![vec![1; 4_000_000 - 67]]));
    // Act
    let measured = assert_serializer_size(exact);
    // Assert
    assert_eq!(
        (measured.payload, measured.frame, measured.peak),
        (4_000_000, 4_000_024, 8_000_064)
    );
    assert_eq!(
        over.encode_payload()
            .expect("codec-only size allowed")
            .len(),
        4_000_001
    );
    assert_eq!(wire_size(&over), Err(Error::LocalPayloadLimit));
    let owner = super::super::FilterTransportOwner::default();
    assert!(matches!(
        owner.encode_pending(1, &over, NetworkMagic::MAINNET),
        Err(Error::LocalPayloadLimit)
    ));
    assert_eq!(
        owner.snapshot().expect("no allocation/reservation"),
        super::super::FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_pending_encoding_valid_maximum_weight_witness_block_fits() {
    use open_bitcoin_core::consensus::context::MinDifficultyRecoveryTarget;
    use open_bitcoin_core::consensus::{
        self, BlockValidationContext, ConsensusParams, ScriptVerifyFlags, SpentOutput,
        TransactionInputContext, TransactionValidationContext,
    };
    // Arrange: an unknown v2 witness program accepts arbitrary witness data under
    // consensus flags. This is deliberately not a standard mempool-policy claim.
    let mut coinbase = transaction(vec![vec![0; 32]]);
    coinbase.inputs[0].previous_output = OutPoint::null();
    coinbase.inputs[0].script_sig = ScriptBuf::from_bytes(vec![1, 1, 0x51]).expect("height1");
    coinbase.outputs[0].value = Amount::from_sats(5_000_000_000).expect("subsidy");
    coinbase.outputs[0].script_pubkey = ScriptBuf::from_bytes(vec![0x51]).expect("true");
    coinbase.outputs.push(TransactionOutput {
        value: Amount::from_sats(0).expect("zero"),
        script_pubkey: ScriptBuf::from_bytes(vec![0; 38]).expect("commitment space"),
    });
    let spend = transaction(vec![vec![1; 1_000_000]]);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            bits: 0x207f_ffff,
            time: 2,
            ..Default::default()
        },
        transactions: vec![coinbase, spend],
    };
    let mut stripped = block.clone();
    for tx in &mut stripped.transactions {
        for input in &mut tx.inputs {
            input.witness = ScriptWitness::default();
        }
    }
    let stripped_len = open_bitcoin_codec::encode_block(&stripped)
        .expect("stripped")
        .len();
    drop(stripped);
    let initial_len = open_bitcoin_codec::encode_block(&block)
        .expect("initial")
        .len();
    let witness_len = consensus::MAX_BLOCK_WEIGHT - 3 * stripped_len - initial_len + 1_000_000;
    block.transactions[1].inputs[0].witness = ScriptWitness::new(vec![vec![1; witness_len]]);
    let wtxid = consensus::crypto::transaction_wtxid(&block.transactions[1]).expect("wtxid");
    let mut pair = [0; 64];
    pair[32..].copy_from_slice(wtxid.as_bytes());
    let witness_root = consensus::crypto::double_sha256(&pair);
    let mut commitment_preimage = [0; 64];
    commitment_preimage[..32].copy_from_slice(&witness_root);
    let commitment = consensus::crypto::double_sha256(&commitment_preimage);
    let mut script = vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
    script.extend_from_slice(&commitment);
    block.transactions[0].outputs[1].script_pubkey =
        ScriptBuf::from_bytes(script).expect("commitment");
    block.header.merkle_root = consensus::block_merkle_root(&block.transactions)
        .expect("merkle")
        .0;
    while consensus::check_block_header(&block.header).is_err() {
        block.header.nonce += 1;
    }
    let context = BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: block.header.bits,
            time: 1,
            ..Default::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
            bits: block.header.bits,
        }),
        previous_median_time_past: 1,
        current_time: 2,
        consensus_params: ConsensusParams::default(),
    };
    let transaction_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(1).expect("value"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x52, 2, 1, 1]).expect("unknown v2"),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 2,
        median_time_past: 1,
        verify_flags: ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::TAPROOT,
        consensus_params: context.consensus_params,
    };
    // Act
    let validation =
        consensus::validate_block_with_context(&block, &[transaction_context], &context);
    let message = WireNetworkMessage::Block(block);
    let measured = assert_serializer_size(message.clone());
    let owner = super::super::FilterTransportOwner::default();
    let encoded = owner
        .encode_pending(1, &message, NetworkMagic::MAINNET)
        .expect("maximum valid block admitted");
    // Assert
    assert_eq!(validation, Ok(()));
    assert_eq!(
        3 * stripped_len as u64 + measured.payload,
        consensus::MAX_BLOCK_WEIGHT as u64
    );
    assert!(measured.payload > 3_999_000);
    assert!(measured.payload <= MAX_GENERIC_PAYLOAD);
    assert_eq!(
        owner.snapshot().expect("retained").generic_pending_bytes,
        encoded.bytes().len() as u64
    );
    drop(encoded);
    assert_eq!(owner.snapshot().expect("released").generic_pending_bytes, 0);
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::*;

pub(super) fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
}

pub(super) fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

pub(super) fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

pub(super) fn sample_tip(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 1_700_000_000 + height,
            bits: EASY_BITS,
            nonce: 1,
        },
        height,
        1,
        i64::from(1_700_000_000 + height),
    )
}

pub(super) fn wallet_with_descriptors() -> Wallet {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change descriptor");
    wallet
}

pub(super) fn funded_snapshot(wallet: &Wallet) -> ChainstateSnapshot {
    let receive_script = wallet
        .default_receive_address()
        .expect("receive address")
        .script_pubkey;
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([7_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(75_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 9,
            created_median_time_past: 1_700_000_009,
        },
    );

    ChainstateSnapshot::new(vec![sample_tip(10)], utxos, Default::default())
}

pub(super) fn serialized_script_num(value: i64) -> Vec<u8> {
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

pub(super) fn coinbase_transaction(
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Transaction {
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
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey,
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
        .expect("nonce");
}

pub(super) fn build_block(
    previous_block_hash: BlockHash,
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Block {
    let transactions = vec![coinbase_transaction(height, value, script_pubkey)];
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

pub(super) fn spend_transaction(previous_txid: Txid, value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

pub(super) fn script_heavy_spend_transaction(previous_txid: Txid, value: i64) -> Transaction {
    let mut datacarrier = vec![0x6a, 0x4c, 80];
    datacarrier.extend(std::iter::repeat_n(0xab_u8, 80));
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![
            TransactionOutput {
                value: Amount::from_sats(value).expect("amount"),
                script_pubkey: p2sh_script(),
            },
            TransactionOutput {
                value: Amount::from_sats(0).expect("zero"),
                script_pubkey: script(&datacarrier),
            },
        ],
        lock_time: 0,
    }
}

pub(super) fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

pub(super) fn decode_hex(text: &str) -> Vec<u8> {
    let trimmed = text.trim();
    trimmed
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).expect("hex") as u8;
            let low = char::from(pair[1]).to_digit(16).expect("hex") as u8;
            (high << 4) | low
        })
        .collect()
}

pub(super) fn empty_context() -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

pub(super) fn relay_enabled_context(nonce: u64) -> ManagedRpcContext {
    let local_config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 18_444,
        },
        nonce,
        relay: true,
        user_agent: "/open-bitcoin:rpc-test/".to_string(),
    };
    let network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config,
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    );
    let wallet = ManagedWallet::from_store(
        MemoryWalletStore::default(),
        Wallet::new(AddressNetwork::Regtest),
    );
    ManagedRpcContext::new(
        AddressNetwork::Regtest,
        ConsensusParams {
            coinbase_maturity: 1,
            ..ConsensusParams::default()
        },
        rpc_verify_flags(),
        network,
        wallet,
    )
}

pub(super) fn rpc_verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::STRICTENC
        | ScriptVerifyFlags::DERSIG
        | ScriptVerifyFlags::LOW_S
        | ScriptVerifyFlags::NULLDUMMY
        | ScriptVerifyFlags::SIGPUSHONLY
        | ScriptVerifyFlags::MINIMALDATA
        | ScriptVerifyFlags::CLEANSTACK
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
        | ScriptVerifyFlags::WITNESS
        | ScriptVerifyFlags::MINIMALIF
        | ScriptVerifyFlags::NULLFAIL
        | ScriptVerifyFlags::WITNESS_PUBKEYTYPE
        | ScriptVerifyFlags::TAPROOT
}

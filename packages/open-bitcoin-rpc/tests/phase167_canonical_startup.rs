// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp
// - packages/bitcoin-knots/test/functional/mining_basic.py

use open_bitcoin_codec::{block::encode_block_view, block::wire::WireBlock, encode_block};
use open_bitcoin_node::core::{
    consensus::{CanonicalChain, block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use serde_json::{Value, json};
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

open_bitcoin_node::register_inspection_worker!();

#[cfg(unix)]
#[path = "phase167_canonical_startup/basic_evidence.rs"]
mod basic_evidence;
#[cfg(unix)]
use basic_evidence::{stable, stopped};

static SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned fixture nonce");
        let path = std::env::temp_dir()
            .canonicalize()
            .expect("physical temp")
            .join(format!("phase167-http-{:x}", u128::from_le_bytes(nonce)));
        fs::create_dir(&path).expect("fixture");
        fs::create_dir(path.join("source")).expect("explicit empty source");
        Self(path)
    }
    fn command(&self, port: u16) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoind"));
        command
            .env_clear()
            .env("HOME", &self.0)
            .current_dir(&self.0)
            .args([
                format!("-datadir={}", self.0.join("source").display()),
                "-chain=regtest".to_owned(),
                "-server=1".to_owned(),
                "-rpcbind=127.0.0.1".to_owned(),
                format!("-rpcport={port}"),
                "-rpcuser=private".to_owned(),
                "-rpcpassword=secret".to_owned(),
                "-blockfilterindex=1".to_owned(),
            ]);
        command
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("owned fixture after all handles drop");
    }
}
struct Daemon {
    child: Child,
    port: u16,
    diagnostic: PathBuf,
}
impl Daemon {
    #[cfg(unix)]
    fn start(fixture: &Fixture, label: &str) -> Self {
        for attempt in 0..8 {
            let reservation = TcpListener::bind("127.0.0.1:0").expect("local port");
            let port = reservation.local_addr().expect("port").port();
            drop(reservation);
            let diagnostic = fixture.0.join(format!("{label}-{attempt}.stderr"));
            let output = fs::File::create(&diagnostic).expect("owned diagnostic sink");
            let child = fixture
                .command(port)
                .stdout(Stdio::null())
                .stderr(Stdio::from(output))
                .spawn()
                .expect("fresh Cargo product executable");
            let mut daemon = Self {
                child,
                port,
                diagnostic,
            };
            let deadline = Instant::now() + Duration::from_secs(300);
            loop {
                if let Some(status) = daemon.child.try_wait().expect("owned status") {
                    let diagnostic = daemon.diagnostic();
                    if diagnostic.contains("Address already in use") {
                        break;
                    }
                    panic!("daemon startup {status}: {diagnostic}");
                }
                if basic_evidence::rpc_ready(port) {
                    return daemon;
                }
                assert!(
                    Instant::now() < deadline,
                    "bounded startup: {}",
                    daemon.diagnostic()
                );
                std::thread::sleep(Duration::from_millis(20));
            }
        }
        panic!("bounded RPC port reservation failed");
    }
    #[cfg(unix)]
    fn stop(mut self) {
        assert!(
            Command::new("/bin/kill")
                .args(["-INT", &self.child.id().to_string()])
                .status()
                .expect("graceful owned signal")
                .success()
        );
        let deadline = Instant::now() + Duration::from_secs(120);
        loop {
            if let Some(status) = self.child.try_wait().expect("owned shutdown") {
                assert!(status.success(), "shutdown: {}", self.diagnostic());
                break;
            }
            assert!(
                Instant::now() < deadline,
                "bounded shutdown: {}",
                self.diagnostic()
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        self.child
            .wait()
            .expect("fully reap before any storage observation");
    }
}
impl Drop for Daemon {
    fn drop(&mut self) {
        if self.child.try_wait().expect("owned status").is_none() {
            self.child.kill().expect("cleanup only this owned child");
        }
        self.child.wait().expect("reap owned child");
    }
}
fn rpc(port: u16, method: &str, params: Value, authorized: bool) -> (u16, Value) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("local RPC");
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .expect("bounded write");
    let body = json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}).to_string();
    let auth = if authorized {
        "Authorization: Basic cHJpdmF0ZTpzZWNyZXQ=\r\n"
    } else {
        ""
    };
    let request = format!(
        "POST / HTTP/1.1\r\nHost: localhost\r\n{auth}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes()).expect("request");
    let deadline = Instant::now() + Duration::from_secs(300);
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        assert!(!remaining.is_zero(), "bounded RPC response");
        stream
            .set_read_timeout(Some(remaining))
            .expect("read bound");
        let received = stream.read(&mut chunk).expect("response before deadline");
        assert!(Instant::now() < deadline, "bounded RPC response");
        if received == 0 {
            break;
        }
        assert!(bytes.len() + received <= 1_048_576, "bounded response");
        bytes.extend_from_slice(&chunk[..received]);
    }
    let text = String::from_utf8(bytes).expect("HTTP UTF8");
    let (head, payload) = text.split_once("\r\n\r\n").expect("HTTP framing");
    let status_text = head.split_whitespace().nth(1).expect("status");
    let status: u16 = status_text.parse().expect("numeric status");
    (
        status,
        if status == 401 {
            Value::Null
        } else {
            serde_json::from_str(payload).expect("RPC JSON")
        },
    )
}
fn call(daemon: &Daemon, method: &str, params: Value) -> Value {
    let (status, value) = rpc(daemon.port, method, params, true);
    assert_eq!(status, 200);
    value
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn raw_submit(daemon: &Daemon, bytes: &[u8]) -> Value {
    call(daemon, "submitblock", json!([hex(bytes)]))
}
fn accepted(daemon: &Daemon, block: &Block) {
    let result = raw_submit(
        daemon,
        &encode_block(block).expect("complete serialization"),
    );
    assert!(result["error"].is_null(), "{result}");
    assert!(result["result"].is_null(), "{result}");
}
fn expect_error(value: Value, code: i64) {
    assert_eq!(value["error"]["code"], code, "{value}");
}
fn expect_reason(value: Value, reason: &str) {
    assert!(value["error"].is_null(), "{value}");
    assert_eq!(value["result"], reason, "{value}");
}
fn remine_wire(block: &mut WireBlock) {
    block.header.merkle_root =
        open_bitcoin_node::core::consensus::crypto::block_merkle_root_view(&block.transactions)
            .expect("raw merkle")
            .0;
    block.header.nonce = (0..100_000)
        .find(|n| {
            block.header.nonce = *n;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded actual PoW");
}
fn child(parent: &BlockHeader, height: u32) -> Block {
    let mut number = Vec::new();
    let mut value = height;
    while value > 0 {
        number.push(value as u8);
        value >>= 8;
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut script = if height <= 16 {
        vec![0x50 + height as u8]
    } else {
        let mut encoded = vec![number.len() as u8];
        encoded.extend(number);
        encoded
    };
    script.push(0);
    let transaction = Transaction {
        version: 1,
        lock_time: height,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(script).expect("height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000 >> (height / 150))
                .expect("independent pinned subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("fixture true"),
        }],
    };
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: block_hash(parent),
            merkle_root: Default::default(),
            time: parent.time + 1,
            bits: parent.bits,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    remine(&mut block);
    block
}
fn remine(block: &mut Block) {
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("fixture merkle")
        .0;
    block.header.nonce = (0..100_000)
        .find(|n| {
            block.header.nonce = *n;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded real regtest PoW");
}

fn combined_amount_defects(daemon: &Daemon, block: &Block, amount: i64) {
    let bytes = encode_block(block).expect("complete fixture wire");
    let (mut raw, _) = open_bitcoin_codec::block::wire::parse_submission_block_prefix(&bytes)
        .expect("raw data only");
    raw.transactions[0].outputs[0].value_sats = amount;
    raw.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);
    remine_wire(&mut raw);
    let mut truncated = encode_block_view(&raw).expect("signed wire");
    truncated.pop();
    expect_error(raw_submit(daemon, &truncated), -22);
    let mut malformed = encode_block_view(&raw).expect("full witness framing");
    assert_eq!(&malformed[85..87], &[0, 1]);
    malformed[86] = 2;
    expect_error(raw_submit(daemon, &malformed), -22);
    let reason = if amount < 0 {
        "bad-txns-vout-negative"
    } else {
        "bad-txns-vout-toolarge"
    };
    expect_reason(
        raw_submit(daemon, &encode_block_view(&raw).expect("complete raw")),
        reason,
    );
    let mut bad_pow = raw.clone();
    bad_pow.header.nonce = (0..100_000)
        .find(|n| {
            bad_pow.header.nonce = *n;
            check_block_header(&bad_pow.header).is_err()
        })
        .expect("real bad proof control");
    expect_reason(
        raw_submit(
            daemon,
            &encode_block_view(&bad_pow).expect("bad PoW signed wire"),
        ),
        "high-hash",
    );
    let mut wrong_merkle = raw.clone();
    wrong_merkle.header.merkle_root = Default::default();
    wrong_merkle.header.nonce = (0..100_000)
        .find(|n| {
            wrong_merkle.header.nonce = *n;
            check_block_header(&wrong_merkle.header).is_ok()
        })
        .expect("real valid PoW with wrong merkle");
    expect_reason(
        raw_submit(
            daemon,
            &encode_block_view(&wrong_merkle).expect("bad merkle signed wire"),
        ),
        "bad-txnmrklroot",
    );
    raw.transactions[0].inputs[0].previous_output = OutPoint {
        txid: open_bitcoin_node::core::primitives::Txid::from_byte_array([0x45; 32]),
        vout: 0,
    };
    remine_wire(&mut raw);
    expect_reason(
        raw_submit(
            daemon,
            &encode_block_view(&raw).expect("noncoinbase signed wire"),
        ),
        "bad-cb-missing",
    );
}

#[cfg(unix)]
#[test]
fn phase167_one_block_stopped_reproof_and_daemon_restart() {
    let _scenario = SCENARIO.lock().expect("process scenario");
    // Arrange
    let fixture = Fixture::new();
    let daemon = Daemon::start(&fixture, "one-block");
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let block = child(&genesis.header, 1);
    let hash = block_hash(&block.header);
    let mut display = hash.to_byte_array();
    display.reverse();
    let expected_hash = hex(&display);
    // Act
    accepted(&daemon, &block);
    let first = call(&daemon, "getblockchaininfo", json!([]));
    assert_eq!(first["result"]["blocks"], 1);
    assert_eq!(first["result"]["bestblockhash"], expected_hash);
    let filter = call(
        &daemon,
        "getblockfilter",
        json!([expected_hash.clone(), "basic"]),
    );
    assert!(filter["error"].is_null());
    daemon.stop();
    let (facts, retained) = stopped(&fixture, hash);
    // Assert: proved stopped state then the same product binary/datadir restarts.
    let tip = facts.0.0.tip().expect("reproved tip");
    assert_eq!(tip.height, 1);
    assert_eq!(tip.block_hash, hash);
    assert_eq!(tip.chain_work.to_hex(), format!("{:064x}", 4u64));
    assert_eq!(retained, block);
    drop(facts);
    let daemon = Daemon::start(&fixture, "one-block-restart");
    let reopened = call(&daemon, "getblockchaininfo", json!([]));
    assert_eq!(reopened["result"]["blocks"], 1);
    assert_eq!(reopened["result"]["bestblockhash"], expected_hash);
    let reopened_filter = call(&daemon, "getblockfilter", json!([expected_hash, "basic"]));
    assert_eq!(reopened_filter, filter);
    daemon.stop();
}

#[cfg(unix)]
#[test]
fn phase167_canonical_startup_authenticated_301_wide_body_stop_reap_reopen_next() {
    let _scenario = SCENARIO.lock().expect("process scenario");
    // Arrange
    let fixture = Fixture::new();
    let daemon = Daemon::start(&fixture, "initial");
    assert_eq!(
        rpc(daemon.port, "getblockchaininfo", json!([]), false).0,
        401
    );
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut parent = genesis.header.clone();
    let mut oracle_cases = vec![basic_evidence::oracle_case(&genesis, 0)];
    // Act: real serialized body submission, no preloaded/caller earned state.
    for height in 1..=301 {
        let block = child(&parent, height);
        oracle_cases.push(basic_evidence::oracle_case(&block, height));
        accepted(&daemon, &block);
        parent = block.header;
    }
    let mut wide = child(&parent, 302);
    wide.transactions[0].outputs[0].script_pubkey =
        ScriptBuf::from_serialized_bytes(vec![0x51; 10_001]);
    remine(&mut wide);
    accepted(&daemon, &wide);
    let hash = block_hash(&wide.header);
    let txid = open_bitcoin_node::core::consensus::transaction_txid(&wide.transactions[0])
        .expect("wide raw-preserving txid");
    oracle_cases.push(basic_evidence::oracle_case(&wide, 302));
    let next = child(&wide.header, 303);
    oracle_cases.push(basic_evidence::oracle_case(&next, 303));
    let oracle = basic_evidence::oracle(&fixture, &oracle_cases);
    let expected = &oracle[302];
    let info = call(&daemon, "getblockchaininfo", json!([]));
    assert!(info["error"].is_null());
    assert_eq!(info["result"]["blocks"], 302);
    assert_eq!(
        info["result"]["bestblockhash"],
        expected["block_hash_display"]
    );
    let filter = call(
        &daemon,
        "getblockfilter",
        json!([info["result"]["bestblockhash"], "basic"]),
    );
    assert!(filter["error"].is_null());
    assert_eq!(filter["result"]["filter"], expected["encoded"]);
    assert_eq!(filter["result"]["header"], expected["header_display"]);
    let wallet = call(&daemon, "getwalletinfo", json!([]));
    assert!(!wallet["error"].is_null());
    let network = call(&daemon, "getnetworkinfo", json!([]));
    assert_eq!(network["result"]["connections"], 0);
    let mut duplicate = wide.clone();
    duplicate.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0x55; 32]]);
    expect_reason(
        raw_submit(
            &daemon,
            &encode_block(&duplicate).expect("witness-only bytes"),
        ),
        "duplicate",
    );
    let mut corrupt = wide.clone();
    corrupt.transactions[0].outputs[0].value = Amount::from_sats(1).expect("valid amount data");
    expect_reason(
        raw_submit(
            &daemon,
            &encode_block(&corrupt).expect("base corruption bytes"),
        ),
        "bad-txnmrklroot",
    );
    assert_eq!(call(&daemon, "getblockchaininfo", json!([])), info);
    daemon.stop();
    // Assert: these are STOPPED reconstructed observations, not live child internals.
    let (before, retained) = stopped(&fixture, hash);
    basic_evidence::assert_basic_facts(&before, &wide, expected, 302);
    assert_eq!(
        encode_block(&retained).expect("exact retained encoding"),
        encode_block(&wide).expect("original wide encoding")
    );
    assert_eq!(
        before.0.0.tip().expect("tip").chain_work.to_hex(),
        format!("{:064x}", 606u64)
    );
    assert!(!before.0.0.utxos.contains_key(&OutPoint { txid, vout: 0 }));
    assert!(!before.0.2.coins.contains_key(&OutPoint { txid, vout: 0 }));
    assert!(
        before.0.0.undo_by_block[&hash]
            .transactions
            .iter()
            .all(|undo| undo.restored_inputs.iter().all(|coin| coin
                .output
                .script_pubkey
                .as_bytes()
                .len()
                <= 10_000))
    );
    let saved = stable(&before);
    drop(before);
    let daemon = Daemon::start(&fixture, "restart");
    assert_eq!(call(&daemon, "getblockchaininfo", json!([])), info);
    assert_eq!(
        call(
            &daemon,
            "getblockfilter",
            json!([info["result"]["bestblockhash"], "basic"])
        ),
        filter
    );
    daemon.stop();
    let (reopened, again) = stopped(&fixture, hash);
    basic_evidence::assert_basic_facts(&reopened, &wide, expected, 302);
    assert_eq!(stable(&reopened), saved);
    assert_eq!(again, wide);
    drop(reopened);
    let daemon = Daemon::start(&fixture, "next");
    accepted(&daemon, &next);
    let next_info = call(&daemon, "getblockchaininfo", json!([]));
    assert_eq!(next_info["result"]["blocks"], 303);
    assert_eq!(
        next_info["result"]["bestblockhash"],
        oracle[303]["block_hash_display"]
    );
    let next_filter = call(
        &daemon,
        "getblockfilter",
        json!([next_info["result"]["bestblockhash"], "basic"]),
    );
    assert!(next_filter["error"].is_null());
    assert_eq!(next_filter["result"]["filter"], oracle[303]["encoded"]);
    assert_eq!(
        next_filter["result"]["header"],
        oracle[303]["header_display"]
    );
    daemon.stop();
    let (next_facts, next_retained) = stopped(&fixture, block_hash(&next.header));
    basic_evidence::assert_basic_facts(&next_facts, &next, &oracle[303], 303);
    assert_eq!(next_retained, next);
    assert_eq!(
        next_facts.0.0.tip().expect("work").chain_work.to_hex(),
        format!("{:064x}", 608u64)
    );
    drop(next_facts);
    assert!(!fixture.0.join("source/wallet.dat").exists());
}

#[cfg(unix)]
#[test]
fn phase167_canonical_startup_real_http_help_decode_and_ordered_combined_defects() {
    let _scenario = SCENARIO.lock().expect("process scenario");
    // Arrange
    let fixture = Fixture::new();
    let daemon = Daemon::start(&fixture, "boundaries");
    let before = call(&daemon, "getblockchaininfo", json!([]));
    for params in [json!([]), json!([null, null, null]), json!({})] {
        let response = call(&daemon, "submitblock", params);
        assert_eq!(response["error"]["code"], -1);
        let help = response["error"]["message"]
            .as_str()
            .expect("complete pinned help");
        assert_eq!(help.len(), 683);
        assert_eq!(
            open_bitcoin_node::core::consensus::crypto::double_sha256(help.as_bytes()),
            [
                45, 250, 224, 125, 63, 174, 161, 41, 20, 204, 222, 86, 133, 102, 110, 49, 47, 9,
                82, 81, 71, 123, 52, 137, 123, 255, 34, 74, 5, 214, 215, 72
            ]
        );
    }
    expect_error(call(&daemon, "submitblock", json!([null])), -3);
    expect_error(call(&daemon, "submitblock", json!({"unknown":"00"})), -8);
    for bad in ["", "0", "gg", " 00", "00\n"] {
        expect_error(call(&daemon, "submitblock", json!([bad])), -22);
    }
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut suffix = encode_block(&genesis).expect("genesis bytes");
    suffix.extend([0xff, 0x00]);
    expect_reason(raw_submit(&daemon, &suffix), "duplicate");
    let block = child(&genesis.header, 1);
    for amount in [-1, 2_100_000_000_000_001] {
        combined_amount_defects(&daemon, &block, amount);
    }
    let mut empty = WireBlock {
        header: block.header.clone(),
        transactions: vec![],
    };
    remine_wire(&mut empty);
    expect_reason(
        raw_submit(
            &daemon,
            &encode_block_view(&empty).expect("empty serialized"),
        ),
        "bad-blk-length",
    );
    let mut no_coinbase = block.clone();
    no_coinbase.transactions[0].inputs[0].previous_output = OutPoint {
        txid: open_bitcoin_node::core::primitives::Txid::from_byte_array([0x45; 32]),
        vout: 0,
    };
    remine(&mut no_coinbase);
    expect_reason(
        raw_submit(
            &daemon,
            &encode_block(&no_coinbase).expect("noncoinbase serialized"),
        ),
        "bad-cb-missing",
    );
    let mut missing_parent = block;
    missing_parent.header.previous_block_hash = BlockHash::from_byte_array([0x66; 32]);
    remine(&mut missing_parent);
    expect_reason(
        raw_submit(
            &daemon,
            &encode_block(&missing_parent).expect("missing-parent bytes"),
        ),
        "prev-blk-not-found",
    );
    // Assert: every combined defect refused without accepted tip/work progress.
    assert_eq!(call(&daemon, "getblockchaininfo", json!([])), before);
    daemon.stop();
    assert!(!fixture.0.join("source/wallet.dat").exists());
}

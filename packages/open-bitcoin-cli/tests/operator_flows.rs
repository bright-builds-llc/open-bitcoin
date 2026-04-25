// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use open_bitcoin_node::core::{
    consensus::{
        block_hash, block_merkle_root, check_block_header, crypto::hash160, transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet},
};
use open_bitcoin_rpc::{
    ManagedRpcContext, RpcErrorCode, RpcErrorDetail, RpcFailure,
    config::{RuntimeConfig, WalletRuntimeConfig},
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};
use serde_json::{Value, json};

const EASY_BITS: u32 = 0x207f_ffff;
const RPC_USERNAME: &str = "alice";
const RPC_PASSWORD: &str = "secret";
const BASIC_AUTH_HEADER: &str = "Basic YWxpY2U6c2VjcmV0";

static NEXT_SANDBOX_ID: AtomicU64 = AtomicU64::new(0);

struct TestSandbox {
    home: PathBuf,
}

impl TestSandbox {
    fn new(label: &str) -> Self {
        let home = std::env::temp_dir().join(format!(
            "open-bitcoin-cli-operator-tests-{label}-{}",
            NEXT_SANDBOX_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&home).expect("sandbox");
        Self { home }
    }
}

impl Drop for TestSandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.home);
    }
}

struct RpcTestServer {
    address: SocketAddr,
    stop: std::sync::mpsc::Sender<()>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl RpcTestServer {
    fn start(context: ManagedRpcContext) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let address = listener.local_addr().expect("local addr");
        let (stop, stop_rx) = std::sync::mpsc::channel();
        let shared_context = Arc::new(Mutex::new(context));

        let join_handle = thread::spawn(move || {
            loop {
                if stop_rx.try_recv().is_ok() {
                    break;
                }

                match listener.accept() {
                    Ok((stream, _)) => handle_connection(stream, &shared_context),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("listener failed: {error}"),
                }
            }
        });

        Self {
            address,
            stop,
            join_handle: Some(join_handle),
        }
    }
}

impl Drop for RpcTestServer {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().expect("server thread");
        }
    }
}

fn handle_connection(mut stream: TcpStream, context: &Arc<Mutex<ManagedRpcContext>>) {
    stream.set_nonblocking(false).expect("blocking stream");
    let request = read_request(&mut stream);
    let response = build_response(context, request);
    stream
        .write_all(response.as_bytes())
        .expect("response write");
    stream.flush().expect("response flush");
}

fn read_request(stream: &mut TcpStream) -> HttpRequest {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    let mut header_end = None;
    let mut content_length = 0_usize;
    let deadline = Instant::now() + Duration::from_secs(2);

    loop {
        let bytes_read = stream.read(&mut chunk).expect("read");
        if bytes_read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..bytes_read]);
        if header_end.is_none()
            && let Some(index) = buffer.windows(4).position(|window| window == b"\r\n\r\n")
        {
            header_end = Some(index + 4);
            let header_text = String::from_utf8(buffer[..index].to_vec()).expect("header text");
            content_length = header_text
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    if name.eq_ignore_ascii_case("content-length") {
                        return value.trim().parse::<usize>().ok();
                    }
                    None
                })
                .unwrap_or(0);
        }
        if let Some(header_end) = header_end
            && buffer.len() >= header_end + content_length
        {
            break;
        }
        if Instant::now() >= deadline {
            panic!("timed out waiting for request body");
        }
    }

    let header_end = header_end.expect("header terminator");
    let header_text = String::from_utf8(buffer[..header_end - 4].to_vec()).expect("header");
    let mut lines = header_text.lines();
    let request_line = lines.next().expect("request line");
    let mut request_line_parts = request_line.split_whitespace();
    let method = request_line_parts.next().expect("method").to_string();
    let headers = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_string()))
        .collect::<HashMap<_, _>>();
    let body = buffer[header_end..header_end + content_length].to_vec();

    HttpRequest {
        method,
        headers,
        body,
    }
}

fn build_response(context: &Arc<Mutex<ManagedRpcContext>>, request: HttpRequest) -> String {
    if request.method != "POST" {
        return plain_response(405, "JSONRPC server handles only POST requests");
    }
    if request.headers.get("authorization").map(String::as_str) != Some(BASIC_AUTH_HEADER) {
        return unauthorized_response();
    }

    let value = match serde_json::from_slice::<Value>(&request.body) {
        Ok(value) => value,
        Err(_) => {
            return json_response(
                200,
                json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": RpcErrorCode::ParseError.as_i32(),
                        "message": "Parse error",
                    },
                    "id": Value::Null,
                }),
            );
        }
    };

    match value {
        Value::Object(_) => json_response(200, handle_single_request(context, value)),
        Value::Array(items) => {
            let responses = items
                .into_iter()
                .map(|item| handle_single_request(context, item))
                .collect::<Vec<_>>();
            json_response(200, Value::Array(responses))
        }
        _ => json_response(
            200,
            json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": RpcErrorCode::InvalidRequest.as_i32(),
                    "message": "Invalid Request object",
                },
                "id": Value::Null,
            }),
        ),
    }
}

fn handle_single_request(context: &Arc<Mutex<ManagedRpcContext>>, value: Value) -> Value {
    let Value::Object(object) = value else {
        return json!({
            "jsonrpc": "2.0",
            "error": {
                "code": RpcErrorCode::InvalidRequest.as_i32(),
                "message": "Invalid Request object",
            },
            "id": Value::Null,
        });
    };

    let id = object.get("id").cloned().unwrap_or(Value::Null);
    let Some(method) = object.get("method").and_then(Value::as_str) else {
        return failure_response(id, RpcFailure::invalid_request("Missing method"));
    };
    let params = object.get("params").cloned().unwrap_or(Value::Null);

    let call = match normalize_method_call(method, RequestParameters::from_json(params)) {
        Ok(call) => call,
        Err(failure) => return failure_response(id, failure),
    };

    let mut context = context.lock().expect("context");
    match dispatch(&mut context, call) {
        Ok(result) => json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id,
        }),
        Err(failure) => failure_response(id, failure),
    }
}

fn failure_response(id: Value, failure: RpcFailure) -> Value {
    let detail = failure
        .maybe_detail
        .unwrap_or_else(|| RpcErrorDetail::new(RpcErrorCode::InternalError, "Internal error"));

    json!({
        "jsonrpc": "2.0",
        "error": {
            "code": detail.code.as_i32(),
            "message": detail.message,
        },
        "id": id,
    })
}

fn json_response(status: u16, body: Value) -> String {
    let body_text = body.to_string();
    format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_text(status),
        body_text.len(),
        body_text,
    )
}

fn plain_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_text(status),
        body.len(),
        body,
    )
}

fn unauthorized_response() -> String {
    "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"jsonrpc\"\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
}

fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        401 => "Unauthorized",
        405 => "Method Not Allowed",
        _ => "Internal Server Error",
    }
}

struct HttpRequest {
    method: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
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

fn coinbase_transaction(height: u32, value: i64, script_pubkey: ScriptBuf) -> Transaction {
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

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("nonce");
}

fn build_block(
    previous_block_hash: BlockHash,
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Block {
    build_block_with_transactions(
        previous_block_hash,
        height,
        value,
        script_pubkey,
        Vec::new(),
    )
}

fn build_block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
    mut transactions: Vec<Transaction>,
) -> Block {
    let mut all_transactions = vec![coinbase_transaction(height, value, script_pubkey)];
    all_transactions.append(&mut transactions);
    let (merkle_root, maybe_mutated) = block_merkle_root(&all_transactions).expect("merkle root");
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
        transactions: all_transactions,
    };
    mine_header(&mut block);
    block
}

fn spend_to_script(previous_txid: Txid, value: i64, script_pubkey: ScriptBuf) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
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

fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn wallet_with_descriptors() -> Wallet {
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

fn empty_context() -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

fn operator_context() -> ManagedRpcContext {
    let mut context = empty_context();
    let receive_script = wallet_with_descriptors()
        .default_receive_address()
        .expect("receive address")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        script(&[0x51]),
    );
    let funding_transaction = spend_to_script(
        transaction_txid(&genesis.transactions[0]).expect("genesis txid"),
        75_000,
        receive_script,
    );
    let funding_block = build_block_with_transactions(
        block_hash(&genesis.header),
        1,
        500_000_000,
        script(&[0x51]),
        vec![funding_transaction],
    );
    context.connect_local_block(&genesis).expect("genesis");
    context
        .connect_local_block(&funding_block)
        .expect("funding block");
    context
}

fn run_cli_with_rpc(server: &RpcTestServer, sandbox: &TestSandbox, args: &[String]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoin-cli"));
    command
        .env("HOME", &sandbox.home)
        .arg(format!("-rpcconnect={}", server.address.ip()))
        .arg(format!("-rpcport={}", server.address.port()))
        .arg(format!("-rpcuser={RPC_USERNAME}"))
        .arg(format!("-rpcpassword={RPC_PASSWORD}"));
    for arg in args {
        command.arg(arg);
    }
    command.output().expect("cli output")
}

fn run_raw_cli(sandbox: &TestSandbox, args: &[String]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoin-cli"));
    command.env("HOME", &sandbox.home);
    for arg in args {
        command.arg(arg);
    }
    command.output().expect("cli output")
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout")
}

fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr")
}

fn assert_success_json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        stdout_text(output),
        stderr_text(output),
    );
    serde_json::from_slice(&output.stdout).expect("stdout json")
}

#[test]
fn descriptor_rescan_balance_build_sign_and_send_roundtrip() {
    // Arrange
    let sandbox = TestSandbox::new("roundtrip");
    let server = RpcTestServer::start(operator_context());
    let descriptor_requests = serde_json::to_string(&json!([
        {
            "desc": "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            "label": "receive",
            "internal": false,
            "timestamp": 0,
        },
        {
            "desc": "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            "label": "change",
            "internal": true,
            "timestamp": 0,
        }
    ]))
    .expect("descriptor requests");
    let recipients = serde_json::to_string(&json!([
        {
            "script_pubkey_hex": encode_hex(p2sh_script().as_bytes()),
            "amount_sats": 30_000,
        }
    ]))
    .expect("recipients");

    // Act
    let import_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "-named".to_string(),
            "importdescriptors".to_string(),
            format!("requests={descriptor_requests}"),
        ],
    );
    let rescan_output = run_cli_with_rpc(&server, &sandbox, &["rescanblockchain".to_string()]);
    let balances_output = run_cli_with_rpc(&server, &sandbox, &["getbalances".to_string()]);
    let unspent_output = run_cli_with_rpc(&server, &sandbox, &["listunspent".to_string()]);
    let import_json = assert_success_json(&import_output);
    let rescan_json = assert_success_json(&rescan_output);
    let balances_json = assert_success_json(&balances_output);
    let unspent_json = assert_success_json(&unspent_output);
    let build_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "-named".to_string(),
            "buildandsigntransaction".to_string(),
            format!("recipients={recipients}"),
            "fee_rate_sat_per_kvb=2000".to_string(),
            "replaceable=true".to_string(),
        ],
    );
    let build_json = assert_success_json(&build_output);
    let transaction_hex = build_json["transaction_hex"]
        .as_str()
        .expect("transaction hex")
        .to_string();
    let send_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "-named".to_string(),
            "sendrawtransaction".to_string(),
            format!(
                "hexstring={}",
                serde_json::to_string(&transaction_hex).expect("quoted hex"),
            ),
        ],
    );

    // Assert
    assert_eq!(import_json["results"][0]["success"], json!(true));
    assert_eq!(import_json["results"][1]["success"], json!(true));

    assert_eq!(rescan_json["start_height"], json!(0));
    assert_eq!(rescan_json["stop_height"], json!(1));

    assert_eq!(balances_json["mine"]["trusted_sats"], json!(75_000));

    assert_eq!(unspent_json["entries"][0]["amount_sats"], json!(75_000));

    assert_eq!(build_json["fee_sats"], json!(286));
    assert_eq!(build_json["inputs"][0]["amount_sats"], json!(75_000));

    let send_json = assert_success_json(&send_output);
    let txid_hex = send_json["txid_hex"].as_str().expect("txid hex");
    assert_eq!(txid_hex.len(), 64);
}

#[test]
fn deferred_surfaces_fail_explicitly() {
    // Arrange
    let sandbox = TestSandbox::new("deferred");
    let server = RpcTestServer::start(operator_context());

    // Act
    let sendtoaddress_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "sendtoaddress".to_string(),
            "bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je".to_string(),
            "1".to_string(),
        ],
    );
    let netinfo_output = run_raw_cli(&sandbox, &["-netinfo".to_string()]);
    let rpcwallet_output = run_raw_cli(
        &sandbox,
        &[
            "-rpcwallet=wallet.dat".to_string(),
            "getwalletinfo".to_string(),
        ],
    );

    // Assert
    assert_eq!(sendtoaddress_output.status.code(), Some(1));
    assert_eq!(
        stderr_text(&sendtoaddress_output).trim(),
        "error code -32601: method sendtoaddress is not supported in Phase 8",
    );

    assert_eq!(netinfo_output.status.code(), Some(1));
    assert_eq!(
        stderr_text(&netinfo_output).trim(),
        "-netinfo is deferred until the getpeerinfo-backed network dashboard lands in a later Phase 8 plan.",
    );

    assert_eq!(rpcwallet_output.status.code(), Some(1));
    assert_eq!(
        stderr_text(&rpcwallet_output).trim(),
        "-rpcwallet is deferred until wallet-scoped RPC endpoints land in a later Phase 8 plan.",
    );
}

#[test]
fn normal_cli_without_stdin_flags_does_not_wait_for_open_stdin() {
    // Arrange
    let sandbox = TestSandbox::new("open-stdin");
    let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoin-cli"));
    command
        .env("HOME", &sandbox.home)
        .arg("-rpcconnect=127.0.0.1")
        .arg("-rpcport=1")
        .arg(format!("-rpcuser={RPC_USERNAME}"))
        .arg(format!("-rpcpassword={RPC_PASSWORD}"))
        .arg("getnetworkinfo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn cli");
    let _stdin_guard = child.stdin.take().expect("stdin pipe");
    let deadline = Instant::now() + Duration::from_secs(2);

    // Act
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll cli") {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("open stdin pipe kept no-stdin CLI invocation running");
        }
        thread::sleep(Duration::from_millis(10));
    };
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    child
        .stdout
        .take()
        .expect("stdout pipe")
        .read_to_end(&mut stdout)
        .expect("stdout");
    child
        .stderr
        .take()
        .expect("stderr pipe")
        .read_to_end(&mut stderr)
        .expect("stderr");

    // Assert
    assert_eq!(status.code(), Some(1));
    assert!(stdout.is_empty());
    assert!(
        String::from_utf8(stderr)
            .expect("stderr text")
            .contains("Could not connect to the server 127.0.0.1:1")
    );
}

#[test]
fn parity_catalog_entry_is_tracked() {
    // Arrange
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");

    // Act
    let catalog = fs::read_to_string(repo_root.join("docs/parity/catalog/rpc-cli-config.md"))
        .expect("rpc-cli-config catalog");
    let index = fs::read_to_string(repo_root.join("docs/parity/index.json")).expect("parity index");

    // Assert
    assert!(catalog.contains("buildtransaction"));
    assert!(catalog.contains("buildandsigntransaction"));
    assert!(catalog.contains("sendtoaddress"));
    assert!(catalog.contains("rpcauth"));
    assert!(catalog.contains("rpcwhitelist"));
    assert!(catalog.contains("rpcwallet"));
    assert!(catalog.contains("getpeerinfo"));
    assert!(catalog.contains("-netinfo"));
    assert!(index.contains("rpc-cli-config"));
}

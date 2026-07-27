// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use super::*;

pub(super) const EASY_BITS: u32 = 0x207f_ffff;
pub(super) const RPC_USERNAME: &str = "alice";
pub(super) const RPC_PASSWORD: &str = "secret";
pub(super) const BASIC_AUTH_HEADER: &str = "Basic YWxpY2U6c2VjcmV0";

pub(super) static NEXT_SANDBOX_ID: AtomicU64 = AtomicU64::new(0);

pub(super) struct TestSandbox {
    pub(super) home: PathBuf,
}

impl TestSandbox {
    pub(super) fn new(label: &str) -> Self {
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

pub(super) struct RpcTestServer {
    pub(super) address: SocketAddr,
    pub(super) stop: std::sync::mpsc::Sender<()>,
    pub(super) join_handle: Option<thread::JoinHandle<()>>,
}

impl RpcTestServer {
    pub(super) fn start(context: ManagedRpcContext) -> Self {
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

pub(super) fn handle_connection(mut stream: TcpStream, context: &Arc<Mutex<ManagedRpcContext>>) {
    stream.set_nonblocking(false).expect("blocking stream");
    let request = read_request(&mut stream);
    let response = build_response(context, request);
    stream
        .write_all(response.as_bytes())
        .expect("response write");
    stream.flush().expect("response flush");
}

pub(super) fn read_request(stream: &mut TcpStream) -> HttpRequest {
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

pub(super) fn build_response(
    context: &Arc<Mutex<ManagedRpcContext>>,
    request: HttpRequest,
) -> String {
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

pub(super) fn handle_single_request(
    context: &Arc<Mutex<ManagedRpcContext>>,
    value: Value,
) -> Value {
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

pub(super) fn failure_response(id: Value, failure: RpcFailure) -> Value {
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

pub(super) fn json_response(status: u16, body: Value) -> String {
    let body_text = body.to_string();
    format!(
        "HTTP/1.1 {status} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_text(status),
        body_text.len(),
        body_text,
    )
}

pub(super) fn plain_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_text(status),
        body.len(),
        body,
    )
}

pub(super) fn unauthorized_response() -> String {
    "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"jsonrpc\"\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
}

pub(super) fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        401 => "Unauthorized",
        405 => "Method Not Allowed",
        _ => "Internal Server Error",
    }
}

pub(super) struct HttpRequest {
    pub(super) method: String,
    pub(super) headers: HashMap<String, String>,
    pub(super) body: Vec<u8>,
}

pub(super) fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
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
    build_block_with_transactions(
        previous_block_hash,
        height,
        value,
        script_pubkey,
        Vec::new(),
    )
}

pub(super) fn build_block_with_transactions(
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

pub(super) fn spend_to_script(
    previous_txid: Txid,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Transaction {
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

pub(super) fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
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

pub(super) fn operator_context() -> ManagedRpcContext {
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

pub(super) fn run_cli_with_rpc(
    server: &RpcTestServer,
    sandbox: &TestSandbox,
    args: &[String],
) -> Output {
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

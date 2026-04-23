use std::{
    ffi::OsString,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use open_bitcoin_cli::{
    args::parse_cli_args,
    startup::{CliRpcConfig, CliStartupConfig},
};
use open_bitcoin_rpc::RpcAuthConfig;
use serde_json::json;

use super::execute_parsed_cli;

fn os(value: &str) -> OsString {
    OsString::from(value)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapturedRequest {
    method: String,
    authorization: Option<String>,
    body: serde_json::Value,
}

struct TestServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl TestServer {
    fn start(response_body: serde_json::Value) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        listener
            .set_nonblocking(true)
            .expect("nonblocking listener");
        let address = listener.local_addr().expect("local addr");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured_requests = Arc::clone(&requests);

        let join_handle = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(2);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            return;
                        }
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("request failed: {error}"),
                }
            };
            let request = read_request(&mut stream);
            captured_requests.lock().expect("requests").push(request);

            let body_text = response_body.to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body_text.len(),
                body_text
            )
            .expect("response");
            stream.flush().expect("flush");
        });

        Self {
            address,
            requests,
            join_handle: Some(join_handle),
        }
    }

    fn requests(&self) -> Vec<CapturedRequest> {
        self.requests.lock().expect("requests").clone()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().expect("server thread");
        }
    }
}

fn read_request(stream: &mut TcpStream) -> CapturedRequest {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    let mut header_end = None;
    let mut content_length = 0_usize;

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
    }

    let header_end = header_end.expect("header terminator");
    let header_text = String::from_utf8(buffer[..header_end - 4].to_vec()).expect("header");
    let mut lines = header_text.lines();
    let request_line = lines.next().expect("request line");
    let method = request_line
        .split_whitespace()
        .next()
        .expect("method")
        .to_string();
    let authorization = lines.find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.eq_ignore_ascii_case("authorization") {
            return Some(value.trim().to_string());
        }
        None
    });
    let body = serde_json::from_slice(&buffer[header_end..]).expect("json body");

    CapturedRequest {
        method,
        authorization,
        body,
    }
}

#[test]
fn rpc_errors_surface_exit_code_one_with_actionable_stderr() {
    // Arrange
    let server = TestServer::start(json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32601,
            "message": "method invalidmethod is not supported in Phase 8",
        },
        "id": 1,
    }));
    let parsed = parse_cli_args(&[os("invalidmethod")], "").expect("parsed cli");
    let startup = CliStartupConfig {
        conf_path: std::env::temp_dir().join("bitcoin.conf"),
        maybe_data_dir: None,
        rpc: CliRpcConfig {
            host: "127.0.0.1".to_string(),
            port: server.address.port(),
            auth: RpcAuthConfig::UserPassword {
                username: "alice".to_string(),
                password: "secret".to_string(),
            },
        },
    };

    // Act
    let failure = execute_parsed_cli(&parsed, &startup).expect_err("rpc failure");

    // Assert
    assert_eq!(failure.exit_code, 1);
    assert_eq!(
        failure.stderr,
        "error code -32601: method invalidmethod is not supported in Phase 8",
    );
    assert_eq!(
        server.requests(),
        vec![CapturedRequest {
            method: "POST".to_string(),
            authorization: Some("Basic YWxpY2U6c2VjcmV0".to_string()),
            body: json!({
                "jsonrpc": "2.0",
                "method": "invalidmethod",
                "params": [],
                "id": 1,
            }),
        }],
    );
}

#[test]
fn getinfo_json_mode_is_stable_for_automation() {
    // Arrange
    let server = TestServer::start(json!([
        {
            "jsonrpc": "2.0",
            "result": {
                "version": 293000,
                "subversion": "/OpenBitcoin:0.1.0/",
                "protocolversion": 70016,
                "localservices": "NETWORK",
                "localrelay": true,
                "connections": 5,
                "connections_in": 2,
                "connections_out": 3,
                "relayfee": 1000,
                "incrementalfee": 1000,
                "warnings": [],
            },
            "id": 0,
        },
        {
            "jsonrpc": "2.0",
            "result": {
                "chain": "regtest",
                "blocks": 144,
                "headers": 144,
                "bestblockhash": "0000000000000000000000000000000000000000000000000000000000000000",
                "mediantime": 1714007000,
                "verificationprogress": 0.995,
                "initialblockdownload": false,
                "warnings": [],
            },
            "id": 1,
        },
        {
            "jsonrpc": "2.0",
            "result": {
                "network": "regtest",
                "descriptor_count": 4,
                "utxo_count": 2,
                "maybe_tip_height": 144,
                "maybe_tip_median_time_past": 1714007000,
            },
            "id": 2,
        },
        {
            "jsonrpc": "2.0",
            "result": {
                "mine": {
                    "trusted_sats": 125000000,
                    "untrusted_pending_sats": 0,
                    "immature_sats": 0,
                },
            },
            "id": 3,
        }
    ]));
    let parsed = parse_cli_args(&[os("-getinfo"), os("--json")], "").expect("parsed cli");
    let startup = CliStartupConfig {
        conf_path: std::env::temp_dir().join("bitcoin.conf"),
        maybe_data_dir: None,
        rpc: CliRpcConfig {
            host: "127.0.0.1".to_string(),
            port: server.address.port(),
            auth: RpcAuthConfig::UserPassword {
                username: "alice".to_string(),
                password: "secret".to_string(),
            },
        },
    };

    // Act
    let output = execute_parsed_cli(&parsed, &startup).expect("getinfo output");

    // Assert
    assert_eq!(
        output,
        r#"{
  "network": {
    "version": 293000,
    "subversion": "/OpenBitcoin:0.1.0/",
    "protocolversion": 70016,
    "localservices": "NETWORK",
    "localrelay": true,
    "connections": 5,
    "connections_in": 2,
    "connections_out": 3,
    "relayfee": 1000,
    "incrementalfee": 1000,
    "warnings": []
  },
  "blockchain": {
    "chain": "regtest",
    "blocks": 144,
    "headers": 144,
    "bestblockhash": "0000000000000000000000000000000000000000000000000000000000000000",
    "mediantime": 1714007000,
    "verificationprogress": 0.995,
    "initialblockdownload": false,
    "warnings": []
  },
  "wallet": {
    "network": "regtest",
    "descriptor_count": 4,
    "utxo_count": 2,
    "maybe_tip_height": 144,
    "maybe_tip_median_time_past": 1714007000
  },
  "balances": {
    "mine": {
      "trusted_sats": 125000000,
      "untrusted_pending_sats": 0,
      "immature_sats": 0
    }
  }
}"#
    );
    assert_eq!(
        server.requests(),
        vec![CapturedRequest {
            method: "POST".to_string(),
            authorization: Some("Basic YWxpY2U6c2VjcmV0".to_string()),
            body: json!([
                {
                    "jsonrpc": "2.0",
                    "method": "getnetworkinfo",
                    "params": {},
                    "id": 0,
                },
                {
                    "jsonrpc": "2.0",
                    "method": "getblockchaininfo",
                    "params": {},
                    "id": 1,
                },
                {
                    "jsonrpc": "2.0",
                    "method": "getwalletinfo",
                    "params": {},
                    "id": 2,
                },
                {
                    "jsonrpc": "2.0",
                    "method": "getbalances",
                    "params": {},
                    "id": 3,
                }
            ]),
        }],
    );
}

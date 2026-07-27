use super::*;

pub(super) fn handle_rpc_connection(
    mut stream: TcpStream,
    requests: &Arc<Mutex<Vec<String>>>,
    behavior: FakeRpcBehavior,
) {
    stream.set_nonblocking(false).expect("blocking stream");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout");
    let request = read_http_request(&mut stream);
    let request_text = String::from_utf8_lossy(&request).into_owned();
    let request_methods = json_rpc_methods_from_request(&request);
    requests
        .lock()
        .expect("request log")
        .push(request_text.clone());
    if behavior == FakeRpcBehavior::Unauthorized {
        let response = "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nWWW-Authenticate: Basic\r\nConnection: close\r\n\r\n";
        write_http_response(&mut stream, response);
        return;
    }
    let result = if has_rpc_method(&request_methods, "openbitcoinsyncstatus") {
        json!({
            "metadata": fake_runtime_metadata(false)
        })
    } else if has_rpc_method(&request_methods, "openbitcoinsyncpause") {
        json!({
            "metadata": fake_runtime_metadata(true)
        })
    } else if has_rpc_method(&request_methods, "openbitcoinsyncresume") {
        json!({
            "metadata": fake_runtime_metadata(false)
        })
    } else if has_rpc_method(&request_methods, "getnetworkinfo") {
        json!({
            "version": 29300,
            "subversion": "/Satoshi:29.3.0/",
            "protocolversion": 70016,
            "localservices": "0000000000000409",
            "localrelay": true,
            "connections": 7,
            "connections_in": 2,
            "connections_out": 5,
            "relayfee": 1000,
            "incrementalfee": 1000,
            "warnings": []
        })
    } else if has_rpc_method(&request_methods, "getblockchaininfo") {
        json!({
            "chain": "regtest",
            "blocks": 144,
            "headers": 150,
            "bestblockhash": "00aabb",
            "verificationprogress": 0.96,
            "initialblockdownload": false,
            "warnings": []
        })
    } else if has_rpc_method(&request_methods, "getmempoolinfo") {
        json!({
            "size": 12,
            "bytes": 2048,
            "usage": 4096,
            "total_fee_sats": 320,
            "maxmempool": 300000000,
            "mempoolminfee": 1000,
            "minrelaytxfee": 1000,
            "incrementalrelayfee": 1000,
            "rollingmempoolfee": 0,
            "effectiveadmissionfee": 1000,
            "capacityenforcement": "accounted_memory",
            "loaded": true
        })
    } else if has_rpc_method(&request_methods, "buildandsigntransaction") {
        json!({
            "transaction_hex": "001122",
            "fee_sats": 220,
            "inputs": [{
                "txid_hex": "aa".repeat(32),
                "vout": 0,
                "descriptor_id": 1,
                "amount_sats": 75000
            }],
            "maybe_change_output_index": 1
        })
    } else if has_rpc_method(&request_methods, "sendtoaddress") {
        json!("bb".repeat(32))
    } else if has_rpc_method(&request_methods, "getwalletinfo") {
        json!({
            "network": "regtest",
            "descriptor_count": 2,
            "utxo_count": 1,
            "maybe_tip_height": 144,
            "walletname": "alpha",
            "freshness": "fresh",
            "scanning": false
        })
    } else {
        json!({
            "mine": {
                "trusted_sats": 50000,
                "untrusted_pending_sats": 0,
                "immature_sats": 0
            }
        })
    };
    let response_body = json!({
        "jsonrpc": "2.0",
        "result": result,
        "id": 1
    })
    .to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    write_http_response(&mut stream, &response);
}

pub(super) fn write_http_response(stream: &mut TcpStream, response: &str) {
    stream
        .write_all(response.as_bytes())
        .expect("write response");
    stream.flush().expect("flush response");
}

pub(super) fn json_rpc_methods_from_request(request: &[u8]) -> Vec<String> {
    let Some(header_end) = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
    else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_slice::<Value>(&request[header_end..]) else {
        return Vec::new();
    };

    match value {
        Value::Object(object) => object
            .get("method")
            .and_then(Value::as_str)
            .map(|method| vec![method.to_owned()])
            .unwrap_or_default(),
        Value::Array(items) => items
            .iter()
            .filter_map(|item| item.get("method").and_then(Value::as_str))
            .map(str::to_owned)
            .collect(),
        _ => Vec::new(),
    }
}

pub(super) fn has_rpc_method(methods: &[String], expected: &str) -> bool {
    methods.iter().any(|method| method == expected)
}

pub(super) fn fake_runtime_metadata(paused: bool) -> Value {
    json!({
        "node_version": "0.1.0",
        "storage_engine": "fjall",
        "last_clean_shutdown": false,
        "maybe_last_recovery_action": null,
        "maybe_sync_state": null,
        "sync_control": {
            "paused": paused
        }
    })
}

pub(super) fn read_http_request(stream: &mut TcpStream) -> Vec<u8> {
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(bytes_read) => buffer.extend_from_slice(&chunk[..bytes_read]),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                if http_request_complete(&buffer) || Instant::now() > deadline {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => panic!("read request: {error}"),
        }
        if http_request_complete(&buffer) {
            break;
        }
        if Instant::now() > deadline {
            break;
        }
    }
    buffer
}

pub(super) fn http_request_complete(buffer: &[u8]) -> bool {
    let Some(header_end) = buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
    else {
        return false;
    };

    let headers = &buffer[..header_end];
    let Some(content_length) = parse_content_length(headers) else {
        return buffer.len() >= header_end;
    };

    buffer.len() >= header_end + content_length
}

pub(super) fn parse_content_length(headers: &[u8]) -> Option<usize> {
    std::str::from_utf8(headers).ok().and_then(|text| {
        text.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                return value.trim().parse::<usize>().ok();
            }
            None
        })
    })
}

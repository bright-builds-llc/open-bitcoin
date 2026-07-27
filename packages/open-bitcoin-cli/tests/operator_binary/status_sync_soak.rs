use super::*;

#[test]
fn open_bitcoin_status_json_succeeds_for_stopped_node() {
    // Arrange
    let sandbox = TestSandbox::new("stopped-json");
    let data_dir = sandbox.child("open-data");
    let core_dir = sandbox.child(".bitcoin");
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::create_dir_all(&core_dir).expect("core datadir");
    fs::write(core_dir.join("bitcoin.conf"), "regtest=1\n").expect("core config");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "status",
            "--format",
            "json",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("status json");
    for field in [
        "node",
        "config",
        "service",
        "sync",
        "peers",
        "mempool",
        "wallet",
        "logs",
        "metrics",
        "recovery_evidence",
        "health_signals",
        "build",
    ] {
        assert!(decoded.get(field).is_some(), "missing {field}");
    }
    assert_eq!(decoded["node"]["state"], "stopped");
    assert_eq!(decoded["sync"]["network"]["state"], "unavailable");
    match decoded["recovery_evidence"]["state"]
        .as_str()
        .expect("recovery evidence state")
    {
        "available" => {
            assert!(decoded["recovery_evidence"]["value"]["category"].is_string());
            assert!(decoded["recovery_evidence"]["value"]["cause"].is_string());
            assert!(decoded["recovery_evidence"]["value"]["action_class"].is_string());
        }
        "unavailable" => {
            assert!(
                decoded["recovery_evidence"]["value"]["reason"]
                    .as_str()
                    .expect("recovery evidence reason")
                    .contains("recovery evidence unavailable")
            );
        }
        state => panic!("unexpected recovery evidence state: {state}"),
    }
    let rendered = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(rendered.contains(core_dir.join("bitcoin.conf").to_str().expect("core path")));
    assert!(rendered.contains("uncertain"));
}

#[test]
fn open_bitcoin_status_json_uses_fake_running_rpc() {
    // Arrange
    let sandbox = TestSandbox::new("running-json");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start();
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "rpcconnect=127.0.0.1\nrpcport={}\nrpcuser=alice\nrpcpassword=secret\n",
            server.address.port()
        ),
    )
    .expect("bitcoin.conf");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "status",
            "--format",
            "json",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("status json");
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["sync"]["network"]["value"], "regtest");
    assert_eq!(decoded["sync"]["chain_tip"]["value"]["height"], 144);
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["outbound"], 5);
    assert_eq!(decoded["mempool"]["transactions"]["value"], 12);
    assert_eq!(decoded["wallet"]["trusted_balance_sats"]["value"], 50_000);
}

#[test]
fn open_bitcoin_sync_pause_and_resume_update_durable_control_state() {
    // Arrange
    let sandbox = TestSandbox::new("sync-control");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let store = FjallNodeStore::open(&data_dir).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save runtime metadata");
    drop(store);

    // Act
    let pause_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "pause",
        ],
    );
    let paused_status = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "sync",
            "status",
        ],
    );
    let resume_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "resume",
        ],
    );
    let resumed_status = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "sync",
            "status",
        ],
    );

    // Assert
    assert_success(&pause_output);
    assert_success(&resume_output);
    let paused: Value = serde_json::from_slice(&paused_status.stdout).expect("paused status");
    let resumed: Value = serde_json::from_slice(&resumed_status.stdout).expect("resumed status");
    assert_eq!(paused["sync_control"]["paused"], json!(true));
    assert_eq!(resumed["sync_control"]["paused"], json!(false));
}

#[test]
fn sync_control_uses_live_rpc_when_datadir_store_is_locked() {
    // Arrange
    let sandbox = TestSandbox::new("sync-control-live-rpc");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start();
    write_rpc_conf(&data_dir, server.address.port());
    let store = FjallNodeStore::open(&data_dir).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save runtime metadata");
    let _store_guard = store;

    // Act
    let status_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "sync",
            "status",
        ],
    );
    let pause_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "pause",
        ],
    );
    let resume_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "resume",
        ],
    );

    // Assert
    assert_success(&status_output);
    assert_success(&pause_output);
    assert_success(&resume_output);
    let status: Value = serde_json::from_slice(&status_output.stdout).expect("status json");
    assert_eq!(status["sync_control"]["paused"], json!(false));
    assert_no_store_lock_error(&status_output);
    assert_no_store_lock_error(&pause_output);
    assert_no_store_lock_error(&resume_output);
    let requests = server.requests().join("\n");
    assert!(requests.contains("openbitcoinsyncstatus"));
    assert!(requests.contains("openbitcoinsyncpause"));
    assert!(requests.contains("openbitcoinsyncresume"));
}

#[test]
fn sync_control_auth_failure_does_not_fallback_to_store() {
    // Arrange
    let sandbox = TestSandbox::new("sync-control-auth");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start_unauthorized();
    write_rpc_conf(&data_dir, server.address.port());
    let store = FjallNodeStore::open(&data_dir).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save runtime metadata");
    let _store_guard = store;

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "pause",
        ],
    );

    // Assert
    assert_failure(&output);
    assert_no_store_lock_error(&output);
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("RPC authentication failed for operator sync command"));
    let requests = server.requests().join("\n");
    assert!(requests.contains("openbitcoinsyncpause"));
}

#[test]
fn open_bitcoin_soak_start_writes_durable_ledger_and_reports() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-start");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);

    // Act
    let output = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("soak start json");
    let run_dir = data_dir.join("soak/runs/soak-1700000000-0001");
    let index_path = data_dir.join("soak/run-index.json");
    let events_path = run_dir.join("events.jsonl");
    let report_json_path = run_dir.join("report.json");
    let report_markdown_path = run_dir.join("report.md");
    assert_eq!(decoded["run_id"], json!("soak-1700000000-0001"));
    assert_eq!(
        decoded["ledger_path"],
        json!(events_path.display().to_string())
    );
    assert_eq!(
        decoded["json_report_path"],
        json!(report_json_path.display().to_string())
    );
    assert_eq!(
        decoded["markdown_report_path"],
        json!(report_markdown_path.display().to_string())
    );
    assert!(index_path.exists());
    assert!(events_path.exists());
    assert!(report_json_path.exists());
    assert!(report_markdown_path.exists());
    let index: Value =
        serde_json::from_str(&fs::read_to_string(index_path).expect("run index json"))
            .expect("run index");
    assert_eq!(index["runs"][0]["run_id"], json!("soak-1700000000-0001"));
    let events = read_jsonl_values(&events_path);
    assert_eq!(events[0]["event"]["kind"], json!("started"));
    assert!(
        events
            .iter()
            .any(|event| event["event"]["kind"] == json!("checkpoint"))
    );
    let report: Value =
        serde_json::from_str(&fs::read_to_string(report_json_path).expect("report json"))
            .expect("report");
    assert_eq!(report["is_projection"], json!(true));
    assert_eq!(report["run_id"], json!("soak-1700000000-0001"));
}

#[test]
fn open_bitcoin_soak_stop_rejects_terminal_verdict() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-stop");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));
    assert_success(&start);

    // Act
    let stop = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--network",
            "regtest",
            "--format",
            "json",
            "soak",
            "stop",
            "--run-id",
            "soak-1700000000-0001",
            "--reason",
            "operator-stop",
        ],
    );

    // Assert
    assert_failure(&stop);
    let stderr = String::from_utf8(stop.stderr).expect("stderr utf8");
    assert!(stderr.contains("already has a terminal verdict"));
    let run_dir = data_dir.join("soak/runs/soak-1700000000-0001");
    let events = read_jsonl_values(&run_dir.join("events.jsonl"));
    assert!(!events.iter().any(|event| {
        event["event"]["kind"] == json!("verdict")
            && event["event"]["outcome"] == json!("operator_stop")
    }));
    let report: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("report.json")).expect("report"))
            .expect("report json");
    assert_eq!(report["verdict"]["outcome"], json!("clean_completion"));
}

#[test]
fn open_bitcoin_soak_report_is_projection_without_ledger_append() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-report");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));
    assert_success(&start);
    let events_path = data_dir.join("soak/runs/soak-1700000000-0001/events.jsonl");
    let before_count = read_jsonl_values(&events_path).len();

    // Act
    let report = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--network",
            "regtest",
            "--format",
            "json",
            "soak",
            "report",
            "--run-id",
            "soak-1700000000-0001",
        ],
    );
    let after_count = read_jsonl_values(&events_path).len();

    // Assert
    assert_success(&report);
    assert_eq!(before_count, after_count);
    let decoded: Value = serde_json::from_slice(&report.stdout).expect("soak report json");
    assert_eq!(decoded["run_id"], json!("soak-1700000000-0001"));
    assert_eq!(
        decoded["ledger_path"],
        json!(events_path.display().to_string())
    );
}

#[test]
fn open_bitcoin_soak_resume_refuses_clean_completion() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-clean-resume");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(
        &sandbox,
        soak_target_height_start_args(&data_dir, "soak-1700000000-0001"),
    );
    assert_success(&start);
    let report: Value = serde_json::from_str(
        &fs::read_to_string(data_dir.join("soak/runs/soak-1700000000-0001/report.json"))
            .expect("report"),
    )
    .expect("report json");
    assert_eq!(report["verdict"]["outcome"], json!("clean_completion"));

    // Act
    let resume = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--network",
            "regtest",
            "--format",
            "json",
            "soak",
            "resume",
            "--run-id",
            "soak-1700000000-0001",
            "--checkpoint-interval-seconds",
            "15",
        ],
    );

    // Assert
    assert_failure(&resume);
    let stderr = String::from_utf8(resume.stderr).expect("stderr utf8");
    assert!(stderr.contains("clean_completion"));
}

#[test]
fn http_request_complete_waits_for_lowercase_content_length_body() {
    // Arrange
    let headers = b"POST / HTTP/1.1\r\ncontent-length: 5\r\n\r\n";
    let incomplete_request = [headers.as_slice(), b"abc"].concat();
    let complete_request = [headers.as_slice(), b"abcde"].concat();

    // Act
    let incomplete_is_complete = http_request_complete(&incomplete_request);
    let complete_is_complete = http_request_complete(&complete_request);

    // Assert
    assert!(!incomplete_is_complete);
    assert!(complete_is_complete);
}

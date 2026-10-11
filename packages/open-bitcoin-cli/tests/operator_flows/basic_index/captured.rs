// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Private producer snapshots replayed at the authenticated client response seam.
//! These tests compose fresh-checked producer evidence with actual client binaries;
//! they do not claim private faults were injected through a live daemon socket.

use super::*;

const CASES: [(&str, &str); 9] = [
    ("owner-integrity", "integrity_or_invalid_facts"),
    ("owner-invalidated", "invalidated"),
    ("owner-resource", "resource_bound"),
    ("persistence-after-commit", "backend"),
    ("persistence-before-records", "backend"),
    ("read-backend", "backend"),
    ("read-integrity", "integrity_or_invalid_facts"),
    ("required-body", "required_body"),
    ("required-undo", "required_undo"),
];

fn committed_captures() -> Value {
    let captures: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../open-bitcoin-node/testdata/basic_operator_captures.json"
    )))
    .expect("committed private producer captures");
    assert_eq!(
        captures
            .as_object()
            .expect("catalog")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        CASES.map(|(name, _)| name)
    );
    for (name, failure) in CASES {
        let value = &captures[name];
        assert_eq!(
            value["configured_indexing"]["value"], true,
            "producer setup {name}"
        );
        assert_eq!(
            value["failure"]["value"], failure,
            "independent producer category {name}"
        );
        assert_ne!(
            value["operational_state"]["value"], "current",
            "failed producer {name}"
        );
        assert!(value.to_string().len() <= 4096);
        let typed: open_bitcoin_node::status::BasicIndexOperatorStatus =
            serde_json::from_value(value.clone()).expect("typed safe BASIC");
        assert_eq!(
            serde_json::to_value(typed).expect("safe round trip"),
            *value
        );
    }
    captures
}

fn captured_response_server(
    runtime: &DurableSyncRuntime,
    maybe_basic: Option<Value>,
) -> (RpcTestServer, Arc<AtomicU64>, Arc<Value>) {
    let mut context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &RuntimeConfig::default(),
        runtime.network_handle(),
        Some(runtime.store().clone()),
    )
    .expect("actual durable context for unrelated groups");
    let call = normalize_method_call(
        "openbitcoinnetworkstatus",
        RequestParameters::from_json(json!([])),
    )
    .expect("real method");
    let mut response = dispatch(&mut context, call).expect("actual response envelope");
    response
        .as_object_mut()
        .expect("response")
        .remove("basic_index");
    if let Some(basic) = maybe_basic {
        response["basic_index"] = basic;
    }
    let captured = Arc::new(response);
    let replay = Arc::clone(&captured);
    let calls = Arc::new(AtomicU64::new(0));
    let counter = Arc::clone(&calls);
    let server = RpcTestServer::start_with_response_transform(context, move |method, result| {
        if method == "openbitcoinnetworkstatus" {
            counter.fetch_add(1, Ordering::Relaxed);
            return (*replay).clone();
        }
        result
    });
    (server, calls, captured)
}

#[test]
fn phase161_operator_consumers_committed_genuine_failures_replay_all_exports() {
    // Arrange: committed captures have separate mandatory fresh producer tests.
    let sandbox = TestSandbox::new("phase161-captured-failures");
    let (runtime, _) = durable_basic_runtime(&sandbox.home.join("store"), 2);
    let captures = committed_captures();
    // Act / Assert: actual authenticated RPC client and all five operator collections.
    for (name, _) in CASES {
        let expected = captures[name].clone();
        let (server, calls, captured) = captured_response_server(&runtime, Some(expected.clone()));
        let original = (*captured).clone();
        let rpc = assert_success_json(&run_cli_with_rpc(
            &server,
            &sandbox,
            &["openbitcoinnetworkstatus".to_string()],
        ));
        assert_eq!(rpc["basic_index"], expected);
        assert_exports(&sandbox, &server, name, &expected);
        assert_eq!(
            calls.load(Ordering::Relaxed),
            6,
            "one capture per actual client command"
        );
        assert_eq!(*captured, original, "export cannot change captured input");
    }
}

#[test]
fn phase161_operator_consumers_legacy_omission_stays_unavailable() {
    // Arrange: omit only BASIC from an otherwise genuine captured response.
    let sandbox = TestSandbox::new("phase161-legacy");
    let (runtime, _) = durable_basic_runtime(&sandbox.home.join("store"), 2);
    let (server, calls, captured) = captured_response_server(&runtime, None);
    let expected =
        serde_json::to_value(open_bitcoin_node::status::BasicIndexOperatorStatus::default())
            .expect("explicit unreported contract");
    // Act
    let status = assert_exports(&sandbox, &server, "legacy", &expected);
    // Assert
    assert_eq!(status["basic_index"], expected);
    assert!(captured.get("basic_index").is_none());
    assert_eq!(calls.load(Ordering::Relaxed), 5);
    for value in expected.as_object().expect("unreported BASIC").values() {
        assert_eq!(value["state"], "unavailable");
        assert_eq!(value["value"]["reason"], "BASIC evidence not reported");
    }
}

#[test]
fn phase161_operator_consumers_hostile_captured_reasons_are_private() {
    // Arrange: preserve every available safe fact from a real discovered read fault.
    let sandbox = TestSandbox::new("phase161-hostile");
    let (runtime, _) = durable_basic_runtime(&sandbox.home.join("store"), 2);
    let expected_original = committed_captures()["read-integrity"].clone();
    let mut poisoned = expected_original.clone();
    let mut expected = expected_original;
    let marker = "/private/phase161-caller-secret wallet=seed peer=192.0.2.1\n\u{1b}[31m";
    for (name, value) in poisoned.as_object_mut().expect("BASIC object") {
        if value["state"] == "unavailable" {
            value["value"]["reason"] = json!(marker);
            expected[name]["value"]["reason"] = json!("BASIC evidence not reported");
        }
    }
    let (server, calls, captured) = captured_response_server(&runtime, Some(poisoned));
    let original = (*captured).clone();
    // Act
    assert_exports(&sandbox, &server, "hostile", &expected);
    // Assert
    assert_eq!(
        *captured, original,
        "client/export does not mutate raw captured input"
    );
    assert_eq!(calls.load(Ordering::Relaxed), 5);
    for name in ["support-evidence.json", "support-evidence.md"] {
        let text = fs::read_to_string(sandbox.home.join("hostile/bundle").join(name))
            .expect("real export");
        assert!(!text.contains("phase161-caller-secret"));
        assert!(!text.contains("192.0.2.1"));
        assert!(!text.contains('\u{1b}'));
    }
}

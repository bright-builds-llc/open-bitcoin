export type Phase135FixtureFiles = Map<string, string>;
export type Phase135Mutator = (files: Phase135FixtureFiles) => void;
export type Phase135Mutation = readonly [
  name: string,
  expected: string,
  mutate: Phase135Mutator,
  exact?: boolean,
];

type PersistedInputMutationPaths = {
  snapshot: string;
  codec: string;
  codecDecode: string;
  codecKeyPreflight: string;
  codecTransactionDecode: string;
  store: string;
  dispatcher: string;
  topology: string;
  staging: string;
  startup: string;
};

type PersistedInputMutationDiagnostics = {
  bounds: string;
  topology: string;
  staging: string;
};

type ReplaceMutation = (
  relativePath: string,
  search: string,
  replacement: string,
) => Phase135Mutator;

export function persistedInputMutations(
  files: PersistedInputMutationPaths,
  diagnostics: PersistedInputMutationDiagnostics,
  replace: ReplaceMutation,
): readonly Phase135Mutation[] {
  return [
    [
      "encoded byte bound removed",
      diagnostics.bounds,
      replace(
        files.codec,
        "if bytes.len() > limits.max_encoded_bytes {",
        "if false {",
      ),
      true,
    ],
    [
      "raw object key bound is disabled",
      diagnostics.bounds,
      replace(
        files.codecKeyPreflight,
        "const MAX_MEMPOOL_FIELD_TOKEN_BYTES: usize = 64;",
        "const MAX_MEMPOOL_FIELD_TOKEN_BYTES: usize = usize::MAX;",
      ),
      true,
    ],
    [
      "raw object key preflight call removed",
      diagnostics.bounds,
      replace(files.codecDecode, "    validate_raw_object_keys(bytes)?;\n", ""),
      true,
    ],
    [
      "raw object key preflight moved after deserializer construction",
      diagnostics.bounds,
      replace(
        files.codecDecode,
        "    validate_raw_object_keys(bytes)?;\n    let mut deserializer = serde_json::Deserializer::from_slice(bytes);",
        "    let mut deserializer = serde_json::Deserializer::from_slice(bytes);\n    validate_raw_object_keys(bytes)?;",
      ),
      true,
    ],
    [
      "raw object key preflight hidden in a closure",
      diagnostics.bounds,
      replace(
        files.codecDecode,
        "    validate_raw_object_keys(bytes)?;\n",
        "    let decoy = || { validate_raw_object_keys(bytes) };\n",
      ),
      true,
    ],
    [
      "raw object key preflight hidden in a local function",
      diagnostics.bounds,
      replace(
        files.codecDecode,
        "    validate_raw_object_keys(bytes)?;\n",
        "    fn decoy(bytes: &[u8]) { let _ = validate_raw_object_keys(bytes); }\n",
      ),
      true,
    ],
    [
      "persisted store limits recouple to live policy",
      diagnostics.bounds,
      replace(
        files.store,
        "    pub fn for_persisted_input() -> Result<Self, SyncRecoveryCategory> {",
        "    pub fn for_persisted_input() -> Result<Self, SyncRecoveryCategory> {\n        let _max_live_entries = MempoolCapacityBounds::from_capacity(policy.mempool_capacity).max_live_entries();",
      ),
      true,
    ],
    [
      "persisted record cap returns to 50,000",
      diagnostics.bounds,
      replace(files.store, "            limits.max_records,", "            50_000,"),
      true,
    ],
    [
      "capture bypasses current live accounting bounds",
      diagnostics.bounds,
      replace(
        files.dispatcher,
        "MempoolCapacityBounds::from_capacity(mempool.config().mempool_capacity)",
        "MempoolCapacityBounds::from_capacity(MempoolCapacity::new(50_000))",
      ),
      true,
    ],
    [
      "topology vertices recouple to current policy",
      diagnostics.topology,
      replace(
        files.topology,
        "max_vertices: limits.max_records,",
        "max_vertices: MempoolCapacityBounds::from_capacity(policy.mempool_capacity).max_live_entries(),",
      ),
      true,
    ],
    [
      "topology edges recouple to current policy",
      diagnostics.topology,
      replace(
        files.topology,
        "max_edges: limits.max_input_edges,",
        "max_edges: MempoolCapacityBounds::from_capacity(policy.mempool_capacity).max_live_input_edges(),",
      ),
      true,
    ],
    [
      "aggregate topology cap returns to 1,600,000",
      diagnostics.topology,
      replace(
        files.topology,
        "max_edges: limits.max_input_edges,",
        "max_edges: 1_600_000,",
      ),
      true,
    ],
    [
      "aggregate edge ceiling is no longer derived by checked division",
      diagnostics.bounds,
      replace(
        files.codec,
        "MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES.checked_div(minimum_input_bytes)?",
        "MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES",
      ),
      true,
    ],
    [
      "per-record edge ceiling is no longer derived by checked division",
      diagnostics.bounds,
      replace(
        files.codec,
        "MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES.checked_div(minimum_input_bytes)?",
        "MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES",
      ),
      true,
    ],
    [
      "streaming aggregate transaction-byte counter is bypassed",
      diagnostics.bounds,
      replace(
        files.codecTransactionDecode,
        ".checked_add(bytes)",
        ".checked_add(0)",
      ),
      true,
    ],
    [
      "streaming per-record transaction-byte counter is bypassed",
      diagnostics.bounds,
      replace(
        files.codecTransactionDecode,
        "if bytes > self.max_transaction_bytes {",
        "if false {",
      ),
      true,
    ],
    [
      "bounded loader reads before checking stored size",
      diagnostics.bounds,
      replace(
        files.store,
        "let Some(encoded_size) = size()? else {",
        "let loaded_early = load()?;\n    let Some(encoded_size) = size()? else {",
      ),
      true,
    ],
    [
      "identity validation removed",
      diagnostics.bounds,
      replace(
        files.snapshot,
        "if txid != actual_txid || wtxid != actual_wtxid {",
        "if false {",
      ),
      true,
    ],
    [
      "checked aggregate topology accumulation removed",
      diagnostics.topology,
      replace(
        files.topology,
        "edge_count = limits.checked_add_edges(edge_count, parents.len())?;",
        "edge_count += parents.len();",
      ),
      true,
    ],
    [
      "staging mutates live state",
      diagnostics.staging,
      replace(
        files.staging,
        "let persisted_unbroadcast = snapshot.unbroadcast_members();",
        "let persisted_unbroadcast = snapshot.unbroadcast_members();\n    network.mempool_mut().clear();",
      ),
      true,
    ],
    [
      "unbroadcast intersection removed",
      diagnostics.staging,
      replace(files.staging, ".intersection(&final_members)", ".iter()"),
      true,
    ],
    [
      "rolling state is reused",
      diagnostics.staging,
      replace(
        files.staging,
        "let mut staged_mempool = Mempool::new(config);",
        "let mut staged_mempool = working;",
      ),
      true,
    ],
    [
      "final DroppedEvicted rewrite is bypassed",
      diagnostics.staging,
      replace(
        files.staging,
        "else if admitted.contains(&txid)",
        "else if false",
      ),
      true,
    ],
    [
      "RPC persisted-input constructor replaced with five usize::MAX arguments",
      diagnostics.bounds,
      replace(
        files.startup,
        "MempoolSnapshotDecodeLimits::for_persisted_input()",
        "MempoolSnapshotDecodeLimits::new(usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX)",
      ),
      true,
    ],
    [
      "topology vertex guard compiled out",
      diagnostics.topology,
      replace(
        files.topology,
        "if records.len() > limits.max_vertices",
        "if false",
      ),
      true,
    ],
    [
      "per-record edge validation bypassed",
      diagnostics.topology,
      replace(
        files.topology,
        "validate_parent_edges(record.record.transaction.inputs.len())",
        "validate_parent_edges(0)",
      ),
      true,
    ],
    [
      "raw-key preflight disabled by cfg(any())",
      diagnostics.bounds,
      replace(
        files.codecDecode,
        "    validate_raw_object_keys(bytes)?;",
        "    #[cfg(any())]\n    validate_raw_object_keys(bytes)?;",
      ),
      true,
    ],
    [
      "encoder restores snapshot-derived encoded ceiling",
      diagnostics.bounds,
      replace(
        files.codec,
        "    let limits = assert_mempool_snapshot_representable(snapshot).map_err(snapshot_failure)?;\n    let dto = MempoolSnapshotV2Dto::try_from(snapshot)?;\n    let bytes = encode_versioned(StorageNamespace::Mempool, &dto)?;\n    if bytes.len() > limits.max_encoded_bytes {",
        "    let dto = MempoolSnapshotV2Dto::try_from(snapshot)?;\n    let bytes = encode_versioned(StorageNamespace::Mempool, &dto)?;\n    if bytes.len() > encoded_size_upper_bound(total_transaction_bytes, dto.records.len(), dto.unbroadcast_members.len()).unwrap_or(usize::MAX) {",
      ),
      true,
    ],
  ];
}

/** Source-only producer controls keep the final persisted snapshot behind quiescence. */
export function producerLifecycleMutations(daemon: string, diagnostic: string): readonly Phase135Mutation[] {
  const cases = [
    ["unearned genesis marker", "let canonical_genesis = authoritative_runtime.maybe_prebuilt_context.is_some();", "let canonical_genesis = true;"],
    ["default genesis marker", "let canonical_genesis = authoritative_runtime.maybe_prebuilt_context.is_some();", "let canonical_genesis = Default::default();"],
    ["inverted retry predicate", "(!canonical_genesis).then(|| start_initial_broadcast_retry_worker(handle))", "canonical_genesis.then(|| start_initial_broadcast_retry_worker(handle))"],
    ["false retry predicate", "(!canonical_genesis).then(|| start_initial_broadcast_retry_worker(handle))", "false.then(|| start_initial_broadcast_retry_worker(handle))"],
    ["ordinary checkpoint disabled", "start_mempool_checkpoint_worker(handle.clone(), store.clone())", "None"],
    ["ordinary coins flush disabled", "start_coins_flush_worker(handle.clone(), store)", "None"],
    ["ordinary retry disabled", "(!canonical_genesis).then(|| start_initial_broadcast_retry_worker(handle))", "None"],
    ["retry shutdown replaced by success", "|| maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)),", "|| maybe_retry_worker.map_or(Ok(()), |_worker| Ok(())),"],
    ["false conditional retry join", "|| maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)),", "|| if false { maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)) } else { Ok(()) },"],
    ["test-only retry declaration", "    let maybe_retry_worker =", "    #[cfg(test)]\n    let maybe_retry_worker ="],
    ["test-only shutdown closure", "    settle_daemon_shutdown(", "    #[cfg(test)]\n    settle_daemon_shutdown("],
    ["checkpoint before optional retry", "|| maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)),", "|| Ok(maybe_checkpoint_worker.map_or(Ok(()), |worker| worker.shutdown_settle())?),"],
  ] as const;
  const optional = cases.map(([name, search, replacement]): Phase135Mutation => [
    `optional producer ${name}`, diagnostic,
    files => files.set(daemon, uniqueLiteralReplacement(files.get(daemon) ?? "", search, replacement)),
    false,
  ]);
  const decoys = ["test", "dead", "comment", "literal"].map((mode): Phase135Mutation => [
    `optional join despite expected ${mode} decoy`, diagnostic, files => {
      const original = files.get(daemon) ?? "";
      const marker = "async fn serve_authoritative_runtime<";
      const start = original.indexOf(marker);
      const end = original.indexOf("\n#[derive(Debug, Clone, PartialEq, Eq)]\nstruct DaemonSyncPreflightError", start);
      if (start < 0 || end <= start) throw new Error("missing owning daemon function");
      const expected = original.slice(start, end);
      const changed = uniqueLiteralReplacement(original,
        "|| maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)),", "|| Ok(()),");
      const decoy = mode === "test" ? `#[cfg(test)]\n${expected}`
        : mode === "dead" ? `fn disconnected_owner() { ${expected} }`
        : mode === "comment" ? `/* ${expected} */` : `const DECOY: &str = r###"${expected}"###;`;
      files.set(daemon, changed.slice(0, start) + decoy + "\n" + changed.slice(start));
    }, false,
  ]);
  const declaredGenesis: Phase135Mutation = ["declared GEN cannot choose mandatory ordinary workers", diagnostic, files => {
    const changes = [
      ["let maybe_checkpoint_worker = if canonical_genesis {\n        None\n    } else {\n        start_mempool_checkpoint_worker(handle.clone(), store.clone())\n    };", "let maybe_checkpoint_worker = start_mempool_checkpoint_worker(handle.clone(), store.clone());"],
      ["let maybe_coins_flush_worker = if canonical_genesis {\n        None\n    } else {\n        start_coins_flush_worker(handle.clone(), store)\n    };", "let maybe_coins_flush_worker = start_coins_flush_worker(handle.clone(), store);"],
      ["let maybe_retry_worker =\n        (!canonical_genesis).then(|| start_initial_broadcast_retry_worker(handle));", "let retry_worker = start_initial_broadcast_retry_worker(handle);"],
      ["|| maybe_retry_worker.map_or(Ok(()), |worker| Ok(worker.shutdown()?)),", "|| Ok(retry_worker.shutdown()?),"],
    ] as const;
    let source = files.get(daemon) ?? "";
    for (const [search, replacement] of changes) source = uniqueLiteralReplacement(source, search, replacement);
    files.set(daemon, source);
  }, false];
  return [...optional, ...decoys, declaredGenesis];
}

function uniqueLiteralReplacement(source: string, search: string, replacement: string): string {
  if (source.split(search).length !== 2) throw new Error("ambiguous producer mutation anchor");
  const changed = source.replace(search, () => replacement);
  if (changed === source) throw new Error("producer mutation left source unchanged");
  return changed;
}

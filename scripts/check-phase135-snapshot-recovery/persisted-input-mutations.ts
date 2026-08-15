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

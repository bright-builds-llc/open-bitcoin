import { addFailure, body, directStatementIndex, hasAll } from "./source";

export type PersistedInputSources = {
  codec: string;
  codecDecode: string;
  capture: string;
  store: string;
  topology: string;
  staging: string;
};

type PersistedInputDiagnostics = {
  bounds: string;
  topology: string;
  staging: string;
};

export function checkPersistedInputContract(
  sources: PersistedInputSources,
  failures: string[],
  diagnostics: PersistedInputDiagnostics,
): void {
  checkPersistedDecodeBounds(sources, failures, diagnostics.bounds);
  checkPersistedTopologyBounds(sources, failures, diagnostics.topology);
  checkCurrentPolicyReplay(sources, failures, diagnostics.staging);
}

function checkPersistedDecodeBounds(
  sources: PersistedInputSources,
  failures: string[],
  diagnostic: string,
): void {
  const persistedLimits = body(
    sources.codec,
    "pub(crate) fn persisted_mempool_input_limits()",
  );
  const encodedUpperBound = body(
    sources.codec,
    "pub(crate) fn encoded_size_upper_bound(",
  );
  const decode = body(
    sources.codec,
    "pub fn decode_mempool_snapshot_with_limits(",
  );
  const storeLimits = body(
    sources.store,
    "pub fn for_persisted_input()",
  );
  const boundedLoad = body(
    sources.store,
    "fn load_mempool_snapshot_with<Size, Load, Bytes>(",
  );
  const globallyCoexistingDimensions =
    "MempoolSnapshotDecodeLimits::new(268_435_456, 220_096, 5_000, 4_194_304, 67_108_864)";
  const boundedDecode = body(
    sources.codecDecode,
    "pub(super) fn decode_bounded_versioned(",
  );
  const preflightIndex = directStatementIndex(
    boundedDecode,
    "validate_raw_object_keys(bytes)?;",
  );
  const serdeIndex = directStatementIndex(
    boundedDecode,
    "let mut deserializer = serde_json::Deserializer::from_slice(bytes);",
  );
  const forbidden = /from_policy|PolicyConfig|MempoolCapacityBounds|max_live_entries|50_000|1_600_000|usize::MAX|saturating_(?:add|mul)|transaction\.len\(\).*max_(?:records|vertices|edges)/s;

  addFailure(
    failures,
    !hasAll(sources.codec, [
      "const MAX_MEMPOOL_SNAPSHOT_ENCODED_BYTES: usize = 256 * 1024 * 1024;",
      "const MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES: usize = 4 * 1024 * 1024;",
      "const MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES: usize = 64 * 1024 * 1024;",
      "const ENCODED_ENVELOPE_OVERHEAD_BYTES: usize = 1_048_576;",
      "const ENCODED_RECORD_OVERHEAD_BYTES: usize = 512;",
      "const ENCODED_UNBROADCAST_MEMBER_OVERHEAD_BYTES: usize = 4_096;",
      "const HEX_CHARS_PER_TRANSACTION_BYTE: usize = 2;",
    ]) ||
      !hasAll(persistedLimits, [
        "max_unbroadcast_members = MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS",
        "MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES.checked_mul(HEX_CHARS_PER_TRANSACTION_BYTE)?",
        "max_unbroadcast_members.checked_mul(ENCODED_UNBROADCAST_MEMBER_OVERHEAD_BYTES)?",
        ".checked_sub(encoded_transaction_bytes)?",
        ".checked_sub(encoded_unbroadcast_bytes)?",
        ".checked_sub(ENCODED_ENVELOPE_OVERHEAD_BYTES)?",
        "encoded_record_budget.checked_div(ENCODED_RECORD_OVERHEAD_BYTES)?",
        "OutPoint::SERIALIZED_LEN",
        ".checked_add(1)?",
        ".checked_add(size_of::<u32>())?",
        "MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES.checked_div(minimum_input_bytes)?",
        "MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES.checked_div(minimum_input_bytes)?",
        "encoded_size_upper_bound(",
        ") != Some(limits.max_encoded_bytes)",
      ]) ||
      !hasAll(encodedUpperBound, [
        ".checked_mul(HEX_CHARS_PER_TRANSACTION_BYTE)?",
        "max_records.checked_mul(ENCODED_RECORD_OVERHEAD_BYTES)?",
        "max_unbroadcast_members.checked_mul(ENCODED_UNBROADCAST_MEMBER_OVERHEAD_BYTES)?",
        ".checked_add(ENCODED_ENVELOPE_OVERHEAD_BYTES)",
      ]) ||
      !decode.includes("if bytes.len() > limits.max_encoded_bytes {") ||
      !hasAll(sources.codecDecode, [
        "limits.max_records",
        "limits.max_unbroadcast_members",
        "limits.max_transaction_bytes",
        "limits.max_total_transaction_bytes",
        "total_transaction_bytes: &mut total_transaction_bytes",
        "self.total_transaction_bytes",
        "if bytes > self.max_transaction_bytes {",
        "checked_add(bytes)",
        "next_total > self.max_total_transaction_bytes",
        "*self.total_transaction_bytes = next_total;",
        "const MAX_MEMPOOL_FIELD_TOKEN_BYTES: usize = 64;",
        "validate_raw_object_keys",
        "reject_extra_element",
      ]) ||
      preflightIndex < 0 ||
      serdeIndex < 0 ||
      preflightIndex >= serdeIndex ||
      !hasAll(storeLimits, [
        "snapshot_codec::persisted_mempool_input_limits()",
        "limits.max_encoded_bytes",
        "limits.max_records",
        "limits.max_unbroadcast_members",
        "limits.max_transaction_bytes",
        "limits.max_total_transaction_bytes",
      ]) ||
      !sources.store.includes(globallyCoexistingDimensions) ||
      !hasAll(boundedLoad, [
        "let Some(encoded_size) = size()?",
        "encoded_size > limits.max_encoded_bytes",
        "load()?",
      ]) ||
      boundedLoad.indexOf("encoded_size > limits.max_encoded_bytes") >
        boundedLoad.indexOf("load()?") ||
      !hasAll(sources.capture, [
        "MempoolCapacityBounds::from_capacity(mempool.config().mempool_capacity)",
        ".max_live_entries()",
        "if mempool.entries().len() > max_records {",
        "MempoolSnapshotError::ResourceBoundExceeded",
      ]) ||
      forbidden.test(
        [persistedLimits, encodedUpperBound, storeLimits, boundedLoad].join("\n"),
      ),
    diagnostic,
  );
}

function checkPersistedTopologyBounds(
  sources: PersistedInputSources,
  failures: string[],
  diagnostic: string,
): void {
  const topologyLimits = body(
    sources.topology,
    "pub(crate) fn for_persisted_input()",
  );
  const topologyPrepare = body(
    sources.topology,
    "pub(crate) fn prepare_recovery_topology(",
  );
  const forbidden = /from_policy|PolicyConfig|MempoolCapacityBounds|50_000|1_600_000|usize::MAX|saturating_(?:add|mul)/;

  addFailure(
    failures,
    !hasAll(topologyLimits, [
      "persisted_mempool_input_limits()",
      "max_vertices: limits.max_records",
      "max_edges: limits.max_input_edges",
      "max_parent_edges_per_record: limits.max_input_edges_per_record",
    ]) ||
      !hasAll(sources.topology, [
        "let next = edge_count",
        ".checked_add(parent_edges)",
        "if next > self.max_edges {",
        "if parent_edges > self.max_parent_edges_per_record {",
        "limits.checked_add_edges(edge_count, parents.len())?",
        "limits.checked_add_edges(0, 1_636_801)",
        "limits.checked_add_edges(0, 1_636_802)",
        "limits.validate_parent_edges(102_300)",
        "limits.validate_parent_edges(102_301)",
      ]) ||
      topologyPrepare.length === 0 ||
      forbidden.test(topologyLimits),
    diagnostic,
  );
}

function checkCurrentPolicyReplay(
  sources: PersistedInputSources,
  failures: string[],
  diagnostic: string,
): void {
  const stage = body(
    sources.staging,
    "pub(super) fn prepare_mempool_recovery(",
  );
  const topologyIndex = stage.indexOf(
    "RecoveryTopologyLimits::for_persisted_input()",
  );
  const currentPolicyReplayIndex = stage.indexOf(
    "let mut working = Mempool::new(config.clone());",
  );
  const finalMembershipIndex = stage.indexOf("let final_members = working");
  const finalRebuildIndex = stage.indexOf("let mut staged_mempool = Mempool::new(config);");

  addFailure(
    failures,
    stage.length === 0 ||
      topologyIndex < 0 ||
      currentPolicyReplayIndex <= topologyIndex ||
      finalMembershipIndex <= currentPolicyReplayIndex ||
      finalRebuildIndex <= finalMembershipIndex ||
      !hasAll(stage, [
        "prepare_recovery_topology(&snapshot.records, topology_limits)",
        ".intersection(&final_members)",
        "else if final_txids.contains(&txid)",
        "MempoolRecoveryStatus::Recovered",
        "else if admitted.contains(&txid)",
        "MempoolRecoveryStatus::DroppedEvicted",
      ]) ||
      /from_policy|50_000|1_600_000|usize::MAX/.test(stage) ||
      /mempool_mut\s*\(|apply_lifecycle_command|install_prepared_recovery/.test(
        stage,
      ),
    diagnostic,
  );
}

import path from "node:path";
import { parsePeerAddress } from "./options";
import { duplicateConnectVerdict, heightDelta, lastSnapshot, peerOutcomeSummary, restartProgressDelta, restartProgressSummary, restartStatus } from "./session";
import type { CommandSpec, EndpointOutcome, EndpointOutcomeSource, EndpointOutcomeState, FinalStatusSummary, FirstBlockProgressEvidence, FirstBlockProgressKind, FirstHeaderProgressEvidence, NoProgressCause, Options, RecoveryDiagnosis, RecoveryDiagnosisCategory, RestartResumeEvidence, RestartStatus, RuntimePeerTelemetry, SmokeSessionResult, SyncStatusSnapshot } from "./types";

export function firstHeaderProgressEvidence(
  maybeSnapshots: { before: SyncStatusSnapshot; after: SyncStatusSnapshot } | null,
  maybeFinalStatus: FinalStatusSummary | null,
): FirstHeaderProgressEvidence | null {
  if (maybeSnapshots === null) return null;
  const maybePeer = maybeFinalStatus?.recentPeers.find((peer) => peer.headersReceived > 0) ?? null;
  return {
    after: maybeSnapshots.after,
    before: maybeSnapshots.before,
    headerDelta: heightDelta(maybeSnapshots.after.headerHeight, maybeSnapshots.before.headerHeight) ?? 0,
    maybeLastActivityUnixSeconds: maybePeer?.maybeLastActivityUnixSeconds ?? null,
    maybePeer: maybePeer?.peer ?? null,
    maybeResolvedEndpoint: maybePeer?.maybeResolvedEndpoint ?? null,
    maybeSource: maybePeer?.source ?? null,
    observedAtUnixSeconds: maybeSnapshots.after.capturedAtUnixSeconds,
  };
}

export function firstBlockProgressEvidence(
  maybeSnapshots: { before: SyncStatusSnapshot; after: SyncStatusSnapshot } | null,
  maybeFinalStatus: FinalStatusSummary | null,
  kind: FirstBlockProgressKind,
): FirstBlockProgressEvidence | null {
  if (maybeSnapshots === null) return null;
  const maybePeer = maybeFinalStatus?.recentPeers.find((peer) => peer.blocksReceived > 0) ?? null;
  return {
    after: maybeSnapshots.after,
    before: maybeSnapshots.before,
    blockHash: kind === "connected" ? maybeSnapshots.after.maybeConnectedBlockHash : maybeSnapshots.after.maybeDownloadedBlockHash,
    height: kind === "connected" ? maybeSnapshots.after.connectedBlockHeight : maybeSnapshots.after.downloadedBlockHeight,
    kind,
    maybeLastActivityUnixSeconds: maybePeer?.maybeLastActivityUnixSeconds ?? null,
    maybePeer: maybePeer?.peer ?? null,
    maybeResolvedEndpoint: maybePeer?.maybeResolvedEndpoint ?? null,
    maybeSource: maybePeer?.source ?? null,
    observedAtUnixSeconds: maybeSnapshots.after.capturedAtUnixSeconds,
  };
}

export function endpointOutcomesFromFinalStatus(
  finalStatus: FinalStatusSummary | null,
): EndpointOutcome[] {
  if (finalStatus === null) {
    return [];
  }
  return finalStatus.recentPeers.map((peer) => {
    const parsedPeer = parsePeerAddress(peer.peer);
    const maybeFailureCause = noProgressCauseFromPeer(peer);
    return {
      address: peer.peer,
      attemptedAtUnixSeconds: Math.floor(Date.now() / 1000),
      host: parsedPeer.host,
      maybeError: peer.maybeError,
      maybeFailureCause,
      maybeResolvedEndpoint: peer.maybeResolvedEndpoint,
      port: parsedPeer.port,
      source: endpointSourceFromPeerTelemetry(peer.source),
      stage: "runtime",
      state: endpointStateFromPeerTelemetry(peer),
    };
  });
}

function endpointSourceFromPeerTelemetry(source: string): EndpointOutcomeSource {
  if (source === "manual") {
    return "manual_peer";
  }
  if (source === "dns_seed") {
    return "dns_seed";
  }
  return "unknown";
}

function endpointStateFromPeerTelemetry(peer: RuntimePeerTelemetry): EndpointOutcomeState {
  if (peer.state === "connected" && peer.maybeCapabilities !== null) {
    return "handshook";
  }
  if (peer.state === "connected" || peer.state === "stalled") {
    return "connected";
  }
  if (peer.state === "failed") {
    return "failed";
  }
  return "skipped";
}

function noProgressCauseFromPeer(peer: RuntimePeerTelemetry): NoProgressCause | null {
  const reason = peer.maybeFailureReason;
  if (reason === "address_resolution") {
    return "dns_resolution_failure";
  }
  if (reason === "connect") {
    return "tcp_connection_failure";
  }
  if (reason === "block_notfound") {
    return "peer_notfound";
  }
  if (reason === "malformed_block") {
    return "malformed_block";
  }
  if (reason === "invalid_block") {
    return "invalid_block";
  }
  if (
    reason === "duplicate_block" ||
    reason === "disconnected_block" ||
    reason === "non_extending_block"
  ) {
    return "duplicate_or_disconnected_block";
  }
  if (reason === "resource_limit") {
    return "resource_limit";
  }
  if (reason === "invalid_data") {
    return "validation_failure";
  }
  if (reason === "invalid_magic" || reason === "network" || reason === "stall") {
    return "handshake_failure";
  }
  if (reason === "storage") {
    return "storage_failure";
  }
  const maybeError = peer.maybeError?.toLowerCase() ?? "";
  if (maybeError.includes("capabil") || maybeError.includes("service")) {
    return "unsupported_peer_capability";
  }
  return null;
}

export function classifyNoProgressCause(
  endpointOutcomes: EndpointOutcome[],
  maybeFinalStatus: FinalStatusSummary | null,
  maybeLastProbeError: string | null,
): NoProgressCause {
  const maybeStatusCause = noProgressCauseFromFinalStatus(maybeFinalStatus);
  if (maybeStatusCause !== null) {
    return maybeStatusCause;
  }

  const attemptedOutcomes = endpointOutcomes.filter((outcome) => outcome.state !== "skipped");
  const maybeRuntimeCause = attemptedOutcomes
    .filter((outcome) => outcome.stage === "runtime")
    .map((outcome) => outcome.maybeFailureCause)
    .find((cause): cause is NoProgressCause => cause !== null);
  if (maybeRuntimeCause !== undefined) {
    return maybeRuntimeCause;
  }

  const connected = attemptedOutcomes.some(
    (outcome) => outcome.state === "connected" || outcome.state === "handshook",
  );
  if (
    !connected &&
    attemptedOutcomes.some((outcome) => outcome.maybeFailureCause === "dns_resolution_failure")
  ) {
    return "dns_resolution_failure";
  }
  if (
    !connected &&
    attemptedOutcomes.some((outcome) => outcome.maybeFailureCause === "tcp_connection_failure")
  ) {
    return "tcp_connection_failure";
  }
  if (connected && maybeFinalStatus?.outboundPeers === 0) {
    return "handshake_failure";
  }
  const maybeEndpointCause = attemptedOutcomes
    .filter((outcome) => outcome.maybeFailureCause !== "dns_resolution_failure")
    .filter((outcome) => outcome.maybeFailureCause !== "tcp_connection_failure")
    .map((outcome) => outcome.maybeFailureCause)
    .find((cause): cause is NoProgressCause => cause !== null);
  if (maybeEndpointCause !== undefined) {
    return maybeEndpointCause;
  }

  if (maybeLastProbeError !== null) {
    const lowered = maybeLastProbeError.toLowerCase();
    if (lowered.includes("storage") || lowered.includes("fjall")) {
      return "storage_failure";
    }
    if (lowered.includes("invalid") || lowered.includes("validation")) {
      return "validation_failure";
    }
  }

  return "timeout";
}

export function noProgressCauseFromFinalStatus(
  maybeFinalStatus: FinalStatusSummary | null,
): NoProgressCause | null {
  const maybeLastError = maybeFinalStatus?.maybeLastError?.toLowerCase() ?? "";
  if (maybeLastError.includes("storage") || maybeLastError.includes("fjall")) {
    return "storage_failure";
  }
  if (maybeLastError.includes("resource")) {
    return "resource_limit";
  }
  if (maybeLastError.includes("malformed block")) {
    return "malformed_block";
  }
  if (maybeLastError.includes("invalid block") || maybeLastError.includes("bad block")) {
    return "invalid_block";
  }
  if (
    maybeLastError.includes("invalid") ||
    maybeLastError.includes("validation")
  ) {
    return "validation_failure";
  }
  return null;
}

export function nextActionForCause(cause: NoProgressCause | null): string {
  switch (cause) {
    case "awaiting_blocks":
      return "Keep the daemon running or retry with peers that can deliver and validate block bodies; Phase 57 passes only after connected block height increases.";
    case "dns_resolution_failure":
      return "Fix DNS resolution or retry with --manual-peer=HOST[:PORT] to bypass DNS seeds.";
    case "tcp_connection_failure":
      return "Fix outbound TCP access to port 8333, check firewall/VPN rules, or retry with a reachable --manual-peer.";
    case "handshake_failure":
      return "Inspect daemon stderr and peer endpoint outcomes; retry with a different manual peer if the endpoint accepts TCP but does not complete the Bitcoin handshake.";
    case "unsupported_peer_capability":
      return "Retry with a peer that advertises the required Bitcoin services for header/block sync.";
    case "validation_failure":
      return "Inspect the daemon last error and durable sync status before retrying; invalid peer data may require a different peer or a later validation fix.";
    case "storage_failure":
      return "Inspect the datadir storage error, free space, and recovery marker before retrying.";
    case "peer_notfound":
      return "Retry with a different peer or more peers; the selected peer reported the requested block as unavailable.";
    case "malformed_block":
      return "Inspect peer diagnostics and retry with a different peer; malformed block payloads are rejected and uncredited.";
    case "invalid_block":
      return "Inspect validation diagnostics and retry with another peer before trusting the block response.";
    case "duplicate_or_disconnected_block":
      return "Review peer outcomes for duplicate, disconnected, or non-extending block responses, then retry with peers advertising the best-chain data.";
    case "resource_limit":
      return "Raise the configured block in-flight or sync loop bounds for this explicit review run, or reduce competing load.";
    case "operator_cancellation":
      return "Review the partial report, then rerun the same command when ready.";
    case "timeout":
      return "Increase --timeout-seconds or use --manual-peer=HOST[:PORT] if endpoint outcomes show reachable peers but no progress yet.";
    case null:
      return "Review the generated report for status snapshots and daemon output.";
  }
}

function recoveryDiagnosis(
  endpointOutcomes: EndpointOutcome[],
  maybeFinalStatus: FinalStatusSummary | null,
  maybeLastProbeError: string | null,
  status: RestartStatus,
): RecoveryDiagnosis {
  const maybePeer = maybeFinalStatus?.recentPeers.find(
    (peer) => peer.maybeFailureReason !== null || peer.maybeError !== null,
  ) ?? null;
  const maybePeerFailureReason = maybePeer?.maybeFailureReason ?? null;
  const maybeLastError = maybeFinalStatus?.maybeLastError ?? maybeLastProbeError ?? maybePeer?.maybeError ?? null;
  const maybeNoProgressCause = status === "completed" ? null : classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError);
  const details = [
    maybeLastError,
    maybePeerFailureReason,
    maybePeer?.maybeError ?? null,
    maybeNoProgressCause,
  ]
    .filter((value): value is string => value !== null)
    .join(" ")
    .toLowerCase();

  if (
    details.includes("schema invalid") ||
    details.includes("invalid schema") ||
    details.includes("schema mismatch") ||
    details.includes("invalid schema version")
  ) {
    return recoveryDiagnosisResult(
      "incompatible_schema",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (
    details.includes("storage corruption") ||
    details.includes("corrupt namespace") ||
    details.includes("corruption in")
  ) {
    return recoveryDiagnosisResult(
      "store_corruption",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (containsLockSignal(details)) {
    return recoveryDiagnosisResult(
      "storage_lock_contention",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (
    maybeNoProgressCause === "storage_failure" ||
    details.includes("backend") ||
    details.includes("unavailable namespace") ||
    details.includes("storage failure") ||
    details.includes("interrupted write")
  ) {
    return recoveryDiagnosisResult(
      "storage_backend_failure",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (details.includes("resource") || maybeNoProgressCause === "resource_limit") {
    return recoveryDiagnosisResult(
      "resource_exhaustion",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (status === "cancelled" || maybeNoProgressCause === "operator_cancellation") {
    return recoveryDiagnosisResult(
      "operator_cancellation",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (
    maybeNoProgressCause === "malformed_block" ||
    maybeNoProgressCause === "invalid_block" ||
    maybeNoProgressCause === "duplicate_or_disconnected_block" ||
    maybeNoProgressCause === "validation_failure" ||
    details.includes("invalid data") ||
    details.includes("malformed block") ||
    details.includes("malformed_block") ||
    details.includes("invalid block") ||
    details.includes("invalid_block") ||
    details.includes("duplicate_block") ||
    details.includes("disconnected_block") ||
    details.includes("non_extending_block") ||
    details.includes("invalid_magic") ||
    details.includes("invalid magic")
  ) {
    return recoveryDiagnosisResult(
      "invalid_peer_data",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  if (
    maybeNoProgressCause === "handshake_failure" ||
    maybeNoProgressCause === "unsupported_peer_capability" ||
    details.includes("capabil") ||
    details.includes("service") ||
    details.includes("version")
  ) {
    return recoveryDiagnosisResult(
      "public_network_unreachable",
      maybeNoProgressCause,
      maybePeerFailureReason,
      maybeLastError,
    );
  }
  return recoveryDiagnosisResult(
    "public_network_unreachable",
    maybeNoProgressCause,
    maybePeerFailureReason,
    maybeLastError,
  );
}

function recoveryDiagnosisResult(
  category: RecoveryDiagnosisCategory,
  maybeNoProgressCause: NoProgressCause | null,
  maybePeerFailureReason: string | null,
  maybeLastError: string | null,
): RecoveryDiagnosis {
  const maybeStorageRecoveryAction = category === "incompatible_schema" ||
      category === "store_corruption" ||
      category === "storage_lock_contention" ||
      category === "storage_backend_failure"
    ? nextActionForCause("storage_failure")
    : null;
  return {
    category,
    maybeLastError,
    maybeNoProgressCause,
    maybePeerFailureReason,
    maybeStorageRecoveryAction,
  };
}

function containsLockSignal(details: string): boolean {
  return (
    containsAsciiWord(details, "lock") ||
    containsAsciiWord(details, "locked") ||
    details.includes("contention")
  );
}

function containsAsciiWord(haystack: string, needle: string): boolean {
  let start = haystack.indexOf(needle);
  while (start !== -1) {
    const end = start + needle.length;
    const beforeIsBoundary = start === 0 || isAsciiWordBoundary(haystack.charCodeAt(start - 1));
    const afterIsBoundary = end === haystack.length || isAsciiWordBoundary(haystack.charCodeAt(end));
    if (beforeIsBoundary && afterIsBoundary) {
      return true;
    }
    start = haystack.indexOf(needle, start + 1);
  }
  return false;
}

function isAsciiWordBoundary(charCode: number): boolean {
  return !(
    (charCode >= 48 && charCode <= 57) ||
    (charCode >= 65 && charCode <= 90) ||
    (charCode >= 97 && charCode <= 122)
  );
}

function datadirArg(commandSpec: CommandSpec): string | null {
  return commandSpec.args.find((arg) => arg.startsWith("-datadir=")) ?? null;
}

export function restartResumeEvidence(
  repoRootPath: string,
  options: Options,
  beforeSession: SmokeSessionResult,
  afterSession: SmokeSessionResult | null,
  endpointOutcomes: EndpointOutcome[],
  maybeFinalStatus: FinalStatusSummary | null,
  maybeLastProbeError: string | null,
): RestartResumeEvidence {
  const beforeRestart = lastSnapshot(beforeSession.snapshots);
  const afterRestart = afterSession?.snapshots[0] ?? null;
  const afterLatest = lastSnapshot(afterSession?.snapshots ?? []);
  const requestedBefore = datadirArg(beforeSession.daemonSpec);
  const requestedAfter = afterSession === null ? null : datadirArg(afterSession.daemonSpec);
  const requestedBeforePath = requestedBefore?.slice("-datadir=".length) ?? null;
  const requestedAfterPath = requestedAfter?.slice("-datadir=".length) ?? null;
  const status = restartStatus(
    beforeRestart,
    afterRestart,
    beforeSession.maybeCancellationSignal ?? afterSession?.maybeCancellationSignal ?? null,
  );
  return {
    afterRestart: restartProgressSummary(afterRestart),
    beforeRestart: restartProgressSummary(beforeRestart),
    duplicateConnectVerdict: duplicateConnectVerdict(beforeRestart, afterRestart),
    maybePostRestartProgressDelta: afterRestart === null || afterLatest === null ? null : restartProgressDelta(afterRestart, afterLatest),
    peerOutcomeSummary: peerOutcomeSummary(endpointOutcomes),
    recoveryDiagnosis: recoveryDiagnosis(
      endpointOutcomes,
      maybeFinalStatus,
      maybeLastProbeError,
      status,
    ),
    restartStatus: status,
    sameDatadir: {
      requestedPathMatched: requestedBefore !== null && requestedAfter !== null && requestedBefore === requestedAfter,
      resolvedPathMatched: requestedBeforePath !== null &&
        requestedAfterPath !== null &&
        path.resolve(repoRootPath, requestedBeforePath) ===
          path.resolve(repoRootPath, requestedAfterPath),
    },
  };
}

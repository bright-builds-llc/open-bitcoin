#!/usr/bin/env bun

import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { lookup } from "node:dns/promises";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createConnection } from "node:net";
import path from "node:path";

const BASELINE = "Bitcoin Knots 29.3.knots20260210";
const DEFAULT_OUTPUT_DIR = "packages/target/live-mainnet-smoke-reports";
const DEFAULT_TIMEOUT_SECONDS = 180;
const DEFAULT_POLL_SECONDS = 10;
const DEFAULT_MIN_FREE_GIB = 20;
const REPORT_STEM = "open-bitcoin-live-mainnet-smoke";
const GENERATED_CONFIG_FILE_NAME = "open-bitcoin-live-mainnet-smoke.jsonc";
const MIN_REASONABLE_UNIX_SECONDS = 1_704_067_200; // 2024-01-01T00:00:00Z
const MAX_TAIL_BYTES = 16 * 1024;
const DEFAULT_NETWORK_PREFLIGHT_TIMEOUT_MS = 1_500;
const DEFAULT_ENDPOINTS_PER_SOURCE = 1;
const DEFAULT_MAINNET_DNS_SEEDS = [
  "seed.bitcoin.sipa.be",
  "dnsseed.bluematt.me",
  "dnsseed.bitcoin.dashjr-list-of-p2p-nodes.us",
  "seed.bitcoinstats.com",
  "seed.bitcoin.jonasschnelli.ch",
];

type Options = {
  datadir: string;
  manualPeers: string[];
  maybeConfigPath: string | null;
  maybeGeneratedConfigPath: string | null;
  minFreeGib: number;
  outputDir: string;
  pollSeconds: number;
  restartAfterProgress: boolean;
  timeoutSeconds: number;
};

type PeerAddress = {
  address: string;
  host: string;
  port: number;
};

type PreflightCheck = {
  detail: string;
  name: string;
  ok: boolean;
};

type CommandSpec = {
  args: string[];
  command: string;
};

type SyncStatusSnapshot = {
  blockHeight: number | null;
  attemptCounters: AttemptCountersSummary | null;
  capturedAtUnixSeconds: number;
  configuredTargets: ConfiguredTargetsSummary | null;
  connectedBlockHeight: number | null;
  downloadedBlockHeight: number | null;
  headerHeight: number | null;
  lifecycle: string;
  maybeConnectedBlockHash: string | null;
  maybeDownloadedBlockHash: string | null;
  maybeAttemptCountersUnavailableReason: string | null;
  maybeConfiguredTargetsUnavailableReason: string | null;
  maybeLastSuccessfulProgressUnixSeconds: number | null;
  maybeLastError: string | null;
  maybeLastErrorUnavailableReason: string | null;
  maybeLatestStopReasonUnavailableReason: string | null;
  maybePeerCountsUnavailableReason: string | null;
  maybeProgressSignalUnavailableReason: string | null;
  maybeRecoveryActionUnavailableReason: string | null;
  maybeRecoveryCategoryUnavailableReason: string | null;
  maybeResourcePressureUnavailableReason: string | null;
  maybeSyncProgressUnavailableReason: string | null;
  outboundPeers: number | null;
  paused: boolean;
  phase: string;
  progressSignal: string | null;
  latestStopReason: StopReasonSummary | null;
  recoveryAction: string | null;
  recoveryCategory: RecoveryDiagnosisCategory | null;
  resourcePressure: ResourcePressureSummary | null;
  updatedAtUnixSeconds: number;
};

type FirstHeaderProgressEvidence = {
  after: SyncStatusSnapshot;
  before: SyncStatusSnapshot;
  headerDelta: number;
  maybeLastActivityUnixSeconds: number | null;
  maybePeer: string | null;
  maybeResolvedEndpoint: string | null;
  maybeSource: string | null;
  observedAtUnixSeconds: number;
};

type FirstBlockProgressKind = "downloaded" | "connected";

type FirstBlockProgressEvidence = {
  after: SyncStatusSnapshot;
  before: SyncStatusSnapshot;
  blockHash: string | null;
  height: number | null;
  kind: FirstBlockProgressKind;
  maybeLastActivityUnixSeconds: number | null;
  maybePeer: string | null;
  maybeResolvedEndpoint: string | null;
  maybeSource: string | null;
  observedAtUnixSeconds: number;
};

type EndpointOutcomeState = "resolved" | "connected" | "handshook" | "failed" | "skipped";
type EndpointOutcomeStage = "preflight" | "runtime";
type EndpointOutcomeSource = "manual_peer" | "dns_seed" | "configured_peer" | "unknown";
type NoProgressCause =
  | "awaiting_blocks"
  | "dns_resolution_failure"
  | "tcp_connection_failure"
  | "handshake_failure"
  | "unsupported_peer_capability"
  | "validation_failure"
  | "storage_failure"
  | "peer_notfound"
  | "malformed_block"
  | "invalid_block"
  | "duplicate_or_disconnected_block"
  | "resource_limit"
  | "timeout"
  | "operator_cancellation";

type EndpointOutcome = {
  address: string;
  attemptedAtUnixSeconds: number;
  host: string;
  maybeError: string | null;
  maybeFailureCause: NoProgressCause | null;
  maybeResolvedEndpoint: string | null;
  port: number;
  source: EndpointOutcomeSource;
  stage: EndpointOutcomeStage;
  state: EndpointOutcomeState;
};

type ReportStatus =
  | "passed"
  | "preflight_failed"
  | "runtime_failed"
  | "no_progress"
  | "cancelled";

type RestartStatus =
  | "not_requested"
  | "completed"
  | "blocked_before_restart"
  | "cancelled";

type DuplicateConnectVerdict =
  | "no_duplicate_connect_observed"
  | "duplicate_connect_suspected"
  | "unavailable";

type RestartProgressSummary = {
  attemptCounters: AttemptCountersSummary | null;
  configuredTargets: ConfiguredTargetsSummary | null;
  connectedBlockHeight: number | null;
  downloadedBlockHeight: number | null;
  headerHeight: number | null;
  lifecycle: string;
  latestStopReason: StopReasonSummary | null;
  maybeAttemptCountersUnavailableReason: string | null;
  maybeConnectedBlockHash: string | null;
  maybeConfiguredTargetsUnavailableReason: string | null;
  maybeDownloadedBlockHash: string | null;
  maybeLastError: string | null;
  maybeLastErrorUnavailableReason: string | null;
  maybeLastSuccessfulProgressUnixSeconds: number | null;
  maybeLatestStopReasonUnavailableReason: string | null;
  maybePeerCountsUnavailableReason: string | null;
  maybeProgressSignalUnavailableReason: string | null;
  maybeRecoveryActionUnavailableReason: string | null;
  maybeRecoveryCategoryUnavailableReason: string | null;
  maybeResourcePressureUnavailableReason: string | null;
  maybeSyncProgressUnavailableReason: string | null;
  phase: string;
  progressSignal: string | null;
  recoveryAction: string | null;
  recoveryCategory: RecoveryDiagnosisCategory | null;
  resourcePressure: ResourcePressureSummary | null;
};

type RestartProgressDelta = {
  connectedBlockDelta: number;
  downloadedBlockDelta: number;
  headerDelta: number;
};

type RestartPeerOutcomeSummary = {
  connected: number;
  failed: number;
  failureCauses: NoProgressCause[];
  handshook: number;
  skipped: number;
};

type RecoveryDiagnosisCategory =
  | "clean_shutdown"
  | "unclean_shutdown"
  | "incompatible_schema"
  | "public_network_unreachable"
  | "invalid_peer_data"
  | "store_corruption"
  | "storage_lock_contention"
  | "storage_backend_failure"
  | "resource_exhaustion"
  | "operator_cancellation";

type RecoveryDiagnosis = {
  category: RecoveryDiagnosisCategory;
  maybeLastError: string | null;
  maybeNoProgressCause: NoProgressCause | null;
  maybePeerFailureReason: string | null;
  maybeStorageRecoveryAction: string | null;
};

type RecoveryEvidenceStatusJson = {
  action_class?: string;
  category?: string;
  cause?: string;
  compatibility_action?: FieldAvailability<string>;
  evidence_basis?: unknown[];
  maybe_affected_namespace?: string | null;
  maybe_affected_path?: string | null;
  next_action?: string;
};

type RecoveryEvidenceSummary = {
  actionClass: string | null;
  affectedNamespace: string | null;
  affectedPath: string | null;
  category: string | null;
  cause: string | null;
  compatibilityAction: string | null;
  evidenceBasis: string[];
  maybeUnavailableReason: string | null;
  nextAction: string | null;
  source: string;
  state: string;
};

type ResourcePressureSummary = {
  blocksInFlight: number;
  maxHeaderRequestsInFlightPerPeer: number;
  maxHeadersPerMessage: number;
  maxBlocksInFlightPerPeer: number;
  maxBlocksInFlightTotal: number;
  maxMessagesPerPeer: number;
  maxSyncRounds: number;
  outboundPeers: number;
  targetOutboundPeers: number;
};

type ConfiguredTargetsSummary = {
  maybeTargetHeaderHeight: number | null;
  targetOutboundPeers: number;
};

type AttemptCountersSummary = {
  attemptedPeers: number;
  connectedPeers: number;
  failedPeers: number;
  maxSyncRounds: number;
};

type StopReasonSummary = {
  label: string;
  message: string;
};

type ObjectSummary = Record<string, string | number | boolean | null>;

type PeerContributionSummary = {
  attempted: number | null;
  connected: number | null;
  failed: number | null;
  outbound: number | null;
};

type RestartResumeEvidence = {
  afterRestart: RestartProgressSummary | null;
  beforeRestart: RestartProgressSummary | null;
  duplicateConnectVerdict: DuplicateConnectVerdict;
  maybePostRestartProgressDelta: RestartProgressDelta | null;
  peerOutcomeSummary: RestartPeerOutcomeSummary;
  recoveryDiagnosis: RecoveryDiagnosis;
  restartStatus: RestartStatus;
  sameDatadir: {
    requestedPathMatched: boolean;
    resolvedPathMatched: boolean;
  };
};

type FinalStatusSummary = {
  attemptCounters: AttemptCountersSummary | null;
  bestKnownTip: ObjectSummary | null;
  blockHeight: number | null;
  configuredTargets: ConfiguredTargetsSummary | null;
  connectedBlockHeight: number | null;
  downloadedBlockHeight: number | null;
  headerHeight: number | null;
  latestStopReason: StopReasonSummary | null;
  latestReorg: ObjectSummary | null;
  maybeAttemptCountersUnavailableReason: string | null;
  maybeBestKnownTipUnavailableReason: string | null;
  maybeConfiguredTargetsUnavailableReason: string | null;
  lifecycle: string;
  maybeConnectedBlockHash: string | null;
  maybeDownloadedBlockHash: string | null;
  maybeLastSuccessfulProgressUnixSeconds: number | null;
  maybeLastError: string | null;
  maybeLastErrorUnavailableReason: string | null;
  maybeLatestStopReasonUnavailableReason: string | null;
  maybeLatestReorgUnavailableReason: string | null;
  maybeNoProgressDiagnosisUnavailableReason: string | null;
  maybeNoProgressNextActionUnavailableReason: string | null;
  maybePeerCountsUnavailableReason: string | null;
  maybeProgressSignalUnavailableReason: string | null;
  maybeReconcileProgressUnavailableReason: string | null;
  maybeRecoveryActionUnavailableReason: string | null;
  maybeRecoveryCategoryUnavailableReason: string | null;
  maybeRecoveryEvidenceUnavailableReason: string | null;
  maybeResourcePressureUnavailableReason: string | null;
  maybeStayCurrentNextActionUnavailableReason: string | null;
  maybeStayCurrentUnavailableReason: string | null;
  maybeSyncProgressUnavailableReason: string | null;
  maybeValidatedActiveChainHeightUnavailableReason: string | null;
  maybeValidatedActiveChainHash: string | null;
  maybeValidatedActiveChainWork: string | null;
  headersReceived: number | null;
  blocksReceived: number | null;
  messagesProcessed: number | null;
  noProgressDiagnosis: string | null;
  noProgressNextAction: string | null;
  outboundPeers: number | null;
  peerContribution: PeerContributionSummary | null;
  phase: string;
  progressSignal: string | null;
  recentPeers: RuntimePeerTelemetry[];
  reconcileProgress: ObjectSummary | null;
  recoveryAction: string | null;
  recoveryActionClass: string | null;
  recoveryCategory: RecoveryDiagnosisCategory | null;
  recoveryCause: string | null;
  recoveryEvidence: RecoveryEvidenceSummary | null;
  recoveryNextAction: string | null;
  resourcePressure: ResourcePressureSummary | null;
  stayCurrent: string | null;
  stayCurrentNextAction: string | null;
  validatedActiveChainHeight: number | null;
};

type SmokeReport = {
  baseline: string;
  commands: {
    daemon: string[];
    finalStatus: string[];
    status: string[];
  };
  daemon_sessions: {
    daemon: string[];
    status: string[];
  }[];
  daemon: {
    maybeExitCode: number | null;
    maybeSignal: NodeJS.Signals | null;
    stderrLineCount: number;
    stderrObserved: boolean;
    stdoutLineCount: number;
    stdoutObserved: boolean;
  };
  final_status: FinalStatusSummary | null;
  generated_at_unix_seconds: number;
  kind: "live_mainnet_smoke";
  options: {
    datadir: string;
    manualPeers: string[];
    maybeConfigPath: string | null;
    maybeGeneratedConfigPath: string | null;
    minFreeGib: number;
    outputDir: string;
    pollSeconds: number;
    restartAfterProgress: boolean;
    timeoutSeconds: number;
  };
  network_preflight: {
    completed: boolean;
    endpoint_outcomes: EndpointOutcome[];
  };
  preflight: {
    checks: PreflightCheck[];
    passed: boolean;
  };
  result: {
    blockDelta: number;
    firstBlockProgress: FirstBlockProgressEvidence | null;
    firstHeaderProgress: FirstHeaderProgressEvidence | null;
    headerDelta: number;
    maybeNoProgressCause: NoProgressCause | null;
    message: string;
    nextAction: string;
    progressDetected: boolean;
    restartResumeEvidence: RestartResumeEvidence | null;
    status: ReportStatus;
  };
  schema_version: 2;
  snapshots: SyncStatusSnapshot[];
};

type SyncControlStatusJson = {
  metadata?: RuntimeMetadataJson;
};

type RuntimeMetadataJson = {
  maybe_sync_state?: DurableSyncStateJson;
  recovery_evidence?: FieldAvailability<RecoveryEvidenceStatusJson>;
  sync_control?: {
    paused?: boolean;
  };
};

type FieldAvailability<T> =
  | {
      state: "available";
      value: T;
    }
  | {
      reason?: string;
      state: string;
      value?: unknown;
    };

type ResourcePressureStatusJson = {
  blocks_in_flight?: number;
  max_header_requests_in_flight_per_peer?: number;
  max_headers_per_message?: number;
  max_blocks_in_flight_per_peer?: number;
  max_blocks_in_flight_total?: number;
  max_messages_per_peer?: number;
  max_sync_rounds?: number;
  outbound_peers?: number;
  target_outbound_peers?: number;
};

type ConfiguredTargetsStatusJson = {
  maybe_target_header_height?: number | null;
  target_outbound_peers?: number;
};

type AttemptCountersStatusJson = {
  attempted_peers?: number;
  connected_peers?: number;
  failed_peers?: number;
  max_sync_rounds?: number;
};

type StopReasonStatusJson = {
  label?: string;
  message?: string;
};

type DurableSyncStateJson = {
  peers?: {
    peer_counts?: FieldAvailability<{
      outbound?: number;
    }>;
    recent_peers?: FieldAvailability<RuntimePeerTelemetryJson[]>;
  };
  sync?: {
    attempt_counters?: FieldAvailability<AttemptCountersStatusJson>;
    configured_targets?: FieldAvailability<ConfiguredTargetsStatusJson>;
    last_error?: FieldAvailability<string>;
    lifecycle?: FieldAvailability<string>;
    latest_stop_reason?: FieldAvailability<StopReasonStatusJson>;
    phase?: FieldAvailability<string>;
    progress_signal?: FieldAvailability<string>;
    recovery_action?: FieldAvailability<string>;
    recovery_category?: FieldAvailability<string>;
    resource_pressure?: FieldAvailability<ResourcePressureStatusJson>;
    best_known_tip?: FieldAvailability<Record<string, unknown>>;
    stay_current?: FieldAvailability<string>;
    stay_current_next_action?: FieldAvailability<string>;
    no_progress_diagnosis?: FieldAvailability<string>;
    no_progress_next_action?: FieldAvailability<string>;
    latest_reorg?: FieldAvailability<Record<string, unknown>>;
    reconcile_progress?: FieldAvailability<Record<string, unknown>>;
    sync_progress?: FieldAvailability<{
      block_height?: number;
      blocks_received?: number;
      connected_block_height?: number;
      downloaded_block_height?: number;
      header_height?: number;
      headers_received?: number;
      maybe_connected_block_hash?: string | null;
      maybe_downloaded_block_hash?: string | null;
      maybe_validated_active_chain_hash?: string | null;
      maybe_validated_active_chain_work?: string | null;
      messages_processed?: number;
      validated_active_chain_height?: number;
    }>;
    last_successful_progress_unix_seconds?: FieldAvailability<number>;
  };
  updated_at_unix_seconds?: number;
};

type RuntimePeerTelemetryJson = {
  attempts?: number;
  blocks_received?: number;
  capabilities?: FieldAvailability<string>;
  error?: FieldAvailability<string>;
  failure_reason?: FieldAvailability<string>;
  headers_received?: number;
  maybe_last_activity_unix_seconds?: FieldAvailability<number>;
  maybe_resolved_endpoint?: FieldAvailability<string>;
  network?: string;
  peer?: string;
  source?: string;
  state?: string;
};

type RuntimePeerTelemetry = {
  attempts: number;
  blocksReceived: number;
  headersReceived: number;
  maybeCapabilities: string | null;
  maybeError: string | null;
  maybeFailureReason: string | null;
  maybeLastActivityUnixSeconds: number | null;
  maybeResolvedEndpoint: string | null;
  peer: string;
  source: string;
  state: string;
};

type SmokeSessionMode = "normal" | "until_progress" | "first_snapshot";

type SmokeSessionResult = {
  blockDelta: number;
  daemonSpec: CommandSpec;
  downloadedBlockDelta: number;
  headerDelta: number;
  maybeCancellationSignal: NodeJS.Signals | null;
  maybeExitCode: number | null;
  maybeFirstConnectedBlockProgressSnapshots: {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null;
  maybeFirstDownloadedBlockProgressSnapshots: {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null;
  maybeFirstHeaderProgressSnapshots: {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null;
  maybeLastProbeError: string | null;
  maybeSignal: NodeJS.Signals | null;
  resultMessage: string;
  resultStatus: ReportStatus;
  snapshots: SyncStatusSnapshot[];
  statusSpec: CommandSpec;
  stderrLineCount: number;
  stderrObserved: boolean;
  stderrTail: string;
  stdoutLineCount: number;
  stdoutObserved: boolean;
  stdoutTail: string;
};

function usage(): string {
  return `Usage: bun run scripts/run-live-mainnet-smoke.ts --datadir=PATH [--config=PATH] [--manual-peer=HOST[:PORT]]... [--output-dir=PATH] [--timeout-seconds=N] [--poll-seconds=N] [--min-free-gib=N] [--restart-after-progress]

Launches an explicit opt-in live mainnet smoke flow, polls durable sync status, and writes local JSON/Markdown evidence reports.`;
}

function parseArgs(argv: string[]): Options {
  const options: Options = {
    datadir: "",
    manualPeers: [],
    maybeConfigPath: null,
    maybeGeneratedConfigPath: null,
    minFreeGib: DEFAULT_MIN_FREE_GIB,
    outputDir: DEFAULT_OUTPUT_DIR,
    pollSeconds: DEFAULT_POLL_SECONDS,
    restartAfterProgress: false,
    timeoutSeconds: DEFAULT_TIMEOUT_SECONDS,
  };

  for (const arg of argv) {
    if (arg === "--help" || arg === "-h") {
      console.log(usage());
      process.exit(0);
    }
    if (arg.startsWith("--datadir=")) {
      options.datadir = normalizeRelativePath(arg.slice("--datadir=".length));
      continue;
    }
    if (arg.startsWith("--config=")) {
      options.maybeConfigPath = normalizeRelativePath(arg.slice("--config=".length));
      continue;
    }
    if (arg.startsWith("--manual-peer=")) {
      const manualPeer = arg.slice("--manual-peer=".length).trim();
      if (manualPeer === "") {
        throw new Error("--manual-peer must not be empty");
      }
      parsePeerAddress(manualPeer);
      options.manualPeers.push(manualPeer);
      continue;
    }
    if (arg.startsWith("--output-dir=")) {
      options.outputDir = normalizeRelativePath(arg.slice("--output-dir=".length));
      continue;
    }
    if (arg.startsWith("--timeout-seconds=")) {
      options.timeoutSeconds = parsePositiveInteger(
        arg.slice("--timeout-seconds=".length),
        "--timeout-seconds",
      );
      continue;
    }
    if (arg.startsWith("--poll-seconds=")) {
      options.pollSeconds = parsePositiveInteger(
        arg.slice("--poll-seconds=".length),
        "--poll-seconds",
      );
      continue;
    }
    if (arg.startsWith("--min-free-gib=")) {
      options.minFreeGib = parsePositiveInteger(
        arg.slice("--min-free-gib=".length),
        "--min-free-gib",
      );
      continue;
    }
    if (arg === "--restart-after-progress") {
      options.restartAfterProgress = true;
      continue;
    }

    throw new Error(`unknown argument: ${arg}`);
  }

  if (options.datadir === "") {
    throw new Error("--datadir is required");
  }
  if (options.maybeConfigPath !== null && options.manualPeers.length > 0) {
    throw new Error(
      "--manual-peer cannot be combined with --config; put manual peers in open-bitcoin.jsonc or omit --config so the smoke runner can generate one.",
    );
  }

  return options;
}

function parsePeerAddress(value: string): PeerAddress {
  const defaultPort = 8333;
  if (value.startsWith("[")) {
    const bracketEnd = value.indexOf("]");
    if (bracketEnd <= 1) {
      throw new Error(`invalid peer address: ${value}`);
    }
    const host = value.slice(1, bracketEnd);
    const suffix = value.slice(bracketEnd + 1);
    if (suffix === "") {
      return { address: value, host, port: defaultPort };
    }
    if (!suffix.startsWith(":")) {
      throw new Error(`invalid peer address: ${value}`);
    }
    return { address: value, host, port: parsePort(suffix.slice(1), value) };
  }

  const colonMatches = [...value.matchAll(/:/g)];
  if (colonMatches.length === 1) {
    const colonIndex = colonMatches[0]?.index ?? -1;
    const host = value.slice(0, colonIndex);
    const port = value.slice(colonIndex + 1);
    if (host === "") {
      throw new Error(`invalid peer address: ${value}`);
    }
    return { address: value, host, port: parsePort(port, value) };
  }

  if (value.trim() === "") {
    throw new Error("invalid peer address: empty value");
  }
  return { address: value, host: value, port: defaultPort };
}

function parsePort(value: string, address: string): number {
  if (!/^[0-9]+$/.test(value)) {
    throw new Error(`invalid peer port in ${address}`);
  }
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0 || parsed > 65_535) {
    throw new Error(`invalid peer port in ${address}`);
  }
  return parsed;
}

function parsePositiveInteger(value: string, label: string): number {
  if (!/^[0-9]+$/.test(value)) {
    throw new Error(`${label} must be a positive integer`);
  }
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${label} must be a positive integer`);
  }
  return parsed;
}

function normalizeRelativePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
}

function optionsWithGeneratedManualPeerConfig(repoRootPath: string, options: Options): Options {
  if (options.manualPeers.length === 0) {
    return options;
  }

  const generatedConfigPath = normalizeRelativePath(
    path.join(options.outputDir, GENERATED_CONFIG_FILE_NAME),
  );
  const absoluteOutputDir = path.resolve(repoRootPath, options.outputDir);
  mkdirSync(absoluteOutputDir, { recursive: true });
  writeFileSync(
    path.resolve(repoRootPath, generatedConfigPath),
    generatedManualPeerConfig(options.manualPeers),
  );

  return {
    ...options,
    maybeConfigPath: generatedConfigPath,
    maybeGeneratedConfigPath: generatedConfigPath,
  };
}

function generatedManualPeerConfig(manualPeers: string[]): string {
  return `${JSON.stringify(
    {
      schema_version: 1,
      sync: {
        network_enabled: true,
        mode: "mainnet-ibd",
        manual_peers: manualPeers,
        dns_seeds: [],
        target_outbound_peers: 1,
      },
    },
    null,
    2,
  )}\n`;
}

function repoRoot(): string {
  return execFileSync("git", ["rev-parse", "--show-toplevel"], {
    cwd: process.cwd(),
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  }).trim();
}

function commandExists(command: string): boolean {
  try {
    execFileSync(
      "sh",
      ["-c", 'command -v "$1" >/dev/null 2>&1', "sh", command],
      {
        stdio: "ignore",
      },
    );
    return true;
  } catch {
    return false;
  }
}

function availableBytesForPath(targetPath: string): number {
  const output = execFileSync("df", ["-Pk", targetPath], {
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  });
  const lines = output.trim().split(/\r?\n/);
  const lastLine = lines.at(-1) ?? "";
  const columns = lastLine.trim().split(/\s+/);
  const availableKilobytes = Number.parseInt(columns[3] ?? "", 10);
  if (!Number.isFinite(availableKilobytes) || availableKilobytes <= 0) {
    throw new Error(`unable to parse available disk space for ${targetPath}`);
  }
  return availableKilobytes * 1024;
}

function buildPreflightChecks(
  repoRootPath: string,
  options: Options,
  daemonOverride: string | null,
  statusOverride: string | null,
): PreflightCheck[] {
  const checks: PreflightCheck[] = [];
  const absoluteDatadir = path.resolve(repoRootPath, options.datadir);
  const clockNowSeconds = Math.floor(Date.now() / 1000);

  checks.push({
    detail: existsSync(absoluteDatadir)
      ? `datadir exists at ${options.datadir}`
      : `open-bitcoind mainnet sync activation requires an existing datadir; create ${options.datadir} before running the smoke command.`,
    name: "existing_datadir",
    ok: existsSync(absoluteDatadir),
  });

  if (options.maybeConfigPath !== null) {
    const absoluteConfigPath = path.resolve(repoRootPath, options.maybeConfigPath);
    checks.push({
      detail: existsSync(absoluteConfigPath)
        ? `config exists at ${options.maybeConfigPath}`
        : `--config points to a missing file: ${options.maybeConfigPath}`,
      name: "config_path",
      ok: existsSync(absoluteConfigPath),
    });
  }

  checks.push({
    detail:
      clockNowSeconds >= MIN_REASONABLE_UNIX_SECONDS
        ? `local clock is plausible (${clockNowSeconds})`
        : "system clock appears too far behind; sync status and peer handshakes may be misleading until time is corrected.",
    name: "system_clock",
    ok: clockNowSeconds >= MIN_REASONABLE_UNIX_SECONDS,
  });

  if (process.env.OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK === "1") {
    checks.push({
      detail: "disk-space preflight skipped by OPEN_BITCOIN_LIVE_SMOKE_SKIP_DISK_CHECK=1",
      name: "disk_space",
      ok: true,
    });
  } else if (existsSync(absoluteDatadir)) {
    const availableBytes = availableBytesForPath(absoluteDatadir);
    const minimumBytes = options.minFreeGib * 1024 * 1024 * 1024;
    checks.push({
      detail:
        availableBytes >= minimumBytes
          ? `available disk ${(availableBytes / 1024 / 1024 / 1024).toFixed(1)} GiB meets the ${options.minFreeGib} GiB smoke floor`
          : `available disk ${(availableBytes / 1024 / 1024 / 1024).toFixed(1)} GiB is below the ${options.minFreeGib} GiB smoke floor; free space first or override --min-free-gib for a smaller explicit review run.`,
      name: "disk_space",
      ok: availableBytes >= minimumBytes,
    });
  }

  const daemonCommand = daemonOverride ?? "cargo";
  const statusCommand = statusOverride ?? "cargo";
  checks.push({
    detail: commandExists(daemonCommand)
      ? `daemon command available: ${daemonCommand}`
      : `required daemon command not found: ${daemonCommand}`,
    name: "daemon_command",
    ok: commandExists(daemonCommand),
  });
  checks.push({
    detail: commandExists(statusCommand)
      ? `status command available: ${statusCommand}`
      : `required status command not found: ${statusCommand}`,
    name: "status_command",
    ok: commandExists(statusCommand),
  });

  return checks;
}

async function networkPreflightEndpointOutcomes(
  repoRootPath: string,
  options: Options,
): Promise<EndpointOutcome[]> {
  const maybeFixturePath = process.env.OPEN_BITCOIN_LIVE_SMOKE_NETWORK_PREFLIGHT_FIXTURE;
  if (maybeFixturePath !== undefined) {
    return readEndpointOutcomeFixture(maybeFixturePath);
  }

  if (process.env.OPEN_BITCOIN_LIVE_SMOKE_SKIP_NETWORK_PREFLIGHT === "1") {
    return skippedEndpointOutcomes(
      peerSourcesFromOptions(repoRootPath, options),
      "network preflight skipped by OPEN_BITCOIN_LIVE_SMOKE_SKIP_NETWORK_PREFLIGHT=1",
    );
  }

  const sources = peerSourcesFromOptions(repoRootPath, options);
  const timeoutMilliseconds = networkPreflightTimeoutMilliseconds();
  const endpointsPerSource = endpointsPerSourceLimit();
  const outcomes: EndpointOutcome[] = [];
  for (const source of sources) {
    if (source.skippedReason !== null) {
      outcomes.push(endpointOutcome(source, {
        maybeError: source.skippedReason,
        maybeFailureCause: null,
        maybeResolvedEndpoint: null,
        state: "skipped",
      }));
      continue;
    }

    let resolvedAddresses: Awaited<ReturnType<typeof lookup>>;
    try {
      resolvedAddresses = await lookup(source.peer.host, { all: true });
    } catch (error) {
      outcomes.push(endpointOutcome(source, {
        maybeError: error instanceof Error ? error.message : String(error),
        maybeFailureCause: "dns_resolution_failure",
        maybeResolvedEndpoint: null,
        state: "failed",
      }));
      continue;
    }

    if (resolvedAddresses.length === 0) {
      outcomes.push(endpointOutcome(source, {
        maybeError: "DNS lookup returned no addresses",
        maybeFailureCause: "dns_resolution_failure",
        maybeResolvedEndpoint: null,
        state: "failed",
      }));
      continue;
    }

    for (const [index, resolvedAddress] of resolvedAddresses.entries()) {
      const resolvedEndpoint = `${resolvedAddress.address}:${source.peer.port}`;
      outcomes.push(endpointOutcome(source, {
        maybeError: null,
        maybeFailureCause: null,
        maybeResolvedEndpoint: resolvedEndpoint,
        state: "resolved",
      }));

      if (index >= endpointsPerSource) {
        outcomes.push(endpointOutcome(source, {
          maybeError: `skipped after ${endpointsPerSource} TCP attempt(s) for this source`,
          maybeFailureCause: null,
          maybeResolvedEndpoint: resolvedEndpoint,
          state: "skipped",
        }));
        continue;
      }

      const connectResult = await tcpConnect(
        resolvedAddress.address,
        source.peer.port,
        resolvedAddress.family,
        timeoutMilliseconds,
      );
      outcomes.push(endpointOutcome(source, {
        maybeError: connectResult.maybeError,
        maybeFailureCause: connectResult.connected ? null : "tcp_connection_failure",
        maybeResolvedEndpoint: resolvedEndpoint,
        state: connectResult.connected ? "connected" : "failed",
      }));
    }
  }

  return outcomes;
}

type PeerSource = {
  peer: PeerAddress;
  skippedReason: string | null;
  source: EndpointOutcomeSource;
};

function peerSourcesFromOptions(repoRootPath: string, options: Options): PeerSource[] {
  if (options.manualPeers.length > 0) {
    const manualSources = options.manualPeers.map((peer) => ({
      peer: parsePeerAddress(peer),
      skippedReason: null,
      source: "manual_peer" as const,
    }));
    return [
      ...manualSources,
      {
        peer: parsePeerAddress(DEFAULT_MAINNET_DNS_SEEDS[0] ?? "seed.bitcoin.sipa.be"),
        skippedReason: "manual peers supplied; generated config disables DNS seeds",
        source: "dns_seed",
      },
    ];
  }

  const configuredSources = configuredPeerSources(repoRootPath, options);
  if (configuredSources.length > 0) {
    return configuredSources;
  }

  return DEFAULT_MAINNET_DNS_SEEDS.map((seed) => ({
    peer: parsePeerAddress(seed),
    skippedReason: null,
    source: "dns_seed" as const,
  }));
}

function configuredPeerSources(repoRootPath: string, options: Options): PeerSource[] {
  if (options.maybeConfigPath === null || options.maybeGeneratedConfigPath !== null) {
    return [];
  }

  const maybeConfig = readOpenBitcoinConfig(repoRootPath, options.maybeConfigPath);
  if (maybeConfig === null) {
    return [
      {
        peer: parsePeerAddress(options.maybeConfigPath),
        skippedReason: "unable to parse open-bitcoin JSONC config for endpoint preflight",
        source: "configured_peer",
      },
    ];
  }

  const sync = maybeConfig.sync ?? {};
  const manualPeers = Array.isArray(sync.manual_peers) ? sync.manual_peers : [];
  const maybeDnsSeeds = Array.isArray(sync.dns_seeds) ? sync.dns_seeds : null;
  const dnsSeeds = maybeDnsSeeds ?? DEFAULT_MAINNET_DNS_SEEDS;
  const sources = [
    ...manualPeers.map((peer) => ({
      peer: parsePeerAddress(String(peer)),
      skippedReason: null,
      source: "manual_peer" as const,
    })),
    ...dnsSeeds.map((seed) => ({
      peer: parsePeerAddress(String(seed)),
      skippedReason: null,
      source: "dns_seed" as const,
    })),
  ];

  if (sources.length === 0) {
    return [
      {
        peer: parsePeerAddress(options.maybeConfigPath),
        skippedReason: "config contains no manual peers or DNS seeds",
        source: "configured_peer",
      },
    ];
  }

  return sources;
}

function readOpenBitcoinConfig(
  repoRootPath: string,
  configPath: string,
): { sync?: { dns_seeds?: unknown; manual_peers?: unknown } } | null {
  try {
    const raw = readFileSync(path.resolve(repoRootPath, configPath), "utf8");
    return JSON.parse(stripJsonCommentsAndTrailingCommas(raw));
  } catch {
    return null;
  }
}

function stripJsonCommentsAndTrailingCommas(raw: string): string {
  let output = "";
  let inString = false;
  let escaped = false;
  for (let index = 0; index < raw.length; index += 1) {
    const char = raw[index] ?? "";
    const next = raw[index + 1] ?? "";
    if (inString) {
      output += char;
      if (escaped) {
        escaped = false;
      } else if (char === "\\") {
        escaped = true;
      } else if (char === "\"") {
        inString = false;
      }
      continue;
    }
    if (char === "\"") {
      inString = true;
      output += char;
      continue;
    }
    if (char === "/" && next === "/") {
      while (index < raw.length && raw[index] !== "\n") {
        index += 1;
      }
      output += "\n";
      continue;
    }
    if (char === "/" && next === "*") {
      index += 2;
      while (index < raw.length && !(raw[index] === "*" && raw[index + 1] === "/")) {
        index += 1;
      }
      index += 1;
      continue;
    }
    output += char;
  }
  return output.replace(/,\s*([}\]])/g, "$1");
}

function skippedEndpointOutcomes(sources: PeerSource[], reason: string): EndpointOutcome[] {
  return sources.map((source) =>
    endpointOutcome(source, {
      maybeError: reason,
      maybeFailureCause: null,
      maybeResolvedEndpoint: null,
      state: "skipped",
    }),
  );
}

function readEndpointOutcomeFixture(fixturePath: string): EndpointOutcome[] {
  const decoded = JSON.parse(readFileSync(fixturePath, "utf8"));
  if (!Array.isArray(decoded)) {
    throw new Error("network preflight fixture must be a JSON array");
  }
  return decoded.map((value) => value as EndpointOutcome);
}

function endpointOutcome(
  source: PeerSource,
  fields: Pick<
    EndpointOutcome,
    "maybeError" | "maybeFailureCause" | "maybeResolvedEndpoint" | "state"
  >,
): EndpointOutcome {
  return {
    address: source.peer.address,
    attemptedAtUnixSeconds: Math.floor(Date.now() / 1000),
    host: source.peer.host,
    maybeError: fields.maybeError,
    maybeFailureCause: fields.maybeFailureCause,
    maybeResolvedEndpoint: fields.maybeResolvedEndpoint,
    port: source.peer.port,
    source: source.source,
    stage: "preflight",
    state: fields.state,
  };
}

function networkPreflightTimeoutMilliseconds(): number {
  const maybeTimeout = process.env.OPEN_BITCOIN_LIVE_SMOKE_NETWORK_TIMEOUT_MS;
  if (maybeTimeout === undefined) {
    return DEFAULT_NETWORK_PREFLIGHT_TIMEOUT_MS;
  }
  return parsePositiveInteger(maybeTimeout, "OPEN_BITCOIN_LIVE_SMOKE_NETWORK_TIMEOUT_MS");
}

function endpointsPerSourceLimit(): number {
  const maybeLimit = process.env.OPEN_BITCOIN_LIVE_SMOKE_ENDPOINTS_PER_SOURCE;
  if (maybeLimit === undefined) {
    return DEFAULT_ENDPOINTS_PER_SOURCE;
  }
  return parsePositiveInteger(maybeLimit, "OPEN_BITCOIN_LIVE_SMOKE_ENDPOINTS_PER_SOURCE");
}

function tcpConnect(
  host: string,
  port: number,
  family: number,
  timeoutMilliseconds: number,
): Promise<{ connected: boolean; maybeError: string | null }> {
  return new Promise((resolve) => {
    const socket = createConnection({ family, host, port });
    let settled = false;
    const settle = (connected: boolean, maybeError: string | null) => {
      if (settled) {
        return;
      }
      settled = true;
      socket.destroy();
      resolve({ connected, maybeError });
    };
    socket.setTimeout(timeoutMilliseconds, () => {
      settle(false, `TCP connect timed out after ${timeoutMilliseconds}ms`);
    });
    socket.once("connect", () => settle(true, null));
    socket.once("error", (error) => settle(false, error.message));
  });
}

function ensureBuiltBinaries(repoRootPath: string): void {
  if (
    process.env.OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN !== undefined &&
    process.env.OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN !== undefined
  ) {
    return;
  }

  execFileSync(
    "cargo",
    [
      "build",
      "--manifest-path",
      "packages/Cargo.toml",
      "-p",
      "open-bitcoin-rpc",
      "-p",
      "open-bitcoin-cli",
      "--bins",
    ],
    {
      cwd: repoRootPath,
      stdio: "inherit",
    },
  );
}

async function findFreePort(): Promise<number> {
  const server = Bun.serve({
    fetch() {
      return new Response("unused");
    },
    hostname: "127.0.0.1",
    port: 0,
  });
  const port = server.port;
  server.stop(true);
  return port;
}

function daemonCommand(repoRootPath: string, options: Options, rpcPort: number): CommandSpec {
  const maybeOverride = process.env.OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN;
  if (maybeOverride !== undefined) {
    return {
      args: daemonArgs(options, rpcPort),
      command: maybeOverride,
    };
  }

  return {
    command: path.join(
      repoRootPath,
      "packages/target/debug",
      process.platform === "win32" ? "open-bitcoind.exe" : "open-bitcoind",
    ),
    args: daemonArgs(options, rpcPort),
  };
}

function daemonArgs(options: Options, rpcPort: number): string[] {
  const args = [
    `-datadir=${options.datadir}`,
    "-main",
    `-rpcport=${rpcPort}`,
    "-rpcbind=127.0.0.1",
    "-rpcuser=smoke",
    "-rpcpassword=smoke",
    "-openbitcoinsync=mainnet-ibd",
  ];
  if (options.maybeConfigPath !== null) {
    args.push(`-openbitcoinconf=${options.maybeConfigPath}`);
  }
  return args;
}

function statusCommand(repoRootPath: string, options: Options): CommandSpec {
  const maybeOverride = process.env.OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN;
  if (maybeOverride !== undefined) {
    return {
      args: statusArgs(options),
      command: maybeOverride,
    };
  }

  return {
    command: path.join(
      repoRootPath,
      "packages/target/debug",
      process.platform === "win32" ? "open-bitcoin-cli.exe" : "open-bitcoin-cli",
    ),
    args: statusArgs(options),
  };
}

function statusArgs(options: Options): string[] {
  void options;
  return [];
}

function statusCommandForRpcPort(
  repoRootPath: string,
  options: Options,
  rpcPort: number,
): CommandSpec {
  const statusSpec = statusCommand(repoRootPath, options);
  statusSpec.args = [
    "-rpcconnect=127.0.0.1",
    `-rpcport=${rpcPort}`,
    "-rpcuser=smoke",
    "-rpcpassword=smoke",
    "openbitcoinsyncstatus",
  ];
  return statusSpec;
}

function readSyncStatus(
  repoRootPath: string,
  commandSpec: CommandSpec,
): SyncStatusSnapshot {
  const stdout = execFileSync(commandSpec.command, commandSpec.args, {
    cwd: repoRootPath,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
  const decoded = JSON.parse(stdout) as SyncControlStatusJson | RuntimeMetadataJson;
  return syncStatusSnapshotFromMetadata(runtimeMetadataFromStatusResponse(decoded));
}

function runtimeMetadataFromStatusResponse(
  decoded: SyncControlStatusJson | RuntimeMetadataJson,
): RuntimeMetadataJson {
  const maybeMetadata = (decoded as SyncControlStatusJson).metadata;
  if (maybeMetadata !== undefined) {
    return {
      ...maybeMetadata,
      recovery_evidence:
        (decoded as RuntimeMetadataJson).recovery_evidence ??
        maybeMetadata.recovery_evidence,
    };
  }

  return decoded as RuntimeMetadataJson;
}

function syncStatusSnapshotFromMetadata(metadata: RuntimeMetadataJson): SyncStatusSnapshot {
  const capturedAtUnixSeconds = Math.floor(Date.now() / 1000);
  const maybeSyncState = metadata.maybe_sync_state;
  const maybeSync = maybeSyncState?.sync;
  const maybeProgress = availableValue(maybeSync?.sync_progress);
  const maybePeerCounts = availableValue(maybeSyncState?.peers?.peer_counts);
  const maybeSyncProgressUnavailableReason = unavailableReasonFromFieldAvailability(
    maybeSync?.sync_progress,
  );
  const maybePeerCountsUnavailableReason = unavailableReasonFromFieldAvailability(
    maybeSyncState?.peers?.peer_counts,
  );
  const blockHeight =
    maybeProgress === null ? null : Number(maybeProgress.block_height ?? 0);
  const connectedBlockHeight =
    maybeProgress === null
      ? null
      : Number(maybeProgress.connected_block_height ?? blockHeight ?? 0);
  const downloadedBlockHeight =
    maybeProgress === null
      ? null
      : Number(maybeProgress.downloaded_block_height ?? connectedBlockHeight);

  return {
    attemptCounters: attemptCountersFromValue(
      availableValue(maybeSync?.attempt_counters),
    ),
    blockHeight,
    capturedAtUnixSeconds,
    configuredTargets: configuredTargetsFromValue(
      availableValue(maybeSync?.configured_targets),
    ),
    connectedBlockHeight,
    downloadedBlockHeight,
    headerHeight:
      maybeProgress === null ? null : Number(maybeProgress.header_height ?? 0),
    lifecycle: String(availableValue(maybeSync?.lifecycle) ?? "unavailable"),
    latestStopReason: stopReasonFromValue(
      availableValue(maybeSync?.latest_stop_reason),
    ),
    maybeConnectedBlockHash: valueAsNullableString(
      maybeProgress?.maybe_connected_block_hash,
    ),
    maybeDownloadedBlockHash: valueAsNullableString(
      maybeProgress?.maybe_downloaded_block_hash,
    ),
    maybeAttemptCountersUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.attempt_counters,
    ),
    maybeConfiguredTargetsUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.configured_targets,
    ),
    maybeLastSuccessfulProgressUnixSeconds: availableValue(
      maybeSync?.last_successful_progress_unix_seconds,
    ),
    maybeLastError: valueAsNullableString(availableValue(maybeSync?.last_error)),
    maybeLastErrorUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_error,
    ),
    maybeLatestStopReasonUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.latest_stop_reason,
    ),
    maybePeerCountsUnavailableReason,
    maybeProgressSignalUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.progress_signal,
    ),
    maybeRecoveryActionUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.recovery_action,
    ),
    maybeRecoveryCategoryUnavailableReason: recoveryCategoryUnavailableReason(
      maybeSync?.recovery_category,
    ),
    maybeResourcePressureUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.resource_pressure,
    ),
    maybeSyncProgressUnavailableReason,
    outboundPeers:
      maybePeerCounts === null ? null : Number(maybePeerCounts.outbound ?? 0),
    paused: metadata.sync_control?.paused === true,
    phase: String(availableValue(maybeSync?.phase) ?? "unavailable"),
    progressSignal: valueAsNullableString(availableValue(maybeSync?.progress_signal)),
    recoveryAction: valueAsNullableString(availableValue(maybeSync?.recovery_action)),
    recoveryCategory: recoveryCategoryFromValue(
      availableValue(maybeSync?.recovery_category),
    ),
    resourcePressure: resourcePressureSummaryFromValue(
      availableValue(maybeSync?.resource_pressure),
    ),
    updatedAtUnixSeconds: Number(maybeSyncState?.updated_at_unix_seconds ?? capturedAtUnixSeconds),
  };
}

function availableValue<T>(value: FieldAvailability<T> | undefined): T | null {
  if (value !== undefined && value.state === "available") {
    return value.value;
  }
  return null;
}

function unavailableReasonFromFieldAvailability<T>(
  value: FieldAvailability<T> | undefined,
): string | null {
  if (value === undefined) {
    return "status field absent";
  }
  if (value.state === "available") {
    return null;
  }

  if (
    typeof value.value === "object" &&
    value.value !== null &&
    "reason" in value.value &&
    typeof value.value.reason === "string" &&
    value.value.reason.trim() !== ""
  ) {
    return value.value.reason;
  }

  if (
    "reason" in value &&
    typeof value.reason === "string" &&
    value.reason.trim() !== ""
  ) {
    return value.reason;
  }

  return "status field unavailable";
}

function valueAsNullableString(value: unknown): string | null {
  return typeof value === "string" && value.trim() !== "" ? value : null;
}

function configuredTargetsFromValue(
  value: ConfiguredTargetsStatusJson | null,
): ConfiguredTargetsSummary | null {
  if (value === null) {
    return null;
  }
  return {
    maybeTargetHeaderHeight:
      typeof value.maybe_target_header_height === "number"
        ? value.maybe_target_header_height
        : null,
    targetOutboundPeers: Number(value.target_outbound_peers ?? 0),
  };
}

function attemptCountersFromValue(
  value: AttemptCountersStatusJson | null,
): AttemptCountersSummary | null {
  if (value === null) {
    return null;
  }
  return {
    attemptedPeers: Number(value.attempted_peers ?? 0),
    connectedPeers: Number(value.connected_peers ?? 0),
    failedPeers: Number(value.failed_peers ?? 0),
    maxSyncRounds: Number(value.max_sync_rounds ?? 0),
  };
}

function stopReasonFromValue(value: StopReasonStatusJson | null): StopReasonSummary | null {
  if (value === null) {
    return null;
  }
  const maybeLabel = valueAsNullableString(value.label);
  if (maybeLabel === null) {
    return null;
  }
  return {
    label: maybeLabel,
    message: valueAsNullableString(value.message) ?? maybeLabel,
  };
}

function bestKnownTipSummaryFromValue(value: unknown): ObjectSummary | null {
  const object = recordFromValue(value);
  if (object === null) {
    return null;
  }
  return {
    source: valueAsNullableString(object.source),
    height: numberOrNull(object.height),
    blockHash: valueAsNullableString(object.block_hash),
    work: valueAsNullableString(object.work),
    blockTimeUnixSeconds: numberOrNull(object.block_time_unix_seconds),
    observedAtUnixSeconds: numberOrNull(object.observed_at_unix_seconds),
    freshness: valueAsNullableString(object.freshness),
  };
}

function latestReorgSummaryFromValue(value: unknown): ObjectSummary | null {
  const object = recordFromValue(value);
  if (object === null) {
    return null;
  }
  return {
    commonAncestorHeight: numberOrNull(object.common_ancestor_height),
    commonAncestorHash: valueAsNullableString(object.common_ancestor_hash),
    disconnectedCount: numberOrNull(object.disconnected_count),
    connectedCount: numberOrNull(object.connected_count),
    finalActiveHeight: numberOrNull(object.final_active_height),
    finalActiveHash: valueAsNullableString(object.final_active_hash),
    fullyPersisted:
      typeof object.fully_persisted === "boolean" ? object.fully_persisted : null,
  };
}

function reconcileProgressSummaryFromValue(value: unknown): ObjectSummary | null {
  const object = recordFromValue(value);
  if (object === null) {
    return null;
  }
  const details = recordFromValue(object.details);
  return {
    state: valueAsNullableString(object.state),
    connectedCount: numberOrNull(details?.connected_count),
    finalActiveHeight: numberOrNull(details?.final_active_height),
    finalActiveHash: valueAsNullableString(details?.final_active_hash),
    missingBlockCount: numberOrNull(details?.missing_block_count),
  };
}

function recoveryEvidenceSummaryFromAvailability(
  value: FieldAvailability<RecoveryEvidenceStatusJson> | undefined,
): RecoveryEvidenceSummary | null {
  const maybeEvidence = availableValue(value);
  if (maybeEvidence === null) {
    return null;
  }
  return {
    actionClass: valueAsNullableString(maybeEvidence.action_class),
    affectedNamespace: valueAsNullableString(maybeEvidence.maybe_affected_namespace),
    affectedPath: valueAsNullableString(maybeEvidence.maybe_affected_path),
    category: valueAsNullableString(maybeEvidence.category),
    cause: valueAsNullableString(maybeEvidence.cause),
    compatibilityAction: valueAsNullableString(
      availableValue(maybeEvidence.compatibility_action),
    ),
    evidenceBasis: Array.isArray(maybeEvidence.evidence_basis)
      ? maybeEvidence.evidence_basis.map((basis) => String(basis))
      : [],
    maybeUnavailableReason: null,
    nextAction: valueAsNullableString(maybeEvidence.next_action),
    source: "status.recovery_evidence",
    state: "available",
  };
}

function recoveryEvidenceUnavailableReason(
  value: FieldAvailability<RecoveryEvidenceStatusJson> | undefined,
): string | null {
  if (value === undefined) {
    return "recovery evidence unavailable";
  }
  return unavailableReasonFromFieldAvailability(value);
}

function peerContributionFromValues(
  attemptCounters: AttemptCountersSummary | null,
  maybePeerCounts: { outbound?: number } | null,
): PeerContributionSummary | null {
  if (attemptCounters === null && maybePeerCounts === null) {
    return null;
  }
  return {
    attempted: attemptCounters?.attemptedPeers ?? null,
    connected: attemptCounters?.connectedPeers ?? null,
    failed: attemptCounters?.failedPeers ?? null,
    outbound:
      maybePeerCounts === null ? null : Number(maybePeerCounts.outbound ?? 0),
  };
}

function recordFromValue(value: unknown): Record<string, unknown> | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return null;
  }
  return value as Record<string, unknown>;
}

function numberOrNull(value: unknown): number | null {
  return typeof value === "number" ? value : null;
}

function finalStatusCommand(repoRootPath: string, options: Options): CommandSpec {
  const maybeOverride = process.env.OPEN_BITCOIN_LIVE_SMOKE_FINAL_STATUS_BIN;
  if (maybeOverride !== undefined) {
    return {
      args: finalStatusArgs(options),
      command: maybeOverride,
    };
  }

  return {
    command: path.join(
      repoRootPath,
      "packages/target/debug",
      process.platform === "win32" ? "open-bitcoin.exe" : "open-bitcoin",
    ),
    args: finalStatusArgs(options),
  };
}

function finalStatusArgs(options: Options): string[] {
  const args = ["--datadir", options.datadir];
  if (options.maybeConfigPath !== null) {
    args.push("--config", options.maybeConfigPath);
  }
  args.push("--format", "json", "sync", "status");
  return args;
}

function readFinalStatus(
  repoRootPath: string,
  commandSpec: CommandSpec,
): FinalStatusSummary | null {
  const stdout = execFileSync(commandSpec.command, commandSpec.args, {
    cwd: repoRootPath,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
  const decoded = JSON.parse(stdout) as RuntimeMetadataJson;
  return finalStatusSummaryFromMetadata(decoded);
}

function finalStatusSummaryFromMetadata(metadata: RuntimeMetadataJson): FinalStatusSummary | null {
  const maybeSyncState = metadata.maybe_sync_state;
  if (maybeSyncState === undefined) {
    return null;
  }

  const maybeSync = maybeSyncState.sync;
  const maybeProgress = availableValue(maybeSync?.sync_progress);
  const maybePeerCounts = availableValue(maybeSyncState.peers?.peer_counts);
  const maybeSyncProgressUnavailableReason = unavailableReasonFromFieldAvailability(
    maybeSync?.sync_progress,
  );
  const maybePeerCountsUnavailableReason = unavailableReasonFromFieldAvailability(
    maybeSyncState.peers?.peer_counts,
  );
  const blockHeight =
    maybeProgress === null ? null : Number(maybeProgress.block_height ?? 0);
  const connectedBlockHeight =
    maybeProgress === null
      ? null
      : Number(maybeProgress.connected_block_height ?? blockHeight ?? 0);
  const downloadedBlockHeight =
    maybeProgress === null
      ? null
      : Number(maybeProgress.downloaded_block_height ?? connectedBlockHeight);
  const maybeValidatedActiveChainHeight =
    maybeProgress === null || typeof maybeProgress.validated_active_chain_height !== "number"
      ? null
      : Number(maybeProgress.validated_active_chain_height);
  const maybeValidatedActiveChainHeightUnavailableReason =
    maybeProgress === null
      ? maybeSyncProgressUnavailableReason
      : typeof maybeProgress.validated_active_chain_height === "number"
        ? null
        : "validated active-chain height unavailable";
  const recentPeers = availableValue(maybeSyncState.peers?.recent_peers)?.map(
    runtimePeerTelemetry,
  ) ?? [];
  const attemptCounters = attemptCountersFromValue(
    availableValue(maybeSync?.attempt_counters),
  );
  const recoveryEvidence = recoveryEvidenceSummaryFromAvailability(
    metadata.recovery_evidence,
  );
  const maybeRecoveryEvidenceUnavailableReason = recoveryEvidenceUnavailableReason(
    metadata.recovery_evidence,
  );
  return {
    attemptCounters,
    bestKnownTip: bestKnownTipSummaryFromValue(
      availableValue(maybeSync?.best_known_tip),
    ),
    blockHeight,
    configuredTargets: configuredTargetsFromValue(
      availableValue(maybeSync?.configured_targets),
    ),
    connectedBlockHeight,
    downloadedBlockHeight,
    headerHeight:
      maybeProgress === null ? null : Number(maybeProgress.header_height ?? 0),
    latestStopReason: stopReasonFromValue(
      availableValue(maybeSync?.latest_stop_reason),
    ),
    latestReorg: latestReorgSummaryFromValue(availableValue(maybeSync?.latest_reorg)),
    maybeAttemptCountersUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.attempt_counters,
    ),
    maybeBestKnownTipUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.best_known_tip,
    ),
    maybeConfiguredTargetsUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.configured_targets,
    ),
    lifecycle: String(availableValue(maybeSync?.lifecycle) ?? "unavailable"),
    maybeConnectedBlockHash: valueAsNullableString(
      maybeProgress?.maybe_connected_block_hash,
    ),
    maybeDownloadedBlockHash: valueAsNullableString(
      maybeProgress?.maybe_downloaded_block_hash,
    ),
    maybeLastSuccessfulProgressUnixSeconds: availableValue(
      maybeSync?.last_successful_progress_unix_seconds,
    ),
    maybeLastError: valueAsNullableString(availableValue(maybeSync?.last_error)),
    maybeLastErrorUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_error,
    ),
    maybeLatestStopReasonUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.latest_stop_reason,
    ),
    maybeLatestReorgUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.latest_reorg,
    ),
    maybeNoProgressDiagnosisUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.no_progress_diagnosis,
    ),
    maybeNoProgressNextActionUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.no_progress_next_action,
    ),
    maybePeerCountsUnavailableReason,
    maybeProgressSignalUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.progress_signal,
    ),
    maybeReconcileProgressUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.reconcile_progress,
    ),
    maybeRecoveryActionUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.recovery_action,
    ),
    maybeRecoveryCategoryUnavailableReason: recoveryCategoryUnavailableReason(
      maybeSync?.recovery_category,
    ),
    maybeResourcePressureUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.resource_pressure,
    ),
    maybeStayCurrentNextActionUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.stay_current_next_action,
    ),
    maybeStayCurrentUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.stay_current,
    ),
    maybeSyncProgressUnavailableReason,
    maybeValidatedActiveChainHeightUnavailableReason,
    maybeValidatedActiveChainHash: valueAsNullableString(
      maybeProgress?.maybe_validated_active_chain_hash,
    ),
    maybeValidatedActiveChainWork: valueAsNullableString(
      maybeProgress?.maybe_validated_active_chain_work,
    ),
    headersReceived:
      maybeProgress === null ? null : Number(maybeProgress.headers_received ?? 0),
    blocksReceived:
      maybeProgress === null ? null : Number(maybeProgress.blocks_received ?? 0),
    messagesProcessed:
      maybeProgress === null
        ? null
        : Number(maybeProgress.messages_processed ?? 0),
    noProgressDiagnosis: valueAsNullableString(
      availableValue(maybeSync?.no_progress_diagnosis),
    ),
    noProgressNextAction: valueAsNullableString(
      availableValue(maybeSync?.no_progress_next_action),
    ),
    outboundPeers:
      maybePeerCounts === null ? null : Number(maybePeerCounts.outbound ?? 0),
    peerContribution: peerContributionFromValues(attemptCounters, maybePeerCounts),
    phase: String(availableValue(maybeSync?.phase) ?? "unavailable"),
    progressSignal: valueAsNullableString(availableValue(maybeSync?.progress_signal)),
    recentPeers,
    reconcileProgress: reconcileProgressSummaryFromValue(
      availableValue(maybeSync?.reconcile_progress),
    ),
    recoveryAction: valueAsNullableString(availableValue(maybeSync?.recovery_action)),
    recoveryActionClass: recoveryEvidence?.actionClass ?? null,
    recoveryCategory: recoveryCategoryFromValue(
      availableValue(maybeSync?.recovery_category),
    ),
    recoveryCause: recoveryEvidence?.cause ?? null,
    recoveryEvidence,
    recoveryNextAction: recoveryEvidence?.nextAction ?? null,
    maybeRecoveryEvidenceUnavailableReason,
    resourcePressure: resourcePressureSummaryFromValue(
      availableValue(maybeSync?.resource_pressure),
    ),
    stayCurrent: valueAsNullableString(availableValue(maybeSync?.stay_current)),
    stayCurrentNextAction: valueAsNullableString(
      availableValue(maybeSync?.stay_current_next_action),
    ),
    validatedActiveChainHeight: maybeValidatedActiveChainHeight,
  };
}

function recoveryCategoryFromValue(value: unknown): RecoveryDiagnosisCategory | null {
  switch (value) {
    case "clean_shutdown":
    case "unclean_shutdown":
    case "incompatible_schema":
    case "store_corruption":
    case "storage_lock_contention":
    case "storage_backend_failure":
    case "resource_exhaustion":
    case "invalid_peer_data":
    case "public_network_unreachable":
    case "operator_cancellation":
      return value;
    default:
      return null;
  }
}

function recoveryCategoryUnavailableReason(
  value: FieldAvailability<string> | undefined,
): string | null {
  const maybeUnavailableReason = unavailableReasonFromFieldAvailability(value);
  if (maybeUnavailableReason !== null) {
    return maybeUnavailableReason;
  }

  const maybeCategory = availableValue(value);
  if (maybeCategory === null || recoveryCategoryFromValue(maybeCategory) !== null) {
    return null;
  }

  return `unknown recovery category: ${String(maybeCategory)}`;
}

function resourcePressureSummaryFromValue(
  value: ResourcePressureStatusJson | null,
): ResourcePressureSummary | null {
  if (value === null) {
    return null;
  }
  return {
    blocksInFlight: Number(value.blocks_in_flight ?? 0),
    maxHeaderRequestsInFlightPerPeer: Number(
      value.max_header_requests_in_flight_per_peer ?? 0,
    ),
    maxHeadersPerMessage: Number(value.max_headers_per_message ?? 0),
    maxBlocksInFlightPerPeer: Number(value.max_blocks_in_flight_per_peer ?? 0),
    maxBlocksInFlightTotal: Number(value.max_blocks_in_flight_total ?? 0),
    maxMessagesPerPeer: Number(value.max_messages_per_peer ?? 0),
    maxSyncRounds: Number(value.max_sync_rounds ?? 0),
    outboundPeers: Number(value.outbound_peers ?? 0),
    targetOutboundPeers: Number(value.target_outbound_peers ?? 0),
  };
}

function runtimePeerTelemetry(value: RuntimePeerTelemetryJson): RuntimePeerTelemetry {
  return {
    attempts: Number(value.attempts ?? 0),
    blocksReceived: Number(value.blocks_received ?? 0),
    headersReceived: Number(value.headers_received ?? 0),
    maybeCapabilities: valueAsNullableString(availableValue(value.capabilities)),
    maybeError: valueAsNullableString(availableValue(value.error)),
    maybeFailureReason: valueAsNullableString(availableValue(value.failure_reason)),
    maybeLastActivityUnixSeconds: availableValue(value.maybe_last_activity_unix_seconds),
    maybeResolvedEndpoint: valueAsNullableString(availableValue(value.maybe_resolved_endpoint)),
    peer: String(value.peer ?? "unavailable"),
    source: String(value.source ?? "unknown"),
    state: String(value.state ?? "unknown"),
  };
}

function endpointOutcomesFromFinalStatus(
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

function classifyNoProgressCause(
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

function noProgressCauseFromFinalStatus(
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

function firstHeaderProgressEvidence(
  maybeSnapshots: { before: SyncStatusSnapshot; after: SyncStatusSnapshot } | null,
  maybeFinalStatus: FinalStatusSummary | null,
): FirstHeaderProgressEvidence | null {
  if (maybeSnapshots === null) {
    return null;
  }
  const maybePeer = maybeFinalStatus?.recentPeers.find(
    (peer) => peer.headersReceived > 0,
  ) ?? null;
  return {
    after: maybeSnapshots.after,
    before: maybeSnapshots.before,
    headerDelta:
      heightDelta(maybeSnapshots.after.headerHeight, maybeSnapshots.before.headerHeight) ?? 0,
    maybeLastActivityUnixSeconds: maybePeer?.maybeLastActivityUnixSeconds ?? null,
    maybePeer: maybePeer?.peer ?? null,
    maybeResolvedEndpoint: maybePeer?.maybeResolvedEndpoint ?? null,
    maybeSource: maybePeer?.source ?? null,
    observedAtUnixSeconds: maybeSnapshots.after.capturedAtUnixSeconds,
  };
}

function firstBlockProgressEvidence(
  maybeSnapshots: { before: SyncStatusSnapshot; after: SyncStatusSnapshot } | null,
  maybeFinalStatus: FinalStatusSummary | null,
  kind: FirstBlockProgressKind,
): FirstBlockProgressEvidence | null {
  if (maybeSnapshots === null) {
    return null;
  }
  const maybePeer = maybeFinalStatus?.recentPeers.find(
    (peer) => peer.blocksReceived > 0,
  ) ?? null;
  return {
    after: maybeSnapshots.after,
    before: maybeSnapshots.before,
    blockHash:
      kind === "connected"
        ? maybeSnapshots.after.maybeConnectedBlockHash
        : maybeSnapshots.after.maybeDownloadedBlockHash,
    height:
      kind === "connected"
        ? maybeSnapshots.after.connectedBlockHeight
        : maybeSnapshots.after.downloadedBlockHeight,
    kind,
    maybeLastActivityUnixSeconds: maybePeer?.maybeLastActivityUnixSeconds ?? null,
    maybePeer: maybePeer?.peer ?? null,
    maybeResolvedEndpoint: maybePeer?.maybeResolvedEndpoint ?? null,
    maybeSource: maybePeer?.source ?? null,
    observedAtUnixSeconds: maybeSnapshots.after.capturedAtUnixSeconds,
  };
}

function nextActionForCause(cause: NoProgressCause | null): string {
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

function attachTailBuffer(
  child: ChildProcess,
  streamName: "stdout" | "stderr",
): {
  lineCount: () => number;
  observed: () => boolean;
  read: () => string;
} {
  let buffer = Buffer.alloc(0);
  let lineCount = 0;
  let currentLineHasBytes = false;
  let observed = false;
  const stream = child[streamName];
  stream?.on("data", (chunk: Buffer | string) => {
    const nextChunk = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    observed = observed || nextChunk.byteLength > 0;
    for (const byte of nextChunk) {
      if (byte === 10) {
        lineCount += 1;
        currentLineHasBytes = false;
        continue;
      }
      if (byte !== 13) {
        currentLineHasBytes = true;
      }
    }
    buffer = Buffer.concat([buffer, nextChunk]);
    if (buffer.byteLength > MAX_TAIL_BYTES) {
      buffer = buffer.subarray(buffer.byteLength - MAX_TAIL_BYTES);
    }
  });

  return {
    lineCount: () => lineCount + (currentLineHasBytes ? 1 : 0),
    observed: () => observed,
    read: () => buffer.toString("utf8"),
  };
}

async function terminateChild(child: ChildProcess): Promise<void> {
  if (child.exitCode !== null || child.signalCode !== null) {
    return;
  }

  child.kill("SIGTERM");
  const exited = await waitForExit(child, 5_000);
  if (exited) {
    return;
  }

  child.kill("SIGKILL");
  await waitForExit(child, 2_000);
}

function waitForExit(child: ChildProcess, timeoutMilliseconds: number): Promise<boolean> {
  return new Promise((resolve) => {
    if (child.exitCode !== null || child.signalCode !== null) {
      resolve(true);
      return;
    }

    const timer = setTimeout(() => {
      child.removeListener("exit", onExit);
      resolve(false);
    }, timeoutMilliseconds);

    function onExit() {
      clearTimeout(timer);
      resolve(true);
    }

    child.once("exit", onExit);
  });
}

function sleep(milliseconds: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

async function runSmokeSession(
  repoRootPath: string,
  options: Options,
  mode: SmokeSessionMode,
): Promise<SmokeSessionResult> {
  const rpcPort = await findFreePort();
  const daemonSpec = daemonCommand(repoRootPath, options, rpcPort);
  const statusSpec = statusCommandForRpcPort(repoRootPath, options, rpcPort);
  const child = spawn(daemonSpec.command, daemonSpec.args, {
    cwd: repoRootPath,
    env: process.env,
    stdio: ["ignore", "pipe", "pipe"],
  });
  const stdoutTail = attachTailBuffer(child, "stdout");
  const stderrTail = attachTailBuffer(child, "stderr");
  let maybeSpawnError: string | null = null;
  child.once("error", (error) => {
    maybeSpawnError = error.message;
  });

  const snapshots: SyncStatusSnapshot[] = [];
  let resultStatus: ReportStatus = "no_progress";
  let resultMessage =
    "No header or block progress was observed before timeout. Check outbound network access, DNS reachability, local disk headroom, and system time.";
  let headerDelta = 0;
  let blockDelta = 0;
  let downloadedBlockDelta = 0;
  let maybeFirstHeaderProgressSnapshots: {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null = null;
  let maybeFirstDownloadedBlockProgressSnapshots: {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null = null;
  let maybeFirstConnectedBlockProgressSnapshots: {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null = null;
  let maybeLastProbeError: string | null = null;
  let maybeCancellationSignal: NodeJS.Signals | null = null;
  const cancellationHandler = (signal: NodeJS.Signals) => {
    maybeCancellationSignal = signal;
  };
  process.once("SIGINT", cancellationHandler);
  process.once("SIGTERM", cancellationHandler);

  try {
    await sleep(2_000);
    const startedAt = Date.now();
    let initialSnapshot: SyncStatusSnapshot | null = null;

    while (Date.now() - startedAt <= options.timeoutSeconds * 1_000) {
      if (maybeCancellationSignal !== null) {
        resultStatus = "cancelled";
        resultMessage = `live mainnet smoke cancelled by operator signal ${maybeCancellationSignal}`;
        break;
      }
      if (child.exitCode !== null && snapshots.length === 0) {
        resultStatus = "runtime_failed";
        resultMessage = "open-bitcoind exited before the first sync status snapshot could be collected.";
        break;
      }
      if (maybeSpawnError !== null) {
        resultStatus = "runtime_failed";
        resultMessage = `open-bitcoind failed to start: ${maybeSpawnError}`;
        break;
      }

      let snapshot: SyncStatusSnapshot;
      try {
        snapshot = readSyncStatus(repoRootPath, statusSpec);
      } catch (error) {
        maybeLastProbeError =
          error instanceof Error ? error.message : String(error);
        if (child.exitCode !== null) {
          resultStatus = "runtime_failed";
          resultMessage = `failed to read daemon RPC state after exit: ${maybeLastProbeError}`;
          break;
        }
        await sleep(options.pollSeconds * 1_000);
        continue;
      }
      snapshots.push(snapshot);
      if (initialSnapshot === null) {
        initialSnapshot = snapshot;
      }

      const maybeHeaderDelta = heightDelta(snapshot.headerHeight, initialSnapshot.headerHeight);
      const maybeDownloadedBlockDelta = heightDelta(
        snapshot.downloadedBlockHeight,
        initialSnapshot.downloadedBlockHeight,
      );
      const maybeConnectedBlockDelta = heightDelta(
        snapshot.connectedBlockHeight,
        initialSnapshot.connectedBlockHeight,
      );
      headerDelta = maybeHeaderDelta ?? 0;
      downloadedBlockDelta = maybeDownloadedBlockDelta ?? 0;
      blockDelta = maybeConnectedBlockDelta ?? 0;
      if (
        maybeHeaderDelta !== null &&
        maybeHeaderDelta > 0 &&
        maybeFirstHeaderProgressSnapshots === null
      ) {
        maybeFirstHeaderProgressSnapshots = {
          after: snapshot,
          before: initialSnapshot,
        };
      }
      if (
        maybeDownloadedBlockDelta !== null &&
        maybeDownloadedBlockDelta > 0 &&
        maybeFirstDownloadedBlockProgressSnapshots === null
      ) {
        maybeFirstDownloadedBlockProgressSnapshots = {
          after: snapshot,
          before: initialSnapshot,
        };
      }
      if (
        maybeConnectedBlockDelta !== null &&
        maybeConnectedBlockDelta > 0 &&
        maybeFirstConnectedBlockProgressSnapshots === null
      ) {
        maybeFirstConnectedBlockProgressSnapshots = {
          after: snapshot,
          before: initialSnapshot,
        };
      }
      if (mode === "first_snapshot") {
        resultStatus = "passed";
        resultMessage = "Collected a fresh post-restart sync status snapshot.";
        break;
      }
      if (
        mode === "until_progress" &&
        ((maybeHeaderDelta !== null && maybeHeaderDelta > 0) ||
          (maybeDownloadedBlockDelta !== null && maybeDownloadedBlockDelta > 0) ||
          (maybeConnectedBlockDelta !== null && maybeConnectedBlockDelta > 0))
      ) {
        resultStatus = "passed";
        resultMessage = `Observed progress before requested restart (header delta ${headerDelta}, downloaded block delta ${downloadedBlockDelta}, connected block delta ${blockDelta}).`;
        break;
      }
      if (mode === "normal" && blockDelta > 0) {
        resultStatus = "passed";
        resultMessage = `Observed first connected mainnet block progress through the daemon status surface (header delta ${headerDelta}, connected block delta ${blockDelta}).`;
        break;
      }

      if (child.exitCode !== null) {
        resultStatus = "runtime_failed";
        resultMessage =
          snapshot.maybeLastError === null
            ? "open-bitcoind exited before reporting progress."
            : `open-bitcoind exited before reporting progress: ${snapshot.maybeLastError}`;
        break;
      }

      await sleep(options.pollSeconds * 1_000);
    }
  } finally {
    await terminateChild(child);
    process.removeListener("SIGINT", cancellationHandler);
    process.removeListener("SIGTERM", cancellationHandler);
  }

  return {
    blockDelta,
    daemonSpec,
    downloadedBlockDelta,
    headerDelta,
    maybeCancellationSignal,
    maybeExitCode: child.exitCode,
    maybeFirstConnectedBlockProgressSnapshots,
    maybeFirstDownloadedBlockProgressSnapshots,
    maybeFirstHeaderProgressSnapshots,
    maybeLastProbeError,
    maybeSignal: child.signalCode,
    resultMessage,
    resultStatus,
    snapshots,
    statusSpec,
    stderrLineCount: stderrTail.lineCount(),
    stderrObserved: stderrTail.observed(),
    stderrTail: stderrTail.read(),
    stdoutLineCount: stdoutTail.lineCount(),
    stdoutObserved: stdoutTail.observed(),
    stdoutTail: stdoutTail.read(),
  };
}

function firstNonNullProgressSnapshots(
  sessions: SmokeSessionResult[],
  selector: (session: SmokeSessionResult) => {
    before: SyncStatusSnapshot;
    after: SyncStatusSnapshot;
  } | null,
): { before: SyncStatusSnapshot; after: SyncStatusSnapshot } | null {
  for (const session of sessions) {
    const maybeSnapshots = selector(session);
    if (maybeSnapshots !== null) {
      return maybeSnapshots;
    }
  }
  return null;
}

function lastSnapshot(snapshots: SyncStatusSnapshot[]): SyncStatusSnapshot | null {
  return snapshots.at(-1) ?? null;
}

function restartProgressSummary(
  maybeSnapshot: SyncStatusSnapshot | null,
): RestartProgressSummary | null {
  if (maybeSnapshot === null) {
    return null;
  }
  return {
    attemptCounters: maybeSnapshot.attemptCounters,
    configuredTargets: maybeSnapshot.configuredTargets,
    connectedBlockHeight: maybeSnapshot.connectedBlockHeight,
    downloadedBlockHeight: maybeSnapshot.downloadedBlockHeight,
    headerHeight: maybeSnapshot.headerHeight,
    latestStopReason: maybeSnapshot.latestStopReason,
    lifecycle: maybeSnapshot.lifecycle,
    maybeAttemptCountersUnavailableReason:
      maybeSnapshot.maybeAttemptCountersUnavailableReason,
    maybeConnectedBlockHash: maybeSnapshot.maybeConnectedBlockHash,
    maybeConfiguredTargetsUnavailableReason:
      maybeSnapshot.maybeConfiguredTargetsUnavailableReason,
    maybeDownloadedBlockHash: maybeSnapshot.maybeDownloadedBlockHash,
    maybeLastError: maybeSnapshot.maybeLastError,
    maybeLastErrorUnavailableReason: maybeSnapshot.maybeLastErrorUnavailableReason,
    maybeLastSuccessfulProgressUnixSeconds:
      maybeSnapshot.maybeLastSuccessfulProgressUnixSeconds,
    maybeLatestStopReasonUnavailableReason:
      maybeSnapshot.maybeLatestStopReasonUnavailableReason,
    maybePeerCountsUnavailableReason:
      maybeSnapshot.maybePeerCountsUnavailableReason,
    maybeProgressSignalUnavailableReason:
      maybeSnapshot.maybeProgressSignalUnavailableReason,
    maybeRecoveryActionUnavailableReason:
      maybeSnapshot.maybeRecoveryActionUnavailableReason,
    maybeRecoveryCategoryUnavailableReason:
      maybeSnapshot.maybeRecoveryCategoryUnavailableReason,
    maybeResourcePressureUnavailableReason:
      maybeSnapshot.maybeResourcePressureUnavailableReason,
    maybeSyncProgressUnavailableReason:
      maybeSnapshot.maybeSyncProgressUnavailableReason,
    phase: maybeSnapshot.phase,
    progressSignal: maybeSnapshot.progressSignal,
    recoveryAction: maybeSnapshot.recoveryAction,
    recoveryCategory: maybeSnapshot.recoveryCategory,
    resourcePressure: maybeSnapshot.resourcePressure,
  };
}

function heightDelta(after: number | null, before: number | null): number | null {
  if (after === null || before === null) {
    return null;
  }
  return after - before;
}

function hasProgressHeights(snapshot: SyncStatusSnapshot): boolean {
  return (
    snapshot.headerHeight !== null &&
    snapshot.downloadedBlockHeight !== null &&
    snapshot.connectedBlockHeight !== null
  );
}

function restartProgressDelta(
  before: SyncStatusSnapshot,
  after: SyncStatusSnapshot,
): RestartProgressDelta | null {
  const maybeConnectedBlockDelta = heightDelta(
    after.connectedBlockHeight,
    before.connectedBlockHeight,
  );
  const maybeDownloadedBlockDelta = heightDelta(
    after.downloadedBlockHeight,
    before.downloadedBlockHeight,
  );
  const maybeHeaderDelta = heightDelta(after.headerHeight, before.headerHeight);
  if (
    maybeConnectedBlockDelta === null ||
    maybeDownloadedBlockDelta === null ||
    maybeHeaderDelta === null
  ) {
    return null;
  }
  return {
    connectedBlockDelta: maybeConnectedBlockDelta,
    downloadedBlockDelta: maybeDownloadedBlockDelta,
    headerDelta: maybeHeaderDelta,
  };
}

function peerOutcomeSummary(endpointOutcomes: EndpointOutcome[]): RestartPeerOutcomeSummary {
  const failureCauses = Array.from(
    new Set(
      endpointOutcomes
        .map((outcome) => outcome.maybeFailureCause)
        .filter((cause): cause is NoProgressCause => cause !== null),
    ),
  );
  return {
    connected: endpointOutcomes.filter((outcome) => outcome.state === "connected").length,
    failed: endpointOutcomes.filter((outcome) => outcome.state === "failed").length,
    failureCauses,
    handshook: endpointOutcomes.filter((outcome) => outcome.state === "handshook").length,
    skipped: endpointOutcomes.filter((outcome) => outcome.state === "skipped").length,
  };
}

function duplicateConnectVerdict(
  beforeRestart: SyncStatusSnapshot | null,
  afterRestart: SyncStatusSnapshot | null,
): DuplicateConnectVerdict {
  if (beforeRestart === null || afterRestart === null) {
    return "unavailable";
  }
  if (!hasProgressHeights(beforeRestart) || !hasProgressHeights(afterRestart)) {
    return "unavailable";
  }
  const beforeDownloadedBlockHeight = beforeRestart.downloadedBlockHeight;
  const afterDownloadedBlockHeight = afterRestart.downloadedBlockHeight;
  const beforeConnectedBlockHeight = beforeRestart.connectedBlockHeight;
  const afterConnectedBlockHeight = afterRestart.connectedBlockHeight;
  if (
    (afterDownloadedBlockHeight === beforeDownloadedBlockHeight &&
      afterRestart.maybeDownloadedBlockHash !== beforeRestart.maybeDownloadedBlockHash) ||
    (afterConnectedBlockHeight === beforeConnectedBlockHeight &&
      afterRestart.maybeConnectedBlockHash !== beforeRestart.maybeConnectedBlockHash)
  ) {
    return "duplicate_connect_suspected";
  }
  if (
    beforeConnectedBlockHeight !== null &&
    afterConnectedBlockHeight !== null &&
    beforeConnectedBlockHeight > 0 &&
    afterConnectedBlockHeight >= beforeConnectedBlockHeight &&
    afterRestart.maybeConnectedBlockHash === beforeRestart.maybeConnectedBlockHash
  ) {
    return "no_duplicate_connect_observed";
  }
  if (
    beforeConnectedBlockHeight !== null &&
    afterConnectedBlockHeight !== null &&
    afterConnectedBlockHeight < beforeConnectedBlockHeight
  ) {
    return "duplicate_connect_suspected";
  }
  return "unavailable";
}

function restartStatus(
  beforeRestart: SyncStatusSnapshot | null,
  afterRestart: SyncStatusSnapshot | null,
  maybeCancellationSignal: NodeJS.Signals | null,
): RestartStatus {
  if (maybeCancellationSignal !== null) {
    return "cancelled";
  }
  if (beforeRestart === null || afterRestart === null) {
    return "blocked_before_restart";
  }
  if (!hasProgressHeights(beforeRestart) || !hasProgressHeights(afterRestart)) {
    return "blocked_before_restart";
  }
  const beforeHeaderHeight = beforeRestart.headerHeight;
  const afterHeaderHeight = afterRestart.headerHeight;
  const beforeDownloadedBlockHeight = beforeRestart.downloadedBlockHeight;
  const afterDownloadedBlockHeight = afterRestart.downloadedBlockHeight;
  const beforeConnectedBlockHeight = beforeRestart.connectedBlockHeight;
  const afterConnectedBlockHeight = afterRestart.connectedBlockHeight;
  if (
    beforeHeaderHeight === null ||
    afterHeaderHeight === null ||
    beforeDownloadedBlockHeight === null ||
    afterDownloadedBlockHeight === null ||
    beforeConnectedBlockHeight === null ||
    afterConnectedBlockHeight === null
  ) {
    return "blocked_before_restart";
  }
  if (
    afterHeaderHeight >= beforeHeaderHeight &&
    afterDownloadedBlockHeight >= beforeDownloadedBlockHeight &&
    afterConnectedBlockHeight >= beforeConnectedBlockHeight &&
    unchangedHeightHashStable(
      beforeDownloadedBlockHeight,
      beforeRestart.maybeDownloadedBlockHash,
      afterDownloadedBlockHeight,
      afterRestart.maybeDownloadedBlockHash,
    ) &&
    unchangedHeightHashStable(
      beforeConnectedBlockHeight,
      beforeRestart.maybeConnectedBlockHash,
      afterConnectedBlockHeight,
      afterRestart.maybeConnectedBlockHash,
    )
  ) {
    return "completed";
  }
  return "blocked_before_restart";
}

function unchangedHeightHashStable(
  beforeHeight: number | null,
  beforeHash: string | null,
  afterHeight: number | null,
  afterHash: string | null,
): boolean {
  if (beforeHeight === null || afterHeight === null) {
    return false;
  }
  if (afterHeight !== beforeHeight) {
    return true;
  }
  return beforeHash === afterHash;
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
  const maybeLastError =
    maybeFinalStatus?.maybeLastError ?? maybeLastProbeError ?? maybePeer?.maybeError ?? null;
  const maybeNoProgressCause =
    status === "completed"
      ? null
      : classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError);
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
  const maybeStorageRecoveryAction =
    category === "incompatible_schema" ||
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
    const beforeIsBoundary =
      start === 0 || isAsciiWordBoundary(haystack.charCodeAt(start - 1));
    const afterIsBoundary =
      end === haystack.length || isAsciiWordBoundary(haystack.charCodeAt(end));
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

function restartResumeEvidence(
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
    maybePostRestartProgressDelta:
      afterRestart === null || afterLatest === null
        ? null
        : restartProgressDelta(afterRestart, afterLatest),
    peerOutcomeSummary: peerOutcomeSummary(endpointOutcomes),
    recoveryDiagnosis: recoveryDiagnosis(
      endpointOutcomes,
      maybeFinalStatus,
      maybeLastProbeError,
      status,
    ),
    restartStatus: status,
    sameDatadir: {
      requestedPathMatched:
        requestedBefore !== null && requestedAfter !== null && requestedBefore === requestedAfter,
      resolvedPathMatched:
        requestedBeforePath !== null &&
        requestedAfterPath !== null &&
        path.resolve(repoRootPath, requestedBeforePath) ===
          path.resolve(repoRootPath, requestedAfterPath),
    },
  };
}

function markdownReport(report: SmokeReport): string {
  const preflightRows = report.preflight.checks
    .map(
      (check) =>
        `| ${check.name} | ${check.ok ? "passed" : "failed"} | ${escapeTableCell(check.detail)} |`,
    )
    .join("\n");
  const snapshotRows =
    report.snapshots.length === 0
      ? "| Unavailable: no sync status snapshots captured | - | - | - | - | - | - | - | - | - | - | - | - |\n"
      : report.snapshots
          .map(
            (snapshot) =>
              `| ${snapshot.capturedAtUnixSeconds} | ${snapshot.lifecycle} | ${snapshot.phase} | ${fieldText(snapshot.progressSignal, snapshot.maybeProgressSignalUnavailableReason)} | ${configuredTargetsText(snapshot.configuredTargets, snapshot.maybeConfiguredTargetsUnavailableReason, snapshot.outboundPeers, snapshot.maybePeerCountsUnavailableReason)} | ${attemptCountersText(snapshot.attemptCounters, snapshot.maybeAttemptCountersUnavailableReason)} | ${fieldText(snapshot.headerHeight, snapshot.maybeSyncProgressUnavailableReason)} | ${blockEvidenceText(snapshot.downloadedBlockHeight, snapshot.maybeDownloadedBlockHash, "downloaded", snapshot.maybeSyncProgressUnavailableReason)} | ${blockEvidenceText(snapshot.connectedBlockHeight, snapshot.maybeConnectedBlockHash, "connected", snapshot.maybeSyncProgressUnavailableReason)} | ${fieldText(snapshot.recoveryCategory, snapshot.maybeRecoveryCategoryUnavailableReason)} | ${resourcePressureText(snapshot.resourcePressure, snapshot.maybeResourcePressureUnavailableReason)} | ${stopReasonText(snapshot.latestStopReason, snapshot.maybeLatestStopReasonUnavailableReason)} | ${fieldText(snapshot.maybeLastError, snapshot.maybeLastErrorUnavailableReason)} |`,
          )
          .join("\n");
  const endpointRows =
    report.network_preflight.endpoint_outcomes.length === 0
      ? "| - | - | - | - | - | - | - | - |\n"
      : report.network_preflight.endpoint_outcomes
          .map(
            (outcome) =>
              `| ${outcome.stage} | ${outcome.source} | ${escapeTableCell(outcome.address)} | ${outcome.state} | ${escapeTableCell(outcome.maybeResolvedEndpoint ?? "-")} | ${outcome.maybeFailureCause ?? "-"} | ${escapeTableCell(outcome.maybeError ?? "-")} | ${outcome.attemptedAtUnixSeconds} |`,
          )
          .join("\n");
  const runtimePeerRows =
    report.final_status?.recentPeers.length === 0 || report.final_status === null
      ? "| - | - | - | - | - | - | - | - |\n"
      : report.final_status.recentPeers
          .map(
            (peer) =>
              `| ${escapeTableCell(peer.peer)} | ${peer.source} | ${peer.state} | ${peer.headersReceived} | ${peer.blocksReceived} | ${peer.maybeLastActivityUnixSeconds ?? "-"} | ${escapeTableCell(peer.maybeFailureReason ?? "-")} | ${escapeTableCell(peer.maybeError ?? "-")} |`,
          )
          .join("\n");
  const daemonSessionRows =
    report.daemon_sessions.length === 0
      ? "| - | - | - |\n"
      : report.daemon_sessions
          .map(
            (session, index) =>
              `| ${index + 1} | ${escapeTableCell(session.daemon.join(" "))} | ${escapeTableCell(session.status.join(" "))} |`,
          )
          .join("\n");
  const firstHeaderProgress = report.result.firstHeaderProgress;
  const firstHeaderProgressDetail =
    firstHeaderProgress === null
      ? "Unavailable"
      : `observed at ${firstHeaderProgress.observedAtUnixSeconds}: ${fieldText(firstHeaderProgress.before.headerHeight, firstHeaderProgress.before.maybeSyncProgressUnavailableReason)} -> ${fieldText(firstHeaderProgress.after.headerHeight, firstHeaderProgress.after.maybeSyncProgressUnavailableReason)} via ${escapeInline(firstHeaderProgress.maybePeer ?? "unknown peer")} (${firstHeaderProgress.maybeSource ?? "unknown source"}, endpoint ${escapeInline(firstHeaderProgress.maybeResolvedEndpoint ?? "unavailable")})`;
  const firstBlockProgress = report.result.firstBlockProgress;
  const firstBlockProgressDetail =
    firstBlockProgress === null
      ? "Unavailable"
      : `${firstBlockProgress.kind} observed at ${firstBlockProgress.observedAtUnixSeconds}: height ${fieldText(firstBlockProgress.height, firstBlockProgress.after.maybeSyncProgressUnavailableReason)}, block hash ${escapeInline(firstBlockProgress.blockHash ?? "unavailable")}, downloaded ${fieldText(firstBlockProgress.before.downloadedBlockHeight, firstBlockProgress.before.maybeSyncProgressUnavailableReason)} -> ${fieldText(firstBlockProgress.after.downloadedBlockHeight, firstBlockProgress.after.maybeSyncProgressUnavailableReason)}, connected ${fieldText(firstBlockProgress.before.connectedBlockHeight, firstBlockProgress.before.maybeSyncProgressUnavailableReason)} -> ${fieldText(firstBlockProgress.after.connectedBlockHeight, firstBlockProgress.after.maybeSyncProgressUnavailableReason)}, peer ${escapeInline(firstBlockProgress.maybePeer ?? "unknown peer")} (${firstBlockProgress.maybeSource ?? "unknown source"}, endpoint ${escapeInline(firstBlockProgress.maybeResolvedEndpoint ?? "unavailable")})`;
  const restartEvidence = report.result.restartResumeEvidence;
  const restartEvidenceDetail =
    restartEvidence === null
      ? "Unavailable"
      : `status ${restartEvidence.restartStatus}, same datadir requested=${restartEvidence.sameDatadir.requestedPathMatched ? "yes" : "no"} resolved=${restartEvidence.sameDatadir.resolvedPathMatched ? "yes" : "no"}, before header/downloaded/connected ${progressTripletText(restartEvidence.beforeRestart)}, after header/downloaded/connected ${progressTripletText(restartEvidence.afterRestart)}, duplicate verdict ${restartEvidence.duplicateConnectVerdict}`;

  return `# Open Bitcoin Live Mainnet Smoke Report

## Result

- Status: \`${report.result.status}\`
- Message: ${report.result.message}
- Progress detected: ${report.result.progressDetected ? "yes" : "no"}
- No-progress cause: ${report.result.maybeNoProgressCause ?? "Unavailable"}
- Next action: ${report.result.nextAction}
- Header delta: ${report.result.headerDelta}
- Block delta: ${report.result.blockDelta}
- First header progress: ${firstHeaderProgressDetail}
- First block progress: ${firstBlockProgressDetail}
- Restart/resume evidence: ${restartEvidenceDetail}

## Restart/resume evidence

- Restart status: ${restartEvidence?.restartStatus ?? "Unavailable"}
- Same datadir requested path matched: ${restartEvidence?.sameDatadir.requestedPathMatched ?? false}
- Same datadir resolved path matched: ${restartEvidence?.sameDatadir.resolvedPathMatched ?? false}
- Before restart header/downloaded/connected: ${progressTripletText(restartEvidence?.beforeRestart ?? null)}
- After restart header/downloaded/connected: ${progressTripletText(restartEvidence?.afterRestart ?? null)}
- Duplicate connect verdict: ${restartEvidence?.duplicateConnectVerdict ?? "Unavailable"}
- Post-restart progress delta: ${restartEvidence?.maybePostRestartProgressDelta === null || restartEvidence?.maybePostRestartProgressDelta === undefined ? "Unavailable" : `${restartEvidence.maybePostRestartProgressDelta.headerDelta}/${restartEvidence.maybePostRestartProgressDelta.downloadedBlockDelta}/${restartEvidence.maybePostRestartProgressDelta.connectedBlockDelta}`}

## Options

- Datadir: \`${report.options.datadir}\`
- Config: ${report.options.maybeConfigPath === null ? "Unavailable" : `\`${report.options.maybeConfigPath}\``}
- Generated config: ${report.options.maybeGeneratedConfigPath === null ? "Unavailable" : `\`${report.options.maybeGeneratedConfigPath}\``}
- Manual peers: ${report.options.manualPeers.length === 0 ? "Unavailable" : report.options.manualPeers.map((peer) => `\`${peer}\``).join(", ")}
- Output directory: \`${report.options.outputDir}\`
- Timeout: ${report.options.timeoutSeconds}s
- Poll interval: ${report.options.pollSeconds}s
- Minimum free disk floor: ${report.options.minFreeGib} GiB

## Preflight

| Check | Result | Detail |
| --- | --- | --- |
${preflightRows}

## Network Endpoint Outcomes

| Stage | Source | Address | State | Resolved Endpoint | Cause | Error | Attempted At |
| --- | --- | --- | --- | --- | --- | --- | ---: |
${endpointRows}

## Snapshots

| Captured At | Lifecycle | Phase | Signal | Configured Targets | Attempts | Header Height | Downloaded Block | Connected Block | Recovery Category | Resource Pressure | Latest Stop Reason | Latest Error |
| --- | --- | --- | --- | --- | --- | ---: | --- | --- | --- | --- | --- | --- |
${snapshotRows}

## Commands

- Daemon: \`${[...report.commands.daemon].join(" ")}\`
- Status: \`${[...report.commands.status].join(" ")}\`
- Final status: \`${[...report.commands.finalStatus].join(" ")}\`

## Daemon Sessions

| Session | Daemon | Status |
| ---: | --- | --- |
${daemonSessionRows}

## Final Durable Status

- Lifecycle: ${report.final_status?.lifecycle ?? "Unavailable"}
- Phase: ${report.final_status?.phase ?? "Unavailable"}
- Configured targets: ${configuredTargetsText(report.final_status?.configuredTargets ?? null, report.final_status?.maybeConfiguredTargetsUnavailableReason ?? null, report.final_status?.outboundPeers ?? null, report.final_status?.maybePeerCountsUnavailableReason ?? null)}
- Attempt counters: ${attemptCountersText(report.final_status?.attemptCounters ?? null, report.final_status?.maybeAttemptCountersUnavailableReason ?? null)}
- Progress signal: ${fieldText(report.final_status?.progressSignal ?? null, report.final_status?.maybeProgressSignalUnavailableReason ?? null)}
- Last progress: ${fieldText(report.final_status?.maybeLastSuccessfulProgressUnixSeconds ?? null, "no successful progress recorded")}
- Latest stop reason: ${stopReasonText(report.final_status?.latestStopReason ?? null, report.final_status?.maybeLatestStopReasonUnavailableReason ?? null)}
- Latest error: ${fieldText(report.final_status?.maybeLastError ?? null, report.final_status?.maybeLastErrorUnavailableReason ?? null)}
- Recovery category: ${fieldText(report.final_status?.recoveryCategory ?? null, report.final_status?.maybeRecoveryCategoryUnavailableReason ?? null)}
- Recovery action: ${fieldText(report.final_status?.recoveryAction ?? null, report.final_status?.maybeRecoveryActionUnavailableReason ?? null)}
- Recovery action class: ${fieldText(report.final_status?.recoveryActionClass ?? null, report.final_status?.maybeRecoveryEvidenceUnavailableReason ?? null)}
- Recovery cause: ${fieldText(report.final_status?.recoveryCause ?? null, report.final_status?.maybeRecoveryEvidenceUnavailableReason ?? null)}
- Recovery next action: ${fieldText(report.final_status?.recoveryNextAction ?? null, report.final_status?.maybeRecoveryEvidenceUnavailableReason ?? null)}
- Resource pressure: ${resourcePressureText(report.final_status?.resourcePressure ?? null, report.final_status?.maybeResourcePressureUnavailableReason ?? null)}
- Peer health: ${peerHealthText(report.final_status?.outboundPeers ?? null, report.final_status?.maybePeerCountsUnavailableReason ?? null)}
- Header height: ${fieldText(report.final_status?.headerHeight ?? null, report.final_status?.maybeSyncProgressUnavailableReason ?? null)}
- Block height: ${fieldText(report.final_status?.blockHeight ?? null, report.final_status?.maybeSyncProgressUnavailableReason ?? null)}
- Downloaded block height: ${fieldText(report.final_status?.downloadedBlockHeight ?? null, report.final_status?.maybeSyncProgressUnavailableReason ?? null)}
- Connected block height: ${fieldText(report.final_status?.connectedBlockHeight ?? null, report.final_status?.maybeSyncProgressUnavailableReason ?? null)}
- Downloaded block hash: ${report.final_status?.maybeDownloadedBlockHash ?? "Unavailable"}
- Connected block hash: ${report.final_status?.maybeConnectedBlockHash ?? "Unavailable"}
- Validated active-chain height: ${fieldText(report.final_status?.validatedActiveChainHeight ?? null, report.final_status?.maybeValidatedActiveChainHeightUnavailableReason ?? report.final_status?.maybeSyncProgressUnavailableReason ?? null)}
- Validated active-chain hash: ${report.final_status?.maybeValidatedActiveChainHash ?? "Unavailable: validated active-chain hash unavailable"}
- Validated active-chain work: ${report.final_status?.maybeValidatedActiveChainWork ?? "Unavailable: validated active-chain work unavailable"}
- Best-known tip: ${objectSummaryText(report.final_status?.bestKnownTip ?? null, report.final_status?.maybeBestKnownTipUnavailableReason ?? null)}
- Stay-current: ${fieldText(report.final_status?.stayCurrent ?? null, report.final_status?.maybeStayCurrentUnavailableReason ?? null)}
- Stay-current action: ${fieldText(report.final_status?.stayCurrentNextAction ?? null, report.final_status?.maybeStayCurrentNextActionUnavailableReason ?? null)}
- No-progress diagnosis: ${fieldText(report.final_status?.noProgressDiagnosis ?? null, report.final_status?.maybeNoProgressDiagnosisUnavailableReason ?? null)}
- No-progress action: ${fieldText(report.final_status?.noProgressNextAction ?? null, report.final_status?.maybeNoProgressNextActionUnavailableReason ?? null)}
- Latest reorg: ${objectSummaryText(report.final_status?.latestReorg ?? null, report.final_status?.maybeLatestReorgUnavailableReason ?? null)}
- Reconcile progress: ${objectSummaryText(report.final_status?.reconcileProgress ?? null, report.final_status?.maybeReconcileProgressUnavailableReason ?? null)}
- Peer contribution: ${peerContributionText(report.final_status?.peerContribution ?? null, report.final_status?.maybeAttemptCountersUnavailableReason ?? report.final_status?.maybePeerCountsUnavailableReason ?? null)}
- Bounded counters: ${boundedCountersText(report.final_status)}

## Runtime Peer Contributions

| Peer | Source | State | Headers Accepted | Blocks Accepted | Last Activity | Failure Reason | Error |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
${runtimePeerRows}

## Daemon Output Summary

- Exit code: ${report.daemon.maybeExitCode ?? "Unavailable"}
- Signal: ${report.daemon.maybeSignal ?? "Unavailable"}
- stdout observed: ${report.daemon.stdoutObserved ? "yes" : "no"}
- stdout line count: ${report.daemon.stdoutLineCount}
- stderr observed: ${report.daemon.stderrObserved ? "yes" : "no"}
- stderr line count: ${report.daemon.stderrLineCount}
`;
}

function fieldText(value: string | number | null, maybeUnavailableReason: string | null): string {
  if (value !== null) {
    return escapeTableCell(String(value));
  }
  return unavailableText(maybeUnavailableReason);
}

function unavailableText(maybeReason: string | null): string {
  return maybeReason === null || maybeReason.trim() === ""
    ? "Unavailable"
    : `Unavailable: ${escapeTableCell(maybeReason)}`;
}

function configuredTargetsText(
  configuredTargets: ConfiguredTargetsSummary | null,
  maybeUnavailableReason: string | null,
  maybeOutboundPeers: number | null,
  maybeOutboundPeersUnavailableReason: string | null,
): string {
  if (configuredTargets === null) {
    return unavailableText(maybeUnavailableReason);
  }
  const targetHeader =
    configuredTargets.maybeTargetHeaderHeight === null
      ? "Unavailable: no target header configured"
      : String(configuredTargets.maybeTargetHeaderHeight);
  const outboundPeers =
    maybeOutboundPeers === null
      ? `${unavailableText(maybeOutboundPeersUnavailableReason)}/${configuredTargets.targetOutboundPeers}`
      : `${maybeOutboundPeers}/${configuredTargets.targetOutboundPeers}`;
  return `outbound_peers=${outboundPeers} target_header_height=${targetHeader}`;
}

function attemptCountersText(
  attemptCounters: AttemptCountersSummary | null,
  maybeUnavailableReason: string | null,
): string {
  if (attemptCounters === null) {
    return unavailableText(maybeUnavailableReason);
  }
  return `attempted=${attemptCounters.attemptedPeers} connected=${attemptCounters.connectedPeers} failed=${attemptCounters.failedPeers} max_rounds=${attemptCounters.maxSyncRounds}`;
}

function stopReasonText(
  latestStopReason: StopReasonSummary | null,
  maybeUnavailableReason: string | null,
): string {
  if (latestStopReason === null) {
    return unavailableText(maybeUnavailableReason);
  }
  return escapeTableCell(`${latestStopReason.label}: ${latestStopReason.message}`);
}

function resourcePressureText(
  resourcePressure: ResourcePressureSummary | null,
  maybeUnavailableReason: string | null,
): string {
  if (resourcePressure === null) {
    return unavailableText(maybeUnavailableReason);
  }
  return `headers_per_peer=${resourcePressure.maxHeaderRequestsInFlightPerPeer} headers_per_message=${resourcePressure.maxHeadersPerMessage} blocks=${resourcePressure.blocksInFlight}/${resourcePressure.maxBlocksInFlightPerPeer}/${resourcePressure.maxBlocksInFlightTotal} messages_per_peer=${resourcePressure.maxMessagesPerPeer} sync_rounds=${resourcePressure.maxSyncRounds} outbound_peers=${resourcePressure.outboundPeers}/${resourcePressure.targetOutboundPeers}`;
}

function objectSummaryText(
  summary: ObjectSummary | null,
  maybeUnavailableReason: string | null,
): string {
  if (summary === null) {
    return unavailableText(maybeUnavailableReason);
  }
  return Object.entries(summary)
    .map(([key, value]) => `${key}=${value === null ? "unavailable" : escapeTableCell(String(value))}`)
    .join(" ");
}

function peerContributionText(
  peerContribution: PeerContributionSummary | null,
  maybeUnavailableReason: string | null,
): string {
  if (peerContribution === null) {
    return unavailableText(maybeUnavailableReason);
  }
  return `attempted=${fieldText(peerContribution.attempted, maybeUnavailableReason)} connected=${fieldText(peerContribution.connected, maybeUnavailableReason)} failed=${fieldText(peerContribution.failed, maybeUnavailableReason)} outbound=${fieldText(peerContribution.outbound, maybeUnavailableReason)}`;
}

function blockEvidenceText(
  height: number | null,
  maybeHash: string | null,
  kind: "connected" | "downloaded",
  maybeUnavailableReason: string | null,
): string {
  if (height === null) {
    return unavailableText(maybeUnavailableReason);
  }
  const hash = maybeHash ?? `Unavailable: no ${kind} block hash recorded`;
  return `height=${height} hash=${escapeTableCell(hash)}`;
}

function peerHealthText(
  maybeOutboundPeers: number | null,
  maybeUnavailableReason: string | null,
): string {
  return `outbound_peers=${fieldText(maybeOutboundPeers, maybeUnavailableReason)}`;
}

function boundedCountersText(finalStatus: FinalStatusSummary | null): string {
  const maybeUnavailableReason = finalStatus?.maybeSyncProgressUnavailableReason ?? null;
  return [
    `messages_processed=${fieldText(finalStatus?.messagesProcessed ?? null, maybeUnavailableReason)}`,
    `headers_received=${fieldText(finalStatus?.headersReceived ?? null, maybeUnavailableReason)}`,
    `blocks_received=${fieldText(finalStatus?.blocksReceived ?? null, maybeUnavailableReason)}`,
  ].join(" ");
}

function progressTripletText(
  maybeProgress: {
    connectedBlockHeight: number | null;
    downloadedBlockHeight: number | null;
    headerHeight: number | null;
    maybeSyncProgressUnavailableReason: string | null;
  } | null,
): string {
  if (maybeProgress === null) {
    return "Unavailable";
  }

  return [
    fieldText(maybeProgress.headerHeight, maybeProgress.maybeSyncProgressUnavailableReason),
    fieldText(
      maybeProgress.downloadedBlockHeight,
      maybeProgress.maybeSyncProgressUnavailableReason,
    ),
    fieldText(
      maybeProgress.connectedBlockHeight,
      maybeProgress.maybeSyncProgressUnavailableReason,
    ),
  ].join("/");
}

function escapeTableCell(value: string): string {
  return value.replaceAll("|", "\\|").replaceAll("\n", "<br>");
}

function escapeInline(value: string): string {
  return value.replaceAll("`", "\\`").replaceAll("\n", " ");
}

function writeReportFiles(repoRootPath: string, report: SmokeReport): { jsonPath: string; markdownPath: string } {
  const absoluteOutputDir = path.resolve(repoRootPath, report.options.outputDir);
  mkdirSync(absoluteOutputDir, { recursive: true });
  const jsonPath = path.join(absoluteOutputDir, `${REPORT_STEM}.json`);
  const markdownPath = path.join(absoluteOutputDir, `${REPORT_STEM}.md`);

  writeFileSync(jsonPath, `${JSON.stringify(report, null, 2)}\n`);
  writeFileSync(markdownPath, `${markdownReport(report)}\n`);

  return { jsonPath, markdownPath };
}

function preflightFailureReport(
  options: Options,
  checks: PreflightCheck[],
  daemonSpec: CommandSpec,
  statusSpec: CommandSpec,
  endpointOutcomes: EndpointOutcome[],
): SmokeReport {
  const message = checks
    .filter((check) => !check.ok)
    .map((check) => check.detail)
    .join(" ");
  return {
    baseline: BASELINE,
    commands: {
      daemon: [daemonSpec.command, ...daemonSpec.args],
      finalStatus: [],
      status: [statusSpec.command, ...statusSpec.args],
    },
    daemon_sessions: [],
    daemon: {
      maybeExitCode: null,
      maybeSignal: null,
      stderrLineCount: 0,
      stderrObserved: false,
      stdoutLineCount: 0,
      stdoutObserved: false,
    },
    final_status: null,
    generated_at_unix_seconds: Math.floor(Date.now() / 1000),
    kind: "live_mainnet_smoke",
    options: {
      datadir: options.datadir,
      manualPeers: options.manualPeers,
      maybeConfigPath: options.maybeConfigPath,
      maybeGeneratedConfigPath: options.maybeGeneratedConfigPath,
      minFreeGib: options.minFreeGib,
      outputDir: options.outputDir,
      pollSeconds: options.pollSeconds,
      restartAfterProgress: options.restartAfterProgress,
      timeoutSeconds: options.timeoutSeconds,
    },
    network_preflight: {
      completed: false,
      endpoint_outcomes: endpointOutcomes,
    },
    preflight: {
      checks,
      passed: false,
    },
    result: {
      blockDelta: 0,
      firstBlockProgress: null,
      firstHeaderProgress: null,
      headerDelta: 0,
      maybeNoProgressCause: null,
      message,
      nextAction: "Fix the failed local preflight checks, then rerun the live smoke command.",
      progressDetected: false,
      restartResumeEvidence: null,
      status: "preflight_failed",
    },
    schema_version: 2,
    snapshots: [],
  };
}

async function main(): Promise<void> {
  const repoRootPath = repoRoot();
  const options = optionsWithGeneratedManualPeerConfig(
    repoRootPath,
    parseArgs(process.argv.slice(2)),
  );

  const previewRpcPort = await findFreePort();
  const daemonSpec = daemonCommand(repoRootPath, options, previewRpcPort);
  const statusSpec = statusCommandForRpcPort(repoRootPath, options, previewRpcPort);
  const preflightChecks = buildPreflightChecks(
    repoRootPath,
    options,
    process.env.OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN ?? null,
    process.env.OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN ?? null,
  );
  if (preflightChecks.some((check) => !check.ok)) {
    const endpointOutcomes = skippedEndpointOutcomes(
      peerSourcesFromOptions(repoRootPath, options),
      "local preflight failed before network endpoint checks",
    );
    const report = preflightFailureReport(
      options,
      preflightChecks,
      daemonSpec,
      statusSpec,
      endpointOutcomes,
    );
    const { jsonPath, markdownPath } = writeReportFiles(repoRootPath, report);
    console.log(`wrote ${path.relative(repoRootPath, jsonPath)}`);
    console.log(`wrote ${path.relative(repoRootPath, markdownPath)}`);
    throw new Error(report.result.message);
  }

  const preflightEndpointOutcomes = await networkPreflightEndpointOutcomes(
    repoRootPath,
    options,
  );

  ensureBuiltBinaries(repoRootPath);

  const postRunStatusSpec = finalStatusCommand(repoRootPath, options);
  const firstSession = await runSmokeSession(
    repoRootPath,
    options,
    options.restartAfterProgress ? "until_progress" : "normal",
  );
  let maybeRestartSession: SmokeSessionResult | null = null;
  if (
    options.restartAfterProgress &&
    firstSession.resultStatus === "passed" &&
    firstSession.maybeCancellationSignal === null
  ) {
    maybeRestartSession = await runSmokeSession(repoRootPath, options, "first_snapshot");
  }

  let maybeFinalStatus: FinalStatusSummary | null = null;
  try {
    maybeFinalStatus = readFinalStatus(repoRootPath, postRunStatusSpec);
  } catch {
    maybeFinalStatus = null;
  }
  const endpointOutcomes = [
    ...preflightEndpointOutcomes,
    ...endpointOutcomesFromFinalStatus(maybeFinalStatus),
  ];
  const sessions = maybeRestartSession === null
    ? [firstSession]
    : [firstSession, maybeRestartSession];
  const daemonSessions = sessions.map((session) => ({
    daemon: [session.daemonSpec.command, ...session.daemonSpec.args],
    status: [session.statusSpec.command, ...session.statusSpec.args],
  }));
  const snapshots = sessions.flatMap((session) => session.snapshots);
  let resultStatus = firstSession.resultStatus;
  let resultMessage = firstSession.resultMessage;
  let headerDelta = sessions.reduce((sum, session) => sum + session.headerDelta, 0);
  let blockDelta = sessions.reduce((sum, session) => sum + session.blockDelta, 0);
  const maybeLastProbeError =
    maybeRestartSession?.maybeLastProbeError ?? firstSession.maybeLastProbeError;
  let maybeRestartResumeEvidence: RestartResumeEvidence | null = null;
  if (options.restartAfterProgress) {
    maybeRestartResumeEvidence = restartResumeEvidence(
      repoRootPath,
      options,
      firstSession,
      maybeRestartSession,
      endpointOutcomes,
      maybeFinalStatus,
      maybeLastProbeError,
    );
    if (maybeRestartResumeEvidence.restartStatus === "completed") {
      resultStatus = "passed";
      resultMessage =
        "Observed same-datadir restart/resume evidence: a fresh post-restart status snapshot preserved durable header, downloaded block, and connected block progress.";
    } else if (maybeRestartResumeEvidence.restartStatus === "cancelled") {
      resultStatus = "cancelled";
      resultMessage = "live mainnet smoke cancelled before restart evidence was captured.";
    } else if (firstSession.resultStatus === "runtime_failed") {
      resultStatus = "runtime_failed";
      resultMessage = firstSession.resultMessage;
    } else if (maybeRestartSession?.resultStatus === "runtime_failed") {
      resultStatus = "runtime_failed";
      resultMessage = `Post-restart daemon session failed before durable resume evidence was captured: ${maybeRestartSession.resultMessage}`;
    } else if (maybeRestartSession === null) {
      resultStatus = "no_progress";
      resultMessage =
        "No pre-restart progress was observed, so the requested same-datadir restart was blocked before evidence could be captured.";
    } else if (lastSnapshot(maybeRestartSession.snapshots) === null) {
      resultStatus = "runtime_failed";
      resultMessage =
        "Post-restart daemon session did not produce a fresh status snapshot.";
    } else {
      resultStatus = "no_progress";
      resultMessage =
        "Post-restart durable resume evidence did not preserve the expected same-datadir heights and hashes.";
    }
  }
  const maybeFirstHeaderProgress = firstHeaderProgressEvidence(
    firstNonNullProgressSnapshots(
      sessions,
      (session) => session.maybeFirstHeaderProgressSnapshots,
    ),
    maybeFinalStatus,
  );
  const maybeFirstConnectedBlockProgress = firstBlockProgressEvidence(
    firstNonNullProgressSnapshots(
      sessions,
      (session) => session.maybeFirstConnectedBlockProgressSnapshots,
    ),
    maybeFinalStatus,
    "connected",
  );
  const maybeFirstDownloadedBlockProgress = firstBlockProgressEvidence(
    firstNonNullProgressSnapshots(
      sessions,
      (session) => session.maybeFirstDownloadedBlockProgressSnapshots,
    ),
    maybeFinalStatus,
    "downloaded",
  );
  const maybeFirstBlockProgress =
    maybeFirstConnectedBlockProgress ?? maybeFirstDownloadedBlockProgress;

  const noProgressCauseFromEvidence =
    resultStatus === "no_progress" &&
    (maybeFirstBlockProgress !== null || maybeFirstHeaderProgress !== null)
      ? "awaiting_blocks"
      : null;

  if (resultStatus === "no_progress" && maybeRestartResumeEvidence === null) {
    const noProgressCause =
      noProgressCauseFromEvidence ??
      classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError);
    if (maybeFirstDownloadedBlockProgress !== null) {
      resultMessage =
        `Downloaded block progress was observed, but connected block height did not advance before timeout; typed no-progress cause: ${noProgressCause}.`;
    } else if (maybeFirstHeaderProgress !== null) {
      resultMessage =
        `Header progress was observed, but no connected block progress was reached before timeout; typed no-progress cause: ${noProgressCause}.`;
    } else if (maybeFinalStatus?.outboundPeers === 0) {
      resultMessage =
        `No header or block progress was observed before timeout. Final durable sync status still showed 0 outbound peers; typed no-progress cause: ${noProgressCause}.`;
    } else if (maybeLastProbeError !== null) {
      resultMessage = `No header or block progress was observed before timeout. Last RPC probe error: ${maybeLastProbeError}`;
    }
  }

  const maybeNoProgressCause =
    resultStatus === "cancelled"
      ? "operator_cancellation"
      : resultStatus === "no_progress"
        ? noProgressCauseFromEvidence ??
          classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError)
        : resultStatus === "runtime_failed"
          ? noProgressCauseFromFinalStatus(maybeFinalStatus)
          : null;

  const report: SmokeReport = {
    baseline: BASELINE,
    commands: {
      daemon: daemonSessions[0]?.daemon ?? [daemonSpec.command, ...daemonSpec.args],
      finalStatus: [postRunStatusSpec.command, ...postRunStatusSpec.args],
      status: daemonSessions[0]?.status ?? [statusSpec.command, ...statusSpec.args],
    },
    daemon_sessions: daemonSessions,
    daemon: {
      maybeExitCode: (maybeRestartSession ?? firstSession).maybeExitCode,
      maybeSignal: (maybeRestartSession ?? firstSession).maybeSignal,
      stderrLineCount: sessions.reduce(
        (sum, session) => sum + session.stderrLineCount,
        0,
      ),
      stderrObserved: sessions.some((session) => session.stderrObserved),
      stdoutLineCount: sessions.reduce(
        (sum, session) => sum + session.stdoutLineCount,
        0,
      ),
      stdoutObserved: sessions.some((session) => session.stdoutObserved),
    },
    final_status: maybeFinalStatus,
    generated_at_unix_seconds: Math.floor(Date.now() / 1000),
    kind: "live_mainnet_smoke",
    options: {
      datadir: options.datadir,
      manualPeers: options.manualPeers,
      maybeConfigPath: options.maybeConfigPath,
      maybeGeneratedConfigPath: options.maybeGeneratedConfigPath,
      minFreeGib: options.minFreeGib,
      outputDir: options.outputDir,
      pollSeconds: options.pollSeconds,
      restartAfterProgress: options.restartAfterProgress,
      timeoutSeconds: options.timeoutSeconds,
    },
    network_preflight: {
      completed: true,
      endpoint_outcomes: endpointOutcomes,
    },
    preflight: {
      checks: preflightChecks,
      passed: true,
    },
    result: {
      blockDelta,
      firstBlockProgress: maybeFirstBlockProgress,
      firstHeaderProgress: maybeFirstHeaderProgress,
      headerDelta,
      maybeNoProgressCause,
      message: resultMessage,
      nextAction: nextActionForCause(maybeNoProgressCause),
      progressDetected: resultStatus === "passed",
      restartResumeEvidence: maybeRestartResumeEvidence,
      status: resultStatus,
    },
    schema_version: 2,
    snapshots,
  };

  const { jsonPath, markdownPath } = writeReportFiles(repoRootPath, report);
  console.log(`wrote ${path.relative(repoRootPath, jsonPath)}`);
  console.log(`wrote ${path.relative(repoRootPath, markdownPath)}`);

  if (resultStatus !== "passed") {
    throw new Error(resultMessage);
  }
}

main().catch((error: unknown) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(message);
  process.exit(1);
});

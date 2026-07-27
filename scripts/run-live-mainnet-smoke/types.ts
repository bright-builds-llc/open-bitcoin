export type Options = {
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
export type PeerAddress = {
  address: string;
  host: string;
  port: number;
};
export type PreflightCheck = {
  detail: string;
  name: string;
  ok: boolean;
};
export type CommandSpec = {
  args: string[];
  command: string;
};
export type SyncStatusSnapshot = {
  blockHeight: number | null;
  attemptCounters: AttemptCountersSummary | null;
  capturedAtUnixSeconds: number;
  configuredTargets: ConfiguredTargetsSummary | null;
  connectedBlockHeight: number | null;
  downloadedBlockHeight: number | null;
  expectedProgressWindowSeconds: number | null;
  headerHeight: number | null;
  lastPeerContribution: ObjectSummary | null;
  lastUsefulWorkHeight: number | null;
  lastUsefulWorkKind: string | null;
  lifecycle: string;
  maybeConnectedBlockHash: string | null;
  maybeDownloadedBlockHash: string | null;
  maybeAttemptCountersUnavailableReason: string | null;
  maybeConfiguredTargetsUnavailableReason: string | null;
  maybeExpectedProgressWindowUnavailableReason: string | null;
  maybeLastSuccessfulProgressUnixSeconds: number | null;
  maybeLastError: string | null;
  maybeLastErrorUnavailableReason: string | null;
  maybeLastPeerContributionUnavailableReason: string | null;
  maybeLastUsefulWorkUnavailableReason: string | null;
  maybeLatestStopReasonUnavailableReason: string | null;
  maybeNoProgressThresholdUnavailableReason: string | null;
  maybePeerCountsUnavailableReason: string | null;
  maybeProgressCreditUnavailableReason: string | null;
  maybeProgressSignalUnavailableReason: string | null;
  maybeRecoveryActionUnavailableReason: string | null;
  maybeRecoveryCategoryUnavailableReason: string | null;
  maybeResourcePressureUnavailableReason: string | null;
  maybeStallDiagnosisUnavailableReason: string | null;
  maybeSyncProgressUnavailableReason: string | null;
  noProgressThresholdSeconds: number | null;
  noProgressThresholdState: string | null;
  outboundPeers: number | null;
  paused: boolean;
  phase: string;
  progressCredit: ObjectSummary | null;
  progressCreditHeight: number | null;
  progressCreditKind: string | null;
  progressCreditSourceUnixSeconds: number | null;
  progressSignal: string | null;
  stallConfidence: string | null;
  stallEvidenceBasis: string[];
  stallNextAction: string | null;
  stalledSubsystem: string | null;
  latestStopReason: StopReasonSummary | null;
  recoveryAction: string | null;
  recoveryCategory: RecoveryDiagnosisCategory | null;
  resourcePressure: ResourcePressureSummary | null;
  updatedAtUnixSeconds: number;
};
export type FirstHeaderProgressEvidence = {
  after: SyncStatusSnapshot;
  before: SyncStatusSnapshot;
  headerDelta: number;
  maybeLastActivityUnixSeconds: number | null;
  maybePeer: string | null;
  maybeResolvedEndpoint: string | null;
  maybeSource: string | null;
  observedAtUnixSeconds: number;
};
export type FirstBlockProgressKind = "downloaded" | "connected";
export type FirstBlockProgressEvidence = {
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
export type EndpointOutcomeState = "resolved" | "connected" | "handshook" | "failed" | "skipped";
export type EndpointOutcomeStage = "preflight" | "runtime";
export type EndpointOutcomeSource = "manual_peer" | "dns_seed" | "configured_peer" | "unknown";
export type NoProgressCause =
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
export type EndpointOutcome = {
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
export type ReportStatus = "passed" | "preflight_failed" | "runtime_failed" | "no_progress" | "cancelled";
export type RestartStatus = "not_requested" | "completed" | "blocked_before_restart" | "cancelled";
export type DuplicateConnectVerdict = "no_duplicate_connect_observed" | "duplicate_connect_suspected" | "unavailable";
export type RestartProgressSummary = {
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
export type RestartProgressDelta = {
  connectedBlockDelta: number;
  downloadedBlockDelta: number;
  headerDelta: number;
};
export type RestartPeerOutcomeSummary = {
  connected: number;
  failed: number;
  failureCauses: NoProgressCause[];
  handshook: number;
  skipped: number;
};
export type RecoveryDiagnosisCategory =
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
export type RecoveryDiagnosis = {
  category: RecoveryDiagnosisCategory;
  maybeLastError: string | null;
  maybeNoProgressCause: NoProgressCause | null;
  maybePeerFailureReason: string | null;
  maybeStorageRecoveryAction: string | null;
};
export type RecoveryEvidenceStatusJson = {
  action_class?: string;
  category?: string;
  cause?: string;
  compatibility_action?: FieldAvailability<string>;
  evidence_basis?: unknown[];
  maybe_affected_namespace?: string | null;
  maybe_affected_path?: string | null;
  next_action?: string;
};
export type RecoveryEvidenceSummary = {
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
export type ResourcePressureSummary = {
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
export type ConfiguredTargetsSummary = {
  maybeTargetHeaderHeight: number | null;
  targetOutboundPeers: number;
};
export type AttemptCountersSummary = {
  attemptedPeers: number;
  connectedPeers: number;
  failedPeers: number;
  maxSyncRounds: number;
};
export type StopReasonSummary = {
  label: string;
  message: string;
};
export type ObjectSummary = Record<string, string | number | boolean | null>;
export type PeerContributionSummary = {
  attempted: number | null;
  connected: number | null;
  failed: number | null;
  outbound: number | null;
};
export type RestartResumeEvidence = {
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
export type FinalStatusSummary = {
  attemptCounters: AttemptCountersSummary | null;
  bestKnownTip: ObjectSummary | null;
  blockHeight: number | null;
  configuredTargets: ConfiguredTargetsSummary | null;
  connectedBlockHeight: number | null;
  downloadedBlockHeight: number | null;
  expectedProgressWindowSeconds: number | null;
  headerHeight: number | null;
  latestStopReason: StopReasonSummary | null;
  latestReorg: ObjectSummary | null;
  lastPeerContribution: ObjectSummary | null;
  lastUsefulWorkHeight: number | null;
  lastUsefulWorkKind: string | null;
  maybeAttemptCountersUnavailableReason: string | null;
  maybeBestKnownTipUnavailableReason: string | null;
  maybeConfiguredTargetsUnavailableReason: string | null;
  lifecycle: string;
  maybeConnectedBlockHash: string | null;
  maybeDownloadedBlockHash: string | null;
  maybeExpectedProgressWindowUnavailableReason: string | null;
  maybeLastSuccessfulProgressUnixSeconds: number | null;
  maybeLastError: string | null;
  maybeLastErrorUnavailableReason: string | null;
  maybeLastPeerContributionUnavailableReason: string | null;
  maybeLastUsefulWorkUnavailableReason: string | null;
  maybeLatestStopReasonUnavailableReason: string | null;
  maybeLatestReorgUnavailableReason: string | null;
  maybeNoProgressDiagnosisUnavailableReason: string | null;
  maybeNoProgressNextActionUnavailableReason: string | null;
  maybeNoProgressThresholdUnavailableReason: string | null;
  maybePeerCountsUnavailableReason: string | null;
  maybeProgressCreditUnavailableReason: string | null;
  maybeProgressSignalUnavailableReason: string | null;
  maybeReconcileProgressUnavailableReason: string | null;
  maybeRecoveryActionUnavailableReason: string | null;
  maybeRecoveryCategoryUnavailableReason: string | null;
  maybeRecoveryEvidenceUnavailableReason: string | null;
  maybeResourcePressureUnavailableReason: string | null;
  maybeStallDiagnosisUnavailableReason: string | null;
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
  noProgressThresholdSeconds: number | null;
  noProgressThresholdState: string | null;
  outboundPeers: number | null;
  peerContribution: PeerContributionSummary | null;
  phase: string;
  progressCredit: ObjectSummary | null;
  progressCreditHeight: number | null;
  progressCreditKind: string | null;
  progressCreditSourceUnixSeconds: number | null;
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
  stallConfidence: string | null;
  stallEvidenceBasis: string[];
  stallNextAction: string | null;
  stalledSubsystem: string | null;
  stayCurrent: string | null;
  stayCurrentNextAction: string | null;
  validatedActiveChainHeight: number | null;
};
export type SmokeReport = {
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
export type SyncControlStatusJson = {
  metadata?: RuntimeMetadataJson;
};
export type RuntimeMetadataJson = {
  maybe_sync_state?: DurableSyncStateJson;
  recovery_evidence?: FieldAvailability<RecoveryEvidenceStatusJson>;
  sync_control?: {
    paused?: boolean;
  };
};
export type FieldAvailability<T> =
  | {
    state: "available";
    value: T;
  }
  | {
    reason?: string;
    state: string;
    value?: unknown;
  };
export type ResourcePressureStatusJson = {
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
export type ConfiguredTargetsStatusJson = {
  maybe_target_header_height?: number | null;
  target_outbound_peers?: number;
};
export type AttemptCountersStatusJson = {
  attempted_peers?: number;
  connected_peers?: number;
  failed_peers?: number;
  max_sync_rounds?: number;
};
export type StopReasonStatusJson = {
  label?: string;
  message?: string;
};
export type DurableSyncStateJson = {
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
export type RuntimePeerTelemetryJson = {
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
export type RuntimePeerTelemetry = {
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
export type SmokeSessionMode = "normal" | "until_progress" | "first_snapshot";
export type SmokeSessionResult = {
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

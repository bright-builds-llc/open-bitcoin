import { bestKnownTipSummaryFromValue, latestReorgSummaryFromValue, peerContributionEvidenceSummaryFromValue, peerContributionFromValues, progressCreditSummaryFromValue, reconcileProgressSummaryFromValue, recoveryEvidenceSummaryFromAvailability, recoveryEvidenceUnavailableReason } from "./report";
import type {
  AttemptCountersStatusJson,
  AttemptCountersSummary,
  ConfiguredTargetsStatusJson,
  ConfiguredTargetsSummary,
  FieldAvailability,
  FinalStatusSummary,
  ObjectSummary,
  RecoveryDiagnosisCategory,
  ResourcePressureStatusJson,
  ResourcePressureSummary,
  RuntimeMetadataJson,
  RuntimePeerTelemetry,
  RuntimePeerTelemetryJson,
  StopReasonStatusJson,
  StopReasonSummary,
  SyncControlStatusJson,
  SyncStatusSnapshot,
} from "./types";

export function runtimeMetadataFromStatusResponse(
  decoded: SyncControlStatusJson | RuntimeMetadataJson,
): RuntimeMetadataJson {
  const maybeMetadata = (decoded as SyncControlStatusJson).metadata;
  if (maybeMetadata !== undefined) {
    return {
      ...maybeMetadata,
      recovery_evidence: (decoded as RuntimeMetadataJson).recovery_evidence ??
        maybeMetadata.recovery_evidence,
    };
  }

  return decoded as RuntimeMetadataJson;
}

export function syncStatusSnapshotFromMetadata(metadata: RuntimeMetadataJson): SyncStatusSnapshot {
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
  const maybeProgressCredit = availableValue(maybeSync?.progress_credit);
  const progressCreditRecord = recordFromValue(maybeProgressCredit);
  const maybeExpectedProgressWindow = availableValue(
    maybeSync?.expected_progress_window,
  );
  const expectedProgressWindowRecord = recordFromValue(maybeExpectedProgressWindow);
  const maybeNoProgressThreshold = availableValue(maybeSync?.no_progress_threshold);
  const noProgressThresholdRecord = recordFromValue(maybeNoProgressThreshold);
  const maybeLastUsefulWork = availableValue(maybeSync?.last_useful_work);
  const lastUsefulWorkRecord = recordFromValue(maybeLastUsefulWork);
  const maybeLastPeerContribution = availableValue(maybeSync?.last_peer_contribution);
  const maybeStallDiagnosis = availableValue(maybeSync?.stall_diagnosis);
  const stallDiagnosisRecord = recordFromValue(maybeStallDiagnosis);
  const blockHeight = maybeProgress === null ? null : Number(maybeProgress.block_height ?? 0);
  const connectedBlockHeight = maybeProgress === null ? null : Number(maybeProgress.connected_block_height ?? blockHeight ?? 0);
  const downloadedBlockHeight = maybeProgress === null ? null : Number(maybeProgress.downloaded_block_height ?? connectedBlockHeight);

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
    expectedProgressWindowSeconds: numberOrNull(
      expectedProgressWindowRecord?.expected_progress_window_seconds,
    ),
    headerHeight: maybeProgress === null ? null : Number(maybeProgress.header_height ?? 0),
    lastPeerContribution: peerContributionEvidenceSummaryFromValue(
      maybeLastPeerContribution,
    ),
    lastUsefulWorkHeight: numberOrNull(
      lastUsefulWorkRecord?.credited_validated_active_chain_height,
    ),
    lastUsefulWorkKind: valueAsNullableString(lastUsefulWorkRecord?.kind),
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
    maybeExpectedProgressWindowUnavailableReason: unavailableReasonFromFieldAvailability(maybeSync?.expected_progress_window),
    maybeLastSuccessfulProgressUnixSeconds: availableValue(
      maybeSync?.last_successful_progress_unix_seconds,
    ),
    maybeLastError: valueAsNullableString(availableValue(maybeSync?.last_error)),
    maybeLastErrorUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_error,
    ),
    maybeLastPeerContributionUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_peer_contribution,
    ),
    maybeLastUsefulWorkUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_useful_work,
    ),
    maybeLatestStopReasonUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.latest_stop_reason,
    ),
    maybeNoProgressThresholdUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.no_progress_threshold,
    ),
    maybePeerCountsUnavailableReason,
    maybeProgressCreditUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.progress_credit,
    ),
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
    maybeStallDiagnosisUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.stall_diagnosis,
    ),
    maybeSyncProgressUnavailableReason,
    noProgressThresholdSeconds: numberOrNull(
      noProgressThresholdRecord?.threshold_seconds,
    ),
    noProgressThresholdState: valueAsNullableString(noProgressThresholdRecord?.state),
    outboundPeers: maybePeerCounts === null ? null : Number(maybePeerCounts.outbound ?? 0),
    paused: metadata.sync_control?.paused === true,
    phase: String(availableValue(maybeSync?.phase) ?? "unavailable"),
    progressCredit: progressCreditSummaryFromValue(maybeProgressCredit),
    progressCreditHeight: numberOrNull(
      progressCreditRecord?.credited_validated_active_chain_height,
    ),
    progressCreditKind: valueAsNullableString(progressCreditRecord?.kind),
    progressCreditSourceUnixSeconds: numberOrNull(
      progressCreditRecord?.source_unix_seconds,
    ),
    progressSignal: valueAsNullableString(availableValue(maybeSync?.progress_signal)),
    stallConfidence: valueAsNullableString(stallDiagnosisRecord?.confidence),
    stallEvidenceBasis: stringArrayFromValue(stallDiagnosisRecord?.evidence_basis),
    stallNextAction: valueAsNullableString(stallDiagnosisRecord?.next_action),
    stalledSubsystem: valueAsNullableString(stallDiagnosisRecord?.stalled_subsystem),
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
    maybeTargetHeaderHeight: typeof value.maybe_target_header_height === "number" ? value.maybe_target_header_height : null,
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

function recordFromValue(value: unknown): Record<string, unknown> | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return null;
  }
  return value as Record<string, unknown>;
}

function numberOrNull(value: unknown): number | null {
  return typeof value === "number" ? value : null;
}

function stringArrayFromValue(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.map((entry) => String(entry));
}

export function finalStatusSummaryFromMetadata(metadata: RuntimeMetadataJson): FinalStatusSummary | null {
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
  const maybeProgressCredit = availableValue(maybeSync?.progress_credit);
  const progressCreditRecord = recordFromValue(maybeProgressCredit);
  const maybeExpectedProgressWindow = availableValue(
    maybeSync?.expected_progress_window,
  );
  const expectedProgressWindowRecord = recordFromValue(maybeExpectedProgressWindow);
  const maybeNoProgressThreshold = availableValue(maybeSync?.no_progress_threshold);
  const noProgressThresholdRecord = recordFromValue(maybeNoProgressThreshold);
  const maybeLastUsefulWork = availableValue(maybeSync?.last_useful_work);
  const lastUsefulWorkRecord = recordFromValue(maybeLastUsefulWork);
  const maybeLastPeerContribution = availableValue(maybeSync?.last_peer_contribution);
  const maybeStallDiagnosis = availableValue(maybeSync?.stall_diagnosis);
  const stallDiagnosisRecord = recordFromValue(maybeStallDiagnosis);
  const blockHeight = maybeProgress === null ? null : Number(maybeProgress.block_height ?? 0);
  const connectedBlockHeight = maybeProgress === null ? null : Number(maybeProgress.connected_block_height ?? blockHeight ?? 0);
  const downloadedBlockHeight = maybeProgress === null ? null : Number(maybeProgress.downloaded_block_height ?? connectedBlockHeight);
  const maybeValidatedActiveChainHeight = maybeProgress === null || typeof maybeProgress.validated_active_chain_height !== "number" ? null : Number(maybeProgress.validated_active_chain_height);
  const maybeValidatedActiveChainHeightUnavailableReason = maybeProgress === null ? maybeSyncProgressUnavailableReason : typeof maybeProgress.validated_active_chain_height === "number" ? null : "validated active-chain height unavailable";
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
    expectedProgressWindowSeconds: numberOrNull(
      expectedProgressWindowRecord?.expected_progress_window_seconds,
    ),
    headerHeight: maybeProgress === null ? null : Number(maybeProgress.header_height ?? 0),
    latestStopReason: stopReasonFromValue(
      availableValue(maybeSync?.latest_stop_reason),
    ),
    latestReorg: latestReorgSummaryFromValue(availableValue(maybeSync?.latest_reorg)),
    lastPeerContribution: peerContributionEvidenceSummaryFromValue(
      maybeLastPeerContribution,
    ),
    lastUsefulWorkHeight: numberOrNull(
      lastUsefulWorkRecord?.credited_validated_active_chain_height,
    ),
    lastUsefulWorkKind: valueAsNullableString(lastUsefulWorkRecord?.kind),
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
    maybeExpectedProgressWindowUnavailableReason: unavailableReasonFromFieldAvailability(maybeSync?.expected_progress_window),
    maybeLastSuccessfulProgressUnixSeconds: availableValue(
      maybeSync?.last_successful_progress_unix_seconds,
    ),
    maybeLastError: valueAsNullableString(availableValue(maybeSync?.last_error)),
    maybeLastErrorUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_error,
    ),
    maybeLastPeerContributionUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_peer_contribution,
    ),
    maybeLastUsefulWorkUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.last_useful_work,
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
    maybeNoProgressThresholdUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.no_progress_threshold,
    ),
    maybePeerCountsUnavailableReason,
    maybeProgressCreditUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.progress_credit,
    ),
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
    maybeStallDiagnosisUnavailableReason: unavailableReasonFromFieldAvailability(
      maybeSync?.stall_diagnosis,
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
    headersReceived: maybeProgress === null ? null : Number(maybeProgress.headers_received ?? 0),
    blocksReceived: maybeProgress === null ? null : Number(maybeProgress.blocks_received ?? 0),
    messagesProcessed: maybeProgress === null ? null : Number(maybeProgress.messages_processed ?? 0),
    noProgressDiagnosis: valueAsNullableString(
      availableValue(maybeSync?.no_progress_diagnosis),
    ),
    noProgressNextAction: valueAsNullableString(
      availableValue(maybeSync?.no_progress_next_action),
    ),
    noProgressThresholdSeconds: numberOrNull(
      noProgressThresholdRecord?.threshold_seconds,
    ),
    noProgressThresholdState: valueAsNullableString(noProgressThresholdRecord?.state),
    outboundPeers: maybePeerCounts === null ? null : Number(maybePeerCounts.outbound ?? 0),
    peerContribution: peerContributionFromValues(attemptCounters, maybePeerCounts),
    phase: String(availableValue(maybeSync?.phase) ?? "unavailable"),
    progressCredit: progressCreditSummaryFromValue(maybeProgressCredit),
    progressCreditHeight: numberOrNull(
      progressCreditRecord?.credited_validated_active_chain_height,
    ),
    progressCreditKind: valueAsNullableString(progressCreditRecord?.kind),
    progressCreditSourceUnixSeconds: numberOrNull(
      progressCreditRecord?.source_unix_seconds,
    ),
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
    stallConfidence: valueAsNullableString(stallDiagnosisRecord?.confidence),
    stallEvidenceBasis: stringArrayFromValue(stallDiagnosisRecord?.evidence_basis),
    stallNextAction: valueAsNullableString(stallDiagnosisRecord?.next_action),
    stalledSubsystem: valueAsNullableString(stallDiagnosisRecord?.stalled_subsystem),
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

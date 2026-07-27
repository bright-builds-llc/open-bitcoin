import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import type { AttemptCountersSummary, ConfiguredTargetsSummary, FieldAvailability, FinalStatusSummary, ObjectSummary, PeerContributionSummary, RecoveryEvidenceStatusJson, RecoveryEvidenceSummary, ResourcePressureSummary, SmokeReport, StopReasonSummary } from "./types";

export const BASELINE = "Bitcoin Knots 29.3.knots20260210";
const REPORT_STEM = "open-bitcoin-live-mainnet-smoke";

function recordFromValue(value: unknown): Record<string, unknown> | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}
function numberOrNull(value: unknown): number | null {
  return typeof value === "number" ? value : null;
}
function stringArrayFromValue(value: unknown): string[] {
  return Array.isArray(value) ? value.map((entry) => String(entry)) : [];
}
function valueAsNullableString(value: unknown): string | null {
  return typeof value === "string" && value.trim() !== "" ? value : null;
}
function availableValue<T>(value: FieldAvailability<T> | undefined): T | null {
  return value !== undefined && value.state === "available" ? value.value : null;
}
function unavailableReasonFromFieldAvailability<T>(
  value: FieldAvailability<T> | undefined,
): string | null {
  if (value === undefined) return "status field absent";
  if (value.state === "available") return null;
  if (typeof value.value === "object" && value.value !== null && "reason" in value.value && typeof value.value.reason === "string" && value.value.reason.trim() !== "") return value.value.reason;
  if ("reason" in value && typeof value.reason === "string" && value.reason.trim() !== "") return value.reason;
  return "status field unavailable";
}

export function bestKnownTipSummaryFromValue(value: unknown): ObjectSummary | null {
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

export function latestReorgSummaryFromValue(value: unknown): ObjectSummary | null {
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
    fullyPersisted: typeof object.fully_persisted === "boolean" ? object.fully_persisted : null,
  };
}

export function reconcileProgressSummaryFromValue(value: unknown): ObjectSummary | null {
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

export function progressCreditSummaryFromValue(value: unknown): ObjectSummary | null {
  const object = recordFromValue(value);
  if (object === null) {
    return null;
  }
  return {
    kind: valueAsNullableString(object.kind),
    height: numberOrNull(object.credited_validated_active_chain_height),
    hash: valueAsNullableString(object.credited_validated_active_chain_hash),
    work: valueAsNullableString(object.credited_validated_active_chain_work),
    sourceUnixSeconds: numberOrNull(object.source_unix_seconds),
    rejectedActivityCount: Array.isArray(object.rejected_activity) ? object.rejected_activity.length : null,
  };
}

export function peerContributionEvidenceSummaryFromValue(value: unknown): ObjectSummary | null {
  const object = recordFromValue(value);
  if (object === null) {
    return null;
  }
  return {
    peer: valueAsNullableString(object.peer),
    endpoint: valueAsNullableString(object.maybe_resolved_endpoint),
    kind: valueAsNullableString(object.kind),
    messagesProcessed: numberOrNull(object.messages_processed),
    headersReceived: numberOrNull(object.headers_received),
    blocksReceived: numberOrNull(object.blocks_received),
    lastActivityUnixSeconds: numberOrNull(object.maybe_last_activity_unix_seconds),
    failureReason: valueAsNullableString(object.maybe_failure_reason_label),
  };
}

export function recoveryEvidenceSummaryFromAvailability(
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
    evidenceBasis: Array.isArray(maybeEvidence.evidence_basis) ? maybeEvidence.evidence_basis.map((basis) => String(basis)) : [],
    maybeUnavailableReason: null,
    nextAction: valueAsNullableString(maybeEvidence.next_action),
    source: "status.recovery_evidence",
    state: "available",
  };
}

export function recoveryEvidenceUnavailableReason(
  value: FieldAvailability<RecoveryEvidenceStatusJson> | undefined,
): string | null {
  if (value === undefined) {
    return "recovery evidence unavailable";
  }
  return unavailableReasonFromFieldAvailability(value);
}

export function peerContributionFromValues(
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
    outbound: maybePeerCounts === null ? null : Number(maybePeerCounts.outbound ?? 0),
  };
}

export function markdownReport(report: SmokeReport): string {
  const preflightRows = report.preflight.checks
    .map(
      (check) => `| ${check.name} | ${check.ok ? "passed" : "failed"} | ${escapeTableCell(check.detail)} |`,
    )
    .join("\n");
  const snapshotRows = report.snapshots.length === 0 ? "| Unavailable: no sync status snapshots captured | - | - | - | - | - | - | - | - | - | - | - | - |\n" : report.snapshots
    .map(
      (snapshot) =>
        `| ${snapshot.capturedAtUnixSeconds} | ${snapshot.lifecycle} | ${snapshot.phase} | ${fieldText(snapshot.progressSignal, snapshot.maybeProgressSignalUnavailableReason)} | ${configuredTargetsText(snapshot.configuredTargets, snapshot.maybeConfiguredTargetsUnavailableReason, snapshot.outboundPeers, snapshot.maybePeerCountsUnavailableReason)} | ${
          attemptCountersText(snapshot.attemptCounters, snapshot.maybeAttemptCountersUnavailableReason)
        } | ${fieldText(snapshot.headerHeight, snapshot.maybeSyncProgressUnavailableReason)} | ${blockEvidenceText(snapshot.downloadedBlockHeight, snapshot.maybeDownloadedBlockHash, "downloaded", snapshot.maybeSyncProgressUnavailableReason)} | ${blockEvidenceText(snapshot.connectedBlockHeight, snapshot.maybeConnectedBlockHash, "connected", snapshot.maybeSyncProgressUnavailableReason)} | ${
          fieldText(snapshot.recoveryCategory, snapshot.maybeRecoveryCategoryUnavailableReason)
        } | ${resourcePressureText(snapshot.resourcePressure, snapshot.maybeResourcePressureUnavailableReason)} | ${stopReasonText(snapshot.latestStopReason, snapshot.maybeLatestStopReasonUnavailableReason)} | ${fieldText(snapshot.maybeLastError, snapshot.maybeLastErrorUnavailableReason)} |`,
    )
    .join("\n");
  const endpointRows = report.network_preflight.endpoint_outcomes.length === 0 ? "| - | - | - | - | - | - | - | - |\n" : report.network_preflight.endpoint_outcomes
    .map(
      (outcome) => `| ${outcome.stage} | ${outcome.source} | ${escapeTableCell(outcome.address)} | ${outcome.state} | ${escapeTableCell(outcome.maybeResolvedEndpoint ?? "-")} | ${outcome.maybeFailureCause ?? "-"} | ${escapeTableCell(outcome.maybeError ?? "-")} | ${outcome.attemptedAtUnixSeconds} |`,
    )
    .join("\n");
  const runtimePeerRows = report.final_status?.recentPeers.length === 0 || report.final_status === null ? "| - | - | - | - | - | - | - | - |\n" : report.final_status.recentPeers
    .map(
      (peer) => `| ${escapeTableCell(peer.peer)} | ${peer.source} | ${peer.state} | ${peer.headersReceived} | ${peer.blocksReceived} | ${peer.maybeLastActivityUnixSeconds ?? "-"} | ${escapeTableCell(peer.maybeFailureReason ?? "-")} | ${escapeTableCell(peer.maybeError ?? "-")} |`,
    )
    .join("\n");
  const daemonSessionRows = report.daemon_sessions.length === 0 ? "| - | - | - |\n" : report.daemon_sessions
    .map(
      (session, index) => `| ${index + 1} | ${escapeTableCell(session.daemon.join(" "))} | ${escapeTableCell(session.status.join(" "))} |`,
    )
    .join("\n");
  const firstHeaderProgress = report.result.firstHeaderProgress;
  const firstHeaderProgressDetail = firstHeaderProgress === null
    ? "Unavailable"
    : `observed at ${firstHeaderProgress.observedAtUnixSeconds}: ${fieldText(firstHeaderProgress.before.headerHeight, firstHeaderProgress.before.maybeSyncProgressUnavailableReason)} -> ${fieldText(firstHeaderProgress.after.headerHeight, firstHeaderProgress.after.maybeSyncProgressUnavailableReason)} via ${escapeInline(firstHeaderProgress.maybePeer ?? "unknown peer")} (${
      firstHeaderProgress.maybeSource ?? "unknown source"
    }, endpoint ${escapeInline(firstHeaderProgress.maybeResolvedEndpoint ?? "unavailable")})`;
  const firstBlockProgress = report.result.firstBlockProgress;
  const firstBlockProgressDetail = firstBlockProgress === null
    ? "Unavailable"
    : `${firstBlockProgress.kind} observed at ${firstBlockProgress.observedAtUnixSeconds}: height ${fieldText(firstBlockProgress.height, firstBlockProgress.after.maybeSyncProgressUnavailableReason)}, block hash ${escapeInline(firstBlockProgress.blockHash ?? "unavailable")}, downloaded ${
      fieldText(firstBlockProgress.before.downloadedBlockHeight, firstBlockProgress.before.maybeSyncProgressUnavailableReason)
    } -> ${fieldText(firstBlockProgress.after.downloadedBlockHeight, firstBlockProgress.after.maybeSyncProgressUnavailableReason)}, connected ${fieldText(firstBlockProgress.before.connectedBlockHeight, firstBlockProgress.before.maybeSyncProgressUnavailableReason)} -> ${fieldText(firstBlockProgress.after.connectedBlockHeight, firstBlockProgress.after.maybeSyncProgressUnavailableReason)}, peer ${
      escapeInline(firstBlockProgress.maybePeer ?? "unknown peer")
    } (${firstBlockProgress.maybeSource ?? "unknown source"}, endpoint ${escapeInline(firstBlockProgress.maybeResolvedEndpoint ?? "unavailable")})`;
  const restartEvidence = report.result.restartResumeEvidence;
  const restartEvidenceDetail = restartEvidence === null
    ? "Unavailable"
    : `status ${restartEvidence.restartStatus}, same datadir requested=${restartEvidence.sameDatadir.requestedPathMatched ? "yes" : "no"} resolved=${restartEvidence.sameDatadir.resolvedPathMatched ? "yes" : "no"}, before header/downloaded/connected ${progressTripletText(restartEvidence.beforeRestart)}, after header/downloaded/connected ${
      progressTripletText(restartEvidence.afterRestart)
    }, duplicate verdict ${restartEvidence.duplicateConnectVerdict}`;

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
- Configured targets: ${
    configuredTargetsText(
      report.final_status?.configuredTargets ?? null,
      report.final_status?.maybeConfiguredTargetsUnavailableReason ?? null,
      report.final_status?.outboundPeers ?? null,
      report.final_status?.maybePeerCountsUnavailableReason ?? null,
    )
  }
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
- Validated active-chain height: ${
    fieldText(
      report.final_status?.validatedActiveChainHeight ?? null,
      report.final_status?.maybeValidatedActiveChainHeightUnavailableReason ?? report.final_status?.maybeSyncProgressUnavailableReason ?? null,
    )
  }
- Validated active-chain hash: ${report.final_status?.maybeValidatedActiveChainHash ?? "Unavailable: validated active-chain hash unavailable"}
- Validated active-chain work: ${report.final_status?.maybeValidatedActiveChainWork ?? "Unavailable: validated active-chain work unavailable"}
- Best-known tip: ${objectSummaryText(report.final_status?.bestKnownTip ?? null, report.final_status?.maybeBestKnownTipUnavailableReason ?? null)}
- Stay-current: ${fieldText(report.final_status?.stayCurrent ?? null, report.final_status?.maybeStayCurrentUnavailableReason ?? null)}
- Stay-current action: ${fieldText(report.final_status?.stayCurrentNextAction ?? null, report.final_status?.maybeStayCurrentNextActionUnavailableReason ?? null)}
- No-progress diagnosis: ${fieldText(report.final_status?.noProgressDiagnosis ?? null, report.final_status?.maybeNoProgressDiagnosisUnavailableReason ?? null)}
- No-progress action: ${fieldText(report.final_status?.noProgressNextAction ?? null, report.final_status?.maybeNoProgressNextActionUnavailableReason ?? null)}
- Progress credit: ${objectSummaryText(report.final_status?.progressCredit ?? null, report.final_status?.maybeProgressCreditUnavailableReason ?? null)}
- Expected progress window: ${fieldText(report.final_status?.expectedProgressWindowSeconds ?? null, report.final_status?.maybeExpectedProgressWindowUnavailableReason ?? null)}
- No-progress threshold: ${noProgressThresholdText(report.final_status)}
- Last useful work: ${lastUsefulWorkText(report.final_status)}
- Last peer contribution: ${objectSummaryText(report.final_status?.lastPeerContribution ?? null, report.final_status?.maybeLastPeerContributionUnavailableReason ?? null)}
- Stalled subsystem: ${stallDiagnosisText(report.final_status)}
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
  return maybeReason === null || maybeReason.trim() === "" ? "Unavailable" : `Unavailable: ${escapeTableCell(maybeReason)}`;
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
  const targetHeader = configuredTargets.maybeTargetHeaderHeight === null ? "Unavailable: no target header configured" : String(configuredTargets.maybeTargetHeaderHeight);
  const outboundPeers = maybeOutboundPeers === null ? `${unavailableText(maybeOutboundPeersUnavailableReason)}/${configuredTargets.targetOutboundPeers}` : `${maybeOutboundPeers}/${configuredTargets.targetOutboundPeers}`;
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

function noProgressThresholdText(finalStatus: FinalStatusSummary | null): string {
  if (finalStatus === null || finalStatus.noProgressThresholdState === null) {
    return unavailableText(finalStatus?.maybeNoProgressThresholdUnavailableReason ?? null);
  }
  return `state=${escapeTableCell(finalStatus.noProgressThresholdState)} seconds=${fieldText(finalStatus.noProgressThresholdSeconds, finalStatus.maybeNoProgressThresholdUnavailableReason)}`;
}

function lastUsefulWorkText(finalStatus: FinalStatusSummary | null): string {
  if (finalStatus === null || finalStatus.lastUsefulWorkKind === null) {
    return unavailableText(finalStatus?.maybeLastUsefulWorkUnavailableReason ?? null);
  }
  return `kind=${escapeTableCell(finalStatus.lastUsefulWorkKind)} height=${fieldText(finalStatus.lastUsefulWorkHeight, finalStatus.maybeLastUsefulWorkUnavailableReason)}`;
}

function stallDiagnosisText(finalStatus: FinalStatusSummary | null): string {
  if (finalStatus === null || finalStatus.stalledSubsystem === null) {
    return unavailableText(finalStatus?.maybeStallDiagnosisUnavailableReason ?? null);
  }
  const basis = finalStatus.stallEvidenceBasis.length === 0 ? "none" : finalStatus.stallEvidenceBasis.map(escapeTableCell).join(",");
  return `stalled_subsystem=${escapeTableCell(finalStatus.stalledSubsystem)} confidence=${fieldText(finalStatus.stallConfidence, finalStatus.maybeStallDiagnosisUnavailableReason)} basis=${basis} next_action=${fieldText(finalStatus.stallNextAction, finalStatus.maybeStallDiagnosisUnavailableReason)}`;
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

export function writeReportFiles(repoRootPath: string, report: SmokeReport): { jsonPath: string; markdownPath: string } {
  const absoluteOutputDir = path.resolve(repoRootPath, report.options.outputDir);
  mkdirSync(absoluteOutputDir, { recursive: true });
  const jsonPath = path.join(absoluteOutputDir, `${REPORT_STEM}.json`);
  const markdownPath = path.join(absoluteOutputDir, `${REPORT_STEM}.md`);

  writeFileSync(jsonPath, `${JSON.stringify(report, null, 2)}\n`);
  writeFileSync(markdownPath, `${markdownReport(report)}\n`);

  return { jsonPath, markdownPath };
}

type JsonRecord = Record<string, any>;

async function readReport(reportPath: string): Promise<JsonRecord> {
  return await Bun.file(reportPath).json() as JsonRecord;
}

function assert(condition: boolean, message: string): void {
  if (!condition) {
    throw new Error(message);
  }
}

async function assertProgress(reportPath: string): Promise<void> {
  const report = await readReport(reportPath);
  assert(
    report.result.firstHeaderProgress.before.headerHeight === 0 &&
      report.result.firstHeaderProgress.after.headerHeight === 1,
    "firstHeaderProgress headerHeight evidence missing",
  );
  assert(
    report.result.firstHeaderProgress.before.progressSignal === "waiting_for_peers" &&
      report.result.firstHeaderProgress.after.progressSignal === "header_progress",
    "firstHeaderProgress progressSignal evidence missing",
  );
  assert(
    report.snapshots[0].progressSignal === "waiting_for_peers",
    "snapshot progressSignal evidence missing",
  );
  assert(
    report.snapshots.at(-1).progressCreditKind === "validated_durable_active_chain" &&
      report.snapshots.at(-1).expectedProgressWindowSeconds === 300 &&
      report.snapshots.at(-1).stalledSubsystem === "at_tip_waiting",
    "phase78 snapshot evidence missing",
  );
  assert(
    report.final_status.configuredTargets.targetOutboundPeers === 4 &&
      report.final_status.configuredTargets.maybeTargetHeaderHeight === 840200,
    "final configuredTargets evidence missing",
  );
  assert(
    report.final_status.attemptCounters.attemptedPeers === 3 &&
      report.final_status.attemptCounters.connectedPeers === 3 &&
      report.final_status.attemptCounters.failedPeers === 1 &&
      report.final_status.attemptCounters.maxSyncRounds === 8,
    "final attemptCounters evidence missing",
  );
  assert(
    report.final_status.latestStopReason.label === "target_header_reached",
    "latestStopReason evidence missing",
  );
  assert(
    report.final_status.recoveryAction === "Retry with a reachable manual peer.",
    "recoveryAction evidence missing",
  );
  assert(
    report.final_status.recoveryEvidence?.category === "storage_lock_contention" &&
      report.final_status.recoveryActionClass === "read_only_inspection" &&
      report.final_status.recoveryCause === "stale_lock_evidence" &&
      report.final_status.recoveryNextAction ===
        "Inspect the datadir read-only and avoid deleting lock artifacts automatically." &&
      report.final_status.maybeRecoveryEvidenceUnavailableReason === null,
    "phase77 recovery evidence missing",
  );
  assert(
    report.final_status.resourcePressure.targetOutboundPeers === 4,
    "resourcePressure evidence missing",
  );
  assert(
    report.final_status.progressCreditKind === "validated_durable_active_chain" &&
      report.final_status.progressCreditHeight === 840004 &&
      report.final_status.progressCreditSourceUnixSeconds === 1777225005 &&
      report.final_status.expectedProgressWindowSeconds === 300 &&
      report.final_status.noProgressThresholdState === "within_window" &&
      report.final_status.noProgressThresholdSeconds === 300 &&
      report.final_status.lastUsefulWorkKind === "current_at_best_known_tip" &&
      report.final_status.lastUsefulWorkHeight === 840004 &&
      report.final_status.lastPeerContribution?.kind === "headers_and_blocks" &&
      report.final_status.stalledSubsystem === "at_tip_waiting" &&
      report.final_status.stallConfidence === "high" &&
      report.final_status.stallEvidenceBasis.join(",") ===
        "validated_active_chain,fresh_tip" &&
      report.final_status.stallNextAction === "No operator action required.",
    "phase78 live-smoke final status evidence missing",
  );
  assert(
    report.final_status.validatedActiveChainHeight === 840004 &&
      report.final_status.maybeValidatedActiveChainHash ===
        "1111111111111111111111111111111111111111111111111111111111111111" &&
      report.final_status.maybeValidatedActiveChainWork === "840005" &&
      report.final_status.bestKnownTip?.freshness === "fresh" &&
      report.final_status.stayCurrent === "current_at_best_known_tip" &&
      report.final_status.stayCurrentNextAction ===
        "Continue monitoring best-known tip freshness." &&
      report.final_status.noProgressDiagnosis === "current_at_best_known_tip" &&
      report.final_status.noProgressNextAction === "No operator action required." &&
      report.final_status.latestReorg?.fullyPersisted === true &&
      report.final_status.reconcileProgress?.state === "extended_active_chain" &&
      report.final_status.peerContribution?.connected === 3 &&
      report.final_status.peerContribution?.failed === 1,
    "phase72 live-smoke final status evidence missing",
  );
  assert(
    report.result.firstBlockProgress.before.downloadedBlockHeight === 0 &&
      report.result.firstBlockProgress.after.connectedBlockHeight === 1,
    "firstBlockProgress downloadedBlockHeight/connectedBlockHeight evidence missing",
  );
  assert(
    !("stdoutTail" in report.daemon) && !("stderrTail" in report.daemon),
    "daemon tails persisted in JSON",
  );
}

async function assertMissingValidatedHeight(reportPath: string): Promise<void> {
  const report = await readReport(reportPath);
  assert(
    report.final_status.validatedActiveChainHeight === null,
    "missing validated active-chain height was synthesized",
  );
  assert(
    report.final_status.maybeValidatedActiveChainHeightUnavailableReason ===
      "validated active-chain height unavailable",
    "missing validated active-chain height reason missing",
  );
  assert(
    report.final_status.connectedBlockHeight === 840004,
    "connected block height evidence was lost",
  );
}

async function assertRestart(reportPath: string): Promise<void> {
  const report = await readReport(reportPath);
  assert(
    typeof report.result.restartResumeEvidence.recoveryDiagnosis.category ===
      "string",
    "restartResumeEvidence recoveryDiagnosis category missing",
  );
}

async function main(): Promise<void> {
  const [command, reportPath] = process.argv.slice(2);
  if (reportPath === undefined) {
    throw new Error(
      "usage: assert-report.ts <progress|missing-validated-height|restart|extract-restart-evidence> REPORT",
    );
  }
  switch (command) {
    case "progress":
      return assertProgress(reportPath);
    case "missing-validated-height":
      return assertMissingValidatedHeight(reportPath);
    case "restart":
      return assertRestart(reportPath);
    case "extract-restart-evidence": {
      const report = await readReport(reportPath);
      console.log(JSON.stringify(report.result.restartResumeEvidence));
      return;
    }
    default:
      throw new Error(`unknown assertion command: ${command}`);
  }
}

await main();

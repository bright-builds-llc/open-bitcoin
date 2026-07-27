import path from "node:path";
import { daemonCommand, finalStatusCommand, findFreePort, readFinalStatus, reportCommand, statusCommandForRpcPort } from "./command";
import { classifyNoProgressCause, endpointOutcomesFromFinalStatus, firstBlockProgressEvidence, firstHeaderProgressEvidence, nextActionForCause, noProgressCauseFromFinalStatus, restartResumeEvidence } from "./diagnosis";
import { optionsWithGeneratedManualPeerConfig, parseArgs } from "./options";
import { buildPreflightChecks, ensureBuiltBinaries, networkPreflightEndpointOutcomes, peerSourcesFromOptions, repoRoot, skippedEndpointOutcomes } from "./preflight";
import { BASELINE, writeReportFiles } from "./report";
import { firstNonNullProgressSnapshots, lastSnapshot, runSmokeSession } from "./session";
import type { CommandSpec, EndpointOutcome, FinalStatusSummary, Options, PreflightCheck, RestartResumeEvidence, SmokeReport, SmokeSessionResult } from "./types";

function preflightFailureReport(
  options: Options,
  checks: PreflightCheck[],
  daemonSpec: CommandSpec,
  statusSpec: CommandSpec,
  endpointOutcomes: EndpointOutcome[],
): SmokeReport {
  const message = checks.filter((check) => !check.ok).map((check) => check.detail).join(" ");
  return {
    baseline: BASELINE,
    commands: { daemon: reportCommand(daemonSpec), finalStatus: [], status: reportCommand(statusSpec) },
    daemon_sessions: [],
    daemon: { maybeExitCode: null, maybeSignal: null, stderrLineCount: 0, stderrObserved: false, stdoutLineCount: 0, stdoutObserved: false },
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
    network_preflight: { completed: false, endpoint_outcomes: endpointOutcomes },
    preflight: { checks, passed: false },
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

export async function main(): Promise<void> {
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
  const sessions = maybeRestartSession === null ? [firstSession] : [firstSession, maybeRestartSession];
  const daemonSessions = sessions.map((session) => ({
    daemon: reportCommand(session.daemonSpec),
    status: reportCommand(session.statusSpec),
  }));
  const snapshots = sessions.flatMap((session) => session.snapshots);
  let resultStatus = firstSession.resultStatus;
  let resultMessage = firstSession.resultMessage;
  let headerDelta = sessions.reduce((sum, session) => sum + session.headerDelta, 0);
  let blockDelta = sessions.reduce((sum, session) => sum + session.blockDelta, 0);
  const maybeLastProbeError = maybeRestartSession?.maybeLastProbeError ?? firstSession.maybeLastProbeError;
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
      resultMessage = "Observed same-datadir restart/resume evidence: a fresh post-restart status snapshot preserved durable header, downloaded block, and connected block progress.";
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
      resultMessage = "No pre-restart progress was observed, so the requested same-datadir restart was blocked before evidence could be captured.";
    } else if (lastSnapshot(maybeRestartSession.snapshots) === null) {
      resultStatus = "runtime_failed";
      resultMessage = "Post-restart daemon session did not produce a fresh status snapshot.";
    } else {
      resultStatus = "no_progress";
      resultMessage = "Post-restart durable resume evidence did not preserve the expected same-datadir heights and hashes.";
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
  const maybeFirstBlockProgress = maybeFirstConnectedBlockProgress ?? maybeFirstDownloadedBlockProgress;

  const noProgressCauseFromEvidence = resultStatus === "no_progress" &&
      (maybeFirstBlockProgress !== null || maybeFirstHeaderProgress !== null)
    ? "awaiting_blocks"
    : null;

  if (resultStatus === "no_progress" && maybeRestartResumeEvidence === null) {
    const noProgressCause = noProgressCauseFromEvidence ??
      classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError);
    if (maybeFirstDownloadedBlockProgress !== null) {
      resultMessage = `Downloaded block progress was observed, but connected block height did not advance before timeout; typed no-progress cause: ${noProgressCause}.`;
    } else if (maybeFirstHeaderProgress !== null) {
      resultMessage = `Header progress was observed, but no connected block progress was reached before timeout; typed no-progress cause: ${noProgressCause}.`;
    } else if (maybeFinalStatus?.outboundPeers === 0) {
      resultMessage = `No header or block progress was observed before timeout. Final durable sync status still showed 0 outbound peers; typed no-progress cause: ${noProgressCause}.`;
    } else if (maybeLastProbeError !== null) {
      resultMessage = `No header or block progress was observed before timeout. Last RPC probe error: ${maybeLastProbeError}`;
    }
  }

  const maybeNoProgressCause = resultStatus === "cancelled" ? "operator_cancellation" : resultStatus === "no_progress"
    ? noProgressCauseFromEvidence ??
      classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError)
    : resultStatus === "runtime_failed"
    ? noProgressCauseFromFinalStatus(maybeFinalStatus)
    : null;

  const report: SmokeReport = {
    baseline: BASELINE,
    commands: {
      daemon: daemonSessions[0]?.daemon ?? reportCommand(daemonSpec),
      finalStatus: reportCommand(postRunStatusSpec),
      status: daemonSessions[0]?.status ?? reportCommand(statusSpec),
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

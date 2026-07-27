import { ChildProcess, spawn } from "node:child_process";
import { daemonCommand, findFreePort, readSyncStatus, statusCommandForRpcPort } from "./command";
import type { DuplicateConnectVerdict, EndpointOutcome, NoProgressCause, Options, RestartPeerOutcomeSummary, RestartProgressDelta, RestartProgressSummary, RestartStatus, SmokeSessionMode, SmokeSessionResult, SyncStatusSnapshot } from "./types";

const MAX_TAIL_BYTES = 16 * 1024;

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

export async function runSmokeSession(
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
  let resultMessage = "No header or block progress was observed before timeout. Check outbound network access, DNS reachability, local disk headroom, and system time.";
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
        maybeLastProbeError = error instanceof Error ? error.message : String(error);
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
        resultMessage = snapshot.maybeLastError === null ? "open-bitcoind exited before reporting progress." : `open-bitcoind exited before reporting progress: ${snapshot.maybeLastError}`;
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

export function firstNonNullProgressSnapshots(
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

export function lastSnapshot(snapshots: SyncStatusSnapshot[]): SyncStatusSnapshot | null {
  return snapshots.at(-1) ?? null;
}

export function restartProgressSummary(
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
    maybeAttemptCountersUnavailableReason: maybeSnapshot.maybeAttemptCountersUnavailableReason,
    maybeConnectedBlockHash: maybeSnapshot.maybeConnectedBlockHash,
    maybeConfiguredTargetsUnavailableReason: maybeSnapshot.maybeConfiguredTargetsUnavailableReason,
    maybeDownloadedBlockHash: maybeSnapshot.maybeDownloadedBlockHash,
    maybeLastError: maybeSnapshot.maybeLastError,
    maybeLastErrorUnavailableReason: maybeSnapshot.maybeLastErrorUnavailableReason,
    maybeLastSuccessfulProgressUnixSeconds: maybeSnapshot.maybeLastSuccessfulProgressUnixSeconds,
    maybeLatestStopReasonUnavailableReason: maybeSnapshot.maybeLatestStopReasonUnavailableReason,
    maybePeerCountsUnavailableReason: maybeSnapshot.maybePeerCountsUnavailableReason,
    maybeProgressSignalUnavailableReason: maybeSnapshot.maybeProgressSignalUnavailableReason,
    maybeRecoveryActionUnavailableReason: maybeSnapshot.maybeRecoveryActionUnavailableReason,
    maybeRecoveryCategoryUnavailableReason: maybeSnapshot.maybeRecoveryCategoryUnavailableReason,
    maybeResourcePressureUnavailableReason: maybeSnapshot.maybeResourcePressureUnavailableReason,
    maybeSyncProgressUnavailableReason: maybeSnapshot.maybeSyncProgressUnavailableReason,
    phase: maybeSnapshot.phase,
    progressSignal: maybeSnapshot.progressSignal,
    recoveryAction: maybeSnapshot.recoveryAction,
    recoveryCategory: maybeSnapshot.recoveryCategory,
    resourcePressure: maybeSnapshot.resourcePressure,
  };
}

export function heightDelta(after: number | null, before: number | null): number | null {
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

export function restartProgressDelta(
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

export function peerOutcomeSummary(endpointOutcomes: EndpointOutcome[]): RestartPeerOutcomeSummary {
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

export function duplicateConnectVerdict(
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

export function restartStatus(
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

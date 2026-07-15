#!/usr/bin/env bun

import { randomUUID } from "node:crypto";
import {
  mkdir,
  readFile,
  readdir,
  rename,
  rm,
  writeFile,
} from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import {
  acquireTargetLock,
  normalizeCommandKey,
  timingProcessIsAlive,
} from "./command-timing-lock";
import type { TargetLock } from "./command-timing-lock";
import { filterProcessEvidence } from "./process-liveness";

export { acquireTargetLock, normalizeCommandKey } from "./command-timing-lock";
export type { TargetLock } from "./command-timing-lock";

export const TIMING_SCHEMA_VERSION = 1;
export const DEFAULT_RETENTION = 200;
const DEFAULT_HEARTBEAT_MS = 60_000;
const ESTABLISHED_MINIMUM_SAMPLES = 5;
const MINIMUM_SOFT_LIMIT_MS = 15 * 60_000;
const WORKSPACE_SOFT_LIMIT_MS = 60 * 60_000;

export type TimingOutcome = "running" | "success" | "failure" | "interrupted";
export type TargetKind = "default" | "isolated";

export type TimingRecord = {
  schemaVersion: number;
  runId: string;
  key: string;
  source: string;
  startedAt: string;
  endedAt: string | null;
  durationMs: number | null;
  outcome: TimingOutcome;
  exitStatus: number | null;
  signal: string | null;
  pid: number;
  git: { commit: string | null; dirty: boolean | null };
  platform: { os: string; arch: string };
  rustVersion: string | null;
  verifyMode: string | null;
  target: { kind: TargetKind };
};

export type TimingSummary = {
  key: string;
  currentRuns: number;
  latestDurationMs: number | null;
  sampleCount: number;
  medianMs: number | null;
  p90Ms: number | null;
  p95Ms: number | null;
  maximumMs: number | null;
  failures: number;
  interruptions: number;
};

type RunOptions = {
  key: string;
  source?: string;
  verifyMode?: string | null;
  cwd?: string;
  stateRoot?: string;
  heartbeatMs?: number;
  retention?: number;
  stderr?: Pick<typeof process.stderr, "write">;
};

export type BatchEntry = {
  key: string;
  startedAtMs: number;
  durationMs: number;
  exitStatus: number;
};

export function percentile(values: readonly number[], fraction: number): number | null {
  if (values.length === 0) {
    return null;
  }
  const sorted = [...values].sort((left, right) => left - right);
  const index = Math.max(0, Math.ceil(fraction * sorted.length) - 1);
  return sorted[index] ?? null;
}

export function resolveStateRoot(repoRoot = process.cwd()): string {
  return (
    process.env.OPEN_BITCOIN_DEV_STATE_DIR ??
    path.join(repoRoot, ".local", "open-bitcoin-dev")
  );
}

export function resolveTarget(repoRoot = process.cwd()): {
  directory: string;
  kind: TargetKind;
} {
  const maybeTarget = process.env.CARGO_TARGET_DIR;
  if (maybeTarget === undefined || maybeTarget.length === 0) {
    return { directory: path.join(repoRoot, "packages", "target"), kind: "default" };
  }
  return {
    directory: path.resolve(repoRoot, maybeTarget),
    kind: "isolated",
  };
}

async function commandOutput(
  command: readonly string[],
  cwd: string,
  preserveEmpty = false,
): Promise<string | null> {
  try {
    const result = Bun.spawnSync(command, { cwd, stderr: "ignore", stdout: "pipe" });
    if (result.exitCode !== 0) {
      return null;
    }
    const output = result.stdout.toString().trim();
    return preserveEmpty ? output : output || null;
  } catch {
    return null;
  }
}

async function createRecord(
  key: string,
  source: string,
  startedAt: Date,
  cwd: string,
  verifyMode: string | null,
  targetKind: TargetKind,
): Promise<TimingRecord> {
  const [maybeCommit, maybeDirtyText, maybeRustVersion] = await Promise.all([
    commandOutput(["git", "rev-parse", "HEAD"], cwd),
    commandOutput(["git", "status", "--porcelain"], cwd, true),
    commandOutput(["rustc", "--version"], cwd),
  ]);
  return {
    schemaVersion: TIMING_SCHEMA_VERSION,
    runId: randomUUID(),
    key: normalizeCommandKey(key),
    source,
    startedAt: startedAt.toISOString(),
    endedAt: null,
    durationMs: null,
    outcome: "running",
    exitStatus: null,
    signal: null,
    pid: process.pid,
    git: { commit: maybeCommit, dirty: maybeDirtyText === null ? null : maybeDirtyText.length > 0 },
    platform: { os: os.platform(), arch: os.arch() },
    rustVersion: maybeRustVersion,
    verifyMode,
    target: { kind: targetKind },
  };
}

function fallbackRecord(
  key: string,
  source: string,
  startedAt: Date,
  verifyMode: string | null,
  targetKind: TargetKind,
): TimingRecord {
  return {
    schemaVersion: TIMING_SCHEMA_VERSION,
    runId: randomUUID(),
    key: normalizeCommandKey(key),
    source,
    startedAt: startedAt.toISOString(),
    endedAt: null,
    durationMs: null,
    outcome: "running",
    exitStatus: null,
    signal: null,
    pid: process.pid,
    git: { commit: null, dirty: null },
    platform: { os: os.platform(), arch: os.arch() },
    rustVersion: null,
    verifyMode,
    target: { kind: targetKind },
  };
}

function recordDirectory(stateRoot: string, key: string): string {
  return path.join(stateRoot, "command-timings", normalizeCommandKey(key));
}

export async function writeTimingRecord(
  stateRoot: string,
  record: TimingRecord,
  retention = DEFAULT_RETENTION,
): Promise<string> {
  const directory = recordDirectory(stateRoot, record.key);
  await mkdir(directory, { recursive: true });
  const filename = `${record.startedAt.replaceAll(":", "-")}-${record.runId}.json`;
  const destination = path.join(directory, filename);
  const temporary = `${destination}.tmp-${process.pid}-${randomUUID()}`;
  await writeFile(temporary, `${JSON.stringify(record, null, 2)}\n`);
  await rename(temporary, destination);

  const files = (await readdir(directory))
    .filter((entry) => entry.endsWith(".json"))
    .sort()
    .reverse();
  await Promise.all(
    files.slice(retention).map((entry) => rm(path.join(directory, entry), { force: true })),
  );
  return destination;
}

function isTimingRecord(value: unknown): value is TimingRecord {
  if (typeof value !== "object" || value === null) {
    return false;
  }
  const record = value as Partial<TimingRecord>;
  return (
    record.schemaVersion === TIMING_SCHEMA_VERSION &&
    typeof record.key === "string" &&
    typeof record.startedAt === "string" &&
    typeof record.outcome === "string"
  );
}

export async function readTimingRecords(
  stateRoot: string,
  maybeKey?: string,
): Promise<TimingRecord[]> {
  const root = path.join(stateRoot, "command-timings");
  let keyDirectories: string[];
  try {
    keyDirectories = maybeKey
      ? [normalizeCommandKey(maybeKey)]
      : (await readdir(root, { withFileTypes: true }))
          .filter((entry) => entry.isDirectory())
          .map((entry) => entry.name);
  } catch {
    return [];
  }

  const records: TimingRecord[] = [];
  for (const keyDirectory of keyDirectories) {
    const directory = path.join(root, keyDirectory);
    let files: string[];
    try {
      files = (await readdir(directory)).filter((entry) => entry.endsWith(".json"));
    } catch {
      continue;
    }
    for (const file of files) {
      try {
        const value = JSON.parse(await readFile(path.join(directory, file), "utf8"));
        if (isTimingRecord(value)) {
          records.push(value);
        }
      } catch {
        // Malformed local records are ignored; the next report remains usable.
      }
    }
  }
  return records.sort((left, right) => right.startedAt.localeCompare(left.startedAt));
}

export function summarizeRecords(records: readonly TimingRecord[]): TimingSummary[] {
  const keys = [...new Set(records.map((record) => record.key))].sort();
  return keys.map((key) => {
    const matching = records.filter((record) => record.key === key);
    const completed = matching.filter((record) => record.durationMs !== null);
    const successful = completed.filter((record) => record.outcome === "success");
    const durations = successful.map((record) => record.durationMs ?? 0);
    return {
      key,
      currentRuns: matching.filter(
        (record) => record.outcome === "running" && timingProcessIsAlive(record.pid),
      ).length,
      latestDurationMs: completed[0]?.durationMs ?? null,
      sampleCount: successful.length,
      medianMs: percentile(durations, 0.5),
      p90Ms: percentile(durations, 0.9),
      p95Ms: percentile(durations, 0.95),
      maximumMs: durations.length > 0 ? Math.max(...durations) : null,
      failures: matching.filter((record) => record.outcome === "failure").length,
      interruptions: matching.filter((record) => record.outcome === "interrupted").length,
    };
  });
}

function isWorkspaceWide(key: string, command: readonly string[]): boolean {
  const joined = `${key} ${command.join(" ")}`.toLowerCase();
  return (
    joined.includes("verify-full") ||
    joined.includes("verify-profile") ||
    (joined.includes("cargo") &&
      (joined.includes("workspace") || joined.includes("cargo-test-workspace")))
  );
}

export function softLimitMilliseconds(
  records: readonly TimingRecord[],
  key: string,
  command: readonly string[],
  maybeTargetKind?: TargetKind,
): number {
  const durations = records
    .filter(
      (record) =>
        record.key === key &&
        record.outcome === "success" &&
        (maybeTargetKind === undefined || record.target.kind === maybeTargetKind),
    )
    .map((record) => record.durationMs)
    .filter((duration): duration is number => duration !== null);
  if (durations.length >= ESTABLISHED_MINIMUM_SAMPLES) {
    return Math.max(MINIMUM_SOFT_LIMIT_MS, 2 * (percentile(durations, 0.9) ?? 0));
  }
  return isWorkspaceWide(key, command) ? WORKSPACE_SOFT_LIMIT_MS : MINIMUM_SOFT_LIMIT_MS;
}

function formatDuration(durationMs: number | null): string {
  if (durationMs === null) {
    return "n/a";
  }
  const seconds = Math.round(durationMs / 100) / 10;
  return `${seconds}s`;
}

export function renderTimingReport(summaries: readonly TimingSummary[]): string {
  if (summaries.length === 0) {
    return "No local command timing history yet.";
  }
  const lines = [
    "Command timing history (local, advisory):",
    "key | running | latest | samples | median | p90 | p95 | max | failures | interrupted",
  ];
  for (const summary of summaries) {
    lines.push(
      [
        summary.key,
        summary.currentRuns,
        formatDuration(summary.latestDurationMs),
        summary.sampleCount,
        formatDuration(summary.medianMs),
        formatDuration(summary.p90Ms),
        formatDuration(summary.p95Ms),
        formatDuration(summary.maximumMs),
        summary.failures,
        summary.interruptions,
      ].join(" | "),
    );
  }
  return lines.join("\n");
}

async function captureLivenessEvidence(
  stateRoot: string,
  key: string,
  childPid: number,
): Promise<string> {
  const directory = path.join(
    stateRoot,
    "stall-diagnostics",
    `${new Date().toISOString().replaceAll(":", "-")}-${normalizeCommandKey(key)}`,
  );
  await mkdir(directory, { recursive: true });
  try {
    const processResult = Bun.spawnSync(
      ["ps", "-axo", "pid,ppid,state,etime,%cpu,%mem,command"],
      { stderr: "pipe", stdout: "pipe" },
    );
    const evidence = filterProcessEvidence(processResult.stdout.toString(), childPid);
    await writeFile(path.join(directory, "process-tree.txt"), evidence.targetTree);
    await writeFile(path.join(directory, "active-cargo-jobs.txt"), evidence.cargoJobs);
  } catch (error) {
    const message = `${String(error)}\n`;
    await writeFile(path.join(directory, "process-tree.txt"), message);
    await writeFile(path.join(directory, "active-cargo-jobs.txt"), message);
  }
  const commands: Array<[string, string[]]> = [
    ["disk.txt", ["df", "-h"]],
    ["lsof.txt", ["lsof", "-p", String(childPid)]],
  ];
  if (os.platform() === "darwin") {
    commands.push(["sample.txt", ["sample", String(childPid), "5", "1"]]);
  }
  for (const [filename, command] of commands) {
    try {
      const result = Bun.spawnSync(command, { stderr: "pipe", stdout: "pipe" });
      await writeFile(
        path.join(directory, filename),
        `${result.stdout.toString()}${result.stderr.toString()}`,
      );
    } catch (error) {
      await writeFile(path.join(directory, filename), `${String(error)}\n`);
    }
  }
  return directory;
}

function outcomeForExit(exitStatus: number, maybeSignal: string | null): TimingOutcome {
  if (maybeSignal !== null || exitStatus === 130 || exitStatus === 143) {
    return "interrupted";
  }
  return exitStatus === 0 ? "success" : "failure";
}

function timingWarning(
  stderr: Pick<typeof process.stderr, "write">,
  context: string,
  error: unknown,
): void {
  stderr.write(
    `[timing] warning: ${context}: ${error instanceof Error ? error.message : String(error)}\n`,
  );
}

async function persistRecordWithoutMasking(
  stateRoot: string,
  record: TimingRecord,
  retention: number | undefined,
  stderr: Pick<typeof process.stderr, "write">,
): Promise<void> {
  try {
    await writeTimingRecord(stateRoot, record, retention);
  } catch (error) {
    timingWarning(stderr, "could not persist local timing history", error);
  }
}

export async function executeTimedCommand(
  command: readonly string[],
  options: RunOptions,
): Promise<TimingRecord> {
  if (command.length === 0) {
    throw new Error("run requires a command after --");
  }
  const cwd = options.cwd ?? process.cwd();
  const stateRoot = options.stateRoot ?? resolveStateRoot(cwd);
  const key = normalizeCommandKey(options.key);
  const target = resolveTarget(cwd);
  const heartbeatMs = options.heartbeatMs ?? DEFAULT_HEARTBEAT_MS;
  const stderr = options.stderr ?? process.stderr;
  const startedAt = new Date();
  const verifyMode = options.verifyMode ?? process.env.OPEN_BITCOIN_VERIFY_MODE ?? null;
  let maybeLock: TargetLock | null = null;
  try {
    try {
      maybeLock = await acquireTargetLock({
        key,
        targetDirectory: target.directory,
        stateRoot,
        heartbeatMs,
        stderr,
      });
    } catch (error) {
      timingWarning(stderr, "cooperative lock unavailable; continuing without it", error);
    }

    let record: TimingRecord;
    try {
      record = await createRecord(
        key,
        options.source ?? "ad-hoc",
        startedAt,
        cwd,
        verifyMode,
        target.kind,
      );
    } catch (error) {
      timingWarning(stderr, "could not collect timing metadata", error);
      record = fallbackRecord(
        key,
        options.source ?? "ad-hoc",
        startedAt,
        verifyMode,
        target.kind,
      );
    }
    await persistRecordWithoutMasking(stateRoot, record, options.retention, stderr);

    const previousRecords = await readTimingRecords(stateRoot, key);
    const softLimitMs = softLimitMilliseconds(previousRecords, key, command, target.kind);
    let child: ReturnType<typeof Bun.spawn>;
    try {
      child = Bun.spawn(command, {
        cwd,
        env: { ...process.env, OPEN_BITCOIN_TIMING_WRAPPED: "1" },
        stdin: "inherit",
        stdout: "inherit",
        stderr: "inherit",
      });
    } catch (error) {
      timingWarning(stderr, "could not spawn command", error);
      record.endedAt = new Date().toISOString();
      record.durationMs = new Date(record.endedAt).getTime() - startedAt.getTime();
      record.exitStatus = 127;
      record.outcome = "failure";
      await persistRecordWithoutMasking(stateRoot, record, options.retention, stderr);
      return record;
    }

    let evidenceCaptured = false;
    let maybeEvidencePromise: Promise<void> | null = null;
    const heartbeat = setInterval(() => {
      const elapsedMs = Date.now() - startedAt.getTime();
      stderr.write(`[timing] ${key} still running (${formatDuration(elapsedMs)})\n`);
      if (!evidenceCaptured && elapsedMs >= softLimitMs) {
        evidenceCaptured = true;
        maybeEvidencePromise = captureLivenessEvidence(stateRoot, key, child.pid)
          .then((evidence) => {
            stderr.write(
              `[timing] ${key} exceeded its advisory threshold; evidence: ${evidence}\n`,
            );
          })
          .catch((error) => {
            timingWarning(stderr, "could not capture liveness evidence", error);
          });
      }
    }, heartbeatMs);

    const forwardSignal = (signal: NodeJS.Signals) => child.kill(signal);
    const onInterrupt = () => forwardSignal("SIGINT");
    const onTerminate = () => forwardSignal("SIGTERM");
    process.on("SIGINT", onInterrupt);
    process.on("SIGTERM", onTerminate);
    let exitStatus = 1;
    try {
      exitStatus = await child.exited;
    } catch (error) {
      timingWarning(stderr, "could not read child exit status", error);
    } finally {
      clearInterval(heartbeat);
      process.off("SIGINT", onInterrupt);
      process.off("SIGTERM", onTerminate);
    }
    await maybeEvidencePromise;
    const maybeSignal =
      (child as unknown as { signalCode?: string | null }).signalCode ??
      (exitStatus === 130 ? "SIGINT" : exitStatus === 143 ? "SIGTERM" : null);
    record.endedAt = new Date().toISOString();
    record.durationMs = new Date(record.endedAt).getTime() - startedAt.getTime();
    record.exitStatus = exitStatus;
    record.signal = maybeSignal;
    record.outcome = outcomeForExit(exitStatus, maybeSignal);
    await persistRecordWithoutMasking(stateRoot, record, options.retention, stderr);
    return record;
  } finally {
    if (maybeLock !== null) {
      try {
        await maybeLock.release();
      } catch (error) {
        timingWarning(stderr, "could not release cooperative lock", error);
      }
    }
  }
}

export async function recordTimingBatch(
  entries: readonly BatchEntry[],
  options: RunOptions,
): Promise<void> {
  const cwd = options.cwd ?? process.cwd();
  const stateRoot = options.stateRoot ?? resolveStateRoot(cwd);
  const target = resolveTarget(cwd);
  const base = await createRecord(
    "verify-step",
    options.source ?? "verify",
    new Date(),
    cwd,
    options.verifyMode ?? null,
    target.kind,
  );
  for (const entry of entries) {
    const startedAt = new Date(entry.startedAtMs);
    const record: TimingRecord = {
      ...base,
      runId: randomUUID(),
      key: normalizeCommandKey(entry.key),
      startedAt: startedAt.toISOString(),
      endedAt: new Date(entry.startedAtMs + entry.durationMs).toISOString(),
      durationMs: entry.durationMs,
      outcome: outcomeForExit(entry.exitStatus, null),
      exitStatus: entry.exitStatus,
      signal: entry.exitStatus === 130 ? "SIGINT" : entry.exitStatus === 143 ? "SIGTERM" : null,
    };
    if (record.signal !== null) {
      record.outcome = "interrupted";
    }
    await writeTimingRecord(stateRoot, record, options.retention);
  }
}

if (import.meta.main) {
  try {
    const { runCommandTimingCli } = await import("./command-timing-cli");
    process.exitCode = await runCommandTimingCli(process.argv.slice(2));
  } catch (error) {
    console.error(`error: ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 2;
  }
}

#!/usr/bin/env bun

import { randomUUID } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { resolveStateRoot, resolveTarget } from "./command-timings";
import { filterProcessEvidence } from "./process-liveness";

type TargetKind = "lib" | "test" | "bin";
const MINIMUM_FREE_BYTES = 100 * 1024 * 1024 * 1024;

export type DiagnosticOptions = {
  packageName: string;
  targetName: string;
  targetKind: TargetKind;
  attempts: number;
  startupThresholdMs: number;
  maybeStopAfterMs: number | null;
  repoRoot: string;
  stateRoot: string;
};

export type HarnessAttempt = {
  attempt: number;
  exitStatus: number;
  startupMs: number | null;
  diagnosticDirectory: string | null;
  stoppedByLimit: boolean;
};

export type DiagnosticClassification =
  | "disk-loader-concurrency-pressure"
  | "pre-harness-stall-reproduced"
  | "harness-list-failed";

type EvidenceCapture = (childPid: number, attempt: number) => Promise<string>;

type KillableChild = {
  kill: (signal: NodeJS.Signals) => unknown;
};

export type HarnessChild = KillableChild & {
  pid: number;
  exited: Promise<number>;
  stdout: ReadableStream<Uint8Array>;
  stderr: ReadableStream<Uint8Array>;
};

export type HarnessSpawner = (executable: string, repoRoot: string) => HarnessChild;

type SignalSource = {
  on: (event: NodeJS.Signals, listener: () => void) => unknown;
  off: (event: NodeJS.Signals, listener: () => void) => unknown;
};

export type ChildSignalForwarding = {
  maybeSignal: () => NodeJS.Signals | null;
  cleanup: () => void;
};

class DiagnosticCancelled extends Error {
  constructor(
    readonly signal: NodeJS.Signals,
    readonly exitStatus: number,
  ) {
    super(`diagnostic cancelled by ${signal}`);
  }
}

export function installChildSignalForwarding(
  child: KillableChild,
  signalSource: SignalSource = process,
): ChildSignalForwarding {
  let maybeSignal: NodeJS.Signals | null = null;
  const onInterrupt = () => {
    maybeSignal = "SIGINT";
    child.kill("SIGINT");
  };
  const onTerminate = () => {
    maybeSignal = "SIGTERM";
    child.kill("SIGTERM");
  };
  signalSource.on("SIGINT", onInterrupt);
  signalSource.on("SIGTERM", onTerminate);
  return {
    maybeSignal: () => maybeSignal,
    cleanup() {
      signalSource.off("SIGINT", onInterrupt);
      signalSource.off("SIGTERM", onTerminate);
    },
  };
}

function throwIfCancelled(maybeSignal: NodeJS.Signals | null): void {
  if (maybeSignal === "SIGINT") {
    throw new DiagnosticCancelled(maybeSignal, 130);
  }
  if (maybeSignal === "SIGTERM") {
    throw new DiagnosticCancelled(maybeSignal, 143);
  }
}

function parsePositiveInteger(value: string, option: string): number {
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${option} must be a positive integer`);
  }
  return parsed;
}

function takeOption(args: string[], name: string): string | undefined {
  const index = args.indexOf(name);
  if (index === -1) {
    return undefined;
  }
  const value = args[index + 1];
  if (value === undefined) {
    throw new Error(`${name} requires a value`);
  }
  args.splice(index, 2);
  return value;
}

export function parseDiagnosticOptions(
  rawArgs: readonly string[],
  repoRoot = process.cwd(),
): DiagnosticOptions {
  const args = [...rawArgs];
  const packageName = takeOption(args, "--package") ?? "open-bitcoin-cli";
  const targetName = takeOption(args, "--target") ?? "open_bitcoin_cli";
  const targetKindText = takeOption(args, "--kind") ?? "lib";
  if (targetKindText !== "lib" && targetKindText !== "test" && targetKindText !== "bin") {
    throw new Error("--kind must be lib, test, or bin");
  }
  const attempts = parsePositiveInteger(takeOption(args, "--attempts") ?? "5", "--attempts");
  const startupThresholdMs = parsePositiveInteger(
    takeOption(args, "--startup-threshold-ms") ?? "10000",
    "--startup-threshold-ms",
  );
  const maybeStopText = takeOption(args, "--stop-after-ms");
  const maybeStopAfterMs =
    maybeStopText === undefined
      ? null
      : parsePositiveInteger(maybeStopText, "--stop-after-ms");
  if (maybeStopAfterMs !== null && maybeStopAfterMs <= startupThresholdMs) {
    throw new Error("--stop-after-ms must exceed --startup-threshold-ms");
  }
  if (args.length > 0) {
    throw new Error(`unsupported argument ${args[0]}`);
  }
  return {
    packageName,
    targetName,
    targetKind: targetKindText,
    attempts,
    startupThresholdMs,
    maybeStopAfterMs,
    repoRoot,
    stateRoot: resolveStateRoot(repoRoot),
  };
}

function targetSelector(options: DiagnosticOptions): string[] {
  if (options.targetKind === "lib") {
    return ["--lib"];
  }
  return [`--${options.targetKind}`, options.targetName];
}

export function selectExecutableFromCargoJson(
  output: string,
  targetName: string,
  targetKind: TargetKind,
): string | null {
  let maybeExecutable: string | null = null;
  for (const line of output.split("\n")) {
    if (!line.startsWith("{")) {
      continue;
    }
    try {
      const message = JSON.parse(line) as {
        reason?: string;
        executable?: string | null;
        target?: { name?: string; kind?: string[] };
      };
      if (
        message.reason === "compiler-artifact" &&
        message.target?.name === targetName &&
        message.target.kind?.includes(targetKind) &&
        typeof message.executable === "string"
      ) {
        maybeExecutable = message.executable;
      }
      const rendered = (message as { message?: { rendered?: string } }).message?.rendered;
      if (typeof rendered === "string") {
        process.stderr.write(rendered);
      }
    } catch {
      // Non-JSON Cargo output cannot identify the emitted test executable.
    }
  }
  return maybeExecutable;
}

async function compileTestExecutable(options: DiagnosticOptions): Promise<string> {
  const command = [
    "cargo",
    "test",
    "--manifest-path",
    "packages/Cargo.toml",
    "-p",
    options.packageName,
    ...targetSelector(options),
    "--all-features",
    "--no-run",
    "--message-format=json",
  ];
  console.error(
    `[stall-diagnostic] compiling ${options.packageName} ${options.targetKind} ${options.targetName}`,
  );
  const child = Bun.spawn(command, {
    cwd: options.repoRoot,
    env: process.env,
    stderr: "pipe",
    stdout: "pipe",
  });
  const signalForwarding = installChildSignalForwarding(child);
  let stdout = "";
  let stderr = "";
  let exitStatus = 1;
  try {
    [stdout, stderr, exitStatus] = await Promise.all([
      new Response(child.stdout).text(),
      new Response(child.stderr).text(),
      child.exited,
    ]);
  } finally {
    signalForwarding.cleanup();
  }
  throwIfCancelled(signalForwarding.maybeSignal());
  if (stderr.length > 0) {
    process.stderr.write(stderr);
  }
  if (exitStatus !== 0) {
    throw new Error(`Cargo compilation failed with status ${exitStatus}`);
  }
  const maybeExecutable = selectExecutableFromCargoJson(
    stdout,
    options.targetName,
    options.targetKind,
  );
  if (maybeExecutable === null) {
    throw new Error(
      `Cargo did not emit an executable for ${options.targetKind} ${options.targetName}`,
    );
  }
  return maybeExecutable;
}

export function parseAvailableBytes(dfOutput: string): number | null {
  const lines = dfOutput.trim().split("\n");
  const maybeDataLine = lines.at(-1);
  if (maybeDataLine === undefined) {
    return null;
  }
  const columns = maybeDataLine.trim().split(/\s+/);
  const maybeAvailableKilobytes = Number(columns[3]);
  if (!Number.isFinite(maybeAvailableKilobytes) || maybeAvailableKilobytes < 0) {
    return null;
  }
  return maybeAvailableKilobytes * 1024;
}

export function classifyDiagnosticAttempts(
  attempts: readonly HarnessAttempt[],
  startupThresholdMs: number,
): DiagnosticClassification {
  if (attempts.length === 0 || attempts.some((attempt) => attempt.exitStatus !== 0)) {
    return "harness-list-failed";
  }
  if (
    attempts.every(
      (attempt) =>
        attempt.startupMs !== null &&
        attempt.startupMs <= startupThresholdMs &&
        !attempt.stoppedByLimit,
    )
  ) {
    return "disk-loader-concurrency-pressure";
  }
  return "pre-harness-stall-reproduced";
}

async function requireFilesystemHeadroom(options: DiagnosticOptions): Promise<void> {
  const targetDirectory = resolveTarget(options.repoRoot).directory;
  await mkdir(targetDirectory, { recursive: true });
  const result = Bun.spawnSync(["df", "-Pk", targetDirectory], {
    stderr: "pipe",
    stdout: "pipe",
  });
  const maybeAvailableBytes = parseAvailableBytes(result.stdout.toString());
  if (result.exitCode !== 0 || maybeAvailableBytes === null) {
    throw new Error("could not determine Cargo target filesystem headroom for the clean reproduction");
  }
  if (maybeAvailableBytes < MINIMUM_FREE_BYTES) {
    const availableGiB = Math.round((maybeAvailableBytes / 1024 ** 3) * 10) / 10;
    throw new Error(
      `clean reproduction requires at least 100 GiB free; ${availableGiB} GiB is available`,
    );
  }
}

async function runCaptureCommand(
  directory: string,
  filename: string,
  command: readonly string[],
): Promise<void> {
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

export async function captureHarnessEvidence(
  options: DiagnosticOptions,
  childPid: number,
  attempt: number,
): Promise<string> {
  const directory = path.join(
    options.stateRoot,
    "stall-diagnostics",
    `${new Date().toISOString().replaceAll(":", "-")}-${randomUUID()}`,
  );
  await mkdir(directory, { recursive: true });
  await writeFile(
    path.join(directory, "summary.json"),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        capturedAt: new Date().toISOString(),
        childPid,
        attempt,
        packageName: options.packageName,
        targetName: options.targetName,
        targetKind: options.targetKind,
        startupThresholdMs: options.startupThresholdMs,
        stopAfterMs: options.maybeStopAfterMs,
      },
      null,
      2,
    )}\n`,
  );

  const processCommand = ["ps", "-axo", "pid,ppid,state,etime,%cpu,%mem,command"];
  const processResult = Bun.spawnSync(processCommand, { stderr: "pipe", stdout: "pipe" });
  const processEvidence = filterProcessEvidence(processResult.stdout.toString(), childPid);
  await writeFile(path.join(directory, "process-tree.txt"), processEvidence.targetTree);
  await writeFile(path.join(directory, "active-cargo-jobs.txt"), processEvidence.cargoJobs);
  await runCaptureCommand(directory, "disk.txt", ["df", "-h"]);
  await runCaptureCommand(directory, "lsof.txt", ["lsof", "-p", String(childPid)]);
  if (os.platform() === "darwin") {
    await runCaptureCommand(directory, "sample.txt", ["sample", String(childPid), "5", "1"]);
  } else {
    await writeFile(
      path.join(directory, "sample.txt"),
      "macOS sample is unavailable on this platform.\n",
    );
  }
  return directory;
}

async function forwardOutput(
  stream: ReadableStream<Uint8Array>,
  writer: Pick<typeof process.stdout, "write">,
  onFirstOutput: () => void,
): Promise<void> {
  const reader = stream.getReader();
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      return;
    }
    onFirstOutput();
    writer.write(value);
  }
}

const spawnHarness: HarnessSpawner = (executable, repoRoot) =>
  Bun.spawn([executable, "--list"], {
    cwd: repoRoot,
    env: process.env,
    stderr: "pipe",
    stdout: "pipe",
  });

export async function observeHarnessStartup(
  executable: string,
  attempt: number,
  options: Pick<
    DiagnosticOptions,
    "repoRoot" | "startupThresholdMs" | "maybeStopAfterMs"
  >,
  captureEvidence: EvidenceCapture,
  spawn: HarnessSpawner = spawnHarness,
): Promise<HarnessAttempt> {
  const startedAt = Date.now();
  let maybeStartupMs: number | null = null;
  let maybeDiagnosticDirectory: string | null = null;
  let stoppedByLimit = false;
  let maybeCapturePromise: Promise<void> | null = null;
  const child = spawn(executable, options.repoRoot);
  const signalForwarding = installChildSignalForwarding(child);

  const markOutput = () => {
    maybeStartupMs ??= Date.now() - startedAt;
  };
  const diagnosticTimer = setTimeout(() => {
    if (maybeStartupMs !== null) {
      return;
    }
    maybeCapturePromise = captureEvidence(child.pid, attempt).then((directory) => {
      maybeDiagnosticDirectory = directory;
      console.error(
        `[stall-diagnostic] attempt ${attempt} exceeded the startup threshold; evidence: ${directory}`,
      );
    }).catch((error) => {
      console.error(
        `[stall-diagnostic] warning: evidence capture failed: ${
          error instanceof Error ? error.message : String(error)
        }`,
      );
    });
  }, options.startupThresholdMs);
  const maybeStopTimer =
    options.maybeStopAfterMs === null
      ? null
      : setTimeout(() => {
          stoppedByLimit = true;
          child.kill("SIGTERM");
        }, options.maybeStopAfterMs);

  let exitStatus = 1;
  try {
    [exitStatus] = await Promise.all([
      child.exited,
      forwardOutput(child.stdout, process.stdout, markOutput),
      forwardOutput(child.stderr, process.stderr, markOutput),
    ]);
  } finally {
    clearTimeout(diagnosticTimer);
    if (maybeStopTimer !== null) {
      clearTimeout(maybeStopTimer);
    }
    signalForwarding.cleanup();
  }
  throwIfCancelled(signalForwarding.maybeSignal());
  await maybeCapturePromise;
  return {
    attempt,
    exitStatus,
    startupMs: maybeStartupMs,
    diagnosticDirectory: maybeDiagnosticDirectory,
    stoppedByLimit,
  };
}

async function runDiagnostic(options: DiagnosticOptions): Promise<number> {
  await requireFilesystemHeadroom(options);
  const executable = await compileTestExecutable(options);
  console.error(`[stall-diagnostic] exact test executable: ${executable}`);
  const attempts: HarnessAttempt[] = [];
  for (let attempt = 1; attempt <= options.attempts; attempt += 1) {
    console.error(`[stall-diagnostic] --list attempt ${attempt}/${options.attempts}`);
    const result = await observeHarnessStartup(executable, attempt, options, (pid, number) =>
      captureHarnessEvidence(options, pid, number),
    );
    attempts.push(result);
    console.error(
      `[stall-diagnostic] attempt ${attempt}: status=${result.exitStatus} startup=${result.startupMs ?? "no output"}ms`,
    );
    if (result.exitStatus !== 0) {
      break;
    }
  }

  const classification = classifyDiagnosticAttempts(attempts, options.startupThresholdMs);
  console.error(`[stall-diagnostic] classification: ${classification}`);

  const summaryDirectory = path.join(options.stateRoot, "stall-diagnostics");
  try {
    await mkdir(summaryDirectory, { recursive: true });
    const summaryPath = path.join(summaryDirectory, "latest-summary.json");
    await writeFile(
      summaryPath,
      `${JSON.stringify(
        {
          schemaVersion: 1,
          generatedAt: new Date().toISOString(),
          packageName: options.packageName,
          targetName: options.targetName,
          targetKind: options.targetKind,
          classification,
          attempts,
        },
        null,
        2,
      )}\n`,
    );
    console.error(`[stall-diagnostic] summary: ${summaryPath}`);
  } catch (error) {
    console.error(
      `[stall-diagnostic] warning: could not write summary: ${
        error instanceof Error ? error.message : String(error)
      }`,
    );
  }
  return attempts.every((attempt) => attempt.exitStatus === 0) ? 0 : 1;
}

async function main(): Promise<number> {
  const options = parseDiagnosticOptions(process.argv.slice(2));
  if (process.env.OPEN_BITCOIN_TIMING_WRAPPED !== "1") {
    const commandTimings = path.join(import.meta.dir, "command-timings.ts");
    const child = Bun.spawn(
      [
        "bun",
        "run",
        commandTimings,
        "run",
        "--key",
        "rust-test-stall-diagnostic",
        "--source",
        "stall-diagnostic",
        "--",
        "bun",
        "run",
        import.meta.path,
        ...process.argv.slice(2),
      ],
      { cwd: options.repoRoot, env: process.env, stderr: "inherit", stdout: "inherit" },
    );
    return child.exited;
  }
  return runDiagnostic(options);
}

export function isDirectScriptInvocation(
  scriptPath: string,
  argv: readonly string[] = process.argv,
): boolean {
  const maybeEntryPath = argv[1];
  return (
    maybeEntryPath !== undefined && path.resolve(maybeEntryPath) === path.resolve(scriptPath)
  );
}

if (isDirectScriptInvocation(import.meta.path)) {
  try {
    process.exitCode = await main();
  } catch (error) {
    console.error(`error: ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = error instanceof DiagnosticCancelled ? error.exitStatus : 2;
  }
}

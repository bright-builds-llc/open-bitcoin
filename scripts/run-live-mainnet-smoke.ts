#!/usr/bin/env bun

import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const BASELINE = "Bitcoin Knots 29.3.knots20260210";
const DEFAULT_OUTPUT_DIR = "packages/target/live-mainnet-smoke-reports";
const DEFAULT_TIMEOUT_SECONDS = 180;
const DEFAULT_POLL_SECONDS = 10;
const DEFAULT_MIN_FREE_GIB = 20;
const REPORT_STEM = "open-bitcoin-live-mainnet-smoke";
const MIN_REASONABLE_UNIX_SECONDS = 1_704_067_200; // 2024-01-01T00:00:00Z
const MAX_TAIL_BYTES = 16 * 1024;

type Options = {
  datadir: string;
  maybeConfigPath: string | null;
  minFreeGib: number;
  outputDir: string;
  pollSeconds: number;
  timeoutSeconds: number;
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
  blockHeight: number;
  capturedAtUnixSeconds: number;
  headerHeight: number;
  lifecycle: string;
  maybeLastError: string | null;
  outboundPeers: number;
  paused: boolean;
  phase: string;
  updatedAtUnixSeconds: number;
};

type ReportStatus = "passed" | "preflight_failed" | "runtime_failed" | "no_progress";

type FinalStatusSummary = {
  blockHeight: number;
  headerHeight: number;
  lifecycle: string;
  maybeLastError: string | null;
  messagesProcessed: number;
  outboundPeers: number;
  phase: string;
};

type SmokeReport = {
  baseline: string;
  commands: {
    daemon: string[];
    finalStatus: string[];
    status: string[];
  };
  daemon: {
    maybeExitCode: number | null;
    maybeSignal: NodeJS.Signals | null;
    stderrTail: string;
    stdoutTail: string;
  };
  final_status: FinalStatusSummary | null;
  generated_at_unix_seconds: number;
  kind: "live_mainnet_smoke";
  options: {
    datadir: string;
    maybeConfigPath: string | null;
    minFreeGib: number;
    outputDir: string;
    pollSeconds: number;
    timeoutSeconds: number;
  };
  preflight: {
    checks: PreflightCheck[];
    passed: boolean;
  };
  result: {
    blockDelta: number;
    headerDelta: number;
    message: string;
    progressDetected: boolean;
    status: ReportStatus;
  };
  schema_version: 1;
  snapshots: SyncStatusSnapshot[];
};

type RuntimeMetadataJson = {
  blocks?: number;
  headers?: number;
  initialblockdownload?: boolean;
  warnings?: string;
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

type DurableStatusJson = {
  maybe_sync_state?: {
    peers?: {
      peer_counts?: FieldAvailability<{
        outbound?: number;
      }>;
    };
    sync?: {
      last_error?: FieldAvailability<string>;
      lifecycle?: FieldAvailability<string>;
      phase?: FieldAvailability<string>;
      sync_progress?: FieldAvailability<{
        block_height?: number;
        header_height?: number;
        messages_processed?: number;
      }>;
    };
  };
};

function usage(): string {
  return `Usage: bun run scripts/run-live-mainnet-smoke.ts --datadir=PATH [--config=PATH] [--output-dir=PATH] [--timeout-seconds=N] [--poll-seconds=N] [--min-free-gib=N]

Launches an explicit opt-in live mainnet smoke flow, polls durable sync status, and writes local JSON/Markdown evidence reports.`;
}

function parseArgs(argv: string[]): Options {
  const options: Options = {
    datadir: "",
    maybeConfigPath: null,
    minFreeGib: DEFAULT_MIN_FREE_GIB,
    outputDir: DEFAULT_OUTPUT_DIR,
    pollSeconds: DEFAULT_POLL_SECONDS,
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

    throw new Error(`unknown argument: ${arg}`);
  }

  if (options.datadir === "") {
    throw new Error("--datadir is required");
  }

  return options;
}

function parsePositiveInteger(value: string, label: string): number {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${label} must be a positive integer`);
  }
  return parsed;
}

function normalizeRelativePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
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
    execFileSync("sh", ["-c", `command -v "${command}" >/dev/null 2>&1`], {
      stdio: "ignore",
    });
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

function ensureBuiltBinaries(repoRootPath: string): void {
  if (
    process.env.OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN !== undefined ||
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
  const decoded = JSON.parse(stdout) as RuntimeMetadataJson;

  return {
    blockHeight: Number(decoded.blocks ?? 0),
    capturedAtUnixSeconds: Math.floor(Date.now() / 1000),
    headerHeight: Number(decoded.headers ?? 0),
    lifecycle: decoded.initialblockdownload === false ? "synced" : "initial_block_download",
    maybeLastError: valueAsNullableString(decoded.warnings),
    outboundPeers: 0,
    paused: false,
    phase: "rpc_getblockchaininfo",
    updatedAtUnixSeconds: Math.floor(Date.now() / 1000),
  };
}

function availableValue<T>(value: FieldAvailability<T> | undefined): T | null {
  if (value !== undefined && value.state === "available") {
    return value.value;
  }
  return null;
}

function valueAsNullableString(value: unknown): string | null {
  return typeof value === "string" && value.trim() !== "" ? value : null;
}

function finalStatusCommand(repoRootPath: string, options: Options): CommandSpec {
  const args = ["--datadir", options.datadir];
  if (options.maybeConfigPath !== null) {
    args.push("--config", options.maybeConfigPath);
  }
  args.push("--format", "json", "sync", "status");
  return {
    command: path.join(
      repoRootPath,
      "packages/target/debug",
      process.platform === "win32" ? "open-bitcoin.exe" : "open-bitcoin",
    ),
    args,
  };
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
  const decoded = JSON.parse(stdout) as DurableStatusJson;
  const maybeSyncState = decoded.maybe_sync_state;
  if (maybeSyncState === undefined) {
    return null;
  }

  const maybeProgress = availableValue(maybeSyncState.sync?.sync_progress);
  const maybePeerCounts = availableValue(maybeSyncState.peers?.peer_counts);
  return {
    blockHeight: Number(maybeProgress?.block_height ?? 0),
    headerHeight: Number(maybeProgress?.header_height ?? 0),
    lifecycle: String(availableValue(maybeSyncState.sync?.lifecycle) ?? "unavailable"),
    maybeLastError: valueAsNullableString(availableValue(maybeSyncState.sync?.last_error)),
    messagesProcessed: Number(maybeProgress?.messages_processed ?? 0),
    outboundPeers: Number(maybePeerCounts?.outbound ?? 0),
    phase: String(availableValue(maybeSyncState.sync?.phase) ?? "unavailable"),
  };
}

function attachTailBuffer(child: ChildProcess, streamName: "stdout" | "stderr"): { read: () => string } {
  let buffer = Buffer.alloc(0);
  const stream = child[streamName];
  stream?.on("data", (chunk: Buffer | string) => {
    const nextChunk = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    buffer = Buffer.concat([buffer, nextChunk]);
    if (buffer.byteLength > MAX_TAIL_BYTES) {
      buffer = buffer.subarray(buffer.byteLength - MAX_TAIL_BYTES);
    }
  });

  return {
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

function markdownReport(report: SmokeReport): string {
  const preflightRows = report.preflight.checks
    .map(
      (check) =>
        `| ${check.name} | ${check.ok ? "passed" : "failed"} | ${escapeTableCell(check.detail)} |`,
    )
    .join("\n");
  const snapshotRows =
    report.snapshots.length === 0
      ? "| - | - | - | - | - | - | - |\n"
      : report.snapshots
          .map(
            (snapshot) =>
              `| ${snapshot.capturedAtUnixSeconds} | ${snapshot.lifecycle} | ${snapshot.phase} | ${snapshot.headerHeight} | ${snapshot.blockHeight} | ${snapshot.outboundPeers} | ${escapeTableCell(snapshot.maybeLastError ?? "-")} |`,
          )
          .join("\n");

  return `# Open Bitcoin Live Mainnet Smoke Report

## Result

- Status: \`${report.result.status}\`
- Message: ${report.result.message}
- Progress detected: ${report.result.progressDetected ? "yes" : "no"}
- Header delta: ${report.result.headerDelta}
- Block delta: ${report.result.blockDelta}

## Options

- Datadir: \`${report.options.datadir}\`
- Config: ${report.options.maybeConfigPath === null ? "Unavailable" : `\`${report.options.maybeConfigPath}\``}
- Output directory: \`${report.options.outputDir}\`
- Timeout: ${report.options.timeoutSeconds}s
- Poll interval: ${report.options.pollSeconds}s
- Minimum free disk floor: ${report.options.minFreeGib} GiB

## Preflight

| Check | Result | Detail |
| --- | --- | --- |
${preflightRows}

## Snapshots

| Captured At | Lifecycle | Phase | Header Height | Block Height | Outbound Peers | Last Error |
| --- | --- | --- | ---: | ---: | ---: | --- |
${snapshotRows}

## Commands

- Daemon: \`${[...report.commands.daemon].join(" ")}\`
- Status: \`${[...report.commands.status].join(" ")}\`
- Final status: \`${[...report.commands.finalStatus].join(" ")}\`

## Final Durable Status

- Lifecycle: ${report.final_status?.lifecycle ?? "Unavailable"}
- Phase: ${report.final_status?.phase ?? "Unavailable"}
- Header height: ${report.final_status?.headerHeight ?? 0}
- Block height: ${report.final_status?.blockHeight ?? 0}
- Messages processed: ${report.final_status?.messagesProcessed ?? 0}
- Outbound peers: ${report.final_status?.outboundPeers ?? 0}
- Last error: ${report.final_status?.maybeLastError ?? "Unavailable"}

## Daemon Output Tail

### stdout

\`\`\`
${report.daemon.stdoutTail.trim()}
\`\`\`

### stderr

\`\`\`
${report.daemon.stderrTail.trim()}
\`\`\`
`;
}

function escapeTableCell(value: string): string {
  return value.replaceAll("|", "\\|").replaceAll("\n", "<br>");
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
    daemon: {
      maybeExitCode: null,
      maybeSignal: null,
      stderrTail: "",
      stdoutTail: "",
    },
    final_status: null,
    generated_at_unix_seconds: Math.floor(Date.now() / 1000),
    kind: "live_mainnet_smoke",
    options: {
      datadir: options.datadir,
      maybeConfigPath: options.maybeConfigPath,
      minFreeGib: options.minFreeGib,
      outputDir: options.outputDir,
      pollSeconds: options.pollSeconds,
      timeoutSeconds: options.timeoutSeconds,
    },
    preflight: {
      checks,
      passed: false,
    },
    result: {
      blockDelta: 0,
      headerDelta: 0,
      message,
      progressDetected: false,
      status: "preflight_failed",
    },
    schema_version: 1,
    snapshots: [],
  };
}

async function main(): Promise<void> {
  const options = parseArgs(process.argv.slice(2));
  const repoRootPath = repoRoot();

  const rpcPort = await findFreePort();
  const daemonSpec = daemonCommand(repoRootPath, options, rpcPort);
  const statusSpec = statusCommand(repoRootPath, options);
  statusSpec.args = [
    "-rpcconnect=127.0.0.1",
    `-rpcport=${rpcPort}`,
    "-rpcuser=smoke",
    "-rpcpassword=smoke",
    "getblockchaininfo",
  ];
  const preflightChecks = buildPreflightChecks(
    repoRootPath,
    options,
    process.env.OPEN_BITCOIN_LIVE_SMOKE_DAEMON_BIN ?? null,
    process.env.OPEN_BITCOIN_LIVE_SMOKE_STATUS_BIN ?? null,
  );
  if (preflightChecks.some((check) => !check.ok)) {
    const report = preflightFailureReport(options, preflightChecks, daemonSpec, statusSpec);
    const { jsonPath, markdownPath } = writeReportFiles(repoRootPath, report);
    console.log(`wrote ${path.relative(repoRootPath, jsonPath)}`);
    console.log(`wrote ${path.relative(repoRootPath, markdownPath)}`);
    throw new Error(report.result.message);
  }

  ensureBuiltBinaries(repoRootPath);

  const postRunStatusSpec = finalStatusCommand(repoRootPath, options);
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
  let maybeLastProbeError: string | null = null;

  try {
    await sleep(2_000);
    const startedAt = Date.now();
    let initialSnapshot: SyncStatusSnapshot | null = null;

    while (Date.now() - startedAt <= options.timeoutSeconds * 1_000) {
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

      headerDelta = snapshot.headerHeight - initialSnapshot.headerHeight;
      blockDelta = snapshot.blockHeight - initialSnapshot.blockHeight;
      if (headerDelta > 0 || blockDelta > 0) {
        resultStatus = "passed";
        resultMessage = `Observed mainnet progress through the daemon status surface (header delta ${headerDelta}, block delta ${blockDelta}).`;
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
  }

  let maybeFinalStatus: FinalStatusSummary | null = null;
  try {
    maybeFinalStatus = readFinalStatus(repoRootPath, postRunStatusSpec);
  } catch {
    maybeFinalStatus = null;
  }

  if (resultStatus === "no_progress") {
    if (maybeFinalStatus?.outboundPeers === 0) {
      resultMessage =
        "No header or block progress was observed before timeout. Final durable sync status still showed 0 outbound peers; check outbound DNS/TCP access or provide explicit peers in open-bitcoin.jsonc.";
    } else if (maybeLastProbeError !== null) {
      resultMessage = `No header or block progress was observed before timeout. Last RPC probe error: ${maybeLastProbeError}`;
    }
  }

  const report: SmokeReport = {
    baseline: BASELINE,
    commands: {
      daemon: [daemonSpec.command, ...daemonSpec.args],
      finalStatus: [postRunStatusSpec.command, ...postRunStatusSpec.args],
      status: [statusSpec.command, ...statusSpec.args],
    },
    daemon: {
      maybeExitCode: child.exitCode,
      maybeSignal: child.signalCode,
      stderrTail: stderrTail.read(),
      stdoutTail: stdoutTail.read(),
    },
    final_status: maybeFinalStatus,
    generated_at_unix_seconds: Math.floor(Date.now() / 1000),
    kind: "live_mainnet_smoke",
    options: {
      datadir: options.datadir,
      maybeConfigPath: options.maybeConfigPath,
      minFreeGib: options.minFreeGib,
      outputDir: options.outputDir,
      pollSeconds: options.pollSeconds,
      timeoutSeconds: options.timeoutSeconds,
    },
    preflight: {
      checks: preflightChecks,
      passed: true,
    },
    result: {
      blockDelta,
      headerDelta,
      message: resultMessage,
      progressDetected: resultStatus === "passed",
      status: resultStatus,
    },
    schema_version: 1,
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

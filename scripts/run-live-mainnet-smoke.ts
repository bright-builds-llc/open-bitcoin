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

type EndpointOutcomeState = "resolved" | "connected" | "handshook" | "failed" | "skipped";
type EndpointOutcomeStage = "preflight" | "runtime";
type EndpointOutcomeSource = "manual_peer" | "dns_seed" | "configured_peer" | "unknown";
type NoProgressCause =
  | "dns_resolution_failure"
  | "tcp_connection_failure"
  | "handshake_failure"
  | "unsupported_peer_capability"
  | "validation_failure"
  | "storage_failure"
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

type FinalStatusSummary = {
  blockHeight: number;
  headerHeight: number;
  lifecycle: string;
  maybeLastError: string | null;
  messagesProcessed: number;
  outboundPeers: number;
  phase: string;
  recentPeers: RuntimePeerTelemetry[];
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
    manualPeers: string[];
    maybeConfigPath: string | null;
    maybeGeneratedConfigPath: string | null;
    minFreeGib: number;
    outputDir: string;
    pollSeconds: number;
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
    headerDelta: number;
    maybeNoProgressCause: NoProgressCause | null;
    message: string;
    nextAction: string;
    progressDetected: boolean;
    status: ReportStatus;
  };
  schema_version: 2;
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
      recent_peers?: FieldAvailability<RuntimePeerTelemetryJson[]>;
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

function usage(): string {
  return `Usage: bun run scripts/run-live-mainnet-smoke.ts --datadir=PATH [--config=PATH] [--manual-peer=HOST[:PORT]]... [--output-dir=PATH] [--timeout-seconds=N] [--poll-seconds=N] [--min-free-gib=N]

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
  const decoded = JSON.parse(stdout) as DurableStatusJson;
  const maybeSyncState = decoded.maybe_sync_state;
  if (maybeSyncState === undefined) {
    return null;
  }

  const maybeProgress = availableValue(maybeSyncState.sync?.sync_progress);
  const maybePeerCounts = availableValue(maybeSyncState.peers?.peer_counts);
  const recentPeers = availableValue(maybeSyncState.peers?.recent_peers)?.map(
    runtimePeerTelemetry,
  ) ?? [];
  return {
    blockHeight: Number(maybeProgress?.block_height ?? 0),
    headerHeight: Number(maybeProgress?.header_height ?? 0),
    lifecycle: String(availableValue(maybeSyncState.sync?.lifecycle) ?? "unavailable"),
    maybeLastError: valueAsNullableString(availableValue(maybeSyncState.sync?.last_error)),
    messagesProcessed: Number(maybeProgress?.messages_processed ?? 0),
    outboundPeers: Number(maybePeerCounts?.outbound ?? 0),
    phase: String(availableValue(maybeSyncState.sync?.phase) ?? "unavailable"),
    recentPeers,
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
  if (
    maybeLastError.includes("invalid") ||
    maybeLastError.includes("validation") ||
    maybeLastError.includes("bad block")
  ) {
    return "validation_failure";
  }
  return null;
}

function nextActionForCause(cause: NoProgressCause | null): string {
  switch (cause) {
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
    case "operator_cancellation":
      return "Review the partial report, then rerun the same command when ready.";
    case "timeout":
      return "Increase --timeout-seconds or use --manual-peer=HOST[:PORT] if endpoint outcomes show reachable peers but no progress yet.";
    case null:
      return "Review the generated report for status snapshots and daemon output.";
  }
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

  return `# Open Bitcoin Live Mainnet Smoke Report

## Result

- Status: \`${report.result.status}\`
- Message: ${report.result.message}
- Progress detected: ${report.result.progressDetected ? "yes" : "no"}
- No-progress cause: ${report.result.maybeNoProgressCause ?? "Unavailable"}
- Next action: ${report.result.nextAction}
- Header delta: ${report.result.headerDelta}
- Block delta: ${report.result.blockDelta}

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

## Runtime Peer Contributions

| Peer | Source | State | Headers Accepted | Blocks Accepted | Last Activity | Failure Reason | Error |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
${runtimePeerRows}

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
      manualPeers: options.manualPeers,
      maybeConfigPath: options.maybeConfigPath,
      maybeGeneratedConfigPath: options.maybeGeneratedConfigPath,
      minFreeGib: options.minFreeGib,
      outputDir: options.outputDir,
      pollSeconds: options.pollSeconds,
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
      headerDelta: 0,
      maybeNoProgressCause: null,
      message,
      nextAction: "Fix the failed local preflight checks, then rerun the live smoke command.",
      progressDetected: false,
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
    process.removeListener("SIGINT", cancellationHandler);
    process.removeListener("SIGTERM", cancellationHandler);
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

  if (resultStatus === "no_progress") {
    const noProgressCause = classifyNoProgressCause(
      endpointOutcomes,
      maybeFinalStatus,
      maybeLastProbeError,
    );
    if (maybeFinalStatus?.outboundPeers === 0) {
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
        ? classifyNoProgressCause(endpointOutcomes, maybeFinalStatus, maybeLastProbeError)
        : resultStatus === "runtime_failed"
          ? noProgressCauseFromFinalStatus(maybeFinalStatus)
          : null;

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
      manualPeers: options.manualPeers,
      maybeConfigPath: options.maybeConfigPath,
      maybeGeneratedConfigPath: options.maybeGeneratedConfigPath,
      minFreeGib: options.minFreeGib,
      outputDir: options.outputDir,
      pollSeconds: options.pollSeconds,
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
      headerDelta,
      maybeNoProgressCause,
      message: resultMessage,
      nextAction: nextActionForCause(maybeNoProgressCause),
      progressDetected: resultStatus === "passed",
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

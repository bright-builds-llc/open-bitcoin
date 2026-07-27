import { execFileSync } from "node:child_process";
import { lookup } from "node:dns/promises";
import { existsSync, readFileSync } from "node:fs";
import { createConnection } from "node:net";
import path from "node:path";
import { parsePeerAddress, parsePositiveInteger } from "./options";
import type { EndpointOutcome, EndpointOutcomeSource, Options, PeerAddress, PreflightCheck } from "./types";

const MIN_REASONABLE_UNIX_SECONDS = 1_704_067_200;
const DEFAULT_NETWORK_PREFLIGHT_TIMEOUT_MS = 1_500;
const DEFAULT_ENDPOINTS_PER_SOURCE = 1;
const DEFAULT_MAINNET_DNS_SEEDS = [
  "seed.bitcoin.sipa.be",
  "dnsseed.bluematt.me",
  "dnsseed.bitcoin.dashjr-list-of-p2p-nodes.us",
  "seed.bitcoinstats.com",
  "seed.bitcoin.jonasschnelli.ch",
];

export function repoRoot(): string {
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

export function buildPreflightChecks(
  repoRootPath: string,
  options: Options,
  daemonOverride: string | null,
  statusOverride: string | null,
): PreflightCheck[] {
  const checks: PreflightCheck[] = [];
  const absoluteDatadir = path.resolve(repoRootPath, options.datadir);
  const clockNowSeconds = Math.floor(Date.now() / 1000);

  checks.push({
    detail: existsSync(absoluteDatadir) ? `datadir exists at ${options.datadir}` : `open-bitcoind mainnet sync activation requires an existing datadir; create ${options.datadir} before running the smoke command.`,
    name: "existing_datadir",
    ok: existsSync(absoluteDatadir),
  });

  if (options.maybeConfigPath !== null) {
    const absoluteConfigPath = path.resolve(repoRootPath, options.maybeConfigPath);
    checks.push({
      detail: existsSync(absoluteConfigPath) ? `config exists at ${options.maybeConfigPath}` : `--config points to a missing file: ${options.maybeConfigPath}`,
      name: "config_path",
      ok: existsSync(absoluteConfigPath),
    });
  }

  checks.push({
    detail: clockNowSeconds >= MIN_REASONABLE_UNIX_SECONDS ? `local clock is plausible (${clockNowSeconds})` : "system clock appears too far behind; sync status and peer handshakes may be misleading until time is corrected.",
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
      detail: availableBytes >= minimumBytes ? `available disk ${(availableBytes / 1024 / 1024 / 1024).toFixed(1)} GiB meets the ${options.minFreeGib} GiB smoke floor` : `available disk ${(availableBytes / 1024 / 1024 / 1024).toFixed(1)} GiB is below the ${options.minFreeGib} GiB smoke floor; free space first or override --min-free-gib for a smaller explicit review run.`,
      name: "disk_space",
      ok: availableBytes >= minimumBytes,
    });
  }

  const daemonCommand = daemonOverride ?? "cargo";
  const statusCommand = statusOverride ?? "cargo";
  checks.push({
    detail: commandExists(daemonCommand) ? `daemon command available: ${daemonCommand}` : `required daemon command not found: ${daemonCommand}`,
    name: "daemon_command",
    ok: commandExists(daemonCommand),
  });
  checks.push({
    detail: commandExists(statusCommand) ? `status command available: ${statusCommand}` : `required status command not found: ${statusCommand}`,
    name: "status_command",
    ok: commandExists(statusCommand),
  });

  return checks;
}

export async function networkPreflightEndpointOutcomes(
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

export function peerSourcesFromOptions(repoRootPath: string, options: Options): PeerSource[] {
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
      } else if (char === '"') {
        inString = false;
      }
      continue;
    }
    if (char === '"') {
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

export function skippedEndpointOutcomes(sources: PeerSource[], reason: string): EndpointOutcome[] {
  return sources.map((source) =>
    endpointOutcome(source, {
      maybeError: reason,
      maybeFailureCause: null,
      maybeResolvedEndpoint: null,
      state: "skipped",
    })
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

export function ensureBuiltBinaries(repoRootPath: string): void {
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

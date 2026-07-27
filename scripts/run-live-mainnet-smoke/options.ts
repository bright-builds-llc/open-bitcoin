import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import type { Options, PeerAddress } from "./types";

const DEFAULT_OUTPUT_DIR = "packages/target/live-mainnet-smoke-reports";
const DEFAULT_TIMEOUT_SECONDS = 180;
const DEFAULT_POLL_SECONDS = 10;
const DEFAULT_MIN_FREE_GIB = 20;
const GENERATED_CONFIG_FILE_NAME = "open-bitcoin-live-mainnet-smoke.jsonc";

export function usage(): string {
  return `Usage: bun run scripts/run-live-mainnet-smoke.ts --datadir=PATH [--config=PATH] [--manual-peer=HOST[:PORT]]... [--output-dir=PATH] [--timeout-seconds=N] [--poll-seconds=N] [--min-free-gib=N] [--restart-after-progress]

Launches an explicit opt-in live mainnet smoke flow, polls durable sync status, and writes local JSON/Markdown evidence reports.`;
}

export function parseArgs(argv: string[]): Options {
  const options: Options = {
    datadir: "",
    manualPeers: [],
    maybeConfigPath: null,
    maybeGeneratedConfigPath: null,
    minFreeGib: DEFAULT_MIN_FREE_GIB,
    outputDir: DEFAULT_OUTPUT_DIR,
    pollSeconds: DEFAULT_POLL_SECONDS,
    restartAfterProgress: false,
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
    if (arg === "--restart-after-progress") {
      options.restartAfterProgress = true;
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

export function parsePeerAddress(value: string): PeerAddress {
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

export function parsePositiveInteger(value: string, label: string): number {
  if (!/^[0-9]+$/.test(value)) {
    throw new Error(`${label} must be a positive integer`);
  }
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${label} must be a positive integer`);
  }
  return parsed;
}

export function normalizeRelativePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
}

export function optionsWithGeneratedManualPeerConfig(repoRootPath: string, options: Options): Options {
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
  return `${
    JSON.stringify(
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
    )
  }\n`;
}

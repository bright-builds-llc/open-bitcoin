import { execFileSync } from "node:child_process";
import path from "node:path";
import { finalStatusSummaryFromMetadata, runtimeMetadataFromStatusResponse, syncStatusSnapshotFromMetadata } from "./status";
import type { CommandSpec, FinalStatusSummary, Options, RuntimeMetadataJson, SyncControlStatusJson, SyncStatusSnapshot } from "./types";

export async function findFreePort(): Promise<number> {
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

export function daemonCommand(repoRootPath: string, options: Options, rpcPort: number): CommandSpec {
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

export function statusCommandForRpcPort(
  repoRootPath: string,
  options: Options,
  rpcPort: number,
): CommandSpec {
  const statusSpec = statusCommand(repoRootPath, options);
  statusSpec.args = [
    "-rpcconnect=127.0.0.1",
    `-rpcport=${rpcPort}`,
    "-rpcuser=smoke",
    "-rpcpassword=smoke",
    "openbitcoinsyncstatus",
  ];
  return statusSpec;
}

const SENSITIVE_COMMAND_ARG_PATTERNS = [
  /^-rpcpassword=/i,
  /^-rpcauth=/i,
  /^-rpc(cookiefile)?=/i,
] as const;

export function reportCommand(commandSpec: CommandSpec): string[] {
  return [commandSpec.command, ...commandSpec.args].map(redactCommandArg);
}

function redactCommandArg(arg: string): string {
  if (SENSITIVE_COMMAND_ARG_PATTERNS.some((pattern) => pattern.test(arg))) {
    return "[redacted]";
  }
  if (/authorization:|authorization=|\bbearer\b|\bbasic\b/i.test(arg)) {
    return "[redacted]";
  }
  return arg;
}

export function finalStatusCommand(repoRootPath: string, options: Options): CommandSpec {
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

export function readSyncStatus(
  repoRootPath: string,
  commandSpec: CommandSpec,
): SyncStatusSnapshot {
  const stdout = execFileSync(commandSpec.command, commandSpec.args, {
    cwd: repoRootPath,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
  const decoded = JSON.parse(stdout) as SyncControlStatusJson | RuntimeMetadataJson;
  return syncStatusSnapshotFromMetadata(runtimeMetadataFromStatusResponse(decoded));
}

export function readFinalStatus(
  repoRootPath: string,
  commandSpec: CommandSpec,
): FinalStatusSummary | null {
  const stdout = execFileSync(commandSpec.command, commandSpec.args, {
    cwd: repoRootPath,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
    stdio: ["ignore", "pipe", "pipe"],
  });
  return finalStatusSummaryFromMetadata(JSON.parse(stdout) as RuntimeMetadataJson);
}

function finalStatusArgs(options: Options): string[] {
  const args = ["--datadir", options.datadir];
  if (options.maybeConfigPath !== null) {
    args.push("--config", options.maybeConfigPath);
  }
  args.push("--format", "json", "sync", "status");
  return args;
}

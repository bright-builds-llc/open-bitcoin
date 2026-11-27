import { randomBytes } from "node:crypto";
import { closeSync, mkdirSync, openSync } from "node:fs";
import { createServer } from "node:net";
import { join } from "node:path";
import { DAEMON_TIMEOUT_MS } from "./process.ts";
import type { RegtestBlock, RegtestTip } from "./regtest-history.ts";

export type DaemonRpc = {
  datadir: string;
  call: (method: string, params?: unknown[]) => Promise<unknown>;
  submit: (block: RegtestBlock) => Promise<RegtestTip>;
};

type DaemonName = "knots" | "open-bitcoin";
const LOOPBACK = "127.0.0.1";
const KNOTS_FLAGS = [
  "-assumevalid=0",
  "-networkactive=0",
  "-listen=0",
  "-listenonion=0",
  "-discover=0",
  "-dnsseed=0",
  "-fixedseeds=0",
  "-natpmp=0",
];
const OPEN_BITCOIN_FLAGS = ["-blockfilterindex=1"];

async function localPort(): Promise<number> {
  const reservation = createServer();
  return new Promise((resolve, reject) => {
    reservation.once("error", reject);
    reservation.listen(0, LOOPBACK, () => {
      const address = reservation.address();
      reservation.close((error) => {
        if (error || !address || typeof address === "string") {
          reject(error ?? new Error("Could not reserve a loopback TCP port"));
          return;
        }
        resolve(address.port);
      });
    });
  });
}

function object(value: unknown): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("Expected a JSON object");
  }
  return value as Record<string, unknown>;
}

function connectRpc(port: number, password: string, deadline: AbortSignal) {
  const url = `http://${LOOPBACK}:${port}/`;
  const authorization = `Basic ${Buffer.from(`cdif01:${password}`).toString("base64")}`;
  return (method: string, params: unknown[] = []) => fetch(url, {
    method: "POST",
    redirect: "error",
    headers: { authorization, "content-type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", id: method, method, params }),
    signal: AbortSignal.any([deadline, AbortSignal.timeout(5_000)]),
  });
}

async function rpcEnvelope(response: Response, method: string) {
  if (!response.ok) {
    await response.body?.cancel();
    throw new Error(`${method} returned HTTP ${response.status}`);
  }
  const envelope = object(await response.json());
  if (envelope.id !== method) throw new Error(`${method} returned the wrong JSON-RPC id`);
  return envelope;
}

function rpcResult(envelope: Record<string, unknown>, method: string): unknown {
  if (envelope.error != null || !Object.hasOwn(envelope, "result")) {
    throw new Error(`${method} did not return a successful JSON-RPC result`);
  }
  return envelope.result;
}

async function ready(child: Bun.Subprocess, rpc: ReturnType<typeof connectRpc>, runDeadline: AbortSignal) {
  const deadline = performance.now() + 30_000;
  let maybeFailure: unknown;
  while (performance.now() < deadline) {
    runDeadline.throwIfAborted();
    if (child.exitCode !== null || child.signalCode !== null) break;
    const maybeResponse = await rpc("getblockchaininfo").catch((error: unknown) => {
      maybeFailure = error;
      return null;
    });
    if (maybeResponse) {
      const envelope = await rpcEnvelope(maybeResponse, "getblockchaininfo");
      const maybeError = envelope.error as { code?: number } | undefined;
      if (maybeError?.code !== -28) return rpcResult(envelope, "getblockchaininfo");
      maybeFailure = new Error("RPC is warming up (JSON-RPC -28)");
    }
    await Bun.sleep(50);
  }
  if (child.exitCode !== null || child.signalCode !== null) {
    throw new Error("Daemon exited before RPC readiness", { cause: maybeFailure });
  }
  throw new DOMException("Daemon did not become RPC-ready within the startup limit", "TimeoutError");
}

function tip(value: unknown, height: number, hash: string, chain: "regtest" | "main") {
  const info = object(value);
  if (info.chain !== chain || info.blocks !== height || info.bestblockhash !== hash) {
    throw new Error(`Unexpected chain tip: ${JSON.stringify({
      network: info.chain, height: info.blocks, hash: info.bestblockhash,
    })}`);
  }
  return { height, hash };
}

async function gracefulExit(child: Bun.Subprocess) {
  if (child.exitCode !== null || child.signalCode !== null) throw new Error("Daemon exited before shutdown");
  child.kill("SIGINT");
  // A plain race with a sleep would keep the runner alive after clean shutdown.
  let maybeTimeout: ReturnType<typeof setTimeout> | undefined;
  try {
    const exit = await Promise.race([
      child.exited,
      new Promise<never>((_resolve, reject) => {
        maybeTimeout = setTimeout(() => reject(new DOMException("Daemon shutdown timed out", "TimeoutError")), 30_000);
      }),
    ]);
    if (exit !== 0) throw new Error(`Daemon shutdown returned exit status ${exit}`);
    return exit;
  } finally {
    clearTimeout(maybeTimeout);
  }
}

export async function compareDaemon<T extends Record<string, unknown>>(
  name: DaemonName,
  executable: string,
  run: string,
  runtimeRoot: string,
  genesisHash: string,
  exercise: (rpc: DaemonRpc) => Promise<T>,
  indexKnots = false,
  maybeStartupTip?: RegtestTip,
  chain: "regtest" | "main" = "regtest",
) {
  // Keep both the home and the datadir fresh so no host configuration or chain can leak in.
  const home = join(runtimeRoot, name);
  const datadir = join(home, "data");
  mkdirSync(datadir, { recursive: true, mode: 0o700 });
  const port = await localPort();
  const password = randomBytes(24).toString("hex");
  const flags = [`-chain=${chain}`, "-server=1", `-rpcbind=${LOOPBACK}`,
    ...(name === "knots" ? KNOTS_FLAGS : []),
    ...(name === "open-bitcoin" || indexKnots ? (chain === "main" ? ["-blockfilterindex=0"] : OPEN_BITCOIN_FLAGS) : [])];
  const log = join(run, `${name}-daemon.log`);
  const fd = openSync(log, "wx", 0o600);
  const signals = ["SIGINT", "SIGTERM"] as const;
  try {
    const args = [executable, ...flags, `-datadir=${datadir}`, `-rpcport=${port}`,
      "-rpcuser=cdif01", `-rpcpassword=${password}`];
    // Knots returns the Darwin soft limit as an int. An inherited RLIM_INFINITY becomes -1.
    const maybeSoftLimit = name === "knots" && process.platform === "darwin" ? 1024 : null;
    const command = maybeSoftLimit === null ? args
      : ["/bin/bash", "-c", 'ulimit -Sn "$1" && shift && exec "$@"', "cdif-01", String(maybeSoftLimit), ...args];
    const child = Bun.spawn(command, {
      cwd: home, env: { HOME: home, TMPDIR: runtimeRoot }, stdin: "ignore", stdout: fd, stderr: fd,
    });
    const interrupt = () => child.kill("SIGKILL");
    for (const signal of signals) process.once(signal, interrupt);
    try {
      const deadline = AbortSignal.timeout(DAEMON_TIMEOUT_MS);
      const rpc = connectRpc(port, password, deadline);
      const call = async (method: string, params?: unknown[]) =>
        rpcResult(await rpcEnvelope(await rpc(method, params), method), method);
      const expectedTip = maybeStartupTip ?? { height: 0, hash: genesisHash };
      const initialTip = tip(await ready(child, rpc, deadline), expectedTip.height, expectedTip.hash, chain);
      const network = object(await call("getnetworkinfo"));
      if (network.connections !== 0 || (name === "knots" && network.networkactive !== false)) {
        throw new Error("Daemon has unexpected P2P activity");
      }
      const submit: DaemonRpc["submit"] = async ({ height, hash, blockHex }) => {
        try {
          const submitted = await call("submitblock", [blockHex]);
          if (submitted !== null) throw new Error(`submitblock returned ${JSON.stringify(submitted)}`);
          return tip(await call("getblockchaininfo"), height, hash, chain);
        } catch (error) {
          const detail = error instanceof Error ? error.message : String(error);
          throw new Error(`${chain === "main" ? "Mainnet" : "Regtest"} height ${height} (${hash}): ${detail}`, { cause: error });
        }
      };
      const outcome = await exercise({ call, submit, datadir });
      const shutdown = await gracefulExit(child);
      deadline.throwIfAborted();
      return {
        flags,
        ...(maybeSoftLimit === null ? {} : { nofile_soft_limit: maybeSoftLimit }),
        initial_tip: initialTip,
        peer_count: network.connections,
        ...outcome,
        exit_status: shutdown,
      };
    } finally {
      for (const signal of signals) process.off(signal, interrupt);
      if (child.exitCode === null && child.signalCode === null) child.kill("SIGKILL");
      await child.exited;
    }
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    throw new Error(`${name}: ${detail}. See ${log}`, { cause: error });
  } finally {
    closeSync(fd);
  }
}

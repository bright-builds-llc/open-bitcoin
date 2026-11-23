import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { buildOpenBitcoin, buildOpenBitcoinCli, sha256File } from "./build.ts";
import { compareDaemon } from "./daemon.ts";
import { qualify } from "./receipt.ts";
import { checkRecoveryReceipt, RECOVERY_FORMAT } from "./recovery.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";
import type { RegtestTip } from "./regtest-history.ts";
import { observeSpends, readRecoveryCorpus } from "./replacement-spends.ts";
import { basicRow, checkWork, observeClosedCoins } from "./state.ts";

function statusCommand(executable: string, config: string, datadir: string, log: string) {
  const args = ["--config", config, "--datadir", datadir, "--network", "regtest", "--format", "json", "sync", "status"];
  const child = spawnSync(executable, args, {
    encoding: "utf8", timeout: 30_000, killSignal: "SIGKILL", maxBuffer: 1_048_576,
  });
  if (child.error) throw child.error;
  if (child.status === null) throw new Error("Status CLI did not return an exit status");
  writeFileSync(log, child.stdout + child.stderr, { flag: "wx", mode: 0o600 });
  return { status: child.status, stdout: child.stdout, stderr: child.stderr };
}

function stoppedStatus(result: ReturnType<typeof statusCommand>, expected: RegtestTip) {
  if (result.status !== 0) throw new Error(`Stopped sync status refused (exit ${result.status})`);
  const status = JSON.parse(result.stdout);
  const progress = status.maybe_sync_state?.sync?.sync_progress?.value;
  const raw = progress?.maybe_validated_active_chain_hash;
  if (status.selected_network !== "regtest"
    || progress?.validated_active_chain_height !== expected.height
    || typeof raw !== "string" || !/^[a-f0-9]{64}$/.test(raw)
    || Buffer.from(raw, "hex").reverse().toString("hex") !== expected.hash
    || status.support?.local_ordinary_implementation !== "bounded_default_regtest"
    || status.basic_index?.safe_durable_height?.value !== expected.height) {
    throw new Error("Stopped sync status lacks the expected proved Regtest tip and BASIC fence");
  }
  return { exit_status: result.status, tip: expected,
    chainwork: checkWork(progress.maybe_validated_active_chain_work), basic_height: expected.height };
}

export async function runRecoveryQualification(root: string, run: string) {
  const readText = (path: string) => readFileSync(join(root, path), "utf8");
  return qualify(run, async (stage) => {
    const corpus = readRecoveryCorpus(readText);
    const { spend, replacement: held, resumed } = corpus;
    const tip = { height: held.end.height, hash: held.end.hash };
    const resumedTip = { height: resumed.height, hash: resumed.hash };
    stage("product_build");
    const daemon = await buildOpenBitcoin(root, run, "CREC-03");
    const cli = await buildOpenBitcoinCli(root, run, "CREC-03");
    const runtime = mkdtempSync(join(realpathSync(tmpdir()), "crec03-"));
    const datadir = join(runtime, "open-bitcoin", "data");
    const config = join(runtime, "open-bitcoin.jsonc");
    const logs = (name: string) => {
      const directory = join(run, name);
      mkdirSync(directory, { recursive: true });
      return directory;
    };
    const runDaemon = (name: string, exercise: Parameters<typeof compareDaemon>[5], maybeStart?: RegtestTip) =>
      compareDaemon("open-bitcoin", daemon.executable, logs(name), runtime, spend.genesisHash, exercise, false, maybeStart);
    async function stopped(name: string, expected: RegtestTip) {
      stage(name);
      const directory = logs(name);
      const status = stoppedStatus(statusCommand(cli.executable, config, datadir, join(directory, "sync-status.log")), expected);
      const coins = await observeClosedCoins(root, datadir, directory, expected, spend.observerSha256, "crec02");
      return { cli: status, ...coins };
    }

    try {
      writeFileSync(config, '{"onboarding":{"wizard_answers":{"network":"regtest"}}}\n', { flag: "wx", mode: 0o600 });
      stage("initial");
      const initial = await runDaemon("initial", async (rpc) => {
        await observeSpends(rpc, spend, spend.runs[0], logs("initial"), "open-bitcoin");
        const active = await observeSpends(rpc, spend, held, logs("initial"), "open-bitcoin");
        const live = statusCommand(cli.executable, config, rpc.datadir, join(logs("initial"), "live-status.log"));
        if (live.status === 0) throw new Error("Offline status accepted a live storage owner");
        const diagnostic = JSON.parse(live.stderr);
        if (diagnostic.error !== "chain_compatibility_refused") throw new Error("Unexpected live owner refusal");
        return { ...active, live_cli: { exit_status: live.status, error: diagnostic.error } };
      });
      const stoppedState = await stopped("stopped", tip);

      stage("restart");
      const restart = await runDaemon("restart", async (rpc) => {
        const duplicate = await rpc.call("submitblock", [held.end.blockHex]);
        const basic = basicRow(tip.hash, await rpc.call("getblockfilter", [tip.hash, "basic"]));
        return { duplicate, final_tip: tip, basic };
      }, tip);
      const reopened = await stopped("reopened", tip);

      stage("continuation");
      const continuation = await runDaemon("continuation", async (rpc) => {
        const final_tip = await rpc.submit(resumed);
        const basic = basicRow(final_tip.hash, await rpc.call("getblockfilter", [final_tip.hash, "basic"]));
        return { submitblock: null, final_tip, basic };
      }, tip);
      const continued = await stopped("continued", resumedTip);
      stage("artifact_identity");
      if (await sha256File(daemon.executable) !== daemon.build.binary_sha256
        || await sha256File(cli.executable) !== cli.build.binary_sha256) {
        throw new Error("An executable changed during recovery qualification");
      }
      return {
        format: RECOVERY_FORMAT, status: "passed", recorded_at: new Date().toISOString(), bun: Bun.version,
        daemon_build: daemon.build, cli_build: cli.build,
        network: "regtest", corpus: REGTEST_CORPUS, corpus_sha256: spend.corpusSha256,
        spend_sha256: spend.spendSha256, resume_sha256: corpus.resumeSha256, resumed_outpoint: corpus.funding,
        open_bitcoin_observer_sha256: spend.observerSha256,
        initial, stopped: stoppedState, restart, reopened, continuation, continued,
      };
    } finally {
      rmSync(runtime, { recursive: true, force: true });
    }
  }, (record) => checkRecoveryReceipt(record, readText), RECOVERY_FORMAT);
}

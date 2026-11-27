#!/usr/bin/env bun

import { mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { BASELINE_COMMIT } from "./check-parity-ledger.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./knots-regtest/build.ts";
import { compareDaemon, type DaemonRpc } from "./knots-regtest/daemon.ts";
import { beginQualification } from "./knots-regtest/qualification-inputs.ts";
import { checkWork, MAINNET_STATE_OBSERVER, observeClosedCoins, observeKnotsCoins } from "./knots-regtest/state.ts";
import { MAINNET_GENESIS, MAINNET_PREFIX_CORPUS, MAINNET_NEXT_BLOCK, PREFIX_LIMIT_REASON, readMainnetPrefix, summarizeMainnetTips } from "./mainnet-prefix.ts";

const LABEL = "MAINNET-PREFIX-02";

async function activeWork(rpc: DaemonRpc): Promise<string> {
  const chain = await rpc.call("getblockchaininfo") as { chainwork?: unknown };
  const network = await rpc.call("getnetworkinfo") as { connections?: unknown };
  if (network.connections !== 0) throw new Error("Unexpected peer during offline Mainnet replay");
  return checkWork(chain.chainwork);
}

async function run() {
  const root = resolve(import.meta.dir, "..");
  const output = join(root, ".local", "open-bitcoin-dev", "mainnet-prefix");
  mkdirSync(output, { recursive: true, mode: 0o700 });
  const run = mkdtempSync(join(output, "run-"));
  console.log(`${LABEL}: evidence directory ${run}`);
  const seal = beginQualification(root);
  const corpus = readMainnetPrefix(root);
  const observerSha256 = await sha256File(join(root, MAINNET_STATE_OBSERVER));
  const knots = await buildKnots(root, run, LABEL);
  const openBitcoin = await buildOpenBitcoin(root, run, LABEL, true);
  const runtimeRoot = mkdtempSync(join(realpathSync(tmpdir()), "open-bitcoin-mainnet-prefix-"));
  try {
    const replay = async (rpc: DaemonRpc, name: string) => {
      const tips = [];
      let previousWork = "";
      for (const block of corpus.blocks) {
        tips.push(await rpc.submit(block));
        if (block.height === corpus.retarget.height - 1) previousWork = await activeWork(rpc);
        if (block.height % 50 === 0) console.log(`${LABEL}: ${name} accepted height ${block.height}`);
      }
      if (previousWork !== corpus.retarget.previous_chainwork) {
        throw new Error(`${name}: chainwork before the retarget differs`);
      }
      return { ...summarizeMainnetTips(tips), final_tip: tips.at(-1), chainwork: await activeWork(rpc), previous_chainwork: previousWork };
    };
    const knotsResult = await compareDaemon("knots", knots.executable, run, runtimeRoot, MAINNET_GENESIS, async (rpc) => {
      const result = await replay(rpc, "Knots");
      const anchor = await rpc.call("getblockheader", [corpus.retarget.anchor_hash]) as { hash?: unknown; height?: unknown; time?: unknown };
      const prior = await rpc.call("getblockheader", [corpus.retarget.previous_hash]) as { bits?: unknown; time?: unknown; hash?: unknown };
      const next = await rpc.call("getblockheader", [corpus.finalTip.hash]) as { bits?: unknown; previousblockhash?: unknown; height?: unknown };
      if (anchor.hash !== corpus.retarget.anchor_hash || anchor.height !== corpus.retarget.anchor_height
        || anchor.time !== corpus.retarget.anchor_time
        || prior.hash !== corpus.retarget.previous_hash || prior.bits !== corpus.retarget.previous_bits
        || prior.time !== corpus.retarget.previous_time || next.bits !== corpus.retarget.bits
        || next.previousblockhash !== corpus.retarget.previous_hash || next.height !== corpus.retarget.height) {
        throw new Error("Knots header observations disagree with the retarget inputs");
      }
      return { ...result, bits: next.bits, previous_bits: prior.bits,
        ...await observeKnotsCoins(rpc, run, corpus.finalTip, "mainnet-prefix-utxos.csv") };
    }, false, undefined, "main");
    const openResult = await compareDaemon("open-bitcoin", openBitcoin.executable, run, runtimeRoot, MAINNET_GENESIS, async (rpc) => {
      const result = await replay(rpc, "Open Bitcoin");
      const rejection = await rpc.call("submitblock", [corpus.nextBlock.blockHex]);
      const after = await rpc.call("getblockchaininfo") as { blocks?: unknown; headers?: unknown; bestblockhash?: unknown };
      if (rejection !== PREFIX_LIMIT_REASON || after.blocks !== corpus.finalTip.height
        || after.headers !== corpus.finalTip.height || after.bestblockhash !== corpus.finalTip.hash
        || await activeWork(rpc) !== result.chainwork) {
        throw new Error("Height-2017 refusal did not preserve the height-2016 state");
      }
      return { ...result, boundary_rejection: rejection };
    }, false, undefined, "main");
    const restart = join(run, "restart");
    mkdirSync(restart, { mode: 0o700 });
    const reopened = await compareDaemon("open-bitcoin", openBitcoin.executable, restart, runtimeRoot,
      MAINNET_GENESIS, async (rpc) => ({ chainwork: await activeWork(rpc) }), false, corpus.finalTip, "main");
    const closed = await observeClosedCoins(root, join(runtimeRoot, "open-bitcoin", "data"),
      run, corpus.finalTip, observerSha256, "mainnet_prefix");
    for (const observed of [knotsResult, openResult, reopened, closed]) {
      if (observed.chainwork !== corpus.chainwork) throw new Error("Mainnet chainwork differs");
    }
    if (closed.bits !== corpus.retarget.bits) throw new Error("Reopened Mainnet target differs from Knots");
    if (!isDeepStrictEqual(closed.utxos, knotsResult.utxos)) throw new Error("Mainnet UTXOs differ after reopen");
    if (await sha256File(knots.executable) !== knots.binarySha256
      || await sha256File(openBitcoin.executable) !== openBitcoin.build.binary_sha256) {
      throw new Error("A daemon executable changed during qualification");
    }
    const receipt = seal({
      format: "open-bitcoin-mainnet-prefix-v2", status: "passed", recorded_at: new Date().toISOString(),
      network: "mainnet", network_enabled: false, corpus: MAINNET_PREFIX_CORPUS, corpus_sha256: corpus.corpusSha256,
      boundary_input: MAINNET_NEXT_BLOCK, boundary_sha256: corpus.boundarySha256, retarget: corpus.retarget,
      source_commit: BASELINE_COMMIT, source_tree: knots.sourceTree, binary_sha256: knots.binarySha256,
      knots_build: knots.build, open_bitcoin_build: openBitcoin.build, open_bitcoin_observer_sha256: observerSha256,
      knots: knotsResult, open_bitcoin: { ...openResult, reopened, bits: closed.bits, utxos: closed.utxos },
    });
    const pending = join(run, "receipt.tmp");
    const destination = join(run, "receipt.json");
    writeFileSync(pending, `${JSON.stringify(receipt, null, 2)}\n`, { flag: "wx", mode: 0o600 });
    renameSync(pending, destination);
    console.log(`${LABEL}: passed. Receipt: ${destination}`);
  } finally {
    rmSync(runtimeRoot, { recursive: true, force: true });
  }
}

if (import.meta.main) {
  run().catch((error: unknown) => {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  });
}

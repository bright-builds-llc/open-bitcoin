#!/usr/bin/env bun

import { mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { BASELINE_COMMIT } from "./check-parity-ledger.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./knots-regtest/build.ts";
import { compareDaemon } from "./knots-regtest/daemon.ts";
import { readRegtestCase, REGTEST_CORPUS } from "./knots-regtest/regtest-case.ts";
import {
  readRegtestHistory,
  summarizeAcceptedTips,
  type RegtestBlock,
  type RegtestTip,
} from "./knots-regtest/regtest-history.ts";

type Submit = (block: RegtestBlock) => Promise<RegtestTip>;

function singleBlockCase(readText: (path: string) => string) {
  const block = readRegtestCase(readText);
  return {
    genesisHash: block.genesisHash,
    corpusSha256: block.corpusSha256,
    inputFacts: { block_sha256: block.blockSha256 },
    exercise: async (submit: Submit) => ({
      submitblock: null,
      final_tip: await submit({ height: 1, hash: block.blockHash, blockHex: block.blockHex }),
    }),
  };
}

function maturityHalvingCase(readText: (path: string) => string) {
  const history = readRegtestHistory(readText);
  return {
    genesisHash: history.genesisHash,
    corpusSha256: history.corpusSha256,
    inputFacts: history.monetaryFacts,
    exercise: async (submit: Submit) => {
      const tips: RegtestTip[] = [];
      for (const block of history.blocks) {
        const tip = await submit(block);
        tips.push(tip);
        if (tip.height % 50 === 0) console.log(`CDIF-02: accepted through height ${tip.height}`);
      }
      return { ...summarizeAcceptedTips(tips), final_tip: tips.at(-1) };
    },
  };
}

async function runDifferential() {
  const args = process.argv.slice(2);
  const maturityAndHalvings = args.length === 1 && args[0] === "--cdif-02";
  if (args.length && !maturityAndHalvings) {
    throw new Error("Usage: bun run scripts/run-knots-regtest-differential.ts [--cdif-02]");
  }
  const id = maturityAndHalvings ? "cdif-02" : "cdif-01";
  const label = id.toUpperCase();
  const root = resolve(import.meta.dir, "..");
  const readText = (path: string) => readFileSync(join(root, path), "utf8");
  const scenario = maturityAndHalvings ? maturityHalvingCase(readText) : singleBlockCase(readText);
  const output = join(root, ".local", "open-bitcoin-dev", id);
  mkdirSync(output, { recursive: true, mode: 0o700 });
  const run = mkdtempSync(join(output, "run-"));
  console.log(`${label}: evidence directory ${run}`);

  const knots = await buildKnots(root, run, label);
  const openBitcoin = await buildOpenBitcoin(root, run, label);
  const runtimeRoot = mkdtempSync(join(realpathSync(tmpdir()), `open-bitcoin-${id}-`));
  try {
    console.log(`${label}: submitting to Knots`);
    const knotsResult = await compareDaemon("knots", knots.executable, run, runtimeRoot, scenario.genesisHash, scenario.exercise);
    console.log(`${label}: submitting to Open Bitcoin`);
    const openBitcoinResult = await compareDaemon("open-bitcoin", openBitcoin.executable, run, runtimeRoot, scenario.genesisHash, scenario.exercise);
    if (await sha256File(knots.executable) !== knots.binarySha256
      || await sha256File(openBitcoin.executable) !== openBitcoin.build.binary_sha256) {
      throw new Error("A daemon executable changed during qualification");
    }

    const receipt = {
      format: `open-bitcoin-${id}-v1`,
      status: "passed",
      recorded_at: new Date().toISOString(),
      bun: Bun.version,
      source_commit: BASELINE_COMMIT,
      source_tree: knots.sourceTree,
      binary_sha256: knots.binarySha256,
      knots_build: knots.build,
      open_bitcoin_build: openBitcoin.build,
      network: "regtest",
      corpus: REGTEST_CORPUS,
      corpus_sha256: scenario.corpusSha256,
      ...scenario.inputFacts,
      knots: knotsResult,
      open_bitcoin: openBitcoinResult,
    };
    const pending = join(run, "receipt.tmp");
    const destination = join(run, "receipt.json");
    writeFileSync(pending, `${JSON.stringify(receipt, null, 2)}\n`, { flag: "wx", mode: 0o600 });
    renameSync(pending, destination);
    console.log(`${label}: passed. Receipt: ${destination}`);
  } finally {
    rmSync(runtimeRoot, { recursive: true, force: true });
  }
}

if (import.meta.main) {
  runDifferential().catch((error: unknown) => {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  });
}

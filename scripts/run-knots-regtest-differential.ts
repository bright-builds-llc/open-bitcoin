#!/usr/bin/env bun

import { mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { runBranchQualification } from "./knots-regtest/branch-qualification.ts";
import { runQualification } from "./knots-regtest/qualification.ts";
import { BASELINE_COMMIT } from "./check-parity-ledger.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./knots-regtest/build.ts";
import { compareDaemon, type DaemonRpc } from "./knots-regtest/daemon.ts";
import { readRegtestCase, REGTEST_CORPUS } from "./knots-regtest/regtest-case.ts";
import {
  readRegtestHistory,
  summarizeAcceptedTips,
  type RegtestTip,
} from "./knots-regtest/regtest-history.ts";

import { observeRejections, readRejectionCorpus, REJECTION_NORMALIZATION } from "./knots-regtest/rejections.ts";
import { observeClosedState, observeLiveState, readStateCorpus, verifyState } from "./knots-regtest/state.ts";

function singleBlockCase(readText: (path: string) => string) {
  const block = readRegtestCase(readText);
  return {
    genesisHash: block.genesisHash,
    corpusSha256: block.corpusSha256,
    inputFacts: { block_sha256: block.blockSha256 },
    exercise: async ({ submit }: DaemonRpc) => ({
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
    exercise: async ({ submit }: DaemonRpc) => {
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
  const id = args[0]?.slice(2) ?? "cdif-01";
  if (args.length && (args.length !== 1 || !["--cdif-02", "--cdif-03", "--cdif-04", "--cdif-05", "--crec-01"].includes(args[0]))) {
    throw new Error("Usage: bun run scripts/run-knots-regtest-differential.ts [--cdif-02 | --cdif-03 | --cdif-04 | --cdif-05 | --crec-01]");
  }
  const label = id.toUpperCase();
  const root = resolve(import.meta.dir, "..");
  const output = join(root, ".local", "open-bitcoin-dev", id);
  mkdirSync(output, { recursive: true, mode: 0o700 });
  const run = mkdtempSync(join(output, "run-"));
  console.log(`${label}: evidence directory ${run}`);

  if (id === "cdif-05") {
    const receipt = await runQualification(root, run);
    console.log(`${label}: passed. Receipt: ${receipt}`);
    return;
  }
  if (id === "crec-01") {
    const receipt = await runBranchQualification(root, run);
    console.log(`${label}: passed. Receipt: ${receipt}`);
    return;
  }
  const readText = (path: string) => readFileSync(join(root, path), "utf8");
  const maybeRejections = id === "cdif-03" ? readRejectionCorpus(readText) : null;
  const maybeState = id === "cdif-04" ? readStateCorpus(readText) : null;
  const scenario = id === "cdif-02" ? maturityHalvingCase(readText) : singleBlockCase(readText);

  const knots = await buildKnots(root, run, label);
  const openBitcoin = await buildOpenBitcoin(root, run, label);
  const runtimeRoot = mkdtempSync(join(realpathSync(tmpdir()), `open-bitcoin-${id}-`));
  try {
    async function runDaemon(name: "knots" | "open-bitcoin", executable: string) {
      console.log(`${label}: submitting to ${name}`);
      if (maybeState) {
        const online = await compareDaemon(name, executable, run, runtimeRoot, maybeState.genesisHash,
          (rpc) => observeLiveState(rpc, maybeState, run, name), true);
        const state = name === "knots" ? online : {
          ...online, ...await observeClosedState(root, join(runtimeRoot, name, "data"), run, maybeState),
        };
        verifyState(state, maybeState, name);
        return state;
      }
      if (!maybeRejections) return compareDaemon(name, executable, run, runtimeRoot, scenario.genesisHash, scenario.exercise);
      const runs = [];
      for (const group of maybeRejections.runs) {
        const logRoot = join(run, group.name);
        mkdirSync(logRoot, { recursive: true });
        const result = await compareDaemon(name, executable, logRoot, join(runtimeRoot, group.name),
          maybeRejections.genesisHash, (rpc) => observeRejections(rpc, group), true);
        runs.push({ name: group.name, ...result });
      }
      return { runs };
    }
    const knotsResult = await runDaemon("knots", knots.executable);
    const openBitcoinResult = await runDaemon("open-bitcoin", openBitcoin.executable);
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
      corpus_sha256: (maybeState ?? maybeRejections ?? scenario).corpusSha256,
      ...(maybeRejections ? { normalization: REJECTION_NORMALIZATION } : maybeState ? {
        expectation_sha256: maybeState.expectationSha256,
        open_bitcoin_observer_sha256: maybeState.observerSha256,
      } : scenario.inputFacts),
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

#!/usr/bin/env bun

import { mkdirSync, mkdtempSync, renameSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { BASELINE_COMMIT } from "./check-parity-ledger.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./knots-regtest/build.ts";
import { compareDaemon } from "./knots-regtest/daemon.ts";
import { loadRegtestCase, REGTEST_CORPUS } from "./knots-regtest/regtest-case.ts";

async function runDifferential() {
  if (process.argv.length !== 2) throw new Error("Usage: bun run scripts/run-knots-regtest-differential.ts");
  const root = resolve(import.meta.dir, "..");
  const singleBlock = loadRegtestCase(root);
  const output = join(root, ".local", "open-bitcoin-dev", "cdif-01");
  mkdirSync(output, { recursive: true, mode: 0o700 });
  const run = mkdtempSync(join(output, "run-"));
  console.log(`CDIF-01: evidence directory ${run}`);

  const knots = await buildKnots(root, run);
  const openBitcoin = await buildOpenBitcoin(root, run);
  const knotsResult = await compareDaemon("knots", knots.executable, run, singleBlock);
  console.log(`CDIF-01: Knots accepted ${knotsResult.final_tip.hash}`);
  const openBitcoinResult = await compareDaemon("open-bitcoin", openBitcoin.executable, run, singleBlock);
  console.log(`CDIF-01: Open Bitcoin accepted ${openBitcoinResult.final_tip.hash}`);
  if (await sha256File(knots.executable) !== knots.binarySha256
    || await sha256File(openBitcoin.executable) !== openBitcoin.build.binary_sha256) {
    throw new Error("A daemon executable changed during qualification");
  }

  const receipt = {
    format: "open-bitcoin-cdif-01-v1",
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
    corpus_sha256: singleBlock.corpusSha256,
    block_sha256: singleBlock.blockSha256,
    knots: knotsResult,
    open_bitcoin: openBitcoinResult,
  };
  const pending = join(run, "receipt.tmp");
  const destination = join(run, "receipt.json");
  writeFileSync(pending, `${JSON.stringify(receipt, null, 2)}\n`, { flag: "wx", mode: 0o600 });
  renameSync(pending, destination);
  console.log(`CDIF-01: passed. Receipt: ${destination}`);
}

if (import.meta.main) {
  runDifferential().catch((error: unknown) => {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  });
}

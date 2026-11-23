import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { BASELINE_COMMIT, checkSpendRecord } from "../check-parity-ledger.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./build.ts";
import { compareDaemon } from "./daemon.ts";
import { qualify } from "./receipt.ts";
import { beginQualification } from "./qualification-inputs.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";
import { observeSpends, readSpendCorpus, SPEND_FORMAT } from "./replacement-spends.ts";
import { observeClosedCoins } from "./state.ts";

export async function runSpendQualification(root: string, run: string) {
  const readTextFile = (path: string) => readFileSync(join(root, path), "utf8");
  let checkoutTree = "";
  return qualify(run, async (stage) => {
    const seal = beginQualification(root);
    const corpus = readSpendCorpus(readTextFile);
    stage("knots_build");
    const knots = await buildKnots(root, run, "CREC-02");
    checkoutTree = knots.sourceTree;
    stage("open_bitcoin_build");
    const openBitcoin = await buildOpenBitcoin(root, run, "CREC-02");
    const runtime = mkdtempSync(join(realpathSync(tmpdir()), "crec02-"));
    async function runDaemon(name: "knots" | "open-bitcoin", executable: string) {
      const runs = [];
      for (const plan of corpus.runs) {
        stage(`${name}/${plan.name}`);
        console.log(`CREC-02: ${name} ${plan.name}`);
        const logs = join(run, plan.name);
        mkdirSync(logs, { recursive: true });
        const observed = await compareDaemon(name, executable, logs, runtime, corpus.genesisHash,
          (rpc) => observeSpends(rpc, corpus, plan, logs, name), true, plan.start);
        if (name === "knots") {
          runs.push(observed);
        } else {
          stage(`${name}/${plan.name}/closed-state`);
          const coins = await observeClosedCoins(root, join(runtime, name, "data"), logs, {
            height: plan.end.height, hash: plan.end.hash,
          }, corpus.observerSha256, "crec02");
          runs.push({ ...observed, ...coins });
        }
      }
      return { runs };
    }
    try {
      const knotsResult = await runDaemon("knots", knots.executable);
      const openResult = await runDaemon("open-bitcoin", openBitcoin.executable);
      stage("artifact_identity");
      if (await sha256File(knots.executable) !== knots.binarySha256
        || await sha256File(openBitcoin.executable) !== openBitcoin.build.binary_sha256) {
        throw new Error("A daemon executable changed during spend qualification");
      }
      stage("input_binding");
      return seal({
        format: SPEND_FORMAT, status: "passed",
        recorded_at: new Date().toISOString(), bun: Bun.version,
        source_commit: BASELINE_COMMIT, source_tree: knots.sourceTree, binary_sha256: knots.binarySha256,
        knots_build: knots.build, open_bitcoin_build: openBitcoin.build,
        network: "regtest", corpus: REGTEST_CORPUS, corpus_sha256: corpus.corpusSha256,
        spend_sha256: corpus.spendSha256, open_bitcoin_observer_sha256: corpus.observerSha256, spend_facts: corpus.spendFacts,
        knots: knotsResult, open_bitcoin: openResult,
      });
    } finally {
      rmSync(runtime, { recursive: true, force: true });
    }
  }, (record) => checkSpendRecord(record, {
    readTextFile, baseline: { gitlinkCommit: BASELINE_COMMIT, checkoutCommit: BASELINE_COMMIT, checkoutTree },
  }), SPEND_FORMAT);
}

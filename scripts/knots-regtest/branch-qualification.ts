import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { BASELINE_COMMIT, checkBranchRecord } from "../check-parity-ledger.ts";
import { BRANCH_FORMAT, observeBranchSchedule, observeRestart, readBranches } from "./branches.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./build.ts";
import { compareDaemon } from "./daemon.ts";
import { qualify } from "./receipt.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";

export async function runBranchQualification(root: string, run: string) {
  const readTextFile = (path: string) => readFileSync(join(root, path), "utf8");
  let checkoutTree = "";
  return qualify(run, async (stage) => {
    const corpus = readBranches(readTextFile);
    stage("knots_build");
    const knots = await buildKnots(root, run, "CREC-01");
    checkoutTree = knots.sourceTree;
    stage("open_bitcoin_build");
    const openBitcoin = await buildOpenBitcoin(root, run, "CREC-01");
    const runtime = mkdtempSync(join(realpathSync(tmpdir()), "crec01-"));
    async function runDaemon(name: "knots" | "open-bitcoin", executable: string) {
      const runs = [];
      for (const schedule of corpus.runs) {
        stage(`${name}/${schedule.name}`);
        const logs = join(run, schedule.name);
        mkdirSync(logs, { recursive: true });
        runs.push(await compareDaemon(name, executable, logs, join(runtime, schedule.name), corpus.genesisHash,
          (rpc) => observeBranchSchedule(rpc, schedule)));
      }
      stage(`${name}/restart`);
      const logs = join(run, "restart");
      mkdirSync(logs, { recursive: true });
      const restart = await compareDaemon(name, executable, logs, join(runtime, "restart-tie"), corpus.genesisHash,
        (rpc) => observeRestart(rpc, corpus, name), false, corpus.blocks.a2);
      return { runs, restart };
    }
    try {
      const knotsResult = await runDaemon("knots", knots.executable);
      const openResult = await runDaemon("open-bitcoin", openBitcoin.executable);
      stage("artifact_identity");
      if (await sha256File(knots.executable) !== knots.binarySha256
        || await sha256File(openBitcoin.executable) !== openBitcoin.build.binary_sha256) {
        throw new Error("A daemon executable changed during branch qualification");
      }
      return {
        format: BRANCH_FORMAT, status: "passed",
        recorded_at: new Date().toISOString(), bun: Bun.version,
        source_commit: BASELINE_COMMIT, source_tree: knots.sourceTree, binary_sha256: knots.binarySha256,
        knots_build: knots.build, open_bitcoin_build: openBitcoin.build,
        network: "regtest", corpus: REGTEST_CORPUS, corpus_sha256: corpus.corpusSha256,
        branch_sha256: corpus.branchSha256, knots: knotsResult, open_bitcoin: openResult,
      };
    } finally {
      rmSync(runtime, { recursive: true, force: true });
    }
  }, (record) => checkBranchRecord(record, {
    readTextFile, baseline: { gitlinkCommit: BASELINE_COMMIT, checkoutCommit: BASELINE_COMMIT, checkoutTree },
  }), BRANCH_FORMAT);
}

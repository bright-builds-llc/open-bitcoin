import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { BASELINE_COMMIT, checkQualificationRecord } from "../check-parity-ledger.ts";
import { buildKnots, buildOpenBitcoin, sha256File } from "./build.ts";
import { compareDaemon } from "./daemon.ts";
import { qualify, QUALIFICATION_FORMAT } from "./receipt.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";
import { observeRejections, readRejectionCorpus, REJECTION_NORMALIZATION } from "./rejections.ts";
import { observeClosedState, observeStateAtTip, readStateCorpus } from "./state.ts";

export async function runQualification(root: string, run: string) {
  const readTextFile = (path: string) => {
    const source = join(root, path);
    if (statSync(source).size > 16 * 1024 * 1024) {
      throw new Error(`Qualification input exceeds 16 MiB: ${path}`);
    }
    return readFileSync(source, "utf8");
  };
  let checkoutTree = "";
  return qualify(run, async (stage) => {
    const rejections = readRejectionCorpus(readTextFile);
    const state = readStateCorpus(readTextFile);
    stage("knots_build");
    const knots = await buildKnots(root, run, "CDIF-05");
    checkoutTree = knots.sourceTree;
    stage("open_bitcoin_build");
    const openBitcoin = await buildOpenBitcoin(root, run, "CDIF-05");
    const runtime = mkdtempSync(join(realpathSync(tmpdir()), "open-bitcoin-cdif-05-"));

    async function runDaemon(name: "knots" | "open-bitcoin", executable: string) {
      const runs = [];
      for (const group of rejections.runs) {
        stage(`${name}/${group.name}`);
        console.log(`CDIF-05: ${name} ${group.name}`);
        const logs = join(run, group.name);
        const runtimeGroup = join(runtime, group.name);
        mkdirSync(logs, { recursive: true });
        const completesState = group.blocks.length === state.expected.tip.height;
        const observed = await compareDaemon(name, executable, logs, runtimeGroup, rejections.genesisHash,
          async (rpc) => {
            const accepted: string[] = [];
            const result = await observeRejections({ ...rpc, submit: async (block) => {
              const tip = await rpc.submit(block);
              accepted.push(tip.hash);
              if (tip.height % 50 === 0) console.log(`CDIF-05: ${name} accepted height ${tip.height}`);
              return tip;
            } }, group);
            const finalTip = result.final_tip;
            if (!finalTip) throw new Error("Run did not reach a final tip");
            return {
              name: group.name, ...result, accepted,
              ...(completesState ? await observeStateAtTip(rpc, state, run, name, finalTip) : {}),
            };
          }, true);
        if (name === "open-bitcoin" && completesState) {
          stage("open-bitcoin/closed-state");
          const closed = await observeClosedState(root, join(runtimeGroup, name, "data"), run, state);
          runs.push({ ...observed, ...closed });
        } else {
          runs.push(observed);
        }
      }
      return { runs };
    }

    try {
      const knotsResult = await runDaemon("knots", knots.executable);
      const openBitcoinResult = await runDaemon("open-bitcoin", openBitcoin.executable);
      stage("artifact_identity");
      if (await sha256File(knots.executable) !== knots.binarySha256
        || await sha256File(openBitcoin.executable) !== openBitcoin.build.binary_sha256) {
        throw new Error("A daemon executable changed during qualification");
      }
      return {
        format: QUALIFICATION_FORMAT, status: "passed",
        recorded_at: new Date().toISOString(), bun: Bun.version,
        source_commit: BASELINE_COMMIT, source_tree: knots.sourceTree, binary_sha256: knots.binarySha256,
        knots_build: knots.build, open_bitcoin_build: openBitcoin.build,
        network: "regtest", corpus: REGTEST_CORPUS, corpus_sha256: rejections.corpusSha256,
        normalization: REJECTION_NORMALIZATION, expectation_sha256: state.expectationSha256,
        open_bitcoin_observer_sha256: state.observerSha256,
        knots: knotsResult, open_bitcoin: openBitcoinResult,
      };
    } finally {
      rmSync(runtime, { recursive: true, force: true });
    }
  }, (record) => checkQualificationRecord(record, {
    readTextFile, baseline: { gitlinkCommit: BASELINE_COMMIT, checkoutCommit: BASELINE_COMMIT, checkoutTree },
  }));
}

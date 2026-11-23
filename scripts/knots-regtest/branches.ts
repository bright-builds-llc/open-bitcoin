import { decodeBlock, sha256 } from "../phase167-historical-corpus.ts";
import {
  BITS, ZERO, blockHash, display, encodeBlock, heightScript, proof, seal, type Body,
} from "../phase167-regtest-serialization.ts";
import type { DaemonRpc } from "./daemon.ts";
import { readRegtestCase } from "./regtest-case.ts";
import type { RegtestBlock, RegtestTip } from "./regtest-history.ts";

export const BRANCH_FORMAT = "open-bitcoin-crec-01-v1";
export const BRANCH_CLASSIFICATION = "reference_ambiguous_equal_work_equal_disk_sequence";

const SCHEDULES = {
  "a-first": [["common", "common"], ["a2", "a2"], ["b2", "a2"], ["b3", "b3"], ["a3", "b3"], ["a4", "a4"]],
  "b-first": [["common", "common"], ["b2", "b2"], ["a2", "b2"], ["a3", "a3"], ["b3", "a3"], ["b4", "b4"]],
  "restart-tie": [["common", "common"], ["a2", "a2"], ["b2", "a2"], ["c2", "a2"]],
} as const;
type BranchName = keyof typeof SCHEDULES;
type BlockId = typeof SCHEDULES[BranchName][number][number];

function forkBlock(parent: RegtestBlock & { time: number }, tag: string) {
  const height = parent.height + 1;
  const time = parent.time + 600;
  const body: Body = seal({
    header: { version: 4, parent: display(parent.hash), time, bits: BITS, merkle: ZERO, nonce: 0 },
    transactions: [{
      version: 1, lock_time: height,
      inputs: [{ txid: ZERO, vout: 0xffffffff, script: heightScript(height) + tag, sequence: 0xffffffff, witness: [] }],
      outputs: [{ sats: "5000000000", script: "51" }],
    }],
  });
  return { height, time, hash: display(blockHash(body)), blockHex: encodeBlock(body).toString("hex") };
}

export function readBranches(readText: (path: string) => string) {
  const seed = readRegtestCase(readText);
  const header = decodeBlock(Buffer.from(seed.blockHex, "hex"), seed.blockHash);
  if (header.bits !== BITS) throw new Error("Expected default-Regtest proof target");
  const common = { height: 1, hash: seed.blockHash, blockHex: seed.blockHex, time: header.time };
  const a2 = forkBlock(common, "51");
  const b2 = forkBlock(common, "52");
  const c2 = forkBlock(common, "53");
  const a3 = forkBlock(a2, "51");
  const b3 = forkBlock(b2, "52");
  const a4 = forkBlock(a3, "51");
  const b4 = forkBlock(b3, "52");
  const blocks: Record<BlockId, RegtestBlock> = { common, a2, b2, c2, a3, b3, a4, b4 };
  if (new Set(Object.values(blocks).map(({ hash }) => hash)).size !== Object.keys(blocks).length) {
    throw new Error("Regtest branch blocks must have distinct identities");
  }
  return {
    genesisHash: seed.genesisHash,
    corpusSha256: seed.corpusSha256,
    branchSha256: sha256(JSON.stringify(Object.entries(blocks).map(([id, block]) => [id, block.blockHex]))),
    tiedChainwork: ((BigInt(a2.height) + 1n) * proof(BITS)).toString(16).padStart(64, "0"),
    blocks,
    runs: (Object.keys(SCHEDULES) as BranchName[]).map((name) => ({
      name, steps: SCHEDULES[name].map(([submit, active]) => ({ submit, active, block: blocks[submit], tip: blocks[active] })),
    })),
  };
}

export type BranchCorpus = ReturnType<typeof readBranches>;

function tipOf(value: unknown): RegtestTip {
  const info = value as { chain?: unknown; blocks?: unknown; bestblockhash?: unknown } | null;
  if (info?.chain !== "regtest" || typeof info.blocks !== "number" || !Number.isInteger(info.blocks)
    || typeof info.bestblockhash !== "string" || !/^[0-9a-f]{64}$/.test(info.bestblockhash)) {
    throw new Error("Invalid Regtest best-block response");
  }
  return { height: info.blocks, hash: info.bestblockhash };
}

export async function observeBranchSchedule(rpc: DaemonRpc, run: BranchCorpus["runs"][number]) {
  const steps = [];
  let maybeTip: RegtestTip | undefined;
  for (const step of run.steps) {
    const result = await rpc.call("submitblock", [step.block.blockHex]).catch((error: unknown) => {
      throw new Error(`${run.name}: submitblock failed at ${step.submit}`, { cause: error });
    });
    const active = tipOf(await rpc.call("getblockchaininfo"));
    const activates = step.submit === step.active;
    if (result !== (activates ? null : "inconclusive")
      || active.height !== step.tip.height || active.hash !== step.tip.hash) {
      throw new Error(`${run.name} ${step.submit}: unexpected submit result/tip ${JSON.stringify({ result, active })}`);
    }
    steps.push({ submitted: step.submit, hash: step.block.hash, submitblock: result, active });
    maybeTip = active;
  }
  if (!maybeTip) throw new Error("Branch schedule did not observe a tip");
  return { name: run.name, steps, final_tip: maybeTip };
}

async function currentTip(rpc: DaemonRpc) {
  return tipOf(await rpc.call("getblockchaininfo"));
}

export async function observeRestart(rpc: DaemonRpc, corpus: BranchCorpus, name: "knots" | "open-bitcoin") {
  const retained = await currentTip(rpc);
  if (retained.hash !== corpus.blocks.a2.hash || retained.height !== corpus.blocks.a2.height) {
    throw new Error(`${name}: restart lost the persisted active tip`);
  }
  if (name === "open-bitcoin") return { retained_tip: retained, final_tip: retained };

  // In this disposable Knots-only probe, exclude the active branch after reload.
  // The two remaining block bodies have equal work and both have the disk sequence ID.
  const alternatives = [corpus.blocks.b2.hash, corpus.blocks.c2.hash].sort();
  const tips = await rpc.call("getchaintips") as { hash: string; height: number }[] | null;
  if (!Array.isArray(tips) || alternatives.some((hash) =>
    !tips.some((tip) => tip.hash === hash && tip.height === retained.height))) {
    throw new Error("Knots did not reload both equal-work side branches");
  }
  if (await rpc.call("invalidateblock", [retained.hash]) !== null) throw new Error("Knots invalidation probe failed");
  const selectedInfo = await rpc.call("getblockchaininfo");
  const final = tipOf(selectedInfo);
  const work = (selectedInfo as { chainwork?: unknown }).chainwork;
  if (final.height !== retained.height || !alternatives.includes(final.hash) || work !== corpus.tiedChainwork) {
    throw new Error("Knots selected outside the equal-work restart candidate set");
  }
  return { retained_tip: retained, final_tip: final, ambiguous: {
    classification: BRANCH_CLASSIFICATION, candidates: alternatives, selected: final.hash, chainwork: work,
  } };
}

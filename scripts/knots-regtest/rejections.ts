import { isDeepStrictEqual } from "node:util";
import { decodeHeader, sha256 } from "../phase167-historical-corpus.ts";
import { REGTEST_CORPUS } from "./regtest-case.ts";
import { readRegtestHistory, type RegtestBlock, type RegtestTip } from "./regtest-history.ts";
import type { DaemonRpc } from "./daemon.ts";

export const REJECTION_NORMALIZATION = "submitblock-regtest-v1";
const REASONS = [
  "bad-cb-amount",
  "bad-txns-premature-spend-of-coinbase",
  "bad-version(0x00000003)",
  "bad-diffbits",
  "time-too-old",
  "bad-cb-height",
  "bad-txns-nonfinal",
  "unexpected-witness",
  "bad-witness-nonce-size",
  "bad-witness-merkle-match",
  "bad-txns-inputs-missingorspent",
  "bad-blk-sigops",
  "high-hash",
] as const;

type Candidate = RegtestBlock & {
  name: string;
  expectedReason: string;
  parentHash: string;
  wireSha256: string;
};

function readCandidate(value: unknown, parents: RegtestTip[]): Candidate {
  const row = value as { name?: unknown; height?: unknown; parent_height?: unknown;
    block_hash_raw?: unknown; block_hex?: unknown; expected_reason?: unknown } | null;
  if (!row || typeof row.name !== "string" || !/^[a-z0-9-]{1,64}$/.test(row.name)
    || typeof row.height !== "number" || !Number.isInteger(row.height) || row.height < 2
    || row.height >= parents.length || row.parent_height !== row.height - 1
    || typeof row.block_hash_raw !== "string" || !/^[a-f0-9]{64}$/.test(row.block_hash_raw)
    || typeof row.block_hex !== "string" || row.block_hex.length < 162 || row.block_hex.length > 65_536
    || row.block_hex.length % 2 !== 0 || !/^[a-f0-9]+$/.test(row.block_hex)
    || !REASONS.some((reason) => reason === row.expected_reason) || typeof row.expected_reason !== "string") {
    throw new Error("Invalid Regtest rejection corpus row");
  }
  const expectedParent = parents[row.height - 1];
  const bytes = Buffer.from(row.block_hex, "hex");
  const hash = Buffer.from(row.block_hash_raw, "hex").reverse().toString("hex");
  const header = decodeHeader(bytes.subarray(0, 80), hash);
  if (header.parent !== expectedParent.hash) throw new Error(`${row.name}: wrong canonical parent`);
  return { name: row.name, height: row.height, hash, blockHex: row.block_hex,
    expectedReason: row.expected_reason, parentHash: header.parent, wireSha256: sha256(bytes) };
}

export function readRejectionCorpus(readText: (path: string) => string) {
  const text = readText(REGTEST_CORPUS);
  const history = readRegtestHistory(() => text);
  const params = JSON.parse(text) as { negatives?: unknown[] };
  if (!Array.isArray(params.negatives) || params.negatives.length !== 19) {
    throw new Error("Expected all 19 Regtest rejection fixtures");
  }
  const parents = [{ height: 0, hash: history.genesisHash }, ...history.blocks];
  const cases = params.negatives.map((row) => readCandidate(row, parents)).sort((a, b) => a.height - b.height);
  if (new Set(cases.map(({ name }) => name)).size !== cases.length
    || new Set(cases.map(({ wireSha256 }) => wireSha256)).size !== cases.length) {
    throw new Error("Duplicate Regtest rejection fixture");
  }
  return {
    genesisHash: history.genesisHash,
    corpusSha256: history.corpusSha256,
    // The daemon retains some rejected headers. Use separate histories to keep the
    // higher cases within its bounded history proof, with a valid successor after each.
    runs: [
      { name: "through-201", blocks: history.blocks.slice(0, 201), cases: cases.filter((row) => row.height <= 201) },
      { name: "through-301", blocks: history.blocks, cases: cases.filter((row) => row.height > 201) },
    ],
  };
}

export type RejectionRun = ReturnType<typeof readRejectionCorpus>["runs"][number];
export type RejectionState = { tip: RegtestTip; basic_header: string; basic_filter_sha256: string; mempool_count: number };

export function isRejectionState(value: unknown, height: number, hash: string): value is RejectionState {
  const state = value as Partial<RejectionState> | null;
  return state?.tip?.height === height && state.tip.hash === hash
    && typeof state.basic_header === "string" && /^[a-f0-9]{64}$/.test(state.basic_header)
    && typeof state.basic_filter_sha256 === "string" && /^[a-f0-9]{64}$/.test(state.basic_filter_sha256)
    && state.mempool_count === 0;
}

async function readState(rpc: DaemonRpc, height: number, hash: string): Promise<RejectionState> {
  const info = await rpc.call("getblockchaininfo") as { chain?: unknown; blocks?: unknown; bestblockhash?: unknown } | null;
  if (info?.chain !== "regtest" || info.blocks !== height || info.bestblockhash !== hash) {
    throw new Error(`Unexpected active Regtest tip at height ${height}`);
  }
  const filter = await rpc.call("getblockfilter", [hash, "basic"]) as { filter?: unknown; header?: unknown } | null;
  const mempool = await rpc.call("getmempoolinfo") as { size?: unknown } | null;
  if (typeof filter?.filter !== "string" || filter.filter.length > 65_536
    || filter.filter.length % 2 !== 0 || !/^[a-f0-9]+$/.test(filter.filter)) {
    throw new Error(`Invalid active BASIC filter at height ${height}`);
  }
  const state = { tip: { height: info.blocks, hash: info.bestblockhash }, basic_header: filter.header,
    basic_filter_sha256: sha256(Buffer.from(filter.filter, "hex")), mempool_count: mempool?.size };
  if (!isRejectionState(state, height, hash)) throw new Error(`Invalid active-chain state at height ${height}`);
  return state;
}

/** Preserve every known BIP22 rule. Null, RPC faults and ambiguous strings have no rejection verdict. */
export function normalizeRejection(result: unknown): string {
  const reason = REASONS.find((reason) => reason === result);
  if (!reason) throw new Error(`Unexpected submitblock result: ${JSON.stringify(result)}`);
  return reason;
}

export async function observeRejections(rpc: DaemonRpc, run: RejectionRun) {
  const observations = [];
  let maybeTip: RegtestTip | undefined;
  for (const block of run.blocks) {
    const candidates = run.cases.filter(({ height }) => height === block.height);
    for (const candidate of candidates) {
      const before = await readState(rpc, candidate.height - 1, candidate.parentHash);
      const result = await rpc.call("submitblock", [candidate.blockHex]);
      const after = await readState(rpc, candidate.height - 1, candidate.parentHash);
      if (!isDeepStrictEqual(before, after)) throw new Error(`${candidate.name}: rejection changed observable active state`);
      const reason = normalizeRejection(result);
      if (reason !== candidate.expectedReason) {
        throw new Error(`${candidate.name}: expected ${candidate.expectedReason}; observed ${reason}`);
      }
      observations.push({ name: candidate.name, height: candidate.height, candidate_hash: candidate.hash,
        wire_sha256: candidate.wireSha256, submitblock: result, normalized_reason: reason, before, after });
      console.log(`CDIF-03: ${candidate.name}: ${reason}`);
    }
    maybeTip = await rpc.submit(block);
  }
  return { final_tip: maybeTip, cases: observations };
}

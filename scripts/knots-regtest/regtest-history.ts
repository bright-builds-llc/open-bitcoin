import { decodeBlock, sha256 } from "../phase167-historical-corpus.ts";
import { REGTEST_CORPUS, REGTEST_GENESIS } from "./regtest-case.ts";

const FINAL_HEIGHT = 301;
const COINBASE_HEIGHT = 1;
const MATURITY = 100;
const MATURITY_HEIGHT = COINBASE_HEIGHT + MATURITY;
const SUBSIDIES = [
  [149, 5_000_000_000n],
  [150, 2_500_000_000n],
  [299, 2_500_000_000n],
  [300, 1_250_000_000n],
] as const;
const CHECKPOINT_HEIGHTS = [MATURITY_HEIGHT - 1, MATURITY_HEIGHT, ...SUBSIDIES.map(([height]) => height), FINAL_HEIGHT];

export type RegtestTip = { height: number; hash: string };
export type RegtestBlock = RegtestTip & { blockHex: string };

function verifyRow(value: unknown, height: number, previousHash: string) {
  const row = value as { height?: unknown; block_hash_raw?: unknown; parent_hash_raw?: unknown; block_hex?: unknown } | null;
  if (row?.height !== height
    || typeof row.block_hash_raw !== "string" || !/^[0-9a-f]{64}$/.test(row.block_hash_raw)
    || typeof row.block_hex !== "string" || !/^(?:[0-9a-f]{2}){82,4096}$/.test(row.block_hex)) {
    throw new Error(`Invalid Regtest corpus row at height ${height}`);
  }
  const hash = Buffer.from(row.block_hash_raw, "hex").reverse().toString("hex");
  const block = decodeBlock(Buffer.from(row.block_hex, "hex"), hash);
  if (block.parent !== previousHash || row.parent_hash_raw !== Buffer.from(previousHash, "hex").reverse().toString("hex")
    || block.bits !== 0x207fffff || (height === 0 && hash !== REGTEST_GENESIS)) {
    throw new Error(`Unexpected Regtest genesis, ancestry or difficulty at height ${height}`);
  }
  return { height, hash, blockHex: row.block_hex, transactions: block.transactions };
}

export function summarizeAcceptedTips(tips: RegtestTip[]) {
  return {
    accepted_count: tips.length,
    accepted_tips_sha256: sha256(JSON.stringify(tips.map(({ height, hash }) => [height, hash]))),
    checkpoints: CHECKPOINT_HEIGHTS.map((height) => {
      const tip = tips[height - 1];
      return { height: tip.height, hash: tip.hash };
    }),
  };
}

/** Select the gapless positive chain and verify the monetary facts from its wire bytes. */
export function readRegtestHistory(readText: (path: string) => string) {
  const text = readText(REGTEST_CORPUS);
  const input = JSON.parse(text) as { rows?: unknown[] } | null;
  if (!Array.isArray(input?.rows) || input.rows.length !== FINAL_HEIGHT + 1) {
    throw new Error(`Expected canonical Regtest rows 0 through ${FINAL_HEIGHT}`);
  }
  let previousHash = "0".repeat(64);
  const chain = input.rows.map((row, height) => {
    const block = verifyRow(row, height, previousHash);
    previousHash = block.hash;
    return block;
  });

  const origin = chain[COINBASE_HEIGHT].transactions[0];
  const spendingBlock = chain[MATURITY_HEIGHT];
  const maybeSpend = spendingBlock.transactions[1];
  const maybeInput = maybeSpend?.inputs[0];
  if (chain[MATURITY_HEIGHT - 1].transactions.length !== 1
    || spendingBlock.transactions.length !== 2 || maybeSpend?.inputs.length !== 1
    || maybeInput?.txid !== origin.txid || maybeInput.vout !== 0) {
    throw new Error("The height-101 block must spend height-one coinbase output zero");
  }

  const subsidyBoundaries = SUBSIDIES.map(([height, reward]) => {
    const block = chain[height];
    const total = block.transactions[0].outputs.reduce((sum, output) => sum + BigInt(output.value), 0n);
    if (block.transactions.length !== 1 || total !== reward) {
      throw new Error(`Unexpected fee-free subsidy at Regtest height ${height}`);
    }
    return { height, hash: block.hash, subsidy_sats: total.toString() };
  });

  return {
    genesisHash: REGTEST_GENESIS,
    corpusSha256: sha256(text),
    blocks: chain.slice(1).map(({ height, hash, blockHex }) => ({ height, hash, blockHex })),
    monetaryFacts: {
      maturity: {
        coinbase_height: COINBASE_HEIGHT,
        spend_height: MATURITY_HEIGHT,
        depth: MATURITY,
        coinbase_txid: origin.txid,
        vout: maybeInput.vout,
        spend_txid: maybeSpend.txid,
      },
      subsidy_boundaries: subsidyBoundaries,
    },
  };
}

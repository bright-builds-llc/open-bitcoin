import { createHash } from "node:crypto";
import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { sha256 } from "./phase167-historical-corpus.ts";

export const MAINNET_GENESIS = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
export const MAINNET_PREFIX_HEIGHT = 200;
export const MAINNET_PREFIX_TIP = "000000008f1a7008320c16b8402b7f11e82951f44ca2663caf6860ab2eeef320";
export const MAINNET_NEXT_HASH = "000000002b50d5963806b024fa09d296a3d8762713536eba9e5bdfa7596f814a";
export const MAINNET_PREFIX_CORPUS = "docs/parity/fixtures/mainnet-prefix-200.hex";
export const MAINNET_NEXT_BLOCK = "docs/parity/fixtures/mainnet-block-201.hex";
export const MAINNET_PREFIX_RECEIPT = "docs/parity/receipts/mainnet-prefix.json";
export const PREFIX_LIMIT_REASON = "mainnet-offline-prefix-limit";

function headerHash(header: Uint8Array): string {
  const inner = createHash("sha256").update(header).digest();
  return createHash("sha256").update(inner).digest().reverse().toString("hex");
}

function blockRow(blockHex: string, height: number, parent: string) {
  if (!/^(?:[0-9a-f]{2}){81,4096}$/.test(blockHex)) {
    throw new Error(`Malformed Mainnet corpus block at height ${height}`);
  }
  const block = Buffer.from(blockHex, "hex");
  if (Buffer.from(block.subarray(4, 36)).reverse().toString("hex") !== parent) {
    throw new Error(`Incorrect Mainnet corpus parent at height ${height}`);
  }
  if (block.readUInt32LE(72) !== 0x1d00ffff) {
    throw new Error(`Unexpected pre-retarget Mainnet target at height ${height}`);
  }
  return { height, hash: headerHash(block.subarray(0, 80)), blockHex };
}

function readBounded(root: string, file: string, maxBytes: number): string {
  const path = join(root, file);
  const stat = statSync(path);
  if (!stat.isFile() || stat.size > maxBytes) throw new Error(`Oversize or missing corpus file: ${file}`);
  return readFileSync(path, "utf8");
}

export function summarizeMainnetTips(tips: readonly { height: number; hash: string }[]) {
  return {
    accepted_count: tips.length,
    accepted_tips_sha256: sha256(JSON.stringify(tips.map(({ height, hash }) => [height, hash]))),
  };
}

export function readMainnetPrefix(root: string) {
  const corpus = readBounded(root, MAINNET_PREFIX_CORPUS, 1_700_000);
  const next = readBounded(root, MAINNET_NEXT_BLOCK, 8193);
  const lines = corpus.split("\n");
  if (lines.pop() !== "" || lines.length !== MAINNET_PREFIX_HEIGHT || !next.endsWith("\n")) {
    throw new Error("Expected exactly 200 Mainnet prefix blocks and one separately recorded height-201 block");
  }
  let parent = MAINNET_GENESIS;
  const blocks = lines.map((line, index) => {
    const row = blockRow(line, index + 1, parent);
    parent = row.hash;
    return row;
  });
  const nextBlock = blockRow(next.slice(0, -1), MAINNET_PREFIX_HEIGHT + 1, parent);
  if (parent !== MAINNET_PREFIX_TIP || nextBlock.hash !== MAINNET_NEXT_HASH) {
    throw new Error("Mainnet prefix terminates at an unexpected identity");
  }
  const target = 0xffffn << 208n;
  const workPerBlock = (1n << 256n) / (target + 1n);
  const chainwork = (workPerBlock * BigInt(MAINNET_PREFIX_HEIGHT + 1)).toString(16).padStart(64, "0");
  return {
    blocks, nextBlock, chainwork,
    finalTip: { height: MAINNET_PREFIX_HEIGHT, hash: MAINNET_PREFIX_TIP },
    accepted: summarizeMainnetTips(blocks),
    corpusSha256: sha256(corpus), boundarySha256: sha256(next),
  };
}

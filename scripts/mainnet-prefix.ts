import { createHash } from "node:crypto";
import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { sha256 } from "./phase167-historical-corpus.ts";

export const MAINNET_GENESIS = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
export const MAINNET_PREFIX_HEIGHT = 2016;
export const MAINNET_PREFIX_TIP = "00000000a141216a896c54f211301c436e557a8d55900637bbdce14c6c7bddef";
export const MAINNET_NEXT_HASH = "00000000a30d73bbfe167ace49a48042099ea64bab675611e29545a7abe06dee";
export const MAINNET_PREFIX_CORPUS = [
  "docs/parity/fixtures/mainnet-prefix-200.hex",
  "docs/parity/fixtures/mainnet-retarget-201-2016.hex",
] as const;
export const MAINNET_NEXT_BLOCK = "docs/parity/fixtures/mainnet-block-2017.hex";
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
  return {
    height,
    hash: headerHash(block.subarray(0, 80)),
    blockHex,
    bits: block.readUInt32LE(72),
    time: block.readUInt32LE(68),
  };
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
  const corpus = MAINNET_PREFIX_CORPUS
    .map(file => readBounded(root, file, 4_000_000)).join("");
  const next = readBounded(root, MAINNET_NEXT_BLOCK, 8193);
  const lines = corpus.split("\n");
  if (lines.pop() !== "" || lines.length !== MAINNET_PREFIX_HEIGHT || !next.endsWith("\n")) {
    throw new Error("Expected exactly 2016 Mainnet prefix blocks and one separately recorded height-2017 block");
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
  const anchorTime = 1_231_006_505;
  const targetTimespan = 14 * 24 * 60 * 60;
  const powLimitBits = 0x1d00ffff;
  const powLimitTarget = 0xffffn << 208n;
  const previous = blocks[MAINNET_PREFIX_HEIGHT - 2];
  const retargetBlock = blocks[MAINNET_PREFIX_HEIGHT - 1];
  const elapsed = previous.time - anchorTime;
  const clamped = Math.max(targetTimespan / 4, Math.min(elapsed, targetTimespan * 4));
  const recomputed = powLimitTarget * BigInt(clamped) / BigInt(targetTimespan);
  if (!blocks.slice(0, -1).every(block => block.bits === powLimitBits)
    || recomputed <= powLimitTarget || retargetBlock.bits !== powLimitBits || nextBlock.bits !== powLimitBits) {
    throw new Error("First Mainnet retarget did not apply the pinned PoW limit");
  }
  const workPerBlock = (1n << 256n) / (powLimitTarget + 1n);
  const workThrough = (height: number) => (workPerBlock * BigInt(height + 1)).toString(16).padStart(64, "0");
  return {
    blocks, nextBlock, chainwork: workThrough(MAINNET_PREFIX_HEIGHT),
    retarget: {
      height: MAINNET_PREFIX_HEIGHT,
      anchor_height: 0,
      anchor_hash: MAINNET_GENESIS,
      anchor_time: anchorTime,
      previous_hash: previous.hash,
      previous_time: previous.time,
      previous_bits: previous.bits.toString(16).padStart(8, "0"),
      elapsed_seconds: elapsed,
      clamped_seconds: clamped,
      target_timespan_seconds: targetTimespan,
      capped_at_pow_limit: true,
      bits: retargetBlock.bits.toString(16).padStart(8, "0"),
      previous_chainwork: workThrough(previous.height),
    },
    finalTip: { height: MAINNET_PREFIX_HEIGHT, hash: MAINNET_PREFIX_TIP },
    accepted: summarizeMainnetTips(blocks),
    corpusSha256: sha256(corpus), boundarySha256: sha256(next),
  };
}

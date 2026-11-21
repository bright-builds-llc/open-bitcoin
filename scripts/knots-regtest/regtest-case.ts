import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";

export const REGTEST_CORPUS = "packages/testdata/phase167/regtest-history.json";
const GENESIS_HASH = "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206";

export type RegtestCase = {
  genesisHash: string;
  blockHash: string;
  blockHex: string;
  blockSha256: string;
  corpusSha256: string;
};

function sha256(bytes: Uint8Array) {
  return createHash("sha256").update(bytes).digest();
}

export function readRegtestCase(readText: (path: string) => string): RegtestCase {
  const text = readText(REGTEST_CORPUS);
  const { rows } = JSON.parse(text);
  const genesis = Buffer.from(GENESIS_HASH, "hex").reverse().toString("hex");
  const block = rows?.[1];
  if (rows?.[0]?.height !== 0 || rows[0].block_hash_raw !== genesis
    || block?.height !== 1 || block.parent_hash_raw !== genesis
    || typeof block.block_hex !== "string" || !/^(?:[0-9a-f]{2}){82,4096}$/.test(block.block_hex)) {
    throw new Error("Regtest corpus must begin with genesis and a serialized height-one descendant");
  }

  const bytes = Buffer.from(block.block_hex, "hex");
  const hash = sha256(sha256(bytes.subarray(0, 80)));
  if (bytes.subarray(4, 36).toString("hex") !== genesis || hash.toString("hex") !== block.block_hash_raw) {
    throw new Error("Regtest corpus height-one header has the wrong parent or hash");
  }
  return {
    genesisHash: GENESIS_HASH,
    blockHash: hash.reverse().toString("hex"),
    blockHex: block.block_hex,
    blockSha256: sha256(bytes).toString("hex"),
    corpusSha256: sha256(Buffer.from(text)).toString("hex"),
  };
}

export function loadRegtestCase(root: string): RegtestCase {
  return readRegtestCase((path) => readFileSync(join(root, path), "utf8"));
}

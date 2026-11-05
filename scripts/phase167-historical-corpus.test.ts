import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { decodeBlock, decodeTransaction, endpoint, requestBytes, LIMITS, REQUIRED, check, checkBodies, checkContexts, sourceExpectations, maybeBip30OriginalHeight } from "./phase167-historical-corpus";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, symlinkSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";

const sha = (v: Buffer) => createHash("sha256").update(v).digest();
const hash = (v: Buffer) => sha(sha(v));
function fixture() {
  const tx = Buffer.from("0100000001" + "00".repeat(32) + "ffffffff020151ffffffff01" +
    "00f2052a01000000015100000000", "hex");
  const header = Buffer.alloc(80);
  header.writeUInt32LE(1); hash(tx).copy(header, 36); header.writeUInt32LE(0x207fffff, 72);
  return { raw: Buffer.concat([header, Buffer.from([1]), tx]), expected: hash(header).reverse().toString("hex") };
}

test("independently decodes exact serialized block, txid and merkle", () => {
  // Arrange
  const { raw, expected } = fixture();
  // Act
  const block = decodeBlock(raw, expected);
  // Assert
  expect(block.hash).toBe(expected);
  expect(block.transactions.length).toBe(1);
  expect(block.transactions[0].coinbase).toBe(true);
});
for (const [name, mutate] of [
  ["header", (raw: Buffer) => { raw[4] ^= 1; }],
  ["body", (raw: Buffer) => { raw[raw.length - 1] ^= 1; }],
] as const) test(`rejects altered ${name} bytes`, () => {
  // Arrange
  const { raw, expected } = fixture(); mutate(raw);
  // Act / Assert
  expect(() => decodeBlock(raw, expected)).toThrow();
});
test("rejects truncated and trailing serialized data", () => {
  // Arrange
  const { raw, expected } = fixture();
  // Act / Assert
  expect(() => decodeBlock(raw.subarray(0, -1), expected)).toThrow();
  expect(() => decodeBlock(Buffer.concat([raw, Buffer.from([0])]), expected)).toThrow();
});
test("requires exact hash rather than a supplied endpoint assertion", () => {
  const { raw } = fixture();
  expect(() => decodeBlock(raw, "00".repeat(32))).toThrow();
});
test("freezes seven selected network identities", () => {
  expect(REQUIRED.length).toBe(7);
  expect(new Set(REQUIRED.map(row => row.hash)).size).toBe(7);
  expect(REQUIRED.filter(row => row.network === "testnet3").length).toBe(1);
});
test("endpoint allowlist refuses origin, scheme, credential and redirect tricks", () => {
  for (const url of ["http://blockstream.info/api/blocks/tip/hash", "https://evil.test/api/block/" + "00".repeat(32) + "/raw",
    "https://user@blockstream.info/api/block/" + "00".repeat(32) + "/raw",
    "https://blockstream.info/api/block/" + "00".repeat(32) + "/raw?redirect=evil"])
    expect(() => endpoint(url)).toThrow();
});
test("streamed downloads enforce finite budget without trusting Content-Length", async () => {
  // Arrange
  const url = "https://blockstream.info/api/block/" + "00".repeat(32) + "/raw";
  const budget = { bytes: LIMITS.download - 1 };
  const fetcher = async () => new Response(new Uint8Array([1, 2]));
  // Act / Assert
  await expect(requestBytes(url, budget, fetcher)).rejects.toThrow("download budget");
});

test("real seven-body offline check makes zero HTTP requests", () => {
  // Arrange
  const original = globalThis.fetch;
  globalThis.fetch = (() => { throw new Error("offline checker attempted network"); }) as typeof fetch;
  try {
    // Act / Assert
    expect(checkBodies().blocks).toBe(7);
  } finally { globalThis.fetch = original; }
});

function mutatedCorpus(mutate: (manifest: any, corpus: any) => void) {
  const root = mkdtempSync(resolve(tmpdir(), "phase167-corpus-"));
  mkdirSync(resolve(root, "packages/testdata/phase167"), { recursive: true });
  symlinkSync(resolve(import.meta.dir, "../packages/bitcoin-knots"), resolve(root, "packages/bitcoin-knots"));
  const manifest = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/manifest.json"), "utf8"));
  const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/blocks.json"), "utf8"));
  mutate(manifest, corpus);
  const bytes = JSON.stringify(corpus); manifest.blocks_sha256 = createHash("sha256").update(bytes).digest("hex");
  writeFileSync(resolve(root, "packages/testdata/phase167/blocks.json"), bytes);
  writeFileSync(resolve(root, "packages/testdata/phase167/manifest.json"), JSON.stringify(manifest));
  try { checkBodies(root); } finally { rmSync(root, { recursive: true }); }
}
test("offline check rejects wrong selected network despite renewed file digest", () => {
  expect(() => mutatedCorpus((_manifest, corpus) => { corpus.blocks[0].network = "testnet3"; })).toThrow("network/hash");
});
test("offline check rejects wrong pin and source digest", () => {
  expect(() => mutatedCorpus(manifest => { manifest.pin = "00".repeat(20); })).toThrow("source pin/digest");
  expect(() => mutatedCorpus(manifest => { manifest.sources[0].sha256 = "00".repeat(32); })).toThrow("source pin/digest");
});
test("independent source expectations include BIP30 recheck and exact replacements", () => {
  // Arrange / Act
  const rows = sourceExpectations();
  // Assert
  expect(rows.bip30.some((row: any) => row.height === 1983701 && !row.enforce)).toBe(true);
  expect(rows.bip30.some((row: any) => row.height === 1983702 && row.enforce)).toBe(true);
  expect(rows.flags.some((row: any) => row.height === 692261 && row.flags.includes("DERSIG") && !row.flags.includes("TAPROOT"))).toBe(true);
});
test("missing historical prerequisites refuse rather than become scalar evidence", () => {
  expect(() => checkContexts({ schema: 1, envelopes: [] }, [])).toThrow("required historical");
});
test("seeded prerequisites cannot claim ordinary or actual daemon authority", () => {
  for (const evidence_class of ["ordinary-canonical", "actual-daemon", "whole-chain"]) {
    // Arrange
    const context = { schema: 1, evidence_class, envelopes: Array(7).fill({}) };
    // Act / Assert
    expect(() => checkContexts(context, Array(7).fill({}))).toThrow("required historical");
  }
});
test("offline block check rejects body SHA mutation despite renewed container digest", () => {
  expect(() => mutatedCorpus((_manifest, corpus) => { corpus.blocks[1].sha256 = "00".repeat(32); })).toThrow("block SHA256");
});
test("refuses unexpected HTTP status before accepting response bytes", async () => {
  const url = "https://blockstream.info/api/block/" + "00".repeat(32) + "/raw";
  await expect(requestBytes(url, { bytes: 0 }, async () => new Response("gone", { status: 404 }))).rejects.toThrow("HTTP 404");
});
test("retains HTTP429 Retry-After fact for explicit bounded acquisition cooldown", async () => {
  const url = "https://blockstream.info/api/block/" + "00".repeat(32) + "/raw";
  await expect(requestBytes(url, { bytes: 0 }, async () => new Response("limited", { status: 429, headers: { "Retry-After": "60" } }))).rejects.toThrow("Retry-After=60");
});
test("enforces per-response bytes independently from cumulative download bound", async () => {
  const url = "https://blockstream.info/api/block/" + "00".repeat(32) + "/raw";
  await expect(requestBytes(url, { bytes: 0 }, async () => new Response(new Uint8Array(LIMITS.response + 1)))).rejects.toThrow("response budget");
});
test("actual Taproot exception body binds coinbase witness nonce to commitment", () => {
  // Arrange
  const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/blocks.json"), "utf8"));
  const row = corpus.blocks[1]; const raw = Buffer.from(row.raw, "hex"); const block = decodeBlock(raw, row.hash);
  const coinbase = block.transactions[0]; const firstTx = raw.indexOf(Buffer.from(coinbase.raw, "hex"), 80);
  const nonceEnd = firstTx + coinbase.raw.length / 2 - 4;
  raw[nonceEnd - 1] ^= 1;
  // Act / Assert
  expect(() => decodeBlock(raw, row.hash)).toThrow("witness commitment");
});
test("retarget expectations clamp, cap and retain compact rounding", () => {
  // Arrange / Act
  const rows = sourceExpectations().retarget;
  // Assert: independently calculated C++ compact-target examples
  expect(rows.find(row => row.bits === 0x1b0404cb && row.elapsed === 0)?.expected).toBe(0x1b010132);
  expect(rows.find(row => row.bits === 0x1b0404cb && row.elapsed === 4838401)?.expected).toBe(0x1b10132c);
  expect(rows.find(row => row.bits === 0x1d00ffff && row.elapsed === 4838400)?.expected).toBe(0x1d00ffff);
});
test("independent transaction decoder separates txid and witness txid", () => {
  // Arrange
  const legacy = fixture().raw.subarray(81);
  const raw = Buffer.concat([legacy.subarray(0, 4), Buffer.from("0001", "hex"), legacy.subarray(4, -4), Buffer.from("010151", "hex"), legacy.subarray(-4)]);
  const expected = hash(legacy).reverse().toString("hex");
  // Act
  const tx = decodeTransaction(raw, expected);
  // Assert
  expect(tx.txid).toBe(expected);
  expect(tx.wtxid).toBe(hash(raw).reverse().toString("hex"));
  expect(tx.wtxid).not.toBe(expected);
});
test("refuses noncanonical CompactSize even with an unchanged exact header", () => {
  // Arrange
  const { raw, expected } = fixture();
  const changed = Buffer.concat([raw.subarray(0, 80), Buffer.from("fd0100", "hex"), raw.subarray(81)]);
  // Act / Assert
  expect(() => decodeBlock(changed, expected)).toThrow("noncanonical");
});
test("witness item count is bounded by actual remaining bytes", () => {
  // Arrange
  const legacy = fixture().raw.subarray(81);
  const raw = Buffer.concat([legacy.subarray(0, 4), Buffer.from("0001", "hex"), legacy.subarray(4, -4), Buffer.from("fe00004000", "hex"), legacy.subarray(-4)]);
  // Act / Assert
  expect(() => decodeTransaction(raw, hash(legacy).reverse().toString("hex"))).toThrow("witness item count bound");
});
test("fixed alternate data origin keeps exact network and request surface", () => {
  // Arrange
  const suffix = "/api/tx/" + "12".repeat(32) + "/raw";
  // Act / Assert
  expect(endpoint("https://mempool.space" + suffix).host).toBe("mempool.space");
  for (const origin of ["http://mempool.space", "https://mempool.space.evil.test", "https://mempool.space:444", "https://user@mempool.space"])
    expect(() => endpoint(origin + suffix)).toThrow("allowlisted");
});
test("BIP30 repeat mapping follows actual identical coinbase bytes", () => {
  // Arrange
  const rows = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/blocks.json"), "utf8")).blocks;
  const byHeight = new Map<number, any>(rows.map((row: any) => [row.height, decodeBlock(Buffer.from(row.raw, "hex"), row.hash)]));
  // Act / Assert
  for (const [repeat, original, wrong] of [[91842, 91812, 91722], [91880, 91722, 91812]]) {
    expect(maybeBip30OriginalHeight(repeat)).toBe(original);
    expect(byHeight.get(repeat).transactions[0].raw).toBe(byHeight.get(original).transactions[0].raw);
    expect(byHeight.get(repeat).transactions[0].raw).not.toBe(byHeight.get(wrong).transactions[0].raw);
  }
});
test("source compact rounding encodes zero target as zero", () => {
  expect(sourceExpectations().retarget.find(row => row.bits === 0x01010000 && row.elapsed === 0)?.expected).toBe(0);
});
test("explicit BIP30 expectations pair heights without zipping unpaired lists", () => {
  expect((sourceExpectations() as any).bip30_pairs?.map((row: any) => [row.repeat_height, row.original_height])).toEqual([[91842, 91812], [91880, 91722]]);
});
test("serialized corpus stores each raw previous transaction once with a checked response reference", () => {
  // Arrange / Act
  const context = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/contexts.json"), "utf8"));
  // Assert
  expect(context.responses.filter((row: any) => row.transaction !== undefined).length).toBe(context.transactions.length);
});
test("complete historical corpus and expiry windows validate with zero HTTP requests", () => {
  // Arrange
  const original = globalThis.fetch; globalThis.fetch = (() => { throw new Error("offline checker attempted HTTP"); }) as typeof fetch;
  try {
    // Act / Assert
    expect(check().previous_transactions).toBe(5263);
  } finally { globalThis.fetch = original; }
});

function historicalMutation(mutate: (context: any) => void, expected: string) {
  // Arrange
  const context = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/contexts.json"), "utf8"));
  const blocks = JSON.parse(readFileSync(resolve(import.meta.dir, "../packages/testdata/phase167/blocks.json"), "utf8")).blocks;
  mutate(context);
  // Act / Assert
  expect(() => checkContexts(context, blocks)).toThrow(expected);
}
const historicalNegatives: [string, (context: any) => void, string][] = [
  ["missing parent MTP ancestor", c => { c.envelopes[0].parent_window.pop(); }, "missing MTP ancestors"],
  ["reordered candidate suffix", c => { c.envelopes[0].candidate_history.reverse(); }, "missing/reordered header ancestor"],
  ["missing expiry MTP sample", c => { c.envelopes[0].expiry_window.pop(); }, "missing MTP ancestors"],
  ["wrong expiry MTP", c => { c.envelopes[0].expiry_mtp++; }, "expiry MTP mismatch"],
  ["wrong UTXO creating MTP", c => { c.envelopes[0].external_utxos[0].created_mtp++; }, "wrong historical UTXO/creating MTP"],
  ["wrong previous outpoint", c => { c.envelopes[0].external_utxos[0].vout = 0xffffffff; }, "wrong historical UTXO/creating MTP"],
  ["missing spending UTXO", c => { c.envelopes[0].external_utxos.pop(); }, "missing/excess historical UTXOs"],
  ["wrong source transaction creation MTP", c => { c.transactions[0].creating_mtp++; }, "creating MTP mismatch"],
  ["wrong envelope network", c => { c.envelopes[0].network = "testnet3"; }, "envelope identity/evidence mismatch"],
  ["wrong BIP34 anchor", c => { c.envelopes[1].bip34.hash = "00".repeat(32); }, "missing/wrong BIP34 ancestor"],
  ["false high-height BIP34 absence", c => { c.envelopes[1].bip34.kind = "absent-parent-below-height"; }, "missing/wrong BIP34 ancestor"],
  ["overlong linked walk", c => { c.envelopes[0].candidate_history = Array(LIMITS.headers + 1).fill(c.envelopes[0].parent); }, "linked header walk bound"],
  ["missing raw-response reference", c => { c.responses.find((r: any) => r.transaction).transaction = "mainnet:" + "00".repeat(32); }, "missing/wrong response transaction reference"],
  ["wrong existing raw-response reference", c => { const refs = c.responses.filter((r: any) => r.transaction); refs[0].transaction = refs[1].transaction; }, "missing/wrong response transaction reference"],
  ["cyclic response reference", c => { const row = c.responses.find((r: any) => r.transaction); row.transaction = row.url; }, "invalid response transaction reference"],
  ["wrong reconstructed response digest", c => { c.responses.find((r: any) => r.transaction).sha256 = "00".repeat(32); }, "response provenance mismatch"],
  ["extra ordinary-authority field", c => { c.envelopes[0].ordinary = true; }, "unexpected schema/authority fields"],
  ["actual-daemon evidence promotion", c => { c.evidence_class = "actual-daemon"; }, "required historical prerequisites missing"],
  ["reversed91842 original", c => { c.envelopes[3].overwrite_original = [REQUIRED[5].hash]; }, "BIP30 original prerequisite mismatch"],
  ["reversed91880 original", c => { c.envelopes[4].overwrite_original = [REQUIRED[6].hash]; }, "BIP30 original prerequisite mismatch"],
  ["changed ancestor header bytes", c => { c.headers[0].raw = c.headers[0].raw.slice(0, -2) + "ff"; }, "header hash mismatch"],
  ["serialized prevtx mutation with renewed response SHA", c => {
    const tx = c.transactions[0]; tx.raw = tx.raw.slice(0, -2) + (tx.raw.endsWith("01") ? "02" : "01");
    c.responses.find((r: any) => r.transaction === `${tx.network}:${tx.txid}`).sha256 = createHash("sha256").update(Buffer.from(tx.raw, "hex")).digest("hex");
  }, "previous transaction identity mismatch"],
];
for (const [name, mutate, message] of historicalNegatives) test(`full exact corpus refuses ${name}`, () => historicalMutation(mutate, message));

function manifestMutation(mutate: (manifest: any) => void) {
  const root = mkdtempSync(resolve(tmpdir(), "phase167-manifest-"));
  const data = "packages/testdata/phase167"; const expectations = "docs/parity/fixtures/phase167-rule-expectations.json";
  mkdirSync(resolve(root, data), { recursive: true }); mkdirSync(resolve(root, "docs/parity/fixtures"), { recursive: true });
  symlinkSync(resolve(import.meta.dir, "../packages/bitcoin-knots"), resolve(root, "packages/bitcoin-knots"));
  const files = [`${data}/blocks.json`, `${data}/contexts.json`, expectations];
  for (const file of files) symlinkSync(resolve(import.meta.dir, "..", file), resolve(root, file));
  const manifest = JSON.parse(readFileSync(resolve(import.meta.dir, "..", data, "manifest.json"), "utf8")); mutate(manifest);
  const payload = files.reduce((sum, file) => sum + statSync(resolve(root, file)).size, 0);
  for (let n = 0; n < 3; n++) manifest.corpus_bytes = payload + Buffer.byteLength(JSON.stringify(manifest, null, 2) + "\n");
  writeFileSync(resolve(root, data, "manifest.json"), JSON.stringify(manifest, null, 2) + "\n");
  try { check(root); } finally { rmSync(root, { recursive: true }); }
}
test("resource manifest cannot change finite limits", () => {
  expect(() => manifestMutation(m => { m.limits.headers++; })).toThrow("manifest limit drift");
});
test("resource manifest cannot claim actual daemon evidence", () => {
  expect(() => manifestMutation(m => { m.resource_fit.actual_daemon = true; })).toThrow("resource/evidence fit mismatch");
});
test("resource manifest rejects impossible download counters", () => {
  expect(() => manifestMutation(m => { m.downloaded_bytes = -1; })).toThrow("corpus/context budget/digest mismatch");
  expect(() => manifestMutation(m => { m.downloaded_bytes = LIMITS.download + 1; })).toThrow("corpus/context budget/digest mismatch");
});
test("source formula distinguishes unavailable anchors from true absence", () => {
  // Arrange / Act
  const rows = sourceExpectations().bip30;
  // Assert
  expect(rows.some(row => row.height === 227931 && row.ancestor === "absent")).toBe(true);
  expect(rows.some(row => row.height === 1983701 && row.ancestor === "unavailable" && "context_admission" in row && row.context_admission === "refuse-missing-required-history")).toBe(true);
  expect(rows.some(row => row.height === 1983702 && "context_admission" in row && row.context_admission === "anchor-unneeded-for-BIP30")).toBe(true);
});

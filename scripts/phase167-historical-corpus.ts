#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { appendFileSync, existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

export const PIN = "a9aee730466ac67d35a3c03ee24676be5e045878";
export const LIMITS = { download: 512 * 1024 * 1024, corpus: 128 * 1024 * 1024,
  response: 4 * 1024 * 1024, headers: 4096, timeout: 30_000 } as const;
type Network = "mainnet" | "testnet3";
export const REQUIRED: readonly { network: Network; hash: string; purpose: string }[] = [
  { network: "mainnet", hash: "00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22", purpose: "script-p2sh-exception" },
  { network: "mainnet", hash: "0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad", purpose: "script-taproot-exception" },
  { network: "testnet3", hash: "00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105", purpose: "script-p2sh-exception" },
  { network: "mainnet", hash: "00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec", purpose: "bip30-repeat-91842" },
  { network: "mainnet", hash: "00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721", purpose: "bip30-repeat-91880" },
  { network: "mainnet", hash: "00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e", purpose: "bip30-original-91722" },
  { network: "mainnet", hash: "00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f", purpose: "bip30-original-91812" },
];
const ROOT = resolve(import.meta.dir, "..");
const DATA = "packages/testdata/phase167";
const LOG = ".local/open-bitcoin-dev/phase167/plan02";
const SOURCE_FILES = ["validation.cpp", "kernel/chainparams.cpp", "consensus/tx_verify.cpp", "pow.cpp", "arith_uint256.cpp", "coins.cpp", "chain.h", "deploymentstatus.h", "script/interpreter.h", "clientversion.h", "clientversion.cpp", "../CMakeLists.txt"];
export const sha256 = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex");
const hash256 = (bytes: Buffer) => Buffer.from(sha256(Buffer.from(sha256(bytes), "hex")), "hex");
const display = (bytes: Buffer) => Buffer.from(bytes).reverse().toString("hex");
function requireFact(ok: unknown, message: string): asserts ok { if (!ok) throw new Error(message); }
function schema(value: object, keys: string[]) {
  requireFact(value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join() === keys.sort().join(), "unexpected schema/authority fields");
}
function fromHex(value: string, maybeLength?: number): Buffer {
  requireFact(typeof value === "string" && value.length % 2 === 0 && /^[0-9a-f]+$/.test(value) && value.length <= LIMITS.response * 2, "invalid bounded hex");
  requireFact(maybeLength === undefined || value.length === maybeLength * 2, "hex length mismatch");
  return Buffer.from(value, "hex");
}
class Reader {
  offset = 0;
  constructor(readonly raw: Buffer) {}
  take(length: number): Buffer {
    requireFact(Number.isSafeInteger(length) && length >= 0 && this.offset + length <= this.raw.length, "truncated payload");
    const bytes = this.raw.subarray(this.offset, this.offset + length); this.offset += length; return bytes;
  }
  u32(): number { return this.take(4).readUInt32LE(); }
  count(): number {
    const tag = this.take(1)[0];
    if (tag < 253) return tag;
    const n = tag === 253 ? this.take(2).readUInt16LE() : tag === 254 ? this.u32() : Number(this.take(8).readBigUInt64LE());
    requireFact(Number.isSafeInteger(n) && n <= LIMITS.response && n >= (tag === 253 ? 253 : tag === 254 ? 65536 : 4294967296), "noncanonical or unbounded CompactSize");
    return n;
  }
  vector(): Buffer { return this.take(this.count()); }
}
export type Transaction = { txid: string; wtxid: string; raw: string; coinbase: boolean;
  inputs: { txid: string; vout: number; script: string; sequence: number }[];
  outputs: { value: string; script: string }[]; witness: string[][] };
function transaction(reader: Reader): Transaction {
  const start = reader.offset; const version = reader.take(4);
  let witness = false;
  if (reader.raw[reader.offset] === 0) { reader.take(1); requireFact(reader.take(1)[0] === 1, "unknown witness flags"); witness = true; }
  const strippedStart = reader.offset; const count = reader.count();
  requireFact(count > 0 && count <= reader.raw.length / 41, "invalid input count");
  const inputs = Array.from({ length: count }, () => ({ txid: display(reader.take(32)), vout: reader.u32(),
    script: reader.vector().toString("hex"), sequence: reader.u32() }));
  const outputCount = reader.count(); requireFact(outputCount > 0 && outputCount <= reader.raw.length / 9, "invalid output count");
  const outputs = Array.from({ length: outputCount }, () => ({ value: reader.take(8).readBigUInt64LE().toString(), script: reader.vector().toString("hex") }));
  const stripped = reader.raw.subarray(strippedStart, reader.offset);
  const stacks = witness ? inputs.map(() => {
    const length = reader.count(); requireFact(length <= reader.raw.length - reader.offset, "witness item count bound");
    return Array.from({ length }, () => reader.vector().toString("hex"));
  }) : [];
  requireFact(!witness || stacks.some(stack => stack.length), "superfluous witness");
  const locktime = reader.take(4); const raw = reader.raw.subarray(start, reader.offset);
  return { txid: display(hash256(Buffer.concat([version, stripped, locktime]))), wtxid: display(hash256(raw)), raw: raw.toString("hex"),
    coinbase: count === 1 && inputs[0].txid === "00".repeat(32) && inputs[0].vout === 0xffffffff, inputs, outputs, witness: stacks };
}
export function decodeTransaction(raw: Buffer, expected: string): Transaction {
  const reader = new Reader(raw); const tx = transaction(reader);
  requireFact(reader.offset === raw.length && tx.txid === expected, "previous transaction identity mismatch"); return tx;
}
function merkle(leaves: Buffer[]): Buffer {
  requireFact(leaves.length > 0, "empty merkle tree"); let level = leaves;
  while (level.length > 1) {
    for (let i = 0; i + 1 < level.length; i += 2) requireFact(!level[i].equals(level[i + 1]), "mutated merkle tree");
    if (level.length % 2) level = [...level, level[level.length - 1]];
    level = Array.from({ length: level.length / 2 }, (_, i) => hash256(Buffer.concat([level[i * 2], level[i * 2 + 1]])));
  }
  return level[0];
}
export function decodeHeader(raw: Buffer, expected: string) {
  requireFact(raw.length === 80 && display(hash256(raw)) === expected, "header hash mismatch");
  return { hash: expected, parent: display(raw.subarray(4, 36)), time: raw.readUInt32LE(68), bits: raw.readUInt32LE(72), version: raw.readInt32LE(0) };
}
export function decodeBlock(raw: Buffer, expected: string) {
  requireFact(raw.length <= LIMITS.response, "block byte bound"); const reader = new Reader(raw);
  const header = reader.take(80); const facts = decodeHeader(header, expected); const count = reader.count();
  requireFact(count > 0 && count <= raw.length / 60, "invalid transaction count");
  const transactions = Array.from({ length: count }, () => transaction(reader));
  requireFact(reader.offset === raw.length, "trailing block bytes");
  requireFact(transactions[0].coinbase && transactions.slice(1).every(tx => !tx.coinbase), "coinbase position");
  requireFact(merkle(transactions.map(tx => Buffer.from(tx.txid, "hex").reverse())).equals(header.subarray(36, 68)), "merkle mismatch");
  const maybeCommitment = transactions[0].outputs.filter(out => out.script.startsWith("6a24aa21a9ed") && out.script.length >= 76).at(-1);
  if (maybeCommitment) {
    const witness = transactions[0].witness;
    requireFact(witness.length === 1 && witness[0].length === 1 && witness[0][0].length === 64, "witness nonce mismatch");
    const root = merkle(transactions.map((tx, i) => i ? Buffer.from(tx.wtxid, "hex").reverse() : Buffer.alloc(32)));
    requireFact(hash256(Buffer.concat([root, Buffer.from(witness[0][0], "hex")])).toString("hex") === maybeCommitment.script.slice(12, 76), "witness commitment mismatch");
  } else requireFact(transactions.every(tx => !tx.witness.length), "unexpected witness");
  return { ...facts, transactions };
}

/** Finite public HTTPS raw-data surface; redirects and URL credentials are refused. */
export function endpoint(value: string): URL {
  const url = new URL(value);
  requireFact(url.protocol === "https:" && ["blockstream.info", "mempool.space"].includes(url.host) && !url.username && !url.password && !url.search && !url.hash &&
    /^\/(?:testnet\/)?api\/(?:block\/[0-9a-f]{64}(?:\/(?:raw|header|txids))?|tx\/[0-9a-f]{64}\/(?:raw|status|merkle-proof)|block-height\/[0-9]{1,8}|blocks\/[0-9]{1,8})$/.test(url.pathname), "endpoint not allowlisted");
  return url;
}
type Fetcher = (url: string, init: RequestInit) => Promise<Response>;
export async function requestBytes(url: string, budget: { bytes: number }, fetcher: Fetcher = fetch): Promise<Buffer> {
  endpoint(url);
  const response = await fetcher(url, { redirect: "error", signal: AbortSignal.timeout(LIMITS.timeout) });
  if (response.status === 429) {
    const maybeRetryAfter = response.headers.get("retry-after");
    await response.body?.cancel();
    throw Object.assign(new Error(`HTTP 429: ${url}; Retry-After=${maybeRetryAfter ?? "absent"}`), { maybeStatus: 429, maybeRetryAfter });
  }
  requireFact(response.ok && response.body, `HTTP ${response.status}: ${url}`);
  const chunks: Buffer[] = []; let length = 0; const reader = response.body.getReader();
  try {
    while (true) {
      const part = await reader.read(); if (part.done) break;
      length += part.value.length; budget.bytes += part.value.length;
      requireFact(budget.bytes <= LIMITS.download, "download budget exceeded");
      requireFact(length <= LIMITS.response, "response budget exceeded"); chunks.push(Buffer.from(part.value));
    }
  } finally { await reader.cancel(); }
  return Buffer.concat(chunks);
}
const base = (network: Network) => `https://blockstream.info/${network === "testnet3" ? "testnet/" : ""}api`;
function sourceUrl(url: string, network: Network, suffix: string): boolean {
  const parsed = endpoint(url); return parsed.pathname === new URL(base(network) + suffix).pathname;
}
type RawRow = { network: Network; hash: string; purpose: string; height: number; raw: string; sha256: string; url: string; acquired_at: string; outcome: "success" };
function sourceFacts(root: string) {
  const source = resolve(root, "packages/bitcoin-knots");
  const result = spawnSync("git", ["-C", source, "rev-parse", "HEAD"], { encoding: "utf8" });
  requireFact(result.status === 0 && result.stdout.trim() === PIN, "source pin mismatch");
  return SOURCE_FILES.map(file => {
    const sourcePath = file.startsWith("../") ? file.slice(3) : `src/${file}`;
    const git = spawnSync("git", ["-C", source, "show", `${PIN}:${sourcePath}`], { maxBuffer: 8 * 1024 * 1024 });
    requireFact(git.status === 0, "source Git read failed"); const digest = sha256(readFileSync(resolve(source, "src", file)));
    requireFact(digest === sha256(git.stdout), "source drift"); return { file: sourcePath, sha256: digest };
  });
}
function save(root: string, file: string, value: unknown) { mkdirSync(resolve(root, file, ".."), { recursive: true }); writeFileSync(resolve(root, file), JSON.stringify(value, null, 2) + "\n"); }
function load(root: string, file: string): any {
  requireFact(statSync(resolve(root, file)).size <= LIMITS.corpus, "corpus byte bound"); return JSON.parse(readFileSync(resolve(root, file), "utf8"));
}
async function acquireBodies(root: string) {
  const sources = sourceFacts(root); const cache = acquisitionCache(root, 0); const budget = cache.budget; const blocks: RawRow[] = [];
  const attempts: unknown[] = [];
  for (const row of REQUIRED) {
    const url = `${base(row.network)}/block/${row.hash}/raw`; const acquired_at = new Date().toISOString();
    try {
      const response = await cache.get(url); const raw = fromHex(response.raw); const block = decodeBlock(raw, row.hash);
      const meta = JSON.parse(fromHex((await cache.get(`${base(row.network)}/block/${row.hash}`)).raw).toString());
      requireFact(meta.id === block.hash && Number.isSafeInteger(meta.height) && meta.height > 0, "metadata identity/height mismatch");
      blocks.push({ ...row, height: meta.height, raw: raw.toString("hex"), sha256: sha256(raw), url: response.url, acquired_at: response.acquired_at, outcome: "success" });
      attempts.push({ url: response.url, acquired_at: response.acquired_at, outcome: "success", bytes: raw.length, hash: block.hash, height: meta.height });
      console.log(`acquired ${row.network} ${row.hash} height=${meta.height} bytes=${raw.length} txs=${block.transactions.length}`);
    } catch (error) {
      attempts.push({ url, acquired_at, outcome: "failure", error: String(error) });
      save(root, `${LOG}/acquire-${Date.now()}.json`, { attempts, downloaded_bytes: budget.bytes, missing: REQUIRED.filter(row => !blocks.some(b => b.hash === row.hash)) });
      throw error;
    }
  }
  save(root, `${DATA}/blocks.json`, { schema: 1, blocks });
  save(root, `${DATA}/manifest.json`, { schema: 1, pin: PIN, sources, evidence_class: "independently-seeded-historical", required: REQUIRED,
    blocks_sha256: sha256(readFileSync(resolve(root, `${DATA}/blocks.json`))), downloaded_bytes: budget.bytes, attempts, limits: LIMITS });
}
export function checkBodies(root = ROOT) {
  const manifest = load(root, `${DATA}/manifest.json`); const corpus = load(root, `${DATA}/blocks.json`);
  schema(corpus, ["schema", "blocks"]);
  requireFact(manifest.pin === PIN && JSON.stringify(manifest.sources) === JSON.stringify(sourceFacts(root)), "source pin/digest drift");
  requireFact(manifest.schema === 1 && manifest.evidence_class === "independently-seeded-historical" && JSON.stringify(manifest.required) === JSON.stringify(REQUIRED), "manifest identity/evidence mismatch");
  requireFact(JSON.stringify(manifest.limits) === JSON.stringify(LIMITS), "manifest limit drift");
  requireFact(manifest.blocks_sha256 === sha256(readFileSync(resolve(root, `${DATA}/blocks.json`))), "corpus digest mismatch");
  requireFact(corpus.schema === 1 && corpus.blocks.length === REQUIRED.length, "required corpus count mismatch");
  for (const [index, row] of (corpus.blocks as RawRow[]).entries()) {
    schema(row, ["network", "hash", "purpose", "height", "raw", "sha256", "url", "acquired_at", "outcome"]);
    const expected = REQUIRED[index]; requireFact(row.network === expected.network && row.hash === expected.hash && row.purpose === expected.purpose && row.height === HEIGHTS[index], "network/hash/height identity mismatch");
    requireFact(sourceUrl(row.url, row.network, `/block/${row.hash}/raw`) && row.outcome === "success" && Number.isFinite(Date.parse(row.acquired_at)), "provenance mismatch");
    const raw = fromHex(row.raw); requireFact(row.sha256 === sha256(raw), "block SHA256 mismatch"); decodeBlock(raw, expected.hash);
  }
  return { blocks: corpus.blocks.length, evidence_class: manifest.evidence_class };
}
export function check(root = ROOT) {
  checkBodies(root);
  const manifest = load(root, `${DATA}/manifest.json`); const corpus = load(root, `${DATA}/blocks.json`);
  schema(manifest, ["schema", "pin", "sources", "evidence_class", "required", "blocks_sha256", "downloaded_bytes", "attempts", "limits", "contexts_sha256", "expectations_sha256", "corpus_bytes", "resource_fit"]);
  const context = load(root, `${DATA}/contexts.json`);
  const expectationsFile = "docs/parity/fixtures/phase167-rule-expectations.json";
  const corpusBytes = [`${DATA}/blocks.json`, `${DATA}/contexts.json`, expectationsFile, `${DATA}/manifest.json`].reduce((sum, file) => sum + statSync(resolve(root, file)).size, 0);
  requireFact(corpusBytes <= LIMITS.corpus && corpusBytes === manifest.corpus_bytes && Number.isSafeInteger(manifest.downloaded_bytes) && manifest.downloaded_bytes >= 0 && manifest.downloaded_bytes <= LIMITS.download && manifest.contexts_sha256 === sha256(readFileSync(resolve(root, `${DATA}/contexts.json`))) &&
    manifest.expectations_sha256 === sha256(readFileSync(resolve(root, expectationsFile))), "corpus/context budget/digest mismatch");
  requireFact(JSON.stringify(load(root, expectationsFile)) === JSON.stringify(sourceExpectations()), "source expectation drift");
  checkContexts(context, corpus.blocks);
  requireFact(JSON.stringify(manifest.resource_fit) === JSON.stringify(resourceFit(context)), "resource/evidence fit mismatch");
  return { blocks: corpus.blocks.length, evidence_class: manifest.evidence_class, corpus_bytes: corpusBytes, previous_transactions: context.transactions.length };
}
const HEIGHTS = [170060, 692261, 394, 91842, 91880, 91722, 91812];
export function maybeBip30OriginalHeight(height: number): number | undefined {
  return height === 91842 ? 91812 : height === 91880 ? 91722 : undefined;
}
const BIP34 = { mainnet: { height: 227931, hash: "000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8" },
  testnet3: { height: 21111, hash: "0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8" } };
const ACTIVATIONS = { mainnet: [227931, 363725, 388381, 419328, 481824], testnet3: [21111, 330776, 581885, 770112, 834624],
  signet: [1, 1, 1, 1, 1], regtest: [1, 1, 1, 1, 0] };
const EXPIRY = (2026 - 1970 + 2) * 31558060 + 26784000;
function flags(network: keyof typeof ACTIVATIONS, height: number, hash: string): string[] {
  const maybeIndex = REQUIRED.findIndex(row => row.network === network && row.hash === hash);
  const result = maybeIndex === 0 || maybeIndex === 2 ? [] : maybeIndex === 1 ? ["P2SH", "WITNESS"] : ["P2SH", "WITNESS", "TAPROOT"];
  const activations = ACTIVATIONS[network];
  for (const [i, flag] of ["DERSIG", "CHECKLOCKTIMEVERIFY", "CHECKSEQUENCEVERIFY", "NULLDUMMY"].entries()) if (height >= activations[i + 1]) result.push(flag);
  return result;
}
export function compactTarget(bits: number): bigint {
  const size = bits >>> 24; const word = bits & 0x7fffff;
  requireFact(!(bits & 0x800000) && word > 0 && size <= 34, "invalid compact target");
  const target = size <= 3 ? BigInt(word) >> BigInt(8 * (3 - size)) : BigInt(word) << BigInt(8 * (size - 3));
  requireFact(target > 0n && target < 1n << 256n, "overflow compact target"); return target;
}
function compact(target: bigint): number {
  if (target === 0n) return 0;
  let size = Math.ceil(target.toString(16).length / 2);
  let word = Number(size <= 3 ? target << BigInt(8 * (3 - size)) : target >> BigInt(8 * (size - 3)));
  if (word & 0x800000) { word >>= 8; size++; } return size * 16777216 + word;
}
function retarget(bits: number, elapsed: number): number {
  const span = Math.max(302400, Math.min(4838400, elapsed));
  const target = compactTarget(bits) * BigInt(span) / 1209600n;
  const limit = BigInt("0x00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff"); return compact(target > limit ? limit : target);
}
/** Independent pinned C++ formulas/literals, with no first-party Rust oracle. */
export function sourceExpectations() {
  const flagRows = [];
  for (const network of Object.keys(ACTIVATIONS) as (keyof typeof ACTIVATIONS)[]) {
    const heights = new Set([0, 1, ...ACTIVATIONS[network].flatMap(height => [Math.max(0, height - 1), height])]);
    for (const height of heights) flagRows.push({ network, height, hash: "00".repeat(32), flags: flags(network, height, "00".repeat(32)),
      minimum_version: height >= ACTIVATIONS[network][2] ? 4 : height >= ACTIVATIONS[network][1] ? 3 : height >= ACTIVATIONS[network][0] ? 2 : 1,
      finality_time: height >= ACTIVATIONS[network][3] ? "parent-mtp" : "candidate-time", sequence_locks: height >= ACTIVATIONS[network][3], witness_commitment: height >= ACTIVATIONS[network][4] });
  }
  for (const [i, row] of REQUIRED.entries()) for (const network of i < 3 ? ["mainnet", "testnet3"] as const : [row.network])
    flagRows.push({ network, height: HEIGHTS[i], hash: row.hash, flags: flags(network, HEIGHTS[i], row.hash) });
  const bip30 = [];
  for (const height of [227931, 227932, 1983701, 1983702]) for (const ancestor of ["matching", "wrong", "absent"])
    bip30.push({ network: "mainnet", height, ancestor: ancestor === "absent" && height > 227931 ? "unavailable" : ancestor,
      case_class: "conditional-source-formula", context_admission: height >= 1983702 ? "anchor-unneeded-for-BIP30" : height > 227931 && ancestor === "absent" ? "refuse-missing-required-history" : height === 227931 && ancestor !== "absent" ? "refuse-inconsistent-parent-height" : "seeded-condition-only",
      enforce: height >= 1983702 || ancestor !== "matching" || height <= 227931 });
  for (const i of [3, 4]) for (const exact of [true, false]) for (const network of ["mainnet", "testnet3"])
    bip30.push({ network, height: HEIGHTS[i], hash: exact ? REQUIRED[i].hash : "00".repeat(32), ancestor: "absent", enforce: !(network === "mainnet" && exact) });
  return { schema: 1, pin: PIN, evidence_class: "source-derived-expectations", source_functions: ["GetBlockScriptFlags", "ConnectBlock", "GetNextWorkRequired", "CalculateSequenceLocks", "ContextualCheckBlockHeader", "IsThisSoftwareExpired"],
    bip30_pairs: [{ repeat_height: 91842, repeat_hash: REQUIRED[3].hash, original_height: 91812, original_hash: REQUIRED[6].hash, coinbase_txid: "d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599" },
      { repeat_height: 91880, repeat_hash: REQUIRED[4].hash, original_height: 91722, original_hash: REQUIRED[5].hash, coinbase_txid: "e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468" }],
    flags: flagRows, bip30, retarget: [0, 302399, 302400, 1209601, 4838400, 4838401].flatMap(elapsed => [0x01010000, 0x1b0404cb, 0x1d00ffff].map(bits => ({ bits, elapsed, expected: retarget(bits, elapsed) }))),
    minimum_difficulty: [{ delta: 1199, parent_bits: 0x1d00ffff, recovery_bits: 0x1c123456, expected: 0x1c123456 },
      { delta: 1200, parent_bits: 0x1d00ffff, recovery_bits: 0x1c123456, expected: 0x1c123456 }, { delta: 1201, parent_bits: 0x1d00ffff, recovery_bits: 0x1c123456, expected: 0x1d00ffff }],
    expiry: [{ candidate: EXPIRY, older_mtp: EXPIRY + 1, reject: false }, { candidate: EXPIRY + 1, older_mtp: EXPIRY, reject: false }, { candidate: EXPIRY + 1, older_mtp: EXPIRY + 1, reject: true }],
    locks: { absolute: [{ lock: 500000001, cutoff: 500000001, all_final: false, final: false }, { lock: 500000000, cutoff: 500000001, all_final: false, final: true },
      { lock: 500000001, cutoff: 500000001, all_final: true, final: true }], relative: [{ created_mtp: 1000, sequence: 0x400001, minimum_time: 1511, parent_mtp: 1511, valid: false },
      { created_mtp: 1000, sequence: 0x400001, minimum_time: 1511, parent_mtp: 1512, valid: true }, { created_height: 100, sequence: 1, minimum_height: 100, candidate_height: 100, valid: false },
      { created_height: 100, sequence: 1, minimum_height: 100, candidate_height: 101, valid: true }] },
    disconnect: { repeats: "lossless-overwrite-undo-required-by-open-bitcoin", pinned_knots: "does-not-restore-overwritten-original", originals_tolerated: [91722, 91812] } };
}
type ResponseRow = { url: string; acquired_at: string; sha256: string; raw: string };
type StoredResponse = ResponseRow | { url: string; acquired_at: string; sha256: string; transaction: string };
type HeaderRow = { network: Network; height: number; hash: string; raw: string; response: string };
type Previous = { network: Network; txid: string; raw: string; response: string; status: string; proof: string; block_hash: string; height: number; mtp_window: string[]; creating_mtp: number };
type Envelope = { network: Network; hash: string; height: number; parent: string; evidence_class: "independently-seeded-historical";
  parent_window: string[]; candidate_history: string[]; expiry_window: string[]; expiry_mtp: number; difficulty: string[]; bip34: { kind: "absent-parent-below-height" } | { kind: "independent-seeded-anchor"; hash: string };
  external_utxos: { txid: string; vout: number; value: string; script: string; coinbase: boolean; created_height: number; created_mtp: number }[];
  overwrite_original: string[]; expected_flags: string[] };
type Contexts = { schema: number; evidence_class: string; headers: HeaderRow[]; transactions: Previous[]; responses: StoredResponse[]; envelopes: Envelope[] };
const headerKey = (network: Network, hash: string) => `${network}:${hash}`;
function metadataHeader(row: any): Buffer {
  requireFact(Number.isSafeInteger(row.height) && row.height >= 0, "invalid header height"); const bytes = Buffer.alloc(80);
  bytes.writeInt32LE(row.version); fromHex(row.previousblockhash ?? "00".repeat(32), 32).reverse().copy(bytes, 4);
  fromHex(row.merkle_root, 32).reverse().copy(bytes, 36); bytes.writeUInt32LE(row.timestamp, 68); bytes.writeUInt32LE(row.bits, 72); bytes.writeUInt32LE(row.nonce, 76);
  decodeHeader(bytes, row.id); return bytes;
}
function proofRoot(txid: string, proof: any): string {
  requireFact(Array.isArray(proof.merkle) && proof.merkle.length <= 32 && Number.isSafeInteger(proof.pos) && proof.pos >= 0 && proof.pos < 2 ** proof.merkle.length, "invalid transaction proof bounds");
  let raw = fromHex(txid, 32).reverse(); let pos = proof.pos;
  for (const sibling of proof.merkle) { const bytes = fromHex(sibling, 32).reverse(); raw = hash256(Buffer.concat(pos % 2 ? [bytes, raw] : [raw, bytes])); pos = Math.floor(pos / 2); }
  return raw.toString("hex");
}
function linkedWindow(network: Network, window: string[], headers: Map<string, HeaderRow>, lastHeight: number, lastHash: string) {
  requireFact(window.length > 0 && window.length <= LIMITS.headers, "linked header walk bound"); const rows = [];
  for (const [i, hash] of window.entries()) {
    const maybeHeader = headers.get(headerKey(network, hash)); requireFact(maybeHeader && maybeHeader.height === lastHeight - window.length + 1 + i, "missing/reordered header ancestor");
    const header = decodeHeader(fromHex(maybeHeader.raw, 80), hash);
    requireFact(i === 0 || header.parent === window[i - 1], "header ancestor link mismatch"); rows.push({ ...header, height: maybeHeader.height });
  }
  requireFact(window.at(-1) === lastHash, "header endpoint mismatch"); return rows;
}
function windowFacts(network: Network, window: string[], headers: Map<string, HeaderRow>, lastHeight: number, lastHash: string): number {
  requireFact(window.length === Math.min(11, lastHeight + 1), "missing MTP ancestors");
  const times = linkedWindow(network, window, headers, lastHeight, lastHash).map(row => row.time).sort((a, b) => a - b); return times[Math.floor(times.length / 2)];
}
export function checkContexts(value: unknown, blocks: RawRow[]): void {
  const context = value as Contexts;
  requireFact(context.schema === 1 && context.evidence_class === "independently-seeded-historical" && context.envelopes.length === 7 && blocks.length === 7, "required historical prerequisites missing");
  schema(context, ["schema", "evidence_class", "headers", "transactions", "responses", "envelopes"]);
  const headers = new Map<string, HeaderRow>(); const responses = new Map<string, ResponseRow>(); const transactions = new Map<string, Previous>();
  const heights = new Set<string>();
  const rawTransactions = new Map<string, string>();
  for (const row of context.transactions) { const key = headerKey(row.network, row.txid); requireFact(!rawTransactions.has(key), "duplicate transaction reference target"); rawTransactions.set(key, row.raw); }
  for (const stored of context.responses) {
    let response: ResponseRow;
    if ("raw" in stored) { schema(stored, ["url", "acquired_at", "sha256", "raw"]); response = stored; }
    else {
      schema(stored, ["url", "acquired_at", "sha256", "transaction"]);
      requireFact(typeof stored.transaction === "string" && /^(mainnet|testnet3):[0-9a-f]{64}$/.test(stored.transaction), "invalid response transaction reference");
      const [network, txid] = stored.transaction.split(":") as [Network, string]; const maybeRaw = rawTransactions.get(stored.transaction);
      requireFact(maybeRaw !== undefined && sourceUrl(stored.url, network, `/tx/${txid}/raw`), "missing/wrong response transaction reference");
      response = { url: stored.url, acquired_at: stored.acquired_at, sha256: stored.sha256, raw: maybeRaw };
    }
    endpoint(response.url); requireFact(!responses.has(response.url) && Number.isFinite(Date.parse(response.acquired_at)) && sha256(fromHex(response.raw)) === response.sha256, "response provenance mismatch"); responses.set(response.url, response);
  }
  const responseData = (url: string) => { const maybeRow = responses.get(url); requireFact(maybeRow, "missing source response"); return JSON.parse(fromHex(maybeRow.raw).toString()); };
  for (const row of context.headers) {
    schema(row, ["network", "height", "hash", "raw", "response"]); requireFact(row.network === "mainnet" || row.network === "testnet3", "unsupported header network");
    const key = headerKey(row.network, row.hash); requireFact(!headers.has(key) && Number.isSafeInteger(row.height) && row.height >= 0, "duplicate/invalid header");
    const heightKey = `${row.network}:${row.height}`; requireFact(!heights.has(heightKey), "conflicting seeded header height"); heights.add(heightKey);
    const raw = fromHex(row.raw, 80); decodeHeader(raw, row.hash);
    const target = compactTarget(raw.readUInt32LE(72));
    requireFact(target <= BigInt("0x00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff") && BigInt(`0x${row.hash}`) <= target, "header PoW mismatch");
    const entries = responseData(row.response); const batch = Math.floor(row.height / 10) * 10 + 9;
    requireFact(sourceUrl(row.response, row.network, `/blocks/${batch}`) && Array.isArray(entries) && entries.length === 10 && entries.every(meta => meta.height > batch - 10 && meta.height <= batch) &&
      entries.some(meta => meta.id === row.hash && meta.height === row.height && metadataHeader(meta).equals(raw)), "header provenance/network mismatch"); headers.set(key, row);
  }
  for (const row of context.transactions) {
    schema(row, ["network", "txid", "raw", "response", "status", "proof", "block_hash", "height", "mtp_window", "creating_mtp"]);
    requireFact(row.network === "mainnet" || row.network === "testnet3", "unsupported previous transaction network");
    const key = headerKey(row.network, row.txid); requireFact(!transactions.has(key), "duplicate previous transaction");
    const tx = decodeTransaction(fromHex(row.raw), row.txid); const maybeResponse = responses.get(row.response);
    requireFact(sourceUrl(row.response, row.network, `/tx/${row.txid}/raw`) && maybeResponse?.raw === row.raw && sourceUrl(row.status, row.network, `/tx/${row.txid}/status`) && sourceUrl(row.proof, row.network, `/tx/${row.txid}/merkle-proof`), "previous transaction provenance mismatch");
    const status = responseData(row.status); const proof = responseData(row.proof); const maybeHeader = headers.get(headerKey(row.network, row.block_hash)); requireFact(maybeHeader, "missing creating header");
    requireFact(status.confirmed && status.block_hash === row.block_hash && status.block_height === row.height && proof.block_height === row.height && maybeHeader.height === row.height, "creating block mismatch");
    requireFact(proofRoot(tx.txid, proof) === maybeHeader.raw.slice(72, 136), "previous transaction merkle mismatch");
    const header = decodeHeader(fromHex(maybeHeader.raw, 80), maybeHeader.hash);
    requireFact(row.creating_mtp === windowFacts(row.network, row.mtp_window, headers, Math.max(0, row.height - 1), row.height ? header.parent : row.block_hash), "creating MTP mismatch"); transactions.set(key, row);
  }
  for (const [index, envelope] of context.envelopes.entries()) {
    schema(envelope, ["network", "hash", "height", "parent", "evidence_class", "parent_window", "candidate_history", "expiry_window", "expiry_mtp", "difficulty", "bip34", "external_utxos", "overwrite_original", "expected_flags"]);
    const row = blocks[index]; requireFact(envelope.hash === row.hash && envelope.network === row.network && envelope.height === HEIGHTS[index] && row.height === HEIGHTS[index] && envelope.evidence_class === "independently-seeded-historical", "envelope identity/evidence mismatch");
    const block = decodeBlock(fromHex(row.raw), row.hash); requireFact(envelope.parent === block.parent, "candidate parent mismatch");
    const parentMtp = windowFacts(row.network, envelope.parent_window, headers, row.height - 1, block.parent); requireFact(block.time > parentMtp, "candidate MTP boundary");
    const history = linkedWindow(row.network, envelope.candidate_history, headers, row.height - 1, block.parent);
    requireFact(history.length >= Math.min(155, row.height) && envelope.parent_window.every(hash => envelope.candidate_history.includes(hash)) && envelope.difficulty.every(hash => envelope.candidate_history.includes(hash)), "missing candidate history suffix");
    const expiryHeight = Math.max(0, row.height - 1 - 144); const maybeExpiry = history.find(header => header.height === expiryHeight); requireFact(maybeExpiry, "missing expiry lookback");
    requireFact(envelope.expiry_window.every(hash => envelope.candidate_history.includes(hash)) && envelope.expiry_mtp === windowFacts(row.network, envelope.expiry_window, headers, expiryHeight, maybeExpiry.hash), "expiry MTP mismatch");
    const activation = BIP34[row.network];
    schema(envelope.bip34, row.height - 1 < activation.height ? ["kind"] : ["kind", "hash"]);
    if (row.height - 1 < activation.height) requireFact(envelope.bip34.kind === "absent-parent-below-height", "wrong BIP34 absence fact");
    else requireFact(envelope.bip34.kind === "independent-seeded-anchor" && envelope.bip34.hash === activation.hash && headers.get(headerKey(row.network, activation.hash))?.height === activation.height, "missing/wrong BIP34 ancestor");
    requireFact(JSON.stringify(envelope.expected_flags) === JSON.stringify(flags(row.network, row.height, row.hash)), "source flags mismatch");
    const needed = new Map<string, Transaction["outputs"][number] & { coinbase: boolean; height: number; mtp: number }>(); const seen = new Map<string, Transaction>();
    for (const tx of block.transactions) {
      if (!tx.coinbase) for (const input of tx.inputs) {
        const maybeSame = seen.get(input.txid); if (maybeSame) { requireFact(maybeSame.outputs[input.vout], "wrong same-block prevout"); continue; }
        const maybePrevious = transactions.get(headerKey(row.network, input.txid)); requireFact(maybePrevious && maybePrevious.height < row.height, "missing previous transaction/creating ancestor");
        const prev = decodeTransaction(fromHex(maybePrevious.raw), input.txid); const maybeOutput = prev.outputs[input.vout]; requireFact(maybeOutput, "wrong prevout index");
        const outpoint = `${input.txid}:${input.vout}`; requireFact(!needed.has(outpoint), "duplicate candidate spend");
        needed.set(outpoint, { ...maybeOutput, coinbase: prev.coinbase, height: maybePrevious.height, mtp: maybePrevious.creating_mtp });
      }
      seen.set(tx.txid, tx);
    }
    requireFact(envelope.external_utxos.length === needed.size, "missing/excess historical UTXOs");
    for (const coin of envelope.external_utxos) {
      schema(coin, ["txid", "vout", "value", "script", "coinbase", "created_height", "created_mtp"]);
      const outpoint = `${coin.txid}:${coin.vout}`; const maybeExpected = needed.get(outpoint);
      requireFact(maybeExpected && coin.value === maybeExpected.value && coin.script === maybeExpected.script && coin.coinbase === maybeExpected.coinbase && coin.created_height === maybeExpected.height && coin.created_mtp === maybeExpected.mtp, "wrong historical UTXO/creating MTP"); needed.delete(outpoint);
    }
    checkDifficulty(envelope, block, headers);
    const maybeOriginalHeight = maybeBip30OriginalHeight(row.height); const originals = maybeOriginalHeight === undefined ? [] : blocks.filter(block => block.height === maybeOriginalHeight).map(block => block.hash);
    requireFact(JSON.stringify(envelope.overwrite_original) === JSON.stringify(originals), "BIP30 original prerequisite mismatch");
    for (const hash of originals) { const maybeOriginal = blocks.find(b => b.hash === hash); requireFact(maybeOriginal && decodeBlock(fromHex(maybeOriginal.raw), hash).transactions[0].raw === block.transactions[0].raw, "BIP30 coinbase identity mismatch"); }
  }
}
function checkDifficulty(envelope: Envelope, block: ReturnType<typeof decodeBlock>, headers: Map<string, HeaderRow>) {
  const rows = envelope.difficulty; requireFact(rows.length > 0 && rows.length <= LIMITS.headers && rows.at(-1) === block.parent, "difficulty ancestor bound/endpoint");
  const linked = linkedWindow(envelope.network, rows, headers, envelope.height - 1, block.parent);
  const parent = linked[linked.length - 1]; requireFact(parent.height === envelope.height - 1, "difficulty height mismatch"); let expected = parent.bits;
  if (envelope.height % 2016 === 0) { requireFact(linked[0].height === envelope.height - 2016, "missing retarget anchor"); expected = retarget(parent.bits, parent.time - linked[0].time); }
  else if (envelope.network === "testnet3") {
    if (block.time > parent.time + 1200) expected = 0x1d00ffff;
    else { const first = linked[0]; requireFact(first.height % 2016 === 0 || first.bits !== 0x1d00ffff, "missing minimum-difficulty recovery"); expected = first.bits; }
  }
  requireFact(block.bits === expected, "candidate difficulty mismatch");
}
function acquisitionCache(root: string, initialBytes: number) {
  const directory = resolve(root, LOG, "http-cache"); mkdirSync(directory, { recursive: true });
  const ledger = resolve(root, LOG, "download-budget.json");
  const budget = { bytes: existsSync(ledger) ? JSON.parse(readFileSync(ledger, "utf8")).bytes : initialBytes };
  requireFact(Number.isSafeInteger(budget.bytes) && budget.bytes >= initialBytes && budget.bytes <= LIMITS.download, "invalid cumulative download ledger");
  const responses = new Map<string, ResponseRow>(); const pending = new Map<string, Promise<ResponseRow>>(); let count = 0;
  let nextRequest = 0; let cooldownUntil = 0; let schedule = Promise.resolve();
  function permit() {
    const result = schedule.then(async () => {
      while (Date.now() < Math.max(nextRequest, cooldownUntil)) await Bun.sleep(Math.max(nextRequest, cooldownUntil) - Date.now());
      nextRequest = Date.now() + 500;
    }); schedule = result; return result;
  }
  async function request(url: string): Promise<ResponseRow> {
    endpoint(url); const preferred = url.replace("https://blockstream.info/", "https://mempool.space/");
    for (const candidate of [url, preferred]) {
      const file = resolve(directory, `${sha256(candidate)}.json`); if (!existsSync(file)) continue;
      const row = JSON.parse(readFileSync(file, "utf8")); requireFact(row.url === candidate && sha256(fromHex(row.raw)) === row.sha256, "cached response drift"); responses.set(row.url, row); return row;
    }
    url = preferred; const file = resolve(directory, `${sha256(url)}.json`);
    try {
      let raw: Buffer;
      for (let attempt = 1; ; attempt++) {
        await permit();
        try { raw = await requestBytes(url, budget); break; }
        catch (error) {
          const limited = error as Error & { maybeStatus?: number; maybeRetryAfter?: string };
          if (limited.maybeStatus !== 429) throw error;
          const retryAfter = limited.maybeRetryAfter;
          const wait = retryAfter && /^\d+$/.test(retryAfter) ? Number(retryAfter) * 1000 : retryAfter ? Date.parse(retryAfter) - Date.now() : 30_000;
          requireFact(Number.isFinite(wait) && wait <= 120_000, `rate-limit cooldown outside bounded policy: ${String(error)}`);
          cooldownUntil = Math.max(cooldownUntil, Date.now() + Math.max(30_000, wait));
          const outcome = { url, outcome: "HTTP429", at: new Date().toISOString(), attempt, retry_after: retryAfter ?? "absent", cooldown_ms: cooldownUntil - Date.now() };
          appendFileSync(resolve(root, LOG, "http-outcomes.jsonl"), JSON.stringify(outcome) + "\n"); console.log(JSON.stringify(outcome));
          if (attempt >= 2) throw error;
        }
      }
      const row = { url, acquired_at: new Date().toISOString(), sha256: sha256(raw), raw: raw.toString("hex") };
      writeFileSync(file, JSON.stringify(row)); responses.set(url, row);
      appendFileSync(resolve(root, LOG, "http-outcomes.jsonl"), JSON.stringify({ url, outcome: "success", acquired_at: row.acquired_at, bytes: raw.length }) + "\n");
      if (++count % 100 === 0) console.log(`acquisition requests=${count} cumulative_download_bytes=${budget.bytes}`); return row;
    } catch (error) {
      appendFileSync(resolve(root, LOG, "http-outcomes.jsonl"), JSON.stringify({ url, outcome: "failure", at: new Date().toISOString(), error: String(error) }) + "\n"); throw error;
    } finally { writeFileSync(ledger, JSON.stringify(budget)); }
  }
  return { budget, responses, get(url: string) {
    const maybePending = pending.get(url); if (maybePending) return maybePending;
    const result = request(url); pending.set(url, result); return result;
  } };
}
async function parallel<T>(items: T[], action: (item: T) => Promise<void>) {
  let index = 0; let failed = false;
  const results = await Promise.allSettled(Array.from({ length: 8 }, async () => {
    while (!failed && index < items.length) { const item = items[index++]; try { await action(item); } catch (error) { failed = true; throw error; } }
  }));
  for (const result of results) if (result.status === "rejected") throw result.reason;
}
function resourceFit(context: Contexts) {
  return { max_header_walk: Math.max(...context.envelopes.map(e => e.candidate_history.length), ...context.envelopes.map(e => e.difficulty.length), 11),
    headers: context.headers.length, previous_transactions: context.transactions.length, utxos: context.envelopes.reduce((sum, e) => sum + e.external_utxos.length, 0),
    ancestry_authority: "seeded-segments-only", utxo_scope: "candidate-inputs-and-known-overwrites", actual_daemon: false, whole_chain: false };
}
async function acquire(root: string) {
  if (!existsSync(resolve(root, DATA, "blocks.json"))) await acquireBodies(root);
  const blocks = load(root, `${DATA}/blocks.json`).blocks as RawRow[];
  const manifest = load(root, `${DATA}/manifest.json`); manifest.sources = sourceFacts(root); save(root, `${DATA}/manifest.json`, manifest); checkBodies(root);
  const cache = acquisitionCache(root, manifest.downloaded_bytes); const headers = new Map<string, HeaderRow>(); const heights = new Map<string, HeaderRow>();
  const context: Contexts = { schema: 1, evidence_class: "independently-seeded-historical", headers: [], transactions: [], responses: [], envelopes: [] };
  async function headerAt(network: Network, height: number, maybeExpected?: string): Promise<HeaderRow> {
    requireFact(Number.isSafeInteger(height) && height >= 0, "invalid seed header height");
    const batchHeight = Math.floor(height / 10) * 10 + 9;
    const response = await cache.get(`${base(network)}/blocks/${batchHeight}`); const rows = JSON.parse(fromHex(response.raw).toString());
    requireFact(Array.isArray(rows) && rows.length === 10, "header batch bounds");
    for (const meta of rows) {
      requireFact(meta.height > batchHeight - 10 && meta.height <= batchHeight, "header batch height range");
      const raw = metadataHeader(meta); const header = { network, height: meta.height, hash: meta.id, raw: raw.toString("hex"), response: response.url };
      headers.set(headerKey(network, meta.id), header); heights.set(`${network}:${meta.height}`, header);
    }
    const maybeRow = heights.get(`${network}:${height}`);
    requireFact(maybeRow && (!maybeExpected || maybeRow.hash === maybeExpected), "seed header identity mismatch"); return maybeRow;
  }
  async function window(network: Network, height: number, expected: string) {
    const rows: string[] = [];
    for (let h = Math.max(0, height - 10); h <= height; h++) rows.push((await headerAt(network, h)).hash);
    windowFacts(network, rows, headers, height, expected); return rows;
  }
  try {
    // These disconnected anchor windows are explicit seeded prerequisites, never canonical lineage.
    for (const [i, row] of blocks.entries()) {
      requireFact(row.height === HEIGHTS[i], "frozen historical height mismatch"); const block = decodeBlock(fromHex(row.raw), row.hash);
      const parent = await headerAt(row.network, row.height - 1, block.parent); const parentWindow = await window(row.network, parent.height, parent.hash);
      const difficulty: string[] = [parent.hash]; let cursor = parent;
      if (row.height % 2016 === 0 || row.network === "testnet3" && block.time <= decodeHeader(fromHex(parent.raw, 80), parent.hash).time + 1200) {
        while (row.height % 2016 === 0 ? cursor.height > row.height - 2016 : cursor.height % 2016 !== 0 && fromHex(cursor.raw, 80).readUInt32LE(72) === 0x1d00ffff) {
          requireFact(difficulty.length < LIMITS.headers, "difficulty walk bound"); cursor = await headerAt(row.network, cursor.height - 1); difficulty.unshift(cursor.hash);
        }
      }
      const activation = BIP34[row.network]; let bip34: Envelope["bip34"] = { kind: "absent-parent-below-height" };
      if (row.height - 1 >= activation.height) { await headerAt(row.network, activation.height, activation.hash); bip34 = { kind: "independent-seeded-anchor", hash: activation.hash }; }
      const oldest = Math.min(Math.max(0, parent.height - 154), cursor.height); const candidateHistory: string[] = [];
      for (let h = oldest; h <= parent.height; h++) { requireFact(candidateHistory.length < LIMITS.headers, "candidate history walk bound"); candidateHistory.push((await headerAt(row.network, h)).hash); }
      linkedWindow(row.network, candidateHistory, headers, parent.height, parent.hash);
      const expiryHeader = await headerAt(row.network, Math.max(0, parent.height - 144)); const expiryWindow = await window(row.network, expiryHeader.height, expiryHeader.hash);
      const maybeOriginalHeight = maybeBip30OriginalHeight(row.height);
      context.envelopes.push({ network: row.network, hash: row.hash, height: row.height, parent: block.parent, evidence_class: "independently-seeded-historical", parent_window: parentWindow,
        candidate_history: candidateHistory, expiry_window: expiryWindow, expiry_mtp: windowFacts(row.network, expiryWindow, headers, expiryHeader.height, expiryHeader.hash), difficulty, bip34, external_utxos: [],
        overwrite_original: maybeOriginalHeight === undefined ? [] : blocks.filter(block => block.height === maybeOriginalHeight).map(block => block.hash), expected_flags: flags(row.network, row.height, row.hash) });
    }
    const needed = new Map<string, { network: Network; txid: string }>();
    for (const row of blocks) {
      const block = decodeBlock(fromHex(row.raw), row.hash); const seen = new Set<string>();
      for (const tx of block.transactions) {
        if (!tx.coinbase) for (const input of tx.inputs) if (!seen.has(input.txid)) needed.set(headerKey(row.network, input.txid), { network: row.network, txid: input.txid });
        seen.add(tx.txid);
      }
    }
    console.log(`required previous transactions=${needed.size}; seeded parent/activation headers=${headers.size}`);
    await parallel([...needed.values()], async ({ network, txid }) => {
      const response = await cache.get(`${base(network)}/tx/${txid}/raw`); decodeTransaction(fromHex(response.raw), txid);
      const statusResponse = await cache.get(`${base(network)}/tx/${txid}/status`); const status = JSON.parse(fromHex(statusResponse.raw).toString());
      requireFact(status.confirmed && Number.isSafeInteger(status.block_height) && status.block_height > 0, "unconfirmed/missing creating fact");
      const proof = await cache.get(`${base(network)}/tx/${txid}/merkle-proof`);
      const header = await headerAt(network, status.block_height, status.block_hash); const facts = decodeHeader(fromHex(header.raw, 80), header.hash);
      const mtpWindow = await window(network, header.height - 1, facts.parent);
      context.transactions.push({ network, txid, raw: response.raw, response: response.url, status: statusResponse.url, proof: proof.url, block_hash: header.hash, height: header.height,
        mtp_window: mtpWindow, creating_mtp: windowFacts(network, mtpWindow, headers, header.height - 1, facts.parent) });
      if (context.transactions.length % 500 === 0) console.log(`assembled_previous_transactions=${context.transactions.length}/${needed.size}; seeded_headers=${headers.size}`);
    });
    const prevs = new Map(context.transactions.map(row => [headerKey(row.network, row.txid), row]));
    for (const [i, row] of blocks.entries()) {
      const seen = new Set<string>();
      for (const tx of decodeBlock(fromHex(row.raw), row.hash).transactions) {
        if (!tx.coinbase) for (const input of tx.inputs) {
          if (seen.has(input.txid)) continue;
          const maybePrev = prevs.get(headerKey(row.network, input.txid)); requireFact(maybePrev, "missing previous transaction");
          const prev = decodeTransaction(fromHex(maybePrev.raw), input.txid); const maybeOutput = prev.outputs[input.vout]; requireFact(maybeOutput, "missing previous output");
          context.envelopes[i].external_utxos.push({ txid: input.txid, vout: input.vout, ...maybeOutput, coinbase: prev.coinbase, created_height: maybePrev.height, created_mtp: maybePrev.creating_mtp });
        }
        seen.add(tx.txid);
      }
    }
    context.headers = [...headers.values()].sort((a, b) => a.network.localeCompare(b.network) || a.height - b.height);
    context.transactions.sort((a, b) => a.network.localeCompare(b.network) || a.txid.localeCompare(b.txid));
    const responseTransactions = new Map(context.transactions.map(row => [row.response, row]));
    context.responses = [...cache.responses.values()].sort((a, b) => a.url.localeCompare(b.url)).map(row => {
      const maybeTx = responseTransactions.get(row.url); if (!maybeTx || maybeTx.raw !== row.raw) return row;
      return { url: row.url, acquired_at: row.acquired_at, sha256: row.sha256, transaction: headerKey(maybeTx.network, maybeTx.txid) };
    });
    checkContexts(context, blocks);
    const expectations = sourceExpectations(); const expectationsFile = "docs/parity/fixtures/phase167-rule-expectations.json";
    const contextText = JSON.stringify(context, null, 2) + "\n"; const expectationsText = JSON.stringify(expectations, null, 2) + "\n";
    manifest.contexts_sha256 = sha256(contextText); manifest.expectations_sha256 = sha256(expectationsText); manifest.downloaded_bytes = cache.budget.bytes;
    manifest.resource_fit = resourceFit(context); const payloadBytes = statSync(resolve(root, `${DATA}/blocks.json`)).size + Buffer.byteLength(contextText) + Buffer.byteLength(expectationsText);
    manifest.corpus_bytes = 0;
    for (let count = 0; count < 3; count++) manifest.corpus_bytes = payloadBytes + Buffer.byteLength(JSON.stringify(manifest, null, 2) + "\n");
    requireFact(manifest.corpus_bytes === payloadBytes + Buffer.byteLength(JSON.stringify(manifest, null, 2) + "\n") && manifest.corpus_bytes <= LIMITS.corpus && manifest.downloaded_bytes <= LIMITS.download, "total corpus/resource bound");
    writeFileSync(resolve(root, `${DATA}/contexts.json`), contextText); writeFileSync(resolve(root, expectationsFile), expectationsText);
    save(root, `${DATA}/manifest.json`, manifest); console.log(JSON.stringify(manifest.resource_fit));
  } catch (error) {
    save(root, `${LOG}/acquisition-failure-${Date.now()}.json`, { outcome: "failure", error: String(error), downloaded_bytes: cache.budget.bytes, completed_prevtxs: context.transactions.length, required_hashes: REQUIRED }); throw error;
  }
}
if (import.meta.main) {
  try {
    requireFact(Bun.argv.length === 3 && ["--acquire", "--check"].includes(Bun.argv[2]), "usage: phase167-historical-corpus.ts --acquire|--check");
    if (Bun.argv[2] === "--acquire") await acquire(ROOT); else console.log(JSON.stringify(check()));
  } catch (error) { console.error(String(error)); process.exitCode = 1; }
}

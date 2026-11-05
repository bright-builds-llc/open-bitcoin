#!/usr/bin/env bun
import { readFileSync, statSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { check, decodeHeader, decodeBlock, maybeBip30OriginalHeight, PIN, sha256 } from "./phase167-historical-corpus";

const ROOT = resolve(import.meta.dir, "..");
export const OUTPUT = "packages/testdata/phase167/bip30-fixtures.txt";
export const BOUNDS = { bytes: 1_048_576, records: 1024, line: 16384, transactions: 32 } as const;
const DATA = "packages/testdata/phase167/";
export const FIXED_INPUTS: readonly [string, number, string][] = [
  ["scripts/phase167-historical-corpus.ts", 52343, "5f9031294493bd42be0ff4a29bf6c4501fc9fb0b1ad8c7d0a8b720ffe7d0459b"],
  [DATA + "manifest.json", 6034, "16649d2cee73bfd769632bd8b59a06af50ee2d8bb69a2a71dcaad8be88d53b09"],
  [DATA + "blocks.json", 2892399, "6e81f7f06037d8c1e18b75ddef67d16aa6ab09a505ca616db612de1f29e0c335"],
  [DATA + "contexts.json", 77269907, "3067e98fb235c97226482a6c1b9fad2484fe58610f5ed1c25e8860a0a970fd5a"],
  ["docs/parity/fixtures/phase167-rule-expectations.json", 22207, "4dddecfbb11a93eac11474d23f80bfff08736713a02941f7f21feb32b7a88c24"],
];
const PINNED_SOURCES: readonly [string, string][] = [
  ["src/validation.cpp", "5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495"],
  ["src/kernel/chainparams.cpp", "17170008ef7abe50f1fbde9a854e443b7d0b1119780c8a58329538e8ee038485"],
  ["src/consensus/tx_verify.cpp", "8fbc9bde843c2cfb0577ae7d179ad11f928b09bf8b0fc3e43a9ede7dc34a7370"],
  ["src/pow.cpp", "9b27f47410b0906b30f12ead4a95e4d345f7832606838f1440ef35b84c708fe2"],
  ["src/arith_uint256.cpp", "33099a41d032d02f49f2d0269f31749c6b9d7cee3fdc8ce0c3cadfe46786d1a8"],
  ["src/coins.cpp", "12bce0202076a4dd462f16833a6a8f650fe1f6cb283cb2e9955d31542128a95d"],
  ["src/chain.h", "c0188574b418a1b67b66d17b4cdb46eb1738bc6e6155514c3d11cc3235db5b1c"],
  ["src/deploymentstatus.h", "7cb50f0350d1a0dfa3d10ecd9c9c74106505662e921c695e6932d69a312dbe09"],
  ["src/script/interpreter.h", "885ccdca3743cdbd659ae82cd6b77aca299472c6b0d8deb014e4775d8bfdbed3"],
  ["src/clientversion.h", "a4ba67ee83b0debad634e7ea7316c6392a8a35dfa130af126313a059413736fa"],
  ["src/clientversion.cpp", "e4c8dab790024c5221faaac423d7fbc42ec9e12e4a660272592babaaf199c568"],
  ["CMakeLists.txt", "91e3f514a72f254e1728f4e9c306cb0a1cd669cff08f7d24b3ae99097f08947b"],
];
const SOURCES = [...FIXED_INPUTS.map(([path, , digest]) => ({ path, digest })),
  ...PINNED_SOURCES.map(([path, digest]) => ({ path: "packages/bitcoin-knots/" + path, digest }))]
  .sort((a, b) => a.path < b.path ? -1 : a.path > b.path ? 1 : 0);
const IDENTITIES = [
  { height: 91842, hash: "00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec", bytes: 214,
    digest: "95434eb656dd8feb2611648dc54f4d85d5fad64eaa6a892c89b09888c60fcd93", observed: "2026-10-21T17:30:28.320Z" },
  { height: 91880, hash: "00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721", bytes: 472,
    digest: "f93f123ca5a9fb01cef7cf5b9caec38619b3c9a6d13eaa8dc65c1753c253f4a2", observed: "2026-10-21T17:30:28.445Z" },
  { height: 91812, hash: "00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f", bytes: 616,
    digest: "19e5a91add4003e8a102b9392fdf3e53c422ed9deae39b519dd353a70555695e", observed: "2026-10-21T17:30:28.704Z" },
  { height: 91722, hash: "00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e", bytes: 214,
    digest: "c7acdfd2acb2da89dfbdb01f3c6405cf7100108aaf09b4f1031592e79d244e8e", observed: "2026-10-21T17:30:28.575Z" },
] as const;
const ANCHOR = "000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8";
type RawHeader = { network: string; height: number; hash: string; raw: string };
type Body = RawHeader & { sha256: string; acquired_at: string };
type Coin = { txid: string; vout: number; value: string; script: string; coinbase: boolean; created_height: number; created_mtp: number };
type Envelope = { network: string; hash: string; height: number; parent: string; parent_window: string[];
  candidate_history: string[]; expiry_window: string[]; expiry_mtp: number; external_utxos: Coin[]; overwrite_original: string[] };
export type ProjectionInputs = { pin: string; evidence: string; sources: { path: string; digest: string }[];
  bodies: Body[]; headers: RawHeader[]; envelopes: Envelope[] };
export type Header = { network: string; height: number; hash: string; version: number; previousHash: string;
  merkleRoot: string; time: number; bits: number; nonce: number };
type TypedTransaction = Omit<ReturnType<typeof decodeBlock>["transactions"][number], "raw"> & { version: number; lockTime: number };
type Case = { height: number; hash: string; originalHeight: number; originalHash: string;
  repeatBytes: number; repeatDigest: string; originalBytes: number; originalDigest: string; observed: number;
  parentMtp: number; creationMtp: number; expiryMtp: number; candidate: Header; originalHeader: Header;
  history: Header[]; mtpPrefix: Header[]; originalMtp: Header[]; repeat: TypedTransaction[]; original: TypedTransaction[];
  overwritten: Coin[]; external: Coin[] };
export type Projection = { sources: ProjectionInputs["sources"]; anchor: Header; cases: Case[] };
const EXTERNAL: Coin = { txid: "cae24697a15419b1de45c99a41149101c252c2dbd3fd0bbce06ac5809d4547e3", vout: 1,
  value: "4480000000", script: "76a914b5635b9c16b17bf34f29fdc999b440e0638c5b9088ac", coinbase: false, created_height: 90644, created_mtp: 1289277562 };
function requireFact(ok: unknown, message: string): asserts ok { if (!ok) throw new Error(message); }
function same(actual: unknown, expected: unknown, message: string) {
  requireFact(JSON.stringify(actual) === JSON.stringify(expected), message);
}
function hex(value: string, maybeBytes?: number) {
  requireFact(typeof value === "string" && value.length <= BOUNDS.line && value.length % 2 === 0 && /^[0-9a-f]*$/.test(value)
    && (maybeBytes === undefined || value.length === maybeBytes * 2), "bounded lowercase hex required");
  return Buffer.from(value, "hex");
}
function integer(value: number, minimum: number, maximum: number, label: string) {
  requireFact(Number.isSafeInteger(value) && value >= minimum && value <= maximum, label + " range");
}
function header(row: RawHeader): Header {
  requireFact(row.network === "mainnet", "header network mismatch"); integer(row.height, 0, 0xffffffff, "height");
  const raw = hex(row.raw, 80); const facts = decodeHeader(raw, row.hash);
  return { network: row.network, height: row.height, hash: facts.hash, version: facts.version, previousHash: facts.parent,
    merkleRoot: Buffer.from(raw.subarray(36, 68)).reverse().toString("hex"), time: facts.time, bits: facts.bits, nonce: raw.readUInt32LE(76) };
}
function median(rows: Header[]) {
  requireFact(rows.length === 11, "MTP requires eleven real headers");
  return rows.map(row => row.time).sort((a, b) => a - b)[5];
}
function transactionFields(body: Body): TypedTransaction[] {
  return decodeBlock(hex(body.raw), body.hash).transactions.map(tx => {
    const raw = hex(tx.raw);
    return { txid: tx.txid, wtxid: tx.wtxid, coinbase: tx.coinbase, inputs: tx.inputs, outputs: tx.outputs,
      witness: tx.witness, version: raw.readInt32LE(), lockTime: raw.readUInt32LE(raw.length - 4) };
  });
}
function walk(map: Map<string, Header>, hashes: string[], height: number, endpoint: string, count: number): Header[] {
  requireFact(hashes.length === count, "window count mismatch");
  const rows = hashes.map((hash, ordinal) => {
    const maybeRow = map.get(hash);
    requireFact(maybeRow && maybeRow.height === height - ordinal, "missing/reordered window header"); return maybeRow;
  });
  requireFact(rows[0].hash === endpoint, "window endpoint mismatch");
  for (let i = 0; i + 1 < rows.length; i++) requireFact(rows[i].previousHash === rows[i + 1].hash, "window parent link mismatch");
  return rows;
}
function prefix(map: Map<string, Header>, oldest: Header): Header[] {
  const rows: Header[] = []; let hash = oldest.previousHash;
  for (let i = 0; i < 10; i++) {
    const maybeRow = map.get(hash);
    requireFact(maybeRow && maybeRow.height === oldest.height - i - 1, "missing/wrong MTP prefix");
    rows.push(maybeRow); hash = maybeRow.previousHash;
  }
  return rows;
}
function buildCase(input: ProjectionInputs, map: Map<string, Header>, index: number): Case {
  const body = input.bodies[index]; const oldBody = input.bodies[index + 2];
  const envelope = input.envelopes[index]; const originalEnvelope = input.envelopes[index + 2];
  const expectedOriginal = index === 0 ? 91812 : 91722;
  requireFact(oldBody.height === expectedOriginal && maybeBip30OriginalHeight(body.height) === expectedOriginal, "BIP30 original pair mismatch");
  same(envelope.overwrite_original, [oldBody.hash], "BIP30 overwrite pair mismatch");
  const candidate = header({ ...body, raw: body.raw.slice(0, 160) });
  const originalHeader = header({ ...oldBody, raw: oldBody.raw.slice(0, 160) });
  requireFact(envelope.parent === candidate.previousHash && originalEnvelope.parent === originalHeader.previousHash, "envelope parent mismatch");
  const history = walk(map, [...envelope.candidate_history].reverse(), body.height - 1, candidate.previousHash, 155);
  const mtpPrefix = prefix(map, history[154]);
  const support = [...history, ...mtpPrefix];
  const parentWindow = walk(map, [...envelope.parent_window].reverse(), body.height - 1, candidate.previousHash, 11);
  same(parentWindow, history.slice(0, 11), "parent MTP window mismatch");
  const expiry = walk(map, [...envelope.expiry_window].reverse(), body.height - 145, history[144].hash, 11);
  same(expiry, history.slice(144, 155), "expiry window mismatch");
  const originalMtp = walk(map, [...originalEnvelope.parent_window].reverse(), oldBody.height - 1, originalHeader.previousHash, 11);
  const creationMtp = median(originalMtp); const parentMtp = median(parentWindow); const expiryMtp = median(expiry);
  requireFact(expiryMtp === envelope.expiry_mtp && candidate.time > parentMtp, "MTP field mismatch");
  // Validate every real stored row's complete eleven-sample clock, including the oldest row.
  for (let i = 0; i < history.length; i++) median(support.slice(i, i + 11));
  const repeat = transactionFields(body); const original = transactionFields(oldBody);
  requireFact(repeat.length <= BOUNDS.transactions && original.length <= BOUNDS.transactions, "transaction count bound");
  same(repeat[0], original[0], "BIP30 full coinbase mismatch");
  const overwritten = original[0].outputs.map((output, vout) => ({ txid: original[0].txid, vout, ...output,
    coinbase: true, created_height: oldBody.height, created_mtp: creationMtp }));
  same(envelope.external_utxos, index === 0 ? [] : [EXTERNAL], "external Coin field mismatch");
  const seen = new Set<string>(); const needed = new Set<string>();
  for (const tx of repeat) {
    if (!tx.coinbase) for (const item of tx.inputs) {
      if (seen.has(item.txid)) continue;
      const outpoint = `${item.txid}:${item.vout}`;
      requireFact(!needed.has(outpoint), "duplicate external spend"); needed.add(outpoint);
    }
    requireFact(!seen.has(tx.txid), "duplicate transaction within role"); seen.add(tx.txid);
  }
  const external = envelope.external_utxos;
  same([...needed].sort(), external.map(coin => `${coin.txid}:${coin.vout}`).sort(), "missing/excess external Coin");
  return { height: body.height, hash: body.hash, originalHeight: oldBody.height, originalHash: oldBody.hash,
    repeatBytes: body.raw.length / 2, repeatDigest: body.sha256, originalBytes: oldBody.raw.length / 2,
    originalDigest: oldBody.sha256, observed: Math.floor(Date.parse(body.acquired_at) / 1000), parentMtp, creationMtp, expiryMtp,
    candidate, originalHeader, history, mtpPrefix, originalMtp, repeat, original, overwritten, external };
}

/** Pure finite projection. Accepted raw bytes remain the sole transaction decoder input. */
export function validateProjectionInputs(input: ProjectionInputs): Projection {
  requireFact(input.pin === PIN && input.evidence === "independently-seeded-historical", "pin/evidence mismatch");
  same(input.sources, SOURCES, "fixed source digest/path mismatch");
  requireFact(input.bodies.length === 4 && input.envelopes.length === 4, "body/envelope count mismatch");
  for (const [i, body] of input.bodies.entries()) {
    const expected = IDENTITIES[i]; const raw = hex(body.raw);
    requireFact(body.network === "mainnet" && body.height === expected.height && body.hash === expected.hash, "body network/identity/order mismatch");
    requireFact(raw.length === expected.bytes && body.sha256 === expected.digest && sha256(raw) === expected.digest, "body bytes/digest mismatch");
    requireFact(body.acquired_at === expected.observed, "frozen observation mismatch");
    const envelope = input.envelopes[i];
    requireFact(envelope.network === body.network && envelope.height === body.height && envelope.hash === body.hash, "envelope identity mismatch");
  }
  requireFact(input.headers.length <= 512, "selected header count bound");
  const map = new Map<string, Header>();
  for (const row of input.headers) {
    requireFact(!map.has(row.hash), "duplicate header identity"); map.set(row.hash, header(row));
  }
  const maybeAnchor = map.get(ANCHOR);
  requireFact(maybeAnchor && maybeAnchor.height === 227931, "missing/wrong detached BIP34 anchor");
  const cases = [buildCase(input, map, 0), buildCase(input, map, 1)];
  const used = new Set([ANCHOR, ...cases.flatMap(row => [...row.history, ...row.mtpPrefix, ...row.originalMtp].map(h => h.hash))]);
  requireFact(used.size === map.size && [...map.keys()].every(hash => used.has(hash)), "unexpected selected header");
  return { sources: input.sources, anchor: maybeAnchor, cases };
}

/** Shell: validate the entire accepted corpus before selecting a small finite input carrier. */
export function loadProjectionInputs(root = ROOT): ProjectionInputs {
  const result = check(root);
  requireFact(result.blocks === 7 && result.previous_transactions === 5263 && result.corpus_bytes === 80190547
    && result.evidence_class === "independently-seeded-historical", "incomplete accepted corpus");
  const data = new Map<string, Buffer>();
  for (const [path, bytes, digest] of FIXED_INPUTS) {
    requireFact(statSync(resolve(root, path)).size === bytes, "fixed source byte count mismatch: " + path);
    const raw = readFileSync(resolve(root, path));
    requireFact(sha256(raw) === digest, "fixed source digest mismatch: " + path); data.set(path, raw);
  }
  const manifest = JSON.parse(data.get(DATA + "manifest.json")!.toString());
  same(manifest.sources, PINNED_SOURCES.map(([file, sha256]) => ({ file, sha256 })), "pinned source inventory mismatch");
  const corpus: { blocks: Body[] } = JSON.parse(data.get(DATA + "blocks.json")!.toString());
  const context: { headers: RawHeader[]; envelopes: Envelope[] } = JSON.parse(data.get(DATA + "contexts.json")!.toString());
  const bodies = IDENTITIES.map(identity => {
    const maybeBody = corpus.blocks.find(row => row.network === "mainnet" && row.hash === identity.hash);
    requireFact(maybeBody, "missing selected body"); return maybeBody;
  });
  const envelopes = bodies.map(body => {
    const maybeEnvelope = context.envelopes.find(row => row.network === "mainnet" && row.hash === body.hash);
    requireFact(maybeEnvelope, "missing selected envelope"); return maybeEnvelope;
  });
  const all = new Map(context.headers.filter(row => row.network === "mainnet").map(row => [row.hash, row]));
  const selected = new Map<string, RawHeader>();
  const take = (hash: string) => {
    const maybeRow = all.get(hash); requireFact(maybeRow, "missing selected source header");
    const row = { network: maybeRow.network, height: maybeRow.height, hash: maybeRow.hash, raw: maybeRow.raw };
    selected.set(hash, row); return row;
  };
  take(ANCHOR);
  for (const [index, envelope] of envelopes.entries()) {
    for (const hash of index < 2 ? envelope.candidate_history : envelope.parent_window) take(hash);
    if (index >= 2) continue;
    let row = take(envelope.candidate_history[0]);
    for (let i = 0; i < 10; i++) row = take(decodeHeader(hex(row.raw, 80), row.hash).parent);
  }
  const input = { pin: manifest.pin, evidence: manifest.evidence_class, sources: SOURCES.map(row => ({ ...row })),
    bodies, envelopes, headers: [...selected.values()] };
  validateProjectionInputs(input); return input;
}

const DOCUMENTATION = [
  "# ASCII/LF; tab-separated fields; closed tags/order/counts. No escaping or executable data.",
  "# Empty byte strings: -; other bytes: lowercase even hex. Hashes: 64 lowercase DISPLAY-order hex.",
  "# Rust reverses each hash once into internal Hash32; scripts/witness never reverse.",
  "# Decimal: versions signed i32; other wire scalars u32; satoshis u64/Amount; clocks signed i64.",
  "# PIN pin; SOURCE fixed_repo_relative_path sha256 (sorted closed allowlist).",
  "# HEADER role ordinal network height hash version previous_hash merkle_root time bits nonce.",
  "# detached_bip34: one independent 227931 seed before CASE; no branch/canonical authority.",
  "# CASE network repeat_height repeat_hash original_height original_hash repeat_bytes repeat_sha256 original_bytes original_sha256 frozen_observation parent_mtp original_creation_mtp expiry_mtp.",
  "# Per CASE HEADER roles: candidate/original=1 each; history=155 parent-first; mtp_prefix=10 descending raw support; original_mtp=11 descending.",
  "# Only history becomes stored positions; prefix stays raw. For history[i], median times of (history+prefix)[i..i+11].",
  "# TX role tx_index version lock_time txid wtxid input_count output_count; repeat then original, all transactions.",
  "# IN role tx_index input_index previous_txid vout script_sig sequence witness_count.",
  "# WITNESS role tx_index input_index item_index bytes; follows its IN before next IN; OUT follows all inputs.",
  "# OUT role tx_index output_index satoshis script_pubkey. Coinbases keep shared txid across original/repeat roles.",
  "# COIN role txid vout satoshis script coinbase_0_or_1 created_height created_mtp; overwritten then external.",
  "# END_CASE repeat_height; END case_count=2. Record identity is case+role+ordinal, not global txid.",
  "# Bounds: 1MiB file; 1024 records; 16KiB line; 32 transactions/body. No work/flags/canonical markers.",
];
const bytesField = (bytes: string) => bytes === "" ? "-" : bytes;
function headerRecord(role: string, ordinal: number, h: Header): (string | number)[] {
  return ["HEADER", role, ordinal, h.network, h.height, h.hash, h.version, h.previousHash, h.merkleRoot, h.time, h.bits, h.nonce];
}
function renderProjection(projection: Projection): string {
  const lines = [...DOCUMENTATION, "OPEN_BITCOIN_PHASE167_BIP30\t1"];
  const row = (...fields: (string | number)[]) => { lines.push(fields.join("\t")); };
  row("PIN", PIN); for (const source of projection.sources) row("SOURCE", source.path, source.digest);
  row(...headerRecord("detached_bip34", 0, projection.anchor));
  for (const item of projection.cases) {
    row("CASE", "mainnet", item.height, item.hash, item.originalHeight, item.originalHash, item.repeatBytes, item.repeatDigest,
      item.originalBytes, item.originalDigest, item.observed, item.parentMtp, item.creationMtp, item.expiryMtp);
    row(...headerRecord("candidate", 0, item.candidate)); row(...headerRecord("original", 0, item.originalHeader));
    for (const [role, headers] of [["history", item.history], ["mtp_prefix", item.mtpPrefix], ["original_mtp", item.originalMtp]] as const)
      headers.forEach((h, i) => row(...headerRecord(role, i, h)));
    for (const role of ["repeat", "original"] as const) item[role].forEach((tx, i) => {
      row("TX", role, i, tx.version, tx.lockTime, tx.txid, tx.wtxid, tx.inputs.length, tx.outputs.length);
      tx.inputs.forEach((input, j) => {
        const stack = tx.witness.length === 0 ? [] : tx.witness[j];
        row("IN", role, i, j, input.txid, input.vout, bytesField(input.script), input.sequence, stack.length);
        stack.forEach((value, k) => row("WITNESS", role, i, j, k, bytesField(value)));
      });
      tx.outputs.forEach((out, j) => row("OUT", role, i, j, out.value, bytesField(out.script)));
    });
    for (const role of ["overwritten", "external"] as const) for (const coin of item[role])
      row("COIN", role, coin.txid, coin.vout, coin.value, bytesField(coin.script), coin.coinbase ? 1 : 0, coin.created_height, coin.created_mtp);
    row("END_CASE", item.height);
  }
  row("END", projection.cases.length); return lines.join("\n") + "\n";
}
/** Pure render always revalidates its finite input; it cannot serialize caller rules or authority. */
export function renderBip30Fixtures(input: ProjectionInputs): string {
  const text = renderProjection(validateProjectionInputs(input)); validateProjectionText(text, text); return text;
}
function numberField(value: string, minimum: bigint, maximum: bigint) {
  requireFact(/^(0|[1-9][0-9]*|-[1-9][0-9]*)$/.test(value), "noncanonical integer field");
  const number = BigInt(value); requireFact(number >= minimum && number <= maximum, "integer field range");
}
function recordGrammar(fields: string[]) {
  const descriptors: Record<string, string[]> = {
    OPEN_BITCOIN_PHASE167_BIP30: ["1"], PIN: ["pin"], SOURCE: ["source", "h"],
    HEADER: ["candidate|original|history|mtp_prefix|original_mtp|detached_bip34", "u", "mainnet", "u", "h", "i", "h", "h", "u", "u", "u"],
    CASE: ["mainnet", "u", "h", "u", "h", "u", "h", "u", "h", "c", "c", "c", "c"],
    TX: ["repeat|original", "u", "i", "u", "h", "h", "u", "u"],
    IN: ["repeat|original", "u", "u", "h", "u", "x", "u", "u"],
    WITNESS: ["repeat|original", "u", "u", "u", "x"], OUT: ["repeat|original", "u", "u", "a", "x"],
    COIN: ["overwritten|external", "h", "u", "a", "x", "0|1", "u", "c"], END_CASE: ["u"], END: ["u"],
  };
  const maybeDescriptor = Object.hasOwn(descriptors, fields[0]) ? descriptors[fields[0]] : undefined;
  requireFact(maybeDescriptor, "unknown projection tag");
  requireFact(fields.length === maybeDescriptor.length + 1, "projection tag arity mismatch");
  for (const [index, kind] of maybeDescriptor.entries()) {
    const value = fields[index + 1];
    if (kind === "u") numberField(value, 0n, 0xffffffffn);
    else if (kind === "i") numberField(value, -0x80000000n, 0x7fffffffn);
    else if (kind === "c") numberField(value, -(1n << 63n), (1n << 63n) - 1n);
    else if (kind === "a") numberField(value, 0n, 2_100_000_000_000_000n);
    else if (kind === "h") hex(value, 32);
    else if (kind === "x") { requireFact(value.length > 0, "empty bytes must use -"); if (value !== "-") hex(value); }
    else if (kind === "pin") requireFact(value === PIN, "projection source pin mismatch");
    else if (kind === "source") requireFact(SOURCES.some(row => row.path === value && row.digest === fields[2]), "projection source digest/path mismatch");
    else requireFact(kind.split("|").includes(value), "projection literal/role mismatch");
  }
}
/** Closed-format validation uses the complete fresh expected record sequence, including all ordinals. */
export function validateProjectionText(text: string, expected: string): void {
  requireFact(Buffer.byteLength(text) <= BOUNDS.bytes, "projection byte bound");
  requireFact(/^[\x09\x0a\x20-\x7e]*$/.test(text) && text.endsWith("\n"), "projection ASCII/LF encoding");
  const lines = text.slice(0, -1).split("\n");
  requireFact(lines.every(line => line.length <= BOUNDS.line), "projection line bound");
  requireFact(lines.filter(line => !line.startsWith("#")).length <= BOUNDS.records, "projection record bound");
  const records = lines.filter(line => !line.startsWith("#"));
  requireFact(records[0] === "OPEN_BITCOIN_PHASE167_BIP30\t1", "projection magic/version mismatch");
  let started = false;
  for (const line of lines) {
    if (line.startsWith("#")) { requireFact(!started, "documentation must precede records"); continue; }
    started = true; recordGrammar(line.split("\t"));
  }
  const wanted = expected.slice(0, -1).split("\n");
  requireFact(lines.length === wanted.length, "projection record count mismatch");
  for (const [i, line] of lines.entries()) requireFact(line === wanted[i], `projection record mismatch at line ${i + 1} (${line.split("\t")[0]})`);
}
/** Shell: fixed output only, after full accepted-corpus and finite projection validation. */
export function runProjection(args: string[], root = ROOT): { bytes: number; digest: string; cases: number } {
  requireFact(args.length === 1 && (args[0] === "--check" || args[0] === "--write"), "use exactly --check or --write");
  const expected = renderBip30Fixtures(loadProjectionInputs(root)); const output = resolve(root, OUTPUT);
  if (args[0] === "--write") writeFileSync(output, expected);
  else {
    requireFact(statSync(output).size <= BOUNDS.bytes, "projection byte bound");
    validateProjectionText(readFileSync(output, "utf8"), expected);
  }
  return { bytes: Buffer.byteLength(expected), digest: sha256(expected), cases: 2 };
}
export function checkProjection(root = ROOT) { return runProjection(["--check"], root); }
if (import.meta.main) {
  try { console.log(JSON.stringify(runProjection(process.argv.slice(2)))); }
  catch (error) { console.error(String(error)); process.exitCode = 1; }
}

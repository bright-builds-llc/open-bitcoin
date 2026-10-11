#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, readFileSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { foreignGitEnvironment } from "./generate-basic-filter-vectors";

export const PIN = "a9aee730466ac67d35a3c03ee24676be5e045878";
export const WIRE_PATH = "docs/parity/fixtures/peer-filter-wire.json";
export const PROOF_PATH = "docs/parity/fixtures/peer-filter-proof.json";
export const LIMITS = { cases: 256, payload: 4_000_000, input: 8 * 1024 * 1024,
  stdout: 16 * 1024 * 1024, golden: 16 * 1024 * 1024, timeout: 10_000 } as const;
const ROOT = resolve(import.meta.dir, "..");
const ZERO = "00".repeat(32);
const COMMANDS = ["getcfilters", "cfilter", "getcfheaders", "cfheaders", "getcfcheckpt", "cfcheckpt"];
type Row = Record<string, unknown>;
type Recipe = Row & { name: string; input: Row };
type WireRecipe = Recipe & { command: string };
export type Corpus = { wire: WireRecipe[]; proof: Recipe[]; handlers: Recipe[]; malformed: Row[] };
export type Fixtures = { wire: string; proof: string };
export type Runner = (command: string, args: string[], root: string, maybeInput?: string) => string;
const digest = (seed: string) => createHash("sha256").update(seed).digest("hex");
const hashes = (n: number) => Array.from({ length: n }, (_, i) => digest(`filter-${i}`));
const wire = (name: string, command: string, input: Row): WireRecipe => ({ name, command, input });

/** Fixed public protocol recipes; no production implementation supplies expectations. */
export function buildCorpus(): Corpus {
  const stop_hash_raw = digest("stop");
  const wireCases: WireRecipe[] = [];
  for (const command of ["getcfilters", "getcfheaders"]) {
    for (const start_height of [0, 1, 999, 1000, 0xffffffff]) {
      wireCases.push(wire(`${command}-${start_height}`, command, { filter_type: 0, start_height, stop_hash_raw }));
    }
    wireCases.push(wire(`${command}-type2`, command, { filter_type: 2, start_height: 0, stop_hash_raw }));
  }
  for (const length of [0, 1, 252, 253, 65535, 65536]) {
    wireCases.push(wire(`cfilter-length-${length}`, "cfilter", {
      filter_type: 0, block_hash_raw: digest("block"), filter_data: "51".repeat(length),
    }));
  }
  for (const count of [0, 1, 252, 253, 2000]) {
    wireCases.push(wire(`cfheaders-count-${count}`, "cfheaders", {
      filter_type: 0, stop_hash_raw, prev_header_raw: digest("predecessor"), hashes_raw: hashes(count),
    }));
  }
  wireCases.push(wire("cfheaders-genesis", "cfheaders", {
    filter_type: 0, stop_hash_raw, prev_header_raw: ZERO, hashes_raw: hashes(1),
  }));
  for (const stop_height of [0, 999, 1000, 1001, 1999, 2000, 3001, 252000, 253000]) {
    wireCases.push(wire(`getcfcheckpt-${stop_height}`, "getcfcheckpt", { filter_type: 0, stop_hash_raw }));
    wireCases.push(wire(`cfcheckpt-${stop_height}`, "cfcheckpt", {
      filter_type: 0, stop_hash_raw, stop_height,
      headers_raw: Array.from({ length: Math.floor(stop_height / 1000) }, (_, i) => digest(`header-${(i + 1) * 1000}`)),
    }));
  }
  const proof: Recipe[] = [];
  const proofRow = (name: string, bits: number, to: bigint, from = 0n, spacing = 600,
    timestamp_age = 0, active = false, scripts_valid = true, best_header = true): Recipe => ({ name,
    input: { bits, to_work: to.toString(), from_work: from.toString(), spacing,
      timestamp_age, active, scripts_valid, best_header } });
  for (const bits of [0, 0x01800001, 0x01010000, 0x1d00ffff, 0x1d80ffff, 0x1c654657, 0x207fffff, 0x2100ffff, 0x23010000]) {
    proof.push(proofRow(`target-${bits.toString(16)}`, bits, 1000000000000n));
  }
  for (const age of [2591999, 2592000, 2592001, -1]) {
    proof.push(proofRow(`timestamp-age-${age}`, 0x207fffff, 2n, 0n, 600, age));
    proof.push(proofRow(`proof-age-${age}`, 0x207fffff, BigInt(Math.abs(age)) * 2n,
      0n, 1, age < 0 ? -1 : 0));
  }
  proof.push(proofRow("negative-equivalent-time", 0x207fffff, 0n, 20n));
  proof.push(proofRow("equal-work", 0x207fffff, 20n, 20n));
  proof.push(proofRow("wrap-256-multiply", 0x207fffff, 1n << 255n, 0n, 2));
  proof.push(proofRow("saturate-positive", 0x207fffff, (1n << 64n) - 2n, 0n, 2));
  proof.push(proofRow("saturate-negative", 0x207fffff, 0n, (1n << 64n) - 2n, 2));
  proof.push(proofRow("int64-exact", 0x207fffff, ((1n << 63n) - 1n) * 2n, 0n, 1));
  proof.push(proofRow("int64-one-over", 0x207fffff, (1n << 63n) * 2n, 0n, 1));
  proof.push(proofRow("active-bypasses-age", 0x207fffff, 1n << 64n, 0n, 600, 9999999, true));
  proof.push(proofRow("stale-not-scripts-valid", 0x207fffff, 2n, 0n, 600, 0, false, false));
  proof.push(proofRow("missing-best-header", 0x207fffff, 2n, 0n, 600, 0, false, true, false));
  const handler = (name: string, patch: Row = {}): Recipe => ({ name, input: {
    family: "getcfilters", filter_type: 0, advertised: true, known_stop: true, allowed_stop: true,
    start_height: 0, stop_height: 0, backend: true, complete: true, ...patch,
  } });
  const handlers = [handler("genesis"), handler("unknown-type-precedes-unknown-stop", { filter_type: 2, known_stop: false }),
    handler("unadvertised", { advertised: false }), handler("unknown-stop", { known_stop: false }),
    handler("disallowed-stop", { allowed_stop: false }), handler("reversed", { start_height: 1 }),
    handler("cfilters-limit", { stop_height: 999 }), handler("cfilters-one-over", { stop_height: 1000 }),
    handler("cfheaders-limit", { family: "getcfheaders", stop_height: 1999 }),
    handler("cfheaders-one-over", { family: "getcfheaders", stop_height: 2000 }),
    handler("all-disabled-index", { backend: false }), handler("missing-record", { complete: false }),
    handler("checkpoints-no-contiguous-cap", { family: "getcfcheckpt", stop_height: 253000 })];
  const malformed = ["getcfilters", "getcfheaders", "getcfcheckpt"].flatMap(command => [
    { name: `${command}-truncated`, command, mutation: "truncate-stop-hash", expected: "parse-failure-no-normal-response" },
    { name: `${command}-trailing`, command, mutation: "append-ff", expected: "prefix-consumed-trailing-ignored" },
  ]);
  return { wire: wireCases, proof, handlers, malformed };
}

function record(value: unknown): Row {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("expected object");
  return value as Row;
}
function keys(row: Row, expected: string[]): void {
  if (Object.keys(row).sort().join() !== [...expected].sort().join()) throw new Error("unexpected schema keys");
}
function integer(value: unknown, max = 0xffffffff, min = 0): void {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < min || value > max) throw new Error("invalid exact integer");
}
function hex(value: unknown, maybeBytes?: number, max = LIMITS.payload): asserts value is string {
  if (typeof value !== "string" || value.length > max * 2 || value.length % 2 || !/^[0-9a-f]*$/.test(value) ||
      (maybeBytes !== undefined && value.length !== maybeBytes * 2)) throw new Error("invalid bounded raw hex");
}
function decimal(value: unknown, max: bigint): void {
  if (typeof value !== "string" || value.length > 78 || !/^(0|[1-9][0-9]*)$/.test(value) || BigInt(value) > max) {
    throw new Error("invalid lossless integer string");
  }
}
function validateWire(row: Row): void {
  keys(row, ["name", "command", "input"]);
  const input = record(row.input);
  integer(input.filter_type, 255);
  const command = row.command;
  if (!COMMANDS.includes(String(command))) throw new Error("unknown wire family");
  const fields = command === "cfilter" ? ["block_hash_raw", "filter_data"] : command === "cfheaders" ?
    ["stop_hash_raw", "prev_header_raw", "hashes_raw"] : command === "cfcheckpt" ?
      ["stop_hash_raw", "stop_height", "headers_raw"] : command === "getcfcheckpt" ?
        ["stop_hash_raw"] : ["start_height", "stop_hash_raw"];
  keys(input, ["filter_type", ...fields]);
  for (const [key, value] of Object.entries(input)) {
    if (key.endsWith("_height")) integer(value);
    else if (key === "filter_data") hex(value, undefined, LIMITS.payload - 42);
    else if (key === "hashes_raw" || key === "headers_raw") {
      if (!Array.isArray(value) || value.length > 2000) throw new Error("vector budget");
      value.forEach(item => hex(item, 32));
    } else if (key.endsWith("_raw")) hex(value, 32);
  }
  if (command === "cfcheckpt" && (input.headers_raw as unknown[]).length !== Math.floor(Number(input.stop_height) / 1000)) {
    throw new Error("positive checkpoint corpus mismatch");
  }
}
function validateProof(row: Row): void {
  keys(row, ["name", "input"]);
  const input = record(row.input);
  keys(input, ["bits", "to_work", "from_work", "spacing", "timestamp_age", "active", "scripts_valid", "best_header"]);
  integer(input.bits);
  integer(input.spacing, 0x7fffffff, 1);
  integer(input.timestamp_age, 0x7fffffff, -0x7fffffff);
  decimal(input.to_work, (1n << 256n) - 1n);
  decimal(input.from_work, (1n << 256n) - 1n);
  for (const key of ["active", "scripts_valid", "best_header"]) if (typeof input[key] !== "boolean") throw new Error("invalid boolean");
}
/** Validate finite recipes before subprocess input allocation. */
export function validateCorpus(corpus: Corpus): void {
  keys(record(corpus), ["wire", "proof", "handlers", "malformed"]);
  const groups = [corpus.wire, corpus.proof, corpus.handlers, corpus.malformed];
  if (groups.some(group => !Array.isArray(group)) || groups.reduce((n, group) => n + group.length, 0) > LIMITS.cases) {
    throw new Error("case budget exceeded");
  }
  const names = new Set();
  for (const group of groups) for (const value of group) {
    const row = record(value);
    if (typeof row.name !== "string" || !/^[a-z0-9-]{1,90}$/.test(row.name) || names.has(row.name)) throw new Error("invalid case identity");
    names.add(row.name);
  }
  corpus.wire.forEach(validateWire);
  corpus.proof.forEach(validateProof);
  // Handler and malformed assertions are a fixed source-audited corpus, never caller policy.
  const fixed = buildCorpus();
  if (JSON.stringify(corpus.handlers) !== JSON.stringify(fixed.handlers) || JSON.stringify(corpus.malformed) !== JSON.stringify(fixed.malformed)) {
    throw new Error("handler corpus mismatch");
  }
}

/** No shell; subprocess time and stdout/stderr allocations have fixed budgets. */
export const runBounded: Runner = (command, args, root, maybeInput) => {
  if (maybeInput !== undefined && Buffer.byteLength(maybeInput) > LIMITS.input) throw new Error("input budget exceeded");
  const child = spawnSync(command, args, { cwd: root, input: maybeInput, encoding: "utf8",
    timeout: LIMITS.timeout, maxBuffer: LIMITS.stdout,
    env: { ...(command === "git" ? foreignGitEnvironment() : process.env), PYTHONDONTWRITEBYTECODE: "1" } });
  if (child.error) throw child.error;
  if (child.status !== 0) throw new Error(`${command} failed (${child.status}): ${child.stderr.slice(0, 2048)}`);
  if (Buffer.byteLength(child.stdout) > LIMITS.stdout) throw new Error("output budget exceeded");
  return child.stdout;
};
function checkedPath(root: string, relative: string): string {
  const candidate = resolve(root, relative);
  const parent = realpathSync(dirname(candidate));
  if (!parent.startsWith(realpathSync(root) + "/") || (existsSync(candidate) && lstatSync(candidate).isSymbolicLink())) {
    throw new Error("path escapes repository or is a symlink");
  }
  return candidate;
}
function validatePayload(payload: Buffer, row: Row): void {
  const input = record(row.input);
  let cursor = 0;
  const take = (length: number): Buffer => {
    if (cursor + length > payload.length) throw new Error("short oracle payload");
    const bytes = payload.subarray(cursor, cursor + length);
    cursor += length;
    return bytes;
  };
  const raw = (value: unknown) => {
    if (take(32).toString("hex") !== value) throw new Error("raw digest contract mismatch");
  };
  const count = (): number => {
    const marker = take(1)[0];
    if (marker < 253) return marker;
    const length = marker === 253 ? 2 : marker === 254 ? 4 : 8;
    const bytes = take(length);
    const value = length === 8 ? Number(bytes.readBigUInt64LE()) : bytes.readUIntLE(0, length);
    if (!Number.isSafeInteger(value) || value > LIMITS.payload || value < (marker === 253 ? 253 : marker === 254 ? 65536 : 4294967296)) {
      throw new Error("noncanonical or oversized CompactSize");
    }
    return value;
  };
  if (take(1)[0] !== input.filter_type) throw new Error("filter type contract mismatch");
  if (row.command === "getcfilters" || row.command === "getcfheaders") {
    if (take(4).readUInt32LE() !== input.start_height) throw new Error("start height contract mismatch");
    raw(input.stop_hash_raw);
  } else if (row.command === "cfilter") {
    raw(input.block_hash_raw);
    if (take(count()).toString("hex") !== input.filter_data) throw new Error("filter bytes contract mismatch");
  } else {
    raw(input.stop_hash_raw);
    if (row.command === "cfheaders") raw(input.prev_header_raw);
    if (row.command === "cfheaders" || row.command === "cfcheckpt") {
      const entries = input[row.command === "cfheaders" ? "hashes_raw" : "headers_raw"] as string[];
      if (count() !== entries.length) throw new Error("vector count contract mismatch");
      entries.forEach(raw);
    }
  }
  if (cursor !== payload.length) throw new Error("unexpected oracle payload suffix");
}
function validateFrame(row: Row): void {
  const expected = record(row.expected);
  keys(expected, ["payload_hex", "frame_hex"]);
  hex(expected.payload_hex);
  hex(expected.frame_hex, undefined, LIMITS.payload + 24);
  const frame = Buffer.from(expected.frame_hex, "hex");
  const payload = Buffer.from(expected.payload_hex, "hex");
  validatePayload(payload, row);
  const checksum = createHash("sha256").update(createHash("sha256").update(payload).digest()).digest().subarray(0, 4);
  const command = Buffer.alloc(12);
  command.write(String(row.command), "ascii");
  if (frame.length !== payload.length + 24 || frame.subarray(0, 4).toString("hex") !== "f9beb4d9" ||
      !frame.subarray(4, 16).equals(command) || frame.readUInt32LE(16) !== payload.length ||
      !frame.subarray(20, 24).equals(checksum) || !frame.subarray(24).equals(payload)) throw new Error("wire frame contract mismatch");
}
function validateProofExpected(value: unknown): void {
  const expected = record(value);
  keys(expected, ["target_hex", "proof", "equivalent_time", "sign", "saturated", "allowed"]);
  hex(expected.target_hex, 32);
  decimal(expected.proof, (1n << 256n) - 1n);
  if (expected.equivalent_time !== null) {
    const time = expected.equivalent_time;
    if (typeof time !== "string" || time.length > 20 || !/^(0|-?[1-9][0-9]*)$/.test(time) ||
        BigInt(time) > (1n << 63n) - 1n || BigInt(time) < -((1n << 63n) - 1n)) throw new Error("invalid signed exact time");
  } else if (expected.proof !== "0") throw new Error("missing equivalent time for valid target");
  if (![1, -1].includes(expected.sign as number) || typeof expected.saturated !== "boolean" || typeof expected.allowed !== "boolean") {
    throw new Error("invalid proof outcome");
  }
}
/** Parse helper output only after byte bounds; reject unexpected fields and input identities. */
export function validateResult(output: string, corpus: Corpus): Fixtures {
  if (Buffer.byteLength(output) > LIMITS.stdout) throw new Error("output budget exceeded");
  const result = record(JSON.parse(output));
  keys(result, ["wire", "proof", "handlers", "malformed"]);
  for (const key of ["wire", "proof", "handlers", "malformed"] as const) {
    const group = result[key];
    if (!Array.isArray(group) || group.length !== corpus[key].length) throw new Error("oracle case count mismatch");
    group.forEach((value: unknown, i: number) => {
      const row = record(value);
      if (key === "malformed") {
        keys(row, [...Object.keys(corpus[key][i]), "payload_hex"]);
        const { payload_hex, ...identity } = row;
        if (JSON.stringify(identity) !== JSON.stringify(corpus[key][i])) throw new Error("malformed contract mismatch");
        const baseLength = row.command === "getcfcheckpt" ? 33 : 37;
        hex(payload_hex, baseLength + (row.mutation === "append-ff" ? 1 : -1));
        return;
      }
      keys(row, [...Object.keys(corpus[key][i]), "expected"]);
      const { expected, ...identity } = row;
      if (JSON.stringify(identity) !== JSON.stringify(corpus[key][i])) throw new Error("oracle input identity mismatch");
      if (key === "wire") validateFrame(row);
      if (key === "proof") validateProofExpected(expected);
      if (key === "handlers") {
        const assertion = record(expected);
        keys(assertion, ["outcome", "reason"]);
        if (!["respond", "disconnect", "no-normal-response"].includes(String(assertion.outcome)) ||
            !["complete", "type-or-service", "stop", "reversed", "range", "index", "missing"].includes(String(assertion.reason))) {
          throw new Error("invalid handler assertion");
        }
      }
    });
  }
  const authority = { version: "29.3.knots20260210", pin: PIN,
    wire: "test/functional/test_framework/messages.py:msg_getcfilters..msg_cfcheckpt",
    frame: "test/functional/test_framework/p2p.py:P2PConnection.build_message (v1/mainnet)",
    proof: "src/chain.cpp:GetBlockProof/GetBlockProofEquivalentTime; src/arith_uint256.cpp:SetCompact",
    handlers: "src/net_processing.cpp:PrepareBlockFilterRequest/ProcessGetCF*/BlockRequestAllowed; src/streams.h:DataStream::read",
    evidence: "independent helper/source expectations; no achieved Rust or socket-write claim" };
  const render = (value: unknown) => `${JSON.stringify(value, null, 2)}\n`;
  const fixtures = { wire: render({ schema_version: 1, authority, raw_hash_order: "protocol bytes; reverse for display",
    wire_envelope: { bytes: LIMITS.payload, source: "src/net.h:MAX_PROTOCOL_MESSAGE_LENGTH", meaning: "wire singleton envelope, independent of local work/queue policy" },
    cases: result.wire, handler_cases: result.handlers, malformed_cases: result.malformed }),
  proof: render({ schema_version: 1, authority, invalid_target_contract: "proof zero; equivalent time undefined (no division by zero)",
    stale_age_limit_seconds: 2592000, cases: result.proof }) };
  if (Buffer.byteLength(fixtures.wire) + Buffer.byteLength(fixtures.proof) > LIMITS.golden) throw new Error("golden budget exceeded");
  return fixtures;
}
export function generate(root = ROOT, runner: Runner = runBounded): Fixtures {
  const pin = runner("git", ["-C", "packages/bitcoin-knots", "rev-parse", "HEAD"], root).trim();
  if (pin !== PIN) throw new Error(`Knots pin mismatch: expected ${PIN}, found ${pin}`);
  if (runner("git", ["-C", "packages/bitcoin-knots", "status", "--porcelain", "--untracked-files=no"], root).trim()) {
    throw new Error("pinned Knots tracked inputs are modified");
  }
  for (const relative of ["scripts/basic-filter-oracle.py", "packages/bitcoin-knots/test/functional/test_framework/messages.py",
    "packages/bitcoin-knots/src/chain.cpp", "packages/bitcoin-knots/src/arith_uint256.cpp",
    "packages/bitcoin-knots/src/streams.h", "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/src/net.h", "packages/bitcoin-knots/test/functional/test_framework/p2p.py"]) checkedPath(root, relative);
  const corpus = buildCorpus();
  validateCorpus(corpus);
  const input = JSON.stringify(corpus);
  if (Buffer.byteLength(input) > LIMITS.input) throw new Error("input budget exceeded");
  return validateResult(runner("python3", ["scripts/basic-filter-oracle.py", "--peer-wire-proof"], root, input), corpus);
}
/** Validate both paths and stage both complete documents before atomic per-file rename. */
export function persistFixtures(root: string, fixtures: Fixtures, mode: "--write" | "--check"): void {
  const paths = [checkedPath(root, WIRE_PATH), checkedPath(root, PROOF_PATH)];
  const content = [fixtures.wire, fixtures.proof];
  if (content.reduce((n, item) => n + Buffer.byteLength(item), 0) > LIMITS.golden) throw new Error("golden budget exceeded");
  if (mode === "--check") {
    paths.forEach((file, i) => {
      if (statSync(file).size > LIMITS.golden || readFileSync(file, "utf8") !== content[i]) throw new Error(`fixture drift: ${file}`);
    });
    return;
  }
  const temporary = paths.map(file => `${file}.${process.pid}.${crypto.randomUUID()}.tmp`);
  try {
    temporary.forEach((file, i) => writeFileSync(file, content[i], { flag: "wx" }));
    temporary.forEach((file, i) => renameSync(file, paths[i]));
  } finally {
    temporary.forEach(file => rmSync(file, { force: true }));
  }
}
function main(): void {
  const mode = process.argv[2];
  if (process.argv.length !== 3 || (mode !== "--write" && mode !== "--check")) throw new Error("usage: bun scripts/generate-peer-filter-fixtures.ts --write|--check");
  const logDirectory = resolve(ROOT, ".local/open-bitcoin-dev/peer-filter-fixtures");
  mkdirSync(logDirectory, { recursive: true });
  const logPath = checkedPath(ROOT, ".local/open-bitcoin-dev/peer-filter-fixtures/last-run.json");
  const messages = [`validating pinned Knots ${PIN} and bounded oracle contracts`];
  console.log(`peer-filter fixtures: ${messages[0]}`);
  try {
    const fixtures = generate();
    persistFixtures(ROOT, fixtures, mode);
    const bytes = Buffer.byteLength(fixtures.wire) + Buffer.byteLength(fixtures.proof);
    messages.push(`${mode} reproduced six families and exact source arithmetic (${bytes} bytes)`);
    writeFileSync(logPath, `${JSON.stringify({ pin: PIN, mode, outcome: "pass", bytes, messages }, null, 2)}\n`);
    console.log(`peer-filter fixtures: ${messages[1]}; log ${logPath}`);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    writeFileSync(logPath, `${JSON.stringify({ pin: PIN, mode, outcome: "failed", messages, error: message }, null, 2)}\n`);
    throw error;
  }
}
if (import.meta.main) main();

#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { resolve } from "node:path";
import { PIN, LIMITS, runBounded, validateResult, type Runner } from "./generate-peer-filter-fixtures";

export const RECIPE_ID = "phase162-normal-maturity-1";
const ROOT = resolve(import.meta.dir, "..");
const ZERO = "00".repeat(32);
type Row = Record<string, unknown>;
export type RawBlock = { name: string; previous_name: string; raw_block: string };
export type RawRecipe = { recipe_id: string; network_magic: string; blocks: RawBlock[] };
export type OracleCase = RawBlock & {
  outputs: string[]; spent: string[]; block_hash_raw: string; block_hash_display: string;
  predecessor_raw: string; encoded: string; hash_raw: string; hash_display: string;
  header_raw: string; header_display: string; count: number; k0: string; k1: string; mapped: number[];
};
type OracleNode = { parent: string; height: number; block_hash_raw: string; header_raw: string;
  outputs: Record<string, string>; spent: string[] };
type Wire = { name: string; command: string; input: Row; expected?: { payload_hex: string; frame_hex: string } };
export type RecipeResult = { recipe_id: string; ordered_input_digest: string; pin: string;
  cases: OracleCase[]; wire: Wire[] };

function record(value: unknown): Row {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("expected object schema");
  return value as Row;
}
function keys(value: Row, expected: string[]): void {
  if (Object.keys(value).sort().join() !== expected.sort().join()) throw new Error("unexpected schema keys");
}
function hex(value: unknown, maybeBytes?: number): asserts value is string {
  if (typeof value !== "string" || !/^[0-9a-f]*$/.test(value) || value.length % 2 ||
      value.length > LIMITS.input * 2 || (maybeBytes !== undefined && value.length !== maybeBytes * 2)) {
    throw new Error("invalid bounded raw hex");
  }
}
function identifier(value: unknown, empty = false): asserts value is string {
  if (typeof value !== "string" || !(empty && value === "") && !/^[a-z][a-z0-9-]{0,63}$/.test(value)) {
    throw new Error("invalid bounded identity");
  }
}
function integer(value: unknown, max: number): asserts value is number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value > max) {
    throw new Error("invalid bounded integer");
  }
}
function parsed(output: string): Row {
  if (Buffer.byteLength(output) > LIMITS.stdout) throw new Error("output budget exceeded");
  return record(JSON.parse(output));
}
const hash256 = (bytes: Buffer) => createHash("sha256").update(createHash("sha256").update(bytes).digest()).digest();

/** Caller schema contains raw blocks and ancestry only; expectations are forbidden. */
export function validateRecipe(value: unknown): asserts value is RawRecipe {
  if (Buffer.byteLength(JSON.stringify(value)) > LIMITS.input) throw new Error("input budget exceeded");
  const recipe = record(value);
  keys(recipe, ["recipe_id", "network_magic", "blocks"]);
  if (recipe.recipe_id !== RECIPE_ID) throw new Error("unknown recipe ID");
  hex(recipe.network_magic, 4);
  if (!Array.isArray(recipe.blocks) || !recipe.blocks.length || recipe.blocks.length > 1108) {
    throw new Error("recipe block budget exceeded");
  }
  const heights = new Map<string, number>();
  let active = 0;
  let branches = 0;
  let previous = "";
  for (const value of recipe.blocks) {
    const row = record(value);
    keys(row, ["name", "previous_name", "raw_block"]);
    identifier(row.name);
    identifier(row.previous_name, true);
    hex(row.raw_block);
    if (row.raw_block.length < 162) throw new Error("short raw block");
    if (heights.has(row.name)) throw new Error("duplicate identity");
    if (heights.size && !heights.has(row.previous_name) || !heights.size && row.previous_name !== "") {
      throw new Error("unresolved or future ancestry");
    }
    const height = row.previous_name ? heights.get(row.previous_name)! + 1 : 0;
    if (row.previous_name !== previous || branches > 0) {
      branches++;
    } else {
      active++;
      if (active > 1100) throw new Error("active block budget exceeded");
    }
    if (branches > 8) throw new Error("branch block budget exceeded");
    heights.set(row.name, height);
    previous = row.name;
  }
}

function validateNode(value: unknown): asserts value is OracleNode {
  const node = record(value);
  keys(node, ["parent", "height", "block_hash_raw", "header_raw", "outputs", "spent"]);
  identifier(node.parent, true);
  integer(node.height, 1107);
  hex(node.block_hash_raw, 32);
  hex(node.header_raw, 32);
  const outpoint = (value: unknown) => {
    if (typeof value !== "string" || !/^[0-9a-f]{64}:(0|[1-9][0-9]{0,9})$/.test(value)) throw new Error("invalid raw outpoint");
  };
  for (const [key, value] of Object.entries(record(node.outputs))) { outpoint(key); hex(value); }
  if (!Array.isArray(node.spent)) throw new Error("invalid spent facts");
  node.spent.forEach(outpoint);
}

function validateCase(value: unknown, raw: RawBlock, node: OracleNode, nodes: Record<string, OracleNode>): OracleCase {
  const row = record(value);
  keys(row, ["name", "previous_name", "raw_block", "outputs", "spent", "block_hash_raw", "block_hash_display",
    "predecessor_raw", "encoded", "hash_raw", "hash_display", "header_raw", "header_display", "count", "k0", "k1", "mapped"]);
  for (const key of ["name", "previous_name", "raw_block"] as const) {
    if (row[key] !== raw[key]) throw new Error("oracle raw identity mismatch");
  }
  for (const key of ["block_hash_raw", "block_hash_display", "predecessor_raw", "hash_raw", "hash_display", "header_raw", "header_display"]) hex(row[key], 32);
  hex(row.encoded);
  for (const key of ["outputs", "spent"]) {
    if (!Array.isArray(row[key])) throw new Error("invalid script facts");
    row[key].forEach((script: unknown) => hex(script));
  }
  integer(row.count, LIMITS.input);
  if (!Array.isArray(row.mapped) || row.mapped.length !== row.count) throw new Error("mapped count mismatch");
  row.mapped.forEach((n: unknown) => integer(n, Number(row.count) * 784931));
  for (const key of ["k0", "k1"]) {
    if (typeof row[key] !== "string" || !/^(0|[1-9][0-9]{0,19})$/.test(row[key]) || BigInt(row[key]) > (1n << 64n) - 1n) {
      throw new Error("invalid SipHash key");
    }
  }
  if (row.block_hash_raw !== node.block_hash_raw || row.header_raw !== node.header_raw ||
      row.predecessor_raw !== (raw.previous_name ? nodes[raw.previous_name].header_raw : ZERO)) {
    throw new Error("oracle predecessor mismatch");
  }
  const digest = hash256(Buffer.from(row.encoded as string, "hex"));
  const header = hash256(Buffer.concat([digest, Buffer.from(row.predecessor_raw as string, "hex")]));
  if (row.block_hash_raw !== hash256(Buffer.from(raw.raw_block.slice(0, 160), "hex")).toString("hex") ||
      row.hash_raw !== digest.toString("hex") || row.header_raw !== header.toString("hex") ||
      row.block_hash_display !== Buffer.from(row.block_hash_raw as string, "hex").reverse().toString("hex") ||
      row.hash_display !== Buffer.from(digest).reverse().toString("hex") ||
      row.header_display !== Buffer.from(header).reverse().toString("hex")) throw new Error("oracle commitment mismatch");
  return row as OracleCase;
}

function rawExpectations(recipe: RawRecipe, root: string, runner: Runner): OracleCase[] {
  let nodes: Record<string, OracleNode> = {};
  const cases: OracleCase[] = [];
  for (let start = 0; start < recipe.blocks.length; start += LIMITS.cases) {
    const batch = recipe.blocks.slice(start, start + LIMITS.cases);
    const input = JSON.stringify({ cases: batch, nodes });
    if (Buffer.byteLength(input) > LIMITS.input) throw new Error("batch input budget exceeded");
    const result = parsed(runner("python3", ["scripts/basic-filter-oracle.py", "--phase162-recipe"], root, input));
    keys(result, ["cases", "nodes"]);
    if (!Array.isArray(result.cases) || result.cases.length !== batch.length) throw new Error("oracle case count mismatch");
    const returned = record(result.nodes);
    const names = [...Object.keys(nodes), ...batch.map(row => row.name)];
    keys(returned, names);
    for (const [name, node] of Object.entries(returned)) {
      validateNode(node);
      if (nodes[name] && JSON.stringify(nodes[name]) !== JSON.stringify(node)) throw new Error("oracle changed prior ancestry");
    }
    nodes = returned as Record<string, OracleNode>;
    result.cases.forEach((row, i) => {
      const raw = batch[i];
      const node = nodes[raw.name];
      if (node.parent !== raw.previous_name || node.height !== (raw.previous_name ? nodes[raw.previous_name].height + 1 : 0)) {
        throw new Error("oracle ancestry mismatch");
      }
      cases.push(validateCase(row, raw, node, nodes));
    });
  }
  return cases;
}

function wireExpectations(recipe: RawRecipe, cases: OracleCase[], root: string, runner: Runner): Wire[] {
  const rows: Wire[] = cases.map(row => ({ name: `${row.name}-cfilter`, command: "cfilter",
    input: { filter_type: 0, block_hash_raw: row.block_hash_raw, filter_data: row.encoded } }));
  const byName = new Map(cases.map(row => [row.name, row]));
  const parents = new Set(cases.map(row => row.previous_name));
  for (const leaf of cases.filter(row => !parents.has(row.name))) {
    const ancestry: OracleCase[] = [];
    let maybeCursor: OracleCase | undefined = leaf;
    while (maybeCursor) { ancestry.push(maybeCursor); maybeCursor = byName.get(maybeCursor.previous_name); }
    ancestry.reverse();
    const range = ancestry.slice(-4);
    rows.push({ name: `${leaf.name}-cfheaders`, command: "cfheaders", input: {
      filter_type: 0, stop_hash_raw: leaf.block_hash_raw, prev_header_raw: range[0].predecessor_raw,
      hashes_raw: range.map(row => row.hash_raw),
    } }, { name: `${leaf.name}-cfcheckpt`, command: "cfcheckpt", input: {
      filter_type: 0, stop_hash_raw: leaf.block_hash_raw,
      headers_raw: ancestry.filter((_, height) => height > 0 && height % 1000 === 0).map(row => row.header_raw),
    } });
  }
  const wire: Wire[] = [];
  for (let start = 0; start < rows.length; start += LIMITS.cases) {
    const corpus = { wire: rows.slice(start, start + LIMITS.cases), proof: [], handlers: [], malformed: [] };
    const output = runner("python3", ["scripts/basic-filter-oracle.py", "--peer-wire-proof"], root, JSON.stringify(corpus));
    // Existing strict validator checks exact input identity, payload, checksum and v1 framing.
    const validated = JSON.parse(validateResult(output, corpus).wire).cases as Wire[];
    wire.push(...validated.map(row => {
      if (!row.expected) throw new Error("missing pinned serializer expectation");
      return { ...row, expected: { ...row.expected, frame_hex: recipe.network_magic + row.expected.frame_hex.slice(8) } };
    }));
  }
  return wire;
}

/** Independently derive BASIC and wire commitments; no Rust query is an input. */
export function deriveRecipe(value: unknown, root = ROOT, runner: Runner = runBounded): RecipeResult {
  validateRecipe(value);
  const pin = runner("git", ["-C", "packages/bitcoin-knots", "rev-parse", "HEAD"], root).trim();
  if (pin !== PIN) throw new Error(`Knots pin mismatch: expected ${PIN}, found ${pin}`);
  if (runner("git", ["-C", "packages/bitcoin-knots", "status", "--porcelain", "--untracked-files=no"], root).trim()) {
    throw new Error("pinned Knots tracked inputs are modified");
  }
  const cases = rawExpectations(value, root, runner);
  const result = { recipe_id: value.recipe_id,
    ordered_input_digest: createHash("sha256").update(JSON.stringify(value.blocks.map(row => ({
      name: row.name, previous_name: row.previous_name, raw_block: row.raw_block,
    })))).digest("hex"), pin,
    cases, wire: wireExpectations(value, cases, root, runner) };
  if (Buffer.byteLength(JSON.stringify(result)) > LIMITS.stdout) throw new Error("output budget exceeded");
  return result;
}

async function main(): Promise<void> {
  if (process.argv.length !== 2) throw new Error("usage: bun scripts/phase162-filter-oracle.ts < raw-recipe.json");
  const chunks: Uint8Array[] = [];
  let bytes = 0;
  for await (const chunk of Bun.stdin.stream()) {
    bytes += chunk.length;
    if (bytes > LIMITS.input) throw new Error("stdin input budget exceeded");
    chunks.push(chunk);
  }
  process.stdout.write(`${JSON.stringify(deriveRecipe(JSON.parse(Buffer.concat(chunks).toString("utf8"))))}\n`);
}
if (import.meta.main) await main();

#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";

export const PIN = "a9aee730466ac67d35a3c03ee24676be5e045878";
export const FIXTURE = "packages/open-bitcoin-consensus/testdata/basic_filter_vectors.rs";
const ZERO = "00".repeat(32);
const ROOT = resolve(import.meta.dir, "..");
const MAX_BYTES = 2_000_000;
export type EdgeInput = Readonly<{
  name: string;
  raw_block: string;
  block_hash_raw: string;
  outputs: readonly string[];
  spent: readonly string[];
  predecessor_raw: string;
  previous_name: string;
}>;
type OracleCase = EdgeInput & {
  block_hash_display: string; encoded: string; hash_raw: string; hash_display: string;
  header_raw: string; header_display: string; count: number; k0: string; k1: string;
  mapped: number[];
};
type OracleResult = { cases: OracleCase[]; siphash: { length: number; expected: string }[] };

function edge(name: string, outputs: readonly string[], spent: readonly string[] = []): EdgeInput {
  return { name, outputs, spent, raw_block: "", block_hash_raw: ZERO,
    predecessor_raw: ZERO, previous_name: "" };
}
function actual(name: string, raw_block: string, spent: readonly string[], previous_name: string): EdgeInput {
  return { ...edge(name, [], spent), raw_block, previous_name };
}
// Public transaction inputs independently serialized using pinned Knots messages.py.
// Corrected height scripts are 0051 / 010151 / 010251; first passing nonces are zero.
const GENESIS = "010000000000000000000000000000000000000000000000000000000000000000000000ed175e79811ecdd682646f324b9a1196d066401bdf258fd7f76ccb88a0909574e8030000ffff7f20000000000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff020051ffffffff0100f2052a01000000015500000000";
const FUNDING = "010000009e6908bf7cad58f0649b7551ce9248831e5103c8db97529c5ace56ddcf43ff72e0b2d11ebadcf85195908dd88a99ed125d05689be1d083f8d7f54cf26fd05bf34c040000ffff7f20000000000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff03010151ffffffff0100f2052a01000000015100000000";
const SPENDING = "01000000e28abf3a07923bc617b0df7e7fb4c0841d7c9a9a7378febd480c302be35ee63e408f60efe812089204a4f691d0ab12a71bcc38cabc3c2fb362619af8d1b83374b0040000ffff7f20000000000301000000010000000000000000000000000000000000000000000000000000000000000000ffffffff03010251ffffffff0100f2052a010000000154000000000200000001e0b2d11ebadcf85195908dd88a99ed125d05689be1d083f8d7f54cf26fd05bf30000000000ffffffff0118ee052a0100000001520000000002000000015e12cf1e2d96d52b13e2e15a5317e82422ecd4be7a9cfc851f7109db950f831a0000000000ffffffff0130ea052a01000000015300000000";
const REPLACEMENT = "01000000e28abf3a07923bc617b0df7e7fb4c0841d7c9a9a7378febd480c302be35ee63ecbc7d89b795f75cdc8ddf3252a718117317820c956ba8bd2a28b5a28ed44342314050000ffff7f20000000000301000000010000000000000000000000000000000000000000000000000000000000000000ffffffff03010251ffffffff0100f2052a010000000154000000000200000001e0b2d11ebadcf85195908dd88a99ed125d05689be1d083f8d7f54cf26fd05bf30000000000ffffffff0118ee052a0100000001520000000002000000015e12cf1e2d96d52b13e2e15a5317e82422ecd4be7a9cfc851f7109db950f831a0000000000ffffffff0130ea052a01000000015600000000";
const SUCCESSOR = "010000004f281baf8875d9c7ab412e10e71926df2fad3688fd04404034c1869a9498ed15221f1250a71b5d0c076a728759d16d8f1713823bdc42c0987f2b0c0faff6359114050000ffff7f20000000000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff03010351ffffffff0100f2052a01000000015700000000";
const REPLACEMENT_SUCCESSOR = "010000003dc149aca71b2fb6bc3f7a32ac4402965fa63bf5e9e63a1bf3cff68f9c57a13a221f1250a71b5d0c076a728759d16d8f1713823bdc42c0987f2b0c0faff6359178050000ffff7f20010000000101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff03010351ffffffff0100f2052a01000000015700000000";
// Frozen bounded discovery: LE32 candidates 0..9999, pinned siphash(0,0),
// N=2, F=1_569_862; first duplicate mapped value at candidate 1666.
export const COLLISION = { k0: "0", k1: "0", count: 2, range: 1_569_862,
  scripts: ["dd040000", "82060000"], mapped: [1_560_268, 1_560_268] } as const;
const countScripts = (count: number) => Array.from({ length: count }, (_, i) =>
  `51${(i & 255).toString(16).padStart(2, "0")}${(i >> 8).toString(16).padStart(2, "0")}`);
export const EDGE_INPUTS: readonly EdgeInput[] = [
  edge("empty", [], [""]), edge("single", ["51"]), edge("duplicate", ["51", "51"], ["51"]),
  edge("collision", COLLISION.scripts), edge("output-op-return", ["6a51"]),
  edge("spent-op-return", [], ["6a51"]), edge("noninitial-op-return", ["516a"]),
  edge("empty-scripts", [""], [""]), edge("malformed-output", ["4c"]),
  edge("count-252", countScripts(252)), edge("count-253", countScripts(253)),
  actual("actual-genesis", GENESIS, [], ""),
  actual("actual-funding", FUNDING, [], "actual-genesis"),
  actual("actual-spending", SPENDING, ["51", "52"], "actual-funding"),
  actual("actual-replacement", REPLACEMENT, ["51", "52"], "actual-funding"),
  actual("actual-successor", SUCCESSOR, [], "actual-spending"),
  actual("actual-replacement-successor", REPLACEMENT_SUCCESSOR, [], "actual-replacement"),
];

function captureChild(
  command: string, args: string[], root: string, env: NodeJS.ProcessEnv, maybeInput?: string,
): string {
  const child = spawnSync(command, args, { cwd: root, input: maybeInput, encoding: "utf8",
    timeout: 10_000, maxBuffer: MAX_BYTES, env });
  if (child.error) throw child.error;
  if (child.status !== 0) throw new Error(`${command} failed (${child.status}): ${child.stderr.trim()}`);
  return child.stdout;
}
/** Preserve global/auth settings while clearing Git's canonical repository-local environment. */
export function foreignGitEnvironment(): NodeJS.ProcessEnv {
  // The name-only probe must not inherit selectors that can prevent Git from starting.
  const probeEnv = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("GIT_")));
  const localNames = captureChild("git", ["rev-parse", "--local-env-vars"], ROOT, probeEnv).trim().split(/\r?\n/);
  const env = { ...process.env };
  // https://git-scm.com/docs/githooks: foreign repository Git must clear these names.
  for (const name of localNames) delete env[name];
  return env;
}
export function runChild(command: string, args: string[], root: string, maybeInput?: string): string {
  if (maybeInput && Buffer.byteLength(maybeInput) > MAX_BYTES) throw new Error("oracle input exceeds bound");
  const env = command === "git" ? foreignGitEnvironment() : { ...process.env };
  return captureChild(command, args, root, { ...env, PYTHONDONTWRITEBYTECODE: "1" }, maybeInput);
}
export function readCorpus(root: string): unknown[][] {
  const text = readFileSync(resolve(root, "packages/bitcoin-knots/src/test/data/blockfilters.json"), "utf8");
  if (Buffer.byteLength(text) > MAX_BYTES) throw new Error("corpus exceeds bound");
  const parsed: unknown = JSON.parse(text);
  if (!Array.isArray(parsed) || parsed.length !== 11 || !Array.isArray(parsed[0]) || parsed[0].length !== 1) {
    throw new Error("expected corpus heading and exactly ten corpus rows");
  }
  const rows = parsed.slice(1);
  for (const row of rows) {
    if (!Array.isArray(row) || row.length !== 8 || typeof row[0] !== "number" ||
      ![1, 2, 4, 5, 6, 7].every(i => typeof row[i] === "string") ||
      !Array.isArray(row[3]) || !row[3].every((item: unknown) => typeof item === "string")) {
      throw new Error("invalid pinned corpus row");
    }
  }
  return rows;
}
const reverseHex = (value: string) => value.match(/../g)?.reverse().join("") ?? "";
const quote = (value: string) => JSON.stringify(value);
function render(result: OracleResult): string {
  const fields = ["name", "raw_block", "block_hash_raw", "block_hash_display", "predecessor_raw",
    "encoded", "hash_raw", "hash_display", "header_raw", "header_display"] as const;
  const structs = result.cases.map(row => {
    const strings = fields.map(key => `        ${key}: ${quote(row[key])},`).join("\n");
    const scripts = ["outputs", "spent"].map(key =>
      `        ${key}: &[${row[key as "outputs" | "spent"].map(quote).join(", ")}],`).join("\n");
    return `    BasicVector {\n${strings}\n${scripts}\n        count: ${row.count},\n        k0: ${row.k0},\n        k1: ${row.k1},\n        mapped: &[${row.mapped.join(", ")}],\n    },`;
  }).join("\n");
  return `// Generated by scripts/generate-basic-filter-vectors.ts --write; do not hand-edit.
// Baseline 29.3.knots20260210 at ${PIN}; all ten corpus rows verified first.
// Source-audited independent Python oracle; pinned test_framework.crypto.siphash.
// Raw hash/header bytes use protocol order; *_display strings reverse them.
// Inputs from src/test/data/blockfilters.json and frozen independent edge recipes.
#[allow(dead_code)]
pub struct BasicVector {
${fields.map(key => `    pub ${key}: &'static str,`).join("\n")}
    pub outputs: &'static [&'static str],
    pub spent: &'static [&'static str],
    pub count: u64,
    pub k0: u64,
    pub k1: u64,
    pub mapped: &'static [u64],
}
pub const BASIC_VECTORS: &[BasicVector] = &[
${structs}
];
#[allow(dead_code)]
pub const SIPHASH_LENGTH_VECTORS: &[(usize, u64)] = &[
${result.siphash.map(row => `    (${row.length}, ${row.expected}),`).join("\n")}
];
`;
}
export function generate(root = ROOT): string {
  const pin = runChild("git", ["-C", "packages/bitcoin-knots", "rev-parse", "HEAD"], root).trim();
  if (pin !== PIN) throw new Error(`Knots pin mismatch: expected ${PIN}, found ${pin}`);
  // Refuse dirty upstream helper/corpus/source inputs as well as a different commit.
  const dirty = runChild("git", ["-C", "packages/bitcoin-knots", "status", "--porcelain", "--untracked-files=no"], root);
  if (dirty.trim()) throw new Error("pinned Knots worktree has modified tracked inputs");
  const rows = readCorpus(root);
  const corpus: EdgeInput[] = rows.map(row => ({ ...edge(`corpus-${row[0]}`, [], row[3] as string[]),
    raw_block: row[2] as string, predecessor_raw: reverseHex(row[4] as string) }));
  const input = JSON.stringify({ cases: [...corpus, ...EDGE_INPUTS] });
  const result: OracleResult = JSON.parse(runChild("python3", ["scripts/basic-filter-oracle.py"], root, input));
  if (result.cases.length !== 10 + EDGE_INPUTS.length) throw new Error("oracle case count mismatch");
  // No edge expectations are accepted until all ten pinned block/byte/header agreements pass.
  rows.forEach((row, index) => {
    const actual = result.cases[index];
    if (actual.block_hash_display !== row[1] || actual.encoded !== row[5] || actual.header_display !== row[6]) {
      throw new Error(`independent oracle disagrees with pinned corpus height ${row[0]}`);
    }
  });
  const maybeCollision = result.cases.find(row => row.name === "collision");
  if (!maybeCollision || maybeCollision.k0 !== COLLISION.k0 || maybeCollision.k1 !== COLLISION.k1 ||
    maybeCollision.count !== COLLISION.count || JSON.stringify(maybeCollision.mapped) !== JSON.stringify(COLLISION.mapped)) {
    throw new Error("frozen collision provenance mismatch");
  }
  return render(result);
}
export function checkVectors(root = ROOT): void {
  const generated = generate(root);
  if (readFileSync(resolve(root, FIXTURE), "utf8") !== generated) throw new Error("BASIC fixture drift; regenerate with --write");
}
if (import.meta.main) {
  const mode = process.argv[2];
  if (!["--write", "--check"].includes(mode) || process.argv.length !== 3) {
    throw new Error("usage: bun run scripts/generate-basic-filter-vectors.ts --write|--check");
  }
  if (mode === "--write") writeFileSync(resolve(ROOT, FIXTURE), generate());
  else checkVectors();
  console.log("BASIC vectors: ten pinned corpus agreements and frozen edge commitments reproduced.");
}

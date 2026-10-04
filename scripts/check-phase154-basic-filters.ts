#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { FIXTURE, PIN } from "./generate-basic-filter-vectors.ts";

const DOC = "docs/parity/catalog/basic-compact-filters.md";
const TEST = "packages/open-bitcoin-consensus/tests/basic_filter.rs";
const HISTORICAL = "packages/open-bitcoin-chainstate/src/block_filter/tests/validated.rs";
const GENERATOR = "scripts/generate-basic-filter-vectors.ts";
const ORACLE = "scripts/basic-filter-oracle.py";
const SURFACE = "v2-5-basic-filter-generation-and-commitments";
export const CHECK_FILES = [FIXTURE, DOC, TEST, HISTORICAL, GENERATOR, ORACLE,
  "AGENTS.md", "README.md", "docs/parity/catalog/README.md", "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json", "scripts/verify.sh"] as const;
const ROOT = resolve(import.meta.dir, "..");
const DENIED = /\b(?:filter index(?: activation| storage)?|filter prune retention|compact[- ]filter (?:(?:rpc|peer) )?serving|compact[- ]filter catch-up|V0|BIP37|GUI|production readiness|production[- ]funds (?:use|wallet use))\s+(?:is|are)\s+(?:enabled|supported|available|active|ready|shipped|safe)\b/i;
const EDGE_NAMES = ["empty", "duplicate", "collision", "output-op-return", "spent-op-return",
  "empty-scripts", "malformed-output", "count-252", "count-253", "actual-spending",
  "actual-replacement", "actual-successor", "actual-replacement-successor"];
const SOURCES = ["packages/bitcoin-knots/src/blockfilter.cpp",
  "packages/bitcoin-knots/src/test/blockfilter_tests.cpp",
  "packages/bitcoin-knots/src/test/data/blockfilters.json",
  "packages/bitcoin-knots/src/crypto/siphash.cpp"];
type Surface = { id: string; requirements: string[]; evidence: string[];
  upstream: { sources: string[]; tests: string[] }; known_gaps: string[] };
type Index = { baseline: string; checklist: { surfaces: Surface[] } };

/** Structural provenance and narrow claim checks; behavioral parity remains in Rust/oracle tests. */
export function checkPhase154BasicFilters(maybeRoot?: string): string[] {
  const root = resolve(maybeRoot ?? ROOT);
  const failures: string[] = [];
  const texts = new Map<string, string>();
  for (const file of CHECK_FILES) {
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch (error) { failures.push(`${file}: cannot read required evidence: ${String(error)}`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  const requireText = (file: string, anchors: string[], context: string) => {
    for (const anchor of anchors) {
      if (!text(file).includes(anchor)) failures.push(`${file}: missing ${context}: ${anchor}`);
    }
  };
  requireText(FIXTURE, [PIN, "29.3.knots20260210"], "baseline pin");
  requireText(GENERATOR, [PIN, '"python3"', '"scripts/basic-filter-oracle.py"',
    "--porcelain", "timeout: 10_000", "maxBuffer: MAX_BYTES",
    "all ten pinned", "independent oracle disagrees", "BASIC fixture drift"], "oracle provenance/bounds");
  requireText(ORACLE, ["from test_framework.crypto.siphash import siphash",
    "from test_framework.messages import CBlock, CBlockHeader, hash256, ser_compact_size",
    ">> 64", "!= 0x6A", "ser_compact_size(count)", "hash256(digest + raw_predecessor)"],
    "independent source mechanism");
  if (/open_bitcoin|cargo|subprocess|os\.system/i.test(text(ORACLE))) {
    failures.push(`${ORACLE}: oracle independence forbids Rust execution/imports`);
  }
  if ((text(FIXTURE).match(/name: "corpus-/g) ?? []).length !== 10) {
    failures.push(`${FIXTURE}: expected exactly ten corpus vectors`);
  }
  requireText(FIXTURE, EDGE_NAMES.map(name => `name: "${name}"`), "edge provenance");
  requireText(FIXTURE, ["1560268, 1560268", "(64,", "(255,", "(256,", "(257,"], "collision/SipHash evidence");
  requireText(TEST, SOURCES.map(source => `// - ${source}`), "source breadcrumb");
  requireText(TEST, ['include!("../testdata/basic_filter_vectors.rs")', "parse_block",
    "filter_hash_display_hex", "filter_header_display_hex", "raw_duplicates",
    "ordered_and_replacement"], "behavioral test evidence");
  requireText(HISTORICAL, ["basic_filter_vectors.rs", "assert_independent", "restored_inputs",
    "actual-spending", "actual-replacement", "actual-replacement-successor"], "validated historical evidence");
  requireText("AGENTS.md", ["scripts/basic-filter-oracle.py", "narrow compatibility exception",
    "Bun"], "oracle compatibility exception");
  requireText(DOC, [PIN, "BASIC-only", "V0", "BIP37", "remain deferred",
    "source-audited", "corpus-cross-validated", "not a directly linked Knots binary",
    "Bun owns", "--check", "--write", "block-bounded", "missing earlier history"], "scoped documentation");
  requireText("README.md", ["./docs/parity/catalog/basic-compact-filters.md", "Pure BASIC filter construction"], "parity navigation");
  requireText("docs/parity/catalog/README.md", ["basic-compact-filters.md", "remain deferred"], "parity navigation");
  checkIndex(text("docs/parity/index.json"), failures);
  checkBreadcrumbs(text("docs/parity/source-breadcrumbs.json"), failures);
  checkVerifier(text("scripts/verify.sh"), failures);
  const readmeSection = text("README.md").split("## Pure BASIC filter construction")[1]?.split("\n## ")[0] ?? "";
  const claims = [[DOC, text(DOC)], ["README.md", readmeSection],
    ["docs/parity/catalog/README.md", text("docs/parity/catalog/README.md")]];
  for (const [file, claim] of claims) {
    const maybeMatch = claim.match(DENIED);
    if (maybeMatch) failures.push(`${file}: unsupported claim: ${maybeMatch[0]}`);
  }
  return failures;
}
function checkIndex(source: string, failures: string[]): void {
  try {
    const index = JSON.parse(source) as Index;
    if (index.baseline !== "29.3.knots20260210") failures.push("parity index: wrong baseline pin");
    const matching = index.checklist.surfaces.filter(surface => surface.id === SURFACE);
    if (matching.length !== 1) { failures.push("parity index: expected one BASIC surface"); return; }
    const surface = matching[0];
    for (const requirement of ["CFIL-01", "CFIL-02"]) {
      const owners = index.checklist.surfaces.filter(row => row.requirements.includes(requirement));
      if (owners.length !== 1 || owners[0].id !== SURFACE) failures.push(`${requirement}: wrong surface owner`);
    }
    for (const file of [DOC, TEST, HISTORICAL, FIXTURE, GENERATOR, ORACLE]) {
      if (!surface.evidence.includes(file)) failures.push(`parity surface evidence missing: ${file}`);
    }
    for (const file of SOURCES) {
      if (![...surface.upstream.sources, ...surface.upstream.tests].includes(file)) {
        failures.push(`parity surface source anchor missing: ${file}`);
      }
    }
    if (!surface.known_gaps.some(gap => gap.includes("remain deferred"))) failures.push("parity surface must retain deferred gaps");
    const maybeMatch = JSON.stringify(surface).match(DENIED);
    if (maybeMatch) failures.push(`parity surface: unsupported claim: ${maybeMatch[0]}`);
  } catch (error) {
    failures.push(`parity index: invalid contract: ${String(error)}`);
  }
}
function checkBreadcrumbs(source: string, failures: string[]): void {
  try {
    const index = JSON.parse(source) as { groups: { files: string[]; breadcrumbs: string[] }[] };
    const matching = index.groups.filter(group => group.files.includes(TEST));
    if (matching.length !== 1 || !SOURCES.every(source => matching[0].breadcrumbs.includes(source))) {
      failures.push("source breadcrumb manifest must register all independent test anchors");
    }
  } catch (error) { failures.push(`source breadcrumb manifest invalid: ${String(error)}`); }
}
function checkVerifier(source: string, failures: string[]): void {
  const required = [
    'bun test ./scripts/check-phase154-basic-filters.test.ts',
    'bun run scripts/check-phase154-basic-filters.ts',
    'bun run scripts/generate-basic-filter-vectors.ts --check',
  ];
  const lines = source.split("\n").filter(line => line.startsWith("run_step "));
  for (const command of required) {
    if (!lines.some(line => line.endsWith(command))) failures.push(`default verifier missing executable step: ${command}`);
  }
}
if (import.meta.main) {
  const failures = checkPhase154BasicFilters();
  if (failures.length) {
    console.error("Phase 154 BASIC filter evidence check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 154 BASIC filter provenance, evidence and scoped claims validated.");
}

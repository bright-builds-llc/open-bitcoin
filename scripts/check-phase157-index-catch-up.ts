#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { CONTRACTS, CORPUS, DOC, DOCUMENTED_LIMITS, EVIDENCE, NEW_RUST, NODE, PHASE, REGISTRATIONS, RPC, SOURCES, SURFACE, TESTS } from "./check-phase157-index-catch-up/contracts.ts";
import { compact, hasAssertions, hasOrderedCode, maybeFunction, ordinaryRust, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";

const GUARDS = ["scripts/check-phase157-index-catch-up.ts", "scripts/check-phase157-index-catch-up.test.ts",
  "scripts/check-phase157-index-catch-up/contracts.ts", "scripts/check-phase157-index-catch-up/rust-evidence.ts",
  "scripts/check-phase156-prune-coordination/rust-evidence.ts"];
export const CHECK_FILES = [...new Set([...CONTRACTS.map(row => row.file), ...TESTS.map(([file]) => file),
  ...REGISTRATIONS.map(([file]) => file), ...NEW_RUST, ...SOURCES, ...CORPUS, ...GUARDS, DOC, "README.md",
  "docs/parity/index.json", "docs/parity/checklist.md", "docs/parity/source-breadcrumbs.json", "scripts/verify.sh",
  PHASE + "157-UAT.md", PHASE + "157-TURN-MEASUREMENTS.md", ...["07", "08", "09"].map(plan => PHASE + `157-${plan}-SUMMARY.md`)])];
const REQUIREMENTS = ["CFAC-01", "CFAC-02", "CFIX-01"];
type Surface = { id: string; status: string; requirements: string[]; evidence: string[];
  upstream: { sources: string[]; tests: string[] }; known_gaps: string[] };

/** Structural source/claim guard; executed Rust measurements and native/security closure remain separate evidence. */
export function checkPhase157IndexCatchUp(maybeRoot?: string): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = new Map<string, string>();
  for (const file of CHECK_FILES) {
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch { failures.push(`${file}: cannot read required evidence`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  for (const row of CONTRACTS) {
    const maybeBody = maybeFunction(text(row.file), row.symbol, true);
    if (!maybeBody || !hasOrderedCode(maybeBody.body, row.anchors)) failures.push(`${row.threat}/${row.decision}: ordinary boundary ${row.file}:${row.symbol}`);
    for (const denied of row.maybeForbidden ?? []) if (maybeBody && compact(maybeBody.body).includes(compact(denied))) {
      const category = denied.endsWith("?") ? "early shutdown error" : "hidden full scan";
      failures.push(`T-157-31/${row.decision}: ${category} ${row.file}:${row.symbol}`);
    }
  }
  for (const [file, name] of TESTS) {
    const maybeTest = maybeFunction(text(file), name);
    if (!maybeTest || !/#\[(?:tokio::)?test(?:\]|\()/.test(maybeTest.attributes) || !hasAssertions(text(file), maybeTest.body)) {
      failures.push(`D-11: named executable assertion missing ${file}:${name}`);
    }
  }
  for (const [file, name, anchors] of EVIDENCE) {
    const maybeBody = maybeFunction(text(file), name);
    if (!maybeBody || !hasOrderedCode(maybeBody.body, anchors)) failures.push(`D-11: concrete evidence missing ${file}:${name}`);
  }
  for (const [file, modules] of REGISTRATIONS) for (const module of modules) {
    if (!compact(rustCode(text(file))).includes(compact(module))) failures.push(`D-12: registration missing ${file}:${module}`);
  }
  checkDefaults(text, failures);
  checkMeasurements(text, failures);
  checkBreadcrumbs(text, failures);
  checkParity(text, root, failures);
  checkDocumentation(text, failures);
  checkVerifier(text("scripts/verify.sh"), failures);
  for (const file of ["filter_index/catch_up.rs", "filter_index/catch_up/budget.rs"].map(file => "packages/open-bitcoin-chainstate/src/" + file)) {
    if (/std::(?:fs|io|net|time|process)|(?:fjall|tokio)::|SystemTime/.test(ordinaryRust(text(file)))) failures.push(`D-10: pure owner gained effects ${file}`);
  }
  return [...new Set(failures)];
}

function checkDefaults(text: (file: string) => string, failures: string[]): void {
  const source = text(NODE + "network/runtime_authority/filter_index/catch_up.rs");
  const maybeBudget = maybeFunction(source, "production_budget", true);
  const values = ["blocks: 8", "body_bytes: 1024 * 1024", "undo_bytes: 4 * 1024 * 1024", "cloned_bytes: 16 * 1024 * 1024",
    "script_items: 4 * 1024 * 1024", "script_bytes: 32 * 1024 * 1024", "encoded_bytes: 1024 * 1024",
    "record_operations: 512", "checkpoint_operations: 1_000_000", "projection_operations: 256",
    "blocks: 1", "body_bytes: 4_000_000 * 32", "undo_bytes: 256 * 1024 * 1024", "cloned_bytes: 1024 * 1024 * 1024",
    "script_items: 128 * 1024 * 1024", "script_bytes: 64 * 1024 * 1024 * 1024", "encoded_bytes: 0x0200_0000 + 170",
    "record_operations: 512", "checkpoint_operations: 1_000_000", "projection_operations: 256",
    "BasicIndexTurnBudget::new(normal, absolute)"];
  if (!maybeBudget || !hasOrderedCode(maybeBudget.body, values)) failures.push("D-14: measured production defaults drift");
  if (!compact(ordinaryRust(text(RPC + "bin/open_bitcoind/coins_flush.rs"))).includes("constTICK_SECS:u64=1;")) failures.push("D-08: ordinary timer default drift");
  const timer = compact(ordinaryRust(text(RPC + "bin/open_bitcoind/coins_flush.rs")));
  const durable = timer.split("implBasicIndexMaintenanceforManagedNetworkHandle<FjallChainstateStore,FjallCoinsView>{")[1]
    ?.split("implBasicIndexMaintenanceforManagedNetworkHandle<MemoryChainstateStore,MemoryCoinsView>")[0] ?? "";
  if (!durable.includes("self.drive_basic_filter_index_turn().map(Some).map_err(CoinsFlushError::BasicFilter)")) failures.push("D-08: concrete durable timer adapter missing");
  const owner = ordinaryRust(text(NODE + "storage/fjall_store/filters/ownership.rs"));
  const maybeFields = owner.match(/struct BasicFilterAppendProof\s*\{([^}]+)\}/)?.[1];
  if (!maybeFields || /\bpub\b/.test(maybeFields)) failures.push("D-06: append capability fields must remain private");
}

function checkMeasurements(text: (file: string) => string, failures: string[]): void {
  const source = text(PHASE + "157-TURN-MEASUREMENTS.md");
  for (const anchor of ["measurement_started_at:", "measurement_ended_at:", "2026-10-05T20:36:57.917", "2026-10-05T20:37:23.326",
    "1 registered measurement test passed", "25,409", "5,415", "989,871", "not RSS", "10,000-byte", "1,000,000", "16", "128", "512"]) {
    if (!source.includes(anchor)) failures.push(`T-157-31: executed measurement contract missing ${anchor}`);
  }
  for (const plan of ["07", "08", "09"]) {
    const summary = text(PHASE + `157-${plan}-SUMMARY.md`);
    if (!summary.includes("phase_lifecycle_id: 157-2026-10-05T14-50-48") || !summary.includes("## Self-Check: PASSED")) failures.push(`D-11: lifecycle evidence missing plan ${plan}`);
  }
}

function checkBreadcrumbs(text: (file: string) => string, failures: string[]): void {
  try {
    const manifest = JSON.parse(text("docs/parity/source-breadcrumbs.json")) as { groups: { files: string[]; breadcrumbs: string[] }[] };
    for (const file of NEW_RUST) {
      const matching = manifest.groups.filter(group => group.files?.includes(file));
      if (matching.length !== 1 || !matching[0].breadcrumbs.length) { failures.push(`D-12: breadcrumb mapping missing/duplicate ${file}`); continue; }
      for (const anchor of matching[0].breadcrumbs) if (!text(file).includes(`// - ${anchor}`)) failures.push(`D-12: source breadcrumb mismatch ${file}`);
    }
  } catch { failures.push("D-12: breadcrumb manifest invalid"); }
}

function checkParity(text: (file: string) => string, root: string, failures: string[]): void {
  try {
    const index = JSON.parse(text("docs/parity/index.json")) as { baseline: string; checklist: { surfaces: Surface[] } };
    const matching = index.checklist.surfaces.filter(row => row.id === SURFACE);
    if (index.baseline !== "29.3.knots20260210" || matching.length !== 1) { failures.push("T-157-29: pinned unique parity surface missing"); return; }
    const row = matching[0];
    if (!["in_progress", "done"].includes(row.status)) failures.push("T-157-29: invalid scoped parity status");
    if (JSON.stringify(row.requirements.toSorted()) !== JSON.stringify(REQUIREMENTS.toSorted())) failures.push("T-157-29: parity requirement scope drift");
    for (const requirement of REQUIREMENTS) {
      const owners = index.checklist.surfaces.filter(row => row.requirements.includes(requirement));
      if (owners.length !== 1 || owners[0].id !== SURFACE) failures.push(`T-157-29: unique requirement owner missing ${requirement}`);
    }
    for (const file of [...new Set([...CONTRACTS.map(row => row.file), ...TESTS.map(([file]) => file), DOC, PHASE + "157-TURN-MEASUREMENTS.md", PHASE + "157-UAT.md", ...GUARDS.slice(0, 4)])]) {
      if (!row.evidence.includes(file)) failures.push(`T-157-29: parity evidence missing ${file}`);
    }
    for (const file of SOURCES) if (!row.upstream.sources.includes(file)) failures.push(`T-157-29: pinned source missing ${file}`);
    for (const file of CORPUS) if (!row.upstream.tests.includes(file)) failures.push(`T-157-29: pinned corpus missing ${file}`);
    if (!row.known_gaps.some(gap => gap.includes("remain deferred"))) failures.push("T-157-29: explicit deferred scope missing");
    checkStringClaims(row, "parity surface", failures);
    if (row.status === "done") {
      for (const file of ["157-VERIFICATION.md", "157-SECURITY.md"]) {
        try {
          const evidence = readFileSync(resolve(root, PHASE, file), "utf8");
          const frontmatter = evidence.split("---")[1] ?? "";
          const valid = file === "157-VERIFICATION.md"
            ? /\nstatus: passed\s*\n/.test(frontmatter) && frontmatter.includes("generated_by: gsd-verifier")
            : /\nthreats_open: 0\s*\n/.test(frontmatter) && frontmatter.includes("generated_by: gsd-secure-phase");
          if (!frontmatter.includes("phase_lifecycle_id: 157-2026-10-05T14-50-48") || !valid) failures.push(`T-157-29: completed scope requires earned ${file}`);
        } catch { failures.push(`T-157-29: completed scope requires earned ${file}`); }
      }
    }
  } catch { failures.push("T-157-29: parity index invalid"); }
}

function checkDocumentation(text: (file: string) => string, failures: string[]): void {
  const current = text(DOC).split("## Safe activation and scheduled catch-up (Phase 157)")[1] ?? "";
  const tableRows = current.split("\n").filter(line => line.startsWith("|")).map(line => line.split("|").slice(1, -1).map(cell => cell.trim()));
  for (const expected of DOCUMENTED_LIMITS) {
    const matching = tableRows.filter(row => row[0] === expected[0]);
    if (matching.length !== 1 || JSON.stringify(matching[0]) !== JSON.stringify(expected)) failures.push(`T-157-29: documented production limit drift ${expected[0]}`);
  }
  for (const anchor of ["CFAC-01", "CFAC-02", "CFIX-01", "double-negative warning", "validated genesis", "one-second", "8 blocks",
    "startup", "suffix", "Periodic", "Always", "714", "989,871", "10,000-byte", "exact durable tip", "full native", "remain deferred"]) {
    if (!current.includes(anchor)) failures.push(`T-157-29: current documentation missing ${anchor}`);
  }
  for (const file of [DOC, "README.md", PHASE + "157-UAT.md"]) checkClaims(text(file), file, failures);
  if (!text("docs/parity/checklist.md").includes(SURFACE)) failures.push("T-157-29: checklist missing parity owner");
  const uat = text(PHASE + "157-UAT.md");
  for (const anchor of ["scripts/command-timings.ts", "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --", "bazel run //packages/open-bitcoin-rpc:open_bitcoind --", "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --", "bazel run //packages/open-bitcoin-cli:open_bitcoin --"]) {
    if (!uat.includes(anchor)) failures.push(`D-12: exact UAT command missing ${anchor}`);
  }
}

function checkClaims(source: string, file: string, failures: string[]): void {
  const subjects = /(?:peer (?:filter )?serving|filter(?:\/index)? RPC|RPC (?:filter )?serving|runtime reorg|unattended full sync|production readiness|production[- ]funds use|hardware power-loss proof|v2\.5)/gi;
  const positive = /\b(?:is|are)\s+(?:enabled|supported|available|active|ready|shipped|safe|established|complete)|\b(?:passed|succeeded)\b/i;
  for (const sentence of source.split(/(?<=[.!?])\s+|\n|;|\bbut\b|\bhowever\b|\band\b|\bwhile\b/)) {
    const matches = [...sentence.matchAll(subjects)];
    for (const [index, subject] of matches.entries()) {
      const end = matches[index + 1]?.index ?? sentence.length;
      const previous = index === 0 ? 0 : matches[index - 1].index + matches[index - 1][0].length;
      if (!positive.test(sentence.slice(subject.index, end))) continue;
      if (/\b(?:does not|do not|no claim|not a|future phase)\b/i.test(sentence.slice(previous, subject.index))) continue;
      failures.push(`T-157-29: unsupported claim in ${file}`);
    }
  }
}
function checkStringClaims(value: unknown, label: string, failures: string[]): void {
  if (typeof value === "string") { checkClaims(value, label, failures); return; }
  if (value === null || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value)) checkStringClaims(child, `${label}.${key}`, failures);
}
function checkVerifier(source: string, failures: string[]): void {
  const steps = source.split("\n").filter(line => line.startsWith("run_step "));
  let cursor = -1;
  for (const command of ["bun test ./scripts/check-phase156-prune-coordination.test.ts", "bun run scripts/check-phase156-prune-coordination.ts", "bun test ./scripts/check-phase157-index-catch-up.test.ts", "bun run scripts/check-phase157-index-catch-up.ts"]) {
    const next = steps.findIndex(line => line.endsWith(command));
    if (next <= cursor) failures.push(`T-157-31: default verifier missing/out-of-order ${command}`);
    cursor = Math.max(cursor, next);
  }
}
if (import.meta.main) {
  const failures = checkPhase157IndexCatchUp();
  if (failures.length) {
    console.error("Phase 157 source and claim check failed:");
    for (const failure of failures.slice(0, 40)) console.error(`- ${failure}`);
    if (failures.length > 40) console.error(`- ${failures.length - 40} additional bounded evidence findings`);
    process.exit(1);
  }
  console.log("Phase 157 ordinary wiring, named tests, scoped claims and provenance checked; structural checks supplement executed native evidence.");
}

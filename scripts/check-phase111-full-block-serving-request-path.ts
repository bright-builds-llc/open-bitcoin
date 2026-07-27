#!/usr/bin/env bun

import { existsSync } from "node:fs";
import path from "node:path";
import { readSourceCorpus } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-1-full-block-serving-request-path";
const PHASE110_TEST_COMMAND =
  "bun test scripts/check-phase110-block-serving-boundary.test.ts";
const PHASE110_CHECKER_COMMAND =
  "bun run scripts/check-phase110-block-serving-boundary.ts";
const PHASE111_TEST_COMMAND =
  "bun test scripts/check-phase111-full-block-serving-request-path.test.ts";
const PHASE111_CHECKER_COMMAND =
  "bun run scripts/check-phase111-full-block-serving-request-path.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_REQUIREMENTS = ["BSRV-04", "GOV-01", "GOV-05"] as const;
const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-node/src/network/block_serving.rs",
  "packages/open-bitcoin-node/src/network/inventory.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/architecture/status-snapshot.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/network/block_serving.rs",
  "packages/open-bitcoin-node/src/network/inventory.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "scripts/check-phase111-full-block-serving-request-path.ts",
  "scripts/check-phase111-full-block-serving-request-path.test.ts",
  "scripts/verify.sh",
  ".planning/phases/111-full-block-serving-request-path/111-01-SUMMARY.md",
  ".planning/phases/111-full-block-serving-request-path/111-02-SUMMARY.md",
  ".planning/phases/111-full-block-serving-request-path/111-03-SUMMARY.md",
] as const;
const REQUIRED_KNOTS_SOURCES = [
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/node/blockstorage.cpp",
  "packages/bitcoin-knots/src/validation.cpp",
] as const;
const REQUIRED_KNOTS_TESTS = [
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
] as const;
const REQUIRED_TERMS = [
  "BSRV-04",
  "GOV-01",
  "GOV-05",
  "InventoryType::Block",
  "InventoryType::WitnessBlock",
  "InventoryType::CompactBlock",
  "WireNetworkMessage::Block",
  "WireNetworkMessage::NotFound",
  "block_status_pruned",
  "block_status_unavailable",
  "block_request_cap_reached",
  "ManagedBlockServeInput",
  "serve_managed_block_request",
  "lookup_block",
] as const;
const REQUIRED_TESTS = [
  "phase111_side_chain_cached_block_is_not_served",
  "phase111_active_chain_non_tip_missing_local_block_returns_pruned_notfound",
  "phase111_active_tip_missing_local_block_returns_unavailable_notfound",
  "phase111_recent_valid_available_block_is_served_after_policy_gate",
  "phase111_stale_block_fact_returns_unavailable_notfound_without_lookup",
  "phase111_cached_old_block_outside_active_chain_is_not_archive_served",
  "phase111_managed_getdata_over_request_cap_disconnects_without_block_payload",
  "phase111_permissioned_block_getdata_still_hits_request_cap",
  "phase111_full_witness_block_cleanup_matrix_uses_phase110_labels",
  "phase111_compact_block_burst_remains_bounded_without_partial_state",
] as const;
const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...",
  "bash scripts/verify.sh",
] as const;
const CLAIM_SCAN_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
] as const;
const FORBIDDEN_CLAIM_PHRASES = [
  "bip152 compact block payload serving",
  "compact block payload serving",
  "bip152 implementation",
  "bip152 codec",
  "bip152 codecs",
  "compact reconstruction",
  "getblocktxn",
  "blocktxn",
  "archive-node behavior",
  "archive node behavior",
  "public block serving by default",
  "public serving by default",
  "package relay",
  "bloom/filter serving",
  "bloom filter serving",
  "compact filter serving",
  "compact-filter serving",
  "public-network ci",
  "production full-node readiness",
  "production service operation",
  "production-service operation",
  "production-funds wallet use",
  "production funds wallet use",
  "schema/orm",
  "schema or orm",
  "schema migration",
  "database migration",
] as const;
const NO_CLAIM_MARKERS = [
  "does not", "do not", "must not", "not ", "without", "outside", "out of scope",
  "deferred", "future", "later", "remain", "remains", "no claim", "not claim", "not supported",
] as const;
const POSITIVE_CLAIM_PATTERNS = [
  /\bsupports?\b/, /\bprovides?\b/, /\benables?\b/, /\badds?\b/, /\bimplements?\b/,
  /\bships?\b/, /\bserves?\b/, /\bresponds?\b/, /\bis supported\b/, /\bis enabled\b/,
  /\bis available\b/, /\bis complete\b/, /\bis ready\b/,
] as const;
const FORBIDDEN_VERIFIER_GATES = [
  "run-live-mainnet-smoke", "public-network", "wall-clock", "service-manager", "systemctl",
  "launchctl", "sleep 86400", "sleep 259200", "production-deployment", "schema-push",
  "database migration",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParitySurface = { name?: unknown; status?: unknown };

export function checkPhase111FullBlockServingRequestPath(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE111_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  checkParityRoots(texts, failures);
  checkRequiredTerms(texts, failures);
  checkRequiredTests(texts, failures);
  checkRequiredEvidenceRoots(texts, failures);
  checkRuntimeGuide(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  checkNoClaimBoundary(texts, failures);
  checkVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`BSRV-04 missing required Phase 111 corpus file: ${relativePath}`);
    return "";
  }
  return readSourceCorpus(repoRoot, relativePath);
}

function checkParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`BSRV-04 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  checkTopLevelSurface(parsed, failures);
  checkChecklistSurface(parsed, failures);
}

function checkTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("BSRV-04 parity index surfaces must be an array");
    return;
  }

  const matches = parsed.surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`BSRV-04 parity index must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  if (matches[0]?.status !== "done") {
    failures.push(`BSRV-04 parity index surface must be done: ${SURFACE_ID}`);
  }
}

function checkChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("BSRV-04 parity checklist surfaces must be an array");
    return;
  }

  const matches = surfaces.filter((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface[];
  if (matches.length !== 1) {
    failures.push(`BSRV-04 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0];
  if (surface.status !== "done") {
    failures.push(`BSRV-04 parity checklist surface must be done: ${SURFACE_ID}`);
  }
  requireExactRequirements(surface.requirements, REQUIRED_REQUIREMENTS, SURFACE_ID, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface.evidence, `${SURFACE_ID}.evidence`, evidence, failures);
  }
  for (const source of REQUIRED_KNOTS_SOURCES) {
    requireArrayIncludes(surface.upstream?.sources, `${SURFACE_ID}.upstream.sources`, source, failures);
  }
  for (const test of REQUIRED_KNOTS_TESTS) {
    requireArrayIncludes(surface.upstream?.tests, `${SURFACE_ID}.upstream.tests`, test, failures);
  }
}

function checkParityRoots(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2p = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklist = texts.get("docs/parity/checklist.md") ?? "";
  const index = texts.get("docs/parity/index.json") ?? "";
  const parityText = [p2p, checklist, index].join("\n");

  requireContains(p2p, SURFACE_ID, "BSRV-04 P2P catalog surface id", failures);
  requireContains(checklist, SURFACE_ID, "BSRV-04 checklist surface id", failures);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(p2p, requirement, `Phase 111 P2P catalog ${requirement}`, failures);
    requireContains(checklist, requirement, `Phase 111 checklist ${requirement}`, failures);
    requireContains(parityText, requirement, `Phase 111 parity root ${requirement}`, failures);
  }
  for (const anchor of [...REQUIRED_KNOTS_SOURCES, ...REQUIRED_KNOTS_TESTS]) {
    requireContains(parityText, anchor, "BSRV-04 Phase 111 Knots anchor", failures);
  }
}

function checkRequiredTerms(texts: Map<TargetFile, string>, failures: string[]): void {
  const combined = Array.from(texts.values()).join("\n");
  for (const term of REQUIRED_TERMS) {
    requireContains(combined, term, "Phase 111 evidence term", failures);
  }
}

function checkRequiredTests(texts: Map<TargetFile, string>, failures: string[]): void {
  const combinedSource = [
    texts.get("packages/open-bitcoin-node/src/network/block_serving.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/network/tests.rs") ?? "",
    texts.get("packages/open-bitcoin-network/src/peer/tests.rs") ?? "",
  ].join("\n");
  for (const testName of REQUIRED_TESTS) {
    requireContains(combinedSource, testName, "Phase 111 regression test", failures);
  }
}

function checkRequiredEvidenceRoots(texts: Map<TargetFile, string>, failures: string[]): void {
  const peerGetdataSource = texts.get("packages/open-bitcoin-network/src/peer/inventory_state.rs") ?? "";
  requireContains(peerGetdataSource, "fn handle_getdata", "Phase 111 peer getdata evidence root", failures);
  requireContains(peerGetdataSource, "request_pressure_input", "Phase 111 peer getdata pressure gate", failures);
  requireContains(peerGetdataSource, "PeerAction::ServeInventory", "Phase 111 peer inventory serving action", failures);

  const relayServingCases =
    texts.get("packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs") ?? "";
  requireContains(
    relayServingCases,
    "managed_getdata_preserves_block_serving_branch",
    "Phase 111 relay serving branch regression",
    failures,
  );
  requireContains(
    relayServingCases,
    "WireNetworkMessage::Block",
    "Phase 111 relay serving block response evidence",
    failures,
  );
}

function checkRuntimeGuide(text: string, failures: string[]): void {
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    requireContains(text, command, "Phase 111 runtime guide command or verifier boundary", failures);
  }
}

function checkNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of CLAIM_SCAN_FILES) {
    const text = texts.get(file) ?? "";
    for (const unit of contextUnits(text)) {
      checkNoForbiddenClaim(file, unit, failures);
    }
  }
}

function checkNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  const lower = normalizedLower(unit);
  if (!isPhase111Unit(lower)) {
    return;
  }
  for (const claim of FORBIDDEN_CLAIM_PHRASES) {
    if (lower.includes(claim) && isPositiveClaim(lower) && !isNoClaimContext(lower, claim)) {
      failures.push(`forbidden Phase 111 positive claim in ${file}: ${unit}`);
    }
  }
}

function isPhase111Unit(lowerUnit: string): boolean {
  return (
    lowerUnit.includes("phase 111")
    || lowerUnit.includes(SURFACE_ID)
    || lowerUnit.includes("inventorytype::block")
    || lowerUnit.includes("wireNetworkMessage::block".toLowerCase())
    || lowerUnit.includes("block_status_pruned")
    || lowerUnit.includes("block_status_unavailable")
  );
}

function isPositiveClaim(lowerUnit: string): boolean {
  return POSITIVE_CLAIM_PATTERNS.some((pattern) => pattern.test(lowerUnit));
}

function isNoClaimContext(lowerUnit: string, claim: string): boolean {
  if (
    lowerUnit.includes("does not claim")
    || lowerUnit.includes("does not add")
    || lowerUnit.includes("does not enable")
    || lowerUnit.includes("do not claim")
    || lowerUnit.includes("do not add")
    || lowerUnit.includes("do not implement")
    || lowerUnit.includes("do not enable")
    || lowerUnit.includes("without adding")
    || lowerUnit.includes("remain out of scope")
    || lowerUnit.includes("remains out of scope")
    || lowerUnit.includes("remain outside")
    || lowerUnit.includes("remains outside")
    || lowerUnit.includes("remain deferred")
    || lowerUnit.includes("remains deferred")
    || lowerUnit.includes("future phases")
    || lowerUnit.includes("future scoped surfaces")
  ) {
    return true;
  }

  let searchFrom = 0;
  while (searchFrom < lowerUnit.length) {
    const claimIndex = lowerUnit.indexOf(claim, searchFrom);
    if (claimIndex === -1) {
      return true;
    }
    const context = lowerUnit.slice(
      Math.max(0, claimIndex - 96),
      Math.min(lowerUnit.length, claimIndex + claim.length + 96),
    );
    if (!NO_CLAIM_MARKERS.some((marker) => context.includes(marker))) {
      return false;
    }
    searchFrom = claimIndex + claim.length;
  }
  return true;
}

function checkVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("BSRV-04 default verifier missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [PHASE110_TEST_COMMAND, PHASE110_CHECKER_COMMAND, PHASE111_TEST_COMMAND, PHASE111_CHECKER_COMMAND],
      "BSRV-04 default verifier visible order must place Phase 111 after Phase 110",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 111 full block-serving request path checker" ${PHASE111_TEST_COMMAND}`,
    "BSRV-04 executable verifier Phase 111 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 111 full block-serving request path" ${PHASE111_CHECKER_COMMAND}`,
    "BSRV-04 executable verifier Phase 111 checker",
    failures,
  );
  requireContains(text, "Phase 108 is followed by Phase 110", "BSRV-04 verifier ordering comment", failures);
  verifyOrderedCommands(
    executableText,
    [PHASE110_TEST_COMMAND, PHASE110_CHECKER_COMMAND, PHASE111_TEST_COMMAND, PHASE111_CHECKER_COMMAND, PURE_CORE_COMMAND],
    "BSRV-04 executable verifier commands must run Phase 111 after Phase 110 and before pure-core checks",
    failures,
  );

  for (const forbidden of FORBIDDEN_VERIFIER_GATES) {
    if (executableText.toLowerCase().includes(forbidden.toLowerCase())) {
      failures.push(`forbidden Phase 111 default verifier gate text: ${forbidden}`);
    }
  }
}

function requireExactRequirements(
  value: unknown,
  expected: readonly string[],
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const wanted = JSON.stringify(expected);
  if (actual !== wanted) {
    failures.push(`${label} requirements mismatch: expected ${wanted}, got ${actual}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} must be an array`);
    return;
  }
  if (!value.includes(required)) {
    failures.push(`${label} missing required value: ${required}`);
  }
}

function verifyOrderedCommands(
  text: string,
  commands: readonly string[],
  failure: string,
  failures: string[],
): void {
  let previousIndex = -1;
  for (const command of commands) {
    const currentIndex = text.indexOf(command);
    if (currentIndex === -1 || currentIndex <= previousIndex) {
      failures.push(failure);
      return;
    }
    previousIndex = currentIndex;
  }
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function contextUnits(text: string): string[] {
  const units: string[] = [];
  for (const block of text.replaceAll("\r\n", "\n").split(/\n\s*\n/)) {
    const lines = block
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
    if (lines.length === 0) {
      continue;
    }

    const tableRows = lines.filter((line) => line.startsWith("|") && !/^\|\s*-/.test(line));
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      units.push(...sentenceUnits(lines.filter((line) => !line.startsWith("|")).join(" ")));
      continue;
    }

    units.push(...sentenceUnits(lines.join(" ")));
  }
  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}

function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  return normalized.length === 0 ? [] : normalized.split(/(?<=[.!?])\s+/);
}

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
}

if (import.meta.main) {
  const failures = checkPhase111FullBlockServingRequestPath();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 111 full block-serving request path");
  }
}

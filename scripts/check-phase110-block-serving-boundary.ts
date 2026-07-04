#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-1-block-serving-activation-eligibility-boundary";
const PHASE108_TEST_COMMAND =
  "bun test scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts";
const PHASE108_CHECKER_COMMAND =
  "bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts";
const PHASE110_TEST_COMMAND =
  "bun test scripts/check-phase110-block-serving-boundary.test.ts";
const PHASE110_CHECKER_COMMAND =
  "bun run scripts/check-phase110-block-serving-boundary.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_REQUIREMENTS = ["BSRV-01", "BSRV-02", "BSRV-03", "BSRV-05", "BSRV-06"] as const;
const TARGET_FILES = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "packages/open-bitcoin-network/src/block_serving.rs",
  "packages/open-bitcoin-network/src/block_serving/tests.rs",
  "packages/open-bitcoin-rpc/src/config/open_bitcoin.rs",
  "packages/open-bitcoin-rpc/src/config/loader/block_serving.rs",
  "packages/open-bitcoin-node/src/status/block_serving.rs",
  "packages/open-bitcoin-node/src/status/block_serving/tests.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "scripts/check-phase110-block-serving-boundary.ts",
  "scripts/check-phase110-block-serving-boundary.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_KNOTS_SOURCES = [
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/validation.cpp",
  "packages/bitcoin-knots/src/node/blockstorage.cpp",
] as const;
const REQUIRED_KNOTS_TESTS = [
  "packages/bitcoin-knots/test/functional/p2p_getdata.py",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
] as const;
const REQUIRED_CONFIG_TERMS = [
  "block_serving.enabled",
  "block_serving.compact_relay_enabled",
  "-openbitcoinblockserving",
  "-openbitcoincompactrelay",
] as const;
const REQUIRED_POLICY_TERMS = [
  "BlockRelayActivationPolicy",
  "classify_block_serving_eligibility",
  "classify_block_serving_status",
  "evaluate_block_serving_resource_gate",
  "classify_block_inflight_cleanup",
  "BlockServingEvidenceStatus",
] as const;
const REQUIRED_LABELS = [
  "eligible",
  "disabled",
  "activation_required",
  "inbound_serving_required",
  "permission_required",
  "protected_not_serving",
  "status_unavailable",
  "permission_effect_inactive",
  "validated",
  "available",
  "stale",
  "side_chain",
  "pruned",
  "unavailable",
  "unvalidated",
  "unknown",
  "suppressed",
  "block_request_cap_reached",
  "block_inflight_cleanup_released",
  "block_inflight_cleanup_peer_removed",
  "block_inflight_cleanup_timeout",
  "block_inflight_cleanup_restart",
  "block_inflight_limit_still_reached",
] as const;
const REQUIRED_RUNTIME_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "Public-network block-serving or compact-relay review is opt-in UAT guidance",
  "bash scripts/verify.sh",
] as const;
const CLAIM_SCAN_FILES = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
] as const;
const FORBIDDEN_CLAIM_PHRASES = [
  "public block serving by default",
  "public serving by default",
  "archive-node behavior",
  "archive node behavior",
  "package relay",
  "bloom/filter serving",
  "bloom filter serving",
  "bip37 serving",
  "compact filter serving",
  "compact-filter serving",
  "bip152 implementation",
  "bip152 codec",
  "bip152 codecs",
  "compact reconstruction",
  "full block serving responses",
  "full block serving response",
  "full block responses",
  "getblocktxn",
  "blocktxn",
  "public-network ci",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
  "production-funds wallet use",
  "production funds wallet use",
] as const;
const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "must not",
  "not ",
  "without",
  "outside",
  "out of scope",
  "deferred",
  "future",
  "later",
  "remain",
  "remains",
  "no claim",
  "not claim",
  "not supported",
  "only",
] as const;
const POSITIVE_CLAIM_PATTERNS = [
  /\bsupports?\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bserves?\b/,
  /\bresponds?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis available\b/,
  /\bis complete\b/,
  /\bis ready\b/,
] as const;
const FORBIDDEN_VERIFIER_GATES = [
  "run-live-mainnet-smoke",
  "public-network",
  "wall-clock",
  "service-manager",
  "systemctl",
  "launchctl",
  "sleep 86400",
  "sleep 259200",
  "production-deployment",
  "schema-push",
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

export function checkPhase110BlockServingBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE110_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  checkConfigTerms(texts.get("docs/architecture/config-precedence.md") ?? "", failures);
  checkParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  checkParityRoots(texts, failures);
  checkRequiredTerms(texts, failures);
  checkRuntimeGuide(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  checkNoClaimBoundary(texts, failures);
  checkVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`BSRV-01 missing required Phase 110 corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function checkConfigTerms(text: string, failures: string[]): void {
  for (const term of REQUIRED_CONFIG_TERMS) {
    requireContains(text, term, "BSRV-01 Phase 110 config activation boundary", failures);
  }
  requireContains(text, "default-off", "BSRV-01 Phase 110 default-off config wording", failures);
}

function checkParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`BSRV-01 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  checkTopLevelSurface(parsed, failures);
  checkChecklistSurface(parsed, failures);
}

function checkTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("BSRV-01 parity index surfaces must be an array");
    return;
  }

  const matches = parsed.surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`BSRV-01 parity index must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  if (matches[0]?.status !== "done") {
    failures.push(`BSRV-01 parity index surface must be done: ${SURFACE_ID}`);
  }
}

function checkChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("BSRV-01 parity checklist surfaces must be an array");
    return;
  }

  const matches = surfaces.filter((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface[];
  if (matches.length !== 1) {
    failures.push(`BSRV-01 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0];
  if (surface.status !== "done") {
    failures.push(`BSRV-01 parity checklist surface must be done: ${SURFACE_ID}`);
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

  requireContains(p2p, SURFACE_ID, "BSRV-01 P2P catalog surface id", failures);
  requireContains(checklist, SURFACE_ID, "BSRV-01 checklist surface id", failures);
  for (const requirement of REQUIRED_REQUIREMENTS) {
    requireContains(p2p, requirement, `Phase 110 P2P catalog ${requirement}`, failures);
    requireContains(checklist, requirement, `Phase 110 checklist ${requirement}`, failures);
    requireContains(parityText, requirement, `Phase 110 parity root ${requirement}`, failures);
  }
  for (const anchor of [...REQUIRED_KNOTS_SOURCES, ...REQUIRED_KNOTS_TESTS]) {
    requireContains(parityText, anchor, "BSRV-06 Phase 110 Knots anchor", failures);
  }
}

function checkRequiredTerms(texts: Map<TargetFile, string>, failures: string[]): void {
  const combined = Array.from(texts.values()).join("\n");
  for (const term of [...REQUIRED_CONFIG_TERMS, ...REQUIRED_POLICY_TERMS, ...REQUIRED_LABELS]) {
    requireContains(combined, term, "Phase 110 evidence term", failures);
  }
}

function checkRuntimeGuide(text: string, failures: string[]): void {
  for (const command of REQUIRED_RUNTIME_COMMANDS) {
    requireContains(text, command, "Phase 110 runtime guide command or UAT boundary", failures);
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
  if (!isPhase110Unit(lower)) {
    return;
  }
  for (const claim of FORBIDDEN_CLAIM_PHRASES) {
    if (lower.includes(claim) && isPositiveClaim(lower) && !isNoClaimContext(lower, claim)) {
      failures.push(`forbidden Phase 110 positive claim in ${file}: ${unit}`);
    }
  }
}

function isPhase110Unit(lowerUnit: string): boolean {
  return (
    lowerUnit.includes("phase 110")
    || lowerUnit.includes(SURFACE_ID)
    || lowerUnit.includes("block_serving.")
    || lowerUnit.includes("-openbitcoinblockserving")
    || lowerUnit.includes("-openbitcoincompactrelay")
    || lowerUnit.includes("blockservingevidencestatus")
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
    || lowerUnit.includes("future v2.1 phases")
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
    failures.push("BSRV-01 default verifier missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [PHASE108_TEST_COMMAND, PHASE108_CHECKER_COMMAND, PHASE110_TEST_COMMAND, PHASE110_CHECKER_COMMAND],
      "BSRV-01 default verifier visible order must place Phase 110 after Phase 108",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 110 block-serving boundary checker" ${PHASE110_TEST_COMMAND}`,
    "BSRV-01 executable verifier Phase 110 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 110 block-serving boundary" ${PHASE110_CHECKER_COMMAND}`,
    "BSRV-01 executable verifier Phase 110 checker",
    failures,
  );
  requireContains(text, "Phase 108 is followed by Phase 110", "BSRV-01 verifier ordering comment", failures);
  verifyOrderedCommands(
    executableText,
    [PHASE108_TEST_COMMAND, PHASE108_CHECKER_COMMAND, PHASE110_TEST_COMMAND, PHASE110_CHECKER_COMMAND, PURE_CORE_COMMAND],
    "BSRV-01 executable verifier commands must run Phase 110 after Phase 108 and before pure-core checks",
    failures,
  );

  for (const forbidden of FORBIDDEN_VERIFIER_GATES) {
    if (executableText.toLowerCase().includes(forbidden.toLowerCase())) {
      failures.push(`forbidden Phase 110 default verifier gate text: ${forbidden}`);
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
  const failures = checkPhase110BlockServingBoundary();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 110 block-serving boundary");
  }
}

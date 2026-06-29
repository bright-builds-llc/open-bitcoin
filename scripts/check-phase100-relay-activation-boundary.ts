#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-0-relay-activation-boundary";
const PHASE99_TEST_COMMAND =
  "bun test scripts/check-phase99-peer-policy-structured-log-emission.test.ts";
const PHASE99_CHECKER_COMMAND =
  "bun run scripts/check-phase99-peer-policy-structured-log-emission.ts";
const PHASE100_TEST_COMMAND =
  "bun test scripts/check-phase100-relay-activation-boundary.test.ts";
const PHASE100_CHECKER_COMMAND =
  "bun run scripts/check-phase100-relay-activation-boundary.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_ACT_REQUIREMENTS = ["ACT-01", "ACT-02", "ACT-03", "ACT-04"] as const;
const TARGET_FILES = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;
const REQUIRED_PHASE100_EVIDENCE = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/src/relay.rs",
  "packages/open-bitcoin-rpc/src/config/open_bitcoin.rs",
  "packages/open-bitcoin-rpc/src/config/loader.rs",
  "scripts/check-phase100-relay-activation-boundary.ts",
  "scripts/check-phase100-relay-activation-boundary.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net_permissions.h",
  "packages/bitcoin-knots/src/net_permissions.cpp",
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/test/functional/p2p_permissions.py",
] as const;
const REQUIRED_EVIDENCE_LABELS = [
  "relay.enabled",
  "openbitcoinrelay",
  "transaction_relay_policy_input",
  "force_relay_policy_input",
  "mempool_policy_input",
  "inactive_bloomfilter",
  "inactive_blockfilters",
  "eligible",
  "disabled",
  "activation_required",
  "inbound_serving_required",
  "permission_required",
  "protected_not_relay",
  "permission_effect_inactive",
] as const;
const REQUIRED_UAT_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
  "-openbitcoinrelay=1",
  "-openbitcoininbound=1",
  "-openbitcoinlisten=127.0.0.1",
  "-openbitcoininboundpermissionclass=relay_loopback@127.0.0.1=in,relay,forcerelay,mempool",
  "Public-network relay review is opt-in and outside `bash scripts/verify.sh`",
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
  "public relay by default",
  "public relay defaults",
  "public transaction relay by default",
  "compact block relay",
  "compact blocks",
  "bloom/filter serving",
  "bloom filter serving",
  "bip37 serving",
  "compact-filter serving",
  "compact filter serving",
  "package relay",
  "production service operation",
  "production-service operation",
  "production full-node readiness",
  "production-funds wallet use",
  "production funds wallet use",
  "public-network relay ci",
  "public relay ci",
  "transaction download scheduling",
  "orphan handling",
  "mempool admission",
  "relay serving/fanout",
  "relay fanout",
  "rebroadcast",
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
  "free of",
] as const;
const POSITIVE_CLAIM_PATTERNS = [
  /\bsupports?\b/,
  /\bprovides?\b/,
  /\benables?\b/,
  /\badds?\b/,
  /\bimplements?\b/,
  /\bships?\b/,
  /\bis supported\b/,
  /\bis enabled\b/,
  /\bis available\b/,
  /\bis complete\b/,
  /\bis ready\b/,
] as const;
const WHOLE_UNIT_NO_CLAIM_MARKERS = [
  "does not ",
  "do not ",
  "must not ",
  "without ",
  "remain outside",
  "remains outside",
  "remain follow-up",
  "remain deferred",
  "remains deferred",
  "outside this scope",
  "outside this surface",
  "future ",
  "later phases own",
  "not supported",
  "free of",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "openbitcoinlisten=0.0.0.0",
  "public-network relay",
  "public relay ci",
  "production full-node readiness",
  "production service operation",
  "production-service operation",
  "service-manager",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "sleep 86400",
  "--restart-after-progress",
  "run-live-mainnet-smoke",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = { name?: unknown; status?: unknown };

export function checkPhase100RelayActivationBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE100_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyConfigBoundary(texts.get("docs/architecture/config-precedence.md") ?? "", failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyParityRoots(texts, failures);
  verifyEvidenceLabels(texts, failures);
  verifyUatCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`ACT-04 missing required Phase 100 corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function verifyConfigBoundary(text: string, failures: string[]): void {
  for (const needle of [
    "relay.enabled",
    "-openbitcoinrelay",
    "default-off",
    "Open Bitcoin-owned",
    "whitelist and whitebind remain rejected",
  ]) {
    requireContains(text, needle, "ACT-01 Phase 100 config activation boundary", failures);
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`ACT-01 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("ACT-01 parity index surfaces must be an array");
    return;
  }

  const matches = parsed.surfaces.filter((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface[];
  if (matches.length !== 1) {
    failures.push(`ACT-01 parity index must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }
  if (matches[0]?.status !== "done") {
    failures.push(`ACT-01 parity index surface must be done: ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("ACT-01 parity checklist surfaces must be an array");
    return;
  }

  const matches = surfaces.filter((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface[];
  if (matches.length !== 1) {
    failures.push(`ACT-01 parity checklist must contain exactly one surface: ${SURFACE_ID}`);
    return;
  }

  const surface = matches[0];
  if (surface.status !== "done") {
    failures.push(`ACT-01 parity checklist surface must be done: ${SURFACE_ID}`);
  }
  requireExactRequirements(
    surface.requirements,
    REQUIRED_ACT_REQUIREMENTS,
    `ACT-01 ${SURFACE_ID}`,
    failures,
  );
  for (const evidence of REQUIRED_PHASE100_EVIDENCE) {
    requireArrayIncludes(surface.evidence, `ACT-01 ${SURFACE_ID}.evidence`, evidence, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS.slice(0, 4)) {
    requireArrayIncludes(surface.upstream?.sources, `ACT-02 ${SURFACE_ID}.upstream.sources`, anchor, failures);
  }
  requireArrayIncludes(
    surface.upstream?.tests,
    `ACT-02 ${SURFACE_ID}.upstream.tests`,
    "packages/bitcoin-knots/test/functional/p2p_permissions.py",
    failures,
  );
}

function verifyParityRoots(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2p = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklist = texts.get("docs/parity/checklist.md") ?? "";
  const breadcrumbs = texts.get("docs/parity/source-breadcrumbs.json") ?? "";
  const parityText = [
    p2p,
    checklist,
    texts.get("docs/parity/index.json") ?? "",
    breadcrumbs,
  ].join("\n");

  for (const id of REQUIRED_ACT_REQUIREMENTS) {
    requireContains(p2p, id, `ACT-01 P2P catalog ${id}`, failures);
    requireContains(checklist, id, `ACT-01 checklist ${id}`, failures);
    requireContains(parityText, id, `ACT-01 parity root ${id}`, failures);
  }
  requireContains(p2p, SURFACE_ID, "ACT-01 P2P catalog surface id", failures);
  requireContains(checklist, SURFACE_ID, "ACT-01 checklist surface id", failures);
  requireContains(breadcrumbs, "network-relay-activation-boundary", "ACT-02 source breadcrumb group", failures);
  requireContains(
    breadcrumbs,
    "packages/open-bitcoin-network/src/relay.rs",
    "ACT-02 source breadcrumb relay policy file",
    failures,
  );
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireContains(parityText, anchor, "ACT-02 Phase 100 Knots anchor", failures);
  }
}

function verifyEvidenceLabels(texts: Map<TargetFile, string>, failures: string[]): void {
  const combined = Array.from(texts.values()).join("\n");
  for (const label of REQUIRED_EVIDENCE_LABELS) {
    requireContains(combined, label, "Phase 100 evidence label", failures);
  }
}

function verifyUatCommands(text: string, failures: string[]): void {
  for (const command of REQUIRED_UAT_COMMANDS) {
    requireContains(text, command, "ACT-01 Phase 100 UAT command", failures);
  }
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of CLAIM_SCAN_FILES) {
    const text = texts.get(file) ?? "";
    for (const unit of contextUnits(text)) {
      verifyNoForbiddenClaim(file, unit, failures);
    }
  }
}

function verifyNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  const lower = normalizedLower(unit);
  for (const claim of FORBIDDEN_CLAIM_PHRASES) {
    if (lower.includes(claim) && isPositiveClaim(lower) && !isNoClaimContext(lower, claim)) {
      failures.push(`ACT-04 forbidden Phase 100 positive claim in ${file}: ${unit}`);
    }
  }
}

function isPositiveClaim(lowerUnit: string): boolean {
  return POSITIVE_CLAIM_PATTERNS.some((pattern) => pattern.test(lowerUnit));
}

function isNoClaimContext(lowerUnit: string, claim: string): boolean {
  if (
    lowerUnit.includes("does not claim")
    || lowerUnit.includes("do not claim")
    || lowerUnit.includes("does not add")
    || lowerUnit.includes("without adding")
    || lowerUnit.includes("remain out of scope")
    || lowerUnit.includes("remains out of scope")
    || WHOLE_UNIT_NO_CLAIM_MARKERS.some((marker) => lowerUnit.includes(marker))
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

function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("ACT-01 default verifier missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [
        PHASE99_TEST_COMMAND,
        PHASE99_CHECKER_COMMAND,
        PHASE100_TEST_COMMAND,
        PHASE100_CHECKER_COMMAND,
      ],
      "ACT-01 default verifier visible order must place Phase 100 immediately after Phase 99",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 100 relay activation boundary checker" ${PHASE100_TEST_COMMAND}`,
    "ACT-01 executable verifier Phase 100 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 100 relay activation boundary" ${PHASE100_CHECKER_COMMAND}`,
    "ACT-01 executable verifier Phase 100 checker",
    failures,
  );
  requireContains(text, "Phase 99 is followed by Phase 100", "ACT-01 verifier ordering comment", failures);
  verifyOrderedCommands(
    executableText,
    [
      PHASE99_TEST_COMMAND,
      PHASE99_CHECKER_COMMAND,
      PHASE100_TEST_COMMAND,
      PHASE100_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "ACT-01 executable verifier commands must run Phase 100 after Phase 99 and before pure-core checks",
    failures,
  );

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.toLowerCase().includes(forbidden.toLowerCase())) {
      failures.push(`ACT-04 default verifier contains forbidden Phase 100 gate text: ${forbidden}`);
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
  const failures = checkPhase100RelayActivationBoundary();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 100 relay activation boundary");
  }
}

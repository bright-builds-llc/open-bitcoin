#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-eviction-ban-misbehavior-policy";
const PHASE92_TEST_COMMAND = "bun test scripts/check-phase92-address-boundaries.test.ts";
const PHASE92_CHECKER_COMMAND = "bun run scripts/check-phase92-address-boundaries.ts";
const PHASE93_TEST_COMMAND = "bun test scripts/check-phase93-peer-policy.test.ts";
const PHASE93_CHECKER_COMMAND = "bun run scripts/check-phase93-peer-policy.ts";
const PHASE93_REQUIREMENTS = ["EVICT-01", "EVICT-02", "EVICT-03", "EVICT-04"] as const;
const PEER_POLICY_PERMISSION_FLAG =
  "-openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=in,noban,forceinbound,download,addr";

type RequiredCommand = { label: string; required: readonly [string, ...string[]] };
type BreadcrumbGroup = { breadcrumbs?: unknown; files?: unknown; label?: unknown };
type BreadcrumbIndex = { groups?: unknown };
type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};
type ParityIndex = { checklist?: { surfaces?: unknown }; surfaces?: unknown };
type ParitySurface = { name?: unknown; status?: unknown };

function requiredCommand(
  label: string,
  first: string,
  ...required: string[]
): RequiredCommand {
  return { label, required: [first, ...required] };
}

const TARGET_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/source-breadcrumbs.json",
] as const;
const REQUIRED_LABELS = [
  "eviction_candidates_evaluated",
  "disconnects_requested",
  "discouraged_peers",
  "active_bans",
  "expired_bans",
  "manual_unbans",
  "misbehavior_observations",
  "protected_no_actions",
  "latest_peer_policy_decision",
  "eviction_candidate_selected",
  "eviction_suppressed",
  "misbehavior_policy_decision",
  "source_eviction_policy",
  "source_misbehavior_policy",
] as const;
const REQUIRED_UAT_COMMANDS = [
  requiredCommand(
    "Cargo peer-policy daemon startup",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
    "-openbitcoininbound=1",
    "-openbitcoinlisten=127.0.0.1:18444",
    PEER_POLICY_PERMISSION_FLAG,
  ),
  requiredCommand(
    "Bazel peer-policy daemon startup",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
    "-openbitcoininbound=1",
    "-openbitcoinlisten=127.0.0.1:18444",
    PEER_POLICY_PERMISSION_FLAG,
  ),
  requiredCommand(
    "Cargo peer-policy network status",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --",
    "openbitcoinnetworkstatus",
  ),
  requiredCommand(
    "Bazel peer-policy network status",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --",
    "openbitcoinnetworkstatus",
  ),
  requiredCommand(
    "Cargo peer-policy operator status",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "status --format json",
  ),
  requiredCommand(
    "Bazel peer-policy operator status",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "status --format json",
  ),
  requiredCommand(
    "Cargo peer-policy support bundle",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "support bundle --output-dir=/tmp/open-bitcoin-peer-policy-support",
  ),
  requiredCommand(
    "Bazel peer-policy support bundle",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "support bundle --output-dir=/tmp/open-bitcoin-peer-policy-support",
  ),
] as const;
const REQUIRED_CATALOG_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/banman.h",
  "packages/bitcoin-knots/src/banman.cpp",
  "packages/bitcoin-knots/src/net_permissions.cpp",
] as const;
const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-network/src/peer/policy_state.rs",
  "packages/open-bitcoin-network/src/peer_policy.rs",
  "packages/open-bitcoin-network/src/peer_policy/tests.rs",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "curl ",
  "nc ",
  "systemctl",
  "launchctl",
  "dig ",
  "nslookup",
  "--public-network",
  "multi-day",
] as const;
const FORBIDDEN_POSITIVE_CLAIMS = [
  "production banlist parity is complete",
  "public ban enforcement is supported",
  "knots discourage parity is supported",
  "broad dos/resource governance is complete",
  "resource exhaustion coverage is complete",
  "transaction relay abuse handling is complete",
  "compact block relay abuse handling is complete",
  "public inbound by default is enabled",
  "production full-node readiness is achieved",
] as const;
const ALLOWED_SCOPE_TERMS = [
  "does not",
  "do not",
  "not ",
  "no ",
  "without",
  "outside",
  "remain outside",
  "remains outside",
  "deferred",
  "future",
  "not claim",
  "not claiming",
  "no-claim",
  "non-claim",
] as const;
const RAW_EVIDENCE_STRINGS = [
  "127.0.0.1:",
  "0.0.0.0:",
  "::1",
  "peer_id=",
  "peer-",
  "raw_permission",
  "operator_loopback",
  "in,noban",
] as const;
const RAW_EVIDENCE_FILES = [
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
] as const;
const COMMAND_PREFIXES = Array.from(
  new Set(REQUIRED_UAT_COMMANDS.map((command) => command.required[0])),
);

export type CheckPhase93Options = { rootDir?: string };
type TargetFile = (typeof TARGET_FILES)[number];

export function checkPhase93PeerPolicy(options: CheckPhase93Options = {}): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyRuntimeGuideCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifyEvidenceLabels(texts, failures);
  verifyParityDocs(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);
  verifyRawEvidenceBoundary(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required Phase 93 corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizeShellCommand(text: string): string {
  return normalizeWhitespace(text.replace(/\\\s*/g, " "));
}

function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
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

function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizedLower(text).includes(normalizedLower(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
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

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`Phase 93 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.surfaces)) {
    failures.push("Phase 93 parity index surfaces must be an array");
    return;
  }
  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`Phase 93 parity index missing done surface: ${SURFACE_ID}`);
  }

  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("Phase 93 checklist surfaces must be an array");
    return;
  }
  const checklistSurface = checklistSurfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  if (checklistSurface?.status !== "done") {
    failures.push(`Phase 93 checklist missing done ${SURFACE_ID}`);
  }
  const actual = JSON.stringify(checklistSurface?.requirements);
  const expected = JSON.stringify(PHASE93_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`Phase 93 requirements mismatch: expected ${expected}, got ${actual}`);
  }
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(checklistSurface?.evidence, `${SURFACE_ID}.evidence`, evidence, failures);
  }
}

function verifyRuntimeGuideCommands(text: string, failures: string[]): void {
  const commandUnits = shellCommandUnits(text);
  for (const command of REQUIRED_UAT_COMMANDS) {
    const found = commandUnits.some((unit) =>
      command.required.every((required) => unit.includes(normalizeShellCommand(required))),
    );
    if (!found) {
      failures.push(`Phase 93 UAT command missing ${command.label}: ${command.required.join(" ")}`);
    }
  }
}

function shellCommandUnits(text: string): string[] {
  const units: string[] = [];
  let currentLines: string[] = [];

  for (const rawLine of text.replaceAll("\r\n", "\n").split("\n")) {
    const line = rawLine.trim();
    if (COMMAND_PREFIXES.some((prefix) => line.startsWith(prefix))) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [line];
      continue;
    }
    if (currentLines.length === 0) {
      continue;
    }
    if (line.length === 0 || line.startsWith("```")) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [];
      continue;
    }
    currentLines.push(line);
  }

  pushCurrentShellCommandUnit(currentLines, units);
  return units;
}

function pushCurrentShellCommandUnit(currentLines: string[], units: string[]): void {
  if (currentLines.length > 0) {
    units.push(normalizeShellCommand(currentLines.join("\n")));
  }
}

function verifyEvidenceLabels(texts: Map<TargetFile, string>, failures: string[]): void {
  const corpus = [
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/status/render/inbound.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/render/inbound.rs") ?? "",
  ].join("\n");

  for (const label of REQUIRED_LABELS) {
    requireNormalizedContains(corpus, label, "Phase 93 evidence label", failures);
  }
}

function verifyParityDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2pText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklistText = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(p2pText, SURFACE_ID, "Phase 93 parity catalog", failures);
  requireContains(checklistText, SURFACE_ID, "Phase 93 parity checklist", failures);
  for (const requirement of PHASE93_REQUIREMENTS) {
    requireContains(p2pText, requirement, "Phase 93 parity catalog", failures);
    requireContains(checklistText, requirement, "Phase 93 parity checklist", failures);
  }
  for (const anchor of REQUIRED_CATALOG_ANCHORS) {
    requireContains(p2pText, anchor, "Phase 93 parity catalog", failures);
  }
}

function verifySourceBreadcrumbs(text: string, failures: string[]): void {
  let parsed: BreadcrumbIndex;
  try {
    parsed = JSON.parse(text) as BreadcrumbIndex;
  } catch (error) {
    failures.push(`Phase 93 source breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }
  if (!Array.isArray(parsed.groups)) {
    failures.push("Phase 93 source breadcrumb groups must be an array");
    return;
  }
  const group = parsed.groups.find((entry) => {
    const maybeGroup = entry as BreadcrumbGroup;
    return maybeGroup.label === "network-peer-policy";
  }) as BreadcrumbGroup | undefined;
  if (group === undefined) {
    failures.push("Phase 93 source breadcrumb coverage missing network-peer-policy group");
    return;
  }
  for (const file of REQUIRED_BREADCRUMB_FILES) {
    requireArrayIncludes(group.files, "Phase 93 source breadcrumb files", file, failures);
  }
  for (const anchor of REQUIRED_CATALOG_ANCHORS) {
    requireArrayIncludes(group.breadcrumbs, "Phase 93 source breadcrumb anchors", anchor, failures);
  }
}

function executableVerifyText(text: string): string {
  return text.replace(/^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m, "");
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(/^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m);
  if (maybeOrderBlock === null) {
    failures.push("Phase 93 verifier-order missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [PHASE92_TEST_COMMAND, PHASE92_CHECKER_COMMAND, PHASE93_TEST_COMMAND, PHASE93_CHECKER_COMMAND],
      "Phase 93 verifier-order printed commands must follow Phase 92",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 93 peer policy checker" ${PHASE93_TEST_COMMAND}`,
    "Phase 93 verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 93 peer policy" ${PHASE93_CHECKER_COMMAND}`,
    "Phase 93 verifier-order",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE92_TEST_COMMAND,
      PHASE92_CHECKER_COMMAND,
      PHASE93_TEST_COMMAND,
      PHASE93_CHECKER_COMMAND,
      "bash scripts/check-pure-core-deps.sh",
    ],
    "Phase 93 verifier-order executed commands must follow Phase 92 and precede pure-core checks",
    failures,
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (containsForbiddenVerifyFragment(executableText, forbidden)) {
      failures.push(`Phase 93 default verifier boundary contains forbidden text: ${forbidden}`);
    }
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

function containsForbiddenVerifyFragment(text: string, fragment: string): boolean {
  if (/^[a-z-]+ $/.test(fragment)) {
    const command = fragment.trim().replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    return new RegExp(`(^|[\\s;&|()])${command}(?=\\s)`).test(text);
  }
  return text.includes(fragment);
}

function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }
    for (const unit of contextUnits(text)) {
      if (isScopedAllowance(unit)) {
        continue;
      }
      const lower = normalizedLower(unit);
      for (const claim of FORBIDDEN_POSITIVE_CLAIMS) {
        if (lower.includes(claim)) {
          failures.push(`Phase 93 no-claim boundary forbidden claim in ${file}: ${unit}`);
        }
      }
    }
  }
}

function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term));
}

function verifyRawEvidenceBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of RAW_EVIDENCE_FILES) {
    const text = texts.get(file) ?? "";
    for (const rawDetail of RAW_EVIDENCE_STRINGS) {
      if (text.includes(rawDetail)) {
        failures.push(`Phase 93 raw evidence boundary raw detail in ${file}: ${rawDetail}`);
      }
    }
  }
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

if (import.meta.main) {
  const failures = checkPhase93PeerPolicy();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 93 eviction, ban, and misbehavior policy evidence");
  }
}

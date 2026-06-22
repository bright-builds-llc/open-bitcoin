#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE84_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-upgrade-rollback-policy";
const PHASE84_REQUIREMENTS = ["UPG-01", "UPG-02", "UPG-03", "UPG-04"] as const;
const POLICY_PATH = "docs/parity/upgrade-and-rollback-policy.md";
const TABLE_HEADER = "Evidence to record | How to collect it | Mutation status | Why it matters";
const PHASE83_CHECKER_COMMAND =
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts";
const PHASE84_TEST_COMMAND =
  "bun test scripts/check-phase84-upgrade-rollback-policy.test.ts";
const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";
const TARGET_FILES = [
  POLICY_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/chainstate.md",
  "scripts/verify.sh",
] as const;
const HUMAN_POINTER_FILES = TARGET_FILES.filter(
  (file) => file !== "docs/parity/index.json" && file !== "scripts/verify.sh",
);
const REQUIRED_EVIDENCE = [
  POLICY_PATH,
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/chainstate.md",
  "scripts/verify.sh",
] as const;
const POLICY_HEADINGS = [
  "# Upgrade And Rollback Policy",
  "## Scope And Non-Claims",
  "## Pre-Upgrade Checklist",
  "## State And Schema Compatibility Decision Table",
  "## Evidence That Is Not Sufficient",
  "## Open Bitcoin Store Versus External State",
  "## Failed Upgrade Guidance",
  "## Rollback Guidance",
  "## Boundary And Deferred Work",
] as const;
const PRE_UPGRADE_ITEMS = [
  "current source revision or commit",
  "repo-local verification status",
  "binary provenance from Cargo or Bazel",
  "Open Bitcoin JSONC config path",
  "bitcoin.conf path",
  "selected datadir",
  "datadir ownership and free-space review",
  "current sync/status evidence",
  "support-bundle evidence when available",
  "service state",
  "wallet scope",
  "backup location",
] as const;
const RECOVERY_LABELS = [
  "clean_shutdown",
  "unclean_shutdown",
  "incompatible_schema",
  "store_corruption",
  "storage_lock_contention",
  "schema_mismatch",
  "corruption_marker",
  "corrupt_record",
  "partial_write",
  "unreadable_namespace",
  "backend_open_failure",
] as const;
const ACTION_CLASSES = [
  "safe_retry",
  "read_only_inspection",
  "backup_then_rebuild",
  "stop_and_escalate",
] as const;
const INSUFFICIENT_PROOF_SIGNALS = [
  "daemon startup",
  "elapsed time",
  "peer reachability",
  "raw logs",
  "report existence alone",
] as const;
const FAILED_UPGRADE_STEPS = [
  "stop the attempted upgraded process",
  "record exact command and commit",
  "collect redacted local evidence",
  "preserve backups",
  "avoid repeated mutation until the compatibility class is understood",
] as const;
const ROLLBACK_STEPS = [
  "return to the previous checked-out source revision or known binary",
  "use the same explicit datadir and config paths",
  "verify with repo-local commands",
  "record rollback evidence",
] as const;
const FORBIDDEN_MUTATION_PERMISSION_STRINGS = [
  "may silently rewrite source datadirs",
  "may mutate external wallets",
  "may rewrite launchd/systemd service files",
  "may edit bitcoin.conf",
  "destructive repair is allowed",
  "automated destructive repair is allowed",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-" + "smoke",
  "system" + "ctl",
  "launch" + "ctl",
  "sleep " + "259200",
] as const;

type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: Record<string, unknown>;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

export function checkPhase84UpgradeRollbackPolicy(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  const policy = texts.get(POLICY_PATH) ?? "";
  verifyPolicy(policy, failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanRoots(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function normalizeEvidenceText(text: string): string {
  return text
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replace(/\s+/g, " ")
    .trim();
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

function requireCaseInsensitiveContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.toLowerCase().includes(needle.toLowerCase())) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizeEvidenceText(text).includes(normalizeEvidenceText(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain ${needle}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root must be an array`);
    return;
  }
  if (!value.includes(required)) {
    failures.push(`${label} parity root missing required value: ${required}`);
  }
}

function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const expected = JSON.stringify(PHASE84_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
  }
}

function sectionBetween(text: string, heading: string): string {
  const startIndex = text.indexOf(heading);
  if (startIndex === -1) {
    return "";
  }

  const nextHeadingIndex = text.indexOf("\n## ", startIndex + heading.length);
  if (nextHeadingIndex === -1) {
    return text.slice(startIndex);
  }
  return text.slice(startIndex, nextHeadingIndex);
}

function verifyPolicy(text: string, failures: string[]): void {
  for (const heading of POLICY_HEADINGS) {
    requireContains(text, heading, "upgrade policy headings", failures);
  }

  verifyPreUpgradeChecklist(text, failures);
  verifyCompatibilityDecisionTable(text, failures);
  verifyInsufficientProofBoundary(text, failures);
  verifyRollbackAndMutationBoundaries(text, failures);
}

function verifyPreUpgradeChecklist(text: string, failures: string[]): void {
  const checklistSection = sectionBetween(text, "## Pre-Upgrade Checklist");
  if (checklistSection === "") {
    failures.push("pre-upgrade checklist missing required section");
    return;
  }

  requireNormalizedContains(checklistSection, TABLE_HEADER, "pre-upgrade checklist", failures);
  for (const item of PRE_UPGRADE_ITEMS) {
    requireContains(checklistSection, item, "pre-upgrade checklist", failures);
  }
  requireNormalizedContains(
    checklistSection,
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "pre-upgrade checklist",
    failures,
  );
  requireNormalizedContains(
    checklistSection,
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "pre-upgrade checklist",
    failures,
  );
  requireContains(checklistSection, "review-only evidence", "pre-upgrade checklist", failures);
}

function verifyCompatibilityDecisionTable(text: string, failures: string[]): void {
  const compatibilitySection = sectionBetween(
    text,
    "## State And Schema Compatibility Decision Table",
  );
  if (compatibilitySection === "") {
    failures.push("compatibility decision table missing required section");
    return;
  }

  for (const label of [...RECOVERY_LABELS, ...ACTION_CLASSES]) {
    requireContains(compatibilitySection, label, "compatibility decision table", failures);
  }
  requireNormalizedContains(
    compatibilitySection,
    "field-level evidence",
    "compatibility decision table",
    failures,
  );
  requireNormalizedContains(
    compatibilitySection,
    "Unavailable: <reason>",
    "compatibility decision table",
    failures,
  );
}

function verifyInsufficientProofBoundary(text: string, failures: string[]): void {
  const insufficientSection = sectionBetween(text, "## Evidence That Is Not Sufficient");
  if (insufficientSection === "") {
    failures.push("insufficient compatibility proof missing required section");
    return;
  }

  for (const signal of INSUFFICIENT_PROOF_SIGNALS) {
    requireContains(insufficientSection, signal, "insufficient compatibility proof", failures);
  }
  requireNormalizedContains(
    insufficientSection,
    "Compatibility decisions require field-level evidence and `Unavailable: <reason>`",
    "insufficient compatibility proof",
    failures,
  );

  const lowerText = text.toLowerCase();
  for (const signal of ["daemon startup", "elapsed time", "peer reachability", "raw logs", "report existence"]) {
    for (const proofPattern of [
      `${signal} as proof`,
      `${signal} is proof`,
      `${signal} proves`,
      `${signal} alone proves`,
    ]) {
      if (lowerText.includes(proofPattern)) {
        failures.push(
          `insufficient compatibility proof must not treat ${signal} as proof`,
        );
      }
    }
  }
}

function verifyRollbackAndMutationBoundaries(text: string, failures: string[]): void {
  for (const phrase of [
    "Open Bitcoin-owned durable store state",
    "external Core/Knots source datadirs and wallets",
  ]) {
    requireCaseInsensitiveContains(text, phrase, "hidden mutation boundary", failures);
  }
  for (const phrase of FAILED_UPGRADE_STEPS) {
    requireContains(text, phrase, "hidden mutation boundary", failures);
  }
  for (const phrase of ROLLBACK_STEPS) {
    requireContains(text, phrase, "hidden mutation boundary", failures);
  }
  for (const phrase of [
    "package-manager rollback",
    "signed release channels",
    "automatic update behavior",
    "Phase 84 does not recommend hidden mutation of source datadirs, external wallets, service files, launchd/systemd state, bitcoin.conf, or Open Bitcoin JSONC config.",
    "Destructive repair remains deferred.",
    "backup_then_rebuild is evidence and operator-decision guidance, not permission for automated destructive rebuild or repair.",
  ]) {
    requireContains(text, phrase, "hidden mutation boundary", failures);
  }

  const lowerText = text.toLowerCase();
  for (const forbidden of FORBIDDEN_MUTATION_PERMISSION_STRINGS) {
    if (lowerText.includes(forbidden)) {
      failures.push(`hidden mutation boundary must not permit: ${forbidden}`);
    }
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`parity root index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
  verifyAuditEntry(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("parity root surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`parity root surfaces missing done ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("parity root checklist.surfaces must be an array");
    return;
  }

  const surface = checklistSurfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`parity root checklist missing done ${SURFACE_ID}`);
  }
  requireExactRequirements(surface?.requirements, `${SURFACE_ID}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface?.evidence, `${SURFACE_ID}.evidence`, evidence, failures);
  }
}

function verifyAuditEntry(parsed: ParityIndex, failures: string[]): void {
  const auditEntry = parsed.audit?.v1_8_upgrade_rollback_policy as AuditEntry | undefined;
  if (auditEntry?.path !== "upgrade-and-rollback-policy.md" || auditEntry.status !== "done") {
    failures.push("parity root audit.v1_8_upgrade_rollback_policy is missing or incomplete");
    return;
  }
  requireExactRequirements(
    auditEntry.requirements,
    "audit.v1_8_upgrade_rollback_policy.requirements",
    failures,
  );
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(
      auditEntry.evidence,
      "audit.v1_8_upgrade_rollback_policy.evidence",
      evidence,
      failures,
    );
  }
}

function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
  for (const file of HUMAN_POINTER_FILES) {
    if (file !== POLICY_PATH) {
      requireContains(texts.get(file) ?? "", "upgrade-and-rollback-policy.md", file, failures);
    }
  }

  for (const file of TARGET_FILES) {
    if (file === POLICY_PATH) {
      continue;
    }
    requireNotContains(texts.get(file) ?? "", TABLE_HEADER, file, failures);
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE84_TEST_COMMAND, PHASE84_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  for (const command of [PHASE84_TEST_COMMAND, PHASE84_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase83Index = executableText.indexOf(PHASE83_CHECKER_COMMAND);
  const phase84TestIndex = executableText.indexOf(PHASE84_TEST_COMMAND);
  const phase84CheckerIndex = executableText.indexOf(PHASE84_CHECKER_COMMAND);
  const orderValid =
    phase83Index !== -1 &&
    phase84TestIndex > phase83Index &&
    phase84CheckerIndex > phase84TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 84 test and checker after Phase 83 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 84 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase84UpgradeRollbackPolicy();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 84 upgrade rollback policy");
  }
}

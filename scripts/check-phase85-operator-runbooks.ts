#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE85_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-operator-runbooks";
const AUDIT_KEY = "v1_8_operator_runbooks";
const PHASE85_REQUIREMENTS = ["RUN-01", "RUN-02", "RUN-03"] as const;
const RUNBOOK_PATH = "docs/parity/operator-runbooks.md";
const TABLE_HEADER = "Evidence to record | How to collect it | Mutation status | Escalation use";
const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";
const PHASE85_TEST_COMMAND =
  "bun test scripts/check-phase85-operator-runbooks.test.ts";
const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
const TARGET_FILES = [
  RUNBOOK_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const HUMAN_POINTER_FILES = TARGET_FILES.filter(
  (file) => file !== RUNBOOK_PATH && file !== "docs/parity/index.json" && file !== "scripts/verify.sh",
);
const REQUIRED_EVIDENCE = [
  RUNBOOK_PATH,
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const RUNBOOK_HEADINGS = [
  "# Operator Runbooks",
  "## Scope And Non-Claims",
  "## Production-Boundary Preflight",
  "## Long-Run Monitoring",
  "## No-Progress Diagnosis",
  "## Recovery And Stop Decisions",
  "## Escalation Evidence Thresholds",
  "## Support-Bundle Timeline",
  "## Privacy And Safety Boundaries",
] as const;
const SUPPORT_TERMS = [
  "supported",
  "preview",
  "opt-in UAT",
  "unsupported",
  "deferred",
] as const;
const PREFLIGHT_ITEMS = [
  "selected datadir",
  "source revision",
  "repo-local verification status",
  "Cargo or Bazel command form",
  "config paths",
  "current status evidence",
  "resource/disk review",
  "service state or unavailable reason",
  "wallet scope",
  "support-bundle availability",
] as const;
const STATUS_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> status --format json",
] as const;
const MONITORING_FIELDS = [
  "progress_credit",
  "last_useful_work",
  "last_peer_contribution",
  "expected_progress_window",
  "no_progress_threshold",
  "stall_diagnosis",
  "sync.no_progress_diagnosis",
  "sync.no_progress_next_action",
  "latest_stop_reason",
  "resource_bounds",
  "sync.resource_pressure",
  "recovery_evidence",
  "support_forensics",
] as const;
const STRUCTURED_MONITORING_TERMS = [
  "structured logs",
  "metrics",
  "support-bundle summaries",
  "soak reports",
  "live-smoke reports",
  "checkpoint timeline",
  "stalled subsystem",
  "public-network opt-in",
  "stay-current opt-in",
  "multi-day soak opt-in",
] as const;
const REQUIRED_INSUFFICIENT_SIGNALS = [
  "elapsed time",
  "daemon startup",
  "peer reachability",
  "raw log tail",
  "report existence",
  "support bundle existence",
] as const;
const PROOF_SIGNALS = [
  "artifact existence",
  "elapsed time",
  "daemon startup",
  "peer reachability",
  "raw logs",
  "raw log tail",
  "report existence",
  "support bundle existence",
] as const;
const ACTION_CLASSES = [
  "safe_retry",
  "read_only_inspection",
  "backup_then_rebuild",
  "stop_and_escalate",
] as const;
const ESCALATION_THRESHOLDS = [
  "repeated no-progress with typed cause",
  "unavailable critical fields",
  "recovery class requiring stop/escalate",
  "resource pressure crossing documented bounds",
  "inconsistent status/support evidence",
  "failure to collect the minimum redacted support-bundle timeline",
] as const;
const FORBIDDEN_BOUNDARY_TERMS = [
  "destructive repair",
  "source datadir mutation",
  "external wallet mutation",
  "service-manager mutation",
  "config rewrite",
  "automatic rebuild",
  "response timelines",
  "hosted support upload",
  "production service ownership",
] as const;
const FORBIDDEN_PERMISSION_STRINGS = [
  "destructive repair is allowed",
  "source datadir mutation is allowed",
  "external wallet mutation is allowed",
  "service-manager mutation is allowed",
  "config rewrite is allowed",
  "automatic rebuild is allowed",
  "automatic support-bundle upload is supported",
  "automatic support-bundle upload is allowed",
] as const;
const TIMELINE_LABELS = [
  "preflight evidence",
  "command start",
  "status snapshots",
  "progress or no-progress events",
  "resource/recovery events",
  "support-bundle collection",
  "operator action taken",
  "final status",
  "escalation decision",
] as const;
const MINIMUM_BUNDLE_ITEMS = [
  "support-evidence.json",
  "support-evidence.md",
  "exact command output",
  "bounded log summary",
  "config summary",
  "service state or unavailable reason",
  "resource evidence",
  "recovery/progress evidence",
  "sync evidence",
  "version/toolchain context",
  "platform details",
  "exact repo-local reproduction command",
  "Unavailable: <reason>",
] as const;
const SUPPORT_BUNDLE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> support bundle --output-dir=<path>/support --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> support bundle --output-dir=<path>/support --format json",
] as const;
const FORBIDDEN_EVIDENCE_ITEMS = [
  "wallet private material",
  "raw wallet files",
  "RPC cookies",
  "rpcpassword",
  "rpcauth",
  "raw datadirs",
  "unredacted logs",
  "raw unbounded logs",
  "full peer tables with sensitive local data",
  "automatic support-bundle upload",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
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

export function checkPhase85OperatorRunbooks(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE85_REPO_ROOT,
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyRunbook(texts.get(RUNBOOK_PATH) ?? "", failures);
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

function requireNotNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (normalizeEvidenceText(text).includes(normalizeEvidenceText(needle))) {
    failures.push(`${label} must not duplicate required text: ${needle}`);
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
  const expected = JSON.stringify(PHASE85_REQUIREMENTS);
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

function verifyRunbook(text: string, failures: string[]): void {
  for (const heading of RUNBOOK_HEADINGS) {
    requireContains(text, heading, "operator runbook headings", failures);
  }
  requireContains(text, SURFACE_ID, "operator runbook surface", failures);
  for (const term of SUPPORT_TERMS) {
    requireContains(text, term, "operator runbook support terms", failures);
  }

  verifyPreflight(text, failures);
  verifyMonitoring(text, failures);
  verifyInsufficientProofBoundary(text, failures);
  verifyRecoveryAndEscalation(text, failures);
  verifySupportTimelineAndPrivacy(text, failures);
}

function verifyPreflight(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Production-Boundary Preflight");
  if (section === "") {
    failures.push("preflight missing required section");
    return;
  }

  requireNormalizedContains(section, TABLE_HEADER, "preflight", failures);
  for (const item of PREFLIGHT_ITEMS) {
    requireContains(section, item, "preflight", failures);
  }
  for (const file of [
    "production-claim-boundary.md",
    "support-matrix.md",
    "upgrade-and-rollback-policy.md",
  ]) {
    requireContains(section, file, "preflight", failures);
  }
  for (const command of STATUS_COMMANDS) {
    requireNormalizedContains(section, command, "preflight", failures);
  }
  requireContains(section, "review-only evidence", "preflight", failures);
}

function verifyMonitoring(text: string, failures: string[]): void {
  const monitoringSection = sectionBetween(text, "## Long-Run Monitoring");
  if (monitoringSection === "") {
    failures.push("monitoring missing required section");
    return;
  }

  for (const field of MONITORING_FIELDS) {
    requireContains(monitoringSection, field, "monitoring", failures);
  }
  for (const term of STRUCTURED_MONITORING_TERMS) {
    requireContains(monitoringSection, term, "monitoring", failures);
  }
}

function verifyInsufficientProofBoundary(text: string, failures: string[]): void {
  const diagnosisSection = sectionBetween(text, "## No-Progress Diagnosis");
  if (diagnosisSection === "") {
    failures.push("insufficient proof missing required section");
    return;
  }

  for (const signal of REQUIRED_INSUFFICIENT_SIGNALS) {
    requireContains(diagnosisSection, signal, "insufficient proof", failures);
  }

  const lowerText = text.toLowerCase();
  for (const signal of PROOF_SIGNALS) {
    for (const proofPattern of [
      `${signal} as proof`,
      `${signal} is proof`,
      `${signal} proves`,
      `${signal} alone proves`,
    ]) {
      if (lowerText.includes(proofPattern)) {
        failures.push(`insufficient proof must not treat ${signal} as proof`);
      }
    }
  }
}

function verifyRecoveryAndEscalation(text: string, failures: string[]): void {
  const recoverySection = sectionBetween(text, "## Recovery And Stop Decisions");
  if (recoverySection === "") {
    failures.push("recovery decisions missing required section");
    return;
  }

  for (const actionClass of ACTION_CLASSES) {
    requireContains(recoverySection, actionClass, "recovery decisions", failures);
  }
  for (const term of FORBIDDEN_BOUNDARY_TERMS) {
    requireContains(text, term, "mutation boundary", failures);
  }
  requireContains(
    text,
    "Default bash scripts/verify.sh remains deterministic, public-network-free, service-manager-free, and multi-day-free.",
    "default verifier boundary",
    failures,
  );

  const escalationSection = sectionBetween(text, "## Escalation Evidence Thresholds");
  if (escalationSection === "") {
    failures.push("escalation thresholds missing required section");
    return;
  }
  for (const threshold of ESCALATION_THRESHOLDS) {
    requireContains(escalationSection, threshold, "escalation thresholds", failures);
  }

  const lowerText = text.toLowerCase();
  for (const forbidden of FORBIDDEN_PERMISSION_STRINGS) {
    if (lowerText.includes(forbidden)) {
      failures.push(`mutation boundary must not permit: ${forbidden}`);
    }
  }
}

function verifySupportTimelineAndPrivacy(text: string, failures: string[]): void {
  const timelineSection = sectionBetween(text, "## Support-Bundle Timeline");
  if (timelineSection === "") {
    failures.push("support-bundle timeline missing required section");
    return;
  }

  for (const label of TIMELINE_LABELS) {
    requireContains(timelineSection, label, "support-bundle timeline", failures);
  }
  for (const item of MINIMUM_BUNDLE_ITEMS) {
    requireNormalizedContains(timelineSection, item, "support-bundle timeline", failures);
  }
  for (const command of SUPPORT_BUNDLE_COMMANDS) {
    requireNormalizedContains(timelineSection, command, "support-bundle timeline", failures);
  }

  const privacySection = sectionBetween(text, "## Privacy And Safety Boundaries");
  if (privacySection === "") {
    failures.push("support-bundle privacy missing required section");
    return;
  }
  for (const item of FORBIDDEN_EVIDENCE_ITEMS) {
    requireContains(privacySection, item, "support-bundle privacy", failures);
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
  const auditEntry = parsed.audit?.[AUDIT_KEY] as AuditEntry | undefined;
  if (auditEntry?.path !== "operator-runbooks.md" || auditEntry.status !== "done") {
    failures.push(`parity root audit.${AUDIT_KEY} is missing or incomplete`);
    return;
  }
  requireExactRequirements(auditEntry.requirements, `audit.${AUDIT_KEY}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(auditEntry.evidence, `audit.${AUDIT_KEY}.evidence`, evidence, failures);
  }
}

function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
  for (const file of HUMAN_POINTER_FILES) {
    requireContains(texts.get(file) ?? "", "operator-runbooks.md", file, failures);
    requireNotNormalizedContains(texts.get(file) ?? "", TABLE_HEADER, file, failures);
  }

  requireContains(texts.get("README.md") ?? "", "docs/parity/operator-runbooks.md", "README.md", failures);
  requireContains(
    texts.get("docs/operator/runtime-guide.md") ?? "",
    "../parity/operator-runbooks.md",
    "docs/operator/runtime-guide.md",
    failures,
  );

  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  requireContains(releaseReadiness, SURFACE_ID, "docs/parity/release-readiness.md", failures);
  for (const requirement of PHASE85_REQUIREMENTS) {
    requireContains(releaseReadiness, requirement, "docs/parity/release-readiness.md", failures);
  }

  const catalog = texts.get("docs/parity/catalog/operator-runtime-release-hardening.md") ?? "";
  for (const phrase of [
    "Phase 85 operator runbooks",
    SURFACE_ID,
    "RUN-01",
    "RUN-02",
    "RUN-03",
    "public-network default checks",
    "real service-manager",
    "multi-day default",
    "automatic support-bundle upload",
    "destructive repair",
    "broad production-node readiness",
  ]) {
    requireContains(catalog, phrase, "docs/parity/catalog/operator-runtime-release-hardening.md", failures);
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE85_TEST_COMMAND, PHASE85_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  for (const command of [PHASE85_TEST_COMMAND, PHASE85_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase84Index = executableText.indexOf(PHASE84_CHECKER_COMMAND);
  const phase85TestIndex = executableText.indexOf(PHASE85_TEST_COMMAND);
  const phase85CheckerIndex = executableText.indexOf(PHASE85_CHECKER_COMMAND);
  const orderValid =
    phase84Index !== -1 &&
    phase85TestIndex > phase84Index &&
    phase85CheckerIndex > phase85TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 85 test and checker after Phase 84 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 85 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase85OperatorRunbooks();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 85 operator runbooks");
  }
}

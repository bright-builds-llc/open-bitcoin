import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE85_REQUIREMENTS, RUNBOOK_PATH, TABLE_HEADER, PHASE84_CHECKER_COMMAND, PHASE85_TEST_COMMAND, PHASE85_CHECKER_COMMAND, TARGET_FILES, HUMAN_POINTER_FILES, REQUIRED_EVIDENCE, RUNBOOK_HEADINGS, SUPPORT_TERMS, PREFLIGHT_ITEMS, STATUS_COMMANDS, MONITORING_FIELDS, STRUCTURED_MONITORING_TERMS, REQUIRED_INSUFFICIENT_SIGNALS, PROOF_SIGNALS, ACTION_CLASSES, ESCALATION_THRESHOLDS, FORBIDDEN_BOUNDARY_TERMS, FORBIDDEN_PERMISSION_STRINGS, TIMELINE_LABELS, MINIMUM_BUNDLE_ITEMS, SUPPORT_BUNDLE_COMMANDS, FORBIDDEN_EVIDENCE_ITEMS, FORBIDDEN_VERIFY_STRINGS } from "./constants.ts";
import type { AuditEntry, ChecklistSurface, ParityIndex, ParitySurface } from "./constants.ts";
import { verifyHumanRoots, executableVerifyText, verifyVerifierWiring } from "./parity.ts";
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

export function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

export function normalizeEvidenceText(text: string): string {
  return text
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replace(/\s+/g, " ")
    .trim();
}

export function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

export function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizeEvidenceText(text).includes(normalizeEvidenceText(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

export function requireNotNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (normalizeEvidenceText(text).includes(normalizeEvidenceText(needle))) {
    failures.push(`${label} must not duplicate required text: ${needle}`);
  }
}

export function requireArrayIncludes(
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

export function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
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

export function sectionBetween(text: string, heading: string): string {
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

export function verifyRunbook(text: string, failures: string[]): void {
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

export function verifyPreflight(text: string, failures: string[]): void {
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

export function verifyMonitoring(text: string, failures: string[]): void {
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

export function verifyInsufficientProofBoundary(text: string, failures: string[]): void {
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

export function verifyRecoveryAndEscalation(text: string, failures: string[]): void {
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

export function verifySupportTimelineAndPrivacy(text: string, failures: string[]): void {
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

export function verifyParityIndex(text: string, failures: string[]): void {
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

export function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
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

export function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
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

export function verifyAuditEntry(parsed: ParityIndex, failures: string[]): void {
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

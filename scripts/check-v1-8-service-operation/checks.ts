import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE86_REQUIREMENTS, SERVICE_DOC_PATH, TABLE_HEADER, PHASE85_CHECKER_COMMAND, PHASE86_TEST_COMMAND, PHASE86_CHECKER_COMMAND, TARGET_FILES, HUMAN_POINTER_FILES, REQUIRED_EVIDENCE, REQUIRED_HEADINGS, SUPPORT_TERMS, SERVICE_SURFACES, SERVICE_COMMANDS, FIELD_EVIDENCE_TERMS, SERVICE_FIELDS, LIFECYCLE_LABELS, RESTART_RESUME_FIELDS, PROOF_SIGNALS, SENSITIVE_EVIDENCE_TERMS, FORBIDDEN_DOC_PERMISSION_STRINGS, FORBIDDEN_VERIFY_STRINGS } from "./constants.ts";
import type { AuditEntry, ChecklistSurface, ParityIndex, ParitySurface } from "./constants.ts";
import { verifyHumanRoots, executableVerifyText, verifyVerifierWiring } from "./parity.ts";
export function checkPhase86ServiceOperationExpectations(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE86_REPO_ROOT,
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyServiceDoc(texts.get(SERVICE_DOC_PATH) ?? "", failures);
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
  const expected = JSON.stringify(PHASE86_REQUIREMENTS);
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

export function verifyServiceDoc(text: string, failures: string[]): void {
  for (const heading of REQUIRED_HEADINGS) {
    requireContains(text, heading, "service expectations headings", failures);
  }
  requireContains(text, SURFACE_ID, "service expectations surface", failures);
  for (const term of SUPPORT_TERMS) {
    requireContains(text, term, "support terms", failures);
  }
  requireNormalizedContains(
    text,
    "generated launchd/systemd definitions supervise `open-bitcoind`, not the `open-bitcoin` operator wrapper.",
    "service expectations scope",
    failures,
  );
  requireContains(
    text,
    "`service preview` is always side-effect-free.",
    "service expectations scope",
    failures,
  );
  requireNormalizedContains(
    text,
    "`service install` and `service uninstall` are previews unless `--apply` is supplied.",
    "service expectations scope",
    failures,
  );

  verifyClassification(text, failures);
  verifyCommandEvidence(text, failures);
  verifyFieldEvidence(text, failures);
  verifyRestartResume(text, failures);
  verifyDefaultBoundary(text, failures);
  verifySensitiveEvidence(text, failures);
  verifyNoProofPromotion(text, failures);
}

export function verifyClassification(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Service Surface Classification");
  if (section === "") {
    failures.push("service classification missing required section");
    return;
  }

  requireNormalizedContains(section, TABLE_HEADER, "service classification", failures);
  for (const surface of SERVICE_SURFACES) {
    requireContains(section, surface, "service classification", failures);
  }
}

export function verifyCommandEvidence(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Repo-Local Command Evidence");
  if (section === "") {
    failures.push("command evidence missing required section");
    return;
  }

  for (const command of SERVICE_COMMANDS) {
    requireNormalizedContains(section, command, "command evidence", failures);
  }
}

export function verifyFieldEvidence(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Field-Based Evidence Rules");
  if (section === "") {
    failures.push("field-based evidence missing required section");
    return;
  }

  for (const term of FIELD_EVIDENCE_TERMS) {
    requireNormalizedContains(section, term, "field-based evidence", failures);
  }
  for (const field of SERVICE_FIELDS) {
    requireContains(section, field, "field-based evidence", failures);
  }
  for (const label of LIFECYCLE_LABELS) {
    requireContains(section, label, "lifecycle labels", failures);
  }
}

export function verifyRestartResume(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Restart Resume Evidence");
  if (section === "") {
    failures.push("restart/resume evidence missing required section");
    return;
  }

  for (const field of RESTART_RESUME_FIELDS) {
    requireContains(section, field, "restart/resume evidence", failures);
  }
  requireContains(section, "same selected datadir", "restart/resume evidence", failures);
  requireContains(section, "do not prove durable resume", "restart/resume evidence", failures);
}

export function verifyDefaultBoundary(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Default Verification And Opt-In UAT Boundaries");
  if (section === "") {
    failures.push("default verifier boundary missing required section");
    return;
  }

  requireContains(
    section,
    "Default bash scripts/verify.sh remains deterministic, public-network-free, real-service-manager-free, and multi-day-free.",
    "default verifier boundary",
    failures,
  );
  for (const phrase of [
    "public-network live smoke",
    "real service-manager commands",
    "long wall-clock sleeps",
    "package-manager service commands",
    "Windows service workflows",
    "automatic support-bundle upload",
    "production service ownership checks",
    "broad production-node readiness checks",
    "opt-in UAT",
  ]) {
    requireNormalizedContains(section, phrase, "default verifier boundary", failures);
  }
}

export function verifySensitiveEvidence(text: string, failures: string[]): void {
  const section = sectionBetween(text, "## Sensitive Evidence Boundaries");
  if (section === "") {
    failures.push("sensitive evidence missing required section");
    return;
  }

  for (const term of SENSITIVE_EVIDENCE_TERMS) {
    requireContains(section, term, "sensitive evidence", failures);
  }
}

export function verifyNoProofPromotion(text: string, failures: string[]): void {
  const lowerText = normalizeEvidenceText(text).toLowerCase();
  for (const signal of PROOF_SIGNALS) {
    for (const proofPattern of [
      `${signal} as proof`,
      `${signal} is proof`,
      `${signal} proves`,
      `${signal} alone proves`,
    ]) {
      if (lowerText.includes(proofPattern)) {
        failures.push(`field-based evidence must not treat ${signal} as proof`);
      }
    }
  }

  for (const forbidden of FORBIDDEN_DOC_PERMISSION_STRINGS) {
    if (lowerText.includes(forbidden.toLowerCase())) {
      failures.push(`sensitive evidence or default boundary must not permit: ${forbidden}`);
    }
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
  if (auditEntry?.path !== "service-operation-expectations.md" || auditEntry.status !== "done") {
    failures.push(`parity root audit.${AUDIT_KEY} is missing or incomplete`);
    return;
  }
  requireExactRequirements(auditEntry.requirements, `audit.${AUDIT_KEY}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(auditEntry.evidence, `audit.${AUDIT_KEY}.evidence`, evidence, failures);
  }
}

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE85_REQUIREMENTS, RUNBOOK_PATH, TABLE_HEADER, PHASE84_CHECKER_COMMAND, PHASE85_TEST_COMMAND, PHASE85_CHECKER_COMMAND, TARGET_FILES, HUMAN_POINTER_FILES, REQUIRED_EVIDENCE, RUNBOOK_HEADINGS, SUPPORT_TERMS, PREFLIGHT_ITEMS, STATUS_COMMANDS, MONITORING_FIELDS, STRUCTURED_MONITORING_TERMS, REQUIRED_INSUFFICIENT_SIGNALS, PROOF_SIGNALS, ACTION_CLASSES, ESCALATION_THRESHOLDS, FORBIDDEN_BOUNDARY_TERMS, FORBIDDEN_PERMISSION_STRINGS, TIMELINE_LABELS, MINIMUM_BUNDLE_ITEMS, SUPPORT_BUNDLE_COMMANDS, FORBIDDEN_EVIDENCE_ITEMS, FORBIDDEN_VERIFY_STRINGS } from "./constants.ts";
import type { AuditEntry, ChecklistSurface, ParityIndex, ParitySurface } from "./constants.ts";
import { checkPhase85OperatorRunbooks, readText, normalizeEvidenceText, requireContains, requireNormalizedContains, requireNotNormalizedContains, requireArrayIncludes, requireExactRequirements, sectionBetween, verifyRunbook, verifyPreflight, verifyMonitoring, verifyInsufficientProofBoundary, verifyRecoveryAndEscalation, verifySupportTimelineAndPrivacy, verifyParityIndex, verifyTopLevelSurface, verifyChecklistSurface, verifyAuditEntry } from "./checks.ts";
export function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
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

export function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

export function verifyVerifierWiring(text: string, failures: string[]): void {
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

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE86_REQUIREMENTS, SERVICE_DOC_PATH, TABLE_HEADER, PHASE85_CHECKER_COMMAND, PHASE86_TEST_COMMAND, PHASE86_CHECKER_COMMAND, TARGET_FILES, HUMAN_POINTER_FILES, REQUIRED_EVIDENCE, REQUIRED_HEADINGS, SUPPORT_TERMS, SERVICE_SURFACES, SERVICE_COMMANDS, FIELD_EVIDENCE_TERMS, SERVICE_FIELDS, LIFECYCLE_LABELS, RESTART_RESUME_FIELDS, PROOF_SIGNALS, SENSITIVE_EVIDENCE_TERMS, FORBIDDEN_DOC_PERMISSION_STRINGS, FORBIDDEN_VERIFY_STRINGS } from "./constants.ts";
import type { AuditEntry, ChecklistSurface, ParityIndex, ParitySurface } from "./constants.ts";
import { checkPhase86ServiceOperationExpectations, readText, normalizeEvidenceText, requireContains, requireNormalizedContains, requireNotNormalizedContains, requireArrayIncludes, requireExactRequirements, sectionBetween, verifyServiceDoc, verifyClassification, verifyCommandEvidence, verifyFieldEvidence, verifyRestartResume, verifyDefaultBoundary, verifySensitiveEvidence, verifyNoProofPromotion, verifyParityIndex, verifyTopLevelSurface, verifyChecklistSurface, verifyAuditEntry } from "./checks.ts";
export function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
  for (const file of HUMAN_POINTER_FILES) {
    requireContains(texts.get(file) ?? "", "service-operation-expectations.md", file, failures);
    requireNotNormalizedContains(texts.get(file) ?? "", TABLE_HEADER, file, failures);
  }

  requireContains(
    texts.get("README.md") ?? "",
    "docs/parity/service-operation-expectations.md",
    "README.md",
    failures,
  );
  requireContains(
    texts.get("docs/operator/runtime-guide.md") ?? "",
    "../parity/service-operation-expectations.md",
    "docs/operator/runtime-guide.md",
    failures,
  );

  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  for (const phrase of [
    SURFACE_ID,
    "SVC-01",
    "SVC-02",
    "source-built daemon operation",
    "launchd/systemd preview",
    "opt-in real service lifecycle UAT",
    "restart/resume fields",
    "repo-local Cargo/Bazel commands",
    "production-service non-claims",
  ]) {
    requireContains(releaseReadiness, phrase, "docs/parity/release-readiness.md", failures);
  }

  const catalog = texts.get("docs/parity/catalog/operator-runtime-release-hardening.md") ?? "";
  for (const phrase of [
    "Phase 86 service operation expectations",
    SURFACE_ID,
    "SVC-01",
    "SVC-02",
    "source-built daemon operation",
    "launchd/systemd preview",
    "opt-in real service lifecycle UAT",
    "restart/resume fields",
    "repo-local Cargo/Bazel commands",
    "production-service non-claims",
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
  for (const command of [PHASE86_TEST_COMMAND, PHASE86_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  for (const command of [PHASE86_TEST_COMMAND, PHASE86_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase85Index = executableText.indexOf(PHASE85_CHECKER_COMMAND);
  const phase86TestIndex = executableText.indexOf(PHASE86_TEST_COMMAND);
  const phase86CheckerIndex = executableText.indexOf(PHASE86_CHECKER_COMMAND);
  const orderValid =
    phase85Index !== -1 &&
    phase86TestIndex > phase85Index &&
    phase86CheckerIndex > phase86TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 86 test and checker after Phase 85 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 86 default command text: ${forbidden}`);
    }
  }
}

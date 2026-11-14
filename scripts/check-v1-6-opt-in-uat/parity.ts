import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, maybeRepoRoot, REPO_ROOT, PHASE_DIR, PHASE73_SURFACE_ID, PHASE73_REGRESSION_TEST_COMMAND, PHASE73_CHECKER_COMMAND, PLAN_FILES, REQUIREMENT_IDS, REQUIRED_VER02_BEHAVIORS, HERMETIC_COVERAGE_FILES, PARITY_CLOSEOUT_FILES, REQUIRED_UAT_MATRIX_DOC_STRINGS, REQUIRED_PARITY_ROOT_STRINGS, REQUIRED_CLOSEOUT_FILES, REQUIRED_BREADCRUMB_FILES, REQUIRED_DEFERRED_SCOPE_STRINGS, FORBIDDEN_PHASE73_CLAIM_STRINGS, FORBIDDEN_VERIFY_STRINGS, VER02_COVERAGE } from "./constants.ts";
import type { Ver02Behavior, CoverageAnchor, CoverageEntry, SourceBreadcrumbFileGroup, SourceBreadcrumbs, ParityIndex, ParityChecklist } from "./constants.ts";
import { repoPath, readText, readJoined, requireContains, requireNotContains, requireFileExists, isRecord, truncateProcessOutput, verifyCoverageBehaviors, verifyRequirements, verifyCoverageAnchors, verifyCoverageMap, verifyHermeticCoverageFiles, verifyRequirementIds, verifyParityIndexRequirements, verifyChecklistRequirements, verifyParityLedgerRequirements, verifyUatMatrixDocs, verifyVerifyScript, verifyParityRootText, verifyDeferredScopeNonClaims, verifyCloseoutFilesExist, verifySourceBreadcrumbRegistry, verifyParityBreadcrumbChecker } from "./checks.ts";
export async function verifyParityAndEvidenceCloseout(failures: string[]): Promise<void> {
  await verifyParityLedgerRequirements(failures);
  await verifyParityRootText(failures);
  await verifyDeferredScopeNonClaims(failures);
  await verifyCloseoutFilesExist(failures);
  await verifySourceBreadcrumbRegistry(failures);
  verifyParityBreadcrumbChecker(failures);
}

export async function main(): Promise<void> {
  const failures: string[] = [];
  await verifyRequirements(failures);
  await verifyCoverageMap(failures);
  await verifyUatMatrixDocs(failures);
  await verifyVerifyScript(failures);
  await verifyParityAndEvidenceCloseout(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 73 opt-in UAT and deterministic verification evidence");
}

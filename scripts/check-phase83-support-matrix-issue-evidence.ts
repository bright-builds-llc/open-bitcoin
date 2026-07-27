#!/usr/bin/env bun
import { checkPhase83SupportMatrixIssueEvidence } from "./check-phase83-support-matrix-issue-evidence/checks.ts";
export { checkPhase83SupportMatrixIssueEvidence };
if (import.meta.main) {
  const failures = checkPhase83SupportMatrixIssueEvidence();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 83 support matrix issue evidence");
  }
}

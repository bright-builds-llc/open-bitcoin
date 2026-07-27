#!/usr/bin/env bun

import { checkPhase132TypedPackageStagedAdmission } from "./check-phase132-typed-package-staged-admission/checks.ts";
export { checkPhase132TypedPackageStagedAdmission } from "./check-phase132-typed-package-staged-admission/checks.ts";
export { PHASE132_TARGET_FILES } from "./check-phase132-typed-package-staged-admission/constants.ts";

if (import.meta.main) {
  const failures = checkPhase132TypedPackageStagedAdmission();
  if (failures.length > 0) {
    for (const failure of failures) console.error(failure);
    process.exit(1);
  }
  console.log(
    "Phase 132 typed package staged admission checks passed: PACK-01 PACK-02 PACK-03 PACK-04 PACK-05 PACK-06 PACK-07.",
  );
}

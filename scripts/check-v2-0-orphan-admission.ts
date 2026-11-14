#!/usr/bin/env bun

import { checkPhase102OrphanAdmissionBridge } from "./check-v2-0-orphan-admission/checks.ts";
export { checkPhase102OrphanAdmissionBridge } from "./check-v2-0-orphan-admission/checks.ts";

if (import.meta.main) {
  const failures = checkPhase102OrphanAdmissionBridge();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }
  console.log("validated Phase 102 orphan handling admission outcome bridge evidence");
}

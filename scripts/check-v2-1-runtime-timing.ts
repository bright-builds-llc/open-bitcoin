#!/usr/bin/env bun

import { checkPhase123RuntimeTimingEvidenceIntegrity } from "./check-v2-1-runtime-timing/checks.ts";
export { checkPhase123RuntimeTimingEvidenceIntegrity } from "./check-v2-1-runtime-timing/checks.ts";

if (import.meta.main) {
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity();
  if (failures.length > 0) {
    console.error("Phase 123 runtime timing and evidence integrity checker failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 123 runtime timing and evidence integrity checker passed.");
}

#!/usr/bin/env bun
import { checkPhase95NetworkParticipationReleaseBoundary } from "./check-phase95-network-participation-release-boundary/checks.ts";
export { checkPhase95NetworkParticipationReleaseBoundary };
if (import.meta.main) {
  const failures = checkPhase95NetworkParticipationReleaseBoundary();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 95 network participation release boundary");
  }
}

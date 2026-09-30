#!/usr/bin/env bun

export { checkPhase151ParityUatReleaseBoundary } from "./check-phase151-parity-uat-release-boundary/checks.ts";
import { checkPhase151ParityUatReleaseBoundary } from "./check-phase151-parity-uat-release-boundary/checks.ts";

if (import.meta.main) {
  const failures = checkPhase151ParityUatReleaseBoundary();
  if (failures.length > 0) {
    console.error("Phase 151 parity UAT release boundary check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 151 parity UAT release boundary validated.");
}

#!/usr/bin/env bun

export { checkPhase138ParityUatReleaseBoundary } from "./check-phase138-parity-uat-release-boundary/checks.ts";
import { checkPhase138ParityUatReleaseBoundary } from "./check-phase138-parity-uat-release-boundary/checks.ts";

if (import.meta.main) {
  const failures = checkPhase138ParityUatReleaseBoundary();
  if (failures.length > 0) {
    console.error("Phase 138 parity UAT release boundary check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 138 parity UAT release boundary validated.");
}

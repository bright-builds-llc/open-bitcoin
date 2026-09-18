#!/usr/bin/env bun

export { checkPhase145ParityUatReleaseBoundary } from "./check-phase145-parity-uat-release-boundary/checks.ts";
import { checkPhase145ParityUatReleaseBoundary } from "./check-phase145-parity-uat-release-boundary/checks.ts";

if (import.meta.main) {
  const failures = checkPhase145ParityUatReleaseBoundary();
  if (failures.length > 0) {
    console.error("Phase 145 parity UAT release boundary check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 145 parity UAT release boundary validated.");
}

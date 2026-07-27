#!/usr/bin/env bun

import { checkActiveMilestoneVerificationTraceability } from "./check-active-milestone-verification-traceability/checks.ts";
export { checkActiveMilestoneVerificationTraceability } from "./check-active-milestone-verification-traceability/checks.ts";
export type { CheckActiveMilestoneVerificationTraceabilityOptions } from "./check-active-milestone-verification-traceability/constants.ts";

if (import.meta.main) {
  const failures = checkActiveMilestoneVerificationTraceability();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`verification-traceability: ${failure}`);
    }
    process.exit(1);
  }
  console.log("Active milestone verification traceability checker passed.");
}

#!/usr/bin/env bun
import { checkPhase86ServiceOperationExpectations } from "./check-v1-8-service-operation/checks.ts";
export { checkPhase86ServiceOperationExpectations };
if (import.meta.main) {
  const failures = checkPhase86ServiceOperationExpectations();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 86 service operation expectations");
  }
}

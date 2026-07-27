#!/usr/bin/env bun
import { checkPhase86ServiceOperationExpectations } from "./check-phase86-service-operation-expectations/checks.ts";
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

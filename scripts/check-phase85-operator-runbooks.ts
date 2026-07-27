#!/usr/bin/env bun
import { checkPhase85OperatorRunbooks } from "./check-phase85-operator-runbooks/checks.ts";
export { checkPhase85OperatorRunbooks };
if (import.meta.main) {
  const failures = checkPhase85OperatorRunbooks();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 85 operator runbooks");
  }
}

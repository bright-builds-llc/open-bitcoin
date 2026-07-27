#!/usr/bin/env bun
import { checkPhase90InboundListenerAdmission } from "./check-phase90-inbound-listener-admission/checks.ts";
export { checkPhase90InboundListenerAdmission };
if (import.meta.main) {
  const failures = checkPhase90InboundListenerAdmission();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 90 inbound listener admission evidence");
  }
}

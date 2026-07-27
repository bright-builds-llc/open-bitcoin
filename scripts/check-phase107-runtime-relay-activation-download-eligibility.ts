#!/usr/bin/env bun

import { checkPhase107RuntimeRelayActivationDownloadEligibility } from "./check-phase107-runtime-relay-activation-download-eligibility/checks.ts";
export { checkPhase107RuntimeRelayActivationDownloadEligibility } from "./check-phase107-runtime-relay-activation-download-eligibility/checks.ts";

if (import.meta.main) {
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility();
  if (failures.length > 0) {
    console.error("Phase 107 runtime relay activation/download eligibility check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log("Phase 107 runtime relay activation/download eligibility validated.");
}

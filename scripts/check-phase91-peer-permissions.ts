#!/usr/bin/env bun
import { checkPhase91PeerPermissions } from "./check-phase91-peer-permissions/checks.ts";
export { checkPhase91PeerPermissions };
if (import.meta.main) {
  const failures = checkPhase91PeerPermissions();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 91 peer permissions evidence");
  }
}

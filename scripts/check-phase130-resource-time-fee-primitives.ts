#!/usr/bin/env bun

import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

export const PHASE130_TARGET_FILES = [
  "README.md",
  "packages/README.md",
  "docs/parity/README.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/resource.rs",
  "packages/open-bitcoin-mempool/src/fee.rs",
  "packages/open-bitcoin-mempool/src/context.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "scripts/check-phase130-resource-time-fee-primitives.ts",
  "scripts/verify.sh",
] as const;

export function checkPhase130ResourceTimeFeePrimitives(
  maybeRepoRoot?: string,
): string[] {
  void maybeRepoRoot;
  void DEFAULT_REPO_ROOT;
  return ["P130 not implemented"];
}

if (import.meta.main) {
  const failures = checkPhase130ResourceTimeFeePrimitives();
  if (failures.length > 0) {
    console.error("Phase 130 resource time and fee primitives check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 130 resource time and fee primitives validated.");
}

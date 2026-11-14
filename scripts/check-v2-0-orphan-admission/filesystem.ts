import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { TargetFile } from "./constants.ts";

const SPLIT_CHILDREN: Partial<Record<TargetFile, readonly string[]>> = {
  "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs": [
    "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases/missing_parent_outcome_collects_unique_parent_txids.rs",
    "packages/open-bitcoin-mempool/src/pool/tests/outcome_cases/outcome_labels_are_fixed_low_cardinality_values.rs",
  ],
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs": [
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/boundedness_cases.rs",
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/candidate_cases.rs",
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/lifecycle_cases.rs",
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/policy_cases.rs",
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/provenance_cases.rs",
  ],
  "packages/open-bitcoin-network/src/peer/tests.rs": [
    "packages/open-bitcoin-network/src/peer/tests/transaction_relay_orphan_lifecycle_cases.rs",
  ],
  "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs": [
    "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases/orphan_reconsideration.rs",
    "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases/peer_admission.rs",
    "packages/open-bitcoin-node/src/network/tests/admission_bridge_cases/replacement_cleanup.rs",
  ],
};

export function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`Phase 102 missing required corpus file: ${relativePath}`);
    return "";
  }

  const childText = (SPLIT_CHILDREN[relativePath] ?? [])
    .map((childPath) => path.join(repoRoot, childPath))
    .filter(existsSync)
    .map((childPath) => readFileSync(childPath, "utf8"))
    .join("\n");
  return `${readFileSync(absolutePath, "utf8")}\n${childText}`;
}

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const SPLIT_CHILDREN = new Map<string, readonly string[]>([
  [
    "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
    [
      "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases/dry_run_submit_valid_parent_invalid_child_partial_acceptance_and_lifecyc.rs",
      "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases/max_bound_shape_fingerprint_order_and_try_from_package_refinement_are_pi.rs",
    ],
  ],
]);

export function readTarget(repoRoot: string, relativePath: string): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) return "";
  const texts = [readFileSync(absolutePath, "utf8")];
  for (const child of SPLIT_CHILDREN.get(relativePath) ?? []) {
    const childPath = path.join(repoRoot, child);
    if (existsSync(childPath)) {
      texts.push(readFileSync(childPath, "utf8"));
    }
  }
  return texts.join("\n");
}

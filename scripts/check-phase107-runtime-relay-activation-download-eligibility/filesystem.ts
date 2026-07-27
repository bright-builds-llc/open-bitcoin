import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { TargetFile } from "./constants.ts";

const SPLIT_CHILDREN = new Map<TargetFile, readonly string[]>([
  [
    "packages/open-bitcoin-cli/src/operator/support/tests.rs",
    [
      "packages/open-bitcoin-cli/src/operator/support/tests/forensics_recovery_relay.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests/inbound.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests/inbound_status_fixtures.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests/soak_forensics_fixtures.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs",
      "packages/open-bitcoin-cli/src/operator/support/tests/sync_soak_forensics.rs",
    ],
  ],
]);

export function readText(repoRoot: string, filePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, filePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing target file ${filePath}`);
    return "";
  }

  const texts = [readFileSync(absolutePath, "utf8")];
  for (const child of SPLIT_CHILDREN.get(filePath) ?? []) {
    const childPath = path.join(repoRoot, child);
    if (existsSync(childPath)) {
      texts.push(readFileSync(childPath, "utf8"));
    }
  }
  return texts.join("\n");
}

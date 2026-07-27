import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { DEFAULT_REPO_ROOT, TARGET_FILES, TargetFile } from "./constants.ts";
import { readText } from "./filesystem.ts";
import { verifyParityIndex, verifyParityDocs, verifySourceBreadcrumbs } from "./parity.ts";
import { verifyOutcomeAndOrphanEvidence, verifyManagedBridgeEvidence, verifyBehaviorTests } from "./bridge.ts";
import { verifyVerifierWiring, verifyNoClaimBoundary } from "./verifier.ts";

export function checkPhase102OrphanAdmissionBridge(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE102_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyParityDocs(texts, failures);
  verifyOutcomeAndOrphanEvidence(texts, failures);
  verifyManagedBridgeEvidence(texts, failures);
  verifyBehaviorTests(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);

  return failures;
}

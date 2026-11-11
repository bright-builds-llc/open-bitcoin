import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { FORBIDDEN_CLAIMS, POSITIVE_CLAIM_PATTERNS, TextCorpus } from "./constants.ts";
import { hasNoClaimMarker } from "./helpers.ts";

export function verifyNoClaimBoundary(texts: TextCorpus, failures: string[]): void {
  const docs = [
    texts.get("docs/parity/catalog/p2p.md") ?? "",
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/index.json") ?? "",
  ].join("\n");

  const units = docs.replace(/\s+/g, " ").split(/(?<=[.!?])\s+/);
  for (const unit of units) {
    const lower = unit.toLowerCase();
    for (const claim of FORBIDDEN_CLAIMS) {
      if (!lower.includes(claim)) {
        continue;
      }
      if (hasNoClaimMarker(lower)) {
        continue;
      }
      if (POSITIVE_CLAIM_PATTERNS.some((pattern) => pattern.test(lower))) {
        failures.push(`no-claim boundary violation for ${claim}: ${unit.trim()}`);
      }
    }
  }
}

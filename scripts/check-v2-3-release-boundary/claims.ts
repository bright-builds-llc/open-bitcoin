import {
  claimClauses,
  hasNoClaimMarker,
  hasPositiveClaim,
  markdownParagraphs,
  tableRowHasNoClaimStatus,
} from "../checker-shared/claim-text.ts";
import {
  CLAIM_FILES,
  D14_REQUIRED_FILES,
  D14_SENTENCE,
  DENIED_OVERCLAIMS,
  FJALL_DIFFERENCE_SENTENCE,
  V24_PRUNE_SENTENCE,
} from "./constants.ts";

export type TextCorpus = Map<string, string>;

export function checkClaims(texts: TextCorpus, failures: string[]): void {
  for (const file of D14_REQUIRED_FILES) {
    const text = texts.get(file) ?? "";
    if (!text.includes(D14_SENTENCE)) {
      failures.push(`${file}: missing required D-14 sentence ${D14_SENTENCE}`);
    }
  }

  for (const file of CLAIM_FILES) {
    const text = texts.get(file) ?? "";
    for (const paragraph of markdownParagraphs(text)) {
      const tableNoClaim = tableRowHasNoClaimStatus(paragraph.text);
      for (const clause of claimClauses(paragraph.text)) {
        const lower = clause.toLowerCase();
        if (!hasPositiveClaim(lower) || hasNoClaimMarker(lower) || tableNoClaim) continue;
        for (const topic of DENIED_OVERCLAIMS) {
          if (
            topic === "prune-mode" &&
            (clause.includes(V24_PRUNE_SENTENCE) || clause.includes(FJALL_DIFFERENCE_SENTENCE))
          ) {
            continue;
          }
          if (lower.includes(topic)) {
            failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 145 claim: ${topic}`);
          }
        }
      }
    }
  }
}

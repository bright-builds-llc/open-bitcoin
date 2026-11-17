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
  DENIED_OVERCLAIMS,
  V23_D14_SENTENCE,
  V24_PRUNE_SENTENCE,
} from "./constants.ts";

export type TextCorpus = Map<string, string>;

const REQUIRED_SENTENCES = [V24_PRUNE_SENTENCE, V23_D14_SENTENCE] as const;

/**
 * Records missing scoped sentences and positive denied claims in the curated corpus.
 * A clause that also contains the allowed prune sentence is still denied.
 */
export function checkClaims(texts: TextCorpus, failures: string[]): void {
  for (const file of D14_REQUIRED_FILES) {
    const text = texts.get(file) ?? "";
    for (const sentence of REQUIRED_SENTENCES) {
      if (!text.includes(sentence)) {
        failures.push(`${file}: missing required sentence ${sentence}`);
      }
    }
  }

  for (const file of CLAIM_FILES) {
    const text = texts.get(file) ?? "";
    for (const paragraph of markdownParagraphs(text)) {
      const tableNoClaim = tableRowHasNoClaimStatus(paragraph.text);
      for (const clause of claimClauses(paragraph.text)) {
        const lower = clause.toLowerCase();
        if (!hasPositiveClaim(lower) || hasNoClaimMarker(lower) || tableNoClaim) continue;
        if (lower.includes("txindex") && lower.includes("prune")) {
          failures.push(
            `${file}:${paragraph.startLine}: forbidden positive Phase 151 claim: txindex with prune`,
          );
        }
        for (const topic of DENIED_OVERCLAIMS) {
          if (lower.includes(topic)) {
            failures.push(
              `${file}:${paragraph.startLine}: forbidden positive Phase 151 claim: ${topic}`,
            );
          }
        }
      }
    }
  }
}

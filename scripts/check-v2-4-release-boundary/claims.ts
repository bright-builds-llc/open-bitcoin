import {
  CLAIM_FILES,
  D14_REQUIRED_FILES,
  DENIED_OVERCLAIMS,
  NO_CLAIM_MARKERS,
  POSITIVE_PATTERNS,
  V23_D14_SENTENCE,
  V24_PRUNE_SENTENCE,
  type ClaimFile,
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

function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let current: string[] = [];
  let startLine = 1;
  for (const [index, line] of text.split("\n").entries()) {
    if (line.trim().startsWith("|") && line.trim().endsWith("|")) {
      if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
      current = [];
      paragraphs.push({ startLine: index + 1, text: line.trim() });
      startLine = index + 2;
      continue;
    }
    if (line.trim() === "") {
      if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
      current = [];
      startLine = index + 2;
      continue;
    }
    if (current.length === 0) startLine = index + 1;
    current.push(line);
  }
  if (current.length > 0) paragraphs.push({ startLine, text: current.join(" ") });
  return paragraphs;
}

function claimClauses(paragraph: string): string[] {
  if (paragraph.startsWith("|") && paragraph.endsWith("|")) {
    return paragraph
      .slice(1, -1)
      .split("|")
      .map((value) => value.trim())
      .filter((value) => value !== "");
  }
  return paragraph.split(/(?<=[.!?])\s+|\s+\|\s+/).filter((value) => value.trim() !== "");
}

function tableRowHasNoClaimStatus(paragraph: string): boolean {
  if (!paragraph.startsWith("|") || !paragraph.endsWith("|")) return false;
  return paragraph
    .slice(1, -1)
    .split("|")
    .map((value) => value.trim().toLowerCase().replaceAll("`", ""))
    .some((value) =>
      ["deferred", "unsupported", "not allowed", "not allowed yet", "not run"].includes(value),
    );
}

function hasPositiveClaim(text: string): boolean {
  return POSITIVE_PATTERNS.some((pattern) => pattern.test(text));
}

function hasNoClaimMarker(text: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => text.includes(marker));
}

export function claimFiles(): readonly ClaimFile[] {
  return CLAIM_FILES;
}

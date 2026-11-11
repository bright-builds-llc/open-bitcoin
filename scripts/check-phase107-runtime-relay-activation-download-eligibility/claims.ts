import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { FORBIDDEN_CLAIMS, SENSITIVE_PUBLIC_EVIDENCE_PATTERNS, TextCorpus } from "./constants.ts";
import { markdownParagraphs, hasNoClaimMarker, hasPositiveClaim, isPublicEvidenceFile } from "./helpers.ts";

export function checkSensitivePublicEvidence(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!isPublicEvidenceFile(file)) {
      continue;
    }
    for (const [lineIndex, line] of text.split("\n").entries()) {
      for (const patternValue of SENSITIVE_PUBLIC_EVIDENCE_PATTERNS) {
        if (patternValue.test(line)) {
          failures.push(`${file}:${lineIndex + 1}: sensitive public evidence must stay aggregate and sanitized`);
        }
      }
    }
  }
}

export function checkForbiddenClaims(texts: TextCorpus, failures: string[]): void {
  for (const [file, text] of texts.entries()) {
    if (!file.startsWith("docs/") && !file.startsWith(".planning/") && file !== "README.md") {
      continue;
    }
    for (const paragraph of markdownParagraphs(text)) {
      const lowerText = paragraph.text.toLowerCase();
      for (const forbidden of FORBIDDEN_CLAIMS) {
        if (!lowerText.includes(forbidden)) {
          continue;
        }
        if (hasNoClaimMarker(lowerText) || !hasPositiveClaim(lowerText)) {
          continue;
        }
        failures.push(`${file}:${paragraph.startLine}: forbidden positive Phase 107 claim: ${forbidden}`);
      }
    }
  }
}

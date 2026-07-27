import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { PHASE106_TEST_COMMAND, PHASE106_CHECKER_COMMAND, PHASE107_TEST_COMMAND, PHASE107_CHECKER_COMMAND, FORBIDDEN_CLAIMS, FORBIDDEN_DEFAULT_VERIFIER_GATES, SENSITIVE_PUBLIC_EVIDENCE_PATTERNS, TextCorpus } from "./constants.ts";
import { orderedIndexes, markdownParagraphs, hasNoClaimMarker, hasPositiveClaim, isPublicEvidenceFile } from "./helpers.ts";

export function checkVerifierOrder(verifyText: string, failures: string[]): void {
  const visibleMarker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const visibleStart = verifyText.indexOf(visibleMarker);
  const visibleBodyStart = visibleStart + visibleMarker.length;
  const visibleEnd = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", visibleBodyStart);
  const visibleText =
    visibleStart === -1 || visibleEnd === -1 ? "" : verifyText.slice(visibleBodyStart, visibleEnd);
  if (
    !orderedIndexes(visibleText, [
      PHASE106_TEST_COMMAND,
      PHASE106_CHECKER_COMMAND,
      PHASE107_TEST_COMMAND,
      PHASE107_CHECKER_COMMAND,
    ])
  ) {
    failures.push("verifier-scope: Phase 107 visible order must immediately follow Phase 106");
  }

  if (
    !orderedIndexes(verifyText, [
      'run_step "test Phase 106 parity UAT release boundary checker"',
      'run_step "check Phase 106 parity UAT release boundary"',
      'run_step "test Phase 107 runtime relay activation/download eligibility checker"',
      'run_step "check Phase 107 runtime relay activation/download eligibility"',
      'run_step "check pure-core dependencies"',
    ])
  ) {
    failures.push("verifier-scope: Phase 107 executable order must follow Phase 106 and precede pure-core checks");
  }
}

export function checkForbiddenDefaultVerifierGates(verifyText: string, failures: string[]): void {
  const runStepLines = verifyText
    .split("\n")
    .map((line) => line.trim().toLowerCase())
    .filter((line) => line.startsWith("run_step "));
  for (const line of runStepLines) {
    for (const forbidden of FORBIDDEN_DEFAULT_VERIFIER_GATES) {
      if (line.includes(forbidden)) {
        failures.push(`verifier-scope: default verifier must not run ${forbidden}`);
      }
    }
  }
}

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

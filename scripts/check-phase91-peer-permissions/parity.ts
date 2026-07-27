import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE90_CHECKER_COMMAND, PHASE91_TEST_COMMAND, PHASE91_CHECKER_COMMAND, REQUIRED_PERMISSION_TOKENS, PHASE91_REQUIREMENTS, TARGET_FILES, REQUIRED_EVIDENCE, REQUIRED_UAT_COMMANDS, REQUIRED_EVIDENCE_LABELS, REQUIRED_METRICS, REQUIRED_CATALOG_ANCHORS, REQUIRED_BREADCRUMB_MAPPINGS, FORBIDDEN_VERIFY_STRINGS, FORBIDDEN_UNSCOPED_CLAIMS, FORBIDDEN_SUPPORT_RAW_DETAILS, ALLOWED_SCOPE_TERMS, COMMAND_PREFIXES } from "./constants.ts";
import type { AuditEntry, BreadcrumbGroup, BreadcrumbIndex, ChecklistSurface, ParityIndex, ParitySurface, TargetFile } from "./constants.ts";
import { checkPhase91PeerPermissions, readText, normalizeWhitespace, normalizeShellCommand, normalizedLower, requireContains, requireNormalizedContains, requireArrayIncludes, requireExactRequirements, verifyParityIndex, verifyTopLevelSurface, verifyChecklistSurface, verifyAuditEntry, verifyHumanDocs, verifyRuntimeGuideCommands, shellCommandUnits, pushCurrentShellCommandUnit, verifyEvidenceLabels, verifyParityDocs, verifySourceBreadcrumbs, verifyBreadcrumbMapping, executableVerifyText, verifyVerifierWiring } from "./checks.ts";
export function verifyVerifierOrder(executableText: string, failures: string[]): void {
  const phase90Index = executableText.indexOf(PHASE90_CHECKER_COMMAND);
  const phase91TestIndex = executableText.indexOf(PHASE91_TEST_COMMAND);
  const phase91CheckerIndex = executableText.indexOf(PHASE91_CHECKER_COMMAND);
  const pureCoreIndex = executableText.indexOf("bash scripts/check-pure-core-deps.sh");
  const orderValid =
    phase90Index !== -1 &&
    phase91TestIndex > phase90Index &&
    phase91CheckerIndex > phase91TestIndex &&
    pureCoreIndex > phase91CheckerIndex;

  if (!orderValid) {
    failures.push(
      "verifier-order requires executed Phase 91 test and checker after Phase 90 and before pure-core checks",
    );
  }
}

export function verifyVerifierBoundary(executableText: string, failures: string[]): void {
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 91 command text: ${forbidden}`);
    }
  }
}

export function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      verifyNoForbiddenClaim(file, unit, failures);
    }
  }
}

export function verifySupportRedactionBoundary(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      const lower = normalizedLower(unit);
      const supportContext =
        lower.includes("support bundle") ||
        (lower.includes("support") && lower.includes("evidence"));
      if (!supportContext || isScopedAllowance(unit)) {
        continue;
      }

      for (const rawDetail of FORBIDDEN_SUPPORT_RAW_DETAILS) {
        if (unit.includes(rawDetail)) {
          failures.push(`Phase 91 support redaction boundary raw detail in ${file}: ${unit}`);
        }
      }
    }
  }
}

export function contextUnits(text: string): string[] {
  const units: string[] = [];
  for (const block of text.replaceAll("\r\n", "\n").split(/\n\s*\n/)) {
    const lines = block
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
    if (lines.length === 0) {
      continue;
    }

    const tableRows = lines.filter(
      (line) => line.startsWith("|") && !/^\|\s*-/.test(line),
    );
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      const prose = lines.filter((line) => !line.startsWith("|")).join(" ");
      units.push(...sentenceUnits(prose));
      continue;
    }

    units.push(...sentenceUnits(lines.join(" ")));
  }

  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}

export function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  if (normalized.length === 0) {
    return [];
  }

  return normalized.split(/(?<=[.!?])\s+(?=[A-Z`])/);
}

export function verifyNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of FORBIDDEN_UNSCOPED_CLAIMS) {
    if (normalizedLower(unit).includes(claim.toLowerCase())) {
      failures.push(`Phase 91 no-claim boundary forbidden claim in ${file}: ${unit}`);
    }
  }
}

export function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term.toLowerCase()));
}

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE88_CHECKER_COMMAND, PHASE90_TEST_COMMAND, PHASE90_CHECKER_COMMAND, PHASE90_REQUIREMENTS, TARGET_FILES, REQUIRED_EVIDENCE, REQUIRED_UAT_COMMANDS, REQUIRED_EVIDENCE_LABELS, REQUIRED_CATALOG_ANCHORS, REQUIRED_BREADCRUMB_MAPPINGS, FORBIDDEN_VERIFY_STRINGS, PUBLIC_DEFAULT_CLAIMS, PRODUCTION_READY_CLAIMS, ALLOWED_SCOPE_TERMS, COMMAND_PREFIXES } from "./constants.ts";
import type { AuditEntry, BreadcrumbIndex, BreadcrumbGroup, ChecklistSurface, ParityIndex, ParitySurface, TargetFile } from "./constants.ts";
import { checkPhase90InboundListenerAdmission, readText, normalizeWhitespace, normalizeShellCommand, normalizedLower, requireContains, requireNormalizedContains, requireArrayIncludes, requireExactRequirements, verifyParityIndex, verifyTopLevelSurface, verifyChecklistSurface, verifyAuditEntry, verifyHumanDocs, verifyRuntimeGuideCommands, shellCommandUnits, pushCurrentShellCommandUnit, verifyEvidenceLabels, verifyParityDocs, verifySourceBreadcrumbs, verifyBreadcrumbMapping, executableVerifyText, verifyVerifierWiring, verifyVerifierOrder, verifyVerifierBoundary } from "./checks.ts";
export function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "docs/parity/source-breadcrumbs.json" || file === "scripts/verify.sh") {
      continue;
    }

    for (const unit of contextUnits(text)) {
      verifyNoPublicDefaultClaim(file, unit, failures);
      verifyNoProductionReadinessClaim(file, unit, failures);
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

export function verifyNoPublicDefaultClaim(
  file: string,
  unit: string,
  failures: string[],
): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of PUBLIC_DEFAULT_CLAIMS) {
    if (normalizedLower(unit).includes(claim.toLowerCase())) {
      failures.push(`Phase 90 no-claim boundary public inbound default claim in ${file}: ${unit}`);
    }
  }
}

export function verifyNoProductionReadinessClaim(
  file: string,
  unit: string,
  failures: string[],
): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of PRODUCTION_READY_CLAIMS) {
    if (normalizeWhitespace(unit).includes(normalizeWhitespace(claim))) {
      failures.push(`Phase 90 no-claim boundary production full-node readiness claim in ${file}: ${unit}`);
    }
  }
}

export function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term.toLowerCase()));
}

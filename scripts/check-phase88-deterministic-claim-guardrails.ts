#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE88_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-deterministic-claim-guardrails";
const AUDIT_KEY = "v1_8_deterministic_claim_guardrails";
const PHASE87_CHECKER_COMMAND = "bun run scripts/check-phase87-release-readiness.ts";
const PHASE88_TEST_COMMAND =
  "bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts";
const PHASE88_CHECKER_COMMAND =
  "bun run scripts/check-phase88-deterministic-claim-guardrails.ts";
const PHASE88_REQUIREMENTS = ["REL-02", "REL-03", "REL-04"] as const;
const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const POINTER_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/check-phase88-deterministic-claim-guardrails.ts",
  "scripts/check-phase88-deterministic-claim-guardrails.test.ts",
  "scripts/verify.sh",
] as const;
const EXACT_OVERCLAIM_SMOKE_STRINGS = [
  "Open Bitcoin is production full-node ready.",
  "v1.8 proves production full-node readiness.",
  "Open Bitcoin has production full-node readiness.",
  "Open Bitcoin supports production service operation.",
] as const;
const DEFERRED_SURFACES = [
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet use",
  "production-funds wallet safety",
  "migration apply mode",
  "signed packaging",
  "package-manager distribution",
  "Windows service integration",
  "hosted dashboards",
  "GUI parity",
  "public-network default checks",
  "public-network CI",
  "release-blocking live sync",
  "automatic support-bundle upload",
  "destructive repair",
  "broad production-node readiness",
] as const;
const PROMOTION_PREDICATES = [
  "production-ready",
  "production ready",
  "production-grade",
  "production grade",
  "fully supported",
  "default-verified",
  "default verified",
  "release-blocking",
  "release blocking",
  "proven",
  "GA",
  "certified",
] as const;
const ALLOWED_SCOPE_TERMS = [
  "does not claim",
  "not allowed yet",
  "deferred",
  "unsupported",
  "historical",
  "opt-in UAT",
  "future gate",
  "outside default verification",
  "defines gates",
  "future milestone",
  "does not prove",
  "does not add",
  "without claiming",
  "without internet access",
  "no public-network",
  "remain outside",
  "remains outside",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "sleep 86400",
  "--restart-after-progress",
  "brew services",
  "public-network CI",
  "public-network default checks",
  "release-blocking live sync",
  "automatic support-bundle upload",
  "destructive repair",
  "broad production-node readiness",
] as const;

type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: Record<string, unknown>;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

export function checkPhase88DeterministicClaimGuardrails(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanPointers(texts, failures);
  verifyTextualClaimGuardrails(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizedLower(text).includes(normalizedLower(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root must be an array`);
    return;
  }
  if (!value.includes(required)) {
    failures.push(`${label} parity root missing required value: ${required}`);
  }
}

function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const expected = JSON.stringify(PHASE88_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
  }
}

function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`parity root index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyChecklistSurface(parsed, failures);
  verifyAuditEntry(parsed, failures);
}

function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("parity root surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`parity root surfaces missing done ${SURFACE_ID}`);
  }
}

function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("parity root checklist.surfaces must be an array");
    return;
  }

  const surface = checklistSurfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`parity root checklist missing done ${SURFACE_ID}`);
  }
  requireExactRequirements(surface?.requirements, `${SURFACE_ID}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(surface?.evidence, `${SURFACE_ID}.evidence`, evidence, failures);
  }
}

function verifyAuditEntry(parsed: ParityIndex, failures: string[]): void {
  const auditEntry = parsed.audit?.[AUDIT_KEY] as AuditEntry | undefined;
  if (auditEntry?.path !== "release-readiness.md" || auditEntry.status !== "done") {
    failures.push(`parity root audit.${AUDIT_KEY} is missing or incomplete`);
    return;
  }
  requireExactRequirements(auditEntry.requirements, `audit.${AUDIT_KEY}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(auditEntry.evidence, `audit.${AUDIT_KEY}.evidence`, evidence, failures);
  }
}

function verifyHumanPointers(texts: Map<string, string>, failures: string[]): void {
  for (const file of POINTER_FILES) {
    requireNormalizedContains(
      texts.get(file) ?? "",
      "v1.8 deterministic claim guardrails",
      file,
      failures,
    );
  }

  requireContains(
    texts.get("docs/parity/checklist.md") ?? "",
    SURFACE_ID,
    "docs/parity/checklist.md",
    failures,
  );
}

function verifyTextualClaimGuardrails(
  texts: Map<string, string>,
  failures: string[],
): void {
  for (const [file, text] of texts) {
    if (file === "docs/parity/index.json" || file === "scripts/verify.sh") {
      continue;
    }
    for (const unit of contextUnits(text)) {
      verifyProductionReadinessClaims(file, unit, failures);
      verifyDeferredSurfacePromotion(file, unit, failures);
    }
  }
}

function contextUnits(text: string): string[] {
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

function sentenceUnits(text: string): string[] {
  const normalized = normalizeWhitespace(text);
  if (normalized.length === 0) {
    return [];
  }

  return normalized.split(/(?<=[.!?])\s+(?=[A-Z`])/);
}

function verifyProductionReadinessClaims(
  file: string,
  unit: string,
  failures: string[],
): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  for (const claim of EXACT_OVERCLAIM_SMOKE_STRINGS) {
    if (normalizeWhitespace(unit).includes(normalizeWhitespace(claim))) {
      failures.push(`${file} production readiness claim must be scoped or removed: ${unit}`);
    }
  }
}

function verifyDeferredSurfacePromotion(
  file: string,
  unit: string,
  failures: string[],
): void {
  if (isScopedAllowance(unit)) {
    return;
  }

  const surface = DEFERRED_SURFACES.find((candidate) => containsPhrase(unit, candidate));
  if (surface === undefined) {
    return;
  }

  const predicate = PROMOTION_PREDICATES.find((candidate) =>
    containsPromotionPredicate(unit, candidate),
  );
  if (predicate === undefined) {
    return;
  }

  failures.push(
    `${file} deferred surface promotion must be scoped or removed: ${surface} + ${predicate}: ${unit}`,
  );
}

function isScopedAllowance(unit: string): boolean {
  const lower = normalizedLower(unit);
  return ALLOWED_SCOPE_TERMS.some((term) => lower.includes(term.toLowerCase()));
}

function containsPhrase(text: string, phrase: string): boolean {
  return normalizedLower(text).includes(normalizedLower(phrase));
}

function containsPromotionPredicate(text: string, predicate: string): boolean {
  if (predicate === "GA") {
    return /\bGA\b/.test(text);
  }
  if (predicate === "release-blocking" || predicate === "release blocking") {
    return stripReleaseBlockingDeferredSurface(text).includes(predicate);
  }

  return containsPhrase(text, predicate);
}

function stripReleaseBlockingDeferredSurface(text: string): string {
  return normalizedLower(text)
    .replaceAll("release-blocking live sync", "")
    .replaceAll("release blocking live sync", "");
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE88_TEST_COMMAND, PHASE88_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  for (const command of [PHASE88_TEST_COMMAND, PHASE88_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  requireContains(
    executableText,
    `run_step "test Phase 88 deterministic claim guardrails checker" ${PHASE88_TEST_COMMAND}`,
    "verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 88 deterministic claim guardrails" ${PHASE88_CHECKER_COMMAND}`,
    "verifier-order",
    failures,
  );

  const phase87CheckerIndex = executableText.indexOf(PHASE87_CHECKER_COMMAND);
  const phase88TestIndex = executableText.indexOf(PHASE88_TEST_COMMAND);
  const phase88CheckerIndex = executableText.indexOf(PHASE88_CHECKER_COMMAND);
  const orderValid =
    phase87CheckerIndex !== -1 &&
    phase88TestIndex > phase87CheckerIndex &&
    phase88CheckerIndex > phase88TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 88 test and checker after Phase 87 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 88 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase88DeterministicClaimGuardrails();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 88 deterministic claim guardrails");
  }
}

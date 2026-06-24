#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE87_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-release-readiness-checklist";
const AUDIT_KEY = "v1_8_release_readiness_checklist";
const RELEASE_READINESS_PATH = "docs/parity/release-readiness.md";
const TABLE_HEADER =
  "Requirement | Phase | Canonical evidence | Default verification | UAT or manual evidence | Residual risk | No-claim or next gate";
const PHASE86_CHECKER_COMMAND =
  "bun run scripts/check-phase86-service-operation-expectations.ts";
const PHASE87_TEST_COMMAND = "bun test scripts/check-phase87-release-readiness.test.ts";
const PHASE87_CHECKER_COMMAND = "bun run scripts/check-phase87-release-readiness.ts";
const PHASE88_TEST_COMMAND =
  "bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts";
const PHASE88_CHECKER_COMMAND =
  "bun run scripts/check-phase88-deterministic-claim-guardrails.ts";
const PHASE87_REQUIREMENTS = [
  "PROD-01",
  "PROD-02",
  "PROD-03",
  "PROD-04",
  "SUP-01",
  "SUP-02",
  "SUP-03",
  "SUP-04",
  "UPG-01",
  "UPG-02",
  "UPG-03",
  "UPG-04",
  "RUN-01",
  "RUN-02",
  "RUN-03",
  "SVC-01",
  "SVC-02",
  "REL-01",
  "REL-02",
  "REL-03",
  "REL-04",
  "REL-05",
  "REL-06",
] as const;
const TARGET_FILES = [
  RELEASE_READINESS_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const HUMAN_POINTER_FILES = TARGET_FILES.filter(
  (file) =>
    file !== RELEASE_READINESS_PATH &&
    file !== "docs/parity/index.json" &&
    file !== "scripts/verify.sh",
);
const REQUIRED_EVIDENCE = [
  RELEASE_READINESS_PATH,
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/check-phase87-release-readiness.ts",
  "scripts/check-phase87-release-readiness.test.ts",
  "scripts/check-phase88-deterministic-claim-guardrails.ts",
  "scripts/check-phase88-deterministic-claim-guardrails.test.ts",
  "scripts/verify.sh",
] as const;
const CANONICAL_ROOTS = [
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;
const RELEASE_CHECKER_COMMANDS = [
  "bun run scripts/check-phase82-production-claim-boundary.ts",
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts",
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts",
  "bun run scripts/check-phase85-operator-runbooks.ts",
  PHASE86_CHECKER_COMMAND,
  PHASE87_CHECKER_COMMAND,
  PHASE87_TEST_COMMAND,
  PHASE88_TEST_COMMAND,
  PHASE88_CHECKER_COMMAND,
  "bash scripts/verify.sh",
] as const;
const NO_CLAIM_TERMS = [
  "production full-node readiness",
  "production service operation",
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet use or safety",
  "migration apply mode",
  "signed packaging or package-manager distribution",
  "Windows service integration",
  "hosted dashboards",
  "GUI parity",
  "public-network default checks",
  "public-network CI",
  "release-blocking live sync",
  "destructive repair",
  "automatic support-bundle upload",
  "broad production-node readiness",
] as const;
const CONTEXT_ONLY_SIGNALS = [
  "Artifact existence",
  "daemon startup",
  "elapsed time",
  "peer reachability",
  "raw log tail",
  "service file existence",
  "support bundle path",
  "context only",
  "named fields",
  "unavailable reasons",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "--restart-after-progress",
  "brew services",
  "Windows service",
  "automatic support-bundle upload",
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

export function checkPhase87ReleaseReadiness(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyReleaseReadiness(texts.get(RELEASE_READINESS_PATH) ?? "", failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanPointers(texts, failures);
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
  if (!normalizeWhitespace(text).includes(normalizeWhitespace(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

function requireNotNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (normalizeWhitespace(text).includes(normalizeWhitespace(needle))) {
    failures.push(`${label} must not duplicate required text: ${needle}`);
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
  const expected = JSON.stringify(PHASE87_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
  }
}

function verifyReleaseReadiness(text: string, failures: string[]): void {
  requireContains(text, "## v1.8 Release Readiness Checklist", "release-readiness", failures);
  requireContains(text, `Surface id: \`${SURFACE_ID}\``, "release-readiness", failures);
  requireNormalizedContains(text, TABLE_HEADER, "release-readiness checklist", failures);
  requireContains(
    text,
    "## v1.8 Release Readiness No-Claim Review",
    "release-readiness no-claim review",
    failures,
  );

  for (const requirement of PHASE87_REQUIREMENTS) {
    requireContains(text, requirement, "release-readiness checklist requirements", failures);
  }
  for (const root of CANONICAL_ROOTS) {
    requireContains(text, root, "release-readiness canonical roots", failures);
  }
  for (const command of RELEASE_CHECKER_COMMANDS) {
    requireContains(text, command, "release-readiness deterministic commands", failures);
  }
  for (const term of NO_CLAIM_TERMS) {
    requireNormalizedContains(text, term, "release-readiness no-claim review", failures);
  }
  for (const signal of CONTEXT_ONLY_SIGNALS) {
    requireNormalizedContains(text, signal, "release-readiness context-only evidence", failures);
  }
  requireNormalizedContains(
    text,
    "Phase 88 owns REL-02, REL-03, and REL-04 broad deterministic claim guardrails.",
    "release-readiness Phase 88 boundary",
    failures,
  );
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
  for (const file of HUMAN_POINTER_FILES) {
    const text = texts.get(file) ?? "";
    const hasSurfaceId = text.includes(SURFACE_ID);
    const hasChecklistPointer = normalizeWhitespace(text).includes("v1.8 release-readiness checklist");
    if (!hasSurfaceId && !hasChecklistPointer) {
      failures.push(`${file} missing compact Phase 87 release-readiness checklist pointer`);
    }
    requireNotNormalizedContains(text, TABLE_HEADER, file, failures);
  }

  requireContains(
    texts.get("docs/parity/checklist.md") ?? "",
    SURFACE_ID,
    "docs/parity/checklist.md",
    failures,
  );
  requireContains(
    texts.get("README.md") ?? "",
    "docs/parity/release-readiness.md#v18-release-readiness-checklist",
    "README.md",
    failures,
  );
  requireContains(
    texts.get("docs/parity/README.md") ?? "",
    "release-readiness.md#v18-release-readiness-checklist",
    "docs/parity/README.md",
    failures,
  );
  requireContains(
    texts.get("docs/parity/catalog/operator-runtime-release-hardening.md") ?? "",
    "Phase 87 release-readiness checklist",
    "docs/parity/catalog/operator-runtime-release-hardening.md",
    failures,
  );
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE87_TEST_COMMAND, PHASE87_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  for (const command of [PHASE87_TEST_COMMAND, PHASE87_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase86CheckerIndex = executableText.indexOf(PHASE86_CHECKER_COMMAND);
  const phase87TestIndex = executableText.indexOf(PHASE87_TEST_COMMAND);
  const phase87CheckerIndex = executableText.indexOf(PHASE87_CHECKER_COMMAND);
  const orderValid =
    phase86CheckerIndex !== -1 &&
    phase87TestIndex > phase86CheckerIndex &&
    phase87CheckerIndex > phase87TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 87 test and checker after Phase 86 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 87 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase87ReleaseReadiness();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 87 release readiness");
  }
}

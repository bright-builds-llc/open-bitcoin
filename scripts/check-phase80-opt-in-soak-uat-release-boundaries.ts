#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE80_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR = ".planning/phases/80-opt-in-soak-uat-and-release-boundaries";
const PHASE80_REQUIREMENTS = ["VER-05", "VER-06", "VER-07", "REL-04"] as const;
const SURFACE_ID = "v1-7-full-sync-soak-recovery-release-boundaries";
const PHASE75_TEST_COMMAND = "bun test scripts/check-phase75-soak-runner.test.ts";
const PHASE75_CHECKER_COMMAND = "bun run scripts/check-phase75-soak-runner.ts";
const PHASE76_TEST_COMMAND = "bun test scripts/check-phase76-resource-bounds.test.ts";
const PHASE76_CHECKER_COMMAND = "bun run scripts/check-phase76-resource-bounds.ts";
const PHASE77_TEST_COMMAND = "bun test scripts/check-phase77-corruption-lock-recovery.test.ts";
const PHASE77_CHECKER_COMMAND = "bun run scripts/check-phase77-corruption-lock-recovery.ts";
const PHASE78_TEST_COMMAND = "bun test scripts/check-phase78-progress-guarantees.test.ts";
const PHASE78_CHECKER_COMMAND = "bun run scripts/check-phase78-progress-guarantees.ts";
const PHASE79_TEST_COMMAND =
  "bun test scripts/check-phase79-diagnostics-support-bundle.test.ts";
const PHASE79_CHECKER_COMMAND = "bun run scripts/check-phase79-diagnostics-support-bundle.ts";
const PHASE80_TEST_COMMAND =
  "bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts";
const PHASE80_CHECKER_COMMAND =
  "bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts";
const REQUIRED_VERIFY_ORDER = [
  PHASE75_TEST_COMMAND,
  PHASE75_CHECKER_COMMAND,
  PHASE76_TEST_COMMAND,
  PHASE76_CHECKER_COMMAND,
  PHASE77_TEST_COMMAND,
  PHASE77_CHECKER_COMMAND,
  PHASE78_TEST_COMMAND,
  PHASE78_CHECKER_COMMAND,
  PHASE79_TEST_COMMAND,
  PHASE79_CHECKER_COMMAND,
  PHASE80_TEST_COMMAND,
  PHASE80_CHECKER_COMMAND,
] as const;
const REQUIRED_PRE_PHASE80_VERIFY_ORDER = REQUIRED_VERIFY_ORDER.slice(0, 10);
const PLAN_FILES = [
  `${PHASE_DIR}/80-01-PLAN.md`,
  `${PHASE_DIR}/80-02-PLAN.md`,
  `${PHASE_DIR}/80-03-PLAN.md`,
] as const;
const WORKFLOWS = [
  "Multi-day soak lifecycle",
  "Bounded recovery drill",
  "Support-bundle generation",
  "Post-failure diagnosis",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl",
  "launchctl",
  "-openbitcoinsync=mainnet-ibd",
  "openbitcoinsync=mainnet-ibd",
  "sleep 86400",
  "sleep 259200",
  "multi-day wall-clock",
  "current tip",
  "current-tip",
  "release-blocking live sync",
  "/proc",
  "lsof",
  "fallocate",
  "mkfile",
  "dd if=",
  "107374182400",
] as const;
const NON_CLAIMS = [
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet use",
  "migration apply mode",
  "signed packaging",
  "Windows service support",
  "GUI",
  "hosted dashboards",
  "public-network default checks",
  "public-network CI",
  "release-blocking live sync",
  "automatic support-bundle upload",
  "destructive repair",
  "broad production-node readiness",
] as const;
const BROAD_CLAIM_STRINGS = [
  "v1.7 proves broad production-node readiness",
  "v1.7 production-node readiness is proven",
  "Phase 80 proves production-node readiness",
] as const;
const PLACEHOLDER_STRINGS = [
  "simplified version",
  "static for now",
  "future enhancement",
  "basic version",
  "will be wired later",
  "placeholder implementation",
  "placeholder for v1.7",
] as const;
const FORBIDDEN_MANIFEST_PATHS = [
  "docs/parity/v1.7-evidence-manifest.json",
  "docs/parity/evidence-manifest-v1.7.json",
  "docs/parity/release-evidence-v1.7.json",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/operator/runtime-guide.md",
  "docs/parity/release-readiness.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-parity-breadcrumbs.ts",
  "scripts/check-phase75-soak-runner.ts",
  "scripts/check-phase76-resource-bounds.ts",
  "scripts/check-phase77-corruption-lock-recovery.ts",
  "scripts/check-phase78-progress-guarantees.ts",
  "scripts/check-phase79-diagnostics-support-bundle.ts",
  "scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts",
  "scripts/verify.sh",
  `${PHASE_DIR}/80-VERIFICATION.md`,
] as const;

type AnchorMap = Record<string, readonly string[]>;

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

type ParityIndex = {
  audit?: unknown;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

const SOURCE_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "SupportEvidenceBundle",
    "support_forensics",
    "resource_bound_evidence",
    "soak_evidence",
  ],
  "packages/open-bitcoin-cli/src/operator/support/forensics.rs": [
    "SupportForensicsEvidence",
    "ForensicTimelineEntry",
    "CheckpointChainEvidence",
    "ForensicNarrative",
  ],
  "packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs": ["SoakSupportEvidence"],
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": ["SoakReportProjection"],
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": ["SoakLedgerEventEnvelope"],
} as const satisfies AnchorMap;
const CLAIM_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;
const MACHINE_ROOT_FILES = [
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;

function repoPath(repoRoot: string, relativePath: string): string {
  return path.join(repoRoot, relativePath);
}

function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = repoPath(repoRoot, relativePath);
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

function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain Phase 80 forbidden text: ${needle}`);
  }
}

function requireArrayIncludes(
  value: unknown,
  label: string,
  required: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} must be an array`);
    return;
  }
  if (!value.includes(required)) {
    failures.push(`${label} missing required value: ${required}`);
  }
}

function frontmatterFor(text: string): string {
  if (!text.startsWith("---")) {
    return text;
  }

  const endIndex = text.indexOf("\n---", 3);
  if (endIndex === -1) {
    return text;
  }

  return text.slice(0, endIndex);
}

function requireAnchors(repoRoot: string, anchors: AnchorMap, failures: string[]): void {
  for (const [file, needles] of Object.entries(anchors)) {
    const text = readText(repoRoot, file, failures);
    for (const needle of needles) {
      requireContains(text, needle, file, failures);
    }
  }
}

function verifyPlanRequirements(repoRoot: string, failures: string[]): void {
  const frontmatters = PLAN_FILES.map((planFile) =>
    frontmatterFor(readText(repoRoot, planFile, failures)),
  ).join("\n");

  for (const requirement of PHASE80_REQUIREMENTS) {
    requireContains(frontmatters, requirement, "Phase 80 plan frontmatter", failures);
  }
}

function verifyRuntimeGuide(repoRoot: string, failures: string[]): void {
  const runtimeGuide = readText(repoRoot, "docs/operator/runtime-guide.md", failures);
  const matrix = phase80Matrix(runtimeGuide, failures);
  for (const needle of [
    "Evidence proves",
    "Does not prove",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "durable run identity",
    "typed final outcome",
    "recovery_evidence",
    "support_forensics",
    "forensic timeline",
    "checkpoint chain",
    "failure narrative",
    "artifact presence",
    "daemon startup",
    "peer reachability",
    "elapsed time",
    "raw logs",
    "stale reports",
  ]) {
    requireContains(matrix, needle, "Phase 80 UAT matrix", failures);
  }
  verifyWorkflowRows(matrix, failures);
}

function phase80Matrix(runtimeGuide: string, failures: string[]): string {
  const heading = "### Phase 80 v1.7 opt-in soak UAT matrix";
  const startIndex = runtimeGuide.indexOf(heading);
  if (startIndex === -1) {
    failures.push(`docs/operator/runtime-guide.md missing required text: ${heading}`);
    return "";
  }

  const afterHeading = runtimeGuide.slice(startIndex);
  const nextSectionMatch = afterHeading.slice(heading.length).match(/\n## /);
  if (nextSectionMatch === null || nextSectionMatch.index === undefined) {
    return afterHeading;
  }

  return afterHeading.slice(0, heading.length + nextSectionMatch.index);
}

function verifyWorkflowRows(matrix: string, failures: string[]): void {
  const workflowNames = matrix
    .split("\n")
    .filter((line) => line.startsWith("| "))
    .filter((line) => !line.startsWith("| ---"))
    .filter((line) => !line.startsWith("| Workflow "))
    .map((line) => line.split("|")[1]?.trim() ?? "")
    .filter((name) => name.length > 0);

  if (workflowNames.length !== WORKFLOWS.length) {
    failures.push(
      `Phase 80 UAT matrix must contain exactly ${WORKFLOWS.length} workflow rows, found ${workflowNames.length}`,
    );
  }
  for (const workflow of WORKFLOWS) {
    if (!workflowNames.includes(workflow)) {
      failures.push(`Phase 80 UAT matrix missing workflow: ${workflow}`);
    }
  }
}

function verifyClaimDocs(repoRoot: string, failures: string[]): void {
  const claimText = CLAIM_FILES.map((file) => readText(repoRoot, file, failures)).join("\n");
  requireNormalizedContains(
    claimText,
    "explicit opt-in full-sync soak and recovery hardening",
    "Phase 80 claim-bearing docs",
    failures,
  );
  for (const nonClaim of NON_CLAIMS) {
    requireContains(claimText, nonClaim, "Phase 80 claim-bearing docs", failures);
  }
  for (const forbidden of [...BROAD_CLAIM_STRINGS, ...PLACEHOLDER_STRINGS]) {
    requireNotContains(claimText, forbidden, "Phase 80 claim-bearing docs", failures);
  }

  for (const file of MACHINE_ROOT_FILES) {
    const text = readText(repoRoot, file, failures);
    requireContains(text, SURFACE_ID, file, failures);
    for (const requirement of PHASE80_REQUIREMENTS) {
      requireContains(text, requirement, file, failures);
    }
  }
}

function verifyParityIndex(repoRoot: string, failures: string[]): void {
  const index = parseParityIndex(repoRoot, failures);
  if (index === null) {
    return;
  }

  verifyTopLevelSurface(index, failures);
  const maybeChecklistSurface = checklistSurface(index, failures);
  if (maybeChecklistSurface === null) {
    return;
  }
  if (maybeChecklistSurface.status !== "done") {
    failures.push(`${SURFACE_ID} checklist status must be done`);
  }
  for (const requirement of PHASE80_REQUIREMENTS) {
    requireArrayIncludes(
      maybeChecklistSurface.requirements,
      `${SURFACE_ID}.requirements`,
      requirement,
      failures,
    );
  }
  for (const evidencePath of REQUIRED_EVIDENCE) {
    requireArrayIncludes(
      maybeChecklistSurface.evidence,
      `${SURFACE_ID}.evidence`,
      evidencePath,
      failures,
    );
  }

  const auditText = JSON.stringify(index.audit ?? {});
  requireContains(auditText, "v1_7_release_boundaries", "docs/parity/index.json audit", failures);
}

function parseParityIndex(repoRoot: string, failures: string[]): ParityIndex | null {
  try {
    return JSON.parse(readText(repoRoot, "docs/parity/index.json", failures)) as ParityIndex;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    failures.push(`docs/parity/index.json must parse as JSON: ${message}`);
    return null;
  }
}

function verifyTopLevelSurface(index: ParityIndex, failures: string[]): void {
  if (!Array.isArray(index.surfaces)) {
    failures.push("docs/parity/index.json surfaces must be an array");
    return;
  }

  const matchingSurfaces = (index.surfaces as ParitySurface[]).filter(
    (surface) => surface.name === SURFACE_ID,
  );
  if (matchingSurfaces.length !== 1) {
    failures.push(
      `expected exactly one top-level surface with name ${SURFACE_ID}, found ${matchingSurfaces.length}`,
    );
    return;
  }
  const [surface] = matchingSurfaces;
  if (surface.status !== "done") {
    failures.push(`${SURFACE_ID} top-level status must be done`);
  }
}

function checklistSurface(index: ParityIndex, failures: string[]): ChecklistSurface | null {
  const maybeSurfaces = index.checklist?.surfaces;
  if (!Array.isArray(maybeSurfaces)) {
    failures.push("docs/parity/index.json checklist.surfaces must be an array");
    return null;
  }

  const matchingSurfaces = (maybeSurfaces as ChecklistSurface[]).filter(
    (surface) => surface.id === SURFACE_ID,
  );
  if (matchingSurfaces.length !== 1) {
    failures.push(
      `expected exactly one checklist surface with id ${SURFACE_ID}, found ${matchingSurfaces.length}`,
    );
    return null;
  }

  return matchingSurfaces[0];
}

function verifyForbiddenManifestPaths(repoRoot: string, failures: string[]): void {
  for (const manifestPath of FORBIDDEN_MANIFEST_PATHS) {
    if (existsSync(repoPath(repoRoot, manifestPath))) {
      failures.push(`Phase 80 must not add evidence manifest path: ${manifestPath}`);
    }
  }
}

function verifyVerifyScript(repoRoot: string, failures: string[]): void {
  const verifyScript = readText(repoRoot, "scripts/verify.sh", failures);
  for (const command of REQUIRED_PRE_PHASE80_VERIFY_ORDER) {
    requireContains(verifyScript, command, "scripts/verify.sh", failures);
  }
  verifyCommandOrder(verifyScript, failures);
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

function verifyCommandOrder(verifyScript: string, failures: string[]): void {
  const lines = verifyScript
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  const indices = REQUIRED_PRE_PHASE80_VERIFY_ORDER.map((command) => lines.indexOf(command));

  if (indices.some((index) => index === -1)) {
    failures.push("scripts/verify.sh missing one or more Phase 75 through Phase 79 commands");
    return;
  }
  for (let index = 1; index < indices.length; index += 1) {
    if (indices[index] <= indices[index - 1]) {
      failures.push("scripts/verify.sh must run Phase 75 through Phase 79 commands in order");
      return;
    }
  }
  const phase79Index = lines.indexOf(PHASE79_CHECKER_COMMAND);
  const phase80TestIndex = lines.indexOf(PHASE80_TEST_COMMAND);
  const phase80CheckerIndex = lines.indexOf(PHASE80_CHECKER_COMMAND);
  if (phase80TestIndex === -1 && phase80CheckerIndex === -1) {
    return;
  }
  if (phase80TestIndex === -1 || phase80CheckerIndex === -1) {
    failures.push("scripts/verify.sh must include both Phase 80 checker commands when either is present");
    return;
  }
  if (phase80TestIndex !== phase79Index + 1 || phase80CheckerIndex !== phase80TestIndex + 1) {
    failures.push(
      "scripts/verify.sh must run the Phase 80 checker test and checker immediately after Phase 79",
    );
  }
}

export function checkPhase80OptInSoakUatReleaseBoundaries(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];

  verifyPlanRequirements(repoRoot, failures);
  verifyRuntimeGuide(repoRoot, failures);
  verifyClaimDocs(repoRoot, failures);
  verifyParityIndex(repoRoot, failures);
  requireAnchors(repoRoot, SOURCE_ANCHORS, failures);
  verifyForbiddenManifestPaths(repoRoot, failures);
  verifyVerifyScript(repoRoot, failures);

  return failures;
}

function main(): void {
  const failures = checkPhase80OptInSoakUatReleaseBoundaries();

  if (failures.length > 0) {
    const output = failures.join("\n");
    console.error(output);
    process.exitCode = 1;
    return;
  }

  console.log("validated Phase 80 opt-in soak UAT and release boundaries");
}

if (import.meta.main) {
  main();
}

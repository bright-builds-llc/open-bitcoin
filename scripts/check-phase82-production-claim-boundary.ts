#!/usr/bin/env bun

import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE82_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-production-claim-boundary";
const PHASE82_REQUIREMENTS = ["PROD-01", "PROD-02", "PROD-03", "PROD-04"] as const;
const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;
const MATRIX_COLUMNS = [
  "Statement",
  "Support term",
  "Current status",
  "Evidence sources",
  "Verification command",
  "UAT status",
  "Residual risk",
  "Next required gate",
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
  "signed packaging or package-manager distribution",
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
const NOT_ALLOWED_STATEMENTS = [
  "Open Bitcoin has production full-node readiness.",
  "Open Bitcoin supports production service operation.",
  "Open Bitcoin supports relay/inbound serving.",
  "Open Bitcoin supports production wallet use.",
  "Open Bitcoin supports migration apply mode.",
  "Open Bitcoin supports signed distribution.",
  "Open Bitcoin supports hosted dashboards.",
  "Open Bitcoin supports public-network CI.",
  "Open Bitcoin supports destructive repair.",
  "Open Bitcoin supports automatic support upload.",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "scripts/verify.sh",
] as const;
const CLAIM_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
] as const;
const FORBIDDEN_NEAR_SYNONYMS = [
  "production-grade",
  "production-ish",
  "beta-supported",
  "ready enough",
] as const;
const EXACT_OVERCLAIMS = [
  "Open Bitcoin is production full-node ready.",
  "v1.8 proves production full-node readiness.",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "release-blocking live sync",
  "automatic support-bundle upload",
  "destructive repair",
] as const;
const PHASE80_CHECKER_COMMAND =
  "bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts";
const PHASE82_TEST_COMMAND =
  "bun test scripts/check-phase82-production-claim-boundary.test.ts";
const PHASE82_CHECKER_COMMAND = "bun run scripts/check-phase82-production-claim-boundary.ts";

type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
  title?: unknown;
};

type ParitySurface = {
  name?: unknown;
  status?: unknown;
};

type AuditEntry = {
  evidence?: unknown;
  path?: unknown;
  requirements?: unknown;
  status?: unknown;
};

type ClaimMatrixRow = {
  currentStatus: string;
  statement: string;
  supportTerm: string;
  verificationCommand: string;
};

type ParityIndex = {
  audit?: Record<string, unknown>;
  checklist?: {
    surfaces?: unknown;
  };
  surfaces?: unknown;
};

export function checkPhase82ProductionClaimBoundary(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of [...CLAIM_FILES, "docs/parity/index.json", "scripts/verify.sh"]) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyCanonicalBoundary(texts.get("docs/parity/production-claim-boundary.md") ?? "", failures);
  verifyReleaseReadiness(texts.get("docs/parity/release-readiness.md") ?? "", failures);
  verifyDeviations(texts.get("docs/parity/deviations-and-unknowns.md") ?? "", failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanRoots(texts, failures);
  verifyForbiddenClaimLanguage(texts, failures);
  verifyNoV18Manifest(repoRoot, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

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
    failures.push(`${label} must not contain ${needle}`);
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

function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const expected = JSON.stringify(PHASE82_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
  }
}

function sectionBetween(text: string, heading: string): string {
  const startIndex = text.indexOf(heading);
  if (startIndex === -1) {
    return "";
  }

  const nextHeadingIndex = text.indexOf("\n## ", startIndex + heading.length);
  if (nextHeadingIndex === -1) {
    return text.slice(startIndex);
  }
  return text.slice(startIndex, nextHeadingIndex);
}

function countSupportRows(section: string, term: string): number {
  const escapedTerm = term.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const rowPattern = new RegExp(`^\\|\\s+\`${escapedTerm}\`\\s+\\|`, "gm");
  return section.match(rowPattern)?.length ?? 0;
}

function parseClaimMatrixRows(text: string): ClaimMatrixRow[] {
  const section = sectionBetween(text, "## Claim-To-Evidence Matrix");
  const rows: ClaimMatrixRow[] = [];

  for (const line of section.split("\n")) {
    if (!line.startsWith("|")) {
      continue;
    }
    if (line.includes("Statement | Support term") || line.includes("---")) {
      continue;
    }

    const cells = line
      .split("|")
      .slice(1, -1)
      .map((cell) => cell.trim());
    if (cells.length !== MATRIX_COLUMNS.length) {
      continue;
    }

    rows.push({
      statement: cells[0] ?? "",
      supportTerm: cells[1] ?? "",
      currentStatus: cells[2] ?? "",
      verificationCommand: cells[4] ?? "",
    });
  }

  return rows;
}

function requireForbiddenClaimRows(rows: ClaimMatrixRow[], failures: string[]): void {
  for (const statement of NOT_ALLOWED_STATEMENTS) {
    const maybeRow = rows.find((row) => row.statement.includes(statement));
    if (maybeRow === undefined) {
      failures.push(`claim-to-evidence matrix missing row for ${statement}`);
      continue;
    }

    if (
      maybeRow.supportTerm !== "`deferred`" ||
      maybeRow.currentStatus !== "not allowed yet" ||
      !maybeRow.verificationCommand.includes("No default verifier may prove this in v1.8")
    ) {
      failures.push(
        `claim-to-evidence matrix row for ${statement} must remain deferred, not allowed yet, and outside default proof`,
      );
    }
  }
}

function verifyCanonicalBoundary(text: string, failures: string[]): void {
  requireContains(text, SURFACE_ID, "canonical boundary", failures);
  requireContains(
    text,
    "v1.8 is a boundary-setting milestone, not the production readiness milestone",
    "canonical boundary",
    failures,
  );

  const supportSection = sectionBetween(text, "## Support Terms");
  if (supportSection === "") {
    failures.push("support vocabulary missing Support Terms section");
  }
  for (const term of SUPPORT_TERMS) {
    const rowCount = countSupportRows(supportSection, term);
    if (rowCount !== 1) {
      failures.push(`support vocabulary must contain exactly one table row for ${term}`);
    }
  }

  requireNormalizedContains(
    text,
    MATRIX_COLUMNS.join(" | "),
    "claim-to-evidence matrix",
    failures,
  );
  requireContains(
    text,
    "Open Bitcoin defines gates required before a future production full-node readiness claim.",
    "claim-to-evidence matrix",
    failures,
  );
  requireForbiddenClaimRows(parseClaimMatrixRows(text), failures);
  for (const surface of DEFERRED_SURFACES) {
    requireContains(text, surface, "deferred inventory", failures);
  }
}

function verifyReleaseReadiness(text: string, failures: string[]): void {
  requireContains(text, "## v1.8 Production Claim Boundary", "release readiness", failures);
  requireContains(text, "production-claim-boundary.md", "release readiness", failures);
  requireContains(text, SURFACE_ID, "release readiness", failures);
  requireContains(
    text,
    "Phase 88 owns broad deterministic claim guardrails",
    "release readiness",
    failures,
  );
  for (const requirement of PHASE82_REQUIREMENTS) {
    requireContains(text, requirement, "release readiness", failures);
  }
  for (const historicalHeading of [
    "## v1.7 Full-Sync Soak and Recovery Hardening Claim Boundary Matrix",
    "## v1.6 Full-Sync Completion Claim Boundary Matrix",
    "## v1.5 Unattended Operation Claim Boundary Matrix",
  ]) {
    requireContains(text, historicalHeading, "release readiness", failures);
  }
}

function verifyDeviations(text: string, failures: string[]): void {
  requireContains(text, "### v1.8 Production Claim Boundary", "deferred inventory", failures);
  for (const requirement of PHASE82_REQUIREMENTS) {
    requireContains(text, requirement, "deferred inventory", failures);
  }
  for (const surface of DEFERRED_SURFACES) {
    requireContains(text, surface, "deferred inventory", failures);
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

  if (!Array.isArray(parsed.surfaces)) {
    failures.push("parity root surfaces must be an array");
  } else {
    const surface = parsed.surfaces.find((entry) => {
      const maybeSurface = entry as ParitySurface;
      return maybeSurface.name === SURFACE_ID;
    }) as ParitySurface | undefined;
    if (surface?.status !== "done") {
      failures.push(`parity root surfaces missing done ${SURFACE_ID}`);
    }
  }

  const checklistSurfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(checklistSurfaces)) {
    failures.push("parity root checklist.surfaces must be an array");
  } else {
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

  const auditEntry = parsed.audit?.v1_8_production_claim_boundary as AuditEntry | undefined;
  if (auditEntry?.path !== "production-claim-boundary.md" || auditEntry.status !== "done") {
    failures.push("parity root audit.v1_8_production_claim_boundary is missing or incomplete");
    return;
  }
  requireExactRequirements(
    auditEntry.requirements,
    "audit.v1_8_production_claim_boundary.requirements",
    failures,
  );
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(
      auditEntry.evidence,
      "audit.v1_8_production_claim_boundary.evidence",
      evidence,
      failures,
    );
  }
}

function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
  const checklist = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(checklist, SURFACE_ID, "human checklist", failures);
  requireContains(checklist, "production-claim-boundary.md", "human checklist", failures);
  for (const requirement of PHASE82_REQUIREMENTS) {
    requireContains(checklist, requirement, "human checklist", failures);
  }

  const parityReadme = texts.get("docs/parity/README.md") ?? "";
  requireContains(parityReadme, "v1.8 production claim boundary", "parity README", failures);
  requireContains(parityReadme, "production-claim-boundary.md", "parity README", failures);
  requireContains(parityReadme, "v1.7", "parity README", failures);
  requireContains(parityReadme, "historical evidence", "parity README", failures);
  requireNotContains(parityReadme, MATRIX_COLUMNS.join(" | "), "parity README", failures);

  const readme = texts.get("README.md") ?? "";
  requireContains(readme, "docs/parity/production-claim-boundary.md", "README", failures);
  requireContains(
    readme,
    "v1.8 defines the support terms and evidence gates required before a future production full-node readiness claim",
    "README",
    failures,
  );
  requireContains(readme, "does not claim production full-node readiness", "README", failures);
  requireContains(
    readme,
    "v1.7 remains historical source-built, explicit opt-in full-sync soak and recovery hardening evidence",
    "README",
    failures,
  );
  for (const surface of [
    "automatic support-bundle upload",
    "destructive repair",
    "broad production-node readiness",
  ]) {
    requireContains(readme, surface, "README", failures);
  }
  requireNotContains(readme, MATRIX_COLUMNS.join(" | "), "README", failures);

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  requireContains(runtimeGuide, "v1.8 production claim boundary", "runtime guide", failures);
  requireContains(runtimeGuide, "../parity/production-claim-boundary.md", "runtime guide", failures);
  for (const term of SUPPORT_TERMS) {
    requireContains(runtimeGuide, term, "runtime guide support vocabulary", failures);
  }
  requireContains(
    runtimeGuide,
    "not a production full-node readiness claim",
    "runtime guide",
    failures,
  );
  requireContains(
    runtimeGuide,
    "### Phase 80 v1.7 opt-in soak UAT matrix",
    "runtime guide",
    failures,
  );
  requireContains(
    runtimeGuide,
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "runtime guide",
    failures,
  );
  requireContains(
    runtimeGuide,
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "runtime guide",
    failures,
  );

  const operatorCatalog = texts.get("docs/parity/catalog/operator-runtime-release-hardening.md") ?? "";
  for (const needle of [
    "Phase 82 production claim boundary",
    SURFACE_ID,
    ...PHASE82_REQUIREMENTS,
    "automatic support-bundle upload",
    "destructive repair",
    "broad production-node readiness",
  ]) {
    requireContains(operatorCatalog, needle, "operator runtime catalog", failures);
  }

  const p2pCatalog = texts.get("docs/parity/catalog/p2p.md") ?? "";
  for (const needle of [
    "v1.8 production claim boundary",
    "production-claim-boundary.md",
    "inbound serving",
    "address relay",
    "block serving",
    "transaction relay",
    "compact block relay",
  ]) {
    requireContains(p2pCatalog, needle, "P2P catalog", failures);
  }

  const chainstateCatalog = texts.get("docs/parity/catalog/chainstate.md") ?? "";
  for (const needle of [
    "v1.8 production claim boundary",
    "production-claim-boundary.md",
    "destructive repair",
    "public-network CI",
    "release-blocking live sync",
    "broad production-node readiness",
  ]) {
    requireContains(chainstateCatalog, needle, "chainstate catalog", failures);
  }
}

function verifyForbiddenClaimLanguage(texts: Map<string, string>, failures: string[]): void {
  for (const file of CLAIM_FILES) {
    const text = texts.get(file) ?? "";
    for (const synonym of FORBIDDEN_NEAR_SYNONYMS) {
      if (text.includes(synonym)) {
        failures.push(`support vocabulary forbidden near-synonym in ${file}: ${synonym}`);
      }
    }
    for (const overclaim of EXACT_OVERCLAIMS) {
      if (text.includes(overclaim)) {
        failures.push(`exact overclaim in ${file}: ${overclaim}`);
      }
    }
  }
}

function verifyNoV18Manifest(repoRoot: string, failures: string[]): void {
  const parityDir = repoPath(repoRoot, "docs/parity");
  if (!existsSync(parityDir)) {
    return;
  }

  for (const entry of readdirSync(parityDir, { withFileTypes: true })) {
    if (!entry.isFile()) {
      continue;
    }
    const lowerName = entry.name.toLowerCase();
    const isV18 = lowerName.includes("v1.8") || lowerName.includes("v1-8");
    const isManifest = lowerName.includes("manifest") || lowerName.includes("evidence");
    if (isV18 && isManifest) {
      failures.push(`parity root must not add a v1.8 evidence manifest: docs/parity/${entry.name}`);
    }
  }
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  const executableText = executableVerifyText(text);

  for (const command of [PHASE82_TEST_COMMAND, PHASE82_CHECKER_COMMAND]) {
    requireContains(executableText, command, "verifier-order", failures);
  }

  const phase80Index = executableText.indexOf(PHASE80_CHECKER_COMMAND);
  const phase82TestIndex = executableText.indexOf(PHASE82_TEST_COMMAND);
  const phase82CheckerIndex = executableText.indexOf(PHASE82_CHECKER_COMMAND);
  const orderValid =
    phase80Index !== -1 && phase82TestIndex > phase80Index && phase82CheckerIndex > phase82TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 82 test and checker after Phase 80 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`verifier-order must not add forbidden Phase 82 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase82ProductionClaimBoundary();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 82 production claim boundary");
  }
}

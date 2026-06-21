#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE83_REPO_ROOT";
const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-8-support-matrix-issue-evidence";
const PHASE83_REQUIREMENTS = ["SUP-01", "SUP-02", "SUP-03", "SUP-04"] as const;
const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;
const FORBIDDEN_MATURITY_LABELS = [
  "best-effort",
  "beta",
  "production-grade",
  "production-ish",
  "partial production",
  "community-supported",
  "GA",
  "certified",
  "validated",
  "fully supported",
] as const;
const MATRIX_COLUMNS = [
  "Environment family",
  "Support term",
  "Evidence basis",
  "Default verification",
  "Opt-in UAT / manual validation",
  "Residual risk",
  "Next gate",
] as const;
const PLACEHOLDER_MATRIX_VALUES = [
  "evidence basis",
  "default verification",
  "opt-in UAT evidence",
  "opt-in UAT / manual validation",
  "residual risk",
  "next gate",
  "todo",
  "tbd",
  "n/a",
] as const;
const REQUIRED_ENVIRONMENT_FAMILIES = [
  "source-built install and repo verification",
  "repo-local operator command forms through Cargo and Bazel",
  "local deterministic runtime, status, config, RPC, and support-bundle surfaces",
  "operator dashboard and shipped operator convenience surfaces",
  "public-network mainnet activation, full-sync, stay-current, and soak evidence",
  "storage/datadir resource-bound evidence and recovery diagnosis",
  "live storage pressure and long-run resource behavior",
  "launchd/systemd service-supervision previews",
  "real launchd/systemd service-manager lifecycle",
  "migration dry-run",
  "migration apply, source service mutation, and source datadir rewrite",
  "support bundle and support forensics",
  "wallet current non-production slice",
  "production-funds wallet use and safety",
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "signed packaging or package-manager distribution",
  "Windows service integration",
  "hosted dashboards and GUI parity",
  "automatic support-bundle upload",
  "destructive repair",
  "public-network default checks, public-network CI, and release-blocking live sync",
  "broad production-node readiness",
] as const;
const REQUIRED_ISSUE_EVIDENCE = [
  "smallest useful redacted evidence set",
  "Unavailable: <reason>",
  "support-evidence.json",
  "support-evidence.md",
  "Relevant command output",
  "Bounded redacted logs",
  "configuration summary",
  "Service state",
  "resource-bound or resource-pressure evidence",
  "recovery/progress evidence",
  "sync status evidence",
  "version, commit, Rust, Cargo, Bun, and Bazel context",
  "Platform details",
  "exact repo-local command",
] as const;
const FORBIDDEN_EVIDENCE_ITEMS = [
  "wallet private material",
  "raw wallet files",
  "RPC cookies",
  "rpcpassword",
  "rpcauth",
  "raw datadirs",
  "unredacted logs",
  "raw unbounded logs",
  "full peer tables with sensitive local data",
  "automatic support-bundle upload",
] as const;
const RESIDUAL_RISK_SURFACES = [
  "dashboard pseudoterminal/raw-input repaint and input behavior",
  "closeout without a dedicated milestone audit artifact",
  "diagnosed-blocker closeout and fresh status supersession",
  "planning traceability correction during archive prep",
  "public-network full-sync, stay-current, and soak evidence",
  "real service-manager lifecycle evidence",
  "multi-day wall-clock soak evidence",
  "support-bundle forensics",
  "recovery diagnosis versus destructive repair",
  "production-scope non-claims",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/support-matrix.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "scripts/verify.sh",
] as const;
const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/support-matrix.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "scripts/verify.sh",
] as const;
const HUMAN_POINTER_FILES = TARGET_FILES.filter((file) => file !== "docs/parity/index.json");
const PHASE82_CHECKER_COMMAND = "bun run scripts/check-phase82-production-claim-boundary.ts";
const PHASE83_TEST_COMMAND =
  "bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts";
const PHASE83_CHECKER_COMMAND =
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts";
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-" + "smoke",
  "system" + "ctl",
  "launch" + "ctl",
  "sleep " + "259200",
  "automatic support-bundle upload" + " --",
  "destructive repair" + " --",
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

type MatrixTable = {
  header: string[];
  rows: MatrixRow[];
};

type MatrixRow = {
  cells: string[];
  environmentFamily: string;
  supportTerm: string;
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

export function checkPhase83SupportMatrixIssueEvidence(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  const supportMatrix = texts.get("docs/parity/support-matrix.md") ?? "";
  const matrixTable = verifySupportMatrix(supportMatrix, failures);
  verifyIssueEvidence(supportMatrix, failures);
  verifyResidualRiskRows(supportMatrix, failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanRoots(texts, failures);
  verifyForbiddenMaturityLabels(texts, matrixTable.rows, failures);
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
  const expected = JSON.stringify(PHASE83_REQUIREMENTS);
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

function splitMarkdownRow(line: string): string[] {
  return line
    .split("|")
    .slice(1, -1)
    .map((cell) => cell.trim());
}

function isSeparatorRow(cells: string[]): boolean {
  return cells.every((cell) => /^:?-{3,}:?$/.test(cell));
}

function parseMatrixTable(text: string): MatrixTable {
  const section = sectionBetween(text, "## Support Matrix");
  const tableRows = section
    .split("\n")
    .filter((line) => line.trim().startsWith("|"))
    .map(splitMarkdownRow);

  const maybeHeader = tableRows[0];
  const header = maybeHeader === undefined ? [] : maybeHeader;
  const rows = tableRows
    .slice(1)
    .filter((cells) => !isSeparatorRow(cells))
    .map((cells) => ({
      cells,
      environmentFamily: cells[0] ?? "",
      supportTerm: normalizeSupportTerm(cells[1] ?? ""),
    }));

  return { header, rows };
}

function normalizeSupportTerm(term: string): string {
  return term.trim().replace(/^`/, "").replace(/`$/, "");
}

function normalizeMatrixCell(cell: string): string {
  return cell.replace(/\s+/g, " ").trim();
}

function isPlaceholderMatrixValue(cell: string): boolean {
  const normalized = normalizeMatrixCell(cell).replaceAll("`", "").toLowerCase();
  return PLACEHOLDER_MATRIX_VALUES.includes(
    normalized as (typeof PLACEHOLDER_MATRIX_VALUES)[number],
  );
}

function findMatrixRowByFamily(rows: MatrixRow[], family: string): MatrixRow | undefined {
  const normalizedFamily = normalizeMatrixCell(family).toLowerCase();
  return rows.find(
    (row) => normalizeMatrixCell(row.environmentFamily).toLowerCase() === normalizedFamily,
  );
}

function verifySupportMatrix(text: string, failures: string[]): MatrixTable {
  requireContains(text, SURFACE_ID, "support matrix", failures);
  requireContains(text, "## Support Matrix", "support matrix", failures);

  const matrixTable = parseMatrixTable(text);
  const actualColumns = JSON.stringify(matrixTable.header);
  const expectedColumns = JSON.stringify(MATRIX_COLUMNS);
  if (actualColumns !== expectedColumns) {
    failures.push(
      `support matrix columns mismatch: expected ${expectedColumns}, got ${actualColumns}`,
    );
  }

  for (const row of matrixTable.rows) {
    if (row.cells.length !== MATRIX_COLUMNS.length || row.cells.some((cell) => cell === "")) {
      failures.push(`support matrix row has blank or malformed cells: ${row.environmentFamily}`);
    }
    for (const cell of row.cells.slice(2)) {
      if (isPlaceholderMatrixValue(cell)) {
        failures.push(
          `support matrix row "${row.environmentFamily}" has placeholder cell: ${cell}`,
        );
      }
    }
    if (!SUPPORT_TERMS.includes(row.supportTerm as (typeof SUPPORT_TERMS)[number])) {
      failures.push(
        `unsupported support term in support matrix row "${row.environmentFamily}": ${row.supportTerm}`,
      );
    }
  }

  for (const family of REQUIRED_ENVIRONMENT_FAMILIES) {
    const maybeRow = findMatrixRowByFamily(matrixTable.rows, family);
    if (maybeRow === undefined) {
      failures.push(`missing environment family in support matrix: ${family}`);
    }
  }

  return matrixTable;
}

function verifyIssueEvidence(text: string, failures: string[]): void {
  const issueSection = sectionBetween(text, "## Issue Evidence Checklist");
  if (issueSection === "") {
    failures.push("issue evidence missing Issue Evidence Checklist section");
    return;
  }

  for (const needle of REQUIRED_ISSUE_EVIDENCE) {
    requireContains(issueSection, needle, "issue evidence", failures);
  }
  requireNormalizedContains(
    issueSection,
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "issue evidence",
    failures,
  );
  requireNormalizedContains(
    issueSection,
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "issue evidence",
    failures,
  );
  requireContains(issueSection, "support bundle --output-dir=/tmp/open-bitcoin-support", "issue evidence", failures);

  const [requestedEvidence = "", maybeDoNotAttach = ""] = issueSection.split("### Do Not Attach");
  for (const forbidden of FORBIDDEN_EVIDENCE_ITEMS) {
    requireContains(maybeDoNotAttach, forbidden, "issue evidence Do Not Attach", failures);
    if (requestedEvidence.includes(forbidden)) {
      failures.push(`issue evidence must not request forbidden support evidence: ${forbidden}`);
    }
  }
}

function verifyResidualRiskRows(text: string, failures: string[]): void {
  const residualSection = sectionBetween(
    text,
    "## Carried-Forward Residual Risks And Manual Validation",
  );
  if (residualSection === "") {
    failures.push("residual risk missing carried-forward section");
    return;
  }

  for (const surface of RESIDUAL_RISK_SURFACES) {
    requireContains(residualSection, surface, "residual risk", failures);
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
  const auditEntry = parsed.audit?.v1_8_support_matrix_issue_evidence as AuditEntry | undefined;
  if (auditEntry?.path !== "support-matrix.md" || auditEntry.status !== "done") {
    failures.push("parity root audit.v1_8_support_matrix_issue_evidence is missing or incomplete");
    return;
  }
  requireExactRequirements(
    auditEntry.requirements,
    "audit.v1_8_support_matrix_issue_evidence.requirements",
    failures,
  );
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(
      auditEntry.evidence,
      "audit.v1_8_support_matrix_issue_evidence.evidence",
      evidence,
      failures,
    );
  }
}

function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
  verifyEntrypointPointers(texts, failures);
  verifyCatalogPointers(texts, failures);

  const matrixHeader = MATRIX_COLUMNS.join(" | ");
  for (const file of HUMAN_POINTER_FILES) {
    if (file === "docs/parity/support-matrix.md") {
      continue;
    }
    requireNotContains(texts.get(file) ?? "", matrixHeader, file, failures);
  }
}

function verifyEntrypointPointers(texts: Map<string, string>, failures: string[]): void {
  const checklist = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(checklist, SURFACE_ID, "human checklist", failures);
  requireContains(checklist, "support-matrix.md", "human checklist", failures);
  for (const requirement of PHASE83_REQUIREMENTS) {
    requireContains(checklist, requirement, "human checklist", failures);
  }

  const parityReadme = texts.get("docs/parity/README.md") ?? "";
  requireContains(parityReadme, SURFACE_ID, "parity README", failures);
  requireContains(parityReadme, "support-matrix.md", "parity README", failures);

  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  requireContains(releaseReadiness, "## v1.8 Support Matrix And Issue Evidence", "release readiness", failures);
  requireContains(releaseReadiness, SURFACE_ID, "release readiness", failures);
  requireContains(releaseReadiness, "support-matrix.md", "release readiness", failures);

  const deviations = texts.get("docs/parity/deviations-and-unknowns.md") ?? "";
  requireContains(deviations, SURFACE_ID, "deviations", failures);
  requireContains(deviations, "support-matrix.md", "deviations", failures);

  const readme = texts.get("README.md") ?? "";
  requireContains(readme, "docs/parity/support-matrix.md", "README", failures);

  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  requireContains(runtimeGuide, "../parity/support-matrix.md", "runtime guide", failures);
  requireContains(runtimeGuide, "Unavailable: <reason>", "runtime guide", failures);
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
}

function verifyCatalogPointers(texts: Map<string, string>, failures: string[]): void {
  const operatorCatalog = texts.get("docs/parity/catalog/operator-runtime-release-hardening.md") ?? "";
  for (const needle of [SURFACE_ID, ...PHASE83_REQUIREMENTS, "support-matrix.md"]) {
    requireContains(operatorCatalog, needle, "operator runtime catalog", failures);
  }

  const p2pCatalog = texts.get("docs/parity/catalog/p2p.md") ?? "";
  for (const needle of [
    "support-matrix.md",
    "inbound serving",
    "address relay",
    "block serving",
    "transaction relay",
    "compact block relay",
  ]) {
    requireContains(p2pCatalog, needle, "P2P catalog", failures);
  }

  requireContains(
    texts.get("docs/parity/catalog/chainstate.md") ?? "",
    "support-matrix.md",
    "chainstate catalog",
    failures,
  );
  requireContains(
    texts.get("docs/parity/catalog/wallet.md") ?? "",
    "support-matrix.md",
    "wallet catalog",
    failures,
  );
  requireContains(
    texts.get("docs/parity/catalog/drop-in-audit-and-migration.md") ?? "",
    "support-matrix.md",
    "migration catalog",
    failures,
  );
}

function verifyForbiddenMaturityLabels(
  texts: Map<string, string>,
  rows: MatrixRow[],
  failures: string[],
): void {
  for (const row of rows) {
    if (FORBIDDEN_MATURITY_LABELS.includes(row.supportTerm as never)) {
      failures.push(
        `unsupported support term in support matrix row "${row.environmentFamily}": ${row.supportTerm}`,
      );
    }
  }

  for (const [file, text] of texts) {
    if (file === "scripts/verify.sh" || file === "docs/parity/index.json") {
      continue;
    }
    for (const label of FORBIDDEN_MATURITY_LABELS) {
      if (containsForbiddenMaturityLabel(text, label)) {
        failures.push(`forbidden maturity label in ${file}: ${label}`);
      }
    }
  }
}

function containsForbiddenMaturityLabel(text: string, label: string): boolean {
  const supportLabelPatterns = [
    `\`${label}\``,
    `| ${label} |`,
    `${label} support`,
    `support label ${label}`,
    `support term ${label}`,
  ];
  return supportLabelPatterns.some((pattern) => text.includes(pattern));
}

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  requireContains(text, PHASE83_TEST_COMMAND, "verifier-order", failures);
  requireContains(text, PHASE83_CHECKER_COMMAND, "verifier-order", failures);

  const executableText = executableVerifyText(text);
  requireContains(executableText, PHASE83_TEST_COMMAND, "verifier-order", failures);
  requireContains(executableText, PHASE83_CHECKER_COMMAND, "verifier-order", failures);

  const phase82CheckerIndex = executableText.indexOf(PHASE82_CHECKER_COMMAND);
  const phase83TestIndex = executableText.indexOf(PHASE83_TEST_COMMAND);
  const phase83CheckerIndex = executableText.indexOf(PHASE83_CHECKER_COMMAND);
  const orderValid =
    phase82CheckerIndex !== -1 &&
    phase83TestIndex > phase82CheckerIndex &&
    phase83CheckerIndex > phase83TestIndex;

  if (!orderValid) {
    failures.push("verifier-order requires executed Phase 83 test and checker after Phase 82 checker");
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`verifier-order must not add forbidden Phase 83 default command text: ${forbidden}`);
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase83SupportMatrixIssueEvidence();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  } else {
    console.log("validated Phase 83 support matrix issue evidence");
  }
}

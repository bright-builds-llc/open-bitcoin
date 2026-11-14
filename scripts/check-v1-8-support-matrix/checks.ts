import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, PHASE83_REQUIREMENTS, SUPPORT_TERMS, FORBIDDEN_MATURITY_LABELS, MATRIX_COLUMNS, PLACEHOLDER_MATRIX_VALUES, REQUIRED_ENVIRONMENT_FAMILIES, REQUIRED_ISSUE_EVIDENCE, FORBIDDEN_EVIDENCE_ITEMS, RESIDUAL_RISK_SURFACES, REQUIRED_EVIDENCE, TARGET_FILES, HUMAN_POINTER_FILES, PHASE82_CHECKER_COMMAND, PHASE83_TEST_COMMAND, PHASE83_CHECKER_COMMAND, FORBIDDEN_VERIFY_STRINGS } from "./constants.ts";
import type { AuditEntry, ChecklistSurface, MatrixTable, MatrixRow, ParityIndex, ParitySurface } from "./constants.ts";
import { verifyEntrypointPointers, verifyCatalogPointers, verifyForbiddenMaturityLabels, containsForbiddenMaturityLabel, executableVerifyText, verifyVerifierWiring } from "./parity.ts";
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

export function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

export function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

export function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

export function requireNormalizedContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!normalizeWhitespace(text).includes(normalizeWhitespace(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
  }
}

export function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain ${needle}`);
  }
}

export function requireArrayIncludes(
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

export function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
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

export function sectionBetween(text: string, heading: string): string {
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

export function splitMarkdownRow(line: string): string[] {
  return line
    .split("|")
    .slice(1, -1)
    .map((cell) => cell.trim());
}

export function isSeparatorRow(cells: string[]): boolean {
  return cells.every((cell) => /^:?-{3,}:?$/.test(cell));
}

export function parseMatrixTable(text: string): MatrixTable {
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

export function normalizeSupportTerm(term: string): string {
  return term.trim().replace(/^`/, "").replace(/`$/, "");
}

export function normalizeMatrixCell(cell: string): string {
  return cell.replace(/\s+/g, " ").trim();
}

export function isPlaceholderMatrixValue(cell: string): boolean {
  const normalized = normalizeMatrixCell(cell).replaceAll("`", "").toLowerCase();
  return PLACEHOLDER_MATRIX_VALUES.includes(
    normalized as (typeof PLACEHOLDER_MATRIX_VALUES)[number],
  );
}

export function findMatrixRowByFamily(rows: MatrixRow[], family: string): MatrixRow | undefined {
  const normalizedFamily = normalizeMatrixCell(family).toLowerCase();
  return rows.find(
    (row) => normalizeMatrixCell(row.environmentFamily).toLowerCase() === normalizedFamily,
  );
}

export function verifySupportMatrix(text: string, failures: string[]): MatrixTable {
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

export function verifyIssueEvidence(text: string, failures: string[]): void {
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

export function verifyResidualRiskRows(text: string, failures: string[]): void {
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

export function verifyParityIndex(text: string, failures: string[]): void {
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

export function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
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

export function verifyChecklistSurface(parsed: ParityIndex, failures: string[]): void {
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

export function verifyAuditEntry(parsed: ParityIndex, failures: string[]): void {
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

export function verifyHumanRoots(texts: Map<string, string>, failures: string[]): void {
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

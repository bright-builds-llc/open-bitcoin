import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, PHASE83_REQUIREMENTS, SUPPORT_TERMS, FORBIDDEN_MATURITY_LABELS, MATRIX_COLUMNS, PLACEHOLDER_MATRIX_VALUES, REQUIRED_ENVIRONMENT_FAMILIES, REQUIRED_ISSUE_EVIDENCE, FORBIDDEN_EVIDENCE_ITEMS, RESIDUAL_RISK_SURFACES, REQUIRED_EVIDENCE, TARGET_FILES, HUMAN_POINTER_FILES, PHASE82_CHECKER_COMMAND, PHASE83_TEST_COMMAND, PHASE83_CHECKER_COMMAND, FORBIDDEN_VERIFY_STRINGS } from "./constants.ts";
import type { AuditEntry, ChecklistSurface, MatrixTable, MatrixRow, ParityIndex, ParitySurface } from "./constants.ts";
import { checkPhase83SupportMatrixIssueEvidence, readText, normalizeWhitespace, requireContains, requireNormalizedContains, requireNotContains, requireArrayIncludes, requireExactRequirements, sectionBetween, splitMarkdownRow, isSeparatorRow, parseMatrixTable, normalizeSupportTerm, normalizeMatrixCell, isPlaceholderMatrixValue, findMatrixRowByFamily, verifySupportMatrix, verifyIssueEvidence, verifyResidualRiskRows, verifyParityIndex, verifyTopLevelSurface, verifyChecklistSurface, verifyAuditEntry, verifyHumanRoots } from "./checks.ts";
export function verifyEntrypointPointers(texts: Map<string, string>, failures: string[]): void {
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

export function verifyCatalogPointers(texts: Map<string, string>, failures: string[]): void {
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

export function verifyForbiddenMaturityLabels(
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

export function containsForbiddenMaturityLabel(text: string, label: string): boolean {
  const supportLabelPatterns = [
    `\`${label}\``,
    `| ${label} |`,
    `${label} support`,
    `support label ${label}`,
    `support term ${label}`,
  ];
  return supportLabelPatterns.some((pattern) => text.includes(pattern));
}

export function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

export function verifyVerifierWiring(text: string, failures: string[]): void {
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

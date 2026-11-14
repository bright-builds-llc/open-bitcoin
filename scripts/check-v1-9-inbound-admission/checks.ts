import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, DEFAULT_REPO_ROOT, SURFACE_ID, AUDIT_KEY, PHASE88_CHECKER_COMMAND, PHASE90_TEST_COMMAND, PHASE90_CHECKER_COMMAND, PHASE90_REQUIREMENTS, TARGET_FILES, REQUIRED_EVIDENCE, REQUIRED_UAT_COMMANDS, REQUIRED_EVIDENCE_LABELS, REQUIRED_CATALOG_ANCHORS, REQUIRED_BREADCRUMB_MAPPINGS, FORBIDDEN_VERIFY_STRINGS, PUBLIC_DEFAULT_CLAIMS, PRODUCTION_READY_CLAIMS, ALLOWED_SCOPE_TERMS, COMMAND_PREFIXES } from "./constants.ts";
import type { AuditEntry, BreadcrumbIndex, BreadcrumbGroup, ChecklistSurface, ParityIndex, ParitySurface, TargetFile } from "./constants.ts";
import { verifyNoClaimBoundary, contextUnits, sentenceUnits, verifyNoPublicDefaultClaim, verifyNoProductionReadinessClaim, isScopedAllowance } from "./parity.ts";
export function checkPhase90InboundListenerAdmission(
  maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV],
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanDocs(texts, failures);
  verifySourceBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);

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

export function normalizeShellCommand(text: string): string {
  return normalizeWhitespace(text.replace(/\\\s*/g, " "));
}

export function normalizedLower(text: string): string {
  return normalizeWhitespace(text).toLowerCase();
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
  if (!normalizedLower(text).includes(normalizedLower(needle))) {
    failures.push(`${label} missing required normalized text: ${needle}`);
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
  const expected = JSON.stringify(PHASE90_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
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
  const auditEntry = parsed.audit?.[AUDIT_KEY] as AuditEntry | undefined;
  if (auditEntry?.path !== "catalog/p2p.md" || auditEntry.status !== "done") {
    failures.push(`parity root audit.${AUDIT_KEY} is missing or incomplete`);
    return;
  }
  requireExactRequirements(auditEntry.requirements, `audit.${AUDIT_KEY}.requirements`, failures);
  for (const evidence of REQUIRED_EVIDENCE) {
    requireArrayIncludes(auditEntry.evidence, `audit.${AUDIT_KEY}.evidence`, evidence, failures);
  }
}

export function verifyHumanDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  verifyRuntimeGuideCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifyEvidenceLabels(texts, failures);
  verifyParityDocs(texts, failures);
}

export function verifyRuntimeGuideCommands(text: string, failures: string[]): void {
  const commandUnits = shellCommandUnits(text);
  for (const command of REQUIRED_UAT_COMMANDS) {
    const commandFound = commandUnits.some((unit) =>
      command.required.every((required) => unit.includes(normalizeShellCommand(required))),
    );
    if (!commandFound) {
      failures.push(`UAT command missing ${command.label}: ${command.required.join(" ")}`);
    }
  }
}

export function shellCommandUnits(text: string): string[] {
  const units: string[] = [];
  let currentLines: string[] = [];

  for (const rawLine of text.replaceAll("\r\n", "\n").split("\n")) {
    const line = rawLine.trim();
    const lineStartsCommand = COMMAND_PREFIXES.some((prefix) => line.startsWith(prefix));

    if (lineStartsCommand) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [line];
      continue;
    }

    if (currentLines.length === 0) {
      continue;
    }

    if (line.length === 0 || line.startsWith("```")) {
      pushCurrentShellCommandUnit(currentLines, units);
      currentLines = [];
      continue;
    }

    currentLines.push(line);
  }

  pushCurrentShellCommandUnit(currentLines, units);
  return units;
}

export function pushCurrentShellCommandUnit(currentLines: string[], units: string[]): void {
  if (currentLines.length === 0) {
    return;
  }

  units.push(normalizeShellCommand(currentLines.join("\n")));
}

export function verifyEvidenceLabels(texts: Map<TargetFile, string>, failures: string[]): void {
  const corpus = [
    texts.get("docs/operator/runtime-guide.md") ?? "",
    texts.get("docs/architecture/config-precedence.md") ?? "",
    texts.get("docs/architecture/status-snapshot.md") ?? "",
    texts.get("docs/architecture/operator-observability.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
  ].join("\n");

  for (const label of REQUIRED_EVIDENCE_LABELS) {
    requireNormalizedContains(corpus, label, "inbound evidence label", failures);
  }
}

export function verifyParityDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const p2pText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const checklistText = texts.get("docs/parity/checklist.md") ?? "";
  requireContains(p2pText, SURFACE_ID, "docs/parity/catalog/p2p.md", failures);
  requireContains(checklistText, SURFACE_ID, "docs/parity/checklist.md", failures);
  for (const requirement of PHASE90_REQUIREMENTS) {
    requireContains(p2pText, requirement, "docs/parity/catalog/p2p.md", failures);
    requireContains(checklistText, requirement, "docs/parity/checklist.md", failures);
  }
  for (const anchor of REQUIRED_CATALOG_ANCHORS) {
    requireContains(p2pText, anchor, "docs/parity/catalog/p2p.md", failures);
  }
}

export function verifySourceBreadcrumbs(text: string, failures: string[]): void {
  for (const mapping of REQUIRED_BREADCRUMB_MAPPINGS) {
    for (const file of mapping.files) {
      if (!text.includes(file)) {
        failures.push(`source breadcrumb mapping missing required Phase 90 file: ${file}`);
      }
    }
  }

  let parsed: BreadcrumbIndex;
  try {
    parsed = JSON.parse(text) as BreadcrumbIndex;
  } catch (error) {
    failures.push(`source breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }

  if (!Array.isArray(parsed.groups)) {
    failures.push("source breadcrumb groups must be an array");
    return;
  }

  for (const mapping of REQUIRED_BREADCRUMB_MAPPINGS) {
    verifyBreadcrumbMapping(parsed.groups, mapping, failures);
  }
}

export function verifyBreadcrumbMapping(
  groups: unknown[],
  mapping: (typeof REQUIRED_BREADCRUMB_MAPPINGS)[number],
  failures: string[],
): void {
  for (const file of mapping.files) {
    const maybeGroup = groups.find((entry) => {
      const group = entry as BreadcrumbGroup;
      return group.label === mapping.label && Array.isArray(group.files) && group.files.includes(file);
    }) as BreadcrumbGroup | undefined;
    if (maybeGroup === undefined) {
      failures.push(`source breadcrumb mapping missing ${mapping.label}: ${file}`);
      continue;
    }

    const actual = JSON.stringify(maybeGroup.breadcrumbs ?? []);
    const expected = JSON.stringify(mapping.breadcrumbs);
    if (actual !== expected) {
      failures.push(`source breadcrumb mapping mismatch for ${file}: expected ${expected}, got ${actual}`);
    }
  }
}

export function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

export function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [PHASE90_TEST_COMMAND, PHASE90_CHECKER_COMMAND]) {
    requireContains(text, command, "verifier-order", failures);
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "test Phase 90 inbound listener admission checker" ${PHASE90_TEST_COMMAND}`,
    "verifier-order",
    failures,
  );
  requireContains(
    executableText,
    `run_step "check Phase 90 inbound listener admission" ${PHASE90_CHECKER_COMMAND}`,
    "verifier-order",
    failures,
  );
  verifyVerifierOrder(executableText, failures);
  verifyVerifierBoundary(executableText, failures);
}

export function verifyVerifierOrder(executableText: string, failures: string[]): void {
  const phase88Index = executableText.indexOf(PHASE88_CHECKER_COMMAND);
  const phase90TestIndex = executableText.indexOf(PHASE90_TEST_COMMAND);
  const phase90CheckerIndex = executableText.indexOf(PHASE90_CHECKER_COMMAND);
  const pureCoreIndex = executableText.indexOf("bash scripts/check-pure-core-deps.sh");
  const orderValid =
    phase88Index !== -1 &&
    phase90TestIndex > phase88Index &&
    phase90CheckerIndex > phase90TestIndex &&
    pureCoreIndex > phase90CheckerIndex;

  if (!orderValid) {
    failures.push(
      "verifier-order requires executed Phase 90 test and checker after Phase 88 and before pure-core checks",
    );
  }
}

export function verifyVerifierBoundary(executableText: string, failures: string[]): void {
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`default verifier boundary must not add forbidden Phase 90 command text: ${forbidden}`);
    }
  }
}

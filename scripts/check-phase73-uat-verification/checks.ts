import path from "node:path";
import { REPO_ROOT_OVERRIDE_ENV, maybeRepoRoot, REPO_ROOT, PHASE_DIR, PHASE73_SURFACE_ID, PHASE73_REGRESSION_TEST_COMMAND, PHASE73_CHECKER_COMMAND, PLAN_FILES, REQUIREMENT_IDS, REQUIRED_VER02_BEHAVIORS, HERMETIC_COVERAGE_FILES, PARITY_CLOSEOUT_FILES, REQUIRED_UAT_MATRIX_DOC_STRINGS, REQUIRED_PARITY_ROOT_STRINGS, REQUIRED_CLOSEOUT_FILES, REQUIRED_BREADCRUMB_FILES, REQUIRED_DEFERRED_SCOPE_STRINGS, FORBIDDEN_PHASE73_CLAIM_STRINGS, FORBIDDEN_VERIFY_STRINGS, VER02_COVERAGE } from "./constants.ts";
import type { Ver02Behavior, CoverageAnchor, CoverageEntry, SourceBreadcrumbFileGroup, SourceBreadcrumbs, ParityIndex, ParityChecklist } from "./constants.ts";
import { verifyParityAndEvidenceCloseout, main } from "./parity.ts";
export function repoPath(relativePath: string): string {
  return path.join(REPO_ROOT, relativePath);
}

export async function readText(relativePath: string, failures: string[]): Promise<string> {
  const file = Bun.file(repoPath(relativePath));
  if (!(await file.exists())) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  const parts = [await file.text()];
  if (relativePath === "packages/open-bitcoin-node/src/sync/tests.rs") {
    for (const child of [
      "packages/open-bitcoin-node/src/sync/tests/block_requests.rs",
      "packages/open-bitcoin-node/src/sync/tests/block_response/connection_progress.rs",
      "packages/open-bitcoin-node/src/sync/tests/block_response/response_failures.rs",
      "packages/open-bitcoin-node/src/sync/tests/bounded_unattended_runtime.rs",
      "packages/open-bitcoin-node/src/sync/tests/phase70_peer/failure_classification.rs",
      "packages/open-bitcoin-node/src/sync/tests/phase70_peer/fallback_rotation.rs",
      "packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs",
      "packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs",
      "packages/open-bitcoin-node/src/sync/tests/restart_resume_matrix.rs",
      "packages/open-bitcoin-node/src/sync/tests/stay_current_persistence.rs",
      "packages/open-bitcoin-node/src/sync/tests/synthetic_long_chain.rs",
    ]) {
      const childFile = Bun.file(repoPath(child));
      if (await childFile.exists()) parts.push(await childFile.text());
    }
  }
  return parts.join("\n");
}

export async function readJoined(files: readonly string[], failures: string[]): Promise<string> {
  const parts = [];
  for (const file of files) {
    parts.push(await readText(file, failures));
  }

  return parts.join("\n");
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

export function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain default verification command or timing gate: ${needle}`);
  }
}

export async function requireFileExists(relativePath: string, failures: string[]): Promise<void> {
  const file = Bun.file(repoPath(relativePath));
  if (!(await file.exists())) {
    failures.push(`missing required file: ${relativePath}`);
  }
}

export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export function truncateProcessOutput(text: string): string {
  const maxLength = 1_200;
  if (text.length <= maxLength) {
    return text;
  }

  return `${text.slice(0, maxLength)}...`;
}

export function verifyCoverageBehaviors(failures: string[]): void {
  const observed = new Set(VER02_COVERAGE.map((entry) => entry.behavior));
  if (observed.size !== VER02_COVERAGE.length) {
    failures.push("VER02_COVERAGE must not contain duplicate behavior keys");
  }

  for (const behavior of REQUIRED_VER02_BEHAVIORS) {
    if (!observed.has(behavior)) {
      failures.push(`VER02_COVERAGE missing behavior: ${behavior}`);
    }
  }

  for (const entry of VER02_COVERAGE) {
    if (!REQUIRED_VER02_BEHAVIORS.includes(entry.behavior)) {
      failures.push(`VER02_COVERAGE contains unexpected behavior: ${entry.behavior}`);
    }
  }

  if (VER02_COVERAGE.length !== REQUIRED_VER02_BEHAVIORS.length) {
    failures.push("VER02_COVERAGE must contain exactly the required VER-02 behavior keys");
  }
}

export async function verifyRequirements(failures: string[]): Promise<void> {
  const planText = await readJoined(PLAN_FILES, failures);
  for (const requirementId of REQUIREMENT_IDS) {
    requireContains(planText, requirementId, `${PHASE_DIR}/73-*-PLAN.md`, failures);
  }
}

export async function verifyCoverageAnchors(failures: string[]): Promise<void> {
  for (const entry of VER02_COVERAGE) {
    for (const anchor of entry.anchors) {
      const text = await readText(anchor.file, failures);
      for (const needle of anchor.needles) {
        requireContains(text, needle, `${entry.behavior} in ${anchor.file}`, failures);
      }
    }
  }
}

export async function verifyCoverageMap(failures: string[]): Promise<void> {
  verifyCoverageBehaviors(failures);
  verifyHermeticCoverageFiles(failures);
  await verifyCoverageAnchors(failures);
}

export function verifyHermeticCoverageFiles(failures: string[]): void {
  const allowed = new Set<string>(HERMETIC_COVERAGE_FILES);
  for (const entry of VER02_COVERAGE) {
    for (const anchor of entry.anchors) {
      if (!allowed.has(anchor.file)) {
        failures.push(`${entry.behavior} uses non-hermetic coverage file: ${anchor.file}`);
      }
    }
  }
}

export function verifyRequirementIds(
  maybeRequirements: unknown,
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(maybeRequirements)) {
    failures.push(`${label} must contain a requirements array`);
    return;
  }

  const requirements = maybeRequirements.filter((requirement) => typeof requirement === "string");
  if (requirements.length !== maybeRequirements.length) {
    failures.push(`${label} requirements must contain only strings`);
    return;
  }

  for (const requirementId of REQUIREMENT_IDS) {
    if (!requirements.includes(requirementId)) {
      failures.push(`${label} missing required Phase 73 requirement: ${requirementId}`);
    }
  }

  if (requirements.length !== REQUIREMENT_IDS.length) {
    failures.push(`${label} must list exactly the Phase 73 requirement IDs`);
  }
}

export async function verifyParityIndexRequirements(failures: string[]): Promise<void> {
  const indexText = await readText("docs/parity/index.json", failures);
  if (indexText.length === 0) {
    return;
  }

  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(indexText) as ParityIndex;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    failures.push(`docs/parity/index.json is invalid JSON: ${message}`);
    return;
  }

  if (!isRecord(parsed) || !isRecord(parsed.checklist)) {
    failures.push("docs/parity/index.json must contain a checklist object");
    return;
  }

  const checklist = parsed.checklist as ParityChecklist;
  if (!Array.isArray(checklist.surfaces)) {
    failures.push("docs/parity/index.json checklist must contain a surfaces array");
    return;
  }

  const maybeSurface = checklist.surfaces.find(
    (surface) => isRecord(surface) && surface.id === PHASE73_SURFACE_ID,
  );
  if (!isRecord(maybeSurface)) {
    failures.push(`docs/parity/index.json missing checklist surface: ${PHASE73_SURFACE_ID}`);
    return;
  }

  verifyRequirementIds(
    maybeSurface.requirements,
    `docs/parity/index.json ${PHASE73_SURFACE_ID}`,
    failures,
  );
}

export async function verifyChecklistRequirements(failures: string[]): Promise<void> {
  const checklistText = await readText("docs/parity/checklist.md", failures);
  const maybeSurfaceLine = checklistText
    .split("\n")
    .find((line) => line.includes(`| \`${PHASE73_SURFACE_ID}\` |`));

  if (maybeSurfaceLine === undefined) {
    failures.push(`docs/parity/checklist.md missing surface row: ${PHASE73_SURFACE_ID}`);
    return;
  }

  const expectedRequirements = REQUIREMENT_IDS.map((requirementId) => `\`${requirementId}\``).join(
    ", ",
  );
  requireContains(
    maybeSurfaceLine,
    expectedRequirements,
    "docs/parity/checklist.md Phase 73 row",
    failures,
  );
}

export async function verifyParityLedgerRequirements(failures: string[]): Promise<void> {
  await verifyParityIndexRequirements(failures);
  await verifyChecklistRequirements(failures);
}

export async function verifyUatMatrixDocs(failures: string[]): Promise<void> {
  const runtimeGuide = await readText("docs/operator/runtime-guide.md", failures);
  for (const needle of REQUIRED_UAT_MATRIX_DOC_STRINGS) {
    requireContains(runtimeGuide, needle, "docs/operator/runtime-guide.md", failures);
  }
}

export async function verifyVerifyScript(failures: string[]): Promise<void> {
  const verifyScript = await readText("scripts/verify.sh", failures);
  const phase72 = "bun run scripts/check-phase72-observability-evidence.ts";
  requireContains(verifyScript, phase72, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE73_REGRESSION_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE73_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const phase72Index = verifyScript.indexOf(phase72);
  const phase73TestIndex = verifyScript.indexOf(PHASE73_REGRESSION_TEST_COMMAND);
  const phase73Index = verifyScript.indexOf(PHASE73_CHECKER_COMMAND);
  if (
    phase72Index === -1 ||
    phase73TestIndex === -1 ||
    phase73Index === -1 ||
    phase73TestIndex < phase72Index ||
    phase73Index < phase73TestIndex
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 73 regression test and hardened checker after the Phase 72 checker",
    );
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

export async function verifyParityRootText(failures: string[]): Promise<void> {
  for (const file of PARITY_CLOSEOUT_FILES) {
    const text = await readText(file, failures);
    for (const needle of REQUIRED_PARITY_ROOT_STRINGS[file]) {
      requireContains(text, needle, file, failures);
    }
  }
}

export async function verifyDeferredScopeNonClaims(failures: string[]): Promise<void> {
  const closeoutText = await readJoined(PARITY_CLOSEOUT_FILES, failures);
  for (const needle of REQUIRED_DEFERRED_SCOPE_STRINGS) {
    requireContains(closeoutText, needle, "Phase 73 parity closeout roots", failures);
  }

  for (const forbidden of FORBIDDEN_PHASE73_CLAIM_STRINGS) {
    if (closeoutText.includes(forbidden)) {
      failures.push(`Phase 73 parity closeout roots must not claim: ${forbidden}`);
    }
  }
}

export async function verifyCloseoutFilesExist(failures: string[]): Promise<void> {
  for (const file of REQUIRED_CLOSEOUT_FILES) {
    await requireFileExists(file, failures);
  }
}

export async function verifySourceBreadcrumbRegistry(failures: string[]): Promise<void> {
  const breadcrumbText = await readText("docs/parity/source-breadcrumbs.json", failures);
  if (breadcrumbText.length === 0) {
    return;
  }

  let parsed: SourceBreadcrumbs;
  try {
    parsed = JSON.parse(breadcrumbText) as SourceBreadcrumbs;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    failures.push(`docs/parity/source-breadcrumbs.json is invalid JSON: ${message}`);
    return;
  }

  if (!isRecord(parsed) || !Array.isArray(parsed.groups)) {
    failures.push("docs/parity/source-breadcrumbs.json must contain a groups array");
    return;
  }

  const registeredFiles = new Set<string>();
  for (const [index, group] of parsed.groups.entries()) {
    const typedGroup = group as SourceBreadcrumbFileGroup;
    if (!isRecord(typedGroup) || !Array.isArray(typedGroup.files)) {
      failures.push(`docs/parity/source-breadcrumbs.json group ${index} must contain a files array`);
      continue;
    }

    for (const file of typedGroup.files) {
      if (typeof file !== "string") {
        failures.push(`docs/parity/source-breadcrumbs.json group ${index} contains a non-string file`);
        continue;
      }
      registeredFiles.add(file);
    }
  }

  for (const requiredFile of REQUIRED_BREADCRUMB_FILES) {
    if (!registeredFiles.has(requiredFile)) {
      failures.push(`docs/parity/source-breadcrumbs.json missing referenced Rust file: ${requiredFile}`);
    }
  }
}

export function verifyParityBreadcrumbChecker(failures: string[]): void {
  if (maybeRepoRoot !== undefined) {
    return;
  }

  const child = Bun.spawnSync(["bun", "run", "scripts/check-parity-breadcrumbs.ts", "--check"], {
    cwd: REPO_ROOT,
    stderr: "pipe",
    stdout: "pipe",
  });
  if (child.exitCode === 0) {
    return;
  }

  const decoder = new TextDecoder();
  const output = [decoder.decode(child.stdout), decoder.decode(child.stderr)]
    .filter((part) => part.trim().length > 0)
    .join("\n");
  const details = output.length > 0 ? `:\n${truncateProcessOutput(output)}` : "";
  failures.push(`scripts/check-parity-breadcrumbs.ts --check failed with exit code ${child.exitCode}${details}`);
}

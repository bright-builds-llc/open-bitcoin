import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { DEFAULT_REPO_ROOT, SURFACE_ID, PHASE94_TEST_COMMAND, PHASE94_CHECKER_COMMAND, PHASE95_TEST_COMMAND, PHASE95_CHECKER_COMMAND, PURE_CORE_COMMAND, REQUIRED_PHASE95_REQUIREMENTS, PHASE_REQUIREMENTS, REQUIRED_V1_9_REQUIREMENTS, REQUIREMENT_PHASE_ASSIGNMENTS, ROADMAP_TRACEABILITY_ROWS, REQUIRED_KNOTS_ANCHORS, REQUIRED_PHASE95_EVIDENCE, REQUIRED_UAT_COMMANDS, REQUIRED_SUPPORT_REDACTION_ROOTS, FORBIDDEN_POSITIVE_CLAIMS, POSITIVE_CLAIM_VERBS, FORBIDDEN_VERIFY_STRINGS, CLAIM_SCAN_FILES, TARGET_FILES } from "./constants.ts";
import type { TargetFile, ChecklistSurface, ParityIndex, ParitySurface, CheckPhase95Options } from "./constants.ts";
import { extractRequirementIdsFromSurfaceRows, verifyRequirementsTable, verifyRoadmapTraceability, verifyRequirementCountsFromArrays, requirementIds, requireContains, contextUnits, sentenceUnits, normalizeWhitespace, normalizedLower, escapeRegExp } from "./parity.ts";
export function checkPhase95NetworkParticipationReleaseBoundary(
  options: CheckPhase95Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyKnotsAnchors(texts, failures);
  verifyNoClaimBoundary(texts, failures);
  verifyUatCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifySupportRedactionRoots(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyRequirementTraceability(texts, failures);

  return failures;
}

export function readText(repoRoot: string, relativePath: string, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`BOUND-03 missing required Phase 95 corpus file: ${relativePath}`);
    return "";
  }

  const parts = [readFileSync(absolutePath, "utf8")];
  if (relativePath === "packages/open-bitcoin-cli/src/operator/support/tests.rs") {
    const splitChild = path.join(
      repoRoot,
      "packages/open-bitcoin-cli/src/operator/support/tests/inbound.rs",
    );
    if (existsSync(splitChild)) parts.push(readFileSync(splitChild, "utf8"));
  }
  return parts.join("\n");
}

export function verifyParityIndex(text: string, failures: string[]): void {
  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(text) as ParityIndex;
  } catch (error) {
    failures.push(`BOUND-06 parity index JSON parse failed: ${String(error)}`);
    return;
  }

  verifyTopLevelSurface(parsed, failures);
  verifyPhaseRequirementSurfaces(parsed, failures);
  verifyPhase95ChecklistSurface(parsed, failures);
}

export function verifyTopLevelSurface(parsed: ParityIndex, failures: string[]): void {
  if (!Array.isArray(parsed.surfaces)) {
    failures.push("BOUND-06 parity index surfaces must be an array");
    return;
  }

  const surface = parsed.surfaces.find((entry) => {
    const maybeSurface = entry as ParitySurface;
    return maybeSurface.name === SURFACE_ID;
  }) as ParitySurface | undefined;
  if (surface?.status !== "done") {
    failures.push(`BOUND-06 parity index missing done surface: ${SURFACE_ID}`);
  }
}

export function verifyPhaseRequirementSurfaces(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    failures.push("BOUND-06 parity checklist surfaces must be an array");
    return;
  }

  for (const [surfaceId, expectedRequirements] of Object.entries(PHASE_REQUIREMENTS)) {
    const surface = surfaces.find((entry) => {
      const maybeSurface = entry as ChecklistSurface;
      return maybeSurface.id === surfaceId;
    }) as ChecklistSurface | undefined;
    if (surface?.status !== "done") {
      failures.push(`BOUND-06 parity checklist missing done v1.9 surface: ${surfaceId}`);
    }
    requireExactRequirements(
      surface?.requirements,
      expectedRequirements,
      `BOUND-06 parity checklist ${surfaceId}`,
      failures,
    );
  }
  verifyRequirementCountsFromArrays(
    Object.keys(PHASE_REQUIREMENTS)
      .map((surfaceId) => {
        const surface = surfaces.find((entry) => {
          const maybeSurface = entry as ChecklistSurface;
          return maybeSurface.id === surfaceId;
        }) as ChecklistSurface | undefined;
        return surface?.requirements;
      })
      .filter(Array.isArray)
      .flat() as string[],
    "BOUND-06 parity index v1.9 checklist surfaces",
    failures,
  );
}

export function verifyPhase95ChecklistSurface(parsed: ParityIndex, failures: string[]): void {
  const surfaces = parsed.checklist?.surfaces;
  if (!Array.isArray(surfaces)) {
    return;
  }

  const surface = surfaces.find((entry) => {
    const maybeSurface = entry as ChecklistSurface;
    return maybeSurface.id === SURFACE_ID;
  }) as ChecklistSurface | undefined;
  for (const evidence of REQUIRED_PHASE95_EVIDENCE) {
    requireArrayIncludes(surface?.evidence, `BOUND-06 ${SURFACE_ID}.evidence`, evidence, failures);
  }
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireArrayIncludes(
      surface?.upstream?.sources,
      `BOUND-02 ${SURFACE_ID}.upstream.sources`,
      anchor,
      failures,
    );
  }
}

export function requireExactRequirements(
  value: unknown,
  expected: readonly string[],
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const wanted = JSON.stringify(expected);
  if (actual !== wanted) {
    failures.push(`${label} requirements mismatch: expected ${wanted}, got ${actual}`);
  }
}

export function requireArrayIncludes(
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

export function verifyKnotsAnchors(texts: Map<TargetFile, string>, failures: string[]): void {
  const catalogText = texts.get("docs/parity/catalog/p2p.md") ?? "";
  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    requireContains(catalogText, anchor, "BOUND-02 P2P catalog Knots anchors", failures);
    requireContains(
      releaseReadiness,
      anchor.replace("packages/bitcoin-knots/src/", ""),
      "BOUND-02 release-readiness Knots anchor rollup",
      failures,
    );
  }
}

export function verifyNoClaimBoundary(texts: Map<TargetFile, string>, failures: string[]): void {
  for (const file of CLAIM_SCAN_FILES) {
    const text = texts.get(file) ?? "";
    for (const unit of contextUnits(text)) {
      verifyNoForbiddenClaim(file, unit, failures);
    }
  }
}

export function verifyNoForbiddenClaim(file: string, unit: string, failures: string[]): void {
  if (isExplicitDeferredMatrixRow(unit)) {
    return;
  }

  const lower = normalizedLower(unit);
  for (const claim of FORBIDDEN_POSITIVE_CLAIMS) {
    if (lower.includes(claim) && isPositiveClaim(lower, claim)) {
      failures.push(`BOUND-01 forbidden v1.9 network participation claim in ${file}: ${unit}`);
    }
  }
}

export function isPositiveClaim(lowerUnit: string, claim: string): boolean {
  return (
    POSITIVE_CLAIM_VERBS.some((verb) => containsUnnegatedVerbClaim(lowerUnit, verb, claim))
    || [
      `${claim} support is enabled`,
      `${claim} support is supported`,
      `${claim} is available`,
      `${claim} is supported`,
      `${claim} is enabled`,
      `${claim} is complete`,
      `${claim} is achieved`,
      `${claim} readiness is achieved`,
    ].some((phrase) => lowerUnit.includes(phrase))
  );
}

export function containsUnnegatedVerbClaim(lowerUnit: string, verb: string, claim: string): boolean {
  const phrase = `${verb} ${claim}`;
  let searchFrom = 0;
  while (searchFrom < lowerUnit.length) {
    const phraseIndex = lowerUnit.indexOf(phrase, searchFrom);
    if (phraseIndex === -1) {
      return false;
    }
    const prefix = lowerUnit.slice(Math.max(0, phraseIndex - 16), phraseIndex);
    if (!/(does not |do not |doesn't |don't |not )$/.test(prefix)) {
      return true;
    }
    searchFrom = phraseIndex + phrase.length;
  }
  return false;
}

export function isExplicitDeferredMatrixRow(unit: string): boolean {
  const lower = normalizedLower(unit);
  return (
    lower.startsWith("|")
    && lower.includes("| `deferred` |")
    && lower.includes("| not allowed yet |")
  );
}

export function verifyUatCommands(text: string, failures: string[]): void {
  for (const command of REQUIRED_UAT_COMMANDS) {
    requireContains(text, command, "BOUND-04 Phase 95 UAT command family", failures);
  }
}

export function verifySupportRedactionRoots(texts: Map<TargetFile, string>, failures: string[]): void {
  const supportText = [
    texts.get("packages/open-bitcoin-cli/src/operator/support/redaction.rs") ?? "",
    texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "",
  ].join("\n");

  for (const root of REQUIRED_SUPPORT_REDACTION_ROOTS) {
    requireContains(supportText, root, "BOUND-05 support redaction roots", failures);
  }
}

export function verifyVerifierWiring(text: string, failures: string[]): void {
  const maybeOrderBlock = text.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("BOUND-03 verifier-order missing VERIFY_COMMAND_ORDER block");
  } else {
    verifyOrderedCommands(
      maybeOrderBlock[1],
      [
        PHASE94_TEST_COMMAND,
        PHASE94_CHECKER_COMMAND,
        PHASE95_TEST_COMMAND,
        PHASE95_CHECKER_COMMAND,
      ],
      "BOUND-03 verifier-order printed commands must place Phase 95 immediately after Phase 94",
      failures,
    );
  }

  const executableText = executableVerifyText(text);
  requireContains(
    executableText,
    `run_step "Phase 95 network participation release boundary checker tests" ${PHASE95_TEST_COMMAND}`,
    "BOUND-03 executable verifier Phase 95 checker tests",
    failures,
  );
  requireContains(
    executableText,
    `run_step "Phase 95 network participation release boundary checker" ${PHASE95_CHECKER_COMMAND}`,
    "BOUND-03 executable verifier Phase 95 checker",
    failures,
  );
  requireContains(
    text,
    "Phase 94 is followed by Phase 95",
    "BOUND-03 verifier ordering comment",
    failures,
  );
  verifyOrderedCommands(
    executableText,
    [
      PHASE94_TEST_COMMAND,
      PHASE94_CHECKER_COMMAND,
      PHASE95_TEST_COMMAND,
      PHASE95_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "BOUND-03 executable verifier commands must run Phase 95 after Phase 94 and before pure-core checks",
    failures,
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    if (executableText.includes(forbidden)) {
      failures.push(`BOUND-03 default verifier boundary contains forbidden text: ${forbidden}`);
    }
  }
}

export function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

export function verifyOrderedCommands(
  text: string,
  commands: readonly string[],
  failure: string,
  failures: string[],
): void {
  let previousIndex = -1;
  for (const command of commands) {
    const currentIndex = text.indexOf(command);
    if (currentIndex === -1 || currentIndex <= previousIndex) {
      failures.push(failure);
      return;
    }
    previousIndex = currentIndex;
  }
}

export function verifyRequirementTraceability(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  verifyChecklistMarkdown(texts.get("docs/parity/checklist.md") ?? "", failures);
  verifyRequirementsTable(texts.get(".planning/milestones/v1.9-REQUIREMENTS.md") ?? "", failures);
  verifyRoadmapTraceability(texts.get(".planning/milestones/v1.9-ROADMAP.md") ?? "", failures);
}

export function verifyChecklistMarkdown(text: string, failures: string[]): void {
  const ids = extractRequirementIdsFromSurfaceRows(text);
  verifyRequirementCountsFromArrays(ids, "BOUND-06 parity checklist markdown v1.9 rows", failures);
}

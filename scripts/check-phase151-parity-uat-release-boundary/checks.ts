import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

import { checkClaims } from "./claims.ts";
import { checkGapClosureCompletion, pendingGapClosureRequirements } from "./gap-closure.ts";
import {
  BREADCRUMB_ANCHOR,
  BREADCRUMB_GROUP,
  CATALOG_FILES,
  CLOSEOUT_SURFACE,
  DEFAULT_REPO_ROOT,
  FJALL_DIFFERENCE_SENTENCE,
  GRD01_CHECKBOX,
  GRD01_ID,
  REQUIRED_CLOSEOUT_SOURCES,
  ROADMAP_FILE,
  REQUIRED_DOC_FILES,
  REQUIRED_KNOTS_SYMBOLS,
  REQUIRED_TOP_LEVEL_NAMES,
  REQUIREMENTS_BY_SURFACE,
  REQUIREMENTS_FILE,
  allV24RequirementIds,
  checkedV24RequirementIds,
} from "./constants.ts";
import { checkUatCommands, checkVerifier } from "./verifier.ts";
import {
  checkArchivedAudit,
  checkPlanningSourceIdentity,
  resolvePlanningSource,
} from "./planning-sources.ts";

type ParitySurface = {
  id?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { surfaces?: unknown; checklist?: { surfaces?: unknown } };

/**
 * Returns Phase 151 parity, UAT, and release-boundary failures.
 * Pass an explicit root, or set OPEN_BITCOIN_PHASE151_REPO_ROOT, to inspect a fixture.
 */
export function checkPhase151ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE151_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  const maybeIndex = parseParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  if (maybeIndex) checkSurfaceOwnership(maybeIndex, failures);
  const requirements = texts.get(REQUIREMENTS_FILE) ?? "";
  const archived = resolvePlanningSource(repoRoot, REQUIREMENTS_FILE) !== REQUIREMENTS_FILE;
  if (archived) checkArchivedAudit(repoRoot, failures);
  const pendingClosures = archived ? new Set<string>() : pendingGapClosureRequirements(
    repoRoot, requirements, texts.get(ROADMAP_FILE) ?? "", failures,
  );
  checkRequirementCheckboxes(requirements, pendingClosures, failures);
  checkGapClosureCompletion(repoRoot, requirements, texts.get(ROADMAP_FILE) ?? "", failures, archived);
  checkRoadmapCoverage(texts.get(ROADMAP_FILE) ?? "", failures);
  checkBreadcrumbGroup(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  checkKnotsSymbols(texts, failures);
  checkClaims(texts, failures);
  checkVerifier(repoRoot, texts, failures);
  checkUatCommands(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): Map<string, string> {
  const texts = new Map<string, string>();
  const resolvedRoot = path.resolve(repoRoot);
  for (const file of REQUIRED_DOC_FILES) {
    const sourceFile = resolvePlanningSource(resolvedRoot, file);
    const absolutePath = path.resolve(resolvedRoot, sourceFile);
    if (!isInsideRepo(resolvedRoot, absolutePath)) {
      failures.push(`path escapes repo root: ${file}`);
      texts.set(file, "");
      continue;
    }
    if (!existsSync(absolutePath)) {
      failures.push(`missing target file ${sourceFile}`);
      texts.set(file, "");
      continue;
    }
    try {
      const text = readFileSync(absolutePath, "utf8");
      texts.set(file, text);
      checkPlanningSourceIdentity(file, text, failures);
    } catch {
      failures.push(`unreadable target file ${file}`);
      texts.set(file, "");
    }
  }
  return texts;
}

function checkSurfaceOwnership(index: ParityIndex, failures: string[]): void {
  const topSurfaces = asSurfaceArray(index.surfaces);
  const checklistSurfaces = asSurfaceArray(index.checklist?.surfaces);

  for (const name of REQUIRED_TOP_LEVEL_NAMES) {
    const matches = topSurfaces.filter((surface) => surface.name === name);
    if (matches.length !== 1) {
      failures.push(`v2.4 surface ${name} must have exactly one top-level entry`);
    } else if (matches[0]?.status !== "done") {
      failures.push(`v2.4 surface ${name} must be done`);
    }
  }

  for (const [surfaceId, expectedRequirements] of Object.entries(REQUIREMENTS_BY_SURFACE)) {
    const matches = checklistSurfaces.filter((surface) => surface.id === surfaceId);
    if (matches.length !== 1) {
      failures.push(`v2.4 surface ${surfaceId} must have exactly one checklist entry`);
    }
    const checklist = matches[0];
    if (checklist && !sameMembers(asStringArray(checklist.requirements), expectedRequirements)) {
      failures.push(`v2.4 surface ${surfaceId} has incorrect requirement ownership`);
    }
    if (checklist && checklist.status !== "done") {
      failures.push(`v2.4 surface ${surfaceId} must be done`);
    }
  }

  const ownersByRequirement = new Map<string, string[]>();
  for (const surface of checklistSurfaces) {
    const surfaceId = typeof surface.id === "string" ? surface.id : "";
    for (const requirement of asStringArray(surface.requirements)) {
      const owners = ownersByRequirement.get(requirement) ?? [];
      owners.push(surfaceId);
      ownersByRequirement.set(requirement, owners);
    }
  }

  for (const requirement of allV24RequirementIds()) {
    const owners = ownersByRequirement.get(requirement) ?? [];
    if (owners.length !== 1) {
      failures.push(`${requirement} must have exactly one parity surface owner`);
    }
  }

  const grdOwners = ownersByRequirement.get(GRD01_ID) ?? [];
  const foreignOwners = grdOwners.filter((owner) => owner !== CLOSEOUT_SURFACE);
  if (foreignOwners.length > 0) {
    failures.push(
      `${GRD01_ID} must be owned only by ${CLOSEOUT_SURFACE}, not ${foreignOwners.join(", ")}`,
    );
  }

  const closeout = checklistSurfaces.find((surface) => surface.id === CLOSEOUT_SURFACE);
  const closeoutSources = asStringArray(closeout?.upstream?.sources);
  for (const source of REQUIRED_CLOSEOUT_SOURCES) {
    if (!closeoutSources.includes(source)) {
      failures.push(`missing Phase 151 closeout Knots source ${source}`);
    }
  }
}

function checkBreadcrumbGroup(raw: string, failures: string[]): void {
  try {
    const parsed = JSON.parse(raw) as {
      groups?: Array<{ breadcrumbs?: unknown; label?: unknown }>;
    };
    const groups = parsed.groups ?? [];
    const operatorPrune = groups.find((group) => group.label === BREADCRUMB_GROUP);
    if (!operatorPrune) {
      failures.push(`missing breadcrumb group ${BREADCRUMB_GROUP}`);
      return;
    }
    const breadcrumbs = asStringArray(operatorPrune.breadcrumbs);
    if (!breadcrumbs.includes(BREADCRUMB_ANCHOR)) {
      failures.push(`missing breadcrumb ${BREADCRUMB_ANCHOR} on group ${BREADCRUMB_GROUP}`);
    }
  } catch (error) {
    failures.push(`invalid source breadcrumbs JSON: ${String(error)}`);
  }
}

function checkKnotsSymbols(texts: Map<string, string>, failures: string[]): void {
  const union = CATALOG_FILES.map((file) => texts.get(file) ?? "").join("\n");
  for (const symbol of REQUIRED_KNOTS_SYMBOLS) {
    if (!union.includes(symbol)) {
      failures.push(`missing Knots symbol ${symbol} from the v2.4 catalog union`);
    }
  }
  if (!union.includes(FJALL_DIFFERENCE_SENTENCE)) {
    failures.push("missing Fjall difference sentence from the v2.4 catalog union");
  }
}

function checkRequirementCheckboxes(
  requirementsText: string,
  pendingClosures: Set<string>,
  failures: string[],
): void {
  for (const id of checkedV24RequirementIds()) {
    const checked = (requirementsText.match(new RegExp(`- \\[x\\] \\*\\*${id}\\*\\*`, "g")) ?? [])
      .length;
    const unchecked = (requirementsText.match(new RegExp(`- \\[ \\] \\*\\*${id}\\*\\*`, "g")) ?? [])
      .length;
    if (checked === 1 && unchecked === 0) continue;
    if (checked === 0 && unchecked === 1 && pendingClosures.has(id)) continue;
    failures.push(`v2.4 requirement ${id} must be checked [x] exactly once`);
  }

  const checkboxCount = requirementsText.split(GRD01_CHECKBOX).length - 1;
  if (checkboxCount !== 1) {
    failures.push(`v2.4 requirement ${GRD01_ID} must include ${GRD01_CHECKBOX} exactly once`);
  }
}

const ROADMAP_COMPLETE_ROWS = [
  "| OPER-01 | Phase 150 | Complete |",
  "| OPER-02 | Phase 150 | Complete |",
  "| OPER-03 | Phase 150 | Complete |",
  "| LOCK-02 | Phase 150 | Complete |",
  "| GRD-01 | Phase 151 | Complete |",
] as const;

function checkRoadmapCoverage(roadmapText: string, failures: string[]): void {
  for (const row of ROADMAP_COMPLETE_ROWS) {
    if (roadmapText.includes(row)) continue;
    const requirementId = row.split("|")[1]?.trim() ?? row;
    failures.push(`ROADMAP coverage row ${requirementId} must be Complete`);
  }
}

function parseParityIndex(raw: string, failures: string[]): ParityIndex | null {
  try {
    return JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`invalid parity index JSON: ${String(error)}`);
    return null;
  }
}

export function isInsideRepo(repoRoot: string, absolutePath: string): boolean {
  const relative = path.relative(repoRoot, absolutePath);
  return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
}

function asSurfaceArray(value: unknown): ParitySurface[] {
  return Array.isArray(value)
    ? value.filter((item): item is ParitySurface => typeof item === "object" && item !== null)
    : [];
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string")
    : [];
}

function sameMembers(actual: string[], expected: readonly string[]): boolean {
  return actual.length === expected.length && expected.every((value) => actual.includes(value));
}

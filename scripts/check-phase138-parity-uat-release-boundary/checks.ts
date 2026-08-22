import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

import {
  ARCHIVED_V22_REQUIREMENTS,
  V22_REQUIREMENTS_MILESTONE_NEEDLE,
  resolvePlanningRequirementsSource,
} from "../source-corpus.ts";
import { checkClaims } from "./claims.ts";
import {
  CLOSEOUT_SURFACE,
  DEFAULT_REPO_ROOT,
  MPVFY_IDS,
  REQUIRED_BREADCRUMB_GROUPS,
  REQUIRED_DOC_FILES,
  REQUIRED_KNOTS_ANCHORS,
  REQUIRED_TOP_LEVEL_NAMES,
  REQUIREMENTS_BY_SURFACE,
  REQUIREMENTS_FILE,
  SNAPSHOT_CHECKLIST_ID,
  allV22RequirementIds,
} from "./constants.ts";
import { cellLabel, COMPOSITION_CELL, MATRIX_CELLS } from "./matrix.ts";
import { checkBenchmarks, checkUatCommands, checkVerifier } from "./verifier.ts";

type ParitySurface = {
  id?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
type ParityIndex = { surfaces?: unknown; checklist?: { surfaces?: unknown } };

export function checkPhase138ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE138_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  checkMatrixEvidence(repoRoot, failures);
  const maybeIndex = parseParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  if (maybeIndex) checkSurfaceOwnership(maybeIndex, failures);
  checkRequirementCheckboxes(texts.get(REQUIREMENTS_FILE) ?? "", failures);
  checkBreadcrumbGroups(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  checkClaims(texts, failures);
  checkVerifier(texts.get("scripts/verify.sh") ?? "", failures);
  checkBenchmarks(texts, failures);
  checkUatCommands(texts, failures);
  return failures;
}

function loadCorpus(repoRoot: string, failures: string[]): Map<string, string> {
  const texts = new Map<string, string>();
  const resolvedRoot = path.resolve(repoRoot);
  for (const file of REQUIRED_DOC_FILES) {
    const sourceFile =
      file === REQUIREMENTS_FILE
        ? resolvePlanningRequirementsSource(
            resolvedRoot,
            ARCHIVED_V22_REQUIREMENTS,
            V22_REQUIREMENTS_MILESTONE_NEEDLE,
          )
        : file;
    const absolutePath = path.resolve(resolvedRoot, sourceFile);
    if (!isInsideRepo(resolvedRoot, absolutePath)) {
      failures.push(`path escapes repo root: ${file}`);
      texts.set(file, "");
      continue;
    }
    if (!existsSync(absolutePath)) {
      failures.push(`missing target file ${file}`);
      texts.set(file, "");
      continue;
    }
    try {
      texts.set(file, readFileSync(absolutePath, "utf8"));
    } catch {
      failures.push(`unreadable target file ${file}`);
      texts.set(file, "");
    }
  }
  return texts;
}

function checkMatrixEvidence(repoRoot: string, failures: string[]): void {
  const resolvedRoot = path.resolve(repoRoot);
  for (const cell of MATRIX_CELLS) {
    const text = readAllowListedFile(resolvedRoot, cell.file, failures);
    if (text === null) {
      failures.push(`missing matrix cell ${cellLabel(cell)}: ${cell.file}`);
      continue;
    }
    if (!text.includes(cell.symbol)) {
      failures.push(`missing matrix symbol ${cell.symbol} for ${cellLabel(cell)} in ${cell.file}`);
    }
  }
  const composition = readAllowListedFile(resolvedRoot, COMPOSITION_CELL.file, failures);
  if (composition === null) {
    failures.push(`missing composition cell ${COMPOSITION_CELL.file}`);
    return;
  }
  if (!composition.includes(COMPOSITION_CELL.symbol)) {
    failures.push(`missing composition symbol ${COMPOSITION_CELL.symbol}`);
  }
}

function checkSurfaceOwnership(index: ParityIndex, failures: string[]): void {
  const topSurfaces = asSurfaceArray(index.surfaces);
  const checklistSurfaces = asSurfaceArray(index.checklist?.surfaces);

  for (const name of REQUIRED_TOP_LEVEL_NAMES) {
    const matches = topSurfaces.filter((surface) => surface.name === name);
    if (matches.length !== 1) {
      failures.push(`v2.2 surface ${name} must have exactly one top-level entry`);
    } else if (matches[0]?.status !== "done") {
      failures.push(`v2.2 surface ${name} must be done`);
    }
  }

  for (const [surfaceId, expectedRequirements] of Object.entries(REQUIREMENTS_BY_SURFACE)) {
    const checklistId = surfaceId === SNAPSHOT_CHECKLIST_ID ? SNAPSHOT_CHECKLIST_ID : surfaceId;
    const matches = checklistSurfaces.filter((surface) => surface.id === checklistId);
    if (matches.length !== 1) {
      failures.push(`v2.2 surface ${checklistId} must have exactly one checklist entry`);
    }
    const checklist = matches[0];
    if (checklist && !sameMembers(asStringArray(checklist.requirements), expectedRequirements)) {
      failures.push(`v2.2 surface ${checklistId} has incorrect requirement ownership`);
    }
    if (checklist && checklist.status !== "done") {
      failures.push(`v2.2 surface ${checklistId} must be done`);
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

  for (const requirement of allV22RequirementIds()) {
    const owners = ownersByRequirement.get(requirement) ?? [];
    if (owners.length !== 1) {
      failures.push(`${requirement} must have exactly one parity surface owner`);
    }
  }

  for (const requirement of MPVFY_IDS) {
    const owners = ownersByRequirement.get(requirement) ?? [];
    const foreign = owners.filter((owner) => owner !== CLOSEOUT_SURFACE);
    if (foreign.length > 0) {
      failures.push(`${requirement} must be owned only by ${CLOSEOUT_SURFACE}, not ${foreign.join(", ")}`);
    }
  }

  const closeout = checklistSurfaces.find((surface) => surface.id === CLOSEOUT_SURFACE);
  const closeoutAnchors = new Set([
    ...asStringArray(closeout?.upstream?.sources),
    ...asStringArray(closeout?.upstream?.tests),
  ]);
  for (const anchor of REQUIRED_KNOTS_ANCHORS) {
    if (!closeoutAnchors.has(anchor)) {
      failures.push(`missing Phase 138 closeout Knots anchor ${anchor}`);
    }
  }
}

function checkBreadcrumbGroups(raw: string, failures: string[]): void {
  try {
    const parsed = JSON.parse(raw) as {
      groups?: Array<{ breadcrumbs?: unknown; files?: unknown; label?: unknown }>;
    };
    const groups = parsed.groups ?? [];
    const labels = new Set(
      groups.map((group) => group.label).filter((label): label is string => typeof label === "string"),
    );
    for (const group of REQUIRED_BREADCRUMB_GROUPS) {
      if (!labels.has(group)) failures.push(`missing breadcrumb group ${group}`);
    }
  } catch (error) {
    failures.push(`invalid source breadcrumbs JSON: ${String(error)}`);
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

function readAllowListedFile(repoRoot: string, file: string, failures: string[]): string | null {
  const absolutePath = path.resolve(repoRoot, file);
  if (!isInsideRepo(repoRoot, absolutePath)) {
    failures.push(`path escapes repo root: ${file}`);
    return null;
  }
  if (!existsSync(absolutePath)) return null;
  try {
    return readFileSync(absolutePath, "utf8");
  } catch {
    failures.push(`unreadable target file ${file}`);
    return null;
  }
}

function isInsideRepo(repoRoot: string, absolutePath: string): boolean {
  const relative = path.relative(repoRoot, absolutePath);
  return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
}

function checkRequirementCheckboxes(requirementsText: string, failures: string[]): void {
  for (const id of allV22RequirementIds()) {
    const checked = (requirementsText.match(new RegExp(`- \\[x\\] \\*\\*${id}\\*\\*`, "g")) ?? [])
      .length;
    if (checked !== 1) {
      failures.push(`v2.2 requirement ${id} must be checked [x] exactly once`);
    }
  }
}

function asSurfaceArray(value: unknown): ParitySurface[] {
  return Array.isArray(value)
    ? value.filter((item): item is ParitySurface => typeof item === "object" && item !== null)
    : [];
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

function sameMembers(actual: string[], expected: readonly string[]): boolean {
  return actual.length === expected.length && expected.every((value) => actual.includes(value));
}

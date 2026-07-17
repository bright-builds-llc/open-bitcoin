#!/usr/bin/env bun

import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";

const DEFAULT_ROOT_DIR = path.resolve(import.meta.dir, "..");
const ROADMAP_FILE = ".planning/ROADMAP.md";
const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";
const PHASES_DIR = ".planning/phases";
const ACTIVE_MILESTONE_HEADING = "## Active Milestone:";
const ACTIVE_REQUIREMENTS_HEADING = "## v2.1 Requirements";
const DEFERRED_REQUIREMENTS_HEADING = "## Deferred Requirements";
const REQUIREMENT_ID_PATTERN = "[A-Z]+-\\d+";

export type CheckActiveMilestoneVerificationTraceabilityOptions = {
  maybeRootDir?: string;
};

type ActiveRequirement = {
  checked: boolean;
  id: string;
};

type TraceabilityRow = {
  id: string;
  phase: number;
  status: "Complete" | "Pending";
};

type LifecycleIdentity = {
  mode: string;
  phaseLifecycleId: string;
};

type PhaseCorpus = {
  directory: string;
  lifecycle: LifecycleIdentity | null;
  phase: number;
  summaries: Artifact[];
  verifications: Artifact[];
};

type Artifact = {
  frontmatter: string | null;
  relativePath: string;
  text: string;
};

export function checkActiveMilestoneVerificationTraceability(
  maybeOptions: CheckActiveMilestoneVerificationTraceabilityOptions = {},
): string[] {
  const rootDir = path.resolve(
    maybeOptions.maybeRootDir ?? DEFAULT_ROOT_DIR,
  );
  const failures: string[] = [];
  const roadmap = readRequiredText(rootDir, ROADMAP_FILE, failures);
  const requirements = readRequiredText(
    rootDir,
    REQUIREMENTS_FILE,
    failures,
  );
  const activePhases = parseActiveRoadmapPhases(roadmap, failures);
  const activeRequirements = parseActiveRequirements(
    requirements,
    failures,
  );
  const traceabilityRows = parseTraceabilityRows(requirements, failures);

  const ownedRequirementIds = verifyTraceabilityOwnership(
    activeRequirements,
    traceabilityRows,
    activePhases,
    failures,
  );

  const phaseCorpora = loadPhaseCorpora(rootDir, activePhases, failures);
  const activatedIds = activatedRequirementIds(
    phaseCorpora,
    ownedRequirementIds,
    failures,
  );
  verifyCompletedRequirementsActivated(
    activeRequirements,
    traceabilityRows,
    ownedRequirementIds,
    activatedIds,
    failures,
  );
  const coveredIds = lifecycleValidCoverage(
    phaseCorpora,
    activatedIds,
    failures,
  );

  for (const requirementId of [...activatedIds].sort()) {
    if (!coveredIds.has(requirementId)) {
      failures.push(
        `activated requirement ${requirementId} is missing lifecycle-valid active-phase verification coverage`,
      );
    }
  }

  return failures;
}

function readRequiredText(
  rootDir: string,
  relativePath: string,
  failures: string[],
): string {
  const absolutePath = path.join(rootDir, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required corpus file ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function parseActiveRoadmapPhases(
  roadmap: string,
  failures: string[],
): Set<number> {
  const maybeSection = sectionBetweenSecondLevelHeadings(
    roadmap,
    ACTIVE_MILESTONE_HEADING,
  );
  if (maybeSection === null) {
    failures.push(`roadmap missing ${ACTIVE_MILESTONE_HEADING} section`);
    return new Set();
  }

  const phases: number[] = [];
  for (const [index, line] of maybeSection.split("\n").entries()) {
    const maybeMatch = line.match(/^- \[[ x]\] \*\*Phase (\d+):/);
    if (maybeMatch) {
      phases.push(Number(maybeMatch[1]));
      continue;
    }
    if (/^- \[[ x]\] \*\*Phase\b/.test(line)) {
      failures.push(
        `malformed active roadmap phase entry at ${ROADMAP_FILE}:${index + 1}`,
      );
    }
  }

  if (phases.length === 0) {
    failures.push("active roadmap contains no phase entries");
  }
  for (const phase of duplicateValues(phases)) {
    failures.push(`active roadmap phase ${phase} appears more than once`);
  }
  return new Set(phases);
}

function parseActiveRequirements(
  requirements: string,
  failures: string[],
): ActiveRequirement[] {
  const maybeSection = sectionBetweenExactHeadings(
    requirements,
    ACTIVE_REQUIREMENTS_HEADING,
    DEFERRED_REQUIREMENTS_HEADING,
  );
  if (maybeSection === null) {
    failures.push(
      `requirements missing ${ACTIVE_REQUIREMENTS_HEADING} to ${DEFERRED_REQUIREMENTS_HEADING} boundary`,
    );
    return [];
  }

  const entries: ActiveRequirement[] = [];
  const checklistPattern = new RegExp(
    `^- \\[([ x])\\] \\*\\*(${REQUIREMENT_ID_PATTERN})\\*\\*:`,
  );
  for (const [index, line] of maybeSection.split("\n").entries()) {
    const maybeMatch = line.match(checklistPattern);
    if (maybeMatch) {
      entries.push({
        checked: maybeMatch[1] === "x",
        id: maybeMatch[2] ?? "",
      });
      continue;
    }
    if (/^- \[[ x]\] \*\*[A-Z]+-\d+\*\*/.test(line)) {
      failures.push(
        `malformed active requirement checklist row at ${REQUIREMENTS_FILE}:${index + 1}`,
      );
    }
  }

  if (entries.length === 0) {
    failures.push("active requirements checklist contains no requirement IDs");
  }
  for (const id of duplicateValues(entries.map((entry) => entry.id))) {
    failures.push(`active requirement checklist duplicates ${id}`);
  }
  return entries;
}

function parseTraceabilityRows(
  requirements: string,
  failures: string[],
): TraceabilityRow[] {
  const rows: TraceabilityRow[] = [];
  const rowPattern = new RegExp(
    `^\\|\\s*(${REQUIREMENT_ID_PATTERN})\\s*\\|\\s*Phase\\s+(\\d+)\\s*\\|\\s*(Complete|Pending)\\s*\\|$`,
  );
  const candidatePattern = new RegExp(
    `^\\|\\s*(${REQUIREMENT_ID_PATTERN})\\s*\\|`,
  );

  for (const [index, line] of requirements.split("\n").entries()) {
    const maybeMatch = line.match(rowPattern);
    if (maybeMatch) {
      rows.push({
        id: maybeMatch[1] ?? "",
        phase: Number(maybeMatch[2]),
        status: maybeMatch[3] === "Complete" ? "Complete" : "Pending",
      });
      continue;
    }
    if (candidatePattern.test(line)) {
      failures.push(
        `malformed traceability row at ${REQUIREMENTS_FILE}:${index + 1}`,
      );
    }
  }
  return rows;
}

function verifyTraceabilityOwnership(
  requirements: ActiveRequirement[],
  rows: TraceabilityRow[],
  activePhases: Set<number>,
  failures: string[],
): Set<string> {
  const ownedIds = new Set<string>();
  const checklistCounts = countValues(
    requirements.map((requirement) => requirement.id),
  );
  for (const requirement of requirements) {
    if (checklistCounts.get(requirement.id) !== 1) {
      continue;
    }
    const owners = rows.filter((row) => row.id === requirement.id);
    if (owners.length !== 1) {
      failures.push(
        `active requirement ${requirement.id} must have exactly one traceability row; found ${owners.length}`,
      );
      continue;
    }
    const owner = owners[0];
    if (owner && !activePhases.has(owner.phase)) {
      failures.push(
        `active requirement ${requirement.id} traceability owner Phase ${owner.phase} is not in the active roadmap`,
      );
      continue;
    }
    if (owner) {
      ownedIds.add(requirement.id);
    }
  }
  return ownedIds;
}

function verifyCompletedRequirementsActivated(
  requirements: ActiveRequirement[],
  rows: TraceabilityRow[],
  ownedRequirementIds: Set<string>,
  activatedIds: Set<string>,
  failures: string[],
): void {
  for (const requirement of requirements) {
    if (!ownedRequirementIds.has(requirement.id)) {
      continue;
    }
    const owner = rows.find((row) => row.id === requirement.id);
    const traceabilityComplete = owner?.status === "Complete";
    if (requirement.checked !== traceabilityComplete) {
      failures.push(
        `active requirement ${requirement.id} has inconsistent checklist and traceability completion state`,
      );
      continue;
    }
    if (!requirement.checked || activatedIds.has(requirement.id)) {
      continue;
    }
    failures.push(
      `completed active requirement ${requirement.id} has no requirements-completed summary activation`,
    );
  }
}

function loadPhaseCorpora(
  rootDir: string,
  activePhases: Set<number>,
  failures: string[],
): PhaseCorpus[] {
  const phasesDir = path.join(rootDir, PHASES_DIR);
  if (!existsSync(phasesDir)) {
    failures.push(`missing required corpus directory ${PHASES_DIR}`);
    return [];
  }

  const directories = readdirSync(phasesDir)
    .filter((entry) => statSync(path.join(phasesDir, entry)).isDirectory())
    .sort();
  const corpora: PhaseCorpus[] = [];

  for (const phase of [...activePhases].sort((left, right) => left - right)) {
    const matches = directories.filter((entry) =>
      entry.startsWith(`${phase}-`),
    );
    if (matches.length > 1) {
      failures.push(
        `active Phase ${phase} resolves to multiple phase directories: ${matches.join(", ")}`,
      );
    }
    const maybeDirectory = matches[0];
    if (maybeDirectory === undefined) {
      continue;
    }
    corpora.push(
      loadPhaseCorpus(rootDir, phase, maybeDirectory, failures),
    );
  }

  return corpora;
}

function loadPhaseCorpus(
  rootDir: string,
  phase: number,
  directory: string,
  failures: string[],
): PhaseCorpus {
  const relativeDirectory = path.join(PHASES_DIR, directory);
  const absoluteDirectory = path.join(rootDir, relativeDirectory);
  const entries = readdirSync(absoluteDirectory).sort();
  const summaryNames = entries.filter((entry) =>
    new RegExp(`^${phase}-\\d+-SUMMARY\\.md$`).test(entry),
  );
  const verificationNames = entries.filter((entry) =>
    new RegExp(`^${phase}-VERIFICATION\\.md$`).test(entry),
  );
  const contextPath = path.join(relativeDirectory, `${phase}-CONTEXT.md`);
  const hasArtifacts =
    summaryNames.length > 0 || verificationNames.length > 0;
  const contextText = existsSync(path.join(rootDir, contextPath))
    ? readFileSync(path.join(rootDir, contextPath), "utf8")
    : "";

  if (hasArtifacts && contextText === "") {
    failures.push(`active Phase ${phase} artifacts are missing ${contextPath}`);
  }

  const maybeContextFrontmatter = hasArtifacts
    ? extractFrontmatter(contextText, contextPath, failures)
    : null;
  const lifecycle =
    maybeContextFrontmatter === null
      ? null
      : parseLifecycleIdentity(
          maybeContextFrontmatter,
          contextPath,
          failures,
        );

  return {
    directory: relativeDirectory,
    lifecycle,
    phase,
    summaries: summaryNames.map((name) =>
      loadArtifact(rootDir, path.join(relativeDirectory, name), failures),
    ),
    verifications: verificationNames.map((name) =>
      loadArtifact(rootDir, path.join(relativeDirectory, name), failures),
    ),
  };
}

function loadArtifact(
  rootDir: string,
  relativePath: string,
  failures: string[],
): Artifact {
  const text = readFileSync(path.join(rootDir, relativePath), "utf8");
  return {
    frontmatter: extractFrontmatter(text, relativePath, failures),
    relativePath,
    text,
  };
}

function activatedRequirementIds(
  corpora: PhaseCorpus[],
  ownedRequirementIds: Set<string>,
  failures: string[],
): Set<string> {
  const activated = new Set<string>();

  for (const corpus of corpora) {
    for (const summary of corpus.summaries) {
      if (summary.frontmatter === null) {
        continue;
      }
      verifyArtifactLifecycle(summary, corpus.lifecycle, failures);
      for (const id of parseRequirementsCompleted(summary, failures)) {
        if (ownedRequirementIds.has(id)) {
          activated.add(id);
        }
      }
    }
  }
  return activated;
}

function lifecycleValidCoverage(
  corpora: PhaseCorpus[],
  activatedIds: Set<string>,
  failures: string[],
): Set<string> {
  const covered = new Set<string>();

  for (const corpus of corpora) {
    for (const verification of corpus.verifications) {
      const lifecycleValid = verifyVerificationLifecycle(
        verification,
        corpus.lifecycle,
        failures,
      );
      if (!lifecycleValid) {
        continue;
      }
      for (const id of activatedIds) {
        if (containsRequirementToken(verification.text, id)) {
          covered.add(id);
        }
      }
    }
  }
  return covered;
}

function verifyArtifactLifecycle(
  artifact: Artifact,
  expected: LifecycleIdentity | null,
  failures: string[],
): boolean {
  if (artifact.frontmatter === null || expected === null) {
    return false;
  }
  const maybeActual = parseLifecycleIdentity(
    artifact.frontmatter,
    artifact.relativePath,
    failures,
  );
  if (maybeActual === null) {
    return false;
  }
  let valid = true;
  if (maybeActual.mode !== expected.mode) {
    failures.push(
      `${artifact.relativePath} lifecycle_mode does not match its phase CONTEXT`,
    );
    valid = false;
  }
  if (maybeActual.phaseLifecycleId !== expected.phaseLifecycleId) {
    failures.push(
      `${artifact.relativePath} phase_lifecycle_id does not match its phase CONTEXT`,
    );
    valid = false;
  }
  return valid;
}

function verifyVerificationLifecycle(
  artifact: Artifact,
  expected: LifecycleIdentity | null,
  failures: string[],
): boolean {
  const lifecycleValid = verifyArtifactLifecycle(
    artifact,
    expected,
    failures,
  );
  if (artifact.frontmatter === null) {
    return false;
  }
  const statusValid = requireExactScalar(
    artifact.frontmatter,
    "status",
    "passed",
    artifact.relativePath,
    failures,
  );
  const validationValid = requireExactScalar(
    artifact.frontmatter,
    "lifecycle_validated",
    "true",
    artifact.relativePath,
    failures,
  );
  return lifecycleValid && statusValid && validationValid;
}

function parseLifecycleIdentity(
  frontmatter: string,
  relativePath: string,
  failures: string[],
): LifecycleIdentity | null {
  const maybeMode = exactScalar(
    frontmatter,
    "lifecycle_mode",
    relativePath,
    failures,
  );
  const maybePhaseLifecycleId = exactScalar(
    frontmatter,
    "phase_lifecycle_id",
    relativePath,
    failures,
  );
  if (maybeMode === null || maybePhaseLifecycleId === null) {
    return null;
  }
  return { mode: maybeMode, phaseLifecycleId: maybePhaseLifecycleId };
}

function parseRequirementsCompleted(
  artifact: Artifact,
  failures: string[],
): string[] {
  const frontmatter = artifact.frontmatter ?? "";
  const fieldPattern = /^requirements-completed[ \t]*:[ \t]*(.*)$/gm;
  const matches = [...frontmatter.matchAll(fieldPattern)];
  if (matches.length !== 1) {
    failures.push(
      `${artifact.relativePath} requires exactly one requirements-completed field; found ${matches.length}`,
    );
    return [];
  }

  const inlineValue = stripYamlComment((matches[0]?.[1] ?? "").trim());
  if (inlineValue !== "") {
    if (!inlineValue.startsWith("[") || !inlineValue.endsWith("]")) {
      failures.push(
        `${artifact.relativePath} has malformed inline requirements-completed`,
      );
      return [];
    }
    return parseRequirementIdList(
      inlineValue.slice(1, -1).split(","),
      artifact.relativePath,
      failures,
    );
  }

  const fieldIndex = matches[0]?.index ?? 0;
  const afterField = frontmatter.slice(fieldIndex).split("\n").slice(1);
  const values: string[] = [];
  for (const line of afterField) {
    if (/^[A-Za-z0-9_-]+\s*:/.test(line)) {
      break;
    }
    const maybeItem = line.match(/^\s+-\s+(.+?)\s*$/);
    if (maybeItem) {
      values.push(maybeItem[1] ?? "");
      continue;
    }
    if (line.trim() !== "") {
      failures.push(
        `${artifact.relativePath} has malformed block requirements-completed entry`,
      );
    }
  }
  return parseRequirementIdList(values, artifact.relativePath, failures);
}

function parseRequirementIdList(
  rawValues: string[],
  relativePath: string,
  failures: string[],
): string[] {
  const ids: string[] = [];
  const idPattern = new RegExp(`^${REQUIREMENT_ID_PATTERN}$`);
  for (const rawValue of rawValues) {
    const value = stripYamlQuotes(rawValue.trim());
    if (value === "") {
      continue;
    }
    if (!idPattern.test(value)) {
      failures.push(`${relativePath} has malformed requirements-completed ID`);
      continue;
    }
    ids.push(value);
  }
  for (const id of duplicateValues(ids)) {
    failures.push(`${relativePath} duplicates requirements-completed ID ${id}`);
  }
  return ids;
}

function extractFrontmatter(
  text: string,
  relativePath: string,
  failures: string[],
): string | null {
  const maybeMatch = text.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  if (maybeMatch === null) {
    failures.push(`${relativePath} has malformed or missing YAML frontmatter`);
    return null;
  }
  return maybeMatch[1] ?? "";
}

function exactScalar(
  frontmatter: string,
  key: string,
  relativePath: string,
  failures: string[],
): string | null {
  const pattern = new RegExp(
    `^${escapeRegExp(key)}[ \\t]*:[ \\t]*(.*?)[ \\t]*$`,
    "gm",
  );
  const matches = [...frontmatter.matchAll(pattern)];
  if (matches.length !== 1) {
    failures.push(
      `${relativePath} requires exactly one ${key} field; found ${matches.length}`,
    );
    return null;
  }
  const value = stripYamlQuotes((matches[0]?.[1] ?? "").trim());
  if (value === "") {
    failures.push(`${relativePath} has empty ${key} field`);
    return null;
  }
  return value;
}

function requireExactScalar(
  frontmatter: string,
  key: string,
  expected: string,
  relativePath: string,
  failures: string[],
): boolean {
  const maybeValue = exactScalar(
    frontmatter,
    key,
    relativePath,
    failures,
  );
  if (maybeValue === null) {
    return false;
  }
  if (maybeValue !== expected) {
    failures.push(`${relativePath} requires ${key}: ${expected}`);
    return false;
  }
  return true;
}

function sectionBetweenSecondLevelHeadings(
  text: string,
  headingPrefix: string,
): string | null {
  const lines = text.split("\n");
  const start = lines.findIndex((line) => line.startsWith(headingPrefix));
  if (start === -1) {
    return null;
  }
  const maybeEnd = lines.findIndex(
    (line, index) => index > start && /^## (?!#)/.test(line),
  );
  return lines.slice(start + 1, maybeEnd === -1 ? undefined : maybeEnd).join("\n");
}

function sectionBetweenExactHeadings(
  text: string,
  startHeading: string,
  endHeading: string,
): string | null {
  const start = text.indexOf(startHeading);
  const end = text.indexOf(endHeading, start + startHeading.length);
  if (start === -1 || end === -1 || end <= start) {
    return null;
  }
  return text.slice(start + startHeading.length, end);
}

function containsRequirementToken(text: string, requirementId: string): boolean {
  const pattern = new RegExp(
    `(?<![A-Z0-9-])${escapeRegExp(requirementId)}(?![A-Z0-9-])`,
  );
  return pattern.test(text);
}

function duplicateValues<T extends string | number>(values: T[]): T[] {
  const seen = new Set<T>();
  const duplicates = new Set<T>();
  for (const value of values) {
    if (seen.has(value)) {
      duplicates.add(value);
    }
    seen.add(value);
  }
  return [...duplicates];
}

function countValues<T extends string | number>(values: T[]): Map<T, number> {
  const counts = new Map<T, number>();
  for (const value of values) {
    counts.set(value, (counts.get(value) ?? 0) + 1);
  }
  return counts;
}

function stripYamlQuotes(value: string): string {
  const isQuoted =
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"));
  return isQuoted ? value.slice(1, -1) : value;
}

function stripYamlComment(value: string): string {
  return value.replace(/[ \t]+#.*$/, "").trim();
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

if (import.meta.main) {
  const failures = checkActiveMilestoneVerificationTraceability();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`verification-traceability: ${failure}`);
    }
    process.exit(1);
  }
  console.log("Active milestone verification traceability checker passed.");
}

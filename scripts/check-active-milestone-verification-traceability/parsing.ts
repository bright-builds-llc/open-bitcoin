import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";
import { ROADMAP_FILE, REQUIREMENTS_FILE, ACTIVE_MILESTONE_HEADING, REQUIREMENT_ID_PATTERN, ActiveRequirement, TraceabilityRow, Artifact } from "./constants.ts";

export function parseActiveRoadmapPhases(
  roadmap: string,
  failures: string[],
): Set<number> {
  const maybeSection =
    secondLevelSection(roadmap, (line) => line === "## Phases") ??
    secondLevelSection(roadmap, (line) =>
      line.startsWith(ACTIVE_MILESTONE_HEADING),
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

export function parseActiveRequirements(
  requirements: string,
  failures: string[],
): ActiveRequirement[] {
  const maybeSection = secondLevelSection(
    requirements,
    (line) => /^## v\d+\.\d+ Requirements$/.test(line),
  );
  if (maybeSection === null) {
    failures.push(
      "requirements missing active milestone requirements section",
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

export function parseTraceabilityRows(
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

export function parseRequirementsCompleted(
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

export function parseRequirementIdList(
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

export function extractFrontmatter(
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

export function exactScalar(
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

export function requireExactScalar(
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

export function secondLevelSection(
  text: string,
  isHeading: (line: string) => boolean,
): string | null {
  const lines = text.split("\n");
  const start = lines.findIndex(
    (line) => /^## (?!#)/.test(line) && isHeading(line),
  );
  if (start === -1) {
    return null;
  }
  const maybeEnd = lines.findIndex(
    (line, index) => index > start && /^## (?!#)/.test(line),
  );
  return lines.slice(start + 1, maybeEnd === -1 ? undefined : maybeEnd).join("\n");
}

export function containsRequirementToken(text: string, requirementId: string): boolean {
  const pattern = new RegExp(
    `(?<![A-Z0-9-])${escapeRegExp(requirementId)}(?![A-Z0-9-])`,
  );
  return pattern.test(text);
}

export function duplicateValues<T extends string | number>(values: T[]): T[] {
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

export function countValues<T extends string | number>(values: T[]): Map<T, number> {
  const counts = new Map<T, number>();
  for (const value of values) {
    counts.set(value, (counts.get(value) ?? 0) + 1);
  }
  return counts;
}

export function stripYamlQuotes(value: string): string {
  const isQuoted =
    (value.startsWith('"') && value.endsWith('"')) ||
    (value.startsWith("'") && value.endsWith("'"));
  return isQuoted ? value.slice(1, -1) : value;
}

export function stripYamlComment(value: string): string {
  return value.replace(/[ \t]+#.*$/, "").trim();
}

export function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

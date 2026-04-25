#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const BREADCRUMB_HEADER = "// Parity breadcrumbs:";
const DEFAULT_MAPPING_PATH = "docs/parity/source-breadcrumbs.json";

type Options = {
  check: boolean;
  mappingPath: string;
  write: boolean;
};

type BreadcrumbGroup = {
  breadcrumbs?: string[];
  files?: string[];
  label: string;
  patterns?: string[];
  reason?: string;
};

type BreadcrumbConfig = {
  groups: BreadcrumbGroup[];
  noneReason: string;
  scope: {
    exclude: string[];
    include: string[];
  };
  version: number;
};

type FileMapping = {
  breadcrumbs: string[];
  label: string;
  reason: string;
};

type StripResult = {
  blockCount: number;
  lines: string[];
};

type InsertionPoint = {
  dropBlankAtIndex: boolean;
  index: number;
};

function usage(): string {
  return `Usage: bun run scripts/check-parity-breadcrumbs.ts [--check|--write] [--mapping=PATH]

Checks or applies parity breadcrumb comments for first-party Rust files.`;
}

function parseArgs(argv: string[]): Options {
  const options: Options = {
    check: true,
    mappingPath: DEFAULT_MAPPING_PATH,
    write: false,
  };

  for (const arg of argv) {
    if (arg === "--check") {
      options.check = true;
      options.write = false;
      continue;
    }
    if (arg === "--write") {
      options.check = false;
      options.write = true;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      console.log(usage());
      process.exit(0);
    }
    if (arg.startsWith("--mapping=")) {
      options.mappingPath = normalizeRelativePath(arg.slice("--mapping=".length));
      continue;
    }

    throw new Error(`unknown argument: ${arg}`);
  }

  return options;
}

function normalizeRelativePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
}

function git(repoRoot: string, args: string[]): string {
  return execFileSync("git", ["-C", repoRoot, ...args], {
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  });
}

function repoRoot(): string {
  return git(process.cwd(), ["rev-parse", "--show-toplevel"]).trim();
}

function trackedPaths(repoRoot: string): string[] {
  return git(repoRoot, ["ls-files", "-z"])
    .split("\0")
    .filter((filePath) => filePath.length > 0)
    .map(normalizeRelativePath)
    .sort();
}

function inScopeRustFile(filePath: string): boolean {
  return /^packages\/open-bitcoin-[^/]+\/(src|tests)\/.+\.rs$/.test(filePath);
}

function loadConfig(repoRoot: string, mappingPath: string): BreadcrumbConfig {
  const configPath = path.join(repoRoot, mappingPath);
  const config = JSON.parse(readFileSync(configPath, "utf8")) as BreadcrumbConfig;

  if (config.version !== 1) {
    throw new Error(`${mappingPath}: unsupported version ${config.version}`);
  }
  if (!Array.isArray(config.groups) || config.groups.length === 0) {
    throw new Error(`${mappingPath}: expected at least one mapping group`);
  }

  return config;
}

function globToRegex(pattern: string): RegExp {
  let source = "^";

  for (let index = 0; index < pattern.length; index += 1) {
    const char = pattern[index];
    const nextChar = pattern[index + 1];

    if (char === "*" && nextChar === "*") {
      source += ".*";
      index += 1;
      continue;
    }
    if (char === "*") {
      source += "[^/]*";
      continue;
    }
    if ("\\^$+?.()|{}[]".includes(char)) {
      source += `\\${char}`;
      continue;
    }

    source += char;
  }

  source += "$";
  return new RegExp(source);
}

function groupMatchesPath(group: BreadcrumbGroup, filePath: string): boolean {
  const files = group.files ?? [];
  if (files.includes(filePath)) {
    return true;
  }

  const patterns = group.patterns ?? [];
  return patterns.some((patternValue) => globToRegex(patternValue).test(filePath));
}

function validateMappingShape(config: BreadcrumbConfig): string[] {
  const errors: string[] = [];

  for (const group of config.groups) {
    const files = group.files ?? [];
    const patterns = group.patterns ?? [];
    const breadcrumbs = group.breadcrumbs ?? [];

    if (group.label.trim() === "") {
      errors.push("mapping group has an empty label");
    }
    if (files.length === 0 && patterns.length === 0) {
      errors.push(`${group.label}: expected at least one file or pattern`);
    }
    for (const filePath of files) {
      if (!inScopeRustFile(filePath)) {
        errors.push(`${group.label}: mapped file is outside breadcrumb scope: ${filePath}`);
      }
    }
    for (const patternValue of patterns) {
      if (patternValue.startsWith("packages/bitcoin-knots/")) {
        errors.push(`${group.label}: pattern points at vendored Knots: ${patternValue}`);
      }
    }
    for (const breadcrumb of breadcrumbs) {
      if (!breadcrumb.startsWith("packages/bitcoin-knots/")) {
        errors.push(`${group.label}: breadcrumb is not repo-root-relative Knots path: ${breadcrumb}`);
      }
    }
    if (breadcrumbs.length === 0 && (group.reason ?? config.noneReason).trim() === "") {
      errors.push(`${group.label}: none mapping needs a reason`);
    }
  }

  return errors;
}

function buildMappings(
  config: BreadcrumbConfig,
  scopePaths: string[],
): { errors: string[]; mappings: Map<string, FileMapping> } {
  const errors = validateMappingShape(config);
  const mappings = new Map<string, FileMapping>();
  const labelsWithMatches = new Set<string>();

  for (const filePath of scopePaths) {
    const matchedGroups = config.groups.filter((group) => groupMatchesPath(group, filePath));
    if (matchedGroups.length === 0) {
      errors.push(`missing breadcrumb mapping for ${filePath}`);
      continue;
    }
    if (matchedGroups.length > 1) {
      const labels = matchedGroups.map((group) => group.label).join(", ");
      errors.push(`duplicate breadcrumb mapping for ${filePath}: ${labels}`);
      continue;
    }

    const [group] = matchedGroups;
    labelsWithMatches.add(group.label);
    mappings.set(filePath, {
      breadcrumbs: group.breadcrumbs ?? [],
      label: group.label,
      reason: group.reason ?? config.noneReason,
    });
  }

  for (const group of config.groups) {
    if (!labelsWithMatches.has(group.label)) {
      errors.push(`${group.label}: mapping did not match any in-scope Rust file`);
    }
  }

  return { errors, mappings };
}

function validateBreadcrumbTargets(repoRoot: string, mappings: Map<string, FileMapping>): string[] {
  const errors: string[] = [];

  for (const [filePath, mapping] of mappings) {
    for (const breadcrumb of mapping.breadcrumbs) {
      if (!existsSync(path.join(repoRoot, breadcrumb))) {
        errors.push(`${filePath}: breadcrumb target does not exist: ${breadcrumb}`);
      }
    }
  }

  return errors;
}

function linesForText(text: string): string[] {
  const lines = text.split("\n");
  if (lines.at(-1) === "") {
    lines.pop();
  }
  return lines;
}

function textForLines(lines: string[]): string {
  return `${lines.join("\n")}\n`;
}

function stripBreadcrumbBlocks(lines: string[]): StripResult {
  const strippedLines: string[] = [];
  let blockCount = 0;

  for (let index = 0; index < lines.length; index += 1) {
    if (lines[index] !== BREADCRUMB_HEADER) {
      strippedLines.push(lines[index]);
      continue;
    }

    blockCount += 1;
    index += 1;
    while (index < lines.length && lines[index].startsWith("// - ")) {
      index += 1;
    }
    if (index < lines.length && lines[index] !== "") {
      index -= 1;
    }
  }

  return { blockCount, lines: strippedLines };
}

function initialInsertionPoint(lines: string[]): InsertionPoint {
  let index = 0;
  let sawAttribute = false;

  while (index < lines.length && lines[index].startsWith("#![")) {
    sawAttribute = true;
    let bracketDepth = 0;

    do {
      bracketDepth += countChar(lines[index], "[");
      bracketDepth -= countChar(lines[index], "]");
      index += 1;
    } while (index < lines.length && bracketDepth > 0);
  }

  if (sawAttribute && lines[index] === "") {
    return {
      dropBlankAtIndex: true,
      index,
    };
  }

  return {
    dropBlankAtIndex: false,
    index,
  };
}

function countChar(value: string, needle: string): number {
  let count = 0;
  for (const char of value) {
    if (char === needle) {
      count += 1;
    }
  }
  return count;
}

function breadcrumbBlock(mapping: FileMapping): string[] {
  const lines = [BREADCRUMB_HEADER];
  if (mapping.breadcrumbs.length === 0) {
    lines.push(`// - none: ${mapping.reason}`);
    return lines;
  }

  for (const breadcrumb of mapping.breadcrumbs) {
    lines.push(`// - ${breadcrumb}`);
  }
  return lines;
}

function expectedContent(currentText: string, mapping: FileMapping): { blockCount: number; text: string } {
  const currentLines = linesForText(currentText);
  const stripped = stripBreadcrumbBlocks(currentLines);
  const insertionPoint = initialInsertionPoint(stripped.lines);
  const afterIndex = insertionPoint.index + (insertionPoint.dropBlankAtIndex ? 1 : 0);
  const expectedLines = [
    ...stripped.lines.slice(0, insertionPoint.index),
    ...breadcrumbBlock(mapping),
    "",
    ...stripped.lines.slice(afterIndex),
  ];

  return {
    blockCount: stripped.blockCount,
    text: textForLines(expectedLines),
  };
}

function checkOrWriteFiles(
  repoRoot: string,
  mappings: Map<string, FileMapping>,
  options: Options,
): string[] {
  const errors: string[] = [];
  let changed = 0;

  for (const [filePath, mapping] of [...mappings.entries()].sort()) {
    const absolutePath = path.join(repoRoot, filePath);
    const currentText = readFileSync(absolutePath, "utf8");
    const expected = expectedContent(currentText, mapping);

    if (options.check && expected.blockCount !== 1) {
      errors.push(`${filePath}: expected exactly one breadcrumb block, found ${expected.blockCount}`);
      continue;
    }

    if (currentText === expected.text) {
      continue;
    }

    if (options.write) {
      writeFileSync(absolutePath, expected.text);
      changed += 1;
      continue;
    }

    errors.push(`${filePath}: breadcrumb block is missing or stale; run with --write`);
  }

  if (options.write) {
    console.log(`Updated parity breadcrumbs in ${changed} file(s).`);
  }

  return errors;
}

function main(): void {
  const options = parseArgs(process.argv.slice(2));
  const root = repoRoot();
  const config = loadConfig(root, options.mappingPath);
  const scopePaths = trackedPaths(root).filter(inScopeRustFile);
  const { errors: mappingErrors, mappings } = buildMappings(config, scopePaths);
  const targetErrors = validateBreadcrumbTargets(root, mappings);
  const fileErrors =
    mappingErrors.length === 0 && targetErrors.length === 0
      ? checkOrWriteFiles(root, mappings, options)
      : [];
  const errors = [...mappingErrors, ...targetErrors, ...fileErrors];

  if (errors.length > 0) {
    throw new Error(errors.join("\n"));
  }

  if (options.check) {
    console.log(`Parity breadcrumbs verified for ${mappings.size} Rust file(s).`);
  }
}

try {
  main();
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(message);
  process.exit(1);
}

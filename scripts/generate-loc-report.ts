#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_OUTPUT = "docs/metrics/lines-of-code.md";
type SourceMode = "worktree" | "index";

type Options = {
  check: boolean;
  output: string;
  source: SourceMode;
};

type GitOptions = {
  encoding?: BufferEncoding | "buffer";
};

type Counts = {
  blank: number;
  code: number;
  comments: number;
  files: number;
  total: number;
};

type FileStats = {
  category: string;
  counts: Counts;
  path: string;
};

type CategoryStats = Counts & {
  name: string;
};

type CrateStats = {
  files: number;
  manifestBuildLines: number;
  name: string;
  productionRustLines: number;
  testRustLines: number;
  totalLines: number;
};

type Metrics = {
  categories: CategoryStats[];
  crates: CrateStats[];
  files: FileStats[];
  fingerprint: string;
  totals: Counts;
};

const ROOT_CONFIG_FILES: ReadonlySet<string> = new Set([
  ".bazelrc",
  ".bazelversion",
  ".gitignore",
  ".gitmodules",
  "BUILD.bazel",
  "MODULE.bazel",
  "MODULE.bazel.lock",
  "packages/BUILD.bazel",
  "packages/Cargo.lock",
  "packages/Cargo.toml",
  "rust-toolchain.toml",
]);

function usage(): string {
  return `Usage: bun run scripts/generate-loc-report.ts [--source=worktree|index] [--output=PATH] [--check]

Generates a deterministic first-party lines-of-code report.`;
}

function parseArgs(argv: string[]): Options {
  const options: Options = {
    check: false,
    output: DEFAULT_OUTPUT,
    source: "worktree",
  };

  for (const arg of argv) {
    if (arg === "--check") {
      options.check = true;
      continue;
    }
    if (arg === "--help" || arg === "-h") {
      console.log(usage());
      process.exit(0);
    }
    if (arg.startsWith("--source=")) {
      options.source = parseSourceMode(arg.slice("--source=".length));
      continue;
    }
    if (arg.startsWith("--output=")) {
      options.output = normalizeRelativePath(arg.slice("--output=".length));
      continue;
    }

    throw new Error(`unknown argument: ${arg}`);
  }
  if (options.output === "") {
    throw new Error("--output must not be empty");
  }

  return options;
}

function parseSourceMode(value: string): SourceMode {
  if (value === "worktree" || value === "index") {
    return value;
  }

  throw new Error("--source must be either worktree or index");
}

function normalizeRelativePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
}

function git(repoRoot: string, args: string[], maybeOptions: { encoding: "buffer" }): Buffer;
function git(repoRoot: string, args: string[], maybeOptions?: { encoding?: BufferEncoding }): string;
function git(repoRoot: string, args: string[], maybeOptions: GitOptions = {}): string | Buffer {
  return execFileSync("git", ["-C", repoRoot, ...args], {
    encoding: maybeOptions.encoding ?? "utf8",
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

function includePath(filePath: string, outputPath: string): boolean {
  if (filePath === outputPath) {
    return false;
  }
  if (
    filePath.startsWith(".bright-builds-rules-backups/") ||
    filePath.startsWith(".planning/") ||
    filePath.startsWith("bazel-") ||
    filePath.startsWith("docs/") ||
    filePath.startsWith("packages/bitcoin-knots/") ||
    filePath.startsWith("packages/target/")
  ) {
    return false;
  }

  if (/^packages\/open-bitcoin-[^/]+\//.test(filePath)) {
    return true;
  }
  if (
    filePath.startsWith(".github/") ||
    filePath.startsWith(".githooks/") ||
    filePath.startsWith("scripts/")
  ) {
    return true;
  }

  return ROOT_CONFIG_FILES.has(filePath);
}

function readFileForSource(repoRoot: string, filePath: string, source: SourceMode): Buffer {
  if (source === "index") {
    return git(repoRoot, ["show", `:${filePath}`], { encoding: "buffer" });
  }

  return readFileSync(path.join(repoRoot, filePath));
}

function linesForContent(content: Buffer): string[] {
  const text = content.toString("utf8");
  if (text.length === 0) {
    return [];
  }

  const lines = text.split(/\r\n|\n|\r/);
  if (lines.at(-1) === "") {
    lines.pop();
  }
  return lines;
}

function isCommentLine(filePath: string, line: string): boolean {
  const trimmed = line.trim();
  if (trimmed === "") {
    return false;
  }

  const extension = path.extname(filePath);
  if ([".rs", ".js", ".mjs", ".ts"].includes(extension)) {
    return (
      trimmed.startsWith("//") ||
      trimmed.startsWith("/*") ||
      trimmed.startsWith("*") ||
      trimmed.startsWith("*/")
    );
  }
  if ([".sh", ".bash", ".yml", ".yaml", ".toml", ".bazel", ".bzl"].includes(extension)) {
    return trimmed.startsWith("#");
  }
  if (
    filePath === ".bazelrc" ||
    filePath === ".bazelversion" ||
    filePath === ".githooks/pre-commit" ||
    filePath.endsWith(".allowlist")
  ) {
    return trimmed.startsWith("#");
  }
  if (extension === ".md") {
    return trimmed.startsWith("<!--") || trimmed.startsWith("-->");
  }

  return false;
}

function classifyFile(filePath: string): string {
  const extension = path.extname(filePath);
  if (extension === ".rs") {
    return isRustTestPath(filePath) ? "Rust tests" : "Rust production";
  }
  if (extension === ".sh" || filePath === ".githooks/pre-commit") {
    return filePath.startsWith(".githooks/") ? "Hooks" : "Shell scripts";
  }
  if (extension === ".ts") {
    return "TypeScript/Bun scripts";
  }
  if (extension === ".mjs" || extension === ".js") {
    return "JavaScript scripts";
  }
  if (extension === ".bazel" || extension === ".bzl" || filePath.startsWith(".bazel")) {
    return "Bazel/Starlark";
  }
  if (extension === ".toml" || filePath.endsWith("Cargo.lock")) {
    return "TOML/config";
  }
  if (extension === ".yml" || extension === ".yaml") {
    return "YAML";
  }
  if (filePath.startsWith(".github/")) {
    return "CI/templates";
  }
  if ([".hex", ".txt", ".allowlist", ".lock"].includes(extension)) {
    return "Fixture/data";
  }

  return "Other config";
}

function isRustTestPath(filePath: string): boolean {
  return (
    filePath.includes("/tests/") ||
    filePath.endsWith("/tests.rs") ||
    /\/tests\/.*\.rs$/.test(filePath)
  );
}

function emptyCounts(): Counts {
  return {
    blank: 0,
    code: 0,
    comments: 0,
    files: 0,
    total: 0,
  };
}

function addCounts(target: Counts, counts: Counts): void {
  target.blank += counts.blank;
  target.code += counts.code;
  target.comments += counts.comments;
  target.files += counts.files;
  target.total += counts.total;
}

function countFile(filePath: string, content: Buffer): Counts {
  const lines = linesForContent(content);
  const counts = emptyCounts();
  counts.files = 1;
  counts.total = lines.length;

  for (const line of lines) {
    if (line.trim() === "") {
      counts.blank += 1;
      continue;
    }
    if (isCommentLine(filePath, line)) {
      counts.comments += 1;
      continue;
    }
    counts.code += 1;
  }

  return counts;
}

function collectMetrics(repoRoot: string, options: Options): Metrics {
  const files: FileStats[] = [];
  const hash = createHash("sha256");
  const totals = emptyCounts();

  for (const filePath of trackedPaths(repoRoot)) {
    if (!includePath(filePath, options.output)) {
      continue;
    }

    const content = readFileForSource(repoRoot, filePath, options.source);
    const counts = countFile(filePath, content);
    const category = classifyFile(filePath);
    files.push({ category, counts, path: filePath });
    addCounts(totals, counts);
    hash.update(filePath);
    hash.update("\0");
    hash.update(content);
    hash.update("\0");
  }

  return {
    categories: summarizeCategories(files),
    crates: summarizeCrates(files),
    files,
    fingerprint: hash.digest("hex"),
    totals,
  };
}

function summarizeCategories(files: FileStats[]): CategoryStats[] {
  const categories = new Map<string, Counts>();
  for (const file of files) {
    if (!categories.has(file.category)) {
      categories.set(file.category, emptyCounts());
    }
    addCounts(categories.get(file.category), file.counts);
  }

  return [...categories.entries()]
    .map(([name, counts]) => ({ name, ...counts }))
    .sort((left, right) => right.total - left.total || left.name.localeCompare(right.name));
}

function summarizeCrates(files: FileStats[]): CrateStats[] {
  const crates = new Map<string, Omit<CrateStats, "name">>();
  for (const file of files) {
    const match = file.path.match(/^packages\/(open-bitcoin-[^/]+)\//);
    if (!match) {
      continue;
    }

    const crateName = match[1];
    if (!crates.has(crateName)) {
      crates.set(crateName, {
        files: 0,
        manifestBuildLines: 0,
        productionRustLines: 0,
        testRustLines: 0,
        totalLines: 0,
      });
    }

    const crateStats = crates.get(crateName);
    crateStats.files += file.counts.files;
    crateStats.totalLines += file.counts.total;

    if (file.path.endsWith(".rs")) {
      if (isRustTestPath(file.path)) {
        crateStats.testRustLines += file.counts.total;
      } else {
        crateStats.productionRustLines += file.counts.total;
      }
      continue;
    }

    if (file.path.endsWith("Cargo.toml") || file.path.endsWith("BUILD.bazel")) {
      crateStats.manifestBuildLines += file.counts.total;
    }
  }

  return [...crates.entries()]
    .map(([name, stats]) => ({ name, ...stats }))
    .sort((left, right) => left.name.localeCompare(right.name));
}

function renderReport(metrics: Metrics, options: Options): string {
  const largestFiles = [...metrics.files]
    .sort((left, right) => right.counts.total - left.counts.total || left.path.localeCompare(right.path))
    .slice(0, 20);

  return [
    "# Lines Of Code Report",
    "",
    "Deterministic first-party LOC report for Open Bitcoin code and tooling.",
    "",
    "## Aggregate",
    "",
    table(
      ["Metric", "Value"],
      [
        ["Included files", formatNumber(metrics.totals.files)],
        ["Total lines", formatNumber(metrics.totals.total)],
        ["Code/content lines", formatNumber(metrics.totals.code)],
        ["Comment-only lines", formatNumber(metrics.totals.comments)],
        ["Blank lines", formatNumber(metrics.totals.blank)],
      ],
    ),
    "",
    "## Per-Crate Modules",
    "",
    table(
      ["Module", "Files", "Production Rust", "Test Rust", "Manifest/Build", "Total", "Test/Source"],
      metrics.crates.map((crateStats) => [
        crateStats.name,
        formatNumber(crateStats.files),
        formatNumber(crateStats.productionRustLines),
        formatNumber(crateStats.testRustLines),
        formatNumber(crateStats.manifestBuildLines),
        formatNumber(crateStats.totalLines),
        ratio(crateStats.testRustLines, crateStats.productionRustLines),
      ]),
    ),
    "",
    "## Language And Category Breakdown",
    "",
    table(
      ["Category", "Files", "Total", "Code/Content", "Comments", "Blank"],
      metrics.categories.map((category) => [
        category.name,
        formatNumber(category.files),
        formatNumber(category.total),
        formatNumber(category.code),
        formatNumber(category.comments),
        formatNumber(category.blank),
      ]),
    ),
    "",
    "## Largest Included Files",
    "",
    table(
      ["Rank", "File", "Category", "Lines"],
      largestFiles.map((file, index) => [
        String(index + 1),
        file.path,
        file.category,
        formatNumber(file.counts.total),
      ]),
    ),
    "",
    "## Metadata",
    "",
    table(
      ["Field", "Value"],
      [
        ["Source mode", "CLI-selected worktree or index; report output is mode-stable"],
        ["Input fingerprint", metrics.fingerprint],
        ["Generator command", metadataGeneratorCommand(options)],
        [
          "Included scope",
          "open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files",
        ],
        [
          "Excluded scope",
          "vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report",
        ],
      ],
    ),
    "",
  ].join("\n");
}

function table(headers: string[], rows: string[][]): string {
  return [
    `| ${headers.map(markdownCell).join(" | ")} |`,
    `| ${headers.map(() => "---").join(" | ")} |`,
    ...rows.map((row) => `| ${row.map(markdownCell).join(" | ")} |`),
  ].join("\n");
}

function markdownCell(value: string): string {
  return String(value).replaceAll("|", "\\|").replaceAll("\n", " ");
}

function formatNumber(value: number): string {
  return new Intl.NumberFormat("en-US").format(value);
}

function ratio(numerator: number, denominator: number): string {
  if (denominator === 0) {
    return "n/a";
  }

  return `${((numerator / denominator) * 100).toFixed(1)}%`;
}

function generatorCommand(options: Options): string {
  return `bun run scripts/generate-loc-report.ts --source=${options.source} --output=${options.output}`;
}

function metadataGeneratorCommand(options: Options): string {
  return `bun run scripts/generate-loc-report.ts --source=MODE --output=${options.output}`;
}

function readExistingReport(repoRoot: string, options: Options): string | null {
  if (options.source === "index") {
    try {
      return readFileForSource(repoRoot, options.output, "index").toString("utf8");
    } catch {
      return null;
    }
  }

  const outputPath = path.join(repoRoot, options.output);
  if (!existsSync(outputPath)) {
    return null;
  }

  return readFileSync(outputPath, "utf8");
}

function writeReport(repoRoot: string, output: string, report: string): void {
  const outputPath = path.join(repoRoot, output);
  mkdirSync(path.dirname(outputPath), { recursive: true });
  writeFileSync(outputPath, report, "utf8");
}

function main(): void {
  try {
    const options = parseArgs(process.argv.slice(2));
    const root = repoRoot();
    const metrics = collectMetrics(root, options);
    const report = renderReport(metrics, options);

    if (options.check) {
      const existingReport = readExistingReport(root, options);
      if (existingReport !== report) {
        console.error(`error: stale LOC report: ${options.output}`);
        console.error(`run: ${generatorCommand(options)}`);
        process.exit(1);
      }

      console.log(`LOC report is current: ${options.output}`);
      return;
    }

    writeReport(root, options.output, report);
    console.log(`Wrote ${options.output} (${formatNumber(metrics.totals.total)} lines counted).`);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`error: ${message}`);
    process.exit(1);
  }
}

main();

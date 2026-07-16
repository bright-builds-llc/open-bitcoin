import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const LIFECYCLE_ID = "124-2026-07-16T20-19-53";
const LIFECYCLE_MODE = "yolo";
const VERIFICATION_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-VERIFICATION.md";
export const PHASE124_SUMMARY_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-02-SUMMARY.md";
const LIFECYCLE_INPUTS = [
  [
    ".planning/phases/124-milestone-closeout-reconciliation/124-CONTEXT.md",
    "gsd-discuss-phase",
  ],
  [
    ".planning/phases/124-milestone-closeout-reconciliation/124-01-PLAN.md",
    "gsd-planner",
  ],
  [
    ".planning/phases/124-milestone-closeout-reconciliation/124-02-PLAN.md",
    "gsd-planner",
  ],
  [
    ".planning/phases/124-milestone-closeout-reconciliation/124-01-SUMMARY.md",
    "gsd-execute-plan",
  ],
  [PHASE124_SUMMARY_FILE, "gsd-execute-plan"],
] as const;

export function verifyPhase124CloseoutLifecycle(
  repoRoot: string,
  phaseComplete: boolean,
  failures: string[],
): void {
  const absolutePath = path.join(repoRoot, VERIFICATION_FILE);
  if (!existsSync(absolutePath)) {
    failures.push(`P124 final verification provenance missing ${VERIFICATION_FILE}`);
    return;
  }
  const maybeVerificationFrontmatter = frontmatter(readFileSync(absolutePath, "utf8"));
  if (maybeVerificationFrontmatter === null) {
    failures.push("P124 final verification provenance requires YAML frontmatter");
    return;
  }
  for (const [key, expected] of [
    ["status", "passed"],
    ["lifecycle_validated", "true"],
    ["phase_lifecycle_id", LIFECYCLE_ID],
    ["lifecycle_mode", LIFECYCLE_MODE],
    ["generated_by", "gsd-verifier"],
  ] as const) {
    requireExactFrontmatterValue(
      maybeVerificationFrontmatter,
      key,
      expected,
      "P124 final verification provenance",
      failures,
    );
  }
  if (!phaseComplete) return;
  verifyArtifactFreshness(repoRoot, maybeVerificationFrontmatter, failures);
}

function verifyArtifactFreshness(
  repoRoot: string,
  verificationFrontmatter: string,
  failures: string[],
): void {
  const maybeVerificationTime = exactGeneratedAt(
    verificationFrontmatter,
    VERIFICATION_FILE,
    failures,
  );
  for (const [file, generatedBy] of LIFECYCLE_INPUTS) {
    const absolutePath = path.join(repoRoot, file);
    if (!existsSync(absolutePath)) {
      failures.push(`P124 archive-ready lifecycle missing ${file}`);
      continue;
    }
    const maybeArtifactFrontmatter = frontmatter(readFileSync(absolutePath, "utf8"));
    if (maybeArtifactFrontmatter === null) {
      failures.push(`P124 archive-ready lifecycle requires YAML frontmatter in ${file}`);
      continue;
    }
    for (const [key, expected] of [
      ["phase_lifecycle_id", LIFECYCLE_ID],
      ["lifecycle_mode", LIFECYCLE_MODE],
      ["generated_by", generatedBy],
    ] as const) {
      requireExactFrontmatterValue(
        maybeArtifactFrontmatter,
        key,
        expected,
        `P124 archive-ready lifecycle ${file}`,
        failures,
      );
    }
    const maybeArtifactTime = exactGeneratedAt(maybeArtifactFrontmatter, file, failures);
    if (
      maybeVerificationTime !== null &&
      maybeArtifactTime !== null &&
      maybeVerificationTime <= maybeArtifactTime
    ) {
      failures.push(`P124 archive-ready lifecycle verification is stale relative to ${file}`);
    }
  }
}

function requireExactFrontmatterValue(
  frontmatterText: string,
  key: string,
  expected: string,
  label: string,
  failures: string[],
): void {
  const values = frontmatterValues(frontmatterText, key);
  if (values.length !== 1 || values[0] !== expected) {
    failures.push(`${label} requires exactly one ${key}: ${expected}`);
  }
}

function exactGeneratedAt(
  frontmatterText: string,
  file: string,
  failures: string[],
): number | null {
  const values = frontmatterValues(frontmatterText, "generated_at");
  const rawValue = values[0]?.replace(/^(["'])(.*)\1$/, "$2") ?? "";
  const generatedAt = Date.parse(rawValue);
  if (values.length !== 1 || !Number.isFinite(generatedAt)) {
    failures.push(`P124 archive-ready lifecycle ${file} requires one valid generated_at`);
    return null;
  }
  return generatedAt;
}

function frontmatter(text: string): string | null {
  const maybeMatch = text.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  return maybeMatch?.[1] ?? null;
}

function frontmatterValues(frontmatterText: string, key: string): string[] {
  const escapedKey = key.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const pattern = new RegExp(`^${escapedKey}:\\s*(.*?)\\s*$`, "gm");
  return [...frontmatterText.matchAll(pattern)].map((match) => match[1] ?? "");
}

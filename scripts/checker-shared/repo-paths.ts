import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";

const HISTORICAL_PHASE_PATH = /\.planning\/phases\/[A-Za-z0-9._-]+/g;
const MILESTONE_CHECKER_SCRIPT = /^check-(?:phase\d+|v\d+-\d+)-.*\.ts$/;

/** True when `absolutePath` is strictly inside `repoRoot`. */
export function isInsideRepo(repoRoot: string, absolutePath: string): boolean {
  const relative = path.relative(repoRoot, absolutePath);
  return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
}

/**
 * Records every `.planning/phases/...` path named by a top-level milestone
 * checker script that no longer exists, so frozen evidence cannot be pruned
 * out from under a checker that still reads it.
 */
export function checkHistoricalPhasePaths(repoRoot: string, failures: string[]): void {
  const referenced = new Set<string>();
  for (const text of readMilestoneCheckerScripts(repoRoot)) {
    for (const match of text.matchAll(HISTORICAL_PHASE_PATH)) {
      referenced.add(match[0]);
    }
  }
  for (const relative of referenced) {
    const absolute = path.resolve(repoRoot, relative);
    if (!isInsideRepo(repoRoot, absolute) || !existsSync(absolute)) {
      failures.push(`missing verifier-referenced historical phase path ${relative}`);
    }
  }
}

function readMilestoneCheckerScripts(repoRoot: string): string[] {
  const scriptsDir = path.join(repoRoot, "scripts");
  if (!existsSync(scriptsDir)) return [];
  const texts: string[] = [];
  for (const name of readdirSync(scriptsDir)) {
    if (!MILESTONE_CHECKER_SCRIPT.test(name)) continue;
    const absolute = path.join(scriptsDir, name);
    if (!isInsideRepo(repoRoot, absolute)) continue;
    try {
      texts.push(readFileSync(absolute, "utf8"));
    } catch {
      // An unreadable entry with a checker-like name holds no paths to verify.
      continue;
    }
  }
  return texts;
}

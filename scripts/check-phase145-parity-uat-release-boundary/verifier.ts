import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";

import {
  REQUIRED_UAT_COMMANDS,
  UAT_PACKAGE,
} from "./constants.ts";

const HISTORICAL_PHASE_PATH = /\.planning\/phases\/[A-Za-z0-9._-]+/g;

export function checkUatCommands(texts: Map<string, string>, failures: string[]): void {
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  const uatPackage = texts.get(UAT_PACKAGE) ?? "";
  const documented = `${runtimeGuide}\n${uatPackage}`;
  for (const command of REQUIRED_UAT_COMMANDS) {
    if (!documented.includes(command)) {
      failures.push(`missing Phase 145 UAT command ${command}`);
    }
  }
}

export function checkHistoricalPhasePaths(repoRoot: string, failures: string[]): void {
  const corpusTexts = readCheckPhaseScripts(repoRoot);
  const referenced = new Set<string>();
  for (const text of corpusTexts) {
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

function readCheckPhaseScripts(repoRoot: string): string[] {
  const scriptsDir = path.join(repoRoot, "scripts");
  if (!existsSync(scriptsDir)) return [];
  const texts: string[] = [];
  for (const name of readdirSync(scriptsDir)) {
    if (!name.startsWith("check-phase") || !name.endsWith(".ts")) continue;
    const absolute = path.join(scriptsDir, name);
    if (!isInsideRepo(repoRoot, absolute)) continue;
    try {
      texts.push(readFileSync(absolute, "utf8"));
    } catch {
      continue;
    }
  }
  return texts;
}

function isInsideRepo(repoRoot: string, absolutePath: string): boolean {
  const relative = path.relative(repoRoot, absolutePath);
  return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
}

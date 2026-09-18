import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";

import {
  FORBIDDEN_RUN_STEP_TOKENS,
  PHASE117_CHECK,
  PHASE117_CHECK_STEP,
  PHASE138_CHECK,
  PHASE138_CHECK_STEP,
  PHASE144_CHECK,
  PHASE144_CHECK_STEP,
  PHASE144_TEST,
  PHASE144_TEST_STEP,
  PHASE145_CHECK,
  PHASE145_CHECK_STEP,
  PHASE145_TEST,
  PHASE145_TEST_STEP,
  REQUIRED_UAT_COMMANDS,
  UAT_PACKAGE,
} from "./constants.ts";

// Keep these script names as source literals so the last-gate scan can name
// check-phase144-operator-flush-availability-evidence,
// check-phase145-parity-uat-release-boundary,
// check-phase117-parity-uat-release-boundary, and
// check-phase138-parity-uat-release-boundary without treating 145 as file-final.
const VISIBLE_SEQUENCE = [PHASE144_TEST, PHASE144_CHECK, PHASE145_TEST, PHASE145_CHECK];
const EXECUTABLE_SEQUENCE = [
  PHASE144_TEST_STEP,
  PHASE144_CHECK_STEP,
  PHASE145_TEST_STEP,
  PHASE145_CHECK_STEP,
];
const HISTORICAL_PHASE_PATH = /\.planning\/phases\/[A-Za-z0-9._-]+/g;

export function checkVerifier(repoRoot: string, texts: Map<string, string>, failures: string[]): void {
  const verifyText = texts.get("scripts/verify.sh") ?? "";
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = verifyText.indexOf(marker);
  const bodyStart = start + marker.length;
  const end = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  const visible = start === -1 || end === -1 ? "" : verifyText.slice(bodyStart, end);
  if (!orderedLines(visible, VISIBLE_SEQUENCE) || !orderedLines(verifyText, EXECUTABLE_SEQUENCE)) {
    failures.push("verifier-scope: visible and executable order must be Phase 144 then Phase 145");
  }

  if (!verifyText.includes(PHASE117_CHECK) || !verifyText.includes(PHASE117_CHECK_STEP)) {
    failures.push(`verifier-scope: Phase 117 BOUND gate ${PHASE117_CHECK} must remain present`);
  }
  if (!verifyText.includes(PHASE138_CHECK) || !verifyText.includes(PHASE138_CHECK_STEP)) {
    failures.push(`verifier-scope: Phase 138 closeout gate ${PHASE138_CHECK} must remain present`);
  }

  for (const command of logicalRunSteps(verifyText)) {
    const lower = command.toLowerCase();
    for (const forbidden of FORBIDDEN_RUN_STEP_TOKENS) {
      if (lower.includes(forbidden.toLowerCase())) {
        failures.push(`verifier-scope: default verifier must not run ${forbidden}`);
      }
    }
  }

  checkHistoricalPhasePaths(repoRoot, verifyText, failures);
}

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

function checkHistoricalPhasePaths(repoRoot: string, verifyText: string, failures: string[]): void {
  const corpusTexts = [verifyText, ...readCheckPhaseScripts(repoRoot)];
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

function orderedLines(text: string, requiredLines: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const required of requiredLines) {
    const index = lines.indexOf(required, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

function logicalRunSteps(text: string): string[] {
  const commands: string[] = [];
  let current = "";
  for (const rawLine of text.split("\n")) {
    const line = rawLine.trim();
    if (current === "" && !line.startsWith("run_step ")) continue;
    current = `${current} ${line}`.trim();
    if (current.endsWith("\\")) {
      current = current.slice(0, -1).trim();
      continue;
    }
    commands.push(current);
    current = "";
  }
  if (current !== "") commands.push(current);
  return commands;
}

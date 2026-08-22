import {
  FORBIDDEN_BENCH_TOKENS,
  FORBIDDEN_RUN_STEP_TOKENS,
  PHASE117_CHECK,
  PHASE117_CHECK_STEP,
  PHASE117_TEST,
  PHASE117_TEST_STEP,
  PHASE138_CHECK,
  PHASE138_CHECK_STEP,
  PHASE138_TEST,
  PHASE138_TEST_STEP,
  RECONCILIATION_CHECK,
  RECONCILIATION_CHECK_STEP,
  RECONCILIATION_TEST,
  RECONCILIATION_TEST_STEP,
  REQUIRED_BENCH_TOKEN,
  REQUIRED_UAT_COMMANDS,
  RUNNABLE_CARGO_FILTERS,
  THRESHOLD_FREE,
  UAT_PACKAGE,
} from "./constants.ts";

const VISIBLE_SEQUENCE = [
  PHASE117_TEST,
  PHASE117_CHECK,
  PHASE138_TEST,
  PHASE138_CHECK,
  RECONCILIATION_TEST,
  RECONCILIATION_CHECK,
];
const EXECUTABLE_SEQUENCE = [
  PHASE117_TEST_STEP,
  PHASE117_CHECK_STEP,
  PHASE138_TEST_STEP,
  PHASE138_CHECK_STEP,
  RECONCILIATION_TEST_STEP,
  RECONCILIATION_CHECK_STEP,
];
const PHASE_COMMAND = /bun (?:test|run) scripts\/check-phase\d+\S*/g;

export function checkVerifier(verifyText: string, failures: string[]): void {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = verifyText.indexOf(marker);
  const bodyStart = start + marker.length;
  const end = verifyText.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  const visible = start === -1 || end === -1 ? "" : verifyText.slice(bodyStart, end);
  if (!orderedLines(visible, VISIBLE_SEQUENCE) || !orderedLines(verifyText, EXECUTABLE_SEQUENCE)) {
    failures.push("verifier-scope: visible and executable order must be Phase 117 then Phase 138 then reconciliation");
  }

  const phaseCommands = [...verifyText.matchAll(PHASE_COMMAND)].map((match) => match[0]);
  const lastPhaseCommand = phaseCommands.at(-1);
  if (lastPhaseCommand !== PHASE138_CHECK) {
    failures.push(`verifier-scope: final gate must end with ${PHASE138_CHECK}`);
  }
  if (!verifyText.includes(PHASE117_CHECK)) {
    failures.push(`verifier-scope: Phase 117 BOUND gate ${PHASE117_CHECK} must remain present`);
  }

  for (const command of logicalRunSteps(verifyText)) {
    const lower = command.toLowerCase();
    for (const forbidden of FORBIDDEN_RUN_STEP_TOKENS) {
      if (lower.includes(forbidden.toLowerCase())) {
        failures.push(`verifier-scope: default verifier must not run ${forbidden}`);
      }
    }
  }
}

export function checkBenchmarks(texts: Map<string, string>, failures: string[]): void {
  const bench = texts.get("packages/open-bitcoin-bench/src/cases/mempool.rs") ?? "";
  if (!bench.includes(REQUIRED_BENCH_TOKEN)) {
    failures.push(`benchmarks: mempool bench must contain ${REQUIRED_BENCH_TOKEN}`);
  }
  for (const token of FORBIDDEN_BENCH_TOKENS) {
    if (bench.includes(token)) {
      failures.push(`benchmarks: mempool bench must not contain ${token}`);
    }
  }
  const reportChecker = texts.get("scripts/check-benchmark-report.ts") ?? "";
  if (!reportChecker.includes(THRESHOLD_FREE)) {
    failures.push(`benchmarks: check-benchmark-report.ts must contain ${THRESHOLD_FREE}`);
  }
}

export function checkUatCommands(texts: Map<string, string>, failures: string[]): void {
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  const uatPackage = texts.get(UAT_PACKAGE) ?? "";
  const documented = `${runtimeGuide}\n${uatPackage}`;
  for (const command of REQUIRED_UAT_COMMANDS) {
    if (!documented.includes(command)) {
      failures.push(`missing Phase 138 UAT command ${command}`);
    }
  }
  for (const filter of RUNNABLE_CARGO_FILTERS) {
    if (!documented.includes(filter)) {
      failures.push(`missing Phase 136/137 runnable cargo filter ${filter}`);
    }
  }
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

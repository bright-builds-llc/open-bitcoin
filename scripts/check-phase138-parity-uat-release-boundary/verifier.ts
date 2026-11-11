import {
  FORBIDDEN_BENCH_TOKENS,
  REQUIRED_BENCH_TOKEN,
  REQUIRED_UAT_COMMANDS,
  RUNNABLE_CARGO_FILTERS,
  THRESHOLD_FREE,
  UAT_PACKAGE,
} from "./constants.ts";

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

#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

const PHASE130_CHECK =
  "bun run scripts/check-phase130-resource-time-fee-primitives.ts";
const PHASE131_TEST =
  "bun test scripts/check-phase131-rolling-fee-expiry-pressure.test.ts";
const PHASE131_CHECK =
  "bun run scripts/check-phase131-rolling-fee-expiry-pressure.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";

export const PHASE131_TARGET_FILES = [
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-bench/src/cases/mempool.rs",
  "packages/open-bitcoin-mempool/src/fee/rolling.rs",
  "packages/open-bitcoin-mempool/src/pool/expiry.rs",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/pressure.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",
  "packages/open-bitcoin-mempool/src/types.rs",
  "scripts/check-phase131-rolling-fee-expiry-pressure.ts",
  "scripts/verify.sh",
  ".planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md",
  ".planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-05-PLAN.md",
] as const;

const REQUIRED_BREADCRUMB_PATHS = [
  "packages/open-bitcoin-mempool/src/fee/rolling.rs",
  "packages/open-bitcoin-mempool/src/pool/expiry.rs",
  "packages/open-bitcoin-mempool/src/pool/pressure.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",
] as const;

const FORBIDDEN_SOAK_CLAIMS = [
  "public-network soak",
  "public network soak",
  "non-deterministic soak",
  "multi-day soak",
  "live mainnet soak",
] as const;

const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "must not",
  "not ",
  "without",
  "outside",
  "out of scope",
  "deferred",
  "no ",
  "forbid",
] as const;

export function checkPhase131RollingFeeExpiryPressure(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE131_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  checkPress01AccountedTrim(repoRoot, failures);
  checkPress02TrackPackageRemoved(repoRoot, failures);
  checkPress03Halflife(repoRoot, failures);
  checkPress04Expire(repoRoot, failures);
  checkPress05OracleAndBench(repoRoot, failures);
  checkEvidenceLabels(repoRoot, failures);
  checkBreadcrumbs(repoRoot, failures);
  checkNoPublicNetworkSoak(repoRoot, failures);
  checkVerifierWiring(repoRoot, failures);
  checkDeterministicScope(repoRoot, failures);
  return failures;
}

function checkPress01AccountedTrim(repoRoot: string, failures: string[]): void {
  const pressure = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/pressure.rs",
  );
  if (
    !pressure.includes("fn trim_to_size") ||
    !pressure.includes("mempool_capacity") ||
    !pressure.includes("accounted_memory()")
  ) {
    failures.push(
      "P131 PRESS-01: accounted-memory trim against MempoolCapacity must remain the active limiter",
    );
  }
}

function checkPress02TrackPackageRemoved(
  repoRoot: string,
  failures: string[],
): void {
  const rolling = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/fee/rolling.rs",
  );
  const pressure = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/pressure.rs",
  );
  if (
    !rolling.includes("fn track_package_removed") ||
    !pressure.includes("track_package_removed")
  ) {
    failures.push(
      "P131 PRESS-02: track_package_removed bump must remain wired through pressure trim",
    );
  }
}

function checkPress03Halflife(repoRoot: string, failures: string[]): void {
  const rolling = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/fee/rolling.rs",
  );
  if (
    !rolling.includes("ROLLING_FEE_HALFLIFE_SECONDS") ||
    !rolling.includes("60 * 60 * 12") ||
    !rolling.includes("fn decay_toward") ||
    !rolling.includes("open_decay_gate_after_block")
  ) {
    failures.push(
      "P131 PRESS-03: ROLLING_FEE_HALFLIFE and block-gated decay must remain present",
    );
  }
}

function checkPress04Expire(repoRoot: string, failures: string[]): void {
  const expiry = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/expiry.rs",
  );
  const types = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/types.rs",
  );
  if (
    !expiry.includes("pub fn expire(") ||
    !types.includes("pub const DEFAULT_MEMPOOL_EXPIRY_HOURS: u64 = 336")
  ) {
    failures.push(
      "P131 PRESS-04: expire API and DEFAULT_MEMPOOL_EXPIRY_HOURS=336 must remain present",
    );
  }
}

function checkPress05OracleAndBench(repoRoot: string, failures: string[]): void {
  const oracle = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",
  );
  const bench = readTarget(
    repoRoot,
    "packages/open-bitcoin-bench/src/cases/mempool.rs",
  );
  const catalog = readTarget(repoRoot, "docs/parity/catalog/mempool-policy.md");
  if (
    !oracle.includes("sustained_pressure_oracle") ||
    !oracle.includes("recompute_resource_ledger") ||
    !oracle.includes("rolling_fee_restarts_at_zero")
  ) {
    failures.push(
      "P131 PRESS-05: sustained-pressure oracle and restart-zero tests must remain present",
    );
  }
  if (
    !bench.includes("mempool-policy.sustained-pressure-trim") ||
    !bench.includes("SUSTAINED_PRESSURE_MAX_ELAPSED") ||
    !catalog.includes("sustained-pressure-trim")
  ) {
    failures.push(
      "P131 PRESS-05: hermetic sustained-pressure bench threshold must remain verifier-reachable",
    );
  }
}

function checkEvidenceLabels(repoRoot: string, failures: string[]): void {
  const lifecycle = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  );
  if (
    !lifecycle.includes("AccountedMemory") ||
    !lifecycle.includes('"accounted_memory"') ||
    !lifecycle.includes("RollingFeeParityStatus::Active") ||
    !lifecycle.includes('"active"')
  ) {
    failures.push(
      "P131 evidence: capacityenforcement accounted_memory and rolling_fee_parity active must remain live",
    );
  }
}

function checkBreadcrumbs(repoRoot: string, failures: string[]): void {
  const breadcrumbs = readTarget(
    repoRoot,
    "docs/parity/source-breadcrumbs.json",
  );
  for (const file of REQUIRED_BREADCRUMB_PATHS) {
    if (!breadcrumbs.includes(`"${file}"`)) {
      failures.push(
        "P131 breadcrumbs: Phase 131 first-party sources must remain registered",
      );
      return;
    }
  }
}

function checkNoPublicNetworkSoak(repoRoot: string, failures: string[]): void {
  const phaseDocs = [
    ".planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md",
    ".planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-05-PLAN.md",
    "docs/parity/catalog/mempool-policy.md",
  ];
  for (const relativePath of phaseDocs) {
    const text = readTarget(repoRoot, relativePath);
    for (const paragraph of text.split(/\r?\n\s*\r?\n/)) {
      const lower = paragraph.toLowerCase();
      for (const claim of FORBIDDEN_SOAK_CLAIMS) {
        if (!lower.includes(claim)) continue;
        if (NO_CLAIM_MARKERS.some((marker) => lower.includes(marker))) continue;
        failures.push(
          "P131 no-claim: Phase 131 must not require public-network soak gates",
        );
        return;
      }
    }
  }
}

function checkVerifierWiring(repoRoot: string, failures: string[]): void {
  const verify = readTarget(repoRoot, "scripts/verify.sh");
  const heredoc = visibleCommandOrder(verify);
  const requiredVisible = [
    PHASE130_CHECK,
    PHASE131_TEST,
    PHASE131_CHECK,
    PHASE117_TEST,
  ];
  const requiredSteps = [
    `run_step "check Phase 130 resource time and fee primitives" ${PHASE130_CHECK}`,
    `run_step "test Phase 131 rolling fee expiry pressure checker" ${PHASE131_TEST}`,
    `run_step "check Phase 131 rolling fee expiry pressure" ${PHASE131_CHECK}`,
    `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
  ];
  if (!orderedLines(heredoc, requiredVisible)) {
    failures.push(
      "P131 verifier heredoc: Phase 131 pair must run between Phase 130 and the Phase 117 gate",
    );
  }
  if (!orderedLines(verify, requiredSteps)) {
    failures.push(
      "P131 verifier run_step: Phase 131 pair must run between Phase 130 and the Phase 117 gate",
    );
  }

  requireFinalPhaseChecker(heredoc, "P131 final gate heredoc order", failures);
  requireFinalPhaseChecker(
    runStepLines(verify),
    "P131 final gate run_step order",
    failures,
  );
}

function checkDeterministicScope(repoRoot: string, failures: string[]): void {
  const checker = readTarget(
    repoRoot,
    "scripts/check-phase131-rolling-fee-expiry-pressure.ts",
  );
  const forbiddenTokens = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbiddenTokens.some((token) => checker.includes(token))) {
    failures.push(
      "P131 deterministic scope: checker must remain local and public-network-free",
    );
  }
}

function requireFinalPhaseChecker(
  text: string,
  label: string,
  failures: string[],
): void {
  const phaseCommands = text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => /\bbun (?:test|run) scripts\/check-phase\d+/.test(line));
  if (!phaseCommands.at(-1)?.includes(PHASE117_CHECK)) {
    failures.push(`${label} must end with ${PHASE117_CHECK}`);
  }
}

function runStepLines(text: string): string {
  return text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.startsWith("run_step "))
    .join("\n");
}

function readTarget(repoRoot: string, relativePath: string): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) return "";
  return readFileSync(absolutePath, "utf8");
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function orderedLines(text: string, required: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const line of required) {
    const index = lines.indexOf(line, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

if (import.meta.main) {
  const failures = checkPhase131RollingFeeExpiryPressure();
  if (failures.length > 0) {
    console.error("Phase 131 rolling fee expiry pressure check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 131 rolling fee expiry and pressure validated.");
}

#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";

const DEFAULT_REPORT_PATH =
  "packages/target/benchmark-reports/open-bitcoin-bench-smoke.json";
const REQUIRED_GROUP_IDS = [
  "consensus-script",
  "block-transaction-codec",
  "chainstate",
  "mempool-policy",
  "network-wire-sync",
  "sync-runtime",
  "storage-recovery",
  "operator-runtime",
  "wallet",
  "wallet-rescan",
  "rpc-cli",
] as const;
const REQUIRED_PHASE_22_CASE_IDS = [
  "sync-runtime.headers-sync",
  "sync-runtime.block-connect",
  "storage-recovery.write-read",
  "storage-recovery.restart-reopen",
  "operator-runtime.status-render",
  "operator-runtime.dashboard-projection",
  "wallet-rescan.runtime-rescan",
] as const;
const ALLOWED_DURABILITY = new Set(["pure", "ephemeral", "durable"]);

type Options = {
  reportPath: string;
};

type BenchMeasurementReport = {
  durability: string;
  fixture: string;
  focus: string;
};

type BenchCaseReport = {
  id: string;
  measurement: BenchMeasurementReport;
};

type BenchGroupReport = {
  cases: BenchCaseReport[];
  id: string;
};

type BenchReport = {
  groups: BenchGroupReport[];
  mode: string;
  profile: {
    binary_profile: string;
    iterations_per_case: number;
    threshold_free: boolean;
  };
  schema_version: number;
};

function usage(): string {
  return `Usage: bun run scripts/check-benchmark-report.ts [--report=PATH]

Validates the generated benchmark smoke report shape and required Phase 22 runtime cases.`;
}

function parseArgs(argv: string[]): Options {
  const options: Options = {
    reportPath: DEFAULT_REPORT_PATH,
  };

  for (const arg of argv) {
    if (arg === "--help" || arg === "-h") {
      console.log(usage());
      process.exit(0);
    }
    if (arg.startsWith("--report=")) {
      options.reportPath = normalizeRelativePath(arg.slice("--report=".length));
      continue;
    }

    throw new Error(`unknown argument: ${arg}`);
  }

  return options;
}

function normalizeRelativePath(value: string): string {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
}

function repoRoot(): string {
  return execFileSync("git", ["rev-parse", "--show-toplevel"], {
    cwd: process.cwd(),
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  }).trim();
}

function requireString(value: unknown, label: string): string {
  if (typeof value === "string" && value.trim() !== "") {
    return value;
  }

  throw new Error(`${label} must be a non-empty string`);
}

function requirePositiveInteger(value: unknown, label: string): number {
  if (typeof value === "number" && Number.isInteger(value) && value > 0) {
    return value;
  }

  throw new Error(`${label} must be a positive integer`);
}

function loadReport(repoRootPath: string, reportPath: string): BenchReport {
  const absolutePath = path.isAbsolute(reportPath)
    ? reportPath
    : path.join(repoRootPath, reportPath);
  if (!existsSync(absolutePath)) {
    throw new Error(`benchmark report does not exist: ${reportPath}`);
  }

  return JSON.parse(readFileSync(absolutePath, "utf8")) as BenchReport;
}

function validateProfile(report: BenchReport): void {
  if (report.schema_version !== 2) {
    throw new Error(`expected schema_version 2, got ${report.schema_version}`);
  }
  if (!report.mode.startsWith("smoke:")) {
    throw new Error(`expected smoke report mode, got ${report.mode}`);
  }
  requirePositiveInteger(
    report.profile.iterations_per_case,
    "profile.iterations_per_case",
  );
  const binaryProfile = requireString(
    report.profile.binary_profile,
    "profile.binary_profile",
  );
  if (binaryProfile !== "debug") {
    throw new Error(
      `expected smoke report binary_profile debug, got ${binaryProfile}`,
    );
  }
  if (report.profile.threshold_free !== true) {
    throw new Error("expected threshold_free to stay true");
  }
}

function validateGroups(report: BenchReport): void {
  const groupIds = report.groups.map((group) => requireString(group.id, "group.id"));
  for (const requiredGroupId of REQUIRED_GROUP_IDS) {
    if (!groupIds.includes(requiredGroupId)) {
      throw new Error(`missing benchmark group ${requiredGroupId}`);
    }
  }
}

function validateCases(report: BenchReport): void {
  const caseIds = new Set<string>();

  for (const group of report.groups) {
    if (!Array.isArray(group.cases) || group.cases.length === 0) {
      throw new Error(`benchmark group ${group.id} has no cases`);
    }

    for (const benchCase of group.cases) {
      const caseId = requireString(benchCase.id, "case.id");
      caseIds.add(caseId);
      requireString(benchCase.measurement.focus, `${caseId} measurement.focus`);
      requireString(benchCase.measurement.fixture, `${caseId} measurement.fixture`);
      const durability = requireString(
        benchCase.measurement.durability,
        `${caseId} measurement.durability`,
      );
      if (!ALLOWED_DURABILITY.has(durability)) {
        throw new Error(
          `${caseId} measurement.durability must be one of ${Array.from(
            ALLOWED_DURABILITY,
          ).join(", ")}`,
        );
      }
    }
  }

  for (const requiredCaseId of REQUIRED_PHASE_22_CASE_IDS) {
    if (!caseIds.has(requiredCaseId)) {
      throw new Error(`missing Phase 22 benchmark case ${requiredCaseId}`);
    }
  }
}

function main(): void {
  const options = parseArgs(process.argv.slice(2));
  const repoRootPath = repoRoot();
  const report = loadReport(repoRootPath, options.reportPath);

  validateProfile(report);
  validateGroups(report);
  validateCases(report);

  console.log(`validated benchmark report: ${options.reportPath}`);
}

main();

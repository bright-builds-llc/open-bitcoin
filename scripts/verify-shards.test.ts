import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

// CI runs the full verification contract as one job per shard. These tests keep
// that split equal to the serial contract: no step dropped, none run twice.

const REPO_ROOT = path.join(import.meta.dir, "..");
const HEAVY_SUITE_SHARDS: ReadonlyArray<readonly [string, string]> = [
  ["test Phase 158 validated reorg checker", "checker-tests-158"],
  ["test Phase 159 authenticated filter RPC checker", "checker-tests-159-163"],
  ["test Phase 163 native archive transitions", "checker-tests-159-163"],
  ["test Phase 164 canonical identity boundary", "checker-tests-164"],
  ["test Phase 165 shared exact work", "checker-tests-165"],
  ["cargo test", "native-test"],
  ["cargo clippy", "checks"],
];

function runVerify(args: string[]): { status: number; stdout: string; stderr: string } {
  const result = Bun.spawnSync(["bash", "scripts/verify.sh", ...args], {
    cwd: REPO_ROOT,
    // CI=1 skips the local timing wrapper and hook installation.
    env: { ...process.env, CI: "1" },
    stdout: "pipe",
    stderr: "pipe",
  });
  return {
    status: result.exitCode,
    stdout: result.stdout.toString("utf8"),
    stderr: result.stderr.toString("utf8"),
  };
}

// Stands in for scripts/verify.sh: background shards outlive the native ones,
// and the shard named by FAKE_FAIL exits non-zero.
const FAKE_VERIFIER = `#!/usr/bin/env bash
shard="$2"
echo "$shard" >>"$FAKE_RAN"
if [[ "$shard" == "\${FAKE_FAIL:-}" ]]; then
  echo "fake $shard boom"
  exit 3
fi
if [[ "$shard" == checker-tests-* ]]; then
  sleep 1 &
  wait
fi
echo "$shard" >>"$FAKE_PASSED"
`;

function runParallelWithFakeVerifier(maybeFailingShard: string): { status: number; stderr: string; ran: string[]; passed: string[] } {
  const root = mkdtempSync(path.join(tmpdir(), "verify-parallel-"));
  try {
    mkdirSync(path.join(root, "scripts"));
    writeFileSync(path.join(root, "scripts/verify.sh"), FAKE_VERIFIER);
    const helpers = path.join(REPO_ROOT, "scripts/verify/helpers.sh");
    const result = Bun.spawnSync(["bash", "-c", `set -euo pipefail; source "${helpers}"; run_parallel_shards`], {
      cwd: root,
      env: { ...process.env, FAKE_FAIL: maybeFailingShard, FAKE_RAN: path.join(root, "ran"), FAKE_PASSED: path.join(root, "passed") },
      stdout: "pipe",
      stderr: "pipe",
    });
    const lines = (name: string) => {
      try {
        return readFileSync(path.join(root, name), "utf8").trim().split("\n").sort();
      } catch {
        return [];
      }
    };
    return { status: result.exitCode, stderr: result.stderr.toString("utf8"), ran: lines("ran"), passed: lines("passed") };
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

function listShards(): string[] {
  const result = runVerify(["--list-shards"]);
  expect(result.status).toBe(0);
  return result.stdout.trim().split("\n");
}

function planSteps(): Array<{ shard: string; label: string }> {
  const result = runVerify(["--plan"]);
  expect(result.status).toBe(0);
  return result.stdout.trim().split("\n").map((line) => {
    const [shard, label] = line.split("\t");
    return { shard, label };
  });
}

function registeredStepLabels(): string[] {
  const verifyScript = readFileSync(path.join(REPO_ROOT, "scripts/verify.sh"), "utf8");
  return [...verifyScript.matchAll(/^\s*run_step "([^"]+)"/gm)].map((match) => match[1]);
}

function ciMatrixShards(): string[] {
  const workflow = readFileSync(path.join(REPO_ROOT, ".github/workflows/ci.yml"), "utf8");
  const maybeMatrix = workflow.match(/^ {8}shard:\n((?: {10}- \S+\n)+)/m);
  if (!maybeMatrix) throw new Error("ci.yml has no verify shard matrix");
  return maybeMatrix[1].trim().split("\n").map((line) => line.trim().replace(/^- /, ""));
}

test("ci_matrix_runs_exactly_the_verifier_shards", () => {
  // Arrange
  const shards = listShards();

  // Act
  const matrix = ciMatrixShards();

  // Assert
  expect(matrix).toEqual(shards);
});

test("ci_invokes_the_profiled_verifier_once_per_matrix_shard", () => {
  // Arrange
  const workflow = readFileSync(path.join(REPO_ROOT, ".github/workflows/ci.yml"), "utf8");

  // Act
  const invocations = workflow.split("\n").filter((line) => line.includes("scripts/verify.sh") && line.includes("run:"));

  // Assert
  expect(invocations).toEqual(['        run: bash scripts/verify.sh --profile --shard "${{ matrix.shard }}"']);
});

test("plan_assigns_every_full_contract_step_to_one_known_shard", () => {
  // Arrange
  const shards = new Set(listShards());
  // "ensure git hooks" is local-only; the plan runs with CI set.
  const registered = registeredStepLabels().filter((label) => label !== "ensure git hooks");

  // Act
  const steps = planSteps();

  // Assert
  expect(steps.map((step) => step.label)).toEqual(registered);
  expect(steps.filter((step) => !shards.has(step.shard))).toEqual([]);
});

test("every_shard_owns_at_least_one_step", () => {
  // Arrange
  const shards = listShards();

  // Act
  const used = new Set(planSteps().map((step) => step.shard));

  // Assert
  expect(shards.filter((shard) => !used.has(shard))).toEqual([]);
});

test("heavy_suites_stay_in_their_dedicated_shards", () => {
  // Arrange
  const steps = new Map(planSteps().map((step) => [step.label, step.shard]));

  // Act
  const actual = HEAVY_SUITE_SHARDS.map(([label]) => [label, steps.get(label)]);

  // Assert
  expect(actual).toEqual(HEAVY_SUITE_SHARDS.map(([label, shard]) => [label, shard]));
});

test("shard_selection_is_refused_outside_the_full_contract", () => {
  // Arrange
  const args = ["--quick", "--shard", "checks"];

  // Act
  const result = runVerify(args);

  // Assert
  expect(result.status).toBe(2);
  expect(result.stderr).toContain("--shard applies only to the full verification contract");
});

test("parallel_mode_is_refused_for_a_partial_contract", () => {
  // Arrange
  const partialContracts = [["--parallel", "--shard", "checks"], ["--quick", "--parallel"], ["--fast", "--parallel"]];

  // Act
  const results = partialContracts.map((args) => runVerify(args));

  // Assert
  expect(results.map((result) => result.status)).toEqual([2, 2, 2]);
  expect(results.every((result) => result.stderr.includes("--parallel applies only to the whole full verification contract"))).toBe(true);
});

test("parallel_mode_plans_the_same_steps_as_the_serial_contract", () => {
  // Arrange
  const serial = runVerify(["--plan"]);

  // Act
  const parallel = runVerify(["--parallel", "--plan"]);

  // Assert
  expect(parallel.status).toBe(0);
  expect(parallel.stdout).toBe(serial.stdout);
});

test("parallel_run_passes_only_after_every_shard_passes", () => {
  // Arrange
  const shards = listShards().sort();

  // Act
  const result = runParallelWithFakeVerifier("");

  // Assert
  expect(result.status).toBe(0);
  expect(result.passed).toEqual(shards);
});

test("parallel_run_fails_when_a_native_shard_fails", () => {
  // Arrange
  const failingShard = "native-test";

  // Act
  const result = runParallelWithFakeVerifier(failingShard);

  // Assert
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("shard native-test failed");
  expect(result.ran).not.toContain("native-stress");
});

test("parallel_run_fails_when_a_background_shard_fails", () => {
  // Arrange
  const failingShard = "checker-tests-164";

  // Act
  const result = runParallelWithFakeVerifier(failingShard);

  // Assert
  expect(result.status).toBe(1);
  expect(result.stderr).toContain("shard checker-tests-164 failed");
  expect(result.stderr).toContain("fake checker-tests-164 boom");
});

test("unknown_shard_is_refused", () => {
  // Arrange
  const args = ["--shard", "not-a-shard"];

  // Act
  const result = runVerify(args);

  // Assert
  expect(result.status).toBe(2);
  expect(result.stderr).toContain("--shard requires one of");
});

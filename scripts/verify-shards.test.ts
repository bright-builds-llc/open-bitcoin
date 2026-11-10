import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
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

test("unknown_shard_is_refused", () => {
  // Arrange
  const args = ["--shard", "not-a-shard"];

  // Act
  const result = runVerify(args);

  // Assert
  expect(result.status).toBe(2);
  expect(result.stderr).toContain("--shard requires one of");
});

#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdtemp, mkdir, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

const CHECKER_PATH = path.join(import.meta.dir, "check-phase73-uat-verification.ts");
const PHASE_DIR = ".planning/phases/73-opt-in-uat-and-deterministic-verification";
const PLAN_FILES = [
  `${PHASE_DIR}/73-01-PLAN.md`,
  `${PHASE_DIR}/73-02-PLAN.md`,
  `${PHASE_DIR}/73-03-PLAN.md`,
  `${PHASE_DIR}/73-04-PLAN.md`,
] as const;
const REQUIRED_UAT_MATRIX_DOC_STRINGS = [
  "### Phase 73 opt-in public-mainnet UAT matrix",
  "Full-sync activation and review",
  "Stay-current/status review",
  "Same-datadir restart/resume review",
  "Status-surface comparison",
  "Live-smoke report collection",
  "Support-bundle collection",
  "Evidence proves",
  "Does not prove",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "bun run scripts/run-live-mainnet-smoke.ts",
  "deterministic fixture validation, not public-network UAT",
] as const;
const DEFAULT_PLAN_TEXTS = [
  "requirements: [VER-02]\n",
  "requirements: [VER-03]\n",
  "requirements: [VER-01, VER-02, VER-03]\n",
  "requirements: [VER-04]\n",
] as const;
const COVERAGE_ANCHORS: Record<string, readonly string[]> = {
  "packages/open-bitcoin-chainstate/tests/parity.rs": [
    "connect_disconnect_and_reorg_preserve_phase_four_outcomes",
  ],
  "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs": [
    "chainstate_snapshot_round_trips_through_storage_dto",
  ],
  "packages/open-bitcoin-node/src/sync/tests.rs": [
    "connected_active_chain_progress_survives_runtime_reopen",
    "phase70_reorg_records_bounded_persisted_evidence",
    "same_datadir_reopen_connects_best_available_branch_when_blocks_are_already_local",
    "competing_header_branch_wins_after_restart_when_it_extends_farther",
    "bounded_block_requests_use_validated_best_chain_headers_only",
    "phase70_notfound_releases_inflight_and_rotates_to_second_peer",
    "block_notfound_is_peer_attributed_no_credit",
    "phase70_duplicate_block_releases_inflight_without_credit",
    "duplicate_block_response_is_peer_attributed_no_credit",
    "phase69_tip_evidence_survives_runtime_reopen",
    "phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight",
    "same_datadir_reopen_does_not_duplicate_connected_block_getdata",
    "phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
    "bounded_unattended_cycles_preserve_resource_pressure_and_retention",
  ],
};

type CheckerRun = {
  exitCode: number;
  stderr: string;
};

const tempRoots: string[] = [];

afterEach(async () => {
  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot === undefined) {
      continue;
    }

    await rm(maybeRoot, { force: true, recursive: true });
  }
});

test("fails when the Phase 73 UAT matrix title is missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeRuntimeGuide: REQUIRED_UAT_MATRIX_DOC_STRINGS.filter(
      (needle) => needle !== "### Phase 73 opt-in public-mainnet UAT matrix",
    ).join("\n"),
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain(
    "docs/operator/runtime-guide.md missing required text: ### Phase 73 opt-in public-mainnet UAT matrix",
  );
});

test("fails when the Phase 73 plan set omits a required VER id", async () => {
  // Arrange
  const root = await createFixture({
    maybePlanTexts: ["requirements: [VER-02]\n", "requirements: [VER-03]\n", "requirements: [VER-01]\n", "\n"],
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain(
    ".planning/phases/73-opt-in-uat-and-deterministic-verification/73-*-PLAN.md missing required text: VER-04",
  );
});

test("fails when a VER-02 coverage anchor loses a required test needle", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmittedCoverageNeedle: "phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain(
    "resource_bounds in packages/open-bitcoin-node/src/sync/tests.rs missing required text: phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
  );
});

async function createFixture(options: {
  maybeRuntimeGuide?: string;
  maybePlanTexts?: readonly string[];
  maybeOmittedCoverageNeedle?: string;
}): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase73-"));
  tempRoots.push(root);

  await writeFiles(root, buildFixtureFiles(options));

  return root;
}

function buildFixtureFiles(options: {
  maybeRuntimeGuide?: string;
  maybePlanTexts?: readonly string[];
  maybeOmittedCoverageNeedle?: string;
}): Record<string, string> {
  const files: Record<string, string> = {};
  const planTexts = options.maybePlanTexts ?? DEFAULT_PLAN_TEXTS;

  for (const [index, planFile] of PLAN_FILES.entries()) {
    files[planFile] = planTexts[index] ?? "";
  }

  files["docs/operator/runtime-guide.md"] =
    options.maybeRuntimeGuide ?? REQUIRED_UAT_MATRIX_DOC_STRINGS.join("\n");

  for (const [file, needles] of Object.entries(COVERAGE_ANCHORS)) {
    files[file] = needles
      .filter((needle) => needle !== options.maybeOmittedCoverageNeedle)
      .join("\n");
  }

  return files;
}

async function writeFiles(root: string, files: Record<string, string>): Promise<void> {
  for (const [relativePath, contents] of Object.entries(files)) {
    const absolutePath = path.join(root, relativePath);
    await mkdir(path.dirname(absolutePath), { recursive: true });
    await writeFile(absolutePath, contents);
  }
}

async function runChecker(root: string): Promise<CheckerRun> {
  const child = Bun.spawn(["bun", "run", CHECKER_PATH], {
    env: {
      ...process.env,
      OPEN_BITCOIN_PHASE73_REPO_ROOT: root,
    },
    stderr: "pipe",
    stdout: "pipe",
  });

  const stderr = await new Response(child.stderr).text();
  const exitCode = await child.exited;

  return { exitCode, stderr };
}

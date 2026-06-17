#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

const CHECKER_PATH = path.join(import.meta.dir, "check-phase78-progress-guarantees.ts");
const PHASE_DIR = ".planning/phases/78-progress-guarantees-and-stall-diagnosis";
const PLAN_FILES = [
  `${PHASE_DIR}/78-01-PLAN.md`,
  `${PHASE_DIR}/78-02-PLAN.md`,
  `${PHASE_DIR}/78-03-PLAN.md`,
  `${PHASE_DIR}/78-04-PLAN.md`,
  `${PHASE_DIR}/78-05-PLAN.md`,
  `${PHASE_DIR}/78-06-PLAN.md`,
  `${PHASE_DIR}/78-07-PLAN.md`,
] as const;
const DEFAULT_PLAN_TEXTS = PLAN_FILES.map(
  () => "---\nrequirements: [PROG-01, PROG-02, PROG-03, PROG-04]\n---\n",
);
const PHASE77_CHECKER_COMMAND = "bun run scripts/check-phase77-corruption-lock-recovery.ts";
const PHASE78_TEST_COMMAND = "bun test scripts/check-phase78-progress-guarantees.test.ts";
const PHASE78_CHECKER_COMMAND = "bun run scripts/check-phase78-progress-guarantees.ts";
const DEFAULT_VERIFY_SCRIPT = [
  PHASE77_CHECKER_COMMAND,
  PHASE78_TEST_COMMAND,
  PHASE78_CHECKER_COMMAND,
].join("\n");
const SURFACE_ID = "phase78-progress-guarantees-stall-diagnosis";
const REQUIREMENTS = "PROG-01 PROG-02 PROG-03 PROG-04";
const PROGRESS_TERMS = [
  "progress_credit",
  "last_useful_work",
  "last_peer_contribution",
  "expected_progress_window",
  "no_progress_threshold",
  "stall_diagnosis",
  "validated_durable_active_chain",
  "current_at_best_known_tip",
  "storage_or_resource_pressure",
  "at_tip_waiting",
  "operator_stop",
  "local_shutdown",
].join("\n");
const PROGRESS_SENTENCE =
  "Headers, downloaded block bodies, peer messages, in-flight requests, retries, and report generation are evidence only and do not advance the credited progress";

type FixtureOptions = {
  maybeAppend?: {
    file: string;
    text: string;
  };
  maybeOmission?: {
    file: string;
    needle: string;
  };
  maybePlanTexts?: readonly string[];
  maybeVerifyScript?: string;
};

type CheckerRun = {
  exitCode: number;
};

const FILE_TEXTS: Record<string, string> = {
  "packages/open-bitcoin-node/src/status/progress_guarantee.rs": [
    "ProgressCreditEvidence",
    "ProgressCreditKind",
    "RejectedProgressActivityKind",
    "ProgressWindowEvidence",
    "NoProgressThresholdEvidence",
    "PeerContributionEvidence",
    "StalledSubsystem",
    "StallDiagnosisEvidence",
  ].join("\n"),
  "packages/open-bitcoin-node/src/status.rs": [
    "progress_credit",
    "expected_progress_window",
    "no_progress_threshold",
    "last_useful_work",
    "last_peer_contribution",
    "stall_diagnosis",
  ].join("\n"),
  "packages/open-bitcoin-node/src/sync/progress.rs": [
    "made_validated_durable_progress",
    "classify_progress_credit",
    "classify_stall_diagnosis",
  ].join("\n"),
  "packages/open-bitcoin-node/src/sync/runtime_state.rs": [
    "made_validated_durable_progress",
    "classify_progress_credit",
    "classify_stall_diagnosis",
    "write_progress_guarantee_log",
  ].join("\n"),
  "packages/open-bitcoin-node/src/sync/tests/soak.rs":
    "phase78_header_and_download_activity_do_not_credit_soak_progress\n",
  "packages/open-bitcoin-node/src/sync/tests.rs": [
    "phase78_branch_competition_does_not_credit_replacement_tip_before_connect",
    "phase78_current_at_tip_credits_stay_current_useful_work",
    "phase78_stale_inflight_cleanup_preserves_prior_credit_and_rotates_peer",
    "phase78_no_credit_peer_rotation_keeps_last_peer_contribution_without_credit",
    "phase78_validation_stall_classifies_validation_subsystem",
    "phase78_storage_resource_pressure_outranks_peer_retry_advice",
    "phase78_operator_stop_and_shutdown_classify_local_subsystems",
  ].join("\n"),
  "docs/operator/runtime-guide.md": [PROGRESS_TERMS, PROGRESS_SENTENCE].join("\n"),
  "docs/architecture/status-snapshot.md": PROGRESS_TERMS,
  "docs/architecture/operator-observability.md": [
    "progress_credit",
    "last_useful_work",
    "last_peer_contribution",
    "expected_progress_window",
    "no_progress_threshold",
    "stall_diagnosis",
  ].join("\n"),
  "docs/parity/index.json": [SURFACE_ID, REQUIREMENTS].join("\n"),
  "docs/parity/checklist.md": [SURFACE_ID, REQUIREMENTS].join("\n"),
  "docs/parity/README.md": [SURFACE_ID, REQUIREMENTS].join("\n"),
  "docs/parity/catalog/p2p.md": [SURFACE_ID, REQUIREMENTS].join("\n"),
  "docs/parity/catalog/chainstate.md": [SURFACE_ID, REQUIREMENTS].join("\n"),
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    SURFACE_ID,
    REQUIREMENTS,
  ].join("\n"),
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

test("passes when the Phase 78 fixture includes every progress guarantee anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
});

test("fails when PROG-04 is absent from Phase 78 plan frontmatter", async () => {
  // Arrange
  const root = await createFixture({
    maybePlanTexts: DEFAULT_PLAN_TEXTS.map((text) => text.replaceAll("PROG-04", "")),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when progress credit source anchors are missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "packages/open-bitcoin-node/src/status/progress_guarantee.rs",
      needle: "ProgressCreditEvidence",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when runtime projection credits header or block counters directly", async () => {
  // Arrange
  const root = await createFixture({
    maybeAppend: {
      file: "packages/open-bitcoin-node/src/sync/runtime_state.rs",
      text: "\nlet made_progress = summary.headers_received > 0 || summary.blocks_received > 0;\n",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when deterministic Phase 78 tests are missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "packages/open-bitcoin-node/src/sync/tests.rs",
      needle: "phase78_branch_competition_does_not_credit_replacement_tip_before_connect",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when a parity root omits Phase 78 requirement coverage", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/parity/catalog/p2p.md",
      needle: "PROG-03",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh wires Phase 78 before the Phase 77 checker", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      PHASE78_TEST_COMMAND,
      PHASE78_CHECKER_COMMAND,
      PHASE77_CHECKER_COMMAND,
    ].join("\n"),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh adds public-network or multi-day default gates", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: `${DEFAULT_VERIFY_SCRIPT}\nbun run scripts/run-live-mainnet-smoke.ts\n`,
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase78-checker-"));
  tempRoots.push(root);

  const maybePlanTexts = options.maybePlanTexts ?? DEFAULT_PLAN_TEXTS;
  for (let index = 0; index < PLAN_FILES.length; index += 1) {
    await writeFixtureFile(root, PLAN_FILES[index], maybePlanTexts[index] ?? "");
  }

  for (const [file, text] of Object.entries(FILE_TEXTS)) {
    await writeFixtureFile(root, file, applyTextOptions(file, text, options));
  }
  await writeFixtureFile(
    root,
    "scripts/verify.sh",
    options.maybeVerifyScript ?? DEFAULT_VERIFY_SCRIPT,
  );

  return root;
}

function applyTextOptions(file: string, text: string, options: FixtureOptions): string {
  let result = text;
  if (options.maybeOmission?.file === file) {
    result = result.replace(options.maybeOmission.needle, "");
  }
  if (options.maybeAppend?.file === file) {
    result = `${result}${options.maybeAppend.text}`;
  }

  return result;
}

async function writeFixtureFile(root: string, relativePath: string, text: string): Promise<void> {
  const absolutePath = path.join(root, relativePath);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, text);
}

function runChecker(root: string): CheckerRun {
  const child = Bun.spawnSync(["bun", "run", CHECKER_PATH], {
    env: {
      ...process.env,
      OPEN_BITCOIN_PHASE78_REPO_ROOT: root,
    },
    stdout: "pipe",
    stderr: "pipe",
  });

  return {
    exitCode: child.exitCode,
  };
}

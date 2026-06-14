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
const REQUIRED_PARITY_ROOT_STRINGS: Record<string, readonly string[]> = {
  "docs/parity/catalog/p2p.md": [
    "## Phase 73 opt-in public-mainnet UAT and deterministic verification",
    "public-mainnet full-sync, manual-peer, and",
    "restart-after-progress commands as explicit opt-in UAT only",
    "outside `bash scripts/verify.sh`",
    "scripts/run-live-mainnet-smoke.ts",
    "scripts/test-run-live-mainnet-smoke.sh",
    "open-bitcoin compatibility harness",
    "public-network CI",
    "release-blocking live sync",
  ],
  "docs/parity/catalog/chainstate.md": [
    "## VER-02 deterministic coverage map",
    "UTXO/undo persistence",
    "block connect/disconnect/reorg across restart",
    "best-chain header selection",
    "peer response failures",
    "crash recovery as durable reopen",
    "duplicate connect prevention",
    "resource bounds",
  ],
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    "docs/operator/runtime-guide.md",
    "scripts/check-phase73-uat-verification.ts",
    "scripts/verify.sh",
    "support bundle --output-dir=/tmp/open-bitcoin-support",
  ],
  "docs/parity/index.json": ["phase73-opt-in-uat-deterministic-verification"],
  "docs/parity/checklist.md": ["phase73-opt-in-uat-deterministic-verification"],
  "docs/parity/README.md": [
    "Phase 73 opt-in public-mainnet UAT and deterministic verification evidence",
    "scripts/check-phase73-uat-verification.ts",
  ],
};
const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs",
] as const;
const DEFAULT_SOURCE_BREADCRUMBS = JSON.stringify(
  {
    groups: [
      {
        breadcrumbs: ["packages/bitcoin-knots/src/net_processing.cpp"],
        files: ["packages/open-bitcoin-node/src/sync/tests.rs"],
        label: "node-sync-tests",
      },
      {
        breadcrumbs: ["packages/bitcoin-knots/src/validation.cpp"],
        files: ["packages/open-bitcoin-chainstate/tests/parity.rs"],
        label: "chainstate-engine",
      },
      {
        breadcrumbs: [],
        files: ["packages/open-bitcoin-cli/src/operator/support/evidence.rs"],
        label: "cli-operator-support-bundles",
      },
    ],
    noneReason: "Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.",
    scope: {
      exclude: ["packages/bitcoin-knots/**"],
      include: ["packages/open-bitcoin-*/src/**/*.rs", "packages/open-bitcoin-*/tests/**/*.rs"],
    },
    version: 1,
  },
  null,
  2,
);
const SOURCE_BREADCRUMBS_WITHOUT_SYNC_TESTS = JSON.stringify(
  {
    groups: [
      {
        breadcrumbs: ["packages/bitcoin-knots/src/validation.cpp"],
        files: ["packages/open-bitcoin-chainstate/tests/parity.rs"],
        label: "chainstate-engine",
      },
      {
        breadcrumbs: [],
        files: ["packages/open-bitcoin-cli/src/operator/support/evidence.rs"],
        label: "cli-operator-support-bundles",
      },
    ],
    noneReason: "Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.",
    scope: {
      exclude: ["packages/bitcoin-knots/**"],
      include: ["packages/open-bitcoin-*/src/**/*.rs", "packages/open-bitcoin-*/tests/**/*.rs"],
    },
    version: 1,
  },
  null,
  2,
);

type CheckerRun = {
  exitCode: number;
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

test("passes when the Phase 73 fixture includes every required evidence anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
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
});

test("fails when a Phase 73 parity root text is missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeParityRootOverrides: {
      "docs/parity/catalog/p2p.md": REQUIRED_PARITY_ROOT_STRINGS["docs/parity/catalog/p2p.md"]
        .filter((needle) => needle !== "## Phase 73 opt-in public-mainnet UAT and deterministic verification")
        .join("\n"),
    },
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when breadcrumb infrastructure omits referenced Rust files", async () => {
  // Arrange
  const root = await createFixture({
    maybeSourceBreadcrumbs: SOURCE_BREADCRUMBS_WITHOUT_SYNC_TESTS,
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when Phase 73 roots claim default public-network UAT or production readiness", async () => {
  // Arrange
  const defaultP2pDoc = REQUIRED_PARITY_ROOT_STRINGS["docs/parity/catalog/p2p.md"].join("\n");
  const root = await createFixture({
    maybeParityRootOverrides: {
      "docs/parity/catalog/p2p.md": [
        defaultP2pDoc,
        "Phase 73 public-network UAT is default verification.",
        "Phase 73 proves broad production-node readiness.",
      ].join("\n"),
    },
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

async function createFixture(options: {
  maybeRuntimeGuide?: string;
  maybePlanTexts?: readonly string[];
  maybeOmittedCoverageNeedle?: string;
  maybeParityRootOverrides?: Record<string, string>;
  maybeSourceBreadcrumbs?: string;
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
  maybeParityRootOverrides?: Record<string, string>;
  maybeSourceBreadcrumbs?: string;
}): Record<string, string> {
  const files: Record<string, string> = {};
  const planTexts = options.maybePlanTexts ?? DEFAULT_PLAN_TEXTS;

  for (const [index, planFile] of PLAN_FILES.entries()) {
    files[planFile] = planTexts[index] ?? "";
  }

  files["docs/operator/runtime-guide.md"] =
    options.maybeRuntimeGuide ?? REQUIRED_UAT_MATRIX_DOC_STRINGS.join("\n");
  files["scripts/verify.sh"] = [
    "bun run scripts/check-phase72-observability-evidence.ts",
    "bun run scripts/check-phase73-uat-verification.ts",
  ].join("\n");

  for (const [file, needles] of Object.entries(COVERAGE_ANCHORS)) {
    files[file] = needles
      .filter((needle) => needle !== options.maybeOmittedCoverageNeedle)
      .join("\n");
  }
  for (const [file, needles] of Object.entries(REQUIRED_PARITY_ROOT_STRINGS)) {
    files[file] = options.maybeParityRootOverrides?.[file] ?? needles.join("\n");
  }
  for (const file of REQUIRED_BREADCRUMB_FILES) {
    files[file] = files[file] ?? "fixture rust source\n";
  }
  files["docs/parity/source-breadcrumbs.json"] =
    options.maybeSourceBreadcrumbs ?? DEFAULT_SOURCE_BREADCRUMBS;
  files["scripts/check-parity-breadcrumbs.ts"] = "fixture breadcrumb checker\n";

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
  const child = Bun.spawnSync(["bun", "run", CHECKER_PATH], {
    env: {
      ...process.env,
      OPEN_BITCOIN_PHASE73_REPO_ROOT: root,
    },
    stderr: "pipe",
    stdout: "pipe",
  });

  return { exitCode: child.exitCode };
}

import path from "node:path";

export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE73_REPO_ROOT";
export const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
export const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "../..") : path.resolve(maybeRepoRoot);
export const PHASE_DIR = ".planning/phases/73-opt-in-uat-and-deterministic-verification";
export const PHASE73_SURFACE_ID = "phase73-opt-in-uat-deterministic-verification";
export const PHASE73_REGRESSION_TEST_COMMAND = "bun test scripts/check-phase73-uat-verification.test.ts";
export const PHASE73_CHECKER_COMMAND = `env -u ${REPO_ROOT_OVERRIDE_ENV} bun run scripts/check-phase73-uat-verification.ts`;
export const PLAN_FILES = [
  `${PHASE_DIR}/73-01-PLAN.md`,
  `${PHASE_DIR}/73-02-PLAN.md`,
  `${PHASE_DIR}/73-03-PLAN.md`,
  `${PHASE_DIR}/73-04-PLAN.md`,
] as const;
export const REQUIREMENT_IDS = ["VER-01", "VER-02", "VER-03", "VER-04"] as const;
export const REQUIRED_VER02_BEHAVIORS = [
  "durable_utxo_undo_writes",
  "block_connect_disconnect_reorg_across_restart",
  "best_chain_header_selection",
  "peer_response_failures",
  "crash_recovery_durable_reopen",
  "duplicate_connect_prevention",
  "resource_bounds",
] as const;
export const HERMETIC_COVERAGE_FILES = [
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
] as const;
export const PARITY_CLOSEOUT_FILES = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
] as const;
export const REQUIRED_UAT_MATRIX_DOC_STRINGS = [
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
export const REQUIRED_PARITY_ROOT_STRINGS = {
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
} as const satisfies Record<(typeof PARITY_CLOSEOUT_FILES)[number], readonly string[]>;
export const REQUIRED_CLOSEOUT_FILES = [
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-parity-breadcrumbs.ts",
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs",
  "scripts/run-live-mainnet-smoke.ts",
  "scripts/test-run-live-mainnet-smoke.sh",
] as const;
export const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs",
] as const;
export const REQUIRED_DEFERRED_SCOPE_STRINGS = [
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet",
  "migration apply mode",
  "signed packaging",
  "Windows service support",
  "GUI",
  "hosted dashboards",
  "broad production-node readiness",
  "public-network CI",
  "release-blocking live sync",
] as const;
export const FORBIDDEN_PHASE73_CLAIM_STRINGS = [
  "Phase 73 public-network UAT is default verification.",
  "Phase 73 proves broad production-node readiness.",
] as const;
export const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl",
  "launchctl",
  "-openbitcoinsync=mainnet-ibd",
  "openbitcoinsync=mainnet-ibd",
  "current-tip timing",
  "wall-clock release gate",
] as const;

export type Ver02Behavior = (typeof REQUIRED_VER02_BEHAVIORS)[number];

export type CoverageAnchor = {
  file: (typeof HERMETIC_COVERAGE_FILES)[number];
  needles: readonly string[];
};

export type CoverageEntry = {
  behavior: Ver02Behavior;
  anchors: readonly CoverageAnchor[];
};

export type SourceBreadcrumbFileGroup = {
  files?: unknown;
};

export type SourceBreadcrumbs = {
  groups?: unknown;
};

export type ParityIndex = {
  checklist?: unknown;
};

export type ParityChecklist = {
  surfaces?: unknown;
};

export const VER02_COVERAGE = [
  {
    behavior: "durable_utxo_undo_writes",
    anchors: [
      {
        file: "packages/open-bitcoin-chainstate/tests/parity.rs",
        needles: ["connect_disconnect_and_reorg_preserve_phase_four_outcomes"],
      },
      {
        file: "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs",
        needles: ["chainstate_snapshot_round_trips_through_storage_dto"],
      },
    ],
  },
  {
    behavior: "block_connect_disconnect_reorg_across_restart",
    anchors: [
      {
        file: "packages/open-bitcoin-node/src/sync/tests.rs",
        needles: [
          "connected_active_chain_progress_survives_runtime_reopen",
          "phase70_reorg_records_bounded_persisted_evidence",
          "same_datadir_reopen_connects_best_available_branch_when_blocks_are_already_local",
        ],
      },
    ],
  },
  {
    behavior: "best_chain_header_selection",
    anchors: [
      {
        file: "packages/open-bitcoin-node/src/sync/tests.rs",
        needles: [
          "competing_header_branch_wins_after_restart_when_it_extends_farther",
          "bounded_block_requests_use_validated_best_chain_headers_only",
        ],
      },
    ],
  },
  {
    behavior: "peer_response_failures",
    anchors: [
      {
        file: "packages/open-bitcoin-node/src/sync/tests.rs",
        needles: [
          "phase70_notfound_releases_inflight_and_rotates_to_second_peer",
          "block_notfound_is_peer_attributed_no_credit",
          "phase70_duplicate_block_releases_inflight_without_credit",
          "duplicate_block_response_is_peer_attributed_no_credit",
        ],
      },
    ],
  },
  {
    behavior: "crash_recovery_durable_reopen",
    anchors: [
      {
        file: "packages/open-bitcoin-node/src/sync/tests.rs",
        needles: [
          "connected_active_chain_progress_survives_runtime_reopen",
          "phase69_tip_evidence_survives_runtime_reopen",
          "phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight",
        ],
      },
    ],
  },
  {
    behavior: "duplicate_connect_prevention",
    anchors: [
      {
        file: "packages/open-bitcoin-node/src/sync/tests.rs",
        needles: [
          "same_datadir_reopen_does_not_duplicate_connected_block_getdata",
          "phase70_duplicate_block_releases_inflight_without_credit",
          "duplicate_block_response_is_peer_attributed_no_credit",
        ],
      },
    ],
  },
  {
    behavior: "resource_bounds",
    anchors: [
      {
        file: "packages/open-bitcoin-node/src/sync/tests.rs",
        needles: [
          "phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network",
          "bounded_unattended_cycles_preserve_resource_pressure_and_retention",
        ],
      },
    ],
  },
] as const satisfies readonly CoverageEntry[];

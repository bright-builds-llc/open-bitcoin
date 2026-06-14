#!/usr/bin/env bun

import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR = ".planning/phases/73-opt-in-uat-and-deterministic-verification";
const PLAN_FILES = [
  `${PHASE_DIR}/73-01-PLAN.md`,
  `${PHASE_DIR}/73-02-PLAN.md`,
  `${PHASE_DIR}/73-03-PLAN.md`,
  `${PHASE_DIR}/73-04-PLAN.md`,
] as const;
const REQUIREMENT_IDS = ["VER-01", "VER-02", "VER-03", "VER-04"] as const;
const REQUIRED_VER02_BEHAVIORS = [
  "durable_utxo_undo_writes",
  "block_connect_disconnect_reorg_across_restart",
  "best_chain_header_selection",
  "peer_response_failures",
  "crash_recovery_durable_reopen",
  "duplicate_connect_prevention",
  "resource_bounds",
] as const;
const HERMETIC_COVERAGE_FILES = [
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
] as const;
const FORBIDDEN_DEFAULT_VERIFICATION_NEEDLES = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl",
  "launchctl",
  "openbitcoinsync=mainnet-ibd",
] as const;

type Ver02Behavior = (typeof REQUIRED_VER02_BEHAVIORS)[number];

type CoverageAnchor = {
  file: (typeof HERMETIC_COVERAGE_FILES)[number];
  needles: readonly string[];
};

type CoverageEntry = {
  behavior: Ver02Behavior;
  anchors: readonly CoverageAnchor[];
};

const VER02_COVERAGE = [
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

function repoPath(relativePath: string): string {
  return path.join(REPO_ROOT, relativePath);
}

async function readText(relativePath: string, failures: string[]): Promise<string> {
  const file = Bun.file(repoPath(relativePath));
  if (!(await file.exists())) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return file.text();
}

async function readJoined(files: readonly string[], failures: string[]): Promise<string> {
  const parts = [];
  for (const file of files) {
    parts.push(await readText(file, failures));
  }

  return parts.join("\n");
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain default verification command: ${needle}`);
  }
}

function verifyCoverageBehaviors(failures: string[]): void {
  const observed = new Set(VER02_COVERAGE.map((entry) => entry.behavior));
  if (observed.size !== VER02_COVERAGE.length) {
    failures.push("VER02_COVERAGE must not contain duplicate behavior keys");
  }

  for (const behavior of REQUIRED_VER02_BEHAVIORS) {
    if (!observed.has(behavior)) {
      failures.push(`VER02_COVERAGE missing behavior: ${behavior}`);
    }
  }

  for (const entry of VER02_COVERAGE) {
    if (!REQUIRED_VER02_BEHAVIORS.includes(entry.behavior)) {
      failures.push(`VER02_COVERAGE contains unexpected behavior: ${entry.behavior}`);
    }
  }

  if (VER02_COVERAGE.length !== REQUIRED_VER02_BEHAVIORS.length) {
    failures.push("VER02_COVERAGE must contain exactly the required VER-02 behavior keys");
  }
}

async function verifyRequirements(failures: string[]): Promise<void> {
  const planText = await readJoined(PLAN_FILES, failures);
  for (const requirementId of REQUIREMENT_IDS) {
    requireContains(planText, requirementId, `${PHASE_DIR}/73-*-PLAN.md`, failures);
  }
}

async function verifyCoverageAnchors(failures: string[]): Promise<void> {
  for (const entry of VER02_COVERAGE) {
    for (const anchor of entry.anchors) {
      const text = await readText(anchor.file, failures);
      for (const needle of anchor.needles) {
        requireContains(text, needle, `${entry.behavior} in ${anchor.file}`, failures);
      }
    }
  }
}

function verifyHermeticCoverageFiles(failures: string[]): void {
  const allowed = new Set<string>(HERMETIC_COVERAGE_FILES);
  for (const entry of VER02_COVERAGE) {
    for (const anchor of entry.anchors) {
      if (!allowed.has(anchor.file)) {
        failures.push(`${entry.behavior} uses non-hermetic coverage file: ${anchor.file}`);
      }
    }
  }
}

async function verifyDefaultVerificationBoundary(failures: string[]): Promise<void> {
  const verifyScript = await readText("scripts/verify.sh", failures);
  for (const needle of FORBIDDEN_DEFAULT_VERIFICATION_NEEDLES) {
    requireNotContains(verifyScript, needle, "scripts/verify.sh", failures);
  }
}

async function main(): Promise<void> {
  const failures: string[] = [];
  verifyCoverageBehaviors(failures);
  verifyHermeticCoverageFiles(failures);
  await verifyRequirements(failures);
  await verifyCoverageAnchors(failures);
  await verifyDefaultVerificationBoundary(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 73 opt-in UAT and deterministic verification evidence");
}

await main();

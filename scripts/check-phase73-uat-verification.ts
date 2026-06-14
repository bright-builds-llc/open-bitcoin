#!/usr/bin/env bun

import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE73_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);
const PHASE_DIR = ".planning/phases/73-opt-in-uat-and-deterministic-verification";
const PHASE73_SURFACE_ID = "phase73-opt-in-uat-deterministic-verification";
const PHASE73_REGRESSION_TEST_COMMAND = "bun test scripts/check-phase73-uat-verification.test.ts";
const PHASE73_CHECKER_COMMAND = `env -u ${REPO_ROOT_OVERRIDE_ENV} bun run scripts/check-phase73-uat-verification.ts`;
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
const PARITY_CLOSEOUT_FILES = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
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
const REQUIRED_PARITY_ROOT_STRINGS = {
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
const REQUIRED_CLOSEOUT_FILES = [
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-parity-breadcrumbs.ts",
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs",
  "scripts/run-live-mainnet-smoke.ts",
  "scripts/test-run-live-mainnet-smoke.sh",
] as const;
const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-chainstate/tests/parity.rs",
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs",
] as const;
const REQUIRED_DEFERRED_SCOPE_STRINGS = [
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
const FORBIDDEN_PHASE73_CLAIM_STRINGS = [
  "Phase 73 public-network UAT is default verification.",
  "Phase 73 proves broad production-node readiness.",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
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

type Ver02Behavior = (typeof REQUIRED_VER02_BEHAVIORS)[number];

type CoverageAnchor = {
  file: (typeof HERMETIC_COVERAGE_FILES)[number];
  needles: readonly string[];
};

type CoverageEntry = {
  behavior: Ver02Behavior;
  anchors: readonly CoverageAnchor[];
};

type SourceBreadcrumbFileGroup = {
  files?: unknown;
};

type SourceBreadcrumbs = {
  groups?: unknown;
};

type ParityIndex = {
  checklist?: unknown;
};

type ParityChecklist = {
  surfaces?: unknown;
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
    failures.push(`${label} must not contain default verification command or timing gate: ${needle}`);
  }
}

async function requireFileExists(relativePath: string, failures: string[]): Promise<void> {
  const file = Bun.file(repoPath(relativePath));
  if (!(await file.exists())) {
    failures.push(`missing required file: ${relativePath}`);
  }
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function truncateProcessOutput(text: string): string {
  const maxLength = 1_200;
  if (text.length <= maxLength) {
    return text;
  }

  return `${text.slice(0, maxLength)}...`;
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

async function verifyCoverageMap(failures: string[]): Promise<void> {
  verifyCoverageBehaviors(failures);
  verifyHermeticCoverageFiles(failures);
  await verifyCoverageAnchors(failures);
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

function verifyRequirementIds(
  maybeRequirements: unknown,
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(maybeRequirements)) {
    failures.push(`${label} must contain a requirements array`);
    return;
  }

  const requirements = maybeRequirements.filter((requirement) => typeof requirement === "string");
  if (requirements.length !== maybeRequirements.length) {
    failures.push(`${label} requirements must contain only strings`);
    return;
  }

  for (const requirementId of REQUIREMENT_IDS) {
    if (!requirements.includes(requirementId)) {
      failures.push(`${label} missing required Phase 73 requirement: ${requirementId}`);
    }
  }

  if (requirements.length !== REQUIREMENT_IDS.length) {
    failures.push(`${label} must list exactly the Phase 73 requirement IDs`);
  }
}

async function verifyParityIndexRequirements(failures: string[]): Promise<void> {
  const indexText = await readText("docs/parity/index.json", failures);
  if (indexText.length === 0) {
    return;
  }

  let parsed: ParityIndex;
  try {
    parsed = JSON.parse(indexText) as ParityIndex;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    failures.push(`docs/parity/index.json is invalid JSON: ${message}`);
    return;
  }

  if (!isRecord(parsed) || !isRecord(parsed.checklist)) {
    failures.push("docs/parity/index.json must contain a checklist object");
    return;
  }

  const checklist = parsed.checklist as ParityChecklist;
  if (!Array.isArray(checklist.surfaces)) {
    failures.push("docs/parity/index.json checklist must contain a surfaces array");
    return;
  }

  const maybeSurface = checklist.surfaces.find(
    (surface) => isRecord(surface) && surface.id === PHASE73_SURFACE_ID,
  );
  if (!isRecord(maybeSurface)) {
    failures.push(`docs/parity/index.json missing checklist surface: ${PHASE73_SURFACE_ID}`);
    return;
  }

  verifyRequirementIds(
    maybeSurface.requirements,
    `docs/parity/index.json ${PHASE73_SURFACE_ID}`,
    failures,
  );
}

async function verifyChecklistRequirements(failures: string[]): Promise<void> {
  const checklistText = await readText("docs/parity/checklist.md", failures);
  const maybeSurfaceLine = checklistText
    .split("\n")
    .find((line) => line.includes(`| \`${PHASE73_SURFACE_ID}\` |`));

  if (maybeSurfaceLine === undefined) {
    failures.push(`docs/parity/checklist.md missing surface row: ${PHASE73_SURFACE_ID}`);
    return;
  }

  const expectedRequirements = REQUIREMENT_IDS.map((requirementId) => `\`${requirementId}\``).join(
    ", ",
  );
  requireContains(
    maybeSurfaceLine,
    expectedRequirements,
    "docs/parity/checklist.md Phase 73 row",
    failures,
  );
}

async function verifyParityLedgerRequirements(failures: string[]): Promise<void> {
  await verifyParityIndexRequirements(failures);
  await verifyChecklistRequirements(failures);
}

async function verifyUatMatrixDocs(failures: string[]): Promise<void> {
  const runtimeGuide = await readText("docs/operator/runtime-guide.md", failures);
  for (const needle of REQUIRED_UAT_MATRIX_DOC_STRINGS) {
    requireContains(runtimeGuide, needle, "docs/operator/runtime-guide.md", failures);
  }
}

async function verifyVerifyScript(failures: string[]): Promise<void> {
  const verifyScript = await readText("scripts/verify.sh", failures);
  const phase72 = "bun run scripts/check-phase72-observability-evidence.ts";
  requireContains(verifyScript, phase72, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE73_REGRESSION_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE73_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const phase72Index = verifyScript.indexOf(phase72);
  const phase73TestIndex = verifyScript.indexOf(PHASE73_REGRESSION_TEST_COMMAND);
  const phase73Index = verifyScript.indexOf(PHASE73_CHECKER_COMMAND);
  if (
    phase72Index === -1 ||
    phase73TestIndex === -1 ||
    phase73Index === -1 ||
    phase73TestIndex < phase72Index ||
    phase73Index < phase73TestIndex
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 73 regression test and hardened checker after the Phase 72 checker",
    );
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

async function verifyParityRootText(failures: string[]): Promise<void> {
  for (const file of PARITY_CLOSEOUT_FILES) {
    const text = await readText(file, failures);
    for (const needle of REQUIRED_PARITY_ROOT_STRINGS[file]) {
      requireContains(text, needle, file, failures);
    }
  }
}

async function verifyDeferredScopeNonClaims(failures: string[]): Promise<void> {
  const closeoutText = await readJoined(PARITY_CLOSEOUT_FILES, failures);
  for (const needle of REQUIRED_DEFERRED_SCOPE_STRINGS) {
    requireContains(closeoutText, needle, "Phase 73 parity closeout roots", failures);
  }

  for (const forbidden of FORBIDDEN_PHASE73_CLAIM_STRINGS) {
    if (closeoutText.includes(forbidden)) {
      failures.push(`Phase 73 parity closeout roots must not claim: ${forbidden}`);
    }
  }
}

async function verifyCloseoutFilesExist(failures: string[]): Promise<void> {
  for (const file of REQUIRED_CLOSEOUT_FILES) {
    await requireFileExists(file, failures);
  }
}

async function verifySourceBreadcrumbRegistry(failures: string[]): Promise<void> {
  const breadcrumbText = await readText("docs/parity/source-breadcrumbs.json", failures);
  if (breadcrumbText.length === 0) {
    return;
  }

  let parsed: SourceBreadcrumbs;
  try {
    parsed = JSON.parse(breadcrumbText) as SourceBreadcrumbs;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    failures.push(`docs/parity/source-breadcrumbs.json is invalid JSON: ${message}`);
    return;
  }

  if (!isRecord(parsed) || !Array.isArray(parsed.groups)) {
    failures.push("docs/parity/source-breadcrumbs.json must contain a groups array");
    return;
  }

  const registeredFiles = new Set<string>();
  for (const [index, group] of parsed.groups.entries()) {
    const typedGroup = group as SourceBreadcrumbFileGroup;
    if (!isRecord(typedGroup) || !Array.isArray(typedGroup.files)) {
      failures.push(`docs/parity/source-breadcrumbs.json group ${index} must contain a files array`);
      continue;
    }

    for (const file of typedGroup.files) {
      if (typeof file !== "string") {
        failures.push(`docs/parity/source-breadcrumbs.json group ${index} contains a non-string file`);
        continue;
      }
      registeredFiles.add(file);
    }
  }

  for (const requiredFile of REQUIRED_BREADCRUMB_FILES) {
    if (!registeredFiles.has(requiredFile)) {
      failures.push(`docs/parity/source-breadcrumbs.json missing referenced Rust file: ${requiredFile}`);
    }
  }
}

function verifyParityBreadcrumbChecker(failures: string[]): void {
  if (maybeRepoRoot !== undefined) {
    return;
  }

  const child = Bun.spawnSync(["bun", "run", "scripts/check-parity-breadcrumbs.ts", "--check"], {
    cwd: REPO_ROOT,
    stderr: "pipe",
    stdout: "pipe",
  });
  if (child.exitCode === 0) {
    return;
  }

  const decoder = new TextDecoder();
  const output = [decoder.decode(child.stdout), decoder.decode(child.stderr)]
    .filter((part) => part.trim().length > 0)
    .join("\n");
  const details = output.length > 0 ? `:\n${truncateProcessOutput(output)}` : "";
  failures.push(`scripts/check-parity-breadcrumbs.ts --check failed with exit code ${child.exitCode}${details}`);
}

async function verifyParityAndEvidenceCloseout(failures: string[]): Promise<void> {
  await verifyParityLedgerRequirements(failures);
  await verifyParityRootText(failures);
  await verifyDeferredScopeNonClaims(failures);
  await verifyCloseoutFilesExist(failures);
  await verifySourceBreadcrumbRegistry(failures);
  verifyParityBreadcrumbChecker(failures);
}

async function main(): Promise<void> {
  const failures: string[] = [];
  await verifyRequirements(failures);
  await verifyCoverageMap(failures);
  await verifyUatMatrixDocs(failures);
  await verifyVerifyScript(failures);
  await verifyParityAndEvidenceCloseout(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 73 opt-in UAT and deterministic verification evidence");
}

await main();

#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

const CHECKER_PATH = path.join(import.meta.dir, "check-phase76-resource-bounds.ts");
const PHASE_DIR = ".planning/phases/76-disk-and-resource-bound-enforcement";
const PLAN_FILES = [
  `${PHASE_DIR}/76-01-PLAN.md`,
  `${PHASE_DIR}/76-02-PLAN.md`,
  `${PHASE_DIR}/76-03-PLAN.md`,
  `${PHASE_DIR}/76-04-PLAN.md`,
  `${PHASE_DIR}/76-05-PLAN.md`,
  `${PHASE_DIR}/76-06-PLAN.md`,
] as const;
const DEFAULT_PLAN_TEXTS = [
  "requirements: [RES-05, RES-06, RES-07, RES-08]\n",
  "requirements: [RES-05, RES-06, RES-08]\n",
  "requirements: [RES-05, RES-06, RES-07, RES-08]\n",
  "requirements: [RES-05, RES-06, RES-07, RES-08]\n",
  "requirements: [RES-05, RES-06, RES-07, RES-08]\n",
  "requirements: [RES-05, RES-06, RES-07, RES-08]\n",
] as const;
const DEFAULT_VERIFY_SCRIPT = [
  "bun run scripts/check-phase75-soak-runner.ts",
  "bun test scripts/check-phase76-resource-bounds.test.ts",
  "bun run scripts/check-phase76-resource-bounds.ts",
].join("\n");

type FixtureOptions = {
  maybePlanTexts?: readonly string[];
  maybeOmission?: {
    file: string;
    needle: string;
  };
  maybeVerifyScript?: string;
};

type CheckerRun = {
  exitCode: number;
};

const FILE_TEXTS: Record<string, string> = {
  "packages/open-bitcoin-node/src/status/resource_bounds.rs": [
    "RESOURCE_BOUND_WARNING_PERCENT: u8 = 80",
    "RESOURCE_BOUND_STOP_PERCENT: u8 = 95",
    "REQUIRED_RESOURCE_MEASUREMENTS_UNAVAILABLE",
    "ResourcePressureLevel",
    "ResourceBoundKind",
    "ResourceBoundUnit",
    "ResourceBoundUsage",
    "ResourceBoundSnapshot",
    "classify_budget_pressure",
    "classify_snapshot_against_disk_budget",
    "has_unavailable_required_measurements",
    "Disk File Cache Queue Peer InFlight Log Metric SupportBundle",
    "disk file cache queue peer in_flight log metric support_bundle",
  ].join("\n"),
  "packages/open-bitcoin-node/src/status.rs": [
    "resource_bounds: FieldAvailability<ResourceBoundSnapshot>",
    "#[serde(default)]",
    "pub use resource_bounds",
  ].join("\n"),
  "packages/open-bitcoin-node/src/status/tests.rs": [
    "resource_bounds_classify_thresholds_and_full_kind_set",
    "resource_bounds_snapshot_aggregates_pressure_and_disk_budget",
    "RESOURCE_BOUND_WARNING_PERCENT",
    "RESOURCE_BOUND_STOP_PERCENT",
  ].join("\n"),
  "docs/parity/source-breadcrumbs.json":
    "packages/open-bitcoin-node/src/status/resource_bounds.rs\n",
  "packages/open-bitcoin-cli/src/operator/status/resource_bounds.rs": [
    "collect_resource_bounds",
    "fs4::available_space",
    "MAX_RESOURCE_WALK_ENTRIES",
    "disk_entry file_entry cache_entry queue_entry peer_entry in_flight_entry log_entry metric_entry support_bundle_entry SupportBundle",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/status/render.rs": [
    "Resource bounds:",
    "resource_bounds_availability",
    "overall=",
    "next_action=",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/dashboard/model.rs": [
    "Resource bounds",
    "resource_bounds",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/dashboard/model/resource_bounds.rs": [
    "resource_bounds",
    "overall=",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/runtime.rs": [
    "validate_resource_bound_preflight",
    "record_run_index",
    "collector.collect",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs": [
    "validate_resource_bound_preflight",
    "soak resource-bound preflight requires an existing datadir",
    "soak resource-bound preflight could not assess required resource bounds",
    "soak resource-bound preflight requires disk usage below 95%",
    "resource_bounds_stop_required",
    "SoakOutcomeLabel::ResourceStop",
    "classify_snapshot_against_disk_budget",
    "maybe_resource_bound_next_action",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": [
    "maybe_resource_bound_state_label",
    "resource_bound_labels",
    "maybe_resource_bound_next_action",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": [
    "Resource bound state",
    "Resource bound labels",
    "Resource bound next action",
    "Source status",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs": [
    "resource_stop_bounds",
    "normal_resource_bounds",
    "resource_bound_labels",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "resource_bound_evidence",
    "collect_resource_bound_support_evidence",
    "resource bounds are recorded as compact status summaries only",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/resource_bounds.rs": [
    "ResourceBoundSupportEvidence",
    "ResourceBoundSupportEntry",
    "collect_resource_bound_support_evidence",
    "maybe_projected_bundle_size_bytes",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/render.rs": [
    "## Resource Bound Evidence",
    "Projected support-bundle size",
    "next_action",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "collect_resource_bound_support_evidence",
    "resource_bound_evidence",
  ].join("\n"),
  "docs/operator/runtime-guide.md": [
    "### Phase 76 disk and resource-bound enforcement",
    "resource_bounds",
    "disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle",
    "80% and 95%",
    "soak start",
    "before ledger mutation",
    "resource_stop",
    "## Resource Bound Evidence",
  ].join("\n"),
  "docs/architecture/status-snapshot.md": [
    "## Phase 76 resource bounds",
    "resource_bounds",
    "RESOURCE_BOUND_WARNING_PERCENT = 80",
    "RESOURCE_BOUND_STOP_PERCENT = 95",
    "before writing a ledger",
  ].join("\n"),
  "docs/architecture/operator-observability.md": [
    "## Phase 76 resource-bound evidence",
    "resource_bounds",
    "before ledger mutation",
    "Support bundles render the same compact summary under `## Resource Bound Evidence`",
  ].join("\n"),
  "docs/parity/index.json": [
    "phase76-disk-and-resource-bound-enforcement",
    "RES-05 RES-06 RES-07 RES-08",
  ].join("\n"),
  "docs/parity/checklist.md": [
    "phase76-disk-and-resource-bound-enforcement",
    "RES-05 RES-06 RES-07 RES-08",
  ].join("\n"),
  "docs/parity/README.md": [
    "phase76-disk-and-resource-bound-enforcement",
    "scripts/check-phase76-resource-bounds.ts",
  ].join("\n"),
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    "phase76-disk-and-resource-bound-enforcement",
    "disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle",
  ].join("\n"),
  "docs/parity/release-readiness.md": [
    "Disk and resource-bound enforcement",
    "RES-05 RES-06 RES-07 RES-08",
  ].join("\n"),
  "README.md": "Typed resource-bound evidence\n",
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

test("passes when the Phase 76 fixture includes every resource-bound anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
});

test("fails when the RES-05 kind set omits support-bundle", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "packages/open-bitcoin-node/src/status/resource_bounds.rs",
      needle: "SupportBundle",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when soak preflight refusal wiring is missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs",
      needle: "soak resource-bound preflight could not assess required resource bounds",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when docs omit the no-mutation preflight wording", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/operator/runtime-guide.md",
      needle: "before ledger mutation",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh does not run the Phase 76 checker after Phase 75", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      "bun test scripts/check-phase76-resource-bounds.test.ts",
      "bun run scripts/check-phase76-resource-bounds.ts",
      "bun run scripts/check-phase75-soak-runner.ts",
    ].join("\n"),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh tries to run live network probes", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [DEFAULT_VERIFY_SCRIPT, "bun run scripts/run-live-mainnet-smoke.ts"].join(
      "\n",
    ),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase76-checker-"));
  tempRoots.push(root);

  const maybePlanTexts = options.maybePlanTexts ?? DEFAULT_PLAN_TEXTS;
  for (let index = 0; index < PLAN_FILES.length; index += 1) {
    await writeFixtureFile(root, PLAN_FILES[index], maybePlanTexts[index] ?? "");
  }

  for (const [file, text] of Object.entries(FILE_TEXTS)) {
    await writeFixtureFile(root, file, omitIfRequested(file, text, options));
  }
  await writeFixtureFile(
    root,
    "scripts/verify.sh",
    options.maybeVerifyScript ?? DEFAULT_VERIFY_SCRIPT,
  );

  return root;
}

function omitIfRequested(file: string, text: string, options: FixtureOptions): string {
  if (options.maybeOmission?.file !== file) {
    return text;
  }

  return text.replace(options.maybeOmission.needle, "");
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
      OPEN_BITCOIN_PHASE76_REPO_ROOT: root,
    },
    stdout: "pipe",
    stderr: "pipe",
  });

  return {
    exitCode: child.exitCode,
  };
}

#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE76_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);
const PHASE_DIR = ".planning/phases/76-disk-and-resource-bound-enforcement";
const RESOURCE_REQUIREMENTS = ["RES-05", "RES-06", "RES-07", "RES-08"] as const;
const PHASE76_CHECKER_COMMAND = "bun run scripts/check-phase76-resource-bounds.ts";
const PHASE76_TEST_COMMAND = "bun test scripts/check-phase76-resource-bounds.test.ts";
const PHASE75_CHECKER_COMMAND = "bun run scripts/check-phase75-soak-runner.ts";
const SURFACE_ID = "phase76-disk-and-resource-bound-enforcement";
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "sleep 86400",
  "multi-day wall-clock",
] as const;
const PLAN_FILES = [
  `${PHASE_DIR}/76-01-PLAN.md`,
  `${PHASE_DIR}/76-02-PLAN.md`,
  `${PHASE_DIR}/76-03-PLAN.md`,
  `${PHASE_DIR}/76-04-PLAN.md`,
  `${PHASE_DIR}/76-05-PLAN.md`,
  `${PHASE_DIR}/76-06-PLAN.md`,
] as const;

type AnchorMap = Record<string, readonly string[]>;

const RESOURCE_KIND_ANCHORS = [
  "Disk",
  "File",
  "Cache",
  "Queue",
  "Peer",
  "InFlight",
  "Log",
  "Metric",
  "SupportBundle",
  "disk",
  "file",
  "cache",
  "queue",
  "peer",
  "in_flight",
  "log",
  "metric",
  "support_bundle",
] as const;

const STATUS_CONTRACT_ANCHORS = {
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
    ...RESOURCE_KIND_ANCHORS,
  ],
  "packages/open-bitcoin-node/src/status.rs": [
    "resource_bounds: FieldAvailability<ResourceBoundSnapshot>",
    "#[serde(default)]",
    "pub use resource_bounds",
  ],
  "packages/open-bitcoin-node/src/status/tests.rs": [
    "resource_bounds_classify_thresholds_and_full_kind_set",
    "resource_bounds_snapshot_aggregates_pressure_and_disk_budget",
    "RESOURCE_BOUND_WARNING_PERCENT",
    "RESOURCE_BOUND_STOP_PERCENT",
  ],
  "docs/parity/source-breadcrumbs.json": [
    "packages/open-bitcoin-node/src/status/resource_bounds.rs",
  ],
} as const satisfies AnchorMap;

const STATUS_COLLECTION_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/status/resource_bounds.rs": [
    "collect_resource_bounds",
    "fs4::available_space",
    "MAX_RESOURCE_WALK_ENTRIES",
    "disk_entry",
    "file_entry",
    "cache_entry",
    "queue_entry",
    "peer_entry",
    "in_flight_entry",
    "log_entry",
    "metric_entry",
    "support_bundle_entry",
    "SupportBundle",
  ],
  "packages/open-bitcoin-cli/src/operator/status/render.rs": [
    "Resource bounds:",
    "resource_bounds_availability",
    "overall=",
    "next_action=",
  ],
  "packages/open-bitcoin-cli/src/operator/dashboard/model.rs": [
    "Resource bounds",
    "resource_bounds",
  ],
  "packages/open-bitcoin-cli/src/operator/dashboard/model/resource_bounds.rs": [
    "resource_bounds",
    "overall=",
  ],
} as const satisfies AnchorMap;

const SOAK_RESOURCE_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/soak/runtime.rs": [
    "validate_resource_bound_preflight",
    "record_run_index",
    "collector.collect",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs": [
    "validate_resource_bound_preflight",
    "soak resource-bound preflight requires an existing datadir",
    "soak resource-bound preflight could not assess required resource bounds",
    "soak resource-bound preflight requires disk usage below 95%",
    "resource_bounds_stop_required",
    "SoakOutcomeLabel::ResourceStop",
    "classify_snapshot_against_disk_budget",
    "maybe_resource_bound_next_action",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": [
    "maybe_resource_bound_state_label",
    "resource_bound_labels",
    "maybe_resource_bound_next_action",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": [
    "Resource bound state",
    "Resource bound labels",
    "Resource bound next action",
    "Source status",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs": [
    "resource_stop_bounds",
    "normal_resource_bounds",
    "resource_bound_labels",
  ],
} as const satisfies AnchorMap;

const SUPPORT_RESOURCE_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "resource_bound_evidence",
    "collect_resource_bound_support_evidence",
    "resource bounds are recorded as compact status summaries only",
  ],
  "packages/open-bitcoin-cli/src/operator/support/resource_bounds.rs": [
    "ResourceBoundSupportEvidence",
    "ResourceBoundSupportEntry",
    "collect_resource_bound_support_evidence",
    "maybe_projected_bundle_size_bytes",
  ],
  "packages/open-bitcoin-cli/src/operator/support/render.rs": [
    "## Resource Bound Evidence",
    "Projected support-bundle size",
    "next_action",
  ],
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "collect_resource_bound_support_evidence",
    "resource_bound_evidence",
  ],
} as const satisfies AnchorMap;

const DOC_AND_PARITY_ANCHORS = {
  "docs/operator/runtime-guide.md": [
    "### Phase 76 disk and resource-bound enforcement",
    "resource_bounds",
    "disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle",
    "80% and 95%",
    "soak start",
    "before ledger mutation",
    "resource_stop",
    "## Resource Bound Evidence",
  ],
  "docs/architecture/status-snapshot.md": [
    "## Phase 76 resource bounds",
    "resource_bounds",
    "RESOURCE_BOUND_WARNING_PERCENT = 80",
    "RESOURCE_BOUND_STOP_PERCENT = 95",
    "before writing a ledger",
  ],
  "docs/architecture/operator-observability.md": [
    "## Phase 76 resource-bound evidence",
    "resource_bounds",
    "before ledger mutation",
    "Support bundles render the same compact summary under `## Resource Bound Evidence`",
  ],
  "docs/parity/index.json": [SURFACE_ID, ...RESOURCE_REQUIREMENTS],
  "docs/parity/checklist.md": [SURFACE_ID, ...RESOURCE_REQUIREMENTS],
  "docs/parity/README.md": [SURFACE_ID, "scripts/check-phase76-resource-bounds.ts"],
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    SURFACE_ID,
    "disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle",
  ],
  "docs/parity/release-readiness.md": [
    "Disk and resource-bound enforcement",
    ...RESOURCE_REQUIREMENTS,
  ],
  "README.md": ["Typed resource-bound evidence"],
} as const satisfies AnchorMap;

function repoPath(relativePath: string): string {
  return path.join(REPO_ROOT, relativePath);
}

function readText(relativePath: string, failures: string[]): string {
  const absolutePath = repoPath(relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
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
    failures.push(`${label} must not contain public-network or wall-clock gate: ${needle}`);
  }
}

function requireAnchors(anchors: AnchorMap, failures: string[]): void {
  for (const [file, needles] of Object.entries(anchors)) {
    const text = readText(file, failures);
    for (const needle of needles) {
      requireContains(text, needle, file, failures);
    }
  }
}

function frontmatterFor(text: string): string {
  if (!text.startsWith("---")) {
    return text;
  }

  const endIndex = text.indexOf("\n---", 3);
  if (endIndex === -1) {
    return text;
  }

  return text.slice(0, endIndex);
}

function verifyPlanRequirements(failures: string[]): void {
  const frontmatters = PLAN_FILES.map((planFile) =>
    frontmatterFor(readText(planFile, failures)),
  ).join("\n");

  for (const requirement of RESOURCE_REQUIREMENTS) {
    requireContains(frontmatters, requirement, "Phase 76 plan frontmatter", failures);
  }
}

function verifyVerifyScript(failures: string[]): void {
  const verifyScript = readText("scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE75_CHECKER_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE76_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE76_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const phase75CheckerIndex = verifyScript.indexOf(PHASE75_CHECKER_COMMAND);
  const phase76TestIndex = verifyScript.indexOf(PHASE76_TEST_COMMAND);
  const phase76CheckerIndex = verifyScript.indexOf(PHASE76_CHECKER_COMMAND);
  if (
    phase75CheckerIndex === -1 ||
    phase76TestIndex === -1 ||
    phase76CheckerIndex === -1 ||
    phase76TestIndex < phase75CheckerIndex ||
    phase76CheckerIndex < phase76TestIndex
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 76 checker test and checker after the Phase 75 checker",
    );
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

function main(): void {
  const failures: string[] = [];

  verifyPlanRequirements(failures);
  requireAnchors(STATUS_CONTRACT_ANCHORS, failures);
  requireAnchors(STATUS_COLLECTION_ANCHORS, failures);
  requireAnchors(SOAK_RESOURCE_ANCHORS, failures);
  requireAnchors(SUPPORT_RESOURCE_ANCHORS, failures);
  requireAnchors(DOC_AND_PARITY_ANCHORS, failures);
  verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 76 disk and resource-bound enforcement boundaries");
}

main();

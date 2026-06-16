#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

const CHECKER_PATH = path.join(import.meta.dir, "check-phase77-corruption-lock-recovery.ts");
const PHASE_DIR = ".planning/phases/77-corruption-and-lock-recovery-hardening";
const PLAN_FILES = [
  `${PHASE_DIR}/77-01-PLAN.md`,
  `${PHASE_DIR}/77-02-PLAN.md`,
  `${PHASE_DIR}/77-03-PLAN.md`,
  `${PHASE_DIR}/77-04-PLAN.md`,
  `${PHASE_DIR}/77-05-PLAN.md`,
  `${PHASE_DIR}/77-06-PLAN.md`,
  `${PHASE_DIR}/77-07-PLAN.md`,
] as const;
const DEFAULT_PLAN_TEXTS = [
  "---\nrequirements: [REC-06, REC-07, REC-08]\n---\n",
  "---\nrequirements: [REC-05, REC-06, REC-08]\n---\n",
  "---\nrequirements: [REC-05, REC-06, REC-07, REC-08]\n---\n",
  "---\nrequirements: [REC-05, REC-06, REC-07, REC-08]\n---\n",
  "---\nrequirements: [REC-06, REC-07, REC-08]\n---\n",
  "---\nrequirements: [REC-05, REC-06, REC-07, REC-08]\n---\n",
  "---\nrequirements: [REC-05, REC-06, REC-07, REC-08]\n---\n",
] as const;
const PHASE76_CHECKER_COMMAND = "bun run scripts/check-phase76-resource-bounds.ts";
const PHASE77_TEST_COMMAND = "bun test scripts/check-phase77-corruption-lock-recovery.test.ts";
const PHASE77_CHECKER_COMMAND = "bun run scripts/check-phase77-corruption-lock-recovery.ts";
const DEFAULT_VERIFY_SCRIPT = [
  PHASE76_CHECKER_COMMAND,
  PHASE77_TEST_COMMAND,
  PHASE77_CHECKER_COMMAND,
].join("\n");
const NO_MUTATION_BOUNDARY =
  "Phase 77 does not delete lock files, clear recovery markers, repair stores, compact stores, reindex stores, relocate datadirs, mutate source datadirs, scan OS process tables, or upload support bundles automatically.";

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
  "packages/open-bitcoin-node/src/recovery.rs": [
    "RecoveryEvidenceSnapshot",
    "RecoveryActionClass",
    "RecoveryCause",
    "RecoveryEvidenceBasis",
    "LockEvidenceKind",
    "recovery_evidence",
    "safe_retry read_only_inspection backup_then_rebuild stop_and_escalate",
    "schema_mismatch corruption_marker partial_write unreadable_namespace backend_open_failure",
    "active_lock stale_lock_evidence concurrent_datadir_use resource_pressure",
  ].join("\n"),
  "packages/open-bitcoin-node/src/storage/lock_probe.rs": [
    "probe_fjall_lock",
    "FJALL_LOCK_FILE_NAME",
  ].join("\n"),
  "packages/open-bitcoin-node/src/storage/fjall_store.rs": [
    "fjall::Error::Locked",
    "database locked by another process",
  ].join("\n"),
  "packages/open-bitcoin-node/src/storage/fjall_store/tests.rs": [
    "lock_probe_missing_datadir_reports_unavailable_reason",
    "lock_probe_held_fjall_store_reports_active_contention_without_opening_store",
    "fjall_recovery_evidence_lock_contention_maps_typed_backend_failure",
    "fjall_recovery_evidence_schema_mismatch_maps_classifier_cause",
    "fjall_recovery_evidence_corruption_marker_maps_classifier_cause",
    "fjall_recovery_evidence_partial_write_maps_classifier_cause",
  ].join("\n"),
  "packages/open-bitcoin-node/src/status.rs":
    "pub recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>\n",
  "packages/open-bitcoin-node/src/status/tests.rs": [
    "status_recovery_evidence_snapshot_json_keeps_top_level_field_visible",
    "stale_lock_evidence",
    "concurrent_datadir_use",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs": [
    "collect_status_recovery_evidence",
    "probe_fjall_lock",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/status/render.rs": "Recovery evidence:\n",
  "packages/open-bitcoin-cli/src/operator/status/service_status.rs":
    "probe-only status does not open Fjall stores\n",
  "packages/open-bitcoin-cli/src/operator/status/tests.rs": [
    "status_recovery_evidence_stale_lock_reports_read_only_inspection",
    "status_recovery_evidence_concurrent_datadir_uses_service_and_rpc_evidence",
    "probe-only status does not open Fjall stores",
    "Recovery evidence:",
    "stale_lock_evidence",
    "concurrent_datadir_use",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support.rs":
    "recovery_evidence status.recovery_evidence\n",
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs": "status.recovery_evidence\n",
  "packages/open-bitcoin-cli/src/operator/support/render.rs": "## Recovery Evidence\n",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "support_recovery_evidence_json_projects_shared_status_evidence",
    "support_recovery_evidence_markdown_renders_operator_fields",
    "recovery_evidence",
    "stale_lock_evidence",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs": [
    "recoveryEvidence",
    "recoveryActionClass",
    "recoveryCause",
    "recoveryNextAction",
    "maybeRecoveryEvidenceUnavailableReason",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs": [
    "live_smoke_recovery_evidence_phase77_live_smoke_summary_preserves_recovery_evidence",
    "phase77_live_smoke_summary_preserves_recovery_evidence",
    "recoveryEvidence",
    "recoveryActionClass",
    "recoveryCause",
    "recoveryNextAction",
    "maybeRecoveryEvidenceUnavailableReason",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/dashboard/model/recovery.rs":
    "Recovery evidence\n",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs":
    "dashboard_recovery_evidence_row_renders_shared_status_evidence\n",
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": [
    "maybe_recovery_action_class_label",
    "maybe_recovery_cause_label",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs": [
    "maybe_recovery_action_class_label",
    "maybe_recovery_cause_label",
    "snapshot.recovery_evidence",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": [
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/tests.rs": [
    "soak_recovery_evidence_report_includes_action_class_cause_and_next_action",
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs":
    "soak_recovery_evidence_checkpoint_projects_top_level_status_evidence\n",
  "scripts/run-live-mainnet-smoke.ts": [
    "recoveryEvidence",
    "recoveryActionClass",
    "recoveryCause",
    "recoveryNextAction",
    "maybeRecoveryEvidenceUnavailableReason",
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ].join("\n"),
  "scripts/test-run-live-mainnet-smoke.sh": [
    '"recoveryEvidence": {',
    '"recoveryActionClass": "read_only_inspection"',
    '"recoveryCause": "stale_lock_evidence"',
    '"recoveryNextAction": "Inspect the datadir read-only and avoid deleting lock artifacts automatically."',
    '"maybeRecoveryEvidenceUnavailableReason": null',
    "Recovery action class: read_only_inspection",
    "Recovery cause: stale_lock_evidence",
    "Recovery next action: Inspect the datadir read-only and avoid deleting lock artifacts automatically.",
  ].join("\n"),
  "docs/operator/runtime-guide.md": [
    "bash scripts/test-run-live-mainnet-smoke.sh",
    NO_MUTATION_BOUNDARY,
  ].join("\n"),
  "docs/architecture/storage-decision.md": NO_MUTATION_BOUNDARY,
  "docs/parity/index.json": [
    "phase77-corruption-and-lock-recovery-hardening",
    "REC-05 REC-06 REC-07 REC-08",
  ].join("\n"),
  "docs/parity/README.md": [
    "phase77-corruption-and-lock-recovery-hardening",
    "REC-05 REC-06 REC-07 REC-08",
  ].join("\n"),
  "docs/parity/checklist.md": [
    "phase77-corruption-and-lock-recovery-hardening",
    "REC-05 REC-06 REC-07 REC-08",
  ].join("\n"),
  "docs/parity/release-readiness.md": [
    "phase77-corruption-and-lock-recovery-hardening",
    "REC-05 REC-06 REC-07 REC-08",
  ].join("\n"),
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    "phase77-corruption-and-lock-recovery-hardening",
    "REC-05 REC-06 REC-07 REC-08",
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

test("passes when the Phase 77 fixture includes every recovery hardening anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
});

test("fails when REC-08 is absent from Phase 77 plan frontmatter", async () => {
  // Arrange
  const root = await createFixture({
    maybePlanTexts: DEFAULT_PLAN_TEXTS.map((text) => text.replaceAll("REC-08", "")),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when recovery evidence source anchors are missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "packages/open-bitcoin-node/src/recovery.rs",
      needle: "RecoveryActionClass",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when status or support inspection calls open Fjall directly", async () => {
  // Arrange
  const root = await createFixture({
    maybeAppend: {
      file: "packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs",
      text: "\nlet _store = FjallNodeStore::open(datadir);\n",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when live-smoke recovery evidence report anchors are missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "scripts/run-live-mainnet-smoke.ts",
      needle: "maybeRecoveryEvidenceUnavailableReason",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh omits the Phase 77 checker or wires it before Phase 76", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      PHASE77_TEST_COMMAND,
      PHASE77_CHECKER_COMMAND,
      PHASE76_CHECKER_COMMAND,
    ].join("\n"),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase77-checker-"));
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
      OPEN_BITCOIN_PHASE77_REPO_ROOT: root,
    },
    stdout: "pipe",
    stderr: "pipe",
  });

  return {
    exitCode: child.exitCode,
  };
}

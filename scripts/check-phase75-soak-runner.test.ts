#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdtemp, mkdir, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

const CHECKER_PATH = path.join(import.meta.dir, "check-phase75-soak-runner.ts");
const PHASE_DIR = ".planning/phases/75-multi-day-soak-runner-and-evidence-ledger";
const PLAN_FILES = [
  `${PHASE_DIR}/75-01-PLAN.md`,
  `${PHASE_DIR}/75-02-PLAN.md`,
  `${PHASE_DIR}/75-03-PLAN.md`,
  `${PHASE_DIR}/75-04-PLAN.md`,
  `${PHASE_DIR}/75-05-PLAN.md`,
  `${PHASE_DIR}/75-06-PLAN.md`,
] as const;
const DEFAULT_PLAN_TEXTS = [
  "requirements: [SOAK-01, SOAK-02, SOAK-03]\n",
  "requirements: [SOAK-01, SOAK-02, SOAK-03]\n",
  "requirements: [SOAK-04]\n",
  "requirements: [SOAK-01, SOAK-03]\n",
  "requirements: [SOAK-01, SOAK-02, SOAK-03, SOAK-04]\n",
  "requirements: [SOAK-01, SOAK-02, SOAK-03, SOAK-04]\n",
] as const;
const RUNTIME_GUIDE_COMMANDS = [
  "### Phase 75 multi-day soak runner",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time",
  "soak resume --run-id <run-id> --checkpoint-interval-seconds 300",
  "soak stop --run-id <run-id> --reason operator-stop",
  "soak report --run-id <run-id>",
  "The durable source of truth is <datadir>/soak/run-index.json plus <datadir>/soak/runs/<run_id>/events.jsonl.",
  "A soak run can prove bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence; it does not prove inbound serving, relay, production-funds wallet safety, migration apply mode, signed packages, GUI readiness, hosted dashboards, or broad production-node readiness.",
] as const;
const SOURCE_ANCHORS: Record<string, readonly string[]> = {
  "packages/open-bitcoin-cli/src/operator/soak.rs": [
    "pub(crate) mod runtime;",
    "SoakRunId",
    "SoakBounds",
    "daemon_configured",
    "manual_peers_only",
    "no_dns_seeds",
    "elapsed_time",
    "target_height",
    "status_verdict",
    "operator_stop",
    "resource_stop",
    "recovery_stop",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/runtime.rs": [
    "run_bounded_soak_loop",
    "checkpoint_interval_seconds",
    "D-11 same-run resume matrix",
    "clean_completion",
    "operator_stop",
    "resource_stop",
    "recovery_stop",
    "unexpected_termination",
    "Soak ledger:",
    "JSON report:",
    "Markdown report:",
    "Final outcome:",
    "ledger_path",
    "json_report_path",
    "markdown_report_path",
    "latest_sequence",
    "final_outcome",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs": [
    "evaluate_stop_outcome",
    "checkpoint_status_from_snapshot",
    "SoakOutcomeLabel::ResourceStop",
    "SoakOutcomeLabel::RecoveryStop",
    "SoakOutcomeLabel::UnexpectedTermination",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": [
    "run-index.json",
    "events.jsonl",
    "report.json",
    "report.md",
    "Started",
    "Checkpoint",
    "Resume",
    "Stop",
    "Verdict",
    "ignored_trailing_bytes",
    "sync_all",
    "append_event",
    "write_atomic",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/outcome.rs": [
    "SoakOutcomeLabel",
    "CleanCompletion",
    "DiagnosedBlocker",
    "OperatorStop",
    "ResourceStop",
    "RecoveryStop",
    "UnexpectedTermination",
    "maybe_sync_stop_reason",
    "maybe_recovery_category",
    "maybe_no_progress_diagnosis",
    "maybe_full_sync_evidence",
    "maybe_process_exit",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": [
    "SoakReportProjection",
    "render_soak_report_json",
    "render_soak_report_markdown",
    "write_soak_reports",
    "is_projection",
    "source_ledger_path",
    "latest_sequence",
    "raw daemon logs",
    "raw live-smoke reports",
    "wallet material",
    "RPC credentials",
    "unbounded peer tables",
  ],
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "soak_evidence",
    "SoakSupportEvidence",
    "collect_soak_support_evidence",
    "soak ledger unavailable",
  ],
  "packages/open-bitcoin-cli/src/operator/support/render.rs": [
    "## Soak Evidence",
    "Final outcome:",
    "Source ledger:",
    "Latest sequence:",
  ],
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "phase75_soak_support_",
    "raw ledger",
    "raw daemon logs",
    "raw reports",
    "wallet material",
    "RPC credentials",
    "unbounded peer tables",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/tests.rs": [
    "soak_synthetic_interrupted_run_replays_as_unexpected_termination_resume",
    "soak_synthetic_clean_completion_refuses_same_run_resume",
    "soak_synthetic_resource_stop_report_preserves_final_outcome",
    "1_777_300_000",
    "1_777_300_060",
    "1_777_300_120",
  ],
  "packages/open-bitcoin-node/src/sync/tests.rs": ["mod soak;"],
  "packages/open-bitcoin-node/src/sync/tests/soak.rs": [
    "SYNTHETIC_SOAK_BLOCKS: usize = 96",
    "dns_seeds: Vec::new()",
    "maybe_target_header_height: Some(95)",
    "max_rounds: 64",
    "phase75_synthetic_soak_long_run_reaches_target_height_without_public_network",
    "phase75_synthetic_soak_reopen_preserves_resume_progress_without_duplicate_getdata",
    "phase75_synthetic_soak_resource_stop_uses_shared_status_evidence",
  ],
  "packages/open-bitcoin-cli/tests/operator_binary.rs": [
    "open_bitcoin_soak_start_writes_durable_ledger_and_reports",
    "open_bitcoin_soak_stop_rejects_terminal_verdict",
    "open_bitcoin_soak_report_is_projection_without_ledger_append",
    "open_bitcoin_soak_resume_refuses_clean_completion",
    "open_bitcoin_support_bundle_includes_phase75_soak_summary",
    "clean_completion",
    "raw ledger",
    "raw daemon logs",
    "raw reports",
    "wallet material",
    "RPC credentials",
    "unbounded peer tables",
  ],
};
const DOC_ANCHORS: Record<string, readonly string[]> = {
  "docs/operator/runtime-guide.md": RUNTIME_GUIDE_COMMANDS,
  "docs/architecture/status-snapshot.md": [
    "started",
    "checkpoint",
    "resume",
    "stop",
    "verdict",
    "clean_completion",
    "diagnosed_blocker",
    "operator_stop",
    "resource_stop",
    "recovery_stop",
    "unexpected_termination",
  ],
  "docs/architecture/operator-observability.md": [
    "The durable source of truth is <datadir>/soak/run-index.json plus <datadir>/soak/runs/<run_id>/events.jsonl.",
    "started",
    "checkpoint",
    "resume",
    "stop",
    "verdict",
    "clean_completion",
    "diagnosed_blocker",
    "operator_stop",
    "resource_stop",
    "recovery_stop",
    "unexpected_termination",
  ],
  "docs/parity/index.json": [
    "phase75-multi-day-soak-runner-evidence-ledger",
    "SOAK-01",
    "SOAK-02",
    "SOAK-03",
    "SOAK-04",
  ],
  "docs/parity/checklist.md": [
    "phase75-multi-day-soak-runner-evidence-ledger",
    "SOAK-01",
    "SOAK-02",
    "SOAK-03",
    "SOAK-04",
  ],
  "docs/parity/README.md": ["phase75-multi-day-soak-runner-evidence-ledger"],
  "docs/parity/catalog/p2p.md": ["phase75-multi-day-soak-runner-evidence-ledger"],
  "docs/parity/catalog/chainstate.md": ["phase75-multi-day-soak-runner-evidence-ledger"],
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    "phase75-multi-day-soak-runner-evidence-ledger",
    "bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence",
  ],
  "README.md": [
    "bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence",
  ],
};
const DEFAULT_VERIFY_SCRIPT = [
  "bun run scripts/check-v1.6-release-boundaries.ts",
  "bun test scripts/check-phase75-soak-runner.test.ts",
  "bun run scripts/check-phase75-soak-runner.ts",
].join("\n");

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

test("passes when the Phase 75 fixture includes every required soak anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
});

test("fails when the Phase 75 plan set omits a SOAK requirement id", async () => {
  // Arrange
  const root = await createFixture({
    maybePlanTexts: DEFAULT_PLAN_TEXTS.map((planText) => planText.replaceAll("SOAK-04", "")),
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when operator docs omit the Bazel soak command form", async () => {
  // Arrange
  const root = await createFixture({
    maybeDocOmission: {
      file: "docs/operator/runtime-guide.md",
      needle: RUNTIME_GUIDE_COMMANDS[2],
    },
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when docs omit the Phase 75 proof-boundary sentence", async () => {
  // Arrange
  const root = await createFixture({
    maybeDocOmission: {
      file: "docs/operator/runtime-guide.md",
      needle: RUNTIME_GUIDE_COMMANDS[7],
    },
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when source anchors omit ledger event kinds or D-11 resume matrix anchors", async () => {
  // Arrange
  const root = await createFixture({
    maybeSourceOmission: {
      file: "packages/open-bitcoin-cli/src/operator/soak/runtime.rs",
      needle: "D-11 same-run resume matrix",
    },
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when support tests omit redaction assertions", async () => {
  // Arrange
  const root = await createFixture({
    maybeSourceOmission: {
      file: "packages/open-bitcoin-cli/src/operator/support/tests.rs",
      needle: "RPC credentials",
    },
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh includes public-network soak execution strings", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [DEFAULT_VERIFY_SCRIPT, "bun run scripts/run-live-mainnet-smoke.ts"].join("\n"),
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails when verify.sh does not run the Phase 75 checker after the v1.6 checker", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      "bun test scripts/check-phase75-soak-runner.test.ts",
      "bun run scripts/check-phase75-soak-runner.ts",
      "bun run scripts/check-v1.6-release-boundaries.ts",
    ].join("\n"),
  });

  // Act
  const result = await runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

async function createFixture(options: {
  maybeDocOmission?: { file: string; needle: string };
  maybePlanTexts?: readonly string[];
  maybeSourceOmission?: { file: string; needle: string };
  maybeVerifyScript?: string;
}): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase75-"));
  tempRoots.push(root);

  await writeFiles(root, buildFixtureFiles(options));

  return root;
}

function buildFixtureFiles(options: {
  maybeDocOmission?: { file: string; needle: string };
  maybePlanTexts?: readonly string[];
  maybeSourceOmission?: { file: string; needle: string };
  maybeVerifyScript?: string;
}): Record<string, string> {
  const files: Record<string, string> = {};
  const planTexts = options.maybePlanTexts ?? DEFAULT_PLAN_TEXTS;

  for (const [index, planFile] of PLAN_FILES.entries()) {
    files[planFile] = planTexts[index] ?? "";
  }

  for (const [file, anchors] of Object.entries(SOURCE_ANCHORS)) {
    files[file] = filteredAnchors(anchors, file, options.maybeSourceOmission).join("\n");
  }
  for (const [file, anchors] of Object.entries(DOC_ANCHORS)) {
    files[file] = filteredAnchors(anchors, file, options.maybeDocOmission).join("\n");
  }
  files["scripts/verify.sh"] = options.maybeVerifyScript ?? DEFAULT_VERIFY_SCRIPT;

  return files;
}

function filteredAnchors(
  anchors: readonly string[],
  file: string,
  maybeOmission: { file: string; needle: string } | undefined,
): string[] {
  return anchors.filter(
    (anchor) => maybeOmission === undefined || maybeOmission.file !== file || anchor !== maybeOmission.needle,
  );
}

async function writeFiles(root: string, files: Record<string, string>): Promise<void> {
  for (const [relativePath, contents] of Object.entries(files)) {
    const absolutePath = path.join(root, relativePath);
    await mkdir(path.dirname(absolutePath), { recursive: true });
    await writeFile(absolutePath, contents);
  }
}

function runChecker(root: string): CheckerRun {
  const child = Bun.spawnSync(["bun", "run", CHECKER_PATH], {
    env: {
      ...process.env,
      OPEN_BITCOIN_PHASE75_REPO_ROOT: root,
    },
    stderr: "pipe",
    stdout: "pipe",
  });

  return { exitCode: child.exitCode };
}

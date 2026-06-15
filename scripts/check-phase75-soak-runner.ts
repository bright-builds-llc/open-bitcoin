#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE75_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);
const PHASE_DIR = ".planning/phases/75-multi-day-soak-runner-and-evidence-ledger";
const SOAK_REQUIREMENTS = ["SOAK-01", "SOAK-02", "SOAK-03", "SOAK-04"] as const;
const PHASE75_CHECKER_COMMAND = "bun run scripts/check-phase75-soak-runner.ts";
const PHASE75_TEST_COMMAND = "bun test scripts/check-phase75-soak-runner.test.ts";
const SURFACE_ID = "phase75-multi-day-soak-runner-evidence-ledger";
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl --user",
  "launchctl",
  "-openbitcoinsync=mainnet-ibd",
  "sleep 86400",
  "current tip",
  "multi-day wall-clock",
] as const;
const PLAN_FILES = [
  `${PHASE_DIR}/75-01-PLAN.md`,
  `${PHASE_DIR}/75-02-PLAN.md`,
  `${PHASE_DIR}/75-03-PLAN.md`,
  `${PHASE_DIR}/75-04-PLAN.md`,
  `${PHASE_DIR}/75-05-PLAN.md`,
  `${PHASE_DIR}/75-06-PLAN.md`,
] as const;

type AnchorMap = Record<string, readonly string[]>;

const SOAK_SOURCE_ANCHORS = {
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
} as const satisfies AnchorMap;

const SYNTHETIC_COVERAGE_ANCHORS = {
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
  "packages/open-bitcoin-cli/src/operator/soak/tests.rs": [
    "soak_synthetic_interrupted_run_replays_as_unexpected_termination_resume",
    "soak_synthetic_clean_completion_refuses_same_run_resume",
    "soak_synthetic_resource_stop_report_preserves_final_outcome",
    "1_777_300_000",
    "1_777_300_060",
    "1_777_300_120",
  ],
  "packages/open-bitcoin-cli/tests/operator_binary.rs": [
    "open_bitcoin_soak_start_writes_durable_ledger_and_reports",
    "open_bitcoin_soak_stop_rejects_terminal_verdict",
    "open_bitcoin_soak_report_is_projection_without_ledger_append",
    "open_bitcoin_soak_resume_refuses_clean_completion",
  ],
} as const satisfies AnchorMap;

const SUPPORT_PROJECTION_ANCHORS = {
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
  "packages/open-bitcoin-cli/tests/operator_binary.rs": [
    "open_bitcoin_support_bundle_includes_phase75_soak_summary",
    "unexpected_termination",
    "raw ledger",
    "raw daemon logs",
    "raw reports",
    "wallet material",
    "RPC credentials",
    "unbounded peer tables",
  ],
} as const satisfies AnchorMap;

const DOC_AND_PARITY_ANCHORS = {
  "docs/operator/runtime-guide.md": [
    "### Phase 75 multi-day soak runner",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir /path/to/open-bitcoin --network mainnet soak start --elapsed-time-seconds 259200 --checkpoint-interval-seconds 300 --target-height <target-height> --peer-policy daemon-configured --disk-budget-bytes 107374182400 --stop-condition elapsed-time",
    "soak resume --run-id <run-id> --checkpoint-interval-seconds 300",
    "soak stop --run-id <run-id> --reason operator-stop",
    "soak report --run-id <run-id>",
    "The durable source of truth is <datadir>/soak/run-index.json plus <datadir>/soak/runs/<run_id>/events.jsonl.",
    "A soak run can prove bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence; it does not prove inbound serving, relay, production-funds wallet safety, migration apply mode, signed packages, GUI readiness, hosted dashboards, or broad production-node readiness.",
  ],
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
  "docs/parity/index.json": [SURFACE_ID, ...SOAK_REQUIREMENTS],
  "docs/parity/checklist.md": [SURFACE_ID, ...SOAK_REQUIREMENTS],
  "docs/parity/README.md": [SURFACE_ID],
  "docs/parity/catalog/p2p.md": [SURFACE_ID],
  "docs/parity/catalog/chainstate.md": [SURFACE_ID],
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    SURFACE_ID,
    "bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence",
  ],
  "README.md": [
    "bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence",
  ],
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
    failures.push(`${label} must not contain default verification command or timing gate: ${needle}`);
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

  for (const requirement of SOAK_REQUIREMENTS) {
    requireContains(frontmatters, requirement, "Phase 75 plan frontmatter", failures);
  }
}

function verifySoakSourceAnchors(failures: string[]): void {
  requireAnchors(SOAK_SOURCE_ANCHORS, failures);
}

function verifySyntheticCoverageAnchors(failures: string[]): void {
  requireAnchors(SYNTHETIC_COVERAGE_ANCHORS, failures);
}

function verifySupportProjectionAnchors(failures: string[]): void {
  requireAnchors(SUPPORT_PROJECTION_ANCHORS, failures);
}

function verifyDocsAndParityRoots(failures: string[]): void {
  requireAnchors(DOC_AND_PARITY_ANCHORS, failures);
}

function verifyVerifyScript(failures: string[]): void {
  const verifyScript = readText("scripts/verify.sh", failures);
  const v16CheckerCommand = "bun run scripts/check-v1.6-release-boundaries.ts";

  requireContains(verifyScript, v16CheckerCommand, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE75_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE75_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const v16Index = verifyScript.indexOf(v16CheckerCommand);
  const phase75TestIndex = verifyScript.indexOf(PHASE75_TEST_COMMAND);
  const phase75CheckerIndex = verifyScript.indexOf(PHASE75_CHECKER_COMMAND);
  if (
    v16Index === -1 ||
    phase75TestIndex === -1 ||
    phase75CheckerIndex === -1 ||
    phase75TestIndex < v16Index ||
    phase75CheckerIndex < phase75TestIndex
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 75 checker test and checker after the v1.6 release-boundary checker",
    );
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

function main(): void {
  const failures: string[] = [];

  verifyPlanRequirements(failures);
  verifySoakSourceAnchors(failures);
  verifySyntheticCoverageAnchors(failures);
  verifySupportProjectionAnchors(failures);
  verifyDocsAndParityRoots(failures);
  verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 75 soak runner and evidence ledger boundaries");
}

main();

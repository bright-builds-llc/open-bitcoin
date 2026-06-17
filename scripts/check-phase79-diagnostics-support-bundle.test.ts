#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

const CHECKER_PATH = path.join(import.meta.dir, "check-phase79-diagnostics-support-bundle.ts");
const PHASE_DIR = ".planning/phases/79-diagnostics-and-support-bundle-forensics";
const PLAN_FILES = [
  `${PHASE_DIR}/79-01-PLAN.md`,
  `${PHASE_DIR}/79-02-PLAN.md`,
  `${PHASE_DIR}/79-03-PLAN.md`,
  `${PHASE_DIR}/79-04-PLAN.md`,
] as const;
const DEFAULT_PLAN_TEXTS = PLAN_FILES.map(
  () => "---\nrequirements: [DIAG-01, DIAG-02, DIAG-03, DIAG-04]\n---\n",
);
const PHASE78_CHECKER_COMMAND = "bun run scripts/check-phase78-progress-guarantees.ts";
const PHASE79_TEST_COMMAND = "bun test scripts/check-phase79-diagnostics-support-bundle.test.ts";
const PHASE79_CHECKER_COMMAND = "bun run scripts/check-phase79-diagnostics-support-bundle.ts";
const DEFAULT_VERIFY_SCRIPT = [
  PHASE78_CHECKER_COMMAND,
  PHASE79_TEST_COMMAND,
  PHASE79_CHECKER_COMMAND,
].join("\n");
const SURFACE_ID = "phase79-diagnostics-support-bundle-forensics";
const REQUIREMENTS = "DIAG-01 DIAG-02 DIAG-03 DIAG-04";
const PARITY_TERMS = [
  "support_forensics",
  "forensic timeline",
  "checkpoint chain",
  "failure narrative",
  "likely cause",
  "evidence basis",
  "next action",
  "confidence",
  "redaction",
  "size bounds",
  "timeline ordering",
  "cross-surface consistency",
].join("\n");
const NON_CLAIMS = [
  "inbound serving",
  "relay",
  "production-funds wallet use",
  "migration apply mode",
  "packaging",
  "GUI",
  "hosted dashboards",
  "public-network default checks",
  "multi-day default gates",
  "automatic support-bundle upload",
  "production-node readiness",
].join("\n");
const FORBIDDEN_SUPPORT_OUTPUT = "rpcpassword=phase79-secret";

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
  "packages/open-bitcoin-cli/src/operator/support/forensics.rs": [
    "SupportForensicsEvidence",
    "ForensicTimelineEntry",
    "CheckpointChainEvidence",
    "ForensicNarrative",
    "ForensicVerdict",
    "ForensicConfidence",
    "ForensicSourceEvidence",
    "ForensicRedactionEvidence",
    "sha256-json-v1",
    "open-bitcoin-support-forensics-v1",
    "likely_cause",
    "evidence_basis",
    "next_action",
    "confidence",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "support_forensics",
    "resource_bound_evidence",
    "collect_soak_support_evidence",
    "redaction_summary",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/render.rs": [
    "push_support_forensics",
    "## Forensic Timeline",
    "## Checkpoint Chain",
    "## Failure Narrative",
    "csv_or_unavailable",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/resource_bounds.rs": [
    "ResourceBoundSupportEvidence",
    "maybe_projected_bundle_size_bytes",
    "ResourceBoundKind::SupportBundle",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/progress_guarantee.rs": [
    "progress_guarantee_summary",
    "stall_diagnosis_summary",
    "progress_credit_summary_text",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs": [
    "validated_active_chain",
    "peer_contribution",
    "no_progress_or_reorg_events",
    "stall_diagnosis",
    "live_smoke_final_status",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs": [
    "summarize_final_status",
    "maybe_no_progress_cause",
    "maybe_recovery_evidence_unavailable_reason",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs": "support_forensics",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/sync_section.rs": [
    "progress_credit_text",
    "stall_diagnosis_text",
    "recovery::recovery_evidence",
  ].join("\n"),
  "packages/open-bitcoin-rpc/src/method/node.rs": [
    "OpenBitcoinSyncStatusRequest",
    "OpenBitcoinSyncControlResponse",
    "RuntimeMetadata",
  ].join("\n"),
  "packages/open-bitcoin-node/src/sync/types/summary.rs": [
    "validated_active_chain_height",
    "progress_credit",
    "last_peer_contribution",
    "stall_diagnosis",
    "latest_stop_reason",
  ].join("\n"),
  "packages/open-bitcoin-node/src/status.rs": [
    "OpenBitcoinStatusSnapshot",
    "progress_credit",
    "resource_bounds",
    "recovery_evidence",
    "no_progress_diagnosis",
    "stall_diagnosis",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "phase79_support_forensics_projection_builds_timeline_chain_and_narrative",
    "phase79_support_forensics_projection_detects_sequence_gaps_and_truncation",
    "phase79_support_forensics_projection_keeps_unavailable_evidence_conservative",
    "phase79_support_forensics_json_includes_sidecar_contract",
    "phase79_support_forensics_json_excludes_sensitive_seed_material",
    "phase79_support_forensics_markdown_renders_timeline_chain_and_failure_narrative",
    "phase79_support_forensics_cross_surface_agreement_uses_shared_status_contract",
    "phase79_support_forensics_markdown_and_json_exclude_forbidden_sensitive_material",
    "rpcpassword=phase79-secret",
  ].join("\n"),
  "docs/operator/runtime-guide.md": [
    "### Phase 79 support bundle forensics",
    "support_forensics",
    "forensic timeline",
    "checkpoint chain",
    "failure narrative",
    "likely_cause",
    "evidence_basis",
    "next_action",
    "confidence",
    "soak_stable",
    "blocker_diagnosed",
    "inconclusive",
    "collection_failed",
    "not authenticity",
    "support bundle existence, elapsed time, peer reachability, daemon startup, raw logs, or stale reports do not prove soak stability",
    "public-network-free",
    "service-manager-free",
  ].join("\n"),
  "docs/architecture/status-snapshot.md": [
    "## Phase 79 shared diagnostic contract and support-forensics sidecar",
    "OpenBitcoinStatusSnapshot",
    "support_forensics",
    "resource_bound_evidence.maybe_projected_bundle_size_bytes",
    "checkpoint-chain validation",
  ].join("\n"),
  "docs/architecture/operator-observability.md": [
    "## Phase 79 support forensics projection",
    "CLI status",
    "dashboard status",
    "RPC status",
    "metrics",
    "structured logs",
    "live-smoke summaries",
    "soak reports",
    "bounded labels and counts, not high-cardinality forensic objects",
  ].join("\n"),
  "docs/parity/index.json": [SURFACE_ID, REQUIREMENTS, PARITY_TERMS, NON_CLAIMS].join("\n"),
  "docs/parity/checklist.md": [SURFACE_ID, REQUIREMENTS, PARITY_TERMS, NON_CLAIMS].join(
    "\n",
  ),
  "docs/parity/README.md": [SURFACE_ID, REQUIREMENTS, PARITY_TERMS, NON_CLAIMS].join("\n"),
  "docs/parity/catalog/p2p.md": [SURFACE_ID, REQUIREMENTS, PARITY_TERMS, NON_CLAIMS].join(
    "\n",
  ),
  "docs/parity/catalog/chainstate.md": [
    SURFACE_ID,
    REQUIREMENTS,
    PARITY_TERMS,
    NON_CLAIMS,
  ].join("\n"),
  "docs/parity/catalog/operator-runtime-release-hardening.md": [
    SURFACE_ID,
    REQUIREMENTS,
    PARITY_TERMS,
    NON_CLAIMS,
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

test("passes_when_phase79_fixture_contains_every_diagnostic_anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
});

test("fails_when_support_forensics_source_anchors_are_missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "packages/open-bitcoin-cli/src/operator/support/forensics.rs",
      needle: "SupportForensicsEvidence",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails_when_phase79_docs_or_parity_roots_are_missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/parity/catalog/chainstate.md",
      needle: SURFACE_ID,
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails_when_verify_order_or_default_boundaries_drift", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      PHASE79_TEST_COMMAND,
      PHASE79_CHECKER_COMMAND,
      PHASE78_CHECKER_COMMAND,
      "bun run scripts/run-live-mainnet-smoke.ts",
    ].join("\n"),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

test("fails_when_sensitive_strings_escape_redaction_fixtures", async () => {
  // Arrange
  const root = await createFixture({
    maybeAppend: {
      file: "packages/open-bitcoin-cli/src/operator/support/render.rs",
      text: `\n${FORBIDDEN_SUPPORT_OUTPUT}\n`,
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase79-checker-"));
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
      OPEN_BITCOIN_PHASE79_REPO_ROOT: root,
    },
    stdout: "pipe",
    stderr: "pipe",
  });

  return {
    exitCode: child.exitCode,
  };
}

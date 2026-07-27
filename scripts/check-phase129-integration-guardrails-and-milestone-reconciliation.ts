#!/usr/bin/env bun

import { existsSync } from "node:fs";
import path from "node:path";

import { checkPhase127AuthoritativeNetworkStateUnification } from "./check-phase127-authoritative-network-state-unification";
import { checkPhase128ProductionCompactAnnouncementTransport } from "./check-phase128-production-compact-announcement-transport";
import { readSourceCorpus } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE128_CHECK =
  "bun run scripts/check-phase128-production-compact-announcement-transport.ts";
const PHASE129_TEST =
  "bun test scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts";
const PHASE129_CHECK =
  "bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";

export const PHASE129_TARGET_FILES = [
  "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
  "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs",
  "packages/open-bitcoin-cli/tests/operator_flows.rs",
  "packages/open-bitcoin-cli/tests/operator_binary.rs",
  "scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts",
  "scripts/verify.sh",
] as const;

const FLOW_ANCHORS = [
  {
    file: "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
    anchor:
      "phase127_production_composition_shares_sync_serving_and_operator_authority",
    failure:
      "P129 FLOW-01: durable validated block to inbound serving production composition anchor is missing",
  },
  {
    file: "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
    anchor:
      "phase127_production_composition_shares_sync_serving_and_operator_authority",
    failure:
      "P129 FLOW-04: authoritative sync runtime to RPC/CLI/dashboard/support production composition anchor is missing",
  },
  {
    file: "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
    anchor: "production_announcement_transport_cases_fanout_uses_live_peer_facts",
    failure:
      "P129 FLOW-02: handshake to bilateral negotiation to live header-aware announcement anchor is missing",
  },
  {
    file: "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
    anchor:
      "production_announcement_transport_cases_partial_failure_credits_only_prefix_and_redacts",
    failure:
      "P129 FLOW-03: high-bandwidth decision to wire emission to post-write evidence anchor is missing",
  },
] as const;

const FLOW03_UNIT_ANCHOR_FILE =
  "packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs";
const FLOW03_UNIT_ANCHORS = [
  "compact_success_receipt_records_achieved_effect_once",
  "failed_or_unsent_emission_receives_no_achieved_effect_credit",
] as const;

const FLOW04_OPERATOR_SURFACE_FILES = [
  "packages/open-bitcoin-cli/tests/operator_flows.rs",
  "packages/open-bitcoin-cli/tests/operator_binary.rs",
] as const;

export function checkPhase129IntegrationGuardrailsAndMilestoneReconciliation(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE129_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  failures.push(...checkPhase127AuthoritativeNetworkStateUnification(repoRoot));
  failures.push(
    ...checkPhase128ProductionCompactAnnouncementTransport(repoRoot),
  );
  checkNamedFlows(repoRoot, failures);
  checkVerifierWiring(repoRoot, failures);
  checkDeterministicScope(repoRoot, failures);
  return failures;
}

function checkNamedFlows(repoRoot: string, failures: string[]): void {
  for (const { file, anchor, failure } of FLOW_ANCHORS) {
    if (!readTarget(repoRoot, file).includes(anchor)) {
      failures.push(failure);
    }
  }

  const unitAnchorText = readTarget(repoRoot, FLOW03_UNIT_ANCHOR_FILE);
  if (!FLOW03_UNIT_ANCHORS.every((anchor) => unitAnchorText.includes(anchor))) {
    failures.push(
      "P129 FLOW-03: post-write-only achieved-effect unit anchors are missing",
    );
  }

  const operatorSurfacePresent = FLOW04_OPERATOR_SURFACE_FILES.every(
    (file) => readTarget(repoRoot, file).trim().length > 0,
  );
  if (!operatorSurfacePresent) {
    failures.push("P129 FLOW-04: CLI operator surface test files are missing");
  }
}

function checkVerifierWiring(repoRoot: string, failures: string[]): void {
  const verify = readTarget(repoRoot, "scripts/verify.sh");
  const heredoc = visibleCommandOrder(verify);
  const requiredVisible = [
    PHASE128_CHECK,
    PHASE129_TEST,
    PHASE129_CHECK,
    PHASE117_TEST,
  ];
  const requiredSteps = [
    `run_step "check Phase 128 production compact announcement transport" ${PHASE128_CHECK}`,
    `run_step "test Phase 129 integration guardrails and milestone reconciliation checker" ${PHASE129_TEST}`,
    `run_step "check Phase 129 integration guardrails and milestone reconciliation" ${PHASE129_CHECK}`,
    `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
  ];
  if (!orderedLines(heredoc, requiredVisible)) {
    failures.push(
      "P129 verifier heredoc: Phase 129 pair must run between Phase 128 and the Phase 117 gate",
    );
  }
  if (!orderedLines(verify, requiredSteps)) {
    failures.push(
      "P129 verifier run_step: Phase 129 pair must run between Phase 128 and the Phase 117 gate",
    );
  }

  requireFinalPhaseChecker(
    heredoc,
    "P129 final gate heredoc order",
    failures,
  );
  requireFinalPhaseChecker(
    runStepLines(verify),
    "P129 final gate run_step order",
    failures,
  );
}

function checkDeterministicScope(repoRoot: string, failures: string[]): void {
  const checker = readTarget(
    repoRoot,
    "scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts",
  );
  const forbiddenTokens = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbiddenTokens.some((token) => checker.includes(token))) {
    failures.push(
      "P129 deterministic scope: checker must remain local and public-network-free",
    );
  }
}

function requireFinalPhaseChecker(
  text: string,
  label: string,
  failures: string[],
): void {
  const phaseCommands = text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => /\bbun (?:test|run) scripts\/check-phase\d+/.test(line));
  if (!phaseCommands.at(-1)?.includes(PHASE117_CHECK)) {
    failures.push(`${label} must end with ${PHASE117_CHECK}`);
  }
}

function runStepLines(text: string): string {
  return text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.startsWith("run_step "))
    .join("\n");
}

function readTarget(repoRoot: string, relativePath: string): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) return "";
  return readSourceCorpus(repoRoot, relativePath);
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function orderedLines(text: string, required: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const line of required) {
    const index = lines.indexOf(line, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

if (import.meta.main) {
  const failures =
    checkPhase129IntegrationGuardrailsAndMilestoneReconciliation();
  if (failures.length > 0) {
    console.error(
      "Phase 129 integration guardrails and milestone reconciliation check failed:",
    );
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log(
    "Phase 129 integration guardrails and milestone reconciliation validated.",
  );
}

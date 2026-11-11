#!/usr/bin/env bun

import { existsSync } from "node:fs";
import path from "node:path";

import { checkPhase127AuthoritativeNetworkStateUnification } from "./check-phase127-authoritative-network-state-unification";
import { checkPhase128ProductionCompactAnnouncementTransport } from "./check-phase128-production-compact-announcement-transport";
import { readSourceCorpus } from "./source-corpus";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

export const PHASE129_TARGET_FILES = [
  "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
  "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs",
  "packages/open-bitcoin-cli/tests/operator_flows.rs",
  "packages/open-bitcoin-cli/tests/operator_binary.rs",
  "scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts",
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

function readTarget(repoRoot: string, relativePath: string): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) return "";
  return readSourceCorpus(repoRoot, relativePath);
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

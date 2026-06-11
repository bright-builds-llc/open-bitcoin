#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR = ".planning/phases/69-tip-tracking-and-stay-current-operation";
const REQUIRED_PHASE_ARTIFACTS = [
  `${PHASE_DIR}/69-CONTEXT.md`,
  `${PHASE_DIR}/69-DISCUSSION-LOG.md`,
  `${PHASE_DIR}/69-RESEARCH.md`,
  `${PHASE_DIR}/69-01-PLAN.md`,
  `${PHASE_DIR}/69-02-PLAN.md`,
  `${PHASE_DIR}/69-03-PLAN.md`,
  `${PHASE_DIR}/69-04-PLAN.md`,
  `${PHASE_DIR}/69-05-PLAN.md`,
] as const;
const STATUS_CONTRACT_NEEDLES = [
  "BestKnownTipStatus",
  "StayCurrentStatus",
  "stay_current_next_action",
  "current_at_best_known_tip",
  "stale_tip",
  "best_known_tip_unavailable",
] as const;
const RUNTIME_PROJECTION_NEEDLES = [
  "sync.best_known_tip",
  "sync.stay_current",
  "sync.stay_current_next_action",
] as const;
const TEST_NEEDLES = [
  "phase69_post_catch_up_new_headers_connect_and_report_stay_current_progress",
  "phase69_fresh_idle_cycle_reports_current_at_best_known_tip",
  "phase69_peer_agreement_classifies_agrees_behind_disagrees_and_no_evidence",
  "phase69_stale_tip_is_distinct_from_no_progress",
  "phase69_tip_evidence_survives_runtime_reopen",
] as const;
const DOC_NEEDLES = [
  "sync.best_known_tip",
  "sync.stay_current_next_action",
  "current_at_best_known_tip",
  "stale_tip",
] as const;
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "systemctl --user",
  "launchctl",
  "openbitcoinsync=mainnet-ibd",
] as const;
const BROAD_SCOPE_TERMS = [
  "production full node",
  "inbound serving",
  "transaction relay",
  "production wallet",
  "migration apply",
  "packaging",
  "GUI",
  "hosted dashboard",
  "production-ready",
  "production-node",
] as const;
const SHIPPED_CLAIM_WORDS = [
  "ships",
  "shipped",
  "delivers",
  "delivered",
  "provides",
  "provided",
  "implements",
  "implemented",
  "enables",
  "enabled",
] as const;
const NEGATED_SCOPE_WORDS = [
  "not ",
  "no ",
  "without ",
  "avoid ",
  "contains `",
  "reject ",
  "out of scope",
  "outside",
  "deferred",
  "must not",
  "should not",
  "do not",
  "does not",
  "is not",
  "are not",
] as const;

function readText(relativePath: string): string {
  return readFileSync(path.join(REPO_ROOT, relativePath), "utf8");
}

function requireExists(relativePath: string): void {
  if (!existsSync(path.join(REPO_ROOT, relativePath))) {
    throw new Error(`missing required Phase 69 artifact: ${relativePath}`);
  }
}

function requireContains(text: string, needle: string, label: string): void {
  if (!text.includes(needle)) {
    throw new Error(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string): void {
  if (text.includes(needle)) {
    throw new Error(`${label} must not contain default verification command: ${needle}`);
  }
}

function requireAllContains(
  text: string,
  needles: readonly string[],
  label: string,
): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function includesAny(text: string, needles: readonly string[]): boolean {
  return needles.some((needle) => text.includes(needle));
}

function rejectShippedScopeClaims(text: string, label: string): void {
  for (const [index, line] of text.split(/\r?\n/).entries()) {
    const lowerLine = line.toLowerCase();
    const maybeScopeTerm = BROAD_SCOPE_TERMS.find((term) =>
      lowerLine.includes(term.toLowerCase()),
    );
    if (maybeScopeTerm === undefined) {
      continue;
    }
    if (includesAny(lowerLine, NEGATED_SCOPE_WORDS)) {
      continue;
    }
    if (maybeScopeTerm !== "production-ready" && !includesAny(lowerLine, SHIPPED_CLAIM_WORDS)) {
      continue;
    }
    throw new Error(
      `${label}:${index + 1} appears to make a shipped-scope Phase 69 claim for ${maybeScopeTerm}: ${line.trim()}`,
    );
  }
}

function verifyPhaseArtifacts(): void {
  for (const artifact of REQUIRED_PHASE_ARTIFACTS) {
    requireExists(artifact);
  }

  const research = readText(`${PHASE_DIR}/69-RESEARCH.md`);
  requireContains(
    research,
    "## Open Questions (RESOLVED)",
    `${PHASE_DIR}/69-RESEARCH.md`,
  );
}

function verifyStatusContract(): void {
  const status = readText("packages/open-bitcoin-node/src/status.rs");
  requireAllContains(status, STATUS_CONTRACT_NEEDLES, "packages/open-bitcoin-node/src/status.rs");
}

function verifyRuntimeProjection(): void {
  const runtimeState = readText("packages/open-bitcoin-node/src/sync/runtime_state.rs");
  requireAllContains(
    runtimeState,
    RUNTIME_PROJECTION_NEEDLES,
    "packages/open-bitcoin-node/src/sync/runtime_state.rs",
  );

  const tip = readText("packages/open-bitcoin-node/src/sync/tip.rs");
  requireContains(tip, "classify_stay_current", "packages/open-bitcoin-node/src/sync/tip.rs");
}

function verifyTests(): void {
  const tests = readText("packages/open-bitcoin-node/src/sync/tests.rs");
  requireAllContains(tests, TEST_NEEDLES, "packages/open-bitcoin-node/src/sync/tests.rs");
}

function verifyDocs(): void {
  for (const relativePath of [
    "docs/operator/runtime-guide.md",
    "docs/architecture/status-snapshot.md",
  ]) {
    const text = readText(relativePath);
    requireAllContains(text, DOC_NEEDLES, relativePath);
    rejectShippedScopeClaims(text, relativePath);
  }

  for (const relativePath of REQUIRED_PHASE_ARTIFACTS) {
    rejectShippedScopeClaims(readText(relativePath), relativePath);
  }
}

function verifyVerifyScript(): void {
  const verifyScript = readText("scripts/verify.sh");
  requireContains(
    verifyScript,
    "bun run scripts/check-phase69-tip-stay-current.ts",
    "scripts/verify.sh",
  );
  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh");
  }
}

function main(): void {
  verifyPhaseArtifacts();
  verifyStatusContract();
  verifyRuntimeProjection();
  verifyTests();
  verifyDocs();
  verifyVerifyScript();

  console.log("validated Phase 69 tip stay-current evidence");
}

main();

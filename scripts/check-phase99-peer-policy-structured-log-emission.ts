#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE98_TEST_COMMAND =
  "bun test scripts/check-phase98-traceability-reconciliation.test.ts";
const PHASE98_CHECKER_COMMAND =
  "bun run scripts/check-phase98-traceability-reconciliation.ts";
const PHASE99_TEST_COMMAND =
  "bun test scripts/check-phase99-peer-policy-structured-log-emission.test.ts";
const PHASE99_CHECKER_COMMAND =
  "bun run scripts/check-phase99-peer-policy-structured-log-emission.ts";
const TARGET_FILES = [
  "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  ".planning/milestones/v1.9-ROADMAP.md",
  ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
  ".planning/phases/99-peer-policy-structured-log-emission/99-VERIFICATION.md",
  "scripts/verify.sh",
] as const;
const RAW_PEER_POLICY_MARKER_NEEDLES = [
  '["peer", "_id"]',
  '["raw", "_end", "point"]',
  '["payload", "_bytes"]',
  '["permission", "_string"]',
  '["cred", "ential"]',
  '["sec", "ret"]',
  "cookie",
] as const;
const FORBIDDEN_PHASE99_VERIFY_GATES = [
  "public-network",
  "dnsseed",
  "seednode",
  "service-manager",
  "systemd",
  "launchd",
  "multi-day",
  "soak",
  "transaction relay",
  "mempool propagation",
  "compact block",
  "production readiness",
  "production full-node readiness",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type CheckPhase99Options = { rootDir?: string };

export function checkPhase99PeerPolicyStructuredLogEmission(
  options: CheckPhase99Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyProductionEmission(texts.get("packages/open-bitcoin-rpc/src/context/peer_policy.rs") ?? "", failures);
  verifyRustBehaviorTest(texts.get("packages/open-bitcoin-rpc/src/context/tests.rs") ?? "", failures);
  verifySanitizer(texts, failures);
  verifyPhaseCompletion(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoForbiddenVerificationGates(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P99 missing required corpus file: ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function verifyProductionEmission(context: string, failures: string[]): void {
  for (const needle of [
    "pub fn record_inbound_peer_policy_event(",
    "pub fn record_latest_inbound_peer_policy_event_at(",
    "pub fn record_peer_policy_ban(",
    "pub fn record_peer_policy_discouragement(",
    "pub fn record_peer_policy_unban(",
    "pub fn record_peer_policy_misbehavior(",
    "record_inbound_peer_policy_event(peer_policy_event_from_ban_decision(&decision))",
    "record_inbound_peer_policy_event(peer_policy_event_from_discouragement_decision",
    "record_inbound_peer_policy_event(peer_policy_event_from_unban_decision(&decision))",
    "record_inbound_peer_policy_event(peer_policy_event_from_misbehavior_decision",
    "discouragement_active",
    "discouragement_expired",
  ]) {
    requireContains(context, needle, "P99 production peer-policy log emission", failures);
  }

  for (const method of [
    "record_latest_inbound_peer_policy_event_at",
    "record_peer_policy_ban",
    "record_peer_policy_discouragement",
    "record_peer_policy_unban",
    "record_peer_policy_misbehavior",
  ]) {
    const cfgPattern = new RegExp(`#\\[cfg\\(test\\)\\][\\s\\S]{0,160}${method}`);
    if (cfgPattern.test(context)) {
      failures.push(`P99 production peer-policy log emission leaves ${method} test-only`);
    }
  }
}

function verifyRustBehaviorTest(contextTests: string, failures: string[]): void {
  for (const needle of [
    "record_peer_policy_runtime_decisions_append_sanitized_logs_automatically",
    "record_peer_policy_ban(",
    "record_peer_policy_discouragement(",
    "record_peer_policy_misbehavior(",
    "record_peer_policy_unban(",
    "assert_eq!(records.len(), 4)",
    "outcome=discouragement_active",
    "raw peer-policy data leaked",
    "peer-raw-42",
    "credential=cookie",
  ]) {
    requireContains(contextTests, needle, "P99 Rust automatic emission test", failures);
  }
}

function verifySanitizer(texts: Map<TargetFile, string>, failures: string[]): void {
  const logging = texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "";
  const context = texts.get("packages/open-bitcoin-rpc/src/context/peer_policy.rs") ?? "";
  for (const needle of [
    "INBOUND_PEER_POLICY_LOG_SOURCE",
    "inbound_peer_policy_log_record",
    "redacted_peer_policy_field",
    "sanitized_peer_policy_log_field",
  ]) {
    requireContains(logging, needle, "P99 peer-policy sanitizer", failures);
  }
  for (const markerNeedle of RAW_PEER_POLICY_MARKER_NEEDLES) {
    requireContains(logging, markerNeedle, "P99 sanitizer raw marker coverage", failures);
  }
  requireContains(
    context,
    "inbound_peer_policy_log_record(&event, timestamp_unix_seconds)",
    "P99 context sanitizer reuse",
    failures,
  );
}

function verifyPhaseCompletion(texts: Map<TargetFile, string>, failures: string[]): void {
  const roadmap = texts.get(".planning/milestones/v1.9-ROADMAP.md") ?? "";
  const audit = texts.get(".planning/milestones/v1.9-MILESTONE-AUDIT.md") ?? "";
  const verification =
    texts.get(".planning/phases/99-peer-policy-structured-log-emission/99-VERIFICATION.md") ?? "";
  for (const needle of [
    "| 99 | Peer Policy Structured Log Emission | 1/1 | Complete |",
    "Phase 99 is complete and verified",
    "**Requirements:** none (optional cleanup",
  ]) {
    requireContains(roadmap, needle, "P99 roadmap completion", failures);
  }
  for (const needle of [
    "TD-01-peer-policy-log-emission-edge: closed",
    "automatic production structured-log emission for `inbound_peer_policy` is now verified",
  ]) {
    requireContains(audit, needle, "P99 audit closure", failures);
  }
  for (const needle of [
    "status: passed",
    "score: \"5/5 must-haves verified\"",
    "Full repo-native verification: passed",
  ]) {
    requireContains(verification, needle, "P99 verification report", failures);
  }
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  for (const command of [
    PHASE98_TEST_COMMAND,
    PHASE98_CHECKER_COMMAND,
    PHASE99_TEST_COMMAND,
    PHASE99_CHECKER_COMMAND,
    "Phase 98 is followed by Phase 99",
  ]) {
    requireContains(text, command, "P99 verifier wiring", failures);
  }
  requireOrdered(text, PHASE98_CHECKER_COMMAND, PHASE99_TEST_COMMAND, "P99 visible verifier order", failures);
  requireOrdered(text, PHASE99_TEST_COMMAND, PHASE99_CHECKER_COMMAND, "P99 visible verifier order", failures);
  requireOrdered(
    text,
    'run_step "test Phase 98 traceability reconciliation checker"',
    'run_step "test Phase 99 peer-policy structured log emission checker"',
    "P99 executable verifier order",
    failures,
  );
  requireOrdered(
    text,
    'run_step "test Phase 99 peer-policy structured log emission checker"',
    'run_step "check Phase 99 peer-policy structured log emission"',
    "P99 executable verifier order",
    failures,
  );
}

function verifyNoForbiddenVerificationGates(text: string, failures: string[]): void {
  for (const line of text.split(/\r?\n/)) {
    const lower = line.toLowerCase();
    if (!lower.includes("phase 99") && !lower.includes("check-phase99-peer-policy")) {
      continue;
    }
    for (const forbidden of FORBIDDEN_PHASE99_VERIFY_GATES) {
      if (lower.includes(forbidden)) {
        failures.push(`P99 default verifier introduces forbidden gate '${forbidden}': ${line.trim()}`);
      }
    }
  }
}

function requireContains(text: string, needle: string, label: string, failures: string[]): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireOrdered(
  text: string,
  before: string,
  after: string,
  label: string,
  failures: string[],
): void {
  const beforeIndex = text.indexOf(before);
  const afterIndex = text.indexOf(after);
  if (beforeIndex === -1 || afterIndex === -1 || beforeIndex >= afterIndex) {
    failures.push(`${label} must order '${before}' before '${after}'`);
  }
}

if (import.meta.main) {
  const failures = checkPhase99PeerPolicyStructuredLogEmission();
  if (failures.length > 0) {
    console.error("Phase 99 peer-policy structured log emission check failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log("Phase 99 peer-policy structured log emission checker passed.");
}

import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase99PeerPolicyStructuredLogEmission } from "./check-phase99-peer-policy-structured-log-emission";

const TARGET_FILES = [
  "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  ".planning/milestones/v1.9-ROADMAP.md",
  ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
  ".planning/phases/99-peer-policy-structured-log-emission/99-VERIFICATION.md",
  "scripts/verify.sh",
] as const;
const PHASE98_TEST_COMMAND =
  "bun test scripts/check-phase98-traceability-reconciliation.test.ts";
const PHASE98_CHECKER_COMMAND =
  "bun run scripts/check-phase98-traceability-reconciliation.ts";
const PHASE99_TEST_COMMAND =
  "bun test scripts/check-phase99-peer-policy-structured-log-emission.test.ts";
const PHASE99_CHECKER_COMMAND =
  "bun run scripts/check-phase99-peer-policy-structured-log-emission.ts";

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with complete Phase 99 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when peer-policy mutation logging is test-only", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
        "pub fn record_peer_policy_ban(",
        "#[cfg(test)]\n    pub fn record_peer_policy_ban(",
      );
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("test-only");
});

test("fails missing specific decision emission helper", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
        "record_inbound_peer_policy_event(peer_policy_event_from_unban_decision(&decision))",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P99 production peer-policy log emission");
});

test("fails missing sanitizer raw marker coverage", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/logging.rs",
        'lower.contains("cookie")',
        "",
      );
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P99 sanitizer raw marker coverage");
});

test("fails missing automatic emission behavior test", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-rpc/src/context/tests.rs",
        "record_peer_policy_runtime_decisions_append_sanitized_logs_automatically",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P99 Rust automatic emission test");
});

test("fails missing Phase 99 verifier wiring", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "scripts/verify.sh", PHASE99_TEST_COMMAND, "");
      replaceInFile(files, "scripts/verify.sh", PHASE99_CHECKER_COMMAND, "");
      replaceInFile(
        files,
        "scripts/verify.sh",
        'run_step "test Phase 99 peer-policy structured log emission checker"',
        "",
      );
      replaceInFile(
        files,
        "scripts/verify.sh",
        'run_step "check Phase 99 peer-policy structured log emission"',
        "",
      );
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P99 executable verifier order");
});

test("fails Phase 99 verifier wiring before Phase 98", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "scripts/verify.sh",
        [
          "#!/usr/bin/env bash",
          "set -euo pipefail",
          "# Phase 98 is followed by Phase 99.",
          ": <<'VERIFY_COMMAND_ORDER'",
          PHASE99_TEST_COMMAND,
          PHASE99_CHECKER_COMMAND,
          PHASE98_TEST_COMMAND,
          PHASE98_CHECKER_COMMAND,
          "VERIFY_COMMAND_ORDER",
          `run_step "test Phase 99 peer-policy structured log emission checker" ${PHASE99_TEST_COMMAND}`,
          `run_step "check Phase 99 peer-policy structured log emission" ${PHASE99_CHECKER_COMMAND}`,
          `run_step "test Phase 98 traceability reconciliation checker" ${PHASE98_TEST_COMMAND}`,
          `run_step "check Phase 98 traceability reconciliation" ${PHASE98_CHECKER_COMMAND}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P99 visible verifier order");
});

test("fails forbidden Phase 99 verifier gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(files, "scripts/verify.sh", 'run_step "Phase 99 public-network check" true');
    },
  });

  // Act
  const failures = checkPhase99PeerPolicyStructuredLogEmission({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("forbidden gate");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase99-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>([
    ["packages/open-bitcoin-rpc/src/context/peer_policy.rs", contextPeerPolicyFixture()],
    ["packages/open-bitcoin-rpc/src/context/tests.rs", contextTestsFixture()],
    ["packages/open-bitcoin-node/src/logging.rs", loggingFixture()],
    [".planning/milestones/v1.9-ROADMAP.md", roadmapFixture()],
    [".planning/milestones/v1.9-MILESTONE-AUDIT.md", auditFixture()],
    [
      ".planning/phases/99-peer-policy-structured-log-emission/99-VERIFICATION.md",
      verificationFixture(),
    ],
    ["scripts/verify.sh", verifyFixture()],
  ]);
  options.maybeMutateFiles?.(files);
  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}

function contextPeerPolicyFixture(): string {
  return [
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
    "inbound_peer_policy_log_record(&event, timestamp_unix_seconds)",
    "discouragement_active",
    "discouragement_expired",
  ].join("\n");
}

function contextTestsFixture(): string {
  return [
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
  ].join("\n");
}

function loggingFixture(): string {
  return [
    "INBOUND_PEER_POLICY_LOG_SOURCE",
    "inbound_peer_policy_log_record",
    "redacted_peer_policy_field",
    "sanitized_peer_policy_log_field",
    'lower.contains(&["peer", "_id"].concat())',
    'lower.contains(&["raw", "_end", "point"].concat())',
    'lower.contains(&["payload", "_bytes"].concat())',
    'lower.contains(&["permission", "_string"].concat())',
    'lower.contains(&["cred", "ential"].concat())',
    'lower.contains(&["sec", "ret"].concat())',
    'lower.contains("cookie")',
  ].join("\n");
}

function roadmapFixture(): string {
  return [
    "| 99 | Peer Policy Structured Log Emission | 1/1 | Complete | 2026-06-29 |",
    "Phase 99 is complete and verified",
    "**Requirements:** none (optional cleanup; evidence hardening)",
  ].join("\n");
}

function auditFixture(): string {
  return [
    "TD-01-peer-policy-log-emission-edge: closed",
    "automatic production structured-log emission for `inbound_peer_policy` is now verified",
  ].join("\n");
}

function verificationFixture(): string {
  return [
    "status: passed",
    'score: "5/5 must-haves verified"',
    "Full repo-native verification: passed",
  ].join("\n");
}

function verifyFixture(): string {
  return [
    "# Phase 98 is followed by Phase 99.",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE98_TEST_COMMAND,
    PHASE98_CHECKER_COMMAND,
    PHASE99_TEST_COMMAND,
    PHASE99_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 98 traceability reconciliation checker" ${PHASE98_TEST_COMMAND}`,
    `run_step "check Phase 98 traceability reconciliation" ${PHASE98_CHECKER_COMMAND}`,
    `run_step "test Phase 99 peer-policy structured log emission checker" ${PHASE99_TEST_COMMAND}`,
    `run_step "check Phase 99 peer-policy structured log emission" ${PHASE99_CHECKER_COMMAND}`,
  ].join("\n");
}

function replaceInFile(files: Map<TargetFile, string>, file: TargetFile, from: string, to: string): void {
  const current = files.get(file);
  if (current === undefined) {
    throw new Error(`missing fixture file: ${file}`);
  }
  if (!current.includes(from)) {
    throw new Error(`fixture file ${file} did not include ${from}`);
  }
  files.set(file, current.replace(from, to));
}

function appendToFile(files: Map<TargetFile, string>, file: TargetFile, text: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${text}\n`);
}

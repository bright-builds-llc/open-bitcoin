import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase104RelayServingFanout } from "./check-phase104-relay-serving-fanout";

const TARGET_FILES = [
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/serving_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "scripts/check-phase104-relay-serving-fanout.ts",
  "scripts/check-phase104-relay-serving-fanout.test.ts",
  "scripts/verify.sh",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-01-SUMMARY.md",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-02-SUMMARY.md",
  ".planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-03-SUMMARY.md",
] as const;
const REQUIRED_REQUIREMENTS = ["REL-01", "REL-02", "REL-03", "REL-04"] as const;

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

test("passes_when_phase104_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase104RelayServingFanout(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase104_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase104RelayServingFanout(root).join("\n"));

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_serving_fanout_or_local_symbol_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "TxServeOutcomeLabel");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "ManagedRelayFanoutState");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "LocalRelaySubmissionEvidence");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase104RelayServingFanout(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("required Phase 104 symbol");
  }
});

test("fails_when_required_behavior_test_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromAllFiles(files, "local_submission_records_queued_internal_relay_evidence");
    },
  });

  // Act
  const failures = checkPhase104RelayServingFanout(root).join("\n");

  // Assert
  expect(failures).toContain("required Phase 104 behavior test");
});

test("fails_when_source_breadcrumb_group_or_knots_anchor_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "docs/parity/source-breadcrumbs.json", "node-network-adapter");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "packages/bitcoin-knots/src/rpc/rawtransaction.cpp");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase104RelayServingFanout(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("source breadcrumb group");
  expect(failureMessages[1]).toContain("Knots anchor");
});

test("fails_when_default_verifier_wiring_is_missing_or_public_network_scoped", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "scripts/verify.sh", "bun run scripts/check-phase104-relay-serving-fanout.ts");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/parity/catalog/p2p.md", "Phase 104 adds public-network relay CI.");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase104RelayServingFanout(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("verifier-scope");
  expect(failureMessages[1]).toContain("forbidden positive Phase 104 claim");
});

test("fails_when_docs_claim_deferred_rebroadcast_scope", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/parity/catalog/p2p.md",
        "Phase 104 supports periodic rebroadcast scheduling.",
      );
    },
  });

  // Act
  const failures = checkPhase104RelayServingFanout(root).join("\n");

  // Assert
  expect(failures).toContain("forbidden positive Phase 104 claim");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase104-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const filePath of TARGET_FILES) {
    files.set(filePath, readFileSync(filePath, "utf8"));
  }
  options.maybeMutateFiles?.(files);
  for (const [filePath, content] of files.entries()) {
    const absolutePath = path.join(root, filePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, content);
  }

  return root;
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const filePath of TARGET_FILES) {
    removeFromFile(files, filePath, needle);
  }
}

function removeFromFile(files: Map<TargetFile, string>, filePath: TargetFile, needle: string): void {
  files.set(filePath, (files.get(filePath) ?? "").replaceAll(needle, ""));
}

function appendToFile(files: Map<TargetFile, string>, filePath: TargetFile, text: string): void {
  files.set(filePath, `${files.get(filePath) ?? ""}\n${text}\n`);
}

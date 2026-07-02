import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase105OperatorRelayEvidence } from "./check-phase105-operator-relay-evidence";

const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "README.md",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/tests.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/tests.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/status/tests.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase105-operator-relay-evidence.ts",
  "scripts/check-phase105-operator-relay-evidence.test.ts",
  "scripts/verify.sh",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-01-SUMMARY.md",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-02-SUMMARY.md",
  ".planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-03-SUMMARY.md",
] as const;
const REQUIRED_REQUIREMENTS = ["OBS-01", "OBS-02", "OBS-03", "OBS-04"] as const;

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

test("passes_when_phase105_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase105OperatorRelayEvidence(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase105_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase105OperatorRelayEvidence(root).join("\n"));

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_fixed_counter_or_shared_contract_symbol_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "rebroadcast_deferred_count");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "packages/open-bitcoin-node/src/logging.rs", "relay_mempool_log_record");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase105OperatorRelayEvidence(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("fixed relay counter");
  expect(failureMessages[1]).toContain("shared Phase 105 contract needle");
});

test("fails_when_redaction_coverage_or_runtime_command_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "packages/open-bitcoin-cli/src/operator/support/tests.rs", "dynamic_label");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "docs/operator/runtime-guide.md",
          "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
        );
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase105OperatorRelayEvidence(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("support redaction coverage");
  expect(failureMessages[1]).toContain("runtime guide command");
});

test("fails_when_source_breadcrumb_or_knots_anchor_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "docs/parity/source-breadcrumbs.json",
          "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "packages/bitcoin-knots/src/rpc/net.cpp");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase105OperatorRelayEvidence(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("source breadcrumb group");
  expect(failureMessages[1]).toContain("Knots anchor");
});

test("fails_when_default_verifier_wiring_is_missing_or_out_of_order", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "scripts/verify.sh", "bun run scripts/check-phase105-operator-relay-evidence.ts");
    },
  });

  // Act
  const failures = checkPhase105OperatorRelayEvidence(root).join("\n");

  // Assert
  expect(failures).toContain("verifier-scope");
});

test("fails_when_docs_claim_deferred_public_or_production_scope", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/parity/catalog/p2p.md", "Phase 105 supports compact block relay.");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/operator/runtime-guide.md", "Phase 105 proves production full-node readiness.");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase105OperatorRelayEvidence(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("forbidden positive Phase 105 claim");
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase105-"));
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

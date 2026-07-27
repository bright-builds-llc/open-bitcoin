import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase116OperatorBlockRelayEvidence } from "./check-phase116-operator-block-relay-evidence";
import { readSourceCorpus } from "./source-corpus";

const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/status/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/tests.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/tests.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-rpc/src/method/node.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/block_relay.rs",
  "packages/open-bitcoin-cli/src/operator/status/tests.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase116-operator-block-relay-evidence.ts",
  "scripts/check-phase116-operator-block-relay-evidence.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_REQUIREMENTS = ["OBS-01", "OBS-02", "OBS-03", "OBS-04", "OBS-05"] as const;

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

test("passes_when_phase116_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase116OperatorBlockRelayEvidence(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase116_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase116OperatorBlockRelayEvidence(root).join("\n"),
  );

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
        removeFromAllFiles(files, "compact_cleanup_count");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "block_relay_log_record");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase116OperatorBlockRelayEvidence(root).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("fixed block-relay evidence");
  expect(failureMessages[1]).toContain("shared Phase 116 contract needle");
});

test("fails_when_redaction_coverage_or_runtime_command_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "packages/open-bitcoin-cli/src/operator/support/tests.rs",
          "dynamic_label",
        );
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
  const failureMessages = roots.map((root) =>
    checkPhase116OperatorBlockRelayEvidence(root).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("support redaction coverage");
  expect(failureMessages[1]).toContain("runtime guide command");
});

test("fails_when_source_breadcrumb_or_verifier_wiring_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "docs/parity/source-breadcrumbs.json",
          "packages/open-bitcoin-cli/src/operator/support/render/block_relay.rs",
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "scripts/verify.sh",
          "bun run scripts/check-phase116-operator-block-relay-evidence.ts",
        );
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase116OperatorBlockRelayEvidence(root).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("source breadcrumb group");
  expect(failureMessages[1]).toContain("verifier-scope");
});

test("fails_when_docs_claim_deferred_public_or_production_scope", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(
          files,
          "docs/architecture/operator-observability.md",
          "Phase 116 supports public block serving by default.",
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(
          files,
          "docs/operator/runtime-guide.md",
          "Phase 116 proves production readiness.",
        );
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase116OperatorBlockRelayEvidence(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("forbidden positive Phase 116 claim");
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase116-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const filePath of TARGET_FILES) {
    files.set(filePath, readSourceCorpus(process.cwd(), filePath));
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

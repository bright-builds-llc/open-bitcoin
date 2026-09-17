import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase144OperatorFlushAvailabilityEvidence } from "./check-phase144-operator-flush-availability-evidence";
import { readSourceCorpus, readSourceRoot } from "./source-corpus";

const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/status.rs",
  "packages/open-bitcoin-node/src/status/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/logging/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs",
  "packages/open-bitcoin-node/src/network/chainstate_durability_evidence/tests.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
  "packages/open-bitcoin-rpc/src/method/node.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase144-operator-flush-availability-evidence.ts",
  "scripts/check-phase144-operator-flush-availability-evidence.test.ts",
  "scripts/verify.sh",
] as const;
const NEW_FIELD_FILES = [
  "packages/open-bitcoin-node/src/status/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/metrics/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/logging/chainstate_durability.rs",
  "packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/chainstate_durability.rs",
] as const;
const REQUIRED_REQUIREMENTS = ["CSOBS-01", "CSOBS-02"] as const;

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

test("phase144 checker passes against the live repo", () => {
  // Arrange
  const repoRoot = process.cwd();

  // Act
  const failures = checkPhase144OperatorFlushAvailabilityEvidence(repoRoot);

  // Assert
  expect(failures).toEqual([]);
});

test("phase144 checker fails when CSOBS-01 or CSOBS-02 is missing from REQUIRED_REQUIREMENTS", () => {
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
    checkPhase144OperatorFlushAvailabilityEvidence(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("phase144 checker fails when getblock or pruned appears on the new-field corpus", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "packages/open-bitcoin-node/src/status/chainstate_durability.rs",
        "getblock\npruned\n",
      );
    },
  });

  // Act
  const message = checkPhase144OperatorFlushAvailabilityEvidence(root).join("\n");

  // Assert
  expect(message).toContain("getblock");
  expect(message).toContain("pruned");
});

test("verify.sh lists both bun test and bun run for the Phase 144 checker", () => {
  // Arrange
  const verifyText = readFileSync(path.join(process.cwd(), "scripts/verify.sh"), "utf8");
  const visibleMarker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const visibleStart = verifyText.indexOf(visibleMarker);
  const visibleBody = verifyText.slice(
    visibleStart + visibleMarker.length,
    verifyText.indexOf("\nVERIFY_COMMAND_ORDER", visibleStart + visibleMarker.length),
  );
  const bunNeedles = [
    "bun test scripts/check-phase116-operator-block-relay-evidence.test.ts",
    "bun run scripts/check-phase116-operator-block-relay-evidence.ts",
    "bun test scripts/check-phase144-operator-flush-availability-evidence.test.ts",
    "bun run scripts/check-phase144-operator-flush-availability-evidence.ts",
  ];
  const runStepNeedles = [
    'run_step "test Phase 116 operator block-relay evidence checker"',
    'run_step "check Phase 116 operator block-relay evidence"',
    'run_step "test Phase 144 operator flush and availability evidence checker"',
    'run_step "check Phase 144 operator flush and availability evidence"',
  ];

  // Act
  const visibleOrder = orderedIndexes(visibleBody, bunNeedles);
  const runStepOrder = orderedIndexes(verifyText, runStepNeedles);

  // Assert
  expect(visibleOrder).toBe(true);
  expect(runStepOrder).toBe(true);
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase144-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const filePath of TARGET_FILES) {
    files.set(filePath, readSourceCorpus(process.cwd(), filePath));
  }
  for (const filePath of NEW_FIELD_FILES) {
    files.set(filePath as TargetFile, readSourceRoot(process.cwd(), filePath));
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

function orderedIndexes(text: string, needles: readonly string[]): boolean {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) {
      return false;
    }
    cursor = index;
  }
  return true;
}

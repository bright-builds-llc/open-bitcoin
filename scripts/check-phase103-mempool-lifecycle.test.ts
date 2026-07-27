import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase103MempoolLifecycle } from "./check-phase103-mempool-lifecycle";
import { readSourceCorpus } from "./source-corpus";

const TARGET_FILES = [
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/storage.rs",
  "packages/open-bitcoin-node/src/storage/mempool_snapshot.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/tests.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec.rs",
  "packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs",
  "scripts/verify.sh",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-01-SUMMARY.md",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-02-SUMMARY.md",
  ".planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-03-SUMMARY.md",
] as const;
const REQUIRED_REQUIREMENTS = ["MEM-03", "MEM-04", "MEM-05", "MEM-06"] as const;

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

test("passes_when_phase103_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase103MempoolLifecycle(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase103_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase103MempoolLifecycle(root).join("\n"));

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_lifecycle_or_storage_symbol_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "MempoolPressureSummary");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "MempoolRemovalCause");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "MempoolRemovalRole");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "StorageNamespace::Mempool");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase103MempoolLifecycle(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("required Phase 103 symbol");
  expect(failureMessages[1]).toContain("required Phase 103 symbol");
  expect(failureMessages[2]).toContain("required Phase 103 symbol");
  expect(failureMessages[3]).toContain("required Phase 103 symbol");
});

test("fails_when_required_behavior_test_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromAllFiles(files, "managed_reorg_reacceptance_uses_explicit_event_time");
    },
  });

  // Act
  const failures = checkPhase103MempoolLifecycle(root).join("\n");

  // Assert
  expect(failures).toContain("required Phase 103 behavior test");
});

test("fails_when_source_breadcrumb_group_or_knots_anchor_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "docs/parity/source-breadcrumbs.json", "node-mempool-storage");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "packages/bitcoin-knots/src/node/mempool_persist.cpp");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase103MempoolLifecycle(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("source breadcrumb group");
  expect(failureMessages[1]).toContain("Knots anchor");
});

test("fails_when_default_verifier_wiring_is_missing_or_public_network_scoped", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(files, "scripts/verify.sh", "bun run scripts/check-phase103-mempool-lifecycle.ts");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 103 public-network relay CI" true');
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase103MempoolLifecycle(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("verifier-scope");
  expect(failureMessages[1]).toContain("verifier-scope forbidden");
});

test("fails_when_docs_claim_deferred_relay_or_production_surfaces", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(files, "docs/parity/catalog/mempool-policy.md", "Phase 103 supports relay serving.");
    },
  });

  // Act
  const failures = checkPhase103MempoolLifecycle(root).join("\n");

  // Assert
  expect(failures).toContain("forbidden positive Phase 103 claim");
});

test("allows_deferred_surface_claim_owned_by_a_later_phase", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/parity/checklist.md",
        "| later surface | done | Phase 112 provides compact block relay protocol evidence. |",
      );
    },
  });

  // Act
  const failures = checkPhase103MempoolLifecycle(root);

  // Assert
  expect(failures).toEqual([]);
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase103-"));
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

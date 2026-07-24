#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { checkCurrentDocumentationReconciliation } from "./check-current-documentation-reconciliation";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const REQUIRED_FILES = [
  "README.md",
  ".planning/ARCHITECTURE.md",
  ".planning/CONVENTIONS.md",
  "docs/parity/release-readiness.md",
  "docs/parity/support-matrix.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "packages/open-bitcoin-rpc/src/method.rs",
  "scripts/verify.sh",
] as const;
const tempRoots: string[] = [];

function createFixture(): string {
  const root = mkdtempSync(path.join(os.tmpdir(), "open-bitcoin-doc-reconciliation-"));
  tempRoots.push(root);
  for (const file of REQUIRED_FILES) {
    const destination = path.join(root, file);
    mkdirSync(path.dirname(destination), { recursive: true });
    writeFileSync(destination, readFileSync(path.join(REPO_ROOT, file), "utf8"));
  }
  return root;
}

function replaceInFixture(
  root: string,
  file: (typeof REQUIRED_FILES)[number],
  needle: string,
  replacement: string,
): void {
  const target = path.join(root, file);
  const original = readFileSync(target, "utf8");
  expect(original).toContain(needle);
  writeFileSync(target, original.replace(needle, replacement));
}

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("reconciled live corpus fixture passes", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkCurrentDocumentationReconciliation(root);

  // Assert
  expect(failures).toEqual([]);
});

test("README rejects the completed-milestone route", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "README.md",
    "Active milestone: v2.2",
    "/gsd-complete-milestone v2.1",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("README archived milestone state");
});

test("current v2.1 release section rejects archive-ready language", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/release-readiness.md",
    "Future work starts only with `/gsd-new-milestone`.",
    "The milestone is archive-ready pending completion.",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("release-readiness archived milestone state");
});

test("architecture rejects later v1.2 sync language", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    ".planning/ARCHITECTURE.md",
    "These are review boundaries, not public defaults:",
    "These remain later v1.2 phases:",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("architecture current sync boundary");
});

test("conventions reject pre-v1.2 sync language", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    ".planning/CONVENTIONS.md",
    "Current\n  full-sync",
    "Pre-v1.2\n  full-sync",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("conventions current operator boundary");
});

test("support matrix requires preview transaction relay", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/support-matrix.md",
    "| transaction relay | `preview` |",
    "| transaction relay | `deferred` |",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("support-matrix transaction relay must be preview");
});

test("production boundary rejects deferring bounded v2.0 relay", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/production-claim-boundary.md",
    "| Open Bitcoin provides bounded, explicit, default-off v2.0 transaction relay and mempool participation. | `preview` |",
    "| Open Bitcoin provides bounded, explicit, default-off v2.0 transaction relay and mempool participation. | `deferred` |",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("production boundary bounded v2.0 relay must be preview");
});

test("production boundary keeps broader transaction relay deferred", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/production-claim-boundary.md",
    "| public/default or production transaction relay beyond the bounded v2.0 path | `deferred` |",
    "| public/default or production transaction relay beyond the bounded v2.0 path | `preview` |",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("production boundary broader relay must be deferred");
});

test("deviation register rejects deferring bounded v2.0 relay", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/deviations-and-unknowns.md",
    "| bounded, explicit, default-off v2.0 transaction relay | `preview` |",
    "| bounded, explicit, default-off v2.0 transaction relay | `deferred` |",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("deviation register bounded v2.0 relay must be preview");
});

test("deviation register keeps broader transaction relay deferred", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/deviations-and-unknowns.md",
    "| public/default or production transaction relay beyond the bounded v2.0 path | `deferred` |",
    "| public/default or production transaction relay beyond the bounded v2.0 path | `preview` |",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("deviation register broader relay must be deferred");
});

test("catalog rejects a missing SupportedMethod serde name", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/catalog/rpc-cli-config.md",
    "  `deriveaddresses`, `sendtoaddress`, `getnewaddress`, `getrawchangeaddress`,\n",
    "  `deriveaddresses`, `getnewaddress`, `getrawchangeaddress`,\n",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("catalog supported-method set mismatch: missing sendtoaddress");
});

test("catalog rejects a catalog-only supported method", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/catalog/rpc-cli-config.md",
    "  `buildandsigntransaction`\n",
    "  `buildandsigntransaction`, and `catalogonlymethod`\n",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("catalog supported-method set mismatch: extra catalogonlymethod");
});

test("catalog rejects blanket sendtoaddress deferral", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/catalog/rpc-cli-config.md",
    "- deferred richer `send` RPC semantics",
    "- deferred `sendtoaddress` and richer `send` RPC semantics",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("catalog must not defer sendtoaddress wholesale");
});

test("catalog rejects blanket rpcwallet deferral", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "docs/parity/catalog/rpc-cli-config.md",
    "- deferred `loadwallet`, `unloadwallet`, and `listwallets`",
    "- deferred `-rpcwallet`, `loadwallet`, `unloadwallet`, and `listwallets`",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("catalog must not defer -rpcwallet wholesale");
});

test("verifier rejects a missing visible checker entry", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "scripts/verify.sh",
    "bun test scripts/check-current-documentation-reconciliation.test.ts\n",
    "",
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("verifier visible reconciliation order");
});

test("verifier rejects reordered executable checker entries", () => {
  // Arrange
  const root = createFixture();
  replaceInFixture(
    root,
    "scripts/verify.sh",
    'run_step "test current documentation reconciliation checker" bun test scripts/check-current-documentation-reconciliation.test.ts\nrun_step "check current documentation reconciliation" bun run scripts/check-current-documentation-reconciliation.ts\n',
    'run_step "check current documentation reconciliation" bun run scripts/check-current-documentation-reconciliation.ts\nrun_step "test current documentation reconciliation checker" bun test scripts/check-current-documentation-reconciliation.test.ts\n',
  );

  // Act
  const failures = checkCurrentDocumentationReconciliation(root).join("\n");

  // Assert
  expect(failures).toContain("verifier executable reconciliation order");
});

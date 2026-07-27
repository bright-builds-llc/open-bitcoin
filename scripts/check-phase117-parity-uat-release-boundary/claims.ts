import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "../check-phase117-parity-uat-release-boundary";
import { createFixture, append } from "./test-fixtures.ts";

test("fails_when_docs_claim_public_block_serving_by_default", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin supports public block serving by default.");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("forbidden positive Phase 117 claim");
});

test("fails_when_docs_claim_production_readiness", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "docs/parity/release-readiness.md", "v2.1 provides production full-node readiness.");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production full-node readiness");
});

test("fails_when_a_deferred_topic_masks_an_unrelated_positive_overclaim", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "docs/parity/release-readiness.md",
        "Package relay remains deferred, while production service operation is supported.",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production service operation");
});

test("fails_when_a_table_cell_mixes_a_deferred_topic_with_a_positive_overclaim", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "docs/parity/support-matrix.md",
        "| Package relay remains deferred, but Open Bitcoin supports production service operation. | `supported` |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production service operation");
});

test("fails_when_default_verifier_adds_a_public_network_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "scripts/verify.sh", 'run_step "live gate" bash scripts/run-live-mainnet-smoke.ts');
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("default verifier must not run run-live-mainnet-smoke");
});

test("allows_bounded_explicit_default_off_compact_relay_claims", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides bounded, explicit, default-off compact block relay.");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("allows_deferred_and_optional_uat_wording", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Package relay remains deferred. Public-network compact-relay review is optional UAT.");
      append(
        files,
        "docs/parity/support-matrix.md",
        "| Open Bitcoin supports production service operation. | `deferred` | not allowed yet |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

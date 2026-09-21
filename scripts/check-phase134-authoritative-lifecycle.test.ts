import { afterEach, expect, test } from "bun:test";
import { rmSync } from "node:fs";

import {
  PHASE134_TARGET_FILES,
  checkPhase134AuthoritativeLifecycle,
} from "./check-phase134-authoritative-lifecycle";
import {
  checkPhase134ApplyBoundaries,
} from "./check-phase134-apply-boundaries";
import {
  APPLY_BOUNDARY_DIAGNOSTIC,
  APPLY_HELPER_SOURCE_FILES,
  applyHelperMutations,
  applyHelperPositiveMutations,
} from "./check-phase134-authoritative-lifecycle.test/apply-helpers";
import {
  parityStatusMutations,
  scopeClaimMutations,
} from "./check-phase134-authoritative-lifecycle.test/scope-claims";
import {
  assertExactFailure,
  createFixture,
  insertInFunction,
  tempRoots,
} from "./check-phase134-authoritative-lifecycle.test/fixture";
import {
  authorityMutations,
  effectMutations,
  scenarioMutations,
  scopeMutations,
  targetMutations,
} from "./check-phase134-authoritative-lifecycle.test/mutations";

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 134 corpus", () => {
  // Arrange
  const root = createFixture(PHASE134_TARGET_FILES);

  // Act
  const failures = checkPhase134AuthoritativeLifecycle(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each(authorityMutations())(
  "rejects authority mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(targetMutations())(
  "rejects closed-target mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(effectMutations())(
  "rejects effect mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(scenarioMutations())(
  "rejects scenario mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(scopeMutations())(
  "rejects scope mutation: %s",
  (_name, expectedFailure, mutate) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(scopeClaimMutations())(
  "rejects canonical scope claim: $name",
  ({ expectedFailure, mutate }) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each(parityStatusMutations())(
  "rejects premature Phase 134 parity status: $name",
  ({ expectedFailure, mutate }) => {
    assertExactFailure(expectedFailure, mutate);
  },
);

test.each([
  ["Result return", "fn apply_prepared_compact(", " -> Result<(), Error>"],
  ["question propagation", "fn apply_prepared_compact(", "\nlet _ = derive()?;"],
  ["identifier derivation", "fn apply_prepared_compact(", "\ntransaction_txid(&tx);"],
  ["I/O type", "fn apply_prepared_compact(", "\nFile::open(\"state\");"],
  ["I/O call", "fn apply_prepared_compact(", "\nwriter.write_all(bytes);"],
  ["async await", "fn apply_prepared_compact(", "\nfuture.await;"],
] as const)(
  "rejects exact apply-body mutation: %s",
  (_name, functionMarker, addition) => {
    // Arrange
    const root = createFixture(APPLY_HELPER_SOURCE_FILES, (files) => {
      insertInFunction(
        files,
        "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs",
        functionMarker,
        addition,
      );
    });

    // Act
    const failures = checkPhase134ApplyBoundaries(root);

    // Assert
    expect(failures).toHaveLength(1);
    expect(failures[0]).toContain("apply_prepared_compact");
  },
);

test.each(applyHelperMutations())(
  "rejects transitive apply-helper mutation: $name",
  ({ mutate }) => {
    // Arrange
    const root = createFixture(APPLY_HELPER_SOURCE_FILES, mutate);

    // Act
    const failures = checkPhase134ApplyBoundaries(root);

    // Assert
    expect(failures).toEqual([APPLY_BOUNDARY_DIAGNOSTIC]);
  },
);

test.each([
  "accepts the exact pure peer identity helper",
  "accepts the legitimate aggregate core-first apply sequence",
])("%s", () => {
  // Arrange
  const root = createFixture(APPLY_HELPER_SOURCE_FILES);

  // Act
  const failures = checkPhase134ApplyBoundaries(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each(applyHelperPositiveMutations())("$name", ({ mutate }) => {
  // Arrange
  const root = createFixture(APPLY_HELPER_SOURCE_FILES, mutate);

  // Act
  const failures = checkPhase134ApplyBoundaries(root);

  // Assert
  expect(failures).toEqual([]);
});

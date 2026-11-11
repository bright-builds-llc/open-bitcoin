import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "../check-phase117-parity-uat-release-boundary";
import { createFixture, replace } from "./test-fixtures.ts";

test("fails_when_a_required_bazel_command_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "docs/operator/runtime-guide.md",
        "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
        "missing bazel command",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing Phase 117 runtime guide command");
});

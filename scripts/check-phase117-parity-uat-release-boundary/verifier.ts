import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "../check-phase117-parity-uat-release-boundary";
import { createFixture, replace, append } from "./test-fixtures.ts";

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

test("fails_when_visible_verifier_order_is_wrong", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        "bun run scripts/check-phase117-parity-uat-release-boundary.ts",
        "bun run scripts/missing-phase117.ts",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 visible commands");
});

test("fails_when_executable_verifier_order_is_wrong", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        'run_step "check Phase 117 parity UAT release boundary"',
        'run_step "missing Phase 117 parity UAT release boundary"',
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 executable commands");
});

test("fails_when_an_expected_run_step_label_executes_the_wrong_command", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        'run_step "check Phase 117 parity UAT release boundary" bun run scripts/check-phase117-parity-uat-release-boundary.ts',
        'run_step "check Phase 117 parity UAT release boundary" bash scripts/wrong-command.sh',
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 executable commands");
});

test("fails_when_visible_phase117_commands_follow_pure_core", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        "bun run scripts/check-phase117-parity-uat-release-boundary.ts\nbash scripts/check-pure-core-deps.sh",
        "bash scripts/check-pure-core-deps.sh\nbun run scripts/check-phase117-parity-uat-release-boundary.ts",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 visible commands");
});

test("fails_when_default_verifier_adds_a_generic_soak_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "scripts/verify.sh", 'run_step "external duration" bash scripts/run-soak-review.sh');
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("must not run soak workflows");
});

test("fails_when_default_verifier_hides_a_public_network_gate_on_a_continuation_line", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "scripts/verify.sh",
        ['run_step "external review" \\', "  bash scripts/public-network-review.sh"].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("default verifier must not run public-network");
});

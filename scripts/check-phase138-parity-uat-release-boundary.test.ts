import { expect, test } from "bun:test";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "./check-phase117-parity-uat-release-boundary.ts";
import { createFixture as createPhase117Fixture, append as append117 } from "./check-phase117-parity-uat-release-boundary/test-fixtures.ts";
import { checkPhase138ParityUatReleaseBoundary } from "./check-phase138-parity-uat-release-boundary.ts";
import {
  CLOSEOUT_SURFACE,
  COMPOSITION_SYMBOL,
  D21_SENTENCE,
  PHASE117_CHECK,
  PHASE132_SURFACE,
  PHASE138_CHECK,
  REQUIRED_UAT_COMMANDS,
  REQUIREMENTS_FILE,
} from "./check-phase138-parity-uat-release-boundary/constants.ts";
import { MATRIX_CELLS } from "./check-phase138-parity-uat-release-boundary/matrix.ts";
import {
  append,
  COMPOSITION_CELL,
  createFixture,
  createVerifyScript,
  mutateIndex,
  removeSymbol,
  replace,
} from "./check-phase138-parity-uat-release-boundary/test-fixtures.ts";

const CHECKER_SOURCES = [
  "scripts/check-phase138-parity-uat-release-boundary.ts",
  "scripts/check-phase138-parity-uat-release-boundary/checks.ts",
  "scripts/check-phase138-parity-uat-release-boundary/constants.ts",
  "scripts/check-phase138-parity-uat-release-boundary/matrix.ts",
  "scripts/check-phase138-parity-uat-release-boundary/claims.ts",
  "scripts/check-phase138-parity-uat-release-boundary/verifier.ts",
];

test("passes_on_a_complete_fixture", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_a_matrix_symbol_is_missing", () => {
  // Arrange
  const cell = MATRIX_CELLS[0];
  const root = createFixture({
    maybeMutate(files) {
      removeSymbol(files, cell.file, cell.symbol);
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(cell.symbol);
  expect(failures).toContain(`${cell.behavior}/${cell.method}`);
});

test("fails_when_the_restart_composition_symbol_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      removeSymbol(files, COMPOSITION_CELL.file, COMPOSITION_SYMBOL);
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(COMPOSITION_SYMBOL);
});

test("fails_when_a_v2_2_requirement_has_duplicate_owners", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const closeout = index.checklist.surfaces.find((surface: { id: string }) => surface.id === CLOSEOUT_SURFACE);
        closeout.requirements.push("PACK-01");
      });
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("PACK-01");
  expect(failures).toContain("exactly one");
});

test("fails_when_mpvfy_is_owned_by_a_phase132_surface", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const pack = index.checklist.surfaces.find((surface: { id: string }) => surface.id === PHASE132_SURFACE);
        pack.requirements.push("MPVFY-01");
        const closeout = index.checklist.surfaces.find((surface: { id: string }) => surface.id === CLOSEOUT_SURFACE);
        closeout.requirements = closeout.requirements.filter((id: string) => id !== "MPVFY-01");
      });
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("MPVFY-01");
  expect(failures).toContain(PHASE132_SURFACE);
});

test("fails_when_a_leftover_mpvfy_checkbox_is_unchecked", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, REQUIREMENTS_FILE, "- [x] **MPVFY-01**", "- [ ] **MPVFY-01**");
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("MPVFY-01");
  expect(failures).toContain("must be checked [x] exactly once");
});

test("fails_when_the_closeout_surface_stays_in_progress", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const closeout = index.checklist.surfaces.find(
          (surface: { id: string }) => surface.id === CLOSEOUT_SURFACE,
        );
        closeout.status = "in_progress";
      });
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(CLOSEOUT_SURFACE);
  expect(failures).toContain("must be done");
});

test("fails_when_the_d21_sentence_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, "README.md", D21_SENTENCE, "local package wording is omitted");
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(D21_SENTENCE);
});

test("fails_when_docs_claim_production_relay", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin supports production relay.");
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production relay");
});

test("fails_when_docs_claim_supports_package_relay_and_also_fails_phase117", () => {
  // Arrange
  const root138 = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin supports package relay.");
    },
  });
  const root117 = createPhase117Fixture({
    maybeMutate(files) {
      append117(files, "README.md", "Open Bitcoin supports package relay.");
    },
  });

  // Act
  const failures138 = checkPhase138ParityUatReleaseBoundary(root138).join("\n");
  const failures117 = checkPhase117ParityUatReleaseBoundary(root117).join("\n");

  // Assert
  expect(failures138).toContain("package relay");
  expect(failures117).toContain("package relay");
});

test("allows_future_gated_production_full_node_readiness_wording", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "README.md",
        "v1.8 defines the support terms required before a future production full-node readiness claim; v1.9 adds bounded opt-in inbound evidence without broadening that claim.",
      );
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("allows_verbatim_d21_sentence_on_phase117_and_phase138", () => {
  // Arrange
  const root138 = createFixture();
  const root117 = createPhase117Fixture({
    maybeMutate(files) {
      append117(files, "README.md", `Open Bitcoin provides ${D21_SENTENCE}.`);
    },
  });

  // Act
  const failures138 = checkPhase138ParityUatReleaseBoundary(root138);
  const failures117 = checkPhase117ParityUatReleaseBoundary(root117);

  // Assert
  expect(failures138).toEqual([]);
  expect(failures117).toEqual([]);
});

test("fails_when_a_required_cargo_uat_command_is_missing", () => {
  // Arrange
  const missing = REQUIRED_UAT_COMMANDS[0];
  const root = createFixture({
    maybeMutate(files) {
      replace(files, "docs/operator/runtime-guide.md", missing, "missing cargo uat command");
      replace(
        files,
        ".planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-UAT.md",
        missing,
        "missing cargo uat command",
      );
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(missing);
});

test("fails_when_the_last_check_phase_command_is_still_phase117", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set(
        "scripts/verify.sh",
        [
          ": <<'VERIFY_COMMAND_ORDER'",
          "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
          PHASE117_CHECK,
          "VERIFY_COMMAND_ORDER",
          'run_step "test Phase 117 parity UAT release boundary checker" bun test scripts/check-phase117-parity-uat-release-boundary.test.ts',
          `run_step "check Phase 117 parity UAT release boundary" ${PHASE117_CHECK}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(`must end with ${PHASE138_CHECK}`);
});

test("fails_when_phase138_is_placed_after_reconciliation", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ place138AfterReconciliation: true }));
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("117 then Phase 138 then reconciliation");
});

test("fails_when_the_instant_gate_is_reintroduced", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "packages/open-bitcoin-bench/src/cases/mempool.rs", "SUSTAINED_PRESSURE_MAX_ELAPSED\nInstant::now");
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toMatch(/SUSTAINED_PRESSURE_MAX_ELAPSED|Instant::now/);
});

test("fails_when_threshold_free_is_removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/check-benchmark-report.ts", "report.profile = {};\n");
    },
  });

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("threshold_free");
});

test("ignores_overclaims_outside_the_curated_claim_corpus", () => {
  // Arrange
  const root = createFixture();
  const archived = path.join(root, ".planning", "milestones", "overclaim.md");
  mkdirSync(path.dirname(archived), { recursive: true });
  writeFileSync(archived, "Open Bitcoin supports production relay.\n");

  // Act
  const failures = checkPhase138ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("checker_source_pins_d21_and_stays_filesystem_only", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "..");
  const source = CHECKER_SOURCES.map((file) => readFileSync(path.join(repoRoot, file), "utf8")).join(
    "\n",
  );

  // Act / Assert
  expect(source).toContain(D21_SENTENCE);
  expect(source).not.toContain("fetch(");
  expect(source).not.toContain("Bun.spawn");
  expect(source).not.toContain("node:child_process");
  expect(source).not.toContain(".planning/milestones");
  expect(MATRIX_CELLS).toHaveLength(24);
  expect(MATRIX_CELLS.some((cell) => cell.symbol === COMPOSITION_SYMBOL)).toBe(false);
  expect(source).toContain(COMPOSITION_SYMBOL);
});

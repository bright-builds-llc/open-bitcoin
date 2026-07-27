import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "../check-phase117-parity-uat-release-boundary";
import { createFixture, mutateIndex, mutateBreadcrumbs, replace } from "./test-fixtures.ts";

test("passes_when_phase117_closeout_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();
  const gapRoot = createFixture({ gapClosureStage: true });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);
  const gapFailures = checkPhase117ParityUatReleaseBoundary(gapRoot);

  // Assert
  expect(failures).toEqual([]);
  expect(gapFailures).toEqual([]);
});

test("passes_when_completed_gap_closure_retains_phase125_and_phase126_ownership", () => {
  // Arrange
  const root = createFixture({
    completedGapClosure: true,
    gapClosureStage: true,
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_when_post_audit_gap_planning_uses_phase127_through_phase129_ownership", () => {
  // Arrange
  const root = createFixture({ postAuditGapPlanning: true });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("uses_archived_v2_1_traceability_when_live_requirements_belong_to_a_later_milestone", () => {
  // Arrange
  const root = createFixture({ newerMilestoneRequirements: true });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_a_required_v2_1_surface_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.surfaces = index.surfaces.filter(
          (surface: { name?: string }) => surface.name !== "v2-1-compact-block-reconstruction",
        );
        index.checklist.surfaces = index.checklist.surfaces.filter(
          (surface: { id?: string }) => surface.id !== "v2-1-compact-block-reconstruction",
        );
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing v2.1 surface v2-1-compact-block-reconstruction");
});

test("fails_when_a_requirement_has_duplicate_surface_owners", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.checklist.surfaces[0].requirements.push("BOUND-01");
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("BOUND-01 must have exactly one parity surface owner");
});

test("fails_when_a_surface_entry_is_duplicated", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.surfaces.push({ ...index.surfaces[0] });
        index.checklist.surfaces.push({ ...index.checklist.surfaces[0] });
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("must have exactly one top-level and checklist entry");
});

test("fails_when_a_checklist_surface_is_not_done", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.checklist.surfaces[0].status = "blocked";
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("checklist v2.1 surface");
  expect(failures).toContain("must be done");
});

test("fails_when_a_required_knots_anchor_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        for (const surface of index.checklist.surfaces) {
          surface.upstream.sources = surface.upstream.sources.filter(
            (anchor: string) => anchor !== "packages/bitcoin-knots/src/blockencodings.cpp",
          );
          surface.upstream.tests = surface.upstream.tests.filter(
            (anchor: string) => anchor !== "packages/bitcoin-knots/src/blockencodings.cpp",
          );
        }
      });
      mutateBreadcrumbs(files, (breadcrumbs) => {
        for (const group of breadcrumbs.groups) {
          group.breadcrumbs = group.breadcrumbs.filter(
            (anchor: string) => anchor !== "packages/bitcoin-knots/src/blockencodings.cpp",
          );
        }
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing Phase 117 parity-index Knots anchor");
  expect(failures).toContain("missing Phase 117 breadcrumb Knots anchor");
});

test("fails_when_a_required_breadcrumb_group_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, "docs/parity/source-breadcrumbs.json", "network-compact-block-download", "missing-group");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing breadcrumb group network-compact-block-download");
});

test("fails_when_a_required_cargo_command_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "docs/operator/runtime-guide.md",
        "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
        "missing cargo command",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing Phase 117 runtime guide command");
});

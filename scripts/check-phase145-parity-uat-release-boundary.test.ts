import { expect, test } from "bun:test";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

import { checkPhase145ParityUatReleaseBoundary } from "./check-phase145-parity-uat-release-boundary.ts";
import {
  CAN_FLUSH_TO_DISK,
  CHECK_BLOCK_DATA_AVAILABILITY,
  CLAIM_FILES,
  CLOSEOUT_SURFACE,
  D14_SENTENCE,
  HISTORICAL_PHASE138_DIR,
  PHASE117_CHECK,
  PHASE138_CHECK,
  PHASE139_SURFACE,
  REQUIRED_KNOTS_ANCHORS,
  REQUIREMENTS_FILE,
  STORED_BLOCK_PRESENCE_GROUP,
} from "./check-phase145-parity-uat-release-boundary/constants.ts";
import {
  append,
  createFixture,
  createVerifyScript,
  mutateIndex,
  replace,
} from "./check-phase145-parity-uat-release-boundary/test-fixtures.ts";

const CHECKER_SOURCES = [
  "scripts/check-phase145-parity-uat-release-boundary.ts",
  "scripts/check-phase145-parity-uat-release-boundary/checks.ts",
  "scripts/check-phase145-parity-uat-release-boundary/constants.ts",
  "scripts/check-phase145-parity-uat-release-boundary/claims.ts",
  "scripts/check-phase145-parity-uat-release-boundary/verifier.ts",
];

test("passes_on_a_complete_fixture", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_a_v2_3_requirement_has_duplicate_owners", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const closeout = index.checklist.surfaces.find(
          (surface: { id: string }) => surface.id === CLOSEOUT_SURFACE,
        );
        closeout.requirements.push("CACHE-01");
      });
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("CACHE-01");
  expect(failures).toContain("exactly one");
});

test("fails_when_csvfy_is_owned_by_a_phase139_surface", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const coins = index.checklist.surfaces.find(
          (surface: { id: string }) => surface.id === PHASE139_SURFACE,
        );
        coins.requirements.push("CSVFY-01");
        const closeout = index.checklist.surfaces.find(
          (surface: { id: string }) => surface.id === CLOSEOUT_SURFACE,
        );
        closeout.requirements = closeout.requirements.filter((id: string) => id !== "CSVFY-01");
      });
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("CSVFY-01");
  expect(failures).toContain(PHASE139_SURFACE);
});

test("fails_when_a_leftover_csvfy_checkbox_is_unchecked", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, REQUIREMENTS_FILE, "- [x] **CSVFY-01**", "- [ ] **CSVFY-01**");
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("CSVFY-01");
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
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(CLOSEOUT_SURFACE);
  expect(failures).toContain("must be done");
});

test("fails_when_a_d06_file_is_missing_from_the_closeout_surface", () => {
  // Arrange
  const missingAnchor = REQUIRED_KNOTS_ANCHORS[0];
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const closeout = index.checklist.surfaces.find(
          (surface: { id: string }) => surface.id === CLOSEOUT_SURFACE,
        );
        closeout.upstream.sources = closeout.upstream.sources.filter(
          (source: string) => source !== missingAnchor,
        );
      });
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(missingAnchor);
});

test("fails_when_node_stored_block_presence_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      const parsed = JSON.parse(files.get("docs/parity/source-breadcrumbs.json") ?? "{}") as {
        groups?: Array<{ label?: string }>;
      };
      parsed.groups = (parsed.groups ?? []).filter(
        (group) => group.label !== STORED_BLOCK_PRESENCE_GROUP,
      );
      files.set("docs/parity/source-breadcrumbs.json", JSON.stringify(parsed, null, 2));
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(STORED_BLOCK_PRESENCE_GROUP);
});

test("fails_when_the_d14_sentence_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, "README.md", D14_SENTENCE, "local coins wording is omitted");
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(D14_SENTENCE);
});

test("fails_when_docs_claim_prune_mode_product_behavior", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides prune-mode product behavior.");
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("prune-mode");
});

test("fails_when_docs_claim_archive_node_or_production_scale_historical_serving", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "README.md",
        "Open Bitcoin provides archive-node or production-scale historical serving.",
      );
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toMatch(/archive-node|production-scale historical serving/);
});

test("fails_when_docs_claim_assumeutxo", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin enables assumeutxo.");
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("assumeutxo");
});

test("accepts_deferred_prune_wording", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin does not add prune-mode product behavior.");
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("accepts_the_verbatim_d14_sentence", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "docs/parity/checklist.md", D14_SENTENCE);
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("ignores_overclaims_outside_the_curated_claim_corpus", () => {
  // Arrange
  const root = createFixture();
  const archived = path.join(root, ".planning", "milestones", "overclaim.md");
  mkdirSync(path.dirname(archived), { recursive: true });
  writeFileSync(archived, "Open Bitcoin provides prune-mode product behavior.\n");

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("checker_source_pins_d14_and_stays_filesystem_only", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "..");
  const source = CHECKER_SOURCES.map((file) => readFileSync(path.join(repoRoot, file), "utf8")).join(
    "\n",
  );

  // Act / Assert
  expect(source).toContain(D14_SENTENCE);
  expect(source).toContain(CAN_FLUSH_TO_DISK);
  expect(source).toContain(CHECK_BLOCK_DATA_AVAILABILITY);
  expect(source).toContain("OPEN_BITCOIN_PHASE145_REPO_ROOT");
  expect(source).not.toContain("fetch(");
  expect(source).not.toContain("Bun.spawn");
  expect(source).not.toContain("node:child_process");
  expect(CLAIM_FILES.some((file) => file.includes(".planning/phases"))).toBe(false);
  expect(CLAIM_FILES.some((file) => file.includes(".planning/milestones"))).toBe(false);
  expect(REQUIRED_KNOTS_ANCHORS.join("\n")).not.toContain("HaveBlockData");
  expect(source).not.toContain("final gate must end with");
  expect(source).not.toContain("lastPhaseCommand !==");
});

test("fails_when_phase145_is_placed_before_phase144", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ place145Before144: true }));
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("Phase 144 then Phase 145");
});

test("fails_when_phase117_is_removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ omit117: true }));
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(PHASE117_CHECK);
});

test("fails_when_phase138_is_removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ omit138: true }));
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(PHASE138_CHECK);
});

test("fails_when_run_live_mainnet_smoke_is_added_as_a_run_step", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ addLiveMainnetSmoke: true }));
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("run-live-mainnet-smoke");
});

test("passes_when_the_last_check_phase_command_is_still_phase138", () => {
  // Arrange
  const root = createFixture();
  const verifyText = readFileSync(path.join(root, "scripts", "verify.sh"), "utf8");
  const phaseCommands = [...verifyText.matchAll(/bun (?:test|run) scripts\/check-phase\d+\S*/g)].map(
    (match) => match[0],
  );

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root);

  // Assert
  expect(phaseCommands.at(-1)).toBe(PHASE138_CHECK);
  expect(failures).toEqual([]);
});

test("fails_when_the_support_bundle_uat_command_is_missing", () => {
  // Arrange
  const missing =
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support";
  const root = createFixture({
    maybeMutate(files) {
      replace(files, "docs/operator/runtime-guide.md", missing, "missing support bundle command");
      replace(
        files,
        ".planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md",
        missing,
        "missing support bundle command",
      );
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("/tmp/open-bitcoin-chainstate-durability-support");
});

test("fails_when_a_verifier_referenced_historical_phase_path_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.delete(`${HISTORICAL_PHASE138_DIR}/138-CONTEXT.md`);
    },
  });

  // Act
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing verifier-referenced historical phase path");
  expect(failures).toContain(HISTORICAL_PHASE138_DIR);
});

import { expect, test } from "bun:test";
import "./check-phase151-parity-uat-release-boundary/gap-closure.test.ts";
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase151ParityUatReleaseBoundary } from "./check-phase151-parity-uat-release-boundary.ts";
import {
  CLAIM_FILES,
  CLOSEOUT_SURFACE,
  DENIED_OVERCLAIMS,
  GRD01_CHECKBOX,
  GRD01_ID,
  HISTORICAL_PHASE_DIR,
  PHASE151_CHECK_STEP,
  PHASE151_TEST_STEP,
  REQUIREMENTS_BY_SURFACE,
  REQUIREMENTS_FILE,
  ROADMAP_FILE,
  UAT_PACKAGE,
  V23_D14_SENTENCE,
  V24_PRUNE_SENTENCE,
  WALLET_LEFTOVER_SURFACE,
} from "./check-phase151-parity-uat-release-boundary/constants.ts";
import {
  append,
  createFixture,
  createVerifyScript,
  mutateIndex,
  replace,
  tempRoots,
} from "./check-phase151-parity-uat-release-boundary/test-fixtures.ts";

const CHECKER_SOURCES = [
  "scripts/check-phase151-parity-uat-release-boundary.ts",
  "scripts/check-phase151-parity-uat-release-boundary/checks.ts",
  "scripts/check-phase151-parity-uat-release-boundary/constants.ts",
  "scripts/check-phase151-parity-uat-release-boundary/claims.ts",
  "scripts/check-phase151-parity-uat-release-boundary/verifier.ts",
  "scripts/check-phase151-parity-uat-release-boundary/gap-closure.ts",
  "scripts/check-phase151-parity-uat-release-boundary/planning-sources.ts",
];

test("passes_on_a_complete_fixture", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_grd01_has_a_duplicate_owner", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const wallet = index.checklist.surfaces.find(
          (surface) => surface.id === WALLET_LEFTOVER_SURFACE,
        );
        wallet?.requirements.push(GRD01_ID);
      });
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  const exactlyOneOwner = failures.filter((failure) =>
    failure.includes("exactly one parity surface owner"),
  );
  expect(exactlyOneOwner).toEqual([`${GRD01_ID} must have exactly one parity surface owner`]);
});

test("fails_when_grd01_moves_onto_the_wallet_leftover_surface", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        const closeout = index.checklist.surfaces.find((surface) => surface.id === CLOSEOUT_SURFACE);
        if (closeout) closeout.requirements = closeout.requirements.filter((id) => id !== GRD01_ID);
        const wallet = index.checklist.surfaces.find(
          (surface) => surface.id === WALLET_LEFTOVER_SURFACE,
        );
        wallet?.requirements.push(GRD01_ID);
      });
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain(GRD01_ID);
  expect(failures).toContain(WALLET_LEFTOVER_SURFACE);
});

test("fails_when_readme_claims_archive_node_serving", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides archive-node serving.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("archive-node");
});

test("fails_when_readme_claims_assumeutxo", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides assumeutxo.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("assumeutxo");
});

test("fails_when_readme_claims_a_second_chainstate", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides a second chainstate.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("second chainstate");
});

test("fails_when_readme_claims_bip37_bloom_serving", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides BIP37 bloom serving.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("bip37");
});

test("fails_when_readme_claims_public_serving_or_relay_by_default", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides public serving or relay by default.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("public serving or relay by default");
});

test("fails_when_readme_claims_production_full_node_readiness", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides production full-node readiness.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production full-node readiness");
});

test("fails_when_readme_claims_a_blk_rev_flat_file_store", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides a blk/rev flat-file store.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("blk/rev");
});

test("fails_when_readme_claims_pruneduringinit", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides -pruneduringinit.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("pruneduringinit");
});

test("fails_when_readme_claims_txindex_with_prune", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides txindex with prune.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("txindex with prune");
});

test("accepts_the_verbatim_v24_prune_sentence", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("README.md", `${V23_D14_SENTENCE}\n\n${V24_PRUNE_SENTENCE}`);
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("accepts_deferred_archive_wording", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Archive serving remains deferred.");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_the_v24_sentence_and_archive_node_share_one_clause", () => {
  // Arrange
  const allowedSentence = V24_PRUNE_SENTENCE.endsWith(".")
    ? V24_PRUNE_SENTENCE.slice(0, -1)
    : V24_PRUNE_SENTENCE;
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", `${allowedSentence} provides archive-node serving.`);
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("archive-node");
});

test("fails_when_a_phase151_uat_command_is_missing", () => {
  // Arrange
  const missing =
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune run 1000";
  const root = createFixture({
    maybeMutate(files) {
      replace(files, UAT_PACKAGE, missing, "missing prune run command");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("prune run 1000");
});

test("fails_when_verify_sh_omits_the_phase151_run_step", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ omitPhase151: true }));
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("Phase 145 then Phase 151");
});

test("fails_when_a_referenced_historical_phase_directory_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.delete(`${HISTORICAL_PHASE_DIR}/145-CONTEXT.md`);
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing verifier-referenced historical phase path");
  expect(failures).toContain(HISTORICAL_PHASE_DIR);
});

test("fails_closed_when_repo_root_override_is_outside_the_fixture", () => {
  // Arrange
  const fixtureRoot = createFixture();
  const outsideRoot = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase151-outside-"));
  tempRoots.push(outsideRoot);
  const previous = process.env.OPEN_BITCOIN_PHASE151_REPO_ROOT;
  process.env.OPEN_BITCOIN_PHASE151_REPO_ROOT = outsideRoot;

  try {
    // Act
    const failures = checkPhase151ParityUatReleaseBoundary();

    // Assert
    expect(path.resolve(outsideRoot)).not.toBe(path.resolve(fixtureRoot));
    expect(failures.length).toBeGreaterThan(0);
    expect(failures.join("\n")).toContain("missing target file");
  } finally {
    if (previous === undefined) delete process.env.OPEN_BITCOIN_PHASE151_REPO_ROOT;
    else process.env.OPEN_BITCOIN_PHASE151_REPO_ROOT = previous;
  }
});

test("fails_when_a_forbidden_run_step_token_is_added", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      files.set("scripts/verify.sh", createVerifyScript({ addForbiddenRunStep: true }));
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("run-live-mainnet-smoke");
});

test("fails_when_grd01_checkbox_is_unchecked", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, REQUIREMENTS_FILE, "- [x] **GRD-01**", "- [ ] **GRD-01**");
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("GRD-01");
});

test("fails_when_grd01_roadmap_row_is_pending", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        ROADMAP_FILE,
        "| GRD-01 | Phase 151 | Complete |",
        "| GRD-01 | Phase 151 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("GRD-01");
  expect(failures).toContain("Complete");
});

test("checker_source_pins_the_v24_contract_and_stays_filesystem_only", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "..");
  const source = CHECKER_SOURCES.map((file) => readFileSync(path.join(repoRoot, file), "utf8")).join(
    "\n",
  );

  // Act / Assert
  expect(source).toContain("checkPhase151ParityUatReleaseBoundary");
  expect(source).toContain("OPEN_BITCOIN_PHASE151_REPO_ROOT");
  expect(source).toContain(V24_PRUNE_SENTENCE);
  expect(source).toContain(GRD01_CHECKBOX);
  expect(source).toContain("second chainstate");
  expect(PHASE151_TEST_STEP).toBe(
    'run_step "test Phase 151 parity UAT release boundary checker" bun test scripts/check-phase151-parity-uat-release-boundary.test.ts',
  );
  expect(PHASE151_CHECK_STEP).toBe(
    'run_step "check Phase 151 parity UAT release boundary" bun run scripts/check-phase151-parity-uat-release-boundary.ts',
  );
  expect(CLAIM_FILES).toHaveLength(9);
  expect(CLAIM_FILES.some((file) => file.includes(".planning/"))).toBe(false);
  expect(REQUIREMENTS_BY_SURFACE[CLOSEOUT_SURFACE]).toEqual([GRD01_ID]);
  expect(DENIED_OVERCLAIMS).toContain("archive-node");
  expect(DENIED_OVERCLAIMS).toContain("assumeutxo");
  expect(DENIED_OVERCLAIMS).toContain("second chainstate");
  expect(DENIED_OVERCLAIMS).toContain("bip37");
  expect(DENIED_OVERCLAIMS).toContain("public serving or relay by default");
  expect(DENIED_OVERCLAIMS).toContain("production full-node readiness");
  expect(DENIED_OVERCLAIMS).toContain("blk/rev");
  expect(DENIED_OVERCLAIMS).toContain("pruneduringinit");
  expect(source).not.toContain("fetch(");
  expect(source).not.toContain("Bun.spawn");
  expect(source).not.toContain("node:child_process");
  expect(source).toContain("resolvePlanningRequirementsSource");
});

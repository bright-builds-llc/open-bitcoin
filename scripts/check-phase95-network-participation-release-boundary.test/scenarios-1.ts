import { expect, test } from "bun:test";
import { SURFACE_ID, PHASE94_TEST_COMMAND, PHASE94_CHECKER_COMMAND, PHASE95_TEST_COMMAND, PHASE95_CHECKER_COMMAND, PURE_CORE_COMMAND, REQUIRED_KNOTS_ANCHORS, PHASE_REQUIREMENTS, REQUIREMENT_PHASE_ASSIGNMENTS, ROADMAP_TRACEABILITY_ROWS, TARGET_FILES, tempRoots, createFixture, fixtureFiles, removeFromAllFiles, replaceInFile, parityIndexText, readmeText, checklistText, p2pCatalogText, releaseReadinessText, productionBoundaryText, supportMatrixText, runtimeGuideText, redactionText, supportTestsText, requirementsText, roadmapText, phaseTraceRows, verifyScriptText, mkdirSync, mkdtempSync, rmSync, writeFileSync, tmpdir, path, checkPhase95NetworkParticipationReleaseBoundary } from "./setup.ts";
import type { TargetFile, FixtureOptions } from "./setup.ts";
test("passes with complete Phase 95 release-boundary corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when any required Knots anchor is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromAllFiles(files, "packages/bitcoin-knots/src/net.cpp");
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-02");
});

test("fails positive deferred network participation claims", () => {
  // Arrange
  const claims = [
    "Phase 95 provides transaction relay support.",
    "Phase 95 provides compact block relay support.",
    "Phase 95 provides mempool propagation support.",
    "Phase 95 provides full address relay support.",
    "Phase 95 provides public inbound default behavior.",
    "Phase 95 says public-network CI is enabled.",
    "Phase 95 supports production service operation.",
    "Phase 95 has production full-node readiness.",
  ];
  const roots = claims.map((claim) =>
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/catalog/p2p.md") ?? "";
        files.set("docs/parity/catalog/p2p.md", `${current}\n${claim}\n`);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-01");
  }
});

test("fails same-unit positive claims with unrelated allowance wording", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/checklist.md") ?? "";
        files.set(
          "docs/parity/checklist.md",
          `${current}\n| \`future-mask\` | \`done\` | \`BOUND-01\` | Phase 95 provides transaction relay support. | Future scoped evidence required. |\n`,
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/catalog/p2p.md") ?? "";
        files.set(
          "docs/parity/catalog/p2p.md",
          `${current}\nPhase 95 provides transaction relay support while a future scoped relay audit remains pending.\n`,
        );
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-01");
  }
});

test("fails positive README network participation claims", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      const current = files.get("README.md") ?? "";
      files.set("README.md", `${current}\nPhase 95 provides transaction relay support.\n`);
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-01");
});

test("fails when required Cargo or Bazel UAT command families are missing", () => {
  // Arrange
  const roots = [
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin",
  ].map((missingCommand) =>
    createFixture({
      maybeMutateFiles(files) {
        replaceInFile(files, "docs/operator/runtime-guide.md", missingCommand, "");
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-04");
  }
});

test("fails when resource-governance support redaction roots are missing", () => {
  // Arrange
  const roots = [
    "redact_inbound_resource_governance_evidence",
    "inbound_support_redacts_raw_phase94_resource_governance_material",
  ].map((missingRoot) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, missingRoot);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-05");
  }
});

test("fails when v1.9 requirement IDs are duplicated or omitted", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "docs/parity/index.json", '"BOUND-06"', '"BOUND-05"');
      replaceInFile(files, "docs/parity/checklist.md", "`BOUND-06`", "`BOUND-05`");
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-06");
});

test("allows later non-v1.9 checklist surfaces to reuse requirement ids", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      const index = JSON.parse(files.get("docs/parity/index.json") ?? "{}");
      index.checklist.surfaces.push({
        id: "v2-0-later-boundary",
        status: "done",
        requirements: ["BOUND-01"],
        evidence: ["docs/parity/catalog/p2p.md"],
      });
      files.set("docs/parity/index.json", `${JSON.stringify(index, null, 2)}\n`);
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when gap-closure traceability maps requirements to stale phases", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/milestones/v1.9-REQUIREMENTS.md",
        "| INB-05 | Phase 97 | Complete |",
        "| INB-05 | Phase 90 | Complete |",
      );
      replaceInFile(
        files,
        ".planning/milestones/v1.9-ROADMAP.md",
        "| Phase 97 | INB-05, DOS-04 | 2 |",
        "| Phase 97 | — | 0 |",
      );
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain(
    "BOUND-06 requirements traceability missing INB-05 -> Phase 97",
  );
  expect(failures.join("\n")).toContain("BOUND-06 roadmap phase traceability");
});

test("fails when Phase 95 verifier commands exist only in VERIFY_COMMAND_ORDER", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "scripts/verify.sh",
        [
          "#!/usr/bin/env bash",
          "set -euo pipefail",
          ": <<'VERIFY_COMMAND_ORDER'",
          PHASE94_TEST_COMMAND,
          PHASE94_CHECKER_COMMAND,
          PHASE95_TEST_COMMAND,
          PHASE95_CHECKER_COMMAND,
          "VERIFY_COMMAND_ORDER",
          `run_step "Phase 94 DoS/resource governance checker tests" ${PHASE94_TEST_COMMAND}`,
          `run_step "Phase 94 DoS/resource governance checker" ${PHASE94_CHECKER_COMMAND}`,
          `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-03");
});

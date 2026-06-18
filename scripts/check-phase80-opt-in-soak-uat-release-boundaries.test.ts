#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase80OptInSoakUatReleaseBoundaries } from "./check-phase80-opt-in-soak-uat-release-boundaries";

const PHASE_DIR = ".planning/phases/80-opt-in-soak-uat-and-release-boundaries";
const PLAN_FILES = [
  `${PHASE_DIR}/80-01-PLAN.md`,
  `${PHASE_DIR}/80-02-PLAN.md`,
  `${PHASE_DIR}/80-03-PLAN.md`,
] as const;
const PHASE80_REQUIREMENTS = ["VER-05", "VER-06", "VER-07", "REL-04"] as const;
const SURFACE_ID = "v1-7-full-sync-soak-recovery-release-boundaries";
const PHASE75_TEST_COMMAND = "bun test scripts/check-phase75-soak-runner.test.ts";
const PHASE75_CHECKER_COMMAND = "bun run scripts/check-phase75-soak-runner.ts";
const PHASE76_TEST_COMMAND = "bun test scripts/check-phase76-resource-bounds.test.ts";
const PHASE76_CHECKER_COMMAND = "bun run scripts/check-phase76-resource-bounds.ts";
const PHASE77_TEST_COMMAND = "bun test scripts/check-phase77-corruption-lock-recovery.test.ts";
const PHASE77_CHECKER_COMMAND = "bun run scripts/check-phase77-corruption-lock-recovery.ts";
const PHASE78_TEST_COMMAND = "bun test scripts/check-phase78-progress-guarantees.test.ts";
const PHASE78_CHECKER_COMMAND = "bun run scripts/check-phase78-progress-guarantees.ts";
const PHASE79_TEST_COMMAND =
  "bun test scripts/check-phase79-diagnostics-support-bundle.test.ts";
const PHASE79_CHECKER_COMMAND = "bun run scripts/check-phase79-diagnostics-support-bundle.ts";
const PHASE80_TEST_COMMAND =
  "bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts";
const PHASE80_CHECKER_COMMAND =
  "bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts";
const DEFAULT_VERIFY_SCRIPT = [
  PHASE75_TEST_COMMAND,
  PHASE75_CHECKER_COMMAND,
  PHASE76_TEST_COMMAND,
  PHASE76_CHECKER_COMMAND,
  PHASE77_TEST_COMMAND,
  PHASE77_CHECKER_COMMAND,
  PHASE78_TEST_COMMAND,
  PHASE78_CHECKER_COMMAND,
  PHASE79_TEST_COMMAND,
  PHASE79_CHECKER_COMMAND,
  PHASE80_TEST_COMMAND,
  PHASE80_CHECKER_COMMAND,
].join("\n");
const NON_CLAIMS = [
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet use",
  "migration apply mode",
  "signed packaging",
  "Windows service support",
  "GUI",
  "hosted dashboards",
  "public-network default checks",
  "public-network CI",
  "release-blocking live sync",
  "automatic support-bundle upload",
  "destructive repair",
  "broad production-node readiness",
].join("\n");
const CLAIM_TEXT = [
  "explicit opt-in full-sync soak and recovery hardening",
  SURFACE_ID,
  PHASE80_REQUIREMENTS.join(" "),
  NON_CLAIMS,
].join("\n");
const RUNTIME_GUIDE_TEXT = [
  "### Phase 80 v1.7 opt-in soak UAT matrix",
  "Evidence proves",
  "Does not prove",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "durable run identity",
  "typed final outcome",
  "recovery_evidence",
  "support_forensics",
  "forensic timeline",
  "checkpoint chain",
  "failure narrative",
  "artifact presence",
  "daemon startup",
  "peer reachability",
  "elapsed time",
  "raw logs",
  "stale reports",
  "| Workflow | Repo-local Cargo commands | Repo-local Bazel commands | Evidence proves | Does not prove |",
  "| --- | --- | --- | --- | --- |",
  "| Multi-day soak lifecycle | cargo | bazel | durable run identity | elapsed time is not enough |",
  "| Bounded recovery drill | cargo | bazel | recovery_evidence | no source-datadir mutation |",
  "| Support-bundle generation | cargo | bazel | support_forensics | bundle existence is not enough |",
  "| Post-failure diagnosis | cargo | bazel | failure narrative | no production readiness |",
  CLAIM_TEXT,
].join("\n");
const SOURCE_TEXTS: Record<string, string> = {
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "SupportEvidenceBundle",
    "support_forensics",
    "resource_bound_evidence",
    "soak_evidence",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/forensics.rs": [
    "SupportForensicsEvidence",
    "ForensicTimelineEntry",
    "CheckpointChainEvidence",
    "ForensicNarrative",
  ].join("\n"),
  "packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs": "SoakSupportEvidence",
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": "SoakReportProjection",
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": "SoakLedgerEventEnvelope",
};
const DOC_TEXTS: Record<string, string> = {
  "README.md": CLAIM_TEXT,
  "docs/operator/runtime-guide.md": RUNTIME_GUIDE_TEXT,
  "docs/parity/release-readiness.md": CLAIM_TEXT,
  "docs/parity/checklist.md": CLAIM_TEXT,
  "docs/parity/README.md": CLAIM_TEXT,
  "docs/parity/deviations-and-unknowns.md": CLAIM_TEXT,
  "docs/parity/catalog/operator-runtime-release-hardening.md": CLAIM_TEXT,
};

type FixtureOptions = {
  maybeAppend?: {
    file: string;
    text: string;
  };
  maybeManifestPath?: string;
  maybeOmission?: {
    file: string;
    needle: string;
  };
  maybeParityIndexText?: string;
  maybePlanTexts?: readonly string[];
  maybeVerifyScript?: string;
};

type CheckerRun = {
  exitCode: number;
  stderr: string;
};

const tempRoots: string[] = [];

afterEach(async () => {
  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot === undefined) {
      continue;
    }

    await rm(maybeRoot, { force: true, recursive: true });
  }
});

test("passes_when_phase80_fixture_contains_every_release_boundary_anchor", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).toBe(0);
});

test("fails_when_uat_matrix_missing_workflow_or_proof_boundary", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/operator/runtime-guide.md",
      needle: "Post-failure diagnosis",
    },
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain("Phase 80 UAT matrix");
});

test("fails_when_parity_roots_omit_v1_7_requirements_or_surface", async () => {
  // Arrange
  const root = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll("VER-07", ""),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain(`${SURFACE_ID}.requirements`);
});

test("fails_when_verify_order_or_default_boundary_forbidden_strings_drift", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      PHASE75_TEST_COMMAND,
      PHASE75_CHECKER_COMMAND,
      PHASE76_TEST_COMMAND,
      PHASE76_CHECKER_COMMAND,
      PHASE77_TEST_COMMAND,
      PHASE77_CHECKER_COMMAND,
      PHASE78_TEST_COMMAND,
      PHASE78_CHECKER_COMMAND,
      PHASE79_TEST_COMMAND,
      PHASE79_CHECKER_COMMAND,
      "bun run scripts/check-unrelated-boundary.ts",
      PHASE80_TEST_COMMAND,
      PHASE80_CHECKER_COMMAND,
      "bun run scripts/run-live-mainnet-smoke.ts",
    ].join("\n"),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain("Phase 80 checker test and checker immediately after Phase 79");
  expect(result.stderr).toContain("run-live-mainnet-smoke");
});

test("fails_when_broad_claim_or_new_manifest_appears", async () => {
  // Arrange
  const root = await createFixture({
    maybeAppend: {
      file: "README.md",
      text: "\nv1.7 proves broad production-node readiness.\n",
    },
    maybeManifestPath: "docs/parity/v1.7-evidence-manifest.json",
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
  expect(result.stderr).toContain("broad production-node readiness");
  expect(result.stderr).toContain("v1.7-evidence-manifest.json");
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase80-checker-"));
  tempRoots.push(root);

  const maybePlanTexts =
    options.maybePlanTexts ??
    PLAN_FILES.map(
      () => `---\nrequirements: [${PHASE80_REQUIREMENTS.join(", ")}]\n---\n`,
    );
  for (let index = 0; index < PLAN_FILES.length; index += 1) {
    await writeFixtureFile(root, PLAN_FILES[index], maybePlanTexts[index] ?? "");
  }

  for (const [file, text] of Object.entries({ ...DOC_TEXTS, ...SOURCE_TEXTS })) {
    await writeFixtureFile(root, file, applyTextOptions(file, text, options));
  }
  await writeFixtureFile(
    root,
    "docs/parity/index.json",
    options.maybeParityIndexText ?? parityIndexText(),
  );
  await writeFixtureFile(
    root,
    "scripts/verify.sh",
    options.maybeVerifyScript ?? DEFAULT_VERIFY_SCRIPT,
  );
  if (options.maybeManifestPath !== undefined) {
    await writeFixtureFile(root, options.maybeManifestPath, "{}\n");
  }

  return root;
}

function parityIndexText(): string {
  const evidence = [
    "docs/operator/runtime-guide.md",
    "docs/parity/release-readiness.md",
    "docs/parity/index.json",
    "docs/parity/checklist.md",
    "docs/parity/README.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/catalog/operator-runtime-release-hardening.md",
    "docs/parity/source-breadcrumbs.json",
    "scripts/check-parity-breadcrumbs.ts",
    "scripts/check-phase75-soak-runner.ts",
    "scripts/check-phase76-resource-bounds.ts",
    "scripts/check-phase77-corruption-lock-recovery.ts",
    "scripts/check-phase78-progress-guarantees.ts",
    "scripts/check-phase79-diagnostics-support-bundle.ts",
    "scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts",
    "scripts/verify.sh",
    `${PHASE_DIR}/80-VERIFICATION.md`,
  ];
  return `${JSON.stringify(
    {
      audit: {
        v1_7_release_boundaries: {
          evidence,
          path: "release-readiness.md",
          requirements: [...PHASE80_REQUIREMENTS],
          status: "done",
        },
      },
      checklist: {
        surfaces: [
          {
            evidence,
            id: SURFACE_ID,
            requirements: [...PHASE80_REQUIREMENTS],
            status: "done",
          },
        ],
      },
      surfaces: [{ name: SURFACE_ID, status: "done" }],
    },
    null,
    2,
  )}\n`;
}

function applyTextOptions(file: string, text: string, options: FixtureOptions): string {
  let result = text;
  if (options.maybeOmission?.file === file) {
    result = result.replace(options.maybeOmission.needle, "");
  }
  if (options.maybeAppend?.file === file) {
    result = `${result}${options.maybeAppend.text}`;
  }

  return result;
}

async function writeFixtureFile(root: string, relativePath: string, text: string): Promise<void> {
  const absolutePath = path.join(root, relativePath);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, text);
}

function runChecker(root: string): CheckerRun {
  const failures = checkPhase80OptInSoakUatReleaseBoundaries(root);

  return {
    exitCode: failures.length > 0 ? 1 : 0,
    stderr: failures.join("\n"),
  };
}

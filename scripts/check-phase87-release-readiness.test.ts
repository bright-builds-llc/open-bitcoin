#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase87ReleaseReadiness } from "./check-phase87-release-readiness";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE87_REPO_ROOT";
const SURFACE_ID = "v1-8-release-readiness-checklist";
const AUDIT_KEY = "v1_8_release_readiness_checklist";
const RELEASE_READINESS_PATH = "docs/parity/release-readiness.md";
const TABLE_HEADER =
  "Requirement | Phase | Canonical evidence | Default verification | UAT or manual evidence | Residual risk | No-claim or next gate";
const PHASE86_CHECKER_COMMAND =
  "bun run scripts/check-phase86-service-operation-expectations.ts";
const PHASE87_TEST_COMMAND = "bun test scripts/check-phase87-release-readiness.test.ts";
const PHASE87_CHECKER_COMMAND = "bun run scripts/check-phase87-release-readiness.ts";
const PHASE87_REQUIREMENTS = [
  "PROD-01",
  "PROD-02",
  "PROD-03",
  "PROD-04",
  "SUP-01",
  "SUP-02",
  "SUP-03",
  "SUP-04",
  "UPG-01",
  "UPG-02",
  "UPG-03",
  "UPG-04",
  "RUN-01",
  "RUN-02",
  "RUN-03",
  "SVC-01",
  "SVC-02",
  "REL-01",
  "REL-05",
  "REL-06",
] as const;
const TARGET_FILES = [
  RELEASE_READINESS_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE = [
  RELEASE_READINESS_PATH,
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/service-operation-expectations.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/check-phase87-release-readiness.ts",
  "scripts/check-phase87-release-readiness.test.ts",
  "scripts/verify.sh",
] as const;
const tempRoots: string[] = [];

type FixtureOptions = {
  maybeOmission?: {
    file: string;
    needle: string;
  };
  maybeParityIndexText?: string;
  maybePointerAppend?: {
    file: string;
    text: string;
  };
  maybeReleaseDocAppend?: string;
  maybeVerifyScript?: string;
};

afterEach(async () => {
  delete process.env[REPO_ROOT_OVERRIDE_ENV];

  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot === undefined) {
      continue;
    }

    await rm(maybeRoot, { force: true, recursive: true });
  }
});

test("passes_when_phase87_fixture_contains_release_readiness_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase87ReleaseReadiness();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_release_readiness_omits_requirement_or_no_claim", async () => {
  // Arrange
  const missingRequirementRoot = await createFixture({
    maybeOmission: { file: RELEASE_READINESS_PATH, needle: "REL-06" },
  });
  const missingNoClaimRoot = await createFixture({
    maybeOmission: { file: RELEASE_READINESS_PATH, needle: "production full-node readiness" },
  });

  // Act
  const missingRequirementFailures = checkPhase87ReleaseReadiness(missingRequirementRoot);
  const missingNoClaimFailures = checkPhase87ReleaseReadiness(missingNoClaimRoot);

  // Assert
  expect(missingRequirementFailures.join("\n")).toContain("release-readiness checklist");
  expect(missingNoClaimFailures.join("\n")).toContain("release-readiness no-claim review");
});

test("fails_when_parity_roots_omit_release_requirement_or_evidence", async () => {
  // Arrange
  const missingRequirementRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll('"REL-06"', '"REL-XX"'),
  });
  const missingEvidenceRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll(
      '"scripts/check-phase87-release-readiness.ts"',
      '"scripts/check-missing-release-readiness.ts"',
    ),
  });

  // Act
  const missingRequirementFailures = checkPhase87ReleaseReadiness(missingRequirementRoot);
  const missingEvidenceFailures = checkPhase87ReleaseReadiness(missingEvidenceRoot);

  // Assert
  expect(missingRequirementFailures.join("\n")).toContain("parity root");
  expect(missingEvidenceFailures.join("\n")).toContain("parity root");
});

test("fails_when_entrypoint_link_is_missing_or_duplicates_checklist_table", async () => {
  // Arrange
  const missingPointerRoot = await createFixture({
    maybeOmission: {
      file: "README.md",
      needle: "docs/parity/release-readiness.md#v18-release-readiness-checklist",
    },
  });
  const duplicateTableRoot = await createFixture({
    maybePointerAppend: { file: "docs/parity/README.md", text: `| ${TABLE_HEADER} |` },
  });

  // Act
  const missingPointerFailures = checkPhase87ReleaseReadiness(missingPointerRoot);
  const duplicateTableFailures = checkPhase87ReleaseReadiness(duplicateTableRoot);

  // Assert
  expect(missingPointerFailures.join("\n")).toContain("README.md");
  expect(duplicateTableFailures.join("\n")).toContain("must not duplicate required text");
});

test("fails_when_phase87_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE86_CHECKER_COMMAND,
      PHASE87_TEST_COMMAND,
      PHASE87_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      `run_step "check Phase 86 service operation expectations" bun run scripts/check-phase86-service-operation-expectations.ts`,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase87ReleaseReadiness(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});

test("fails_when_default_verify_contains_public_network_service_or_production_drift", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "run-live-mainnet-smoke",
      "systemctl status open-bitcoin",
      "launchctl print gui/501/open-bitcoin",
      "sleep 259200",
      "--restart-after-progress",
      "brew services",
      "Windows service",
      "automatic support-bundle upload",
      "broad production-node readiness",
    ].map((forbiddenText) =>
      createFixture({ maybeVerifyScript: `${verifyScriptText()}\n${forbiddenText}\n` }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase87ReleaseReadiness(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase87-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    [
      RELEASE_READINESS_PATH,
      options.maybeReleaseDocAppend
        ? `${releaseReadinessText()}\n${options.maybeReleaseDocAppend}\n`
        : releaseReadinessText(),
    ],
    ["docs/parity/index.json", options.maybeParityIndexText ?? parityIndexText()],
    ["scripts/verify.sh", options.maybeVerifyScript ?? verifyScriptText()],
  ]);

  for (const file of TARGET_FILES) {
    if (!files.has(file)) {
      files.set(file, humanPointerText(file));
    }
  }

  for (const [file, text] of files) {
    let nextText = text;
    if (options.maybeOmission?.file === file) {
      nextText = nextText.replaceAll(options.maybeOmission.needle, "");
    }
    if (options.maybePointerAppend?.file === file) {
      nextText = `${nextText}\n${options.maybePointerAppend.text}\n`;
    }
    await writeFixtureFile(root, file, nextText);
  }

  return root;
}

async function writeFixtureFile(root: string, file: string, contents: string): Promise<void> {
  const absolutePath = path.join(root, file);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, contents);
}

function releaseReadinessText(): string {
  const rows = PHASE87_REQUIREMENTS.map(
    (requirement) =>
      `| ${requirement} | Phase 87 release readiness | docs/parity/index.json, ${REQUIRED_EVIDENCE.join(", ")} | ${PHASE87_CHECKER_COMMAND}; bash scripts/verify.sh | reviewer evidence | residual risk | next gate |`,
  ).join("\n");

  return [
    `## v1.8 Release Readiness Checklist\n\nSurface id: \`${SURFACE_ID}\``,
    `| ${TABLE_HEADER} |`,
    "| --- | --- | --- | --- | --- | --- | --- |",
    rows,
    "Required deterministic reviewer commands:",
    "bun run scripts/check-phase82-production-claim-boundary.ts",
    "bun run scripts/check-phase83-support-matrix-issue-evidence.ts",
    "bun run scripts/check-phase84-upgrade-rollback-policy.ts",
    "bun run scripts/check-phase85-operator-runbooks.ts",
    PHASE86_CHECKER_COMMAND,
    PHASE87_TEST_COMMAND,
    PHASE87_CHECKER_COMMAND,
    "bash scripts/verify.sh",
    "## v1.8 Release Readiness No-Claim Review",
    "v1.8 is a boundary-setting milestone: it defines gates only and does not claim production full-node readiness. It does not claim production service operation, inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet use or safety, migration apply mode, signed packaging or package-manager distribution, Windows service integration, hosted dashboards, GUI parity, public-network default checks, public-network CI, release-blocking live sync, destructive repair, automatic support-bundle upload, or broad production-node readiness.",
    "Artifact existence, daemon startup, elapsed time, peer reachability, raw log tail, service file existence, and support bundle path are context only. Release reviewers must use named fields, unavailable reasons, canonical evidence roots, and deterministic checker output before accepting any scoped claim.",
    "Phase 88 owns REL-02, REL-03, and REL-04 broad deterministic claim guardrails.",
  ].join("\n\n");
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            status: "done",
            requirements: PHASE87_REQUIREMENTS,
            evidence: REQUIRED_EVIDENCE,
          },
        ],
      },
      audit: {
        [AUDIT_KEY]: {
          path: "release-readiness.md",
          status: "done",
          requirements: PHASE87_REQUIREMENTS,
          evidence: REQUIRED_EVIDENCE,
        },
      },
    },
    null,
    2,
  );
}

function humanPointerText(file: string): string {
  const maybeReadmeLink =
    file === "README.md"
      ? "docs/parity/release-readiness.md#v18-release-readiness-checklist"
      : "";
  const maybeParityReadmeLink =
    file === "docs/parity/README.md"
      ? "release-readiness.md#v18-release-readiness-checklist"
      : "";
  const maybeCatalogRow =
    file === "docs/parity/catalog/operator-runtime-release-hardening.md"
      ? `Phase 87 release-readiness checklist ${SURFACE_ID}`
      : "";
  return [
    `# ${file}`,
    `For release review, use the v1.8 release-readiness checklist in ${RELEASE_READINESS_PATH}.`,
    maybeReadmeLink,
    maybeParityReadmeLink,
    SURFACE_ID,
    PHASE87_REQUIREMENTS.join(" "),
    maybeCatalogRow,
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE86_CHECKER_COMMAND,
    PHASE87_TEST_COMMAND,
    PHASE87_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 86 service operation expectations" bun run scripts/check-phase86-service-operation-expectations.ts`,
    `run_step "test Phase 87 release readiness checker" bun test scripts/check-phase87-release-readiness.test.ts`,
    `run_step "check Phase 87 release readiness" bun run scripts/check-phase87-release-readiness.ts`,
  ].join("\n");
}

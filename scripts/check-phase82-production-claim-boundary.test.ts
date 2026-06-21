#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase82ProductionClaimBoundary } from "./check-phase82-production-claim-boundary";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE82_REPO_ROOT";
const SURFACE_ID = "v1-8-production-claim-boundary";
const PHASE82_REQUIREMENTS = ["PROD-01", "PROD-02", "PROD-03", "PROD-04"] as const;
const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;
const MATRIX_HEADER =
  "Statement | Support term | Current status | Evidence sources | Verification command | UAT status | Residual risk | Next required gate";
const PHASE80_CHECKER_COMMAND =
  "bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts";
const PHASE82_TEST_COMMAND =
  "bun test scripts/check-phase82-production-claim-boundary.test.ts";
const PHASE82_CHECKER_COMMAND = "bun run scripts/check-phase82-production-claim-boundary.ts";
const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
] as const;
const DEFERRED_SURFACES = [
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "production-funds wallet use",
  "production-funds wallet safety",
  "migration apply mode",
  "signed packaging or package-manager distribution",
  "Windows service integration",
  "hosted dashboards",
  "GUI parity",
  "public-network default checks",
  "public-network CI",
  "release-blocking live sync",
  "automatic support-bundle upload",
  "destructive repair",
  "broad production-node readiness",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "scripts/verify.sh",
] as const;

const tempRoots: string[] = [];

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
  maybeReplacement?: {
    file: string;
    needle: string;
    replacement: string;
  };
  maybeVerifyScript?: string;
};

afterEach(async () => {
  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot === undefined) {
      continue;
    }

    await rm(maybeRoot, { force: true, recursive: true });
  }
});

test("passes_when_phase82_fixture_contains_boundary_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_support_vocabulary_is_missing_or_forbidden_near_synonym_appears", async () => {
  // Arrange
  const root = await createFixture({
    maybeAppend: {
      file: "README.md",
      text: "\nThis is not production-grade support language.\n",
    },
    maybeOmission: {
      file: "docs/parity/production-claim-boundary.md",
      needle: "opt-in UAT",
    },
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("support vocabulary");
});

test("fails_when_deferred_inventory_omits_required_production_surface", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/parity/production-claim-boundary.md",
      needle: "automatic support-bundle upload",
    },
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("deferred inventory");
  expect(failures.join("\n")).toContain("automatic support-bundle upload");
});

test("fails_when_parity_roots_omit_requirement_or_canonical_evidence", async () => {
  // Arrange
  const root = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll("PROD-04", ""),
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("parity root");
});

test("fails_when_forbidden_claim_matrix_row_is_promoted", async () => {
  // Arrange
  const root = await createFixture({
    maybeReplacement: {
      file: "docs/parity/production-claim-boundary.md",
      needle:
        "| Open Bitcoin has production full-node readiness. | `deferred` | not allowed yet | evidence | No default verifier may prove this in v1.8 | none | risk | gate |",
      replacement:
        "| Open Bitcoin has production full-node readiness. | `supported` | allowed | evidence | bash scripts/verify.sh | none | risk | gate |",
    },
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("must remain deferred");
});

test("fails_when_targeted_claim_file_contains_exact_phase82_overclaim", async () => {
  // Arrange
  const root = await createFixture({
    maybeAppend: {
      file: "README.md",
      text:
        "\nOpen Bitcoin is production full-node ready.\n" +
        "v1.8 proves production full-node readiness.\n",
    },
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("exact overclaim");
});

test("fails_when_phase82_verify_commands_are_missing_or_before_phase80", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      PHASE82_TEST_COMMAND,
      PHASE82_CHECKER_COMMAND,
      PHASE80_CHECKER_COMMAND,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});

test("fails_when_phase82_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE80_CHECKER_COMMAND,
      PHASE82_TEST_COMMAND,
      PHASE82_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      PHASE80_CHECKER_COMMAND,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase82ProductionClaimBoundary(root);

  // Assert
  expect(failures.join("\n")).toContain("executed Phase 82 test and checker");
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase82-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    ["docs/parity/production-claim-boundary.md", productionBoundaryText()],
    ["docs/parity/release-readiness.md", releaseReadinessText()],
    ["docs/parity/deviations-and-unknowns.md", deviationsText()],
    ["docs/parity/index.json", options.maybeParityIndexText ?? parityIndexText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/README.md", parityReadmeText()],
    ["README.md", readmeText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/parity/catalog/operator-runtime-release-hardening.md", operatorCatalogText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/catalog/chainstate.md", chainstateCatalogText()],
    ["scripts/verify.sh", options.maybeVerifyScript ?? verifyScriptText()],
  ]);

  for (const [file, text] of files) {
    let nextText = text;
    if (options.maybeOmission?.file === file) {
      nextText = nextText.replaceAll(options.maybeOmission.needle, "");
    }
    if (options.maybeReplacement?.file === file) {
      nextText = nextText.replaceAll(
        options.maybeReplacement.needle,
        options.maybeReplacement.replacement,
      );
    }
    if (options.maybeAppend?.file === file) {
      nextText += options.maybeAppend.text;
    }
    await writeFixtureFile(root, file, nextText);
  }

  if (options.maybeManifestPath !== undefined) {
    await writeFixtureFile(root, options.maybeManifestPath, "{}\n");
  }

  return root;
}

async function writeFixtureFile(root: string, file: string, contents: string): Promise<void> {
  const absolutePath = path.join(root, file);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, contents);
}

function productionBoundaryText(): string {
  const notAllowed = [
    "Open Bitcoin has production full-node readiness.",
    "Open Bitcoin supports production service operation.",
    "Open Bitcoin supports relay/inbound serving.",
    "Open Bitcoin supports production wallet use.",
    "Open Bitcoin supports migration apply mode.",
    "Open Bitcoin supports signed distribution.",
    "Open Bitcoin supports hosted dashboards.",
    "Open Bitcoin supports public-network CI.",
    "Open Bitcoin supports destructive repair.",
    "Open Bitcoin supports automatic support upload.",
  ];

  return [
    `# Production Claim Boundary\nSurface id: \`${SURFACE_ID}\``,
    "v1.8 is a boundary-setting milestone, not the production readiness milestone.",
    "## Support Terms",
    "| Term | Definition |",
    "| --- | --- |",
    ...SUPPORT_TERMS.map((term) => `| \`${term}\` | definition |`),
    "## Claim-To-Evidence Matrix",
    `| ${MATRIX_HEADER} |`,
    "| --- | --- | --- | --- | --- | --- | --- | --- |",
    "| Open Bitcoin defines gates required before a future production full-node readiness claim. | `supported` | allowed | evidence | bash scripts/verify.sh | docs/parity verification only | risk | gate |",
    ...notAllowed.map(
      (statement) =>
        `| ${statement} | \`deferred\` | not allowed yet | evidence | No default verifier may prove this in v1.8 | none | risk | gate |`,
    ),
    "## Deferred Production-Adjacent Surfaces",
    DEFERRED_SURFACES.map((surface) => `| ${surface} | \`deferred\` | why | gate |`).join("\n"),
  ].join("\n");
}

function releaseReadinessText(): string {
  return [
    "## v1.8 Production Claim Boundary",
    "[production-claim-boundary.md](production-claim-boundary.md)",
    PHASE82_REQUIREMENTS.join(" "),
    SURFACE_ID,
    "Phase 88 owns broad deterministic claim guardrails",
    "## v1.7 Full-Sync Soak and Recovery Hardening Claim Boundary Matrix",
    "## v1.6 Full-Sync Completion Claim Boundary Matrix",
    "## v1.5 Unattended Operation Claim Boundary Matrix",
  ].join("\n");
}

function deviationsText(): string {
  return [
    "### v1.8 Production Claim Boundary",
    PHASE82_REQUIREMENTS.join(" "),
    "| Surface | Support term | Why deferred | Required future gate |",
    "| --- | --- | --- | --- |",
    DEFERRED_SURFACES.map((surface) => `| ${surface} | \`deferred\` | why | gate |`).join("\n"),
  ].join("\n");
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            title: "v1.8 Production Claim Boundary",
            status: "done",
            requirements: PHASE82_REQUIREMENTS,
            evidence: REQUIRED_EVIDENCE,
            known_gaps: ["v1.8 defines gates only"],
            suspected_unknowns: ["Phase 88 guardrails need a separate phase"],
          },
        ],
      },
      audit: {
        v1_8_production_claim_boundary: {
          path: "production-claim-boundary.md",
          status: "done",
          requirements: PHASE82_REQUIREMENTS,
          evidence: REQUIRED_EVIDENCE,
        },
      },
    },
    null,
    2,
  );
}

function checklistText(): string {
  return [`\`${SURFACE_ID}\``, PHASE82_REQUIREMENTS.join(" "), "production-claim-boundary.md"].join(
    "\n",
  );
}

function parityReadmeText(): string {
  return [
    "v1.8 production claim boundary",
    "production-claim-boundary.md",
    "v1.7 remains historical evidence",
  ].join("\n");
}

function readmeText(): string {
  return [
    "docs/parity/production-claim-boundary.md",
    "v1.8 defines the support terms and evidence gates required before a future production full-node readiness claim",
    "does not claim production full-node readiness",
    "v1.7 remains historical source-built, explicit opt-in full-sync soak and recovery hardening evidence",
    DEFERRED_SURFACES.join("\n"),
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "v1.8 production claim boundary",
    "../parity/production-claim-boundary.md",
    SUPPORT_TERMS.join(" "),
    "not a production full-node readiness claim",
    "### Phase 80 v1.7 opt-in soak UAT matrix",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  ].join("\n");
}

function operatorCatalogText(): string {
  return [
    "Phase 82 production claim boundary",
    SURFACE_ID,
    PHASE82_REQUIREMENTS.join(" "),
    "automatic support-bundle upload",
    "destructive repair",
    "broad production-node readiness",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "v1.8 production claim boundary",
    "production-claim-boundary.md",
    "inbound serving",
    "address relay",
    "block serving",
    "transaction relay",
    "compact block relay",
  ].join("\n");
}

function chainstateCatalogText(): string {
  return [
    "v1.8 production claim boundary",
    "production-claim-boundary.md",
    "destructive repair",
    "public-network CI",
    "release-blocking live sync",
    "broad production-node readiness",
  ].join("\n");
}

function verifyScriptText(): string {
  return [PHASE80_CHECKER_COMMAND, PHASE82_TEST_COMMAND, PHASE82_CHECKER_COMMAND].join("\n");
}

#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase83SupportMatrixIssueEvidence } from "./check-phase83-support-matrix-issue-evidence";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE83_REPO_ROOT";
const SURFACE_ID = "v1-8-support-matrix-issue-evidence";
const PHASE83_REQUIREMENTS = ["SUP-01", "SUP-02", "SUP-03", "SUP-04"] as const;
const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;
const FORBIDDEN_MATURITY_LABELS = [
  "best-effort",
  "beta",
  "partial production",
  "community-supported",
] as const;
const MATRIX_COLUMNS = [
  "Environment family",
  "Support term",
  "Evidence basis",
  "Default verification",
  "Opt-in UAT / manual validation",
  "Residual risk",
  "Next gate",
] as const;
const REQUIRED_ENVIRONMENT_FAMILIES = [
  "source-built install and repo verification",
  "repo-local operator command forms through Cargo and Bazel",
  "local deterministic runtime, status, config, RPC, and support-bundle surfaces",
  "operator dashboard and shipped operator convenience surfaces",
  "public-network mainnet activation, full-sync, stay-current, and soak evidence",
  "storage/datadir resource-bound evidence and recovery diagnosis",
  "live storage pressure and long-run resource behavior",
  "launchd/systemd service-supervision previews",
  "real launchd/systemd service-manager lifecycle",
  "migration dry-run",
  "migration apply, source service mutation, and source datadir rewrite",
  "support bundle and support forensics",
  "wallet current non-production slice",
  "production-funds wallet use and safety",
  "inbound serving",
  "address relay",
  "block serving",
  "transaction relay",
  "compact block relay",
  "signed packaging or package-manager distribution",
  "Windows service integration",
  "hosted dashboards and GUI parity",
  "automatic support-bundle upload",
  "destructive repair",
  "public-network default checks, public-network CI, and release-blocking live sync",
  "broad production-node readiness",
] as const;
const RESIDUAL_RISK_SURFACES = [
  "dashboard pseudoterminal/raw-input repaint and input behavior",
  "closeout without a dedicated milestone audit artifact",
  "diagnosed-blocker closeout and fresh status supersession",
  "planning traceability correction during archive prep",
  "public-network full-sync, stay-current, and soak evidence",
  "real service-manager lifecycle evidence",
  "multi-day wall-clock soak evidence",
  "support-bundle forensics",
  "recovery diagnosis versus destructive repair",
  "production-scope non-claims",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/support-matrix.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "scripts/verify.sh",
] as const;
const PHASE82_TEST_COMMAND =
  "bun test scripts/check-phase82-production-claim-boundary.test.ts";
const PHASE82_CHECKER_COMMAND = "bun run scripts/check-phase82-production-claim-boundary.ts";
const PHASE83_TEST_COMMAND =
  "bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts";
const PHASE83_CHECKER_COMMAND =
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts";

const tempRoots: string[] = [];

type FixtureOptions = {
  maybeAppend?: {
    file: string;
    text: string;
  };
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
  delete process.env[REPO_ROOT_OVERRIDE_ENV];

  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot === undefined) {
      continue;
    }

    await rm(maybeRoot, { force: true, recursive: true });
  }
});

test("passes_when_phase83_fixture_contains_support_matrix_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase83SupportMatrixIssueEvidence();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_support_matrix_uses_non_phase82_support_term", async () => {
  // Arrange
  const roots = await Promise.all(
    FORBIDDEN_MATURITY_LABELS.map((label) =>
      createFixture({
        maybeReplacement: {
          file: "docs/parity/support-matrix.md",
          needle: "| source-built install and repo verification | `supported` |",
          replacement: `| source-built install and repo verification | \`${label}\` |`,
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase83SupportMatrixIssueEvidence(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("unsupported support term");
  }
});

test("fails_when_support_matrix_required_column_is_missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/parity/support-matrix.md",
      needle: "Opt-in UAT / manual validation",
    },
  });

  // Act
  const failures = checkPhase83SupportMatrixIssueEvidence(root);

  // Assert
  expect(failures.join("\n")).toContain("support matrix columns");
});

test("fails_when_required_environment_family_is_missing", async () => {
  // Arrange
  const missingFamilies = [
    "automatic support-bundle upload",
    "inbound serving",
    "Windows service integration",
    "broad production-node readiness",
  ] as const;
  const roots = await Promise.all(
    missingFamilies.map((family) =>
      createFixture({
        maybeOmission: {
          file: "docs/parity/support-matrix.md",
          needle: family,
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase83SupportMatrixIssueEvidence(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("missing environment family");
  }
});

test("fails_when_required_environment_family_is_only_a_substring_match", async () => {
  // Arrange
  const root = await createFixture({
    maybeReplacement: {
      file: "docs/parity/support-matrix.md",
      needle: "| inbound serving | `deferred` |",
      replacement: "| not inbound serving | `deferred` |",
    },
  });

  // Act
  const failures = checkPhase83SupportMatrixIssueEvidence(root);

  // Assert
  expect(failures.join("\n")).toContain("missing environment family in support matrix");
});

test("fails_when_support_matrix_uses_placeholder_evidence_cells", async () => {
  // Arrange
  const root = await createFixture({
    maybeReplacement: {
      file: "docs/parity/support-matrix.md",
      needle: "docs/parity/support-matrix.md and bash scripts/verify.sh",
      replacement: "evidence basis",
    },
  });

  // Act
  const failures = checkPhase83SupportMatrixIssueEvidence(root);

  // Assert
  expect(failures.join("\n")).toContain("placeholder cell");
});

test("fails_when_issue_evidence_redaction_boundary_drifts", async () => {
  // Arrange
  const missingUnavailableRoot = await createFixture({
    maybeOmission: {
      file: "docs/parity/support-matrix.md",
      needle: "Unavailable: <reason>",
    },
  });
  const requestedSecretRoot = await createFixture({
    maybeReplacement: {
      file: "docs/parity/support-matrix.md",
      needle: "- The exact repo-local command that reproduced the issue.",
      replacement:
        "- The exact repo-local command that reproduced the issue.\n" +
        "- rpcpassword from the reporter's local config.",
    },
  });

  // Act
  const missingUnavailableFailures =
    checkPhase83SupportMatrixIssueEvidence(missingUnavailableRoot);
  const requestedSecretFailures =
    checkPhase83SupportMatrixIssueEvidence(requestedSecretRoot);

  // Assert
  expect(missingUnavailableFailures.join("\n")).toContain("issue evidence");
  expect(requestedSecretFailures.join("\n")).toContain("issue evidence");
});

test("fails_when_residual_risk_row_is_missing", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/parity/support-matrix.md",
      needle: "dashboard pseudoterminal/raw-input repaint and input behavior",
    },
  });

  // Act
  const failures = checkPhase83SupportMatrixIssueEvidence(root);

  // Assert
  expect(failures.join("\n")).toContain("residual risk");
});

test("fails_when_parity_roots_omit_requirement_or_support_matrix_evidence", async () => {
  // Arrange
  const missingRequirementRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll('"SUP-04"', '"SUP-XX"'),
  });
  const missingEvidenceRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll("docs/parity/support-matrix.md", ""),
  });

  // Act
  const missingRequirementFailures =
    checkPhase83SupportMatrixIssueEvidence(missingRequirementRoot);
  const missingEvidenceFailures = checkPhase83SupportMatrixIssueEvidence(missingEvidenceRoot);

  // Assert
  expect(missingRequirementFailures.join("\n")).toContain("parity root");
  expect(missingEvidenceFailures.join("\n")).toContain("parity root");
});

test("fails_when_phase83_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE82_TEST_COMMAND,
      PHASE82_CHECKER_COMMAND,
      PHASE83_TEST_COMMAND,
      PHASE83_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      `run_step "test Phase 82 production claim boundary checker" bun test scripts/check-phase82-production-claim-boundary.test.ts`,
      `run_step "check Phase 82 production claim boundary" bun run scripts/check-phase82-production-claim-boundary.ts`,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase83SupportMatrixIssueEvidence(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase83-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    ["README.md", readmeText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/parity/support-matrix.md", supportMatrixText()],
    ["docs/parity/README.md", parityReadmeText()],
    ["docs/parity/release-readiness.md", releaseReadinessText()],
    ["docs/parity/deviations-and-unknowns.md", deviationsText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/index.json", options.maybeParityIndexText ?? parityIndexText()],
    ["docs/parity/catalog/operator-runtime-release-hardening.md", operatorCatalogText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/catalog/chainstate.md", chainstateCatalogText()],
    ["docs/parity/catalog/wallet.md", walletCatalogText()],
    ["docs/parity/catalog/drop-in-audit-and-migration.md", migrationCatalogText()],
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

  return root;
}

async function writeFixtureFile(root: string, file: string, contents: string): Promise<void> {
  const absolutePath = path.join(root, file);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, contents);
}

function supportMatrixText(): string {
  return [
    `# Support Matrix And Issue Evidence\nSurface id: \`${SURFACE_ID}\``,
    "Phase 83 keeps support matrix edits narrow while broad all-doc production-claim scanning remains Phase 88 scope.",
    "## Support Terms",
    SUPPORT_TERMS.map((term) => `- \`${term}\``).join("\n"),
    "## Support Matrix",
    `| ${MATRIX_COLUMNS.join(" | ")} |`,
    "| --- | --- | --- | --- | --- | --- | --- |",
    REQUIRED_ENVIRONMENT_FAMILIES.map((family) => supportMatrixRow(family)).join("\n"),
    "## Issue Evidence Checklist",
    "Issue reports should include the smallest useful redacted evidence set.",
    "For every missing field, write `Unavailable: <reason>` so reviewers can distinguish absence from passing evidence.",
    "- Redacted support bundle files `support-evidence.json` and `support-evidence.md` when available.",
    "- Relevant command output, copied from the command that reproduced the issue.",
    "- Bounded redacted logs, log paths, or compact log summaries.",
    "- A configuration summary for the selected datadir.",
    "- Service state, including whether no service manager was involved.",
    "- resource-bound or resource-pressure evidence from status, support evidence, or the affected run report.",
    "- recovery/progress evidence, including recovery category/action, progress credit, stall diagnosis, or no-progress reason when applicable.",
    "- sync status evidence, including header, downloaded block, connected block, best-known tip, stay-current, and latest stop reason when applicable.",
    "- version, commit, Rust, Cargo, Bun, and Bazel context from the checkout and active toolchain.",
    "- Platform details: OS, CPU architecture, filesystem, shell, terminal when UI behavior matters, and whether the run used Cargo or Bazel.",
    "- The exact repo-local command that reproduced the issue.",
    "```bash\ncargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support\n```",
    "```bash\nbazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support\n```",
    "A support bundle path is not sufficient by itself.",
    "### Do Not Attach",
    "- wallet private material",
    "- raw wallet files",
    "- RPC cookies",
    "- rpcpassword",
    "- rpcauth",
    "- raw datadirs",
    "- unredacted logs",
    "- raw unbounded logs",
    "- full peer tables with sensitive local data",
    "- automatic support-bundle upload",
    "## Contributor Update Rules",
    "Deferred surfaces cannot be promoted by prose-only edits.",
    "docs/parity/production-claim-boundary.md",
    "docs/parity/release-readiness.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/operator/runtime-guide.md",
    "scripts/verify.sh",
    "## Carried-Forward Residual Risks And Manual Validation",
    "| Milestone | Surface | Handling status | Latest evidence source | Current support effect | Next gate |",
    "| --- | --- | --- | --- | --- | --- |",
    RESIDUAL_RISK_SURFACES.map(
      (surface) =>
        `| v1.8 | ${surface} | verified deterministic behavior | docs/parity/support-matrix.md | scoped support effect | future gate |`,
    ).join("\n"),
  ].join("\n\n");
}

function supportMatrixRow(family: string): string {
  let term = "`supported`";
  if (
    family.includes("public-network") ||
    family.includes("long-run") ||
    family.includes("real launchd")
  ) {
    term = "`opt-in UAT`";
  } else if (
    family.includes("dashboard") ||
    family.includes("service-supervision") ||
    family.includes("wallet current")
  ) {
    term = "`preview`";
  } else if (
    family.includes("apply") ||
    family.includes("production-funds") ||
    family.includes("inbound") ||
    family.includes("relay") ||
    family.includes("Windows") ||
    family.includes("hosted") ||
    family.includes("upload") ||
    family.includes("repair") ||
    family.includes("broad production")
  ) {
    term = "`deferred`";
  }

  return `| ${[
    family,
    term,
    "docs/parity/support-matrix.md and bash scripts/verify.sh",
    "covered by deterministic repo verifier or explicitly outside default verification",
    "operator-owned manual validation when the support term requires it",
    "tracked in the support matrix and release-readiness residual-risk sections",
    "future milestone or release-policy decision before promotion",
  ].join(" | ")} |`;
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            title: "v1.8 Support Matrix And Issue Evidence",
            status: "done",
            requirements: PHASE83_REQUIREMENTS,
            evidence: REQUIRED_EVIDENCE,
          },
        ],
      },
      audit: {
        v1_8_support_matrix_issue_evidence: {
          path: "support-matrix.md",
          status: "done",
          requirements: PHASE83_REQUIREMENTS,
          evidence: REQUIRED_EVIDENCE,
        },
      },
    },
    null,
    2,
  );
}

function checklistText(): string {
  return [`\`${SURFACE_ID}\``, PHASE83_REQUIREMENTS.join(" "), "support-matrix.md"].join("\n");
}

function readmeText(): string {
  return [
    "docs/parity/support-matrix.md",
    "v1.8 defines support terms and evidence gates without claiming production full-node readiness.",
    "inbound serving",
    "address relay",
    "block serving",
    "transaction relay",
    "compact block relay",
    "Windows service integration",
    "broad production-node readiness",
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "../parity/support-matrix.md",
    "smallest useful redacted evidence set",
    "Unavailable: <reason>",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  ].join("\n");
}

function parityReadmeText(): string {
  return [
    SURFACE_ID,
    PHASE83_REQUIREMENTS.join(" "),
    "support-matrix.md",
    "single source for support levels",
  ].join("\n");
}

function releaseReadinessText(): string {
  return [
    "## v1.8 Support Matrix And Issue Evidence",
    "docs/parity/support-matrix.md",
    SURFACE_ID,
    PHASE83_REQUIREMENTS.join(" "),
  ].join("\n");
}

function deviationsText(): string {
  return [
    "support-matrix.md",
    SURFACE_ID,
    "residual risks and support-matrix update boundaries",
  ].join("\n");
}

function operatorCatalogText(): string {
  return [
    SURFACE_ID,
    PHASE83_REQUIREMENTS.join(" "),
    "docs/parity/support-matrix.md",
    "production service operation remains deferred",
    "public-network CI remains deferred",
    "support upload remains deferred",
    "destructive repair remains deferred",
    "broad production-node readiness remains deferred",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "docs/parity/support-matrix.md",
    "inbound serving",
    "address relay",
    "block serving",
    "transaction relay",
    "compact block relay",
  ].join("\n");
}

function chainstateCatalogText(): string {
  return ["docs/parity/support-matrix.md", "destructive repair"].join("\n");
}

function walletCatalogText(): string {
  return ["docs/parity/support-matrix.md", "production-funds wallet"].join("\n");
}

function migrationCatalogText(): string {
  return ["docs/parity/support-matrix.md", "migration apply"].join("\n");
}

function verifyScriptText(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE82_TEST_COMMAND,
    PHASE82_CHECKER_COMMAND,
    PHASE83_TEST_COMMAND,
    PHASE83_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 82 production claim boundary checker" bun test scripts/check-phase82-production-claim-boundary.test.ts`,
    `run_step "check Phase 82 production claim boundary" bun run scripts/check-phase82-production-claim-boundary.ts`,
    `run_step "test Phase 83 support matrix issue evidence checker" bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts`,
    `run_step "check Phase 83 support matrix issue evidence" bun run scripts/check-phase83-support-matrix-issue-evidence.ts`,
  ].join("\n");
}

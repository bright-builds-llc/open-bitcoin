#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase84UpgradeRollbackPolicy } from "./check-phase84-upgrade-rollback-policy";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE84_REPO_ROOT";
const SURFACE_ID = "v1-8-upgrade-rollback-policy";
const PHASE84_REQUIREMENTS = ["UPG-01", "UPG-02", "UPG-03", "UPG-04"] as const;
const POLICY_PATH = "docs/parity/upgrade-and-rollback-policy.md";
const TABLE_HEADER = "Evidence to record | How to collect it | Mutation status | Why it matters";
const PHASE83_CHECKER_COMMAND =
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts";
const PHASE84_TEST_COMMAND =
  "bun test scripts/check-phase84-upgrade-rollback-policy.test.ts";
const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";
const PRE_UPGRADE_ITEMS = [
  "current source revision or commit",
  "repo-local verification status",
  "binary provenance from Cargo or Bazel",
  "Open Bitcoin JSONC config path",
  "bitcoin.conf path",
  "selected datadir",
  "datadir ownership and free-space review",
  "current sync/status evidence",
  "support-bundle evidence when available",
  "service state",
  "wallet scope",
  "backup location",
] as const;
const RECOVERY_LABELS_AND_ACTIONS = [
  "clean_shutdown",
  "unclean_shutdown",
  "incompatible_schema",
  "store_corruption",
  "storage_lock_contention",
  "schema_mismatch",
  "corruption_marker",
  "corrupt_record",
  "partial_write",
  "unreadable_namespace",
  "backend_open_failure",
  "safe_retry",
  "read_only_inspection",
  "backup_then_rebuild",
  "stop_and_escalate",
] as const;
const INSUFFICIENT_PROOF_SIGNALS = [
  "daemon startup",
  "elapsed time",
  "peer reachability",
  "raw logs",
  "report existence",
] as const;
const TARGET_FILES = [
  POLICY_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "docs/parity/catalog/drop-in-audit-and-migration.md",
  "docs/parity/catalog/wallet.md",
  "docs/parity/catalog/chainstate.md",
] as const;

const tempRoots: string[] = [];

type FixtureOptions = {
  maybeOmission?: {
    file: string;
    needle: string;
  };
  maybeParityIndexText?: string;
  maybePolicyAppend?: string;
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

test("passes_when_phase84_fixture_contains_policy_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase84UpgradeRollbackPolicy();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_pre_upgrade_checklist_omits_required_evidence", async () => {
  // Arrange
  const roots = await Promise.all(
    PRE_UPGRADE_ITEMS.map((item) =>
      createFixture({
        maybeOmission: {
          file: POLICY_PATH,
          needle: item,
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase84UpgradeRollbackPolicy(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("pre-upgrade checklist");
  }
});

test("fails_when_compatibility_decision_table_missing_recovery_label", async () => {
  // Arrange
  const roots = await Promise.all(
    RECOVERY_LABELS_AND_ACTIONS.map((label) =>
      createFixture({
        maybeOmission: {
          file: POLICY_PATH,
          needle: label,
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase84UpgradeRollbackPolicy(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("compatibility decision table");
  }
});

test("fails_when_policy_claims_startup_or_report_existence_as_proof", async () => {
  // Arrange
  const roots = await Promise.all(
    INSUFFICIENT_PROOF_SIGNALS.map((signal) =>
      createFixture({
        maybeReplacement: {
          file: POLICY_PATH,
          needle:
            "Compatibility decisions require field-level evidence and `Unavailable: <reason>` for missing fields.",
          replacement: `Compatibility decisions may treat ${signal} as proof.`,
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase84UpgradeRollbackPolicy(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("insufficient compatibility proof");
  }
});

test("fails_when_policy_allows_hidden_mutation_or_destructive_repair", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "The upgrade may silently rewrite source datadirs.",
      "The upgrade may mutate external wallets.",
      "The upgrade may rewrite launchd/systemd service files.",
      "The upgrade may edit bitcoin.conf and Open Bitcoin JSONC config.",
      "Destructive repair is allowed during rollback.",
    ].map((forbiddenText) => createFixture({ maybePolicyAppend: forbiddenText })),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase84UpgradeRollbackPolicy(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("hidden mutation boundary");
  }
});

test("fails_when_parity_roots_omit_upgrade_policy_requirement_or_evidence", async () => {
  // Arrange
  const missingRequirementRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll('"UPG-04"', '"UPG-XX"'),
  });
  const missingEvidenceRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll(
      `"${POLICY_PATH}"`,
      '"docs/parity/missing-upgrade-policy.md"',
    ),
  });

  // Act
  const missingRequirementFailures =
    checkPhase84UpgradeRollbackPolicy(missingRequirementRoot);
  const missingEvidenceFailures = checkPhase84UpgradeRollbackPolicy(missingEvidenceRoot);

  // Assert
  expect(missingRequirementFailures.join("\n")).toContain("parity root");
  expect(missingEvidenceFailures.join("\n")).toContain("parity root");
});

test("fails_when_phase84_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE83_CHECKER_COMMAND,
      PHASE84_TEST_COMMAND,
      PHASE84_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      `run_step "check Phase 83 support matrix issue evidence" bun run scripts/check-phase83-support-matrix-issue-evidence.ts`,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase84UpgradeRollbackPolicy(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});

test("fails_when_default_verify_contains_public_network_service_manager_or_multiday_drift", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "run-live-mainnet-smoke",
      "systemctl status open-bitcoin",
      "launchctl print gui/501/open-bitcoin",
      "sleep 259200",
    ].map((forbiddenText) =>
      createFixture({
        maybeVerifyScript: `${verifyScriptText()}\n${forbiddenText}\n`,
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase84UpgradeRollbackPolicy(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase84-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    [POLICY_PATH, options.maybePolicyAppend ? `${policyText()}\n${options.maybePolicyAppend}\n` : policyText()],
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
    if (options.maybeReplacement?.file === file) {
      nextText = nextText.replaceAll(
        options.maybeReplacement.needle,
        options.maybeReplacement.replacement,
      );
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

function policyText(): string {
  return [
    `# Upgrade And Rollback Policy\n\nSurface id: \`${SURFACE_ID}\``,
    "## Scope And Non-Claims",
    "Use this policy with the Phase 82 support terms exactly: `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`.",
    "It does not claim production full-node readiness or create a second support matrix.",
    "## Pre-Upgrade Checklist",
    "Record this evidence before changing binaries or runtime state.",
    `| ${TABLE_HEADER} |`,
    "| --- | --- | --- | --- |",
    PRE_UPGRADE_ITEMS.map(
      (item) =>
        `| ${item} | collect ${item} with repo-local evidence | review-only evidence | keeps ${item} auditable |`,
    ).join("\n"),
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
    "Status, support bundle, config summary, service state, source revision, and backup-location recording are review-only evidence.",
    "Source datadir, wallet, service, and config mutation requires a future scoped plan before any operator workflow may recommend or automate it.",
    "## State And Schema Compatibility Decision Table",
    "Compatibility decisions require field-level evidence and `Unavailable: <reason>` for missing fields.",
    "| Evidence observed | Compatibility category | Action class | Allowed next action | Forbidden hidden mutation | Required evidence |",
    "| --- | --- | --- | --- | --- | --- |",
    compatibilityRows(),
    "## Evidence That Is Not Sufficient",
    "The following signals are useful context but are not compatibility proof by themselves:",
    "- daemon startup",
    "- elapsed time",
    "- peer reachability",
    "- raw logs",
    "- report existence alone",
    "Compatibility decisions require field-level evidence and `Unavailable: <reason>` for missing fields.",
    "## Open Bitcoin Store Versus External State",
    "Open Bitcoin-owned durable store state is the selected Open Bitcoin datadir and its typed status/support evidence.",
    "external Core/Knots source datadirs and wallets are high-value input and must not be rewritten.",
    "## Failed Upgrade Guidance",
    "stop the attempted upgraded process",
    "record exact command and commit",
    "collect redacted local evidence",
    "preserve backups",
    "avoid repeated mutation until the compatibility class is understood",
    "## Rollback Guidance",
    "return to the previous checked-out source revision or known binary",
    "use the same explicit datadir and config paths",
    "verify with repo-local commands",
    "record rollback evidence",
    "This policy does not imply package-manager rollback, signed release channels, or automatic update behavior.",
    "## Boundary And Deferred Work",
    "Phase 84 does not recommend hidden mutation of source datadirs, external wallets, service files, launchd/systemd state, bitcoin.conf, or Open Bitcoin JSONC config.",
    "Destructive repair remains deferred.",
    "backup_then_rebuild is evidence and operator-decision guidance, not permission for automated destructive rebuild or repair.",
  ].join("\n\n");
}

function compatibilityRows(): string {
  return [
    ["clean_shutdown", "clean selected store state", "safe_retry"],
    ["unclean_shutdown", "interrupted selected store state", "safe_retry"],
    ["storage_lock_contention", "selected store lock contention", "read_only_inspection"],
    ["incompatible_schema", "selected store schema is incompatible", "stop_and_escalate"],
    ["schema_mismatch", "schema mismatch cause", "stop_and_escalate"],
    ["store_corruption", "selected store corruption", "backup_then_rebuild"],
    ["corruption_marker", "corruption marker cause", "backup_then_rebuild"],
    ["corrupt_record", "corrupt record cause", "backup_then_rebuild"],
    ["partial_write", "partial write cause", "backup_then_rebuild"],
    ["unreadable_namespace", "unreadable namespace cause", "backup_then_rebuild"],
    ["backend_open_failure", "backend-open failure pending classification", "read_only_inspection"],
  ]
    .map(
      ([evidence, category, action]) =>
        `| \`${evidence}\` | ${category} | \`${action}\` | preserve evidence before action | no hidden mutation | field-level evidence |`,
    )
    .join("\n");
}

function parityIndexText(): string {
  const evidence = [
    POLICY_PATH,
    "docs/parity/production-claim-boundary.md",
    "docs/parity/support-matrix.md",
    "docs/parity/release-readiness.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/checklist.md",
    "docs/parity/README.md",
    "README.md",
    "docs/operator/runtime-guide.md",
    "docs/parity/catalog/operator-runtime-release-hardening.md",
    "docs/parity/catalog/drop-in-audit-and-migration.md",
    "docs/parity/catalog/wallet.md",
    "docs/parity/catalog/chainstate.md",
    "scripts/verify.sh",
  ];

  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            status: "done",
            requirements: PHASE84_REQUIREMENTS,
            evidence,
          },
        ],
      },
      audit: {
        v1_8_upgrade_rollback_policy: {
          path: "upgrade-and-rollback-policy.md",
          status: "done",
          requirements: PHASE84_REQUIREMENTS,
          evidence,
        },
      },
    },
    null,
    2,
  );
}

function humanPointerText(file: string): string {
  return [
    `# ${file}`,
    `\`${SURFACE_ID}\``,
    PHASE84_REQUIREMENTS.join(" "),
    "upgrade-and-rollback-policy.md",
    "source-built upgrade, rollback, backup, and compatibility policy",
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE83_CHECKER_COMMAND,
    PHASE84_TEST_COMMAND,
    PHASE84_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 83 support matrix issue evidence" bun run scripts/check-phase83-support-matrix-issue-evidence.ts`,
    `run_step "test Phase 84 upgrade rollback policy checker" bun test scripts/check-phase84-upgrade-rollback-policy.test.ts`,
    `run_step "check Phase 84 upgrade rollback policy" bun run scripts/check-phase84-upgrade-rollback-policy.ts`,
  ].join("\n");
}

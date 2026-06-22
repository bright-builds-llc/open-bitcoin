#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase85OperatorRunbooks } from "./check-phase85-operator-runbooks";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE85_REPO_ROOT";
const SURFACE_ID = "v1-8-operator-runbooks";
const PHASE85_REQUIREMENTS = ["RUN-01", "RUN-02", "RUN-03"] as const;
const RUNBOOK_PATH = "docs/parity/operator-runbooks.md";
const TABLE_HEADER = "Evidence to record | How to collect it | Mutation status | Escalation use";
const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";
const PHASE85_TEST_COMMAND =
  "bun test scripts/check-phase85-operator-runbooks.test.ts";
const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
const TARGET_FILES = [
  RUNBOOK_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const PREFLIGHT_ITEMS = [
  "selected datadir",
  "source revision",
  "repo-local verification status",
  "Cargo or Bazel command form",
  "config paths",
  "current status evidence",
  "resource/disk review",
  "service state or unavailable reason",
  "wallet scope",
  "support-bundle availability",
] as const;
const STRUCTURED_MONITORING_TERMS = [
  "structured logs",
  "metrics",
  "support-bundle summaries",
  "soak reports",
  "live-smoke reports",
  "checkpoint timeline",
  "stalled subsystem",
  "public-network opt-in",
  "stay-current opt-in",
  "multi-day soak opt-in",
] as const;
const INSUFFICIENT_PROOF_SIGNALS = [
  "artifact existence",
  "elapsed time",
  "daemon startup",
  "peer reachability",
  "raw logs",
  "report existence",
] as const;
const TIMELINE_AND_PRIVACY_TERMS = [
  "preflight evidence",
  "command start",
  "status snapshots",
  "progress or no-progress events",
  "resource/recovery events",
  "support-bundle collection",
  "operator action taken",
  "final status",
  "escalation decision",
  "wallet private material",
  "raw wallet files",
  "RPC cookies",
  "rpcpassword",
  "rpcauth",
  "raw datadirs",
  "unredacted logs",
  "raw unbounded logs",
  "full peer tables with sensitive local data",
  "automatic support-bundle upload",
] as const;

const tempRoots: string[] = [];

type FixtureOptions = {
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
  maybeRunbookAppend?: string;
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

test("passes_when_phase85_fixture_contains_runbook_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase85OperatorRunbooks();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_runbook_omits_preflight_evidence", async () => {
  // Arrange
  const roots = await Promise.all(
    PREFLIGHT_ITEMS.map((item) =>
      createFixture({ maybeOmission: { file: RUNBOOK_PATH, needle: item } }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase85OperatorRunbooks(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("preflight");
  }
});

test("fails_when_runbook_omits_structured_monitoring_or_opt_in_evidence", async () => {
  // Arrange
  const roots = await Promise.all(
    STRUCTURED_MONITORING_TERMS.map((term) =>
      createFixture({ maybeOmission: { file: RUNBOOK_PATH, needle: term } }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase85OperatorRunbooks(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("monitoring");
  }
});

test("fails_when_runbook_treats_artifact_existence_or_elapsed_time_as_proof", async () => {
  // Arrange
  const roots = await Promise.all(
    INSUFFICIENT_PROOF_SIGNALS.map((signal) =>
      createFixture({ maybeRunbookAppend: `The runbook treats ${signal} as proof.` }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase85OperatorRunbooks(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("insufficient proof");
  }
});

test("fails_when_runbook_allows_hidden_mutation_or_automatic_upload", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "The runbook says destructive repair is allowed.",
      "The runbook says source datadir mutation is allowed.",
      "The runbook says service-manager mutation is allowed.",
      "The runbook says config rewrite is allowed.",
      "The runbook says automatic rebuild is allowed.",
      "The runbook says automatic support-bundle upload is supported.",
    ].map((forbiddenText) => createFixture({ maybeRunbookAppend: forbiddenText })),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase85OperatorRunbooks(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("mutation boundary");
  }
});

test("fails_when_support_timeline_omits_required_field_or_redaction_boundary", async () => {
  // Arrange
  const roots = await Promise.all(
    TIMELINE_AND_PRIVACY_TERMS.map((term) =>
      createFixture({ maybeOmission: { file: RUNBOOK_PATH, needle: term } }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase85OperatorRunbooks(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("support-bundle");
  }
});

test("fails_when_parity_roots_omit_runbook_requirement_or_evidence", async () => {
  // Arrange
  const missingRequirementRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll('"RUN-03"', '"RUN-XX"'),
  });
  const missingEvidenceRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll(
      `"${RUNBOOK_PATH}"`,
      '"docs/parity/missing-operator-runbooks.md"',
    ),
  });

  // Act
  const missingRequirementFailures =
    checkPhase85OperatorRunbooks(missingRequirementRoot);
  const missingEvidenceFailures = checkPhase85OperatorRunbooks(missingEvidenceRoot);

  // Assert
  expect(missingRequirementFailures.join("\n")).toContain("parity root");
  expect(missingEvidenceFailures.join("\n")).toContain("parity root");
});

test("fails_when_phase85_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE84_CHECKER_COMMAND,
      PHASE85_TEST_COMMAND,
      PHASE85_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      `run_step "check Phase 84 upgrade rollback policy" bun run scripts/check-phase84-upgrade-rollback-policy.ts`,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase85OperatorRunbooks(root);

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
      createFixture({ maybeVerifyScript: `${verifyScriptText()}\n${forbiddenText}\n` }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase85OperatorRunbooks(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase85-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    [
      RUNBOOK_PATH,
      options.maybeRunbookAppend
        ? `${runbookText()}\n${options.maybeRunbookAppend}\n`
        : runbookText(),
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

function runbookText(): string {
  return [
    `# Operator Runbooks\n\nSurface id: \`${SURFACE_ID}\``,
    "## Scope And Non-Claims",
    "Use exactly `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`.",
    "This runbook forbids destructive repair, source datadir mutation, external wallet mutation, service-manager mutation, config rewrite, automatic rebuild, response timelines, hosted support upload, production service ownership, and automatic support-bundle upload.",
    "Default bash scripts/verify.sh remains deterministic, public-network-free, service-manager-free, and multi-day-free.",
    "## Production-Boundary Preflight",
    "production-claim-boundary.md support-matrix.md upgrade-and-rollback-policy.md",
    `| ${TABLE_HEADER} |`,
    "| --- | --- | --- | --- |",
    PREFLIGHT_ITEMS.map(
      (item) => `| ${item} | collect ${item} | review-only evidence | use ${item} |`,
    ).join("\n"),
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> status --format json",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> status --format json",
    "## Long-Run Monitoring",
    "progress_credit last_useful_work last_peer_contribution expected_progress_window no_progress_threshold stall_diagnosis sync.no_progress_diagnosis sync.no_progress_next_action latest_stop_reason resource_bounds sync.resource_pressure recovery_evidence support_forensics",
    STRUCTURED_MONITORING_TERMS.join(" "),
    "## No-Progress Diagnosis",
    "artifact existence, elapsed time, daemon startup, peer reachability, raw log tail, raw logs, report existence, and support bundle existence are not sufficient proof.",
    "## Recovery And Stop Decisions",
    "safe_retry read_only_inspection backup_then_rebuild stop_and_escalate",
    "## Escalation Evidence Thresholds",
    "repeated no-progress with typed cause unavailable critical fields recovery class requiring stop/escalate resource pressure crossing documented bounds inconsistent status/support evidence failure to collect the minimum redacted support-bundle timeline",
    "## Support-Bundle Timeline",
    "preflight evidence command start status snapshots progress or no-progress events resource/recovery events support-bundle collection operator action taken final status escalation decision",
    "support-evidence.json support-evidence.md exact command output bounded log summary config summary service state or unavailable reason resource evidence recovery/progress evidence sync evidence version/toolchain context platform details exact repo-local reproduction command Unavailable: <reason>",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> support bundle --output-dir=<path>/support --format json",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> support bundle --output-dir=<path>/support --format json",
    "## Privacy And Safety Boundaries",
    TIMELINE_AND_PRIVACY_TERMS.join("\n"),
  ].join("\n\n");
}

function parityIndexText(): string {
  const evidence = [
    RUNBOOK_PATH,
    "docs/parity/production-claim-boundary.md",
    "docs/parity/support-matrix.md",
    "docs/parity/upgrade-and-rollback-policy.md",
    "docs/parity/release-readiness.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/checklist.md",
    "docs/parity/README.md",
    "README.md",
    "docs/operator/runtime-guide.md",
    "docs/parity/catalog/operator-runtime-release-hardening.md",
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
            requirements: PHASE85_REQUIREMENTS,
            evidence,
          },
        ],
      },
      audit: {
        v1_8_operator_runbooks: {
          path: "operator-runbooks.md",
          status: "done",
          requirements: PHASE85_REQUIREMENTS,
          evidence,
        },
      },
    },
    null,
    2,
  );
}

function humanPointerText(file: string): string {
  const maybeReadmeLink = file === "README.md" ? "docs/parity/operator-runbooks.md" : "";
  const maybeRuntimeLink =
    file === "docs/operator/runtime-guide.md" ? "../parity/operator-runbooks.md" : "";
  const maybeCatalogRow =
    file === "docs/parity/catalog/operator-runtime-release-hardening.md"
      ? "Phase 85 operator runbooks public-network default checks real service-manager multi-day default automatic support-bundle upload destructive repair broad production-node readiness"
      : "";

  return [
    `# ${file}`,
    "operator-runbooks.md",
    maybeReadmeLink,
    maybeRuntimeLink,
    maybeCatalogRow,
    `\`${SURFACE_ID}\``,
    PHASE85_REQUIREMENTS.join(" "),
    "production-boundary preflight long-run monitoring no-progress diagnosis recovery/stop decisions support-bundle timeline escalation evidence",
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE84_CHECKER_COMMAND,
    PHASE85_TEST_COMMAND,
    PHASE85_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 84 upgrade rollback policy" bun run scripts/check-phase84-upgrade-rollback-policy.ts`,
    `run_step "test Phase 85 operator runbooks checker" bun test scripts/check-phase85-operator-runbooks.test.ts`,
    `run_step "check Phase 85 operator runbooks" bun run scripts/check-phase85-operator-runbooks.ts`,
  ].join("\n");
}

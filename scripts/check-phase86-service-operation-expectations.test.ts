#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase86ServiceOperationExpectations } from "./check-phase86-service-operation-expectations";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE86_REPO_ROOT";
const SURFACE_ID = "v1-8-service-operation-expectations";
const PHASE86_REQUIREMENTS = ["SVC-01", "SVC-02"] as const;
const SERVICE_DOC_PATH = "docs/parity/service-operation-expectations.md";
const TABLE_HEADER =
  "Service surface | Support term | What evidence proves | Cargo command evidence | Bazel command evidence | Default verification | Opt-in UAT | Residual risk | Next gate";
const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
const PHASE86_TEST_COMMAND =
  "bun test scripts/check-phase86-service-operation-expectations.test.ts";
const PHASE86_CHECKER_COMMAND =
  "bun run scripts/check-phase86-service-operation-expectations.ts";
const TARGET_FILES = [
  SERVICE_DOC_PATH,
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/operator-runbooks.md",
  "docs/parity/upgrade-and-rollback-policy.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const SERVICE_SURFACES = [
  "Direct source-built open-bitcoind operation",
  "Local status and support evidence",
  "launchd/systemd generated definition preview",
  "Real user-level launchd/systemd lifecycle",
  "Service-manager unavailable status",
  "Packaged service distribution",
  "Windows service integration",
  "Automatic updates",
  "Production service ownership and uptime guarantees",
  "Broad production full-node readiness",
] as const;
const SERVICE_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1",
  "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-mainnet -openbitcoinsync=mainnet-ibd -server=1",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service preview",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service install --apply",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service start",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service status",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service restart",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service stop",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service disable",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet service uninstall --apply",
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support",
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
  maybeServiceDocAppend?: string;
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

test("passes_when_phase86_fixture_contains_service_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase86ServiceOperationExpectations();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_service_doc_omits_support_term_or_lifecycle_label", async () => {
  // Arrange
  const supportRoot = await createFixture({
    maybeOmission: { file: SERVICE_DOC_PATH, needle: "opt-in UAT" },
  });
  const lifecycleRoot = await createFixture({
    maybeOmission: { file: SERVICE_DOC_PATH, needle: "unavailable-manager" },
  });

  // Act
  const supportFailures = checkPhase86ServiceOperationExpectations(supportRoot);
  const lifecycleFailures = checkPhase86ServiceOperationExpectations(lifecycleRoot);

  // Assert
  expect(supportFailures.join("\n")).toContain("support terms");
  expect(lifecycleFailures.join("\n")).toContain("lifecycle labels");
});

test("fails_when_service_doc_omits_required_command_group_or_build_path", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: SERVICE_DOC_PATH,
      needle: SERVICE_COMMANDS[1],
    },
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("command evidence");
});

test("fails_when_service_doc_treats_artifact_existence_as_evidence", async () => {
  // Arrange
  const root = await createFixture({
    maybeServiceDocAppend: "service file existence proves installed service readiness.",
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("field-based evidence");
});

test("fails_when_service_doc_omits_restart_resume_field", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: { file: SERVICE_DOC_PATH, needle: "durable_progress" },
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("restart/resume evidence");
});

test("fails_when_service_doc_allows_mutating_default_verification", async () => {
  // Arrange
  const root = await createFixture({
    maybeServiceDocAppend: "Default verification runs real service-manager commands.",
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("default boundary");
});

test("fails_when_service_doc_allows_sensitive_support_artifacts_or_upload", async () => {
  // Arrange
  const root = await createFixture({
    maybeServiceDocAppend: "automatic support-bundle upload is supported.",
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("sensitive evidence");
});

test("fails_when_parity_roots_omit_service_requirement_or_evidence", async () => {
  // Arrange
  const missingRequirementRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll('"SVC-02"', '"SVC-XX"'),
  });
  const missingEvidenceRoot = await createFixture({
    maybeParityIndexText: parityIndexText().replaceAll(
      `"${SERVICE_DOC_PATH}"`,
      '"docs/parity/missing-service-operation-expectations.md"',
    ),
  });

  // Act
  const missingRequirementFailures =
    checkPhase86ServiceOperationExpectations(missingRequirementRoot);
  const missingEvidenceFailures =
    checkPhase86ServiceOperationExpectations(missingEvidenceRoot);

  // Assert
  expect(missingRequirementFailures.join("\n")).toContain("parity root");
  expect(missingEvidenceFailures.join("\n")).toContain("parity root");
});

test("fails_when_pointer_docs_duplicate_service_table", async () => {
  // Arrange
  const root = await createFixture({
    maybePointerAppend: {
      file: "README.md",
      text: `| ${TABLE_HEADER} |`,
    },
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("must not duplicate required text");
});

test("fails_when_phase86_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE85_CHECKER_COMMAND,
      PHASE86_TEST_COMMAND,
      PHASE86_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      `run_step "check Phase 85 operator runbooks" bun run scripts/check-phase85-operator-runbooks.ts`,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase86ServiceOperationExpectations(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});

test("fails_when_default_verify_contains_service_manager_network_or_production_drift", async () => {
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
      "production service ownership",
      "broad production-node readiness",
    ].map((forbiddenText) =>
      createFixture({ maybeVerifyScript: `${verifyScriptText()}\n${forbiddenText}\n` }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase86ServiceOperationExpectations(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase86-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    [
      SERVICE_DOC_PATH,
      options.maybeServiceDocAppend
        ? `${serviceDocText()}\n${options.maybeServiceDocAppend}\n`
        : serviceDocText(),
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

function serviceDocText(): string {
  return [
    `# Service Operation Expectations\n\nSurface id: \`${SURFACE_ID}\``,
    "## Scope And Non-Claims",
    "generated launchd/systemd definitions supervise `open-bitcoind`, not the `open-bitcoin` operator wrapper.",
    "`service preview` is always side-effect-free.",
    "`service install` and `service uninstall` are previews unless `--apply` is supplied.",
    "## Support Terms",
    "The only terms are `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`.",
    "## Service Surface Classification",
    `| ${TABLE_HEADER} |`,
    "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
    SERVICE_SURFACES.map(
      (surface) => `| ${surface} | supported | field evidence | cargo | bazel | docs | UAT | risk | gate |`,
    ).join("\n"),
    "## Repo-Local Command Evidence",
    SERVICE_COMMANDS.join("\n"),
    "## Field-Based Evidence Rules",
    "service file existence, daemon startup, elapsed time, raw log tail, public peer reachability, or a support bundle path is context only unless expected fields and unavailable reasons are present. Write Unavailable: <reason>.",
    "service.lifecycle service.log_path service.manager_command service.generated_service_file_path service.unavailable_reason service.restart_resume resource_bounds sync.resource_pressure recovery_category recovery_action next_action support-evidence.json support-evidence.md",
    "unmanaged installed-stopped running failed disabled unavailable-manager",
    "## Restart Resume Evidence",
    "service.restart_resume same_datadir prior_shutdown durable_progress stale_inflight recovery_category next_action same selected datadir",
    "service restart command success, daemon startup, and elapsed time do not prove durable resume.",
    "## Default Verification And Opt-In UAT Boundaries",
    "Default bash scripts/verify.sh remains deterministic, public-network-free, real-service-manager-free, and multi-day-free.",
    "It must not run public-network live smoke, real service-manager commands, long wall-clock sleeps, package-manager service commands, Windows service workflows, automatic support-bundle upload, production service ownership checks, or broad production-node readiness checks. opt-in UAT only.",
    "## Sensitive Evidence Boundaries",
    "wallet private material raw wallet files RPC cookies rpcpassword rpcauth raw datadirs unredacted logs raw unbounded logs automatic support-bundle upload production service ownership",
  ].join("\n\n");
}

function parityIndexText(): string {
  const evidence = [
    SERVICE_DOC_PATH,
    "docs/parity/production-claim-boundary.md",
    "docs/parity/support-matrix.md",
    "docs/parity/operator-runbooks.md",
    "docs/parity/upgrade-and-rollback-policy.md",
    "docs/parity/release-readiness.md",
    "docs/parity/deviations-and-unknowns.md",
    "docs/parity/checklist.md",
    "docs/parity/README.md",
    "README.md",
    "docs/operator/runtime-guide.md",
    "docs/parity/catalog/operator-runtime-release-hardening.md",
    "scripts/check-phase86-service-operation-expectations.ts",
    "scripts/check-phase86-service-operation-expectations.test.ts",
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
            requirements: PHASE86_REQUIREMENTS,
            evidence,
          },
        ],
      },
      audit: {
        v1_8_service_operation_expectations: {
          path: "service-operation-expectations.md",
          status: "done",
          requirements: PHASE86_REQUIREMENTS,
          evidence,
        },
      },
    },
    null,
    2,
  );
}

function humanPointerText(file: string): string {
  const maybeReadmeLink =
    file === "README.md" ? "docs/parity/service-operation-expectations.md" : "";
  const maybeRuntimeLink =
    file === "docs/operator/runtime-guide.md"
      ? "../parity/service-operation-expectations.md"
      : "";
  const serviceSummary = [
    `\`${SURFACE_ID}\``,
    PHASE86_REQUIREMENTS.join(" "),
    "source-built daemon operation",
    "launchd/systemd preview",
    "opt-in real service lifecycle UAT",
    "restart/resume fields",
    "repo-local Cargo/Bazel commands",
    "production-service non-claims",
  ].join(" ");
  const maybeCatalogRow =
    file === "docs/parity/catalog/operator-runtime-release-hardening.md"
      ? `Phase 86 service operation expectations ${serviceSummary}`
      : "";

  return [
    `# ${file}`,
    "service-operation-expectations.md",
    maybeReadmeLink,
    maybeRuntimeLink,
    maybeCatalogRow,
    serviceSummary,
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE85_CHECKER_COMMAND,
    PHASE86_TEST_COMMAND,
    PHASE86_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 85 operator runbooks" bun run scripts/check-phase85-operator-runbooks.ts`,
    `run_step "test Phase 86 service operation expectations checker" bun test scripts/check-phase86-service-operation-expectations.test.ts`,
    `run_step "check Phase 86 service operation expectations" bun run scripts/check-phase86-service-operation-expectations.ts`,
  ].join("\n");
}

import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase100RelayActivationBoundary } from "./check-phase100-relay-activation-boundary";

const SURFACE_ID = "v2-0-relay-activation-boundary";
const REQUIRED_ACT_REQUIREMENTS = ["ACT-01", "ACT-02", "ACT-03", "ACT-04"] as const;
const PHASE99_TEST_COMMAND =
  "bun test scripts/check-phase99-peer-policy-structured-log-emission.test.ts";
const PHASE99_CHECKER_COMMAND =
  "bun run scripts/check-phase99-peer-policy-structured-log-emission.ts";
const PHASE100_TEST_COMMAND =
  "bun test scripts/check-phase100-relay-activation-boundary.test.ts";
const PHASE100_CHECKER_COMMAND =
  "bun run scripts/check-phase100-relay-activation-boundary.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const TARGET_FILES = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE_LABELS = [
  "transaction_relay_policy_input",
  "force_relay_policy_input",
  "mempool_policy_input",
  "inactive_bloomfilter",
  "inactive_blockfilters",
  "permission_effect_inactive",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_when_phase100_fixture_contains_activation_policy_roots_and_verify_wiring", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase100RelayActivationBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_act_requirement_is_missing_from_parity_roots", () => {
  // Arrange
  const roots = REQUIRED_ACT_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase100RelayActivationBoundary(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_ACT_REQUIREMENTS[index]);
  }
});

test("fails_when_repo_local_cargo_or_bazel_relay_activation_uat_commands_are_missing", () => {
  // Arrange
  const missingCommands = [
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind --",
    "-openbitcoininboundpermissionclass=relay_loopback@127.0.0.1=in,relay,forcerelay,mempool",
  ];
  const roots = missingCommands.map((missingCommand) =>
    createFixture({
      maybeMutateFiles(files) {
        replaceInFile(files, "docs/operator/runtime-guide.md", missingCommand, "");
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase100RelayActivationBoundary(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("ACT-01 Phase 100 UAT command");
  }
});

test("fails_when_scoped_relay_policy_labels_are_missing", () => {
  // Arrange
  const roots = REQUIRED_EVIDENCE_LABELS.map((label) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, label);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase100RelayActivationBoundary(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 100 evidence label");
  }
});

test("fails_when_docs_claim_default_public_relay_or_deferred_protocol_support", () => {
  // Arrange
  const claims = [
    "Phase 100 supports public relay by default.",
    "Phase 100 provides compact block relay support.",
    "Phase 100 enables bloom filter serving support.",
    "Phase 100 adds package relay support.",
  ];
  const roots = claims.map((claim) =>
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/parity/catalog/p2p.md", claim);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase100RelayActivationBoundary(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("forbidden Phase 100 positive claim");
  }
});

test("allows_compact_block_claim_owned_by_a_later_phase", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/parity/catalog/p2p.md",
        "Phase 112 provides compact block relay support through its separately owned surface.",
      );
    },
  });

  // Act
  const failures = checkPhase100RelayActivationBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("rejects_unowned_production_claim_even_when_a_later_phase_is_named", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/parity/catalog/p2p.md",
        "Phase 117 provides production full-node readiness.",
      );
    },
  });

  // Act
  const failures = checkPhase100RelayActivationBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("forbidden Phase 100 positive claim");
});

test("fails_when_default_verifier_wiring_is_missing_or_public_network_scoped", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        replaceInFile(files, "scripts/verify.sh", PHASE100_TEST_COMMAND, "");
        replaceInFile(files, "scripts/verify.sh", PHASE100_CHECKER_COMMAND, "");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 100 public-network relay CI" true');
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 100 service-manager" systemctl status open-bitcoind');
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase100RelayActivationBoundary(root).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("default verifier");
  expect(failureMessages[1]).toContain("forbidden Phase 100 gate");
  expect(failureMessages[2]).toContain("forbidden Phase 100 gate");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase100-"));
  tempRoots.push(root);

  const files = fixtureFiles();
  options.maybeMutateFiles?.(files);

  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }

  return root;
}

function fixtureFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    ["docs/architecture/config-precedence.md", configPrecedenceText()],
    ["docs/architecture/status-snapshot.md", statusSnapshotText()],
    ["docs/architecture/operator-observability.md", operatorObservabilityText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/index.json", parityIndexText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/source-breadcrumbs.json", sourceBreadcrumbsText()],
    ["scripts/verify.sh", verifyScriptText()],
  ]);
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const [file, current] of files) {
    files.set(file, current.replaceAll(needle, ""));
  }
}

function replaceInFile(
  files: Map<TargetFile, string>,
  file: TargetFile,
  needle: string,
  replacement: string,
): void {
  files.set(file, (files.get(file) ?? "").replaceAll(needle, replacement));
}

function appendToFile(files: Map<TargetFile, string>, file: TargetFile, line: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${line}\n`);
}

function configPrecedenceText(): string {
  return [
    "# Config Ownership and Precedence",
    "Open Bitcoin-owned config includes `relay.enabled` and daemon `-openbitcoinrelay` overrides.",
    "`relay.enabled` is default-off and only feeds the Phase 100 activation policy.",
    "Baseline whitelist and whitebind remain rejected as relay aliases; whitelist and whitebind remain rejected.",
  ].join("\n");
}

function statusSnapshotText(): string {
  return [
    "# Status Snapshot Contract",
    "Phase 100 is a policy/config boundary.",
    "Eligibility reasons are eligible, disabled, activation_required, inbound_serving_required, permission_required, protected_not_relay, and permission_effect_inactive.",
    "Permission effects are transaction_relay_policy_input, force_relay_policy_input, mempool_policy_input, inactive_bloomfilter, and inactive_blockfilters.",
    "active_permission_effects still covers bounded non-relay effects such as download_serving_policy_input and address_response_policy_input.",
    "Phase 100 does not claim transaction download scheduling, orphan handling, mempool admission, relay serving/fanout, rebroadcast, compact block relay, bloom/filter serving, package relay, public relay by default, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet use.",
  ].join("\n");
}

function operatorObservabilityText(): string {
  return [
    "# Operator Observability Contracts",
    "Phase 100 evidence uses only low-cardinality labels: transaction_relay_policy_input, force_relay_policy_input, mempool_policy_input, inactive_bloomfilter, inactive_blockfilters, permission_effect_inactive, eligible, disabled, activation_required, inbound_serving_required, permission_required, and protected_not_relay.",
    "It must not expose raw permission class names, raw permission strings, peer ids, endpoints, transaction ids, raw transaction hex, credentials, or dynamic labels.",
    "Public-network relay review is opt-in and outside bash scripts/verify.sh.",
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "# Operator Runtime Guide",
    "Public-network relay review is opt-in and outside `bash scripts/verify.sh`.",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \\",
    "  -regtest \\",
    "  -openbitcoinrelay=1 \\",
    "  -openbitcoininbound=1 \\",
    "  -openbitcoinlisten=127.0.0.1:18444 \\",
    "  -openbitcoininboundpermissionclass=relay_loopback@127.0.0.1=in,relay,forcerelay,mempool",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- \\",
    "  -regtest \\",
    "  -openbitcoinrelay=1 \\",
    "  -openbitcoininbound=1 \\",
    "  -openbitcoinlisten=127.0.0.1:18444 \\",
    "  -openbitcoininboundpermissionclass=relay_loopback@127.0.0.1=in,relay,forcerelay,mempool",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `Phase 100 ${SURFACE_ID} keeps ACT-01, ACT-02, ACT-03, and ACT-04 auditable.`,
    "packages/bitcoin-knots/src/net_permissions.h",
    "packages/bitcoin-knots/src/net_permissions.cpp",
    "packages/bitcoin-knots/src/net.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/test/functional/p2p_permissions.py",
    "`relay.enabled`, `openbitcoinrelay`, transaction_relay_policy_input, force_relay_policy_input, mempool_policy_input, inactive_bloomfilter, inactive_blockfilters, eligible, disabled, activation_required, inbound_serving_required, permission_required, protected_not_relay, permission_effect_inactive.",
    "Phase 100 does not claim transaction download scheduling, orphan handling, mempool admission, relay serving/fanout, rebroadcast, compact block relay, bloom/filter serving, package relay, public relay by default, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet use.",
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
            status: "done",
            requirements: REQUIRED_ACT_REQUIREMENTS,
            evidence: [
              "docs/architecture/config-precedence.md",
              "docs/architecture/status-snapshot.md",
              "docs/architecture/operator-observability.md",
              "docs/operator/runtime-guide.md",
              "docs/parity/catalog/p2p.md",
              "docs/parity/checklist.md",
              "docs/parity/source-breadcrumbs.json",
              "packages/open-bitcoin-network/src/relay.rs",
              "packages/open-bitcoin-rpc/src/config/open_bitcoin.rs",
              "packages/open-bitcoin-rpc/src/config/loader.rs",
              "scripts/check-phase100-relay-activation-boundary.ts",
              "scripts/check-phase100-relay-activation-boundary.test.ts",
              "scripts/verify.sh",
            ],
            upstream: {
              sources: [
                "packages/bitcoin-knots/src/net_permissions.h",
                "packages/bitcoin-knots/src/net_permissions.cpp",
                "packages/bitcoin-knots/src/net.cpp",
                "packages/bitcoin-knots/src/net_processing.cpp",
              ],
              tests: ["packages/bitcoin-knots/test/functional/p2p_permissions.py"],
            },
          },
        ],
      },
    },
    null,
    2,
  );
}

function checklistText(): string {
  return [
    "# Parity Checklist",
    "| Surface | Status | Requirements | Evidence | Known Gaps | Suspected Unknowns |",
    "| --- | --- | --- | --- | --- | --- |",
    `| \`${SURFACE_ID}\` | \`done\` | \`ACT-01\`, \`ACT-02\`, \`ACT-03\`, \`ACT-04\` | evidence | Phase 100 defines activation only and does not claim public relay by default, compact block relay, bloom/filter serving, package relay, production service operation, production full-node readiness, or production-funds wallet use. | Future scoped evidence required. |`,
  ].join("\n");
}

function sourceBreadcrumbsText(): string {
  return JSON.stringify(
    {
      groups: [
        {
          label: "network-relay-activation-boundary",
          files: ["packages/open-bitcoin-network/src/relay.rs"],
          breadcrumbs: [
            "packages/bitcoin-knots/src/net_permissions.h",
            "packages/bitcoin-knots/src/net_permissions.cpp",
            "packages/bitcoin-knots/src/net.cpp",
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/test/functional/p2p_permissions.py",
          ],
        },
      ],
    },
    null,
    2,
  );
}

function verifyScriptText(): string {
  return [
    "#!/usr/bin/env bash",
    "set -euo pipefail",
    "# Phase 99 is followed by Phase 100.",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE99_TEST_COMMAND,
    PHASE99_CHECKER_COMMAND,
    PHASE100_TEST_COMMAND,
    PHASE100_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 99 peer-policy structured log emission checker" ${PHASE99_TEST_COMMAND}`,
    `run_step "check Phase 99 peer-policy structured log emission" ${PHASE99_CHECKER_COMMAND}`,
    `run_step "test Phase 100 relay activation boundary checker" ${PHASE100_TEST_COMMAND}`,
    `run_step "check Phase 100 relay activation boundary" ${PHASE100_CHECKER_COMMAND}`,
    `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
    "# OPEN_BITCOIN_PHASE100_REPO_ROOT may point the checker at a fixture root.",
  ].join("\n");
}

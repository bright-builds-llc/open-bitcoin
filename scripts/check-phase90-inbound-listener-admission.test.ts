#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase90InboundListenerAdmission } from "./check-phase90-inbound-listener-admission";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE90_REPO_ROOT";
const SURFACE_ID = "v1-9-inbound-listener-admission-policy";
const AUDIT_KEY = "v1_9_inbound_listener_admission_policy";
const PHASE88_CHECKER_COMMAND =
  "bun run scripts/check-phase88-deterministic-claim-guardrails.ts";
const PHASE90_TEST_COMMAND =
  "bun test scripts/check-phase90-inbound-listener-admission.test.ts";
const PHASE90_CHECKER_COMMAND =
  "bun run scripts/check-phase90-inbound-listener-admission.ts";
const PHASE90_REQUIREMENTS = [
  "INB-01",
  "INB-02",
  "INB-03",
  "INB-04",
  "INB-05",
] as const;
const REQUIRED_BREADCRUMB_PATHS = [
  "packages/open-bitcoin-network/src/inbound.rs",
  "packages/open-bitcoin-network/src/inbound/tests.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/tests.rs",
  "packages/open-bitcoin-node/src/status/inbound.rs",
  "packages/open-bitcoin-node/src/status/inbound/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
] as const;
const CARGO_DAEMON_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --";
const BAZEL_DAEMON_COMMAND = "bazel run //packages/open-bitcoin-rpc:open_bitcoind --";
const CARGO_CLI_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --";
const BAZEL_CLI_COMMAND = "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --";
const CARGO_OPERATOR_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --";
const BAZEL_OPERATOR_COMMAND = "bazel run //packages/open-bitcoin-cli:open_bitcoin --";
const SUPPORT_COMMAND = "support bundle --output-dir=/tmp/open-bitcoin-inbound-support";
const TARGET_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;
const tempRoots: string[] = [];

type TargetFile = (typeof TARGET_FILES)[number];

type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
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

test("passes_when_phase90_fixture_contains_inbound_admission_roots_and_verify_wiring", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase90InboundListenerAdmission();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_inb_requirement_is_missing_from_parity_roots", async () => {
  // Arrange
  const root = await createFixture({
    maybeMutateFiles(files) {
      files.set(
        "docs/parity/index.json",
        parityIndexText(["INB-01", "INB-02", "INB-03", "INB-05"]),
      );
    },
  });

  // Act
  const failures = checkPhase90InboundListenerAdmission(root);

  // Assert
  expect(failures.join("\n")).toContain("INB-04");
});

test("fails_when_repo_local_cargo_or_bazel_uat_command_forms_are_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    [CARGO_DAEMON_COMMAND, BAZEL_OPERATOR_COMMAND, SUPPORT_COMMAND].map((command) =>
      createFixture({
        maybeMutateFiles(files) {
          const current = files.get("docs/operator/runtime-guide.md") ?? "";
          files.set("docs/operator/runtime-guide.md", current.replace(command, ""));
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase90InboundListenerAdmission(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("UAT command");
  }
});

test("fails_when_required_new_rust_source_breadcrumb_path_is_missing", async () => {
  // Arrange
  const missingPath = REQUIRED_BREADCRUMB_PATHS[2];
  const root = await createFixture({
    maybeMutateFiles(files) {
      const current = files.get("docs/parity/source-breadcrumbs.json") ?? "";
      files.set("docs/parity/source-breadcrumbs.json", current.replace(`"${missingPath}"`, ""));
    },
  });

  // Act
  const failures = checkPhase90InboundListenerAdmission(root);

  // Assert
  expect(failures.join("\n")).toContain(missingPath);
});

test("fails_when_inbound_log_labels_or_reserved_slot_evidence_labels_are_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    ["inbound_listener_state", "reserved_slot"].map((label) =>
      createFixture({
        maybeMutateFiles(files) {
          for (const file of TARGET_FILES) {
            const current = files.get(file) ?? "";
            files.set(file, current.replaceAll(label, ""));
          }
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase90InboundListenerAdmission(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("inbound evidence label");
  }
});

test("fails_when_default_verifier_contains_public_network_service_or_deferred_governance_drift", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "nc -z 203.0.113.10 8333",
      "-openbitcoinlisten=0.0.0.0:8333",
      "-openbitcoinlisten=[::]:8333",
      "systemctl status open-bitcoin",
      "launchctl print gui/501/open-bitcoin",
      "sleep 259200",
      "transaction relay",
      "compact block relay",
      "mempool propagation",
      "permission classes",
      "address relay",
      "eviction",
      "ban policy",
      "DoS governance",
    ].map((forbiddenText) =>
      createFixture({
        maybeMutateFiles(files) {
          const current = files.get("scripts/verify.sh") ?? "";
          files.set("scripts/verify.sh", `${current}\n${forbiddenText}\n`);
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase90InboundListenerAdmission(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

test("fails_when_docs_claim_public_inbound_by_default_or_production_full_node_readiness", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "Open Bitcoin supports public inbound by default.",
      "Open Bitcoin is production full-node ready.",
    ].map((claim) =>
      createFixture({
        maybeMutateFiles(files) {
          const current = files.get("docs/parity/catalog/p2p.md") ?? "";
          files.set("docs/parity/catalog/p2p.md", `${current}\n${claim}\n`);
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase90InboundListenerAdmission(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 90 no-claim boundary");
  }
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase90-"));
  tempRoots.push(root);

  const files = new Map<TargetFile, string>([
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/architecture/config-precedence.md", configPrecedenceText()],
    ["docs/architecture/status-snapshot.md", statusSnapshotText()],
    ["docs/architecture/operator-observability.md", operatorObservabilityText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/index.json", parityIndexText(PHASE90_REQUIREMENTS)],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/source-breadcrumbs.json", sourceBreadcrumbsText()],
    ["scripts/verify.sh", verifyScriptText()],
  ]);

  options.maybeMutateFiles?.(files);

  for (const [file, text] of files) {
    await writeFixtureFile(root, file, text);
  }

  return root;
}

async function writeFixtureFile(root: string, file: string, contents: string): Promise<void> {
  const absolutePath = path.join(root, file);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, contents);
}

function runtimeGuideText(): string {
  return [
    "# Operator Runtime Guide",
    "Phase 90 Loopback Inbound Listener Review keeps inbound serving disabled by default.",
    "It does not make public inbound listening a default and does not claim production full-node readiness.",
    "`inbound.allow_public = true` is required before wildcard or public endpoints can pass listener preflight.",
    "Public or wildcard listener review is explicit operator UAT only and stays outside `bash scripts/verify.sh`.",
    "Daemon CLI forms:",
    CARGO_DAEMON_COMMAND,
    "  -regtest -datadir=/tmp/open-bitcoin-inbound-loopback -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 -server=1",
    BAZEL_DAEMON_COMMAND,
    "  -regtest -datadir=/tmp/open-bitcoin-inbound-loopback -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 -server=1",
    "Inspect baseline peer counts:",
    CARGO_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 getnetworkinfo",
    BAZEL_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 getnetworkinfo",
    "Inspect Open Bitcoin-owned inbound listener/admission evidence:",
    CARGO_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 openbitcoinnetworkstatus",
    BAZEL_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 openbitcoinnetworkstatus",
    "Inspect the shared operator status snapshot:",
    CARGO_OPERATOR_COMMAND,
    "  --network regtest --datadir=/tmp/open-bitcoin-inbound-loopback --format json status",
    BAZEL_OPERATOR_COMMAND,
    "  --network regtest --datadir=/tmp/open-bitcoin-inbound-loopback --format json status",
    "Collect a bounded local support bundle:",
    CARGO_OPERATOR_COMMAND,
    `  --network regtest --datadir=/tmp/open-bitcoin-inbound-loopback ${SUPPORT_COMMAND}`,
    BAZEL_OPERATOR_COMMAND,
    `  --network regtest --datadir=/tmp/open-bitcoin-inbound-loopback ${SUPPORT_COMMAND}`,
    "Expected evidence includes openbitcoinnetworkstatus, OpenBitcoinStatusSnapshot.peers.inbound, inbound_listener_state, inbound_preflight_reason, bound_endpoint, admission_reject_reason, reserved_slot_reject_count, and bounded support evidence.",
  ].join("\n");
}

function configPrecedenceText(): string {
  return [
    "# Config Ownership and Precedence",
    "`open-bitcoin.jsonc` owns inbound.enabled, inbound.listen_addresses, inbound.max_peers, inbound.reserved_slots, and inbound.allow_public.",
    "The daemon accepts -openbitcoininbound=1 and -openbitcoinlisten=127.0.0.1:18444 as Open Bitcoin-owned controls.",
    "Loopback endpoints are default review targets; public endpoints require explicit inbound.allow_public acknowledgement.",
  ].join("\n");
}

function statusSnapshotText(): string {
  return [
    "# Status Snapshot Contract",
    "OpenBitcoinStatusSnapshot.peers.inbound carries listener state, inbound_preflight_reason, bound endpoint evidence, admission counters, handshake counts, rejection counters, reserved_slot evidence, and latest admission event.",
    "The openbitcoinnetworkstatus RPC extension exposes the same shared inbound status while getnetworkinfo keeps connections_in and connections_out only.",
    "This does not promote public inbound defaults, permission classes, address relay, eviction, ban policy, broad DoS governance, transaction relay, compact block relay, mempool propagation, or production full-node readiness.",
  ].join("\n");
}

function operatorObservabilityText(): string {
  return [
    "# Operator Observability Contracts",
    "Phase 90 inbound logs use inbound_listener_state, inbound_preflight_reason, bound_endpoint, and admission_reject_reason.",
    "Metrics include low-cardinality admitted, rejected, cap-reject, reserved_slot, duplicate, and self-connection counters only.",
    "Support evidence stays bounded and redacted while public-network listener review stays opt-in UAT.",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `Phase 90 \`${SURFACE_ID}\` evidence keeps ${PHASE90_REQUIREMENTS.join(", ")} auditable.`,
    "Knots anchors are packages/bitcoin-knots/src/net.cpp, packages/bitcoin-knots/src/net_processing.cpp, and packages/bitcoin-knots/test/functional/p2p_handshake.py.",
    "Open Bitcoin uses open-bitcoin.jsonc inbound.enabled, inbound.listen_addresses, inbound.max_peers, inbound.reserved_slots, inbound.allow_public, -openbitcoininbound=1, and -openbitcoinlisten=127.0.0.1:18444.",
    "Detailed evidence belongs to openbitcoinnetworkstatus and OpenBitcoinStatusSnapshot.peers.inbound with inbound_listener_state, inbound_preflight_reason, admission_reject_reason, and reserved_slot labels.",
    "Phase 90 does not claim public listener defaults, transaction relay, compact block relay, mempool propagation, permission classes, address relay, eviction, ban policy, broad DoS governance, or production full-node readiness.",
  ].join("\n");
}

function checklistText(): string {
  return [
    "# Parity Checklist",
    "| Surface | Status | Requirements | Evidence | Known Gaps | Suspected Unknowns |",
    "| --- | --- | --- | --- | --- | --- |",
    `| \`${SURFACE_ID}\` | \`done\` | \`${PHASE90_REQUIREMENTS.join("`, `")}\` | runtime guide, config precedence, status snapshot, operator observability, P2P catalog, source breadcrumbs | public or wildcard endpoint review stays opt-in UAT outside \`bash scripts/verify.sh\`; peer permissions, address relay, eviction, ban, broad DoS governance, relay behavior, public listener defaults, and production readiness remain future-scoped. | Future Phase 91 through Phase 95 work owns broader network-participation claims. |`,
  ].join("\n");
}

function parityIndexText(requirements: readonly string[]): string {
  const evidence = [
    "docs/operator/runtime-guide.md",
    "docs/architecture/config-precedence.md",
    "docs/architecture/status-snapshot.md",
    "docs/architecture/operator-observability.md",
    "docs/parity/catalog/p2p.md",
    "docs/parity/source-breadcrumbs.json",
  ];

  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            status: "done",
            requirements,
            evidence,
          },
        ],
      },
      audit: {
        [AUDIT_KEY]: {
          path: "catalog/p2p.md",
          status: "done",
          requirements,
          evidence,
        },
      },
    },
    null,
    2,
  );
}

function sourceBreadcrumbsText(): string {
  return JSON.stringify(
    {
      version: 1,
      groups: [
        {
          label: "network-inbound-admission",
          files: [
            "packages/open-bitcoin-network/src/inbound.rs",
            "packages/open-bitcoin-network/src/inbound/tests.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/net.cpp",
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/test/functional/p2p_handshake.py",
          ],
        },
        {
          label: "rpc-inbound-listener",
          files: [
            "packages/open-bitcoin-rpc/src/inbound_listener.rs",
            "packages/open-bitcoin-rpc/src/inbound_listener/tests.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/net.cpp",
            "packages/bitcoin-knots/src/net_processing.cpp",
          ],
        },
        {
          label: "node-status-contract",
          files: [
            "packages/open-bitcoin-node/src/status/inbound.rs",
            "packages/open-bitcoin-node/src/status/inbound/tests.rs",
          ],
          breadcrumbs: [],
        },
        {
          label: "cli-operator-onboarding-contracts",
          files: ["packages/open-bitcoin-cli/src/operator/status/render/inbound.rs"],
          breadcrumbs: [],
        },
        {
          label: "cli-operator-support-bundles",
          files: ["packages/open-bitcoin-cli/src/operator/support/render/inbound.rs"],
          breadcrumbs: [],
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
    "run_step() {",
    "  local label=\"$1\"",
    "  shift",
    "  \"$@\"",
    "}",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE88_CHECKER_COMMAND,
    PHASE90_TEST_COMMAND,
    PHASE90_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 88 deterministic claim guardrails" ${PHASE88_CHECKER_COMMAND}`,
    `run_step "test Phase 90 inbound listener admission checker" ${PHASE90_TEST_COMMAND}`,
    `run_step "check Phase 90 inbound listener admission" ${PHASE90_CHECKER_COMMAND}`,
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
    'run_step "check file lengths" bash scripts/check-file-lengths.sh',
    'run_step "cargo clippy" cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings',
  ].join("\n");
}

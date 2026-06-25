#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase91PeerPermissions } from "./check-phase91-peer-permissions";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE91_REPO_ROOT";
const SURFACE_ID = "v1-9-peer-permissions-connection-classes";
const AUDIT_KEY = "v1_9_peer_permissions_connection_classes";
const PHASE90_CHECKER_COMMAND =
  "bun run scripts/check-phase90-inbound-listener-admission.ts";
const PHASE91_TEST_COMMAND =
  "bun test scripts/check-phase91-peer-permissions.test.ts";
const PHASE91_CHECKER_COMMAND = "bun run scripts/check-phase91-peer-permissions.ts";
const PHASE91_REQUIREMENTS = ["PERM-01", "PERM-02", "PERM-03", "PERM-04"] as const;
const REQUIRED_PERMISSION_TOKENS =
  "in,noban,forceinbound,download,addr,relay,forcerelay,mempool,bloomfilter,blockfilters";
const CARGO_DAEMON_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --";
const BAZEL_DAEMON_COMMAND = "bazel run //packages/open-bitcoin-rpc:open_bitcoind --";
const CARGO_CLI_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --";
const BAZEL_CLI_COMMAND = "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --";
const CARGO_OPERATOR_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --";
const BAZEL_OPERATOR_COMMAND = "bazel run //packages/open-bitcoin-cli:open_bitcoin --";
const SUPPORT_COMMAND = "support bundle --output-dir=/tmp/open-bitcoin-permission-support";
const PERMISSION_FLAG = `-openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=${REQUIRED_PERMISSION_TOKENS}`;
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

test("passes_when_phase91_fixture_contains_peer_permission_roots_and_verify_wiring", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase91PeerPermissions();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_perm_requirement_is_missing_from_parity_roots", async () => {
  // Arrange
  const root = await createFixture({
    maybeMutateFiles(files) {
      files.set(
        "docs/parity/index.json",
        parityIndexText(["PERM-01", "PERM-02", "PERM-04"]),
      );
    },
  });

  // Act
  const failures = checkPhase91PeerPermissions(root);

  // Assert
  expect(failures.join("\n")).toContain("PERM-03");
});

test("fails_when_repo_local_cargo_or_bazel_permission_uat_command_forms_are_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    [CARGO_DAEMON_COMMAND, BAZEL_OPERATOR_COMMAND, SUPPORT_COMMAND, PERMISSION_FLAG].map(
      (command) =>
        createFixture({
          maybeMutateFiles(files) {
            const current = files.get("docs/operator/runtime-guide.md") ?? "";
            files.set("docs/operator/runtime-guide.md", current.replace(command, ""));
          },
        }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase91PeerPermissions(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("UAT command");
  }
});

test("fails_when_required_peer_permission_source_breadcrumb_path_is_missing", async () => {
  // Arrange
  const missingPath = "packages/open-bitcoin-network/src/inbound/permissions.rs";
  const root = await createFixture({
    maybeMutateFiles(files) {
      const current = files.get("docs/parity/source-breadcrumbs.json") ?? "";
      files.set("docs/parity/source-breadcrumbs.json", current.replace(`"${missingPath}"`, ""));
    },
  });

  // Act
  const failures = checkPhase91PeerPermissions(root);

  // Assert
  expect(failures.join("\n")).toContain(missingPath);
});

test("fails_when_permission_status_or_inactive_effect_labels_are_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    ["permission_class", "active_permission_effects", "inactive_relay", "inactive_blockfilters"].map(
      (label) =>
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
  const failureMessages = roots.map((root) => checkPhase91PeerPermissions(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 91 evidence label");
  }
});

test("fails_when_default_verifier_contains_public_network_or_knots_permission_drift", async () => {
  // Arrange
  const roots = await Promise.all(
    ["public-network", "service-manager", "multi-day", "whitebind", "whitelist"].map(
      (forbiddenText) =>
        createFixture({
          maybeMutateFiles(files) {
            const current = files.get("scripts/verify.sh") ?? "";
            files.set("scripts/verify.sh", `${current}\n${forbiddenText}\n`);
          },
        }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase91PeerPermissions(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

test("fails_when_docs_claim_relay_public_default_production_or_whitelist_support", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "The all permission activates transaction relay support.",
      "Open Bitcoin offers compact block relay support.",
      "Open Bitcoin has public inbound by default.",
      "Phase 91 proves production full-node readiness.",
      "Open Bitcoin accepts Knots whitebind compatibility.",
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
  const failureMessages = roots.map((root) => checkPhase91PeerPermissions(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 91 no-claim boundary");
  }
});

test("fails_when_support_bundle_evidence_includes_raw_config_or_peer_details", async () => {
  // Arrange
  const root = await createFixture({
    maybeMutateFiles(files) {
      const current = files.get("docs/architecture/operator-observability.md") ?? "";
      files.set(
        "docs/architecture/operator-observability.md",
        `${current}\nSupport bundle evidence includes operator_loopback peer_id=7 127.0.0.1:18444 rpc_password=secret cookie=phase91-secret.\n`,
      );
    },
  });

  // Act
  const failures = checkPhase91PeerPermissions(root);

  // Assert
  expect(failures.join("\n")).toContain("Phase 91 support redaction boundary");
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase91-"));
  tempRoots.push(root);

  const files = new Map<TargetFile, string>([
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/architecture/config-precedence.md", configPrecedenceText()],
    ["docs/architecture/status-snapshot.md", statusSnapshotText()],
    ["docs/architecture/operator-observability.md", operatorObservabilityText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/index.json", parityIndexText(PHASE91_REQUIREMENTS)],
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
    "Phase 91 adds Open Bitcoin-owned peer permission classes to the explicit loopback listener review path.",
    "It does not accept Knots whitelist or whitebind compatibility inputs, and it does not enable public inbound defaults, transaction relay, compact block relay, mempool propagation, BIP37 bloom serving, compact-filter serving, full address relay, or production full-node readiness.",
    "The CLI form is -openbitcoininboundpermissionclass=<name>@<literal_ip>=<tokens> and the address component must be a literal IP address.",
    "Daemon CLI forms:",
    CARGO_DAEMON_COMMAND,
    "  -regtest -datadir=/tmp/open-bitcoin-permission-loopback -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 -openbitcoinreservedslots=1",
    `  ${PERMISSION_FLAG} -server=1`,
    BAZEL_DAEMON_COMMAND,
    "  -regtest -datadir=/tmp/open-bitcoin-permission-loopback -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 -openbitcoinreservedslots=1",
    `  ${PERMISSION_FLAG} -server=1`,
    "Inspect Open Bitcoin-owned permission evidence:",
    CARGO_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 openbitcoinnetworkstatus",
    BAZEL_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 openbitcoinnetworkstatus",
    "Inspect shared operator status:",
    CARGO_OPERATOR_COMMAND,
    "  --network regtest --datadir=/tmp/open-bitcoin-permission-loopback status --format json",
    BAZEL_OPERATOR_COMMAND,
    "  --network regtest --datadir=/tmp/open-bitcoin-permission-loopback status --format json",
    "Collect a redacted permission support bundle:",
    CARGO_OPERATOR_COMMAND,
    `  --network regtest --datadir=/tmp/open-bitcoin-permission-loopback ${SUPPORT_COMMAND}`,
    BAZEL_OPERATOR_COMMAND,
    `  --network regtest --datadir=/tmp/open-bitcoin-permission-loopback ${SUPPORT_COMMAND}`,
    "Expected review evidence includes permission_class, permissioned_inbound_peers, protected_inbound_peers, active_permission_effects, inactive_permission_effects, latest_permission_decision, inactive_relay, inactive_forcerelay, inactive_mempool, inactive_bloomfilter, and inactive_blockfilters.",
    "Support bundles are redacted and do not include raw peer ids, raw endpoints, raw config strings, RPC password values, or cookie contents.",
  ].join("\n");
}

function configPrecedenceText(): string {
  return [
    "# Config Ownership and Precedence",
    "Phase 91 extends inbound.permission_classes with Open Bitcoin-owned peer permission classes.",
    "The JSONC class uses operator_loopback, addresses, and permissions tokens in, noban, forceinbound, download, addr, relay, forcerelay, mempool, bloomfilter, and blockfilters.",
    "The repeatable CLI form is -openbitcoininboundpermissionclass=<name>@<literal_ip>=<tokens>.",
    `An example is ${PERMISSION_FLAG}.`,
    "The matching key is a literal IP address only. CIDR ranges, hostnames, and endpoint-shaped values are rejected at config parse time.",
    "Baseline Knots whitelist and whitebind compatibility is not silently accepted.",
  ].join("\n");
}

function statusSnapshotText(): string {
  return [
    "# Status Snapshot Contract",
    "Phase 91 extends OpenBitcoinStatusSnapshot.peers.inbound with bounded permission evidence.",
    "Fields are permission_class, permissioned_inbound_peers, protected_inbound_peers, active_permission_effects, inactive_permission_effects, and latest_permission_decision.",
    "Inactive labels include inactive_relay, inactive_forcerelay, inactive_mempool, inactive_bloomfilter, and inactive_blockfilters.",
    "Those labels are diagnostic evidence only; they do not claim transaction relay, compact block relay, mempool propagation, BIP37 serving, compact-filter serving, full address relay, public inbound defaults, or production full-node readiness.",
  ].join("\n");
}

function operatorObservabilityText(): string {
  return [
    "# Operator Observability Contracts",
    "Phase 91 permission metrics are InboundPermissionedAdmitCount, InboundProtectedAdmitCount, InboundInactivePermissionEffectCount, and InboundPermissionValidationFailureCount.",
    "Permission evidence labels include permission_class, permissioned_inbound_peers, protected_inbound_peers, active_permission_effects, inactive_permission_effects, and latest_permission_decision.",
    "Support bundle evidence is redacted and includes inactive labels such as inactive_relay, inactive_mempool, inactive_bloomfilter, and inactive_blockfilters without raw values.",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `Phase 91 ${SURFACE_ID} evidence keeps ${PHASE91_REQUIREMENTS.join(", ")} auditable.`,
    "Knots anchors are packages/bitcoin-knots/src/net_permissions.h, packages/bitcoin-knots/src/net_permissions.cpp, packages/bitcoin-knots/src/net.cpp, packages/bitcoin-knots/src/net_processing.cpp, and packages/bitcoin-knots/test/functional/p2p_permissions.py.",
    "Open Bitcoin uses inbound.permission_classes and -openbitcoininboundpermissionclass=<name>@<literal_ip>=<tokens> with permission tokens in,noban,forceinbound,download,addr,relay,forcerelay,mempool,bloomfilter,blockfilters.",
    "Phase 91 status and support evidence uses permission_class, permissioned_inbound_peers, protected_inbound_peers, active_permission_effects, inactive_permission_effects, latest_permission_decision, inactive_relay, inactive_forcerelay, inactive_mempool, inactive_bloomfilter, and inactive_blockfilters.",
    "Phase 91 does not claim Knots whitelist or whitebind compatibility, transaction relay, compact block relay, mempool propagation, BIP37 bloom serving, compact-filter serving, full address relay, public inbound defaults, or production full-node readiness.",
  ].join("\n");
}

function checklistText(): string {
  return [
    "# Parity Checklist",
    "| Surface | Status | Requirements | Evidence | Known Gaps | Suspected Unknowns |",
    "| --- | --- | --- | --- | --- | --- |",
    `| ${SURFACE_ID} | done | ${PHASE91_REQUIREMENTS.join(", ")} | runtime guide, config precedence, status snapshot, operator observability, P2P catalog, source breadcrumbs | Knots whitelist or whitebind compatibility, transaction relay, compact block relay, mempool propagation, BIP37, compact filters, public inbound defaults, and production readiness remain outside Phase 91. | Future phases own broader network-participation claims. |`,
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
    ".planning/phases/91-peer-permissions-and-connection-classes/91-01-SUMMARY.md",
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
          label: "network-peer-permissions",
          files: ["packages/open-bitcoin-network/src/inbound/permissions.rs"],
          breadcrumbs: [
            "packages/bitcoin-knots/src/net_permissions.h",
            "packages/bitcoin-knots/src/net_permissions.cpp",
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
    "run_step() {",
    "  local label=\"$1\"",
    "  shift",
    "  \"$@\"",
    "}",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE90_CHECKER_COMMAND,
    PHASE91_TEST_COMMAND,
    PHASE91_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 90 inbound listener admission" ${PHASE90_CHECKER_COMMAND}`,
    `run_step "test Phase 91 peer permissions checker" ${PHASE91_TEST_COMMAND}`,
    `run_step "check Phase 91 peer permissions" ${PHASE91_CHECKER_COMMAND}`,
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
    'run_step "check file lengths" bash scripts/check-file-lengths.sh',
  ].join("\n");
}

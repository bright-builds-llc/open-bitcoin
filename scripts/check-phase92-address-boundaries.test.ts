#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase92AddressBoundaries } from "./check-phase92-address-boundaries";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v1-9-address-advertisement-discovery-boundaries";
const PHASE92_REQUIREMENTS = ["ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04"] as const;
const PHASE91_TEST_COMMAND =
  "bun test scripts/check-phase91-peer-permissions.test.ts";
const PHASE91_CHECKER_COMMAND = "bun run scripts/check-phase91-peer-permissions.ts";
const PHASE92_TEST_COMMAND =
  "bun test scripts/check-phase92-address-boundaries.test.ts";
const PHASE92_CHECKER_COMMAND = "bun run scripts/check-phase92-address-boundaries.ts";
const CARGO_DAEMON_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --";
const BAZEL_DAEMON_COMMAND = "bazel run //packages/open-bitcoin-rpc:open_bitcoind --";
const CARGO_CLI_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --";
const BAZEL_CLI_COMMAND = "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli --";
const CARGO_OPERATOR_COMMAND =
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --";
const BAZEL_OPERATOR_COMMAND = "bazel run //packages/open-bitcoin-cli:open_bitcoin --";
const ADDRESS_PERMISSION_FLAG =
  "-openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=in,noban,forceinbound,download,addr";
const SUPPORT_COMMAND = "support bundle --output-dir=/tmp/open-bitcoin-address-support";
const TARGET_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
  "scripts/verify.sh",
] as const;
const REQUIRED_LABELS = [
  "local_advertisement_candidates",
  "suppressed_advertisements",
  "not_publicly_routable",
  "bounded getaddr",
  "learned_address_entries",
  "learned_address_rejections",
  "latest_address_decision",
  "full_relay_deferred",
] as const;
const REQUIRED_BREADCRUMB_FILES = [
  "packages/open-bitcoin-network/src/address.rs",
  "packages/open-bitcoin-network/src/address/advertisement.rs",
  "packages/open-bitcoin-network/src/address/book.rs",
  "packages/open-bitcoin-network/src/address/response.rs",
  "packages/open-bitcoin-network/src/address/tests.rs",
] as const;
const FORBIDDEN_PUBLIC_VERIFY_FRAGMENTS = [
  "curl ",
  "nc ",
  "systemctl",
  "launchctl",
  "dig ",
  "nslookup",
  "--public-network",
  "multi-day",
] as const;
const RAW_EVIDENCE_STRINGS = [
  "127.0.0.1:",
  "0.0.0.0:",
  "::1",
  "address_bytes",
  "peer_id=",
  "raw_permission",
  "operator_loopback",
  "inbound.allow_public=true",
] as const;
const tempRoots: string[] = [];

type TargetFile = (typeof TARGET_FILES)[number];

type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
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

test("passes_when_phase92_fixture_contains_address_boundary_roots", async () => {
  // Arrange
  const root = await createFixture();

  // Act
  const failures = checkPhase92AddressBoundaries({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("passes_for_real_repo_corpus_after_phase92_docs_are_registered", () => {
  // Arrange, Act
  const failures = checkPhase92AddressBoundaries({ rootDir: REPO_ROOT });

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_addr_requirement_is_missing_from_parity_roots", async () => {
  // Arrange
  const roots = await Promise.all(
    PHASE92_REQUIREMENTS.map((missingRequirement) =>
      createFixture({
        maybeMutateFiles(files) {
          files.set(
            "docs/parity/index.json",
            parityIndexText(PHASE92_REQUIREMENTS.filter((id) => id !== missingRequirement)),
          );
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 requirement coverage");
  }
});

test("fails_when_required_docs_status_or_support_labels_are_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    REQUIRED_LABELS.map((label) =>
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
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 evidence label");
  }
});

test("fails_when_required_phase92_source_breadcrumb_path_is_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    REQUIRED_BREADCRUMB_FILES.map((missingPath) =>
      createFixture({
        maybeMutateFiles(files) {
          const current = files.get("docs/parity/source-breadcrumbs.json") ?? "";
          files.set("docs/parity/source-breadcrumbs.json", current.replace(`"${missingPath}"`, ""));
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 source breadcrumb coverage");
  }
});

test("fails_when_repo_local_address_uat_command_fragments_are_missing", async () => {
  // Arrange
  const roots = await Promise.all(
    [CARGO_DAEMON_COMMAND, BAZEL_DAEMON_COMMAND, CARGO_CLI_COMMAND, BAZEL_CLI_COMMAND, SUPPORT_COMMAND].map(
      (fragment) =>
        createFixture({
          maybeMutateFiles(files) {
            const current = files.get("docs/operator/runtime-guide.md") ?? "";
            files.set("docs/operator/runtime-guide.md", current.replace(fragment, ""));
          },
        }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 UAT command");
  }
});

test("fails_when_default_verifier_contains_public_network_command_fragments", async () => {
  // Arrange
  const roots = await Promise.all(
    FORBIDDEN_PUBLIC_VERIFY_FRAGMENTS.map((fragment) =>
      createFixture({
        maybeMutateFiles(files) {
          const current = files.get("scripts/verify.sh") ?? "";
          files.set("scripts/verify.sh", `${current}\n${fragment}\n`);
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 default verifier boundary");
  }
});

test("fails_when_docs_claim_full_relay_discovery_public_or_production_support", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "Phase 92 supports full address relay.",
      "Phase 92 includes peer discovery support.",
      "public inbound by default is enabled.",
      "public-network readiness is achieved.",
      "production full-node readiness is achieved.",
      "unsolicited address relay is supported.",
      "addr gossip relay is supported.",
      "DNS seed discovery support is enabled.",
      "UPnP/NAT-PMP discovery support is enabled.",
      "-discover parity is supported.",
      "-externalip parity is supported.",
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
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 no-claim boundary");
  }
});

test("allows_required_negative_no_claim_sentences", async () => {
  // Arrange
  const root = await createFixture({
    maybeMutateFiles(files) {
      const current = files.get("docs/parity/catalog/p2p.md") ?? "";
      files.set(
        "docs/parity/catalog/p2p.md",
        [
          current,
          "Peer discovery, unsolicited address relay, DNS seed discovery, UPnP/NAT-PMP discovery, and public-network readiness remain outside this surface.",
          "This does not imply full address relay and is documented without claiming full address relay.",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase92AddressBoundaries({ rootDir: root });

  // Assert
  expect(failures.filter((failure) => failure.includes("Phase 92 no-claim boundary"))).toEqual([]);
});

test("fails_when_status_or_support_evidence_includes_raw_material", async () => {
  // Arrange
  const roots = await Promise.all(
    RAW_EVIDENCE_STRINGS.map((rawEvidence) =>
      createFixture({
        maybeMutateFiles(files) {
          const current =
            files.get("packages/open-bitcoin-cli/src/operator/support/render/inbound.rs") ??
            "";
          files.set(
            "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
            `${current}\nconst RAW_PHASE92_EVIDENCE: &str = \"${rawEvidence}\";\n`,
          );
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("Phase 92 raw evidence boundary");
  }
});

test("fails_when_phase92_verifier_order_is_missing_or_misordered", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      (text: string) => text.replace(PHASE92_TEST_COMMAND, ""),
      (text: string) => text.replace(PHASE92_CHECKER_COMMAND, ""),
      (text: string) =>
        text.replace(
          PHASE92_TEST_COMMAND,
          "bun test scripts/check-phase89-not-real.test.ts",
        ),
      (text: string) =>
        text.replace(
          `run_step "check Phase 91 peer permissions" ${PHASE91_CHECKER_COMMAND}`,
          `run_step "check Phase 92 address boundaries" ${PHASE92_CHECKER_COMMAND}`,
        ),
    ].map((mutateVerify) =>
      createFixture({
        maybeMutateFiles(files) {
          const current = files.get("scripts/verify.sh") ?? "";
          files.set("scripts/verify.sh", mutateVerify(current));
        },
      }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase92AddressBoundaries({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("verifier-order");
  }
});

async function createFixture(options: FixtureOptions = {}): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase92-"));
  tempRoots.push(root);

  const files = new Map<TargetFile, string>([
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/architecture/status-snapshot.md", statusSnapshotText()],
    ["docs/architecture/operator-observability.md", operatorObservabilityText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/index.json", parityIndexText(PHASE92_REQUIREMENTS)],
    ["docs/parity/source-breadcrumbs.json", sourceBreadcrumbsText()],
    [
      "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
      statusRendererText(),
    ],
    [
      "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
      supportRendererText(),
    ],
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
    `Phase 92 ${SURFACE_ID} keeps ${PHASE92_REQUIREMENTS.join(", ")} auditable.`,
    "It covers local listener-derived advertisement decisions, direct bounded getaddr request-response evidence, and learned-address intake counts.",
    "It does not add peer discovery support, full address relay support, public inbound by default, public-network readiness, or production full-node readiness.",
    CARGO_DAEMON_COMMAND,
    "  -regtest -datadir=/tmp/open-bitcoin-address-boundary -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 -openbitcoinreservedslots=1",
    `  ${ADDRESS_PERMISSION_FLAG} -server=1`,
    BAZEL_DAEMON_COMMAND,
    "  -regtest -datadir=/tmp/open-bitcoin-address-boundary -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 -openbitcoinreservedslots=1",
    `  ${ADDRESS_PERMISSION_FLAG} -server=1`,
    CARGO_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 openbitcoinnetworkstatus",
    BAZEL_CLI_COMMAND,
    "  -regtest -rpcconnect=127.0.0.1 -rpcport=18443 openbitcoinnetworkstatus",
    CARGO_OPERATOR_COMMAND,
    "  --network regtest --datadir=/tmp/open-bitcoin-address-boundary status --format json",
    BAZEL_OPERATOR_COMMAND,
    "  --network regtest --datadir=/tmp/open-bitcoin-address-boundary status --format json",
    CARGO_OPERATOR_COMMAND,
    `  --network regtest --datadir=/tmp/open-bitcoin-address-boundary ${SUPPORT_COMMAND}`,
    BAZEL_OPERATOR_COMMAND,
    `  --network regtest --datadir=/tmp/open-bitcoin-address-boundary ${SUPPORT_COMMAND}`,
    `Expected evidence includes ${REQUIRED_LABELS.join(", ")}.`,
  ].join("\n");
}

function statusSnapshotText(): string {
  return [
    "# Status Snapshot Contract",
    "Phase 92 extends OpenBitcoinStatusSnapshot.peers.inbound with bounded address evidence.",
    "Shared fields include local_advertisement_candidates, suppressed_advertisements, getaddr_responses_served, getaddr_requests_suppressed, learned_address_entries, learned_address_rejections, and latest_address_decision.",
    "The fields separate local listener advertisement, direct bounded getaddr handling, and learned-address storage from peer discovery and relay claims.",
    "They do not claim peer discovery support, full address relay support, public inbound by default, DNS seed discovery, UPnP/NAT-PMP discovery, public-network CI, or production full-node readiness.",
    "Use full_relay_deferred when a no-claim label is needed.",
  ].join("\n");
}

function operatorObservabilityText(): string {
  return [
    "# Operator Observability Contracts",
    "Phase 92 address-boundary observability uses local_advertisement_candidates, suppressed_advertisements, not_publicly_routable, bounded getaddr, learned_address_entries, latest_address_decision, and full_relay_deferred.",
    "Metrics remain aggregate and low-cardinality.",
    "Support bundles preserve safe labels and counts while redacting raw address material.",
    "These fields do not claim peer discovery support, full address relay support, public inbound by default, unsolicited addr gossip, DNS seed discovery, UPnP/NAT-PMP discovery, or production full-node readiness.",
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `The ${SURFACE_ID} surface covers ${PHASE92_REQUIREMENTS.join(", ")}.`,
    "Knots anchors include packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/src/netaddress.h, packages/bitcoin-knots/src/netaddress.cpp, packages/bitcoin-knots/src/net.cpp, packages/bitcoin-knots/src/net_processing.cpp, packages/bitcoin-knots/src/addrman.h, packages/bitcoin-knots/src/addrman.cpp, packages/bitcoin-knots/src/addrdb.h, and packages/bitcoin-knots/src/addrdb.cpp.",
    "Accepted local listener evidence appears as local_advertisement_candidates; rejected listener evidence appears as suppressed_advertisements with not_publicly_routable.",
    "Direct getaddr handling is bounded getaddr evidence only.",
    "Learned-address storage records learned_address_entries, learned_address_rejections, freshness, source, routability, service, port, and persistence eligibility evidence.",
    "Peer discovery, unsolicited address relay, DNS seed discovery, UPnP/NAT-PMP discovery, and public-network readiness remain outside this surface.",
    "Unsolicited addr relay and addr gossip relay remain full_relay_deferred without claiming full address relay.",
    "`-discover` parity and `-externalip` parity remain outside this surface.",
  ].join("\n");
}

function checklistText(): string {
  return [
    "# Parity Checklist",
    "| Surface | Status | Requirements | Evidence | Known Gaps | Suspected Unknowns |",
    "| --- | --- | --- | --- | --- | --- |",
    `| ${SURFACE_ID} | done | ${PHASE92_REQUIREMENTS.join(", ")} | runtime guide, status snapshot, operator observability, P2P catalog, source breadcrumbs | Full address relay, unsolicited fanout, addr gossip relay, DNS seed discovery, UPnP/NAT-PMP discovery, -discover/-externalip parity, public inbound defaults, public-network CI, and production full-node readiness remain outside this surface. | Future phases own broader network-participation claims. |`,
  ].join("\n");
}

function parityIndexText(requirements: readonly string[]): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            title: "v1.9 Address Advertisement and Discovery Boundaries",
            status: "done",
            requirements,
            evidence: [
              "docs/operator/runtime-guide.md",
              "docs/architecture/status-snapshot.md",
              "docs/architecture/operator-observability.md",
              "docs/parity/catalog/p2p.md",
              "docs/parity/source-breadcrumbs.json",
            ],
          },
        ],
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
          label: "network-address-boundaries",
          files: [...REQUIRED_BREADCRUMB_FILES],
          breadcrumbs: [
            "packages/bitcoin-knots/src/protocol.h",
            "packages/bitcoin-knots/src/netaddress.h",
            "packages/bitcoin-knots/src/netaddress.cpp",
            "packages/bitcoin-knots/src/net.cpp",
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/src/addrman.h",
            "packages/bitcoin-knots/src/addrman.cpp",
            "packages/bitcoin-knots/src/addrdb.h",
            "packages/bitcoin-knots/src/addrdb.cpp",
          ],
        },
      ],
    },
    null,
    2,
  );
}

function statusRendererText(): string {
  return [
    "fn address_boundary_text() -> &'static str {",
    '    "local_advertisement_candidates suppressed_advertisements not_publicly_routable bounded getaddr learned_address_entries learned_address_rejections latest_address_decision full_relay_deferred"',
    "}",
  ].join("\n");
}

function supportRendererText(): string {
  return [
    "const PHASE92_ADDRESS_BOUNDARY_NEXT_ACTION: &str =",
    '    "Treat Phase 92 as bounded local advertisement and direct getaddr evidence only; peer discovery, unsolicited address relay, DNS seed discovery, UPnP/NAT-PMP discovery, and public-network readiness remain outside this surface.";'
  ].join("\n");
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
    PHASE91_TEST_COMMAND,
    PHASE91_CHECKER_COMMAND,
    PHASE92_TEST_COMMAND,
    PHASE92_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 91 peer permissions checker" ${PHASE91_TEST_COMMAND}`,
    `run_step "check Phase 91 peer permissions" ${PHASE91_CHECKER_COMMAND}`,
    `run_step "test Phase 92 address boundaries checker" ${PHASE92_TEST_COMMAND}`,
    `run_step "check Phase 92 address boundaries" ${PHASE92_CHECKER_COMMAND}`,
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
    'run_step "check file lengths" bash scripts/check-file-lengths.sh',
  ].join("\n");
}

import { afterEach, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase93PeerPolicy } from "./check-phase93-peer-policy";

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with complete Phase 93 evidence", () => {
  const root = writeFixture();

  expect(checkPhase93PeerPolicy({ rootDir: root })).toEqual([]);
});

test("fails when the parity index omits a Phase 93 requirement", () => {
  const root = writeFixture({
    "docs/parity/index.json": parityIndexText(["EVICT-01", "EVICT-02", "EVICT-03"]),
  });

  const failures = checkPhase93PeerPolicy({ rootDir: root });

  expect(failures.join("\n")).toContain("Phase 93 requirements mismatch");
});

test("fails when peer-policy source breadcrumbs are missing", () => {
  const root = writeFixture({
    "docs/parity/source-breadcrumbs.json": JSON.stringify({ groups: [] }, null, 2),
  });

  const failures = checkPhase93PeerPolicy({ rootDir: root });

  expect(failures.join("\n")).toContain("network-peer-policy");
});

test("fails when renderer text includes raw peer-policy evidence", () => {
  const root = writeFixture({
    "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs":
      "latest_peer_policy_decision peer_id=93",
  });

  const failures = checkPhase93PeerPolicy({ rootDir: root });

  expect(failures.join("\n")).toContain("raw detail");
});

test("fails when verifier does not run Phase 93 after Phase 92", () => {
  const root = writeFixture({
    "scripts/verify.sh": verifyText({ includePhase93: false }),
  });

  const failures = checkPhase93PeerPolicy({ rootDir: root });

  expect(failures.join("\n")).toContain("Phase 93 verifier-order");
});

function writeFixture(overrides: Record<string, string> = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase93-"));
  tempRoots.push(root);
  const files = { ...fixtureFiles(), ...overrides };
  for (const [relativePath, contents] of Object.entries(files)) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}

function fixtureFiles(): Record<string, string> {
  const commonText = [
    "v1-9-eviction-ban-misbehavior-policy",
    "EVICT-01 EVICT-02 EVICT-03 EVICT-04",
    "eviction_candidates_evaluated disconnects_requested discouraged_peers active_bans",
    "expired_bans manual_unbans misbehavior_observations protected_no_actions",
    "latest_peer_policy_decision eviction_candidate_selected eviction_suppressed",
    "misbehavior_policy_decision source_eviction_policy source_misbehavior_policy",
    "packages/bitcoin-knots/src/net.cpp packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/src/banman.h packages/bitcoin-knots/src/banman.cpp",
    "packages/bitcoin-knots/src/net_permissions.cpp",
    "Phase 93 does not claim production banlist parity, public ban enforcement,",
    "Knots discourage parity, broad DoS/resource governance, transaction relay abuse handling,",
    "compact block relay abuse handling, public inbound by default, or production full-node readiness.",
  ].join("\n");
  const rendererText = [
    "eviction_candidates_evaluated disconnects_requested discouraged_peers active_bans",
    "expired_bans manual_unbans misbehavior_observations protected_no_actions",
    "latest_peer_policy_decision eviction_candidate_selected eviction_suppressed",
    "misbehavior_policy_decision source_eviction_policy source_misbehavior_policy",
  ].join("\n");

  return {
    "docs/operator/runtime-guide.md": `${commonText}\n${runtimeCommandsText()}`,
    "docs/architecture/status-snapshot.md": commonText,
    "docs/architecture/operator-observability.md": commonText,
    "docs/parity/catalog/p2p.md": commonText,
    "docs/parity/checklist.md": commonText,
    "docs/parity/index.json": parityIndexText(),
    "docs/parity/source-breadcrumbs.json": sourceBreadcrumbsText(),
    "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs": rendererText,
    "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs": rendererText,
    "scripts/verify.sh": verifyText(),
  };
}

function runtimeCommandsText(): string {
  return [
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \\",
    "  -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 \\",
    "  -openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=in,noban,forceinbound,download,addr",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- \\",
    "  -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444 \\",
    "  -openbitcoininboundpermissionclass=operator_loopback@127.0.0.1=in,noban,forceinbound,download,addr",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- openbitcoinnetworkstatus",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- openbitcoinnetworkstatus",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-peer-policy-support",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-peer-policy-support",
  ].join("\n");
}

function parityIndexText(
  requirements: string[] = ["EVICT-01", "EVICT-02", "EVICT-03", "EVICT-04"],
): string {
  return JSON.stringify(
    {
      surfaces: [{ name: "v1-9-eviction-ban-misbehavior-policy", status: "done" }],
      checklist: {
        surfaces: [
          {
            id: "v1-9-eviction-ban-misbehavior-policy",
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
      groups: [
        {
          label: "network-peer-policy",
          files: [
            "packages/open-bitcoin-network/src/peer/policy_state.rs",
            "packages/open-bitcoin-network/src/peer_policy.rs",
            "packages/open-bitcoin-network/src/peer_policy/tests.rs",
          ],
          breadcrumbs: [
            "packages/bitcoin-knots/src/net.cpp",
            "packages/bitcoin-knots/src/net_processing.cpp",
            "packages/bitcoin-knots/src/banman.h",
            "packages/bitcoin-knots/src/banman.cpp",
            "packages/bitcoin-knots/src/net_permissions.cpp",
          ],
        },
      ],
    },
    null,
    2,
  );
}

function verifyText(options: { includePhase93?: boolean } = {}): string {
  const includePhase93 = options.includePhase93 ?? true;
  const printed = [
    "bun test scripts/check-phase92-address-boundaries.test.ts",
    "bun run scripts/check-phase92-address-boundaries.ts",
    ...(includePhase93
      ? [
          "bun test scripts/check-phase93-peer-policy.test.ts",
          "bun run scripts/check-phase93-peer-policy.ts",
        ]
      : []),
  ].join("\n");
  const executed = [
    'run_step "test Phase 92 address boundaries checker" bun test scripts/check-phase92-address-boundaries.test.ts',
    'run_step "check Phase 92 address boundaries" bun run scripts/check-phase92-address-boundaries.ts',
    ...(includePhase93
      ? [
          'run_step "test Phase 93 peer policy checker" bun test scripts/check-phase93-peer-policy.test.ts',
          'run_step "check Phase 93 peer policy" bun run scripts/check-phase93-peer-policy.ts',
        ]
      : []),
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
  return [": <<'VERIFY_COMMAND_ORDER'", printed, "VERIFY_COMMAND_ORDER", executed].join("\n");
}

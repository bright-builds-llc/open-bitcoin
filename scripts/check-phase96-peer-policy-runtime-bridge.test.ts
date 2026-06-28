import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase96PeerPolicyRuntimeBridge } from "./check-phase96-peer-policy-runtime-bridge";

const TARGET_FILES = [
  "packages/open-bitcoin-network/src/peer_policy.rs",
  "packages/open-bitcoin-node/src/network.rs",
  "packages/open-bitcoin-node/src/network/peer_policy.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
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

test("passes with complete Phase 96 runtime bridge corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails empty peer-policy decision slices", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "packages/open-bitcoin-node/src/network.rs",
        "ManagedPeerPolicyInfo::from_policy_decisions(count, Some(eviction), &[], &[], &[])",
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("empty decision slices");
});

test("fails aggregate-only reconnect suppression", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "packages/open-bitcoin-rpc/src/context/network.rs",
        "let _ = (remote_addr, now_unix_seconds); ReconnectSuppressionInput { banned: peer_policy_info.active_bans > 0, discouraged: peer_policy_info.discouraged_peers > 0 }",
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  const message = failures.join("\n");
  expect(message).toContain("active_bans > 0");
  expect(message).toContain("remote_addr.ip()");
});

test("fails raw peer-policy material in output surfaces", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      const current =
        files.get("packages/open-bitcoin-cli/src/operator/support/render/inbound.rs") ?? "";
      files.set(
        "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
        `${current}\noutput.push_str("raw_endpoint peer_id=42 permission_string payload_bytes credential secret cookie=value");\n`,
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("raw_endpoint");
});

test("fails forbidden Phase 96 default verifier gates", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      const current = files.get("scripts/verify.sh") ?? "";
      files.set(
        "scripts/verify.sh",
        `${current}\nrun_step "Phase 96 public-network service-manager gate" echo ok\n`,
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("public-network");
});

test("fails public banlist and production claim creep", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      const current = files.get("docs/parity/catalog/p2p.md") ?? "";
      files.set(
        "docs/parity/catalog/p2p.md",
        `${current}\nPhase 96 provides public banlist support and production readiness.\n`,
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  const message = failures.join("\n");
  expect(message).toContain("public banlist");
  expect(message).toContain("production readiness");
});

test("fails missing verifier order", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "scripts/verify.sh",
        "bun test scripts/check-phase96-peer-policy-runtime-bridge.test.ts",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("check-phase96-peer-policy-runtime-bridge.test.ts");
});

test("fails duplicate canonical v1.9 requirement ownership", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "docs/parity/index.json",
        '"requirements":[]',
        '"requirements":["EVICT-03","EVICT-04","DOS-03"]',
      );
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("must not duplicate canonical v1.9 ownership");
});

test("fails missing structured log and breadcrumb evidence", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "packages/open-bitcoin-node/src/logging.rs", "INBOUND_PEER_POLICY_LOG_SOURCE", "");
      replaceInFile(files, "packages/open-bitcoin-rpc/src/context/peer_policy.rs", "packages/bitcoin-knots/src/banman.cpp", "");
    },
  });

  // Act
  const failures = checkPhase96PeerPolicyRuntimeBridge({ rootDir: root });

  // Assert
  const message = failures.join("\n");
  expect(message).toContain("INBOUND_PEER_POLICY_LOG_SOURCE");
  expect(message).toContain("packages/bitcoin-knots/src/banman.cpp");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase96-"));
  tempRoots.push(root);
  const files = completeFiles();
  options.maybeMutateFiles?.(files);
  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}

function completeFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    [
      "packages/open-bitcoin-network/src/peer_policy.rs",
      `
pub const MAX_PEER_POLICY_RUNTIME_DECISIONS: usize = 32;
pub enum BanScope {}
impl BanScope { pub fn matches_ip(&self, remote_ip: IpAddr) -> bool { true } }
pub struct PeerPolicyRuntimeState;
impl PeerPolicyRuntimeState {
  pub fn reconnect_suppression_input_for_ip(&self) {}
  pub fn misbehavior_decisions(&self) {}
  pub fn ban_decisions(&self) {}
  pub fn unban_decisions(&self) {}
}
`,
    ],
    [
      "packages/open-bitcoin-node/src/network.rs",
      "mod peer_policy;",
    ],
    [
      "packages/open-bitcoin-node/src/network/peer_policy.rs",
      `
pub fn peer_policy_info(&self) {
  ManagedPeerPolicyInfo::from_policy_decisions(
    eviction_candidate_count,
    Some(self.peer_manager.eviction_decision()),
    peer_policy_runtime_state.misbehavior_decisions(),
    peer_policy_runtime_state.ban_decisions(),
    peer_policy_runtime_state.unban_decisions(),
  )
}
pub fn record_peer_policy_ban() {}
pub fn record_peer_policy_unban() {}
pub fn record_peer_policy_misbehavior() {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/logging.rs",
      `
pub const INBOUND_PEER_POLICY_LOG_SOURCE: &str = "inbound_peer_policy";
const REDACTED_PEER_POLICY_FIELD: &str = "redacted_peer_policy_field";
pub fn inbound_peer_policy_log_record() {}
fn sanitizer() {
  let _ = "peer_id= raw_endpoint permission_string payload_bytes credential secret cookie=";
}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/context/network.rs",
      `
pub fn reconnect_suppression_input_for_remote_addr(remote_addr: SocketAddr, now_unix_seconds: i64) {
  self.network.reconnect_suppression_input_for_ip(remote_addr.ip(), now_unix_seconds)
}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
      `
// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp
pub fn record_inbound_peer_policy_event_at() { append_structured_log_record(); }
`,
    ],
    [
      "packages/open-bitcoin-cli/src/operator/status/render/inbound.rs",
      "peer policy evidence: active_bans={} manual_unbans={} protected_no_actions={}",
    ],
    [
      "packages/open-bitcoin-cli/src/operator/support/render/inbound.rs",
      "## Inbound Peer Policy Evidence\nconst PHASE96_PEER_POLICY_RUNTIME_BRIDGE_NEXT_ACTION: &str = \"Treat Phase 96 as scoped runtime peer-policy bridge evidence only; review ban, discourage, unban, and misbehavior labels before changing listener exposure or peer policy.\";",
    ],
    [
      "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
      "fn contains_raw_address_evidence() { let _ = \"peer_id= raw_endpoint permission_string payload_bytes credential secret cookie=\"; }",
    ],
    [
      "docs/operator/runtime-guide.md",
      `
Phase 96 scoped runtime peer-policy bridge evidence records bounded reconnect suppression and is not a public banlist or production participation claim.
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json
`,
    ],
    [
      "docs/parity/catalog/p2p.md",
      `
## Phase 96 peer-policy runtime bridge
The v1-9-peer-policy-runtime-bridge surface covers EVICT-03, EVICT-04, and DOS-03 with scoped runtime peer-policy bridge evidence, bounded reconnect suppression, and is not a public banlist or production participation claim.
packages/bitcoin-knots/src/net.cpp packages/bitcoin-knots/src/net_processing.cpp packages/bitcoin-knots/src/banman.h packages/bitcoin-knots/src/banman.cpp packages/bitcoin-knots/src/net_permissions.cpp
`,
    ],
    [
      "docs/parity/index.json",
      JSON.stringify({
        surfaces: [{ name: "v1-9-peer-policy-runtime-bridge", status: "done" }],
        checklist: {
          surfaces: [
            {
              id: "v1-9-peer-policy-runtime-bridge",
              status: "done",
              requirements: [],
              upstream: {
                sources: [
                  "packages/bitcoin-knots/src/net.cpp",
                  "packages/bitcoin-knots/src/net_processing.cpp",
                  "packages/bitcoin-knots/src/banman.h",
                  "packages/bitcoin-knots/src/banman.cpp",
                  "packages/bitcoin-knots/src/net_permissions.cpp",
                ],
              },
            },
          ],
        },
      }),
    ],
    [
      "docs/parity/source-breadcrumbs.json",
      JSON.stringify({
        groups: [
          {
            files: [
              "packages/open-bitcoin-node/src/network/peer_policy.rs",
              "packages/open-bitcoin-rpc/src/context/peer_policy.rs",
            ],
            breadcrumbs: ["packages/bitcoin-knots/src/banman.cpp"],
          },
        ],
      }),
    ],
    [
      "scripts/verify.sh",
      `
# Phase 94 is followed by Phase 95. Phase 95 is followed by Phase 96.
bun test scripts/check-phase95-network-participation-release-boundary.test.ts
bun run scripts/check-phase95-network-participation-release-boundary.ts
bun test scripts/check-phase96-peer-policy-runtime-bridge.test.ts
bun run scripts/check-phase96-peer-policy-runtime-bridge.ts
run_step "Phase 95 network participation release boundary checker" bun run scripts/check-phase95-network-participation-release-boundary.ts
run_step "Phase 96 peer-policy runtime bridge checker tests" bun test scripts/check-phase96-peer-policy-runtime-bridge.test.ts
run_step "Phase 96 peer-policy runtime bridge checker" bun run scripts/check-phase96-peer-policy-runtime-bridge.ts
`,
    ],
  ]);
}

function replaceInFile(
  files: Map<TargetFile, string>,
  file: TargetFile,
  needle: string,
  replacement: string,
): void {
  files.set(file, (files.get(file) ?? "").replace(needle, replacement));
}

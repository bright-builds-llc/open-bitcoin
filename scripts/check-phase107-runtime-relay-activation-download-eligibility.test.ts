import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase107RuntimeRelayActivationDownloadEligibility } from "./check-phase107-runtime-relay-activation-download-eligibility";

const TARGET_FILES = [
  "README.md",
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-node/src/network/relay_serving.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-network/src/peer/relay_download.rs",
  "packages/open-bitcoin-network/src/peer/inventory_state.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.ts",
  "scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts",
  "scripts/verify.sh",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-RESEARCH.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-01-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-03-SUMMARY.md",
  ".planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-04-SUMMARY.md",
] as const;
const REQUIRED_REQUIREMENTS = ["ACT-01", "ACT-02", "INV-02", "INV-03", "DL-01", "DL-02", "REL-03"] as const;

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

test("passes_when_phase107_runtime_activation_download_eligibility_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_runtime_context_drops_relay_activation_config", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "packages/open-bitcoin-rpc/src/context/network.rs", "config.relay,", "RelayActivationConfig::default(),");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("config.relay");
});

test("fails_when_runtime_context_drops_inbound_serving_config", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "packages/open-bitcoin-rpc/src/context/network.rs", "config.inbound.enabled,", "false,");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("config.inbound.enabled");
});

test("fails_when_runtime_context_uses_default_managed_network_constructor", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-rpc/src/context/network.rs",
        "ManagedPeerNetwork::new_with_relay_activation(",
        "ManagedPeerNetwork::new(",
      );
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("new_with_relay_activation");
});

test("fails_when_a_required_scheduler_suppression_label_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "packages/open-bitcoin-network/src/peer/transaction_relay.rs", "protected_not_relay");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("protected_not_relay");
});

test("fails_when_scheduler_can_mutate_in_flight_state_before_eligibility_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs",
        "if let Some(action) =\n            relay_eligibility_suppression(input.peer_id, relay_id, &input.relay_eligibility)",
        "self.insert_in_flight(relay_id, input.peer_id, input.now_unix_seconds);\n        if let Some(action) =\n            relay_eligibility_suppression(input.peer_id, relay_id, &input.relay_eligibility)",
      );
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("before insert_in_flight");
});

test("fails_when_status_evidence_field_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "packages/open-bitcoin-node/src/status/relay_evidence.rs", "RelayDownloadEligibilityCounters");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("RelayDownloadEligibilityCounters");
});

test("fails_when_runtime_uat_repo_local_commands_are_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(
        files,
        "docs/operator/runtime-guide.md",
        "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --",
      );
      removeFromFile(files, "docs/operator/runtime-guide.md", "bazel run //packages/open-bitcoin-rpc:open_bitcoind --");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("runtime guide command");
});

test("fails_when_docs_parity_surface_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromAllFiles(files, "v2-0-runtime-relay-activation-download-eligibility");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("v2-0-runtime-relay-activation-download-eligibility");
});

test("fails_when_verifier_wiring_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "scripts/verify.sh", "bun run scripts/check-phase107-runtime-relay-activation-download-eligibility.ts");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("verifier-scope");
});

test("fails_when_docs_claim_forbidden_public_or_production_scope", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(files, "README.md", "Phase 107 supports compact block relay.");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("forbidden positive Phase 107 claim");
});

test("fails_when_default_verifier_adds_public_network_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(files, "scripts/verify.sh", 'run_step "public-network relay CI" bun run scripts/run-live-mainnet-smoke.ts');
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("default verifier must not run public-network");
});

test("fails_when_public_evidence_contains_sensitive_material", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/architecture/operator-observability.md",
        "Phase 107 operator evidence exposes txid=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa and peer_id=107.",
      );
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("sensitive public evidence");
});

test("fails_when_aggregate_sanitized_status_decision_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "docs/operator/runtime-guide.md", "aggregate, sanitized, and fixed-label only");
    },
  });

  // Act
  const failures = checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n");

  // Assert
  expect(failures).toContain("aggregate sanitized");
});

test("fails_when_any_phase107_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase107RuntimeRelayActivationDownloadEligibility(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase107-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const filePath of TARGET_FILES) {
    files.set(filePath, readFileSync(filePath, "utf8"));
  }
  options.maybeMutateFiles?.(files);
  for (const [filePath, content] of files.entries()) {
    const absolutePath = path.join(root, filePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, content);
  }

  return root;
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const filePath of TARGET_FILES) {
    removeFromFile(files, filePath, needle);
  }
}

function removeFromFile(files: Map<TargetFile, string>, filePath: TargetFile, needle: string): void {
  files.set(filePath, (files.get(filePath) ?? "").replaceAll(needle, ""));
}

function replaceInFile(
  files: Map<TargetFile, string>,
  filePath: TargetFile,
  oldText: string,
  newText: string,
): void {
  files.set(filePath, (files.get(filePath) ?? "").replace(oldText, newText));
}

function appendToFile(files: Map<TargetFile, string>, filePath: TargetFile, text: string): void {
  files.set(filePath, `${files.get(filePath) ?? ""}\n${text}\n`);
}

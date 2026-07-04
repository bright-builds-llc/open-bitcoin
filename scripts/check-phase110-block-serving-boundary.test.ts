import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase110BlockServingBoundary } from "./check-phase110-block-serving-boundary";

const SURFACE_ID = "v2-1-block-serving-activation-eligibility-boundary";
const REQUIRED_REQUIREMENTS = ["BSRV-01", "BSRV-02", "BSRV-03", "BSRV-05", "BSRV-06"] as const;
const PHASE110_TEST_COMMAND =
  "bun test scripts/check-phase110-block-serving-boundary.test.ts";
const PHASE110_CHECKER_COMMAND =
  "bun run scripts/check-phase110-block-serving-boundary.ts";
const TARGET_FILES = [
  "docs/architecture/config-precedence.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
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

test("passes_when_phase110_fixture_contains_boundary_roots_and_verify_wiring", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase110BlockServingBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_bsrv_requirement_is_missing_from_parity_roots", () => {
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
    checkPhase110BlockServingBoundary(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_required_config_symbols_labels_or_knots_anchors_are_missing", () => {
  // Arrange
  const missingTerms = [
    "block_serving.compact_relay_enabled",
    "-openbitcoincompactrelay",
    "BlockServingEvidenceStatus",
    "block_request_cap_reached",
    "block_inflight_limit_still_reached",
    "packages/bitcoin-knots/src/node/blockstorage.cpp",
  ];
  const roots = missingTerms.map((missingTerm) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, missingTerm);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase110BlockServingBoundary(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message.length).toBeGreaterThan(0);
  }
});

test("fails_when_docs_claim_public_default_archive_bip152_or_response_support", () => {
  // Arrange
  const claims = [
    "Phase 110 supports public block serving by default.",
    "Phase 110 provides archive-node behavior.",
    "Phase 110 implements BIP152 codecs.",
    "Phase 110 adds full block serving responses.",
    "Phase 110 enables package relay support.",
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
    checkPhase110BlockServingBoundary(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("forbidden Phase 110 positive claim");
  }
});

test("allows_explicit_no_claim_or_deferred_wording_for_out_of_scope_surfaces", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/parity/catalog/p2p.md",
        "Phase 110 does not add full block serving responses, BIP152 codecs, compact reconstruction, getblocktxn, blocktxn, package relay, public block serving by default, archive-node behavior, or production full-node readiness.",
      );
    },
  });

  // Act
  const failures = checkPhase110BlockServingBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_default_verifier_wiring_is_missing_or_non_deterministic", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        replaceInFile(files, "scripts/verify.sh", PHASE110_TEST_COMMAND, "");
        replaceInFile(files, "scripts/verify.sh", PHASE110_CHECKER_COMMAND, "");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 110 public-network CI" true');
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 110 service-manager" systemctl status open-bitcoind');
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase110BlockServingBoundary(root).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("default verifier");
  expect(failureMessages[1]).toContain("forbidden Phase 110 default verifier gate");
  expect(failureMessages[2]).toContain("forbidden Phase 110 default verifier gate");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase110-"));
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
    ["docs/architecture/status-snapshot.md", evidenceText()],
    ["docs/architecture/operator-observability.md", evidenceText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/index.json", parityIndexText()],
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
    "Phase 110 uses default-off `block_serving.enabled`, `block_serving.compact_relay_enabled`, `-openbitcoinblockserving`, and `-openbitcoincompactrelay`.",
    "These settings feed `BlockRelayActivationPolicy` only.",
    noClaimText(),
  ].join("\n");
}

function evidenceText(): string {
  return [
    "# Phase 110 Evidence",
    "The shared contract is `BlockServingEvidenceStatus`.",
    "Policy functions are `classify_block_serving_eligibility`, `classify_block_serving_status`, `evaluate_block_serving_resource_gate`, and `classify_block_inflight_cleanup`.",
    labelsText(),
    noClaimText(),
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "# Runtime Guide",
    "Review Phase 110 with repo-local commands:",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
    "Public-network block-serving or compact-relay review is opt-in UAT guidance only and remains outside `bash scripts/verify.sh`.",
    evidenceText(),
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Catalog",
    `${SURFACE_ID} covers ${REQUIRED_REQUIREMENTS.join(", ")}.`,
    evidenceText(),
    "Knots anchors: packages/bitcoin-knots/src/net_permissions.h packages/bitcoin-knots/src/net_permissions.cpp packages/bitcoin-knots/src/net.cpp packages/bitcoin-knots/src/net_processing.cpp packages/bitcoin-knots/src/validation.cpp packages/bitcoin-knots/src/node/blockstorage.cpp packages/bitcoin-knots/test/functional/p2p_getdata.py packages/bitcoin-knots/test/functional/p2p_permissions.py.",
  ].join("\n");
}

function checklistText(): string {
  return [
    "# Checklist",
    `| \`${SURFACE_ID}\` | done | ${REQUIRED_REQUIREMENTS.join(", ")} | docs/parity/index.json | ${labelsText()} ${noClaimText()} | Future phases own BIP152 codecs. |`,
  ].join("\n");
}

function labelsText(): string {
  return [
    "Terms: `block_serving.enabled`, `block_serving.compact_relay_enabled`, `-openbitcoinblockserving`, `-openbitcoincompactrelay`, `BlockRelayActivationPolicy`, `BlockServingEvidenceStatus`, `eligible`, `disabled`, `activation_required`, `inbound_serving_required`, `permission_required`, `protected_not_serving`, `status_unavailable`, `permission_effect_inactive`, `validated`, `available`, `stale`, `side_chain`, `pruned`, `unavailable`, `unvalidated`, `unknown`, `suppressed`, `block_request_cap_reached`, `block_inflight_cleanup_released`, `block_inflight_cleanup_peer_removed`, `block_inflight_cleanup_timeout`, `block_inflight_cleanup_restart`, and `block_inflight_limit_still_reached`.",
  ].join("\n");
}

function noClaimText(): string {
  return "Phase 110 does not add full block serving responses, BIP152 implementation, compact reconstruction, getblocktxn, blocktxn, archive-node behavior, package relay, bloom/filter serving, compact filter serving, public block serving by default, public-network CI, production service operation, production full-node readiness, or production-funds wallet use.";
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            title: "v2.1 Block Serving Activation and Eligibility Boundary",
            status: "done",
            requirements: REQUIRED_REQUIREMENTS,
            evidence: [
              "docs/architecture/config-precedence.md",
              "docs/architecture/status-snapshot.md",
              "docs/architecture/operator-observability.md",
              "docs/operator/runtime-guide.md",
              "docs/parity/catalog/p2p.md",
              "docs/parity/checklist.md",
              "docs/parity/index.json",
              "packages/open-bitcoin-network/src/block_serving.rs",
              "packages/open-bitcoin-network/src/block_serving/tests.rs",
              "packages/open-bitcoin-rpc/src/config/open_bitcoin.rs",
              "packages/open-bitcoin-rpc/src/config/loader/block_serving.rs",
              "packages/open-bitcoin-node/src/status/block_serving.rs",
              "packages/open-bitcoin-node/src/status/block_serving/tests.rs",
              "packages/open-bitcoin-network/src/peer/tests.rs",
              "packages/open-bitcoin-node/src/sync/tests.rs",
              "scripts/check-phase110-block-serving-boundary.ts",
              "scripts/check-phase110-block-serving-boundary.test.ts",
              "scripts/verify.sh",
            ],
            rationale: `${evidenceText()} ${labelsText()}`,
            upstream: {
              sources: [
                "packages/bitcoin-knots/src/net_permissions.h",
                "packages/bitcoin-knots/src/net_permissions.cpp",
                "packages/bitcoin-knots/src/net.cpp",
                "packages/bitcoin-knots/src/net_processing.cpp",
                "packages/bitcoin-knots/src/validation.cpp",
                "packages/bitcoin-knots/src/node/blockstorage.cpp",
              ],
              tests: [
                "packages/bitcoin-knots/test/functional/p2p_getdata.py",
                "packages/bitcoin-knots/test/functional/p2p_permissions.py",
              ],
            },
            known_gaps: [noClaimText()],
          },
        ],
      },
    },
    null,
    2,
  );
}

function verifyScriptText(): string {
  return [
    "# Phase 108 is followed by Phase 110.",
    ": <<'VERIFY_COMMAND_ORDER'",
    "bun test scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts",
    "bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts",
    PHASE110_TEST_COMMAND,
    PHASE110_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    'run_step "test Phase 108 durable mempool relay state recovery checker" bun test scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts',
    'run_step "check Phase 108 durable mempool relay state recovery" bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts',
    `run_step "test Phase 110 block-serving boundary checker" ${PHASE110_TEST_COMMAND}`,
    `run_step "check Phase 110 block-serving boundary" ${PHASE110_CHECKER_COMMAND}`,
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
}

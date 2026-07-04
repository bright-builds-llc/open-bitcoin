import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase111FullBlockServingRequestPath } from "./check-phase111-full-block-serving-request-path";

const SURFACE_ID = "v2-1-full-block-serving-request-path";
const REQUIRED_REQUIREMENTS = ["BSRV-04", "GOV-01", "GOV-05"] as const;
const PHASE111_TEST_COMMAND =
  "bun test scripts/check-phase111-full-block-serving-request-path.test.ts";
const PHASE111_CHECKER_COMMAND =
  "bun run scripts/check-phase111-full-block-serving-request-path.ts";
const TARGET_FILES = [
  "docs/architecture/status-snapshot.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/network/block_serving.rs",
  "packages/open-bitcoin-node/src/network/inventory.rs",
  "packages/open-bitcoin-node/src/network/tests.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
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

test("passes_when_phase111_fixture_contains_request_path_roots_and_verify_wiring", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase111FullBlockServingRequestPath(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase111_requirement_is_missing_from_parity_roots", () => {
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
    checkPhase111FullBlockServingRequestPath(root).join("\n"),
  );

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_required_inventory_terms_tests_or_knots_anchors_are_missing", () => {
  // Arrange
  const missingTerms = [
    "InventoryType::WitnessBlock",
    "WireNetworkMessage::NotFound",
    "block_status_pruned",
    "phase111_permissioned_block_getdata_still_hits_request_cap",
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
    checkPhase111FullBlockServingRequestPath(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message.length).toBeGreaterThan(0);
  }
});

test("fails_when_docs_claim_compact_block_archive_public_or_production_support", () => {
  // Arrange
  const claims = [
    "Phase 111 supports BIP152 compact block payload serving.",
    "Phase 111 provides archive-node behavior.",
    "Phase 111 implements getblocktxn and blocktxn.",
    "Phase 111 adds package relay support.",
    "Phase 111 ships production full-node readiness.",
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
    checkPhase111FullBlockServingRequestPath(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("forbidden Phase 111 positive claim");
  }
});

test("allows_explicit_no_claim_wording_for_out_of_scope_surfaces", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/parity/catalog/p2p.md",
        "Phase 111 does not add BIP152 compact block payload serving, compact reconstruction, getblocktxn, blocktxn, archive-node behavior, package relay, bloom/filter serving, compact filter serving, public block serving by default, public-network CI, production service operation, production full-node readiness, production-funds wallet use, or schema/ORM work.",
      );
    },
  });

  // Act
  const failures = checkPhase111FullBlockServingRequestPath(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_runtime_commands_or_default_verifier_wiring_are_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(
          files,
          "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...",
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        replaceInFile(files, "scripts/verify.sh", PHASE111_TEST_COMMAND, "");
        replaceInFile(files, "scripts/verify.sh", PHASE111_CHECKER_COMMAND, "");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "scripts/verify.sh", 'run_step "Phase 111 public-network CI" true');
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase111FullBlockServingRequestPath(root).join("\n"),
  );

  // Assert
  expect(failureMessages[0]).toContain("runtime guide");
  expect(failureMessages[1]).toContain("default verifier");
  expect(failureMessages[2]).toContain("forbidden Phase 111 default verifier gate");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase111-"));
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
    ["docs/architecture/status-snapshot.md", evidenceText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/index.json", parityIndexText()],
    ["docs/parity/source-breadcrumbs.json", breadcrumbsText()],
    ["packages/open-bitcoin-node/src/network/block_serving.rs", adapterSourceText()],
    ["packages/open-bitcoin-node/src/network/inventory.rs", inventorySourceText()],
    ["packages/open-bitcoin-node/src/network/tests.rs", nodeTestsText()],
    ["packages/open-bitcoin-network/src/peer/tests.rs", peerTestsText()],
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

function evidenceText(): string {
  return [
    "# Phase 111 Evidence",
    `${SURFACE_ID} covers ${REQUIRED_REQUIREMENTS.join(", ")}.`,
    "The managed adapter uses `ManagedBlockServeInput`, `serve_managed_block_request`, and lazy `lookup_block` after policy gates.",
    "It handles `InventoryType::Block`, `InventoryType::WitnessBlock`, and `InventoryType::CompactBlock` with `WireNetworkMessage::Block` or `WireNetworkMessage::NotFound`.",
    "Bounded labels include `block_status_pruned`, `block_status_unavailable`, and `block_request_cap_reached`.",
    knotsAnchorsText(),
    noClaimText(),
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "# Runtime Guide",
    "Phase 111 review uses repo-local command forms:",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...",
    "Keep `bash scripts/verify.sh` deterministic.",
    evidenceText(),
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Catalog",
    evidenceText(),
    "Evidence files include docs/parity/source-breadcrumbs.json.",
  ].join("\n");
}

function checklistText(): string {
  return [
    "# Checklist",
    `| \`${SURFACE_ID}\` | done | ${REQUIRED_REQUIREMENTS.join(", ")} | docs/parity/index.json | ${evidenceText()} | ${noClaimText()} |`,
  ].join("\n");
}

function breadcrumbsText(): string {
  return [
    "packages/open-bitcoin-node/src/network/block_serving.rs",
    knotsAnchorsText(),
  ].join("\n");
}

function adapterSourceText(): string {
  return [
    "struct ManagedBlockServeInput;",
    "fn serve_managed_block_request(input: ManagedBlockServeInput, lookup_block: impl FnOnce()) {}",
    "phase111_recent_valid_available_block_is_served_after_policy_gate",
    "phase111_stale_block_fact_returns_unavailable_notfound_without_lookup",
    "block_status_pruned block_status_unavailable",
  ].join("\n");
}

function inventorySourceText(): string {
  return [
    "InventoryType::Block InventoryType::WitnessBlock InventoryType::CompactBlock",
    "WireNetworkMessage::Block WireNetworkMessage::NotFound",
    "lookup_block",
  ].join("\n");
}

function nodeTestsText(): string {
  return [
    "phase111_side_chain_cached_block_is_not_served",
    "phase111_active_chain_non_tip_missing_local_block_returns_pruned_notfound",
    "phase111_active_tip_missing_local_block_returns_unavailable_notfound",
    "phase111_cached_old_block_outside_active_chain_is_not_archive_served",
    "phase111_managed_getdata_over_request_cap_disconnects_without_block_payload",
    "phase111_permissioned_block_getdata_still_hits_request_cap",
    "WireNetworkMessage::Block WireNetworkMessage::NotFound block_request_cap_reached",
  ].join("\n");
}

function peerTestsText(): string {
  return [
    "phase111_full_witness_block_cleanup_matrix_uses_phase110_labels",
    "phase111_compact_block_burst_remains_bounded_without_partial_state",
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
            title: "v2.1 Full Block Serving Request Path",
            status: "done",
            requirements: REQUIRED_REQUIREMENTS,
            evidence: [
              "docs/architecture/status-snapshot.md",
              "docs/operator/runtime-guide.md",
              "docs/parity/catalog/p2p.md",
              "docs/parity/checklist.md",
              "docs/parity/index.json",
              "docs/parity/source-breadcrumbs.json",
              "packages/open-bitcoin-node/src/network/block_serving.rs",
              "packages/open-bitcoin-node/src/network/inventory.rs",
              "packages/open-bitcoin-node/src/network/tests.rs",
              "packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs",
              "packages/open-bitcoin-network/src/peer/inventory_state.rs",
              "packages/open-bitcoin-network/src/peer/tests.rs",
              "scripts/check-phase111-full-block-serving-request-path.ts",
              "scripts/check-phase111-full-block-serving-request-path.test.ts",
              "scripts/verify.sh",
              ".planning/phases/111-full-block-serving-request-path/111-01-SUMMARY.md",
              ".planning/phases/111-full-block-serving-request-path/111-02-SUMMARY.md",
              ".planning/phases/111-full-block-serving-request-path/111-03-SUMMARY.md",
            ],
            rationale: evidenceText(),
            upstream: {
              sources: [
                "packages/bitcoin-knots/src/net_processing.cpp",
                "packages/bitcoin-knots/src/node/blockstorage.cpp",
                "packages/bitcoin-knots/src/validation.cpp",
              ],
              tests: ["packages/bitcoin-knots/test/functional/p2p_getdata.py"],
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
    "# Phase 108 is followed by Phase 110 and Phase 111.",
    ": <<'VERIFY_COMMAND_ORDER'",
    "bun test scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts",
    "bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts",
    "bun test scripts/check-phase110-block-serving-boundary.test.ts",
    "bun run scripts/check-phase110-block-serving-boundary.ts",
    PHASE111_TEST_COMMAND,
    PHASE111_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    'run_step "test Phase 110 block-serving boundary checker" bun test scripts/check-phase110-block-serving-boundary.test.ts',
    'run_step "check Phase 110 block-serving boundary" bun run scripts/check-phase110-block-serving-boundary.ts',
    `run_step "test Phase 111 full block-serving request path checker" ${PHASE111_TEST_COMMAND}`,
    `run_step "check Phase 111 full block-serving request path" ${PHASE111_CHECKER_COMMAND}`,
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
}

function knotsAnchorsText(): string {
  return "Knots anchors: packages/bitcoin-knots/src/net_processing.cpp packages/bitcoin-knots/src/node/blockstorage.cpp packages/bitcoin-knots/src/validation.cpp packages/bitcoin-knots/test/functional/p2p_getdata.py.";
}

function noClaimText(): string {
  return "Phase 111 does not add BIP152 compact block payload serving, compact reconstruction, getblocktxn, blocktxn, archive-node behavior, package relay, bloom/filter serving, compact filter serving, public block serving by default, public-network CI, production service operation, production full-node readiness, production-funds wallet use, or schema/ORM work.";
}

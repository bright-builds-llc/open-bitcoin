import { afterEach, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  PHASE127_DAEMON_HELPER_DIR,
  PHASE127_TARGET_FILES,
} from "./check-phase127-authoritative-network-state-unification";
import { PHASE128_TARGET_FILES } from "./check-phase128-production-compact-announcement-transport";
import {
  PHASE129_TARGET_FILES,
  checkPhase129IntegrationGuardrailsAndMilestoneReconciliation,
} from "./check-phase129-integration-guardrails-and-milestone-reconciliation";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const ARCHIVED_V21_ROADMAP = ".planning/milestones/v2.1-ROADMAP.md";
type Mutator = (files: Map<string, string>) => void;
const tempRoots: string[] = [];

const FIXTURE_FILES = [
  ...new Set<string>([
    ...PHASE127_TARGET_FILES,
    ...PHASE128_TARGET_FILES,
    ...PHASE129_TARGET_FILES,
    ...daemonHelperFiles(),
  ]),
];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 129 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures =
    checkPhase129IntegrationGuardrailsAndMilestoneReconciliation(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "FLOW-01 production composition anchor",
    "P129 FLOW-01: durable validated block to inbound serving production composition anchor is missing",
    replace(
      "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
      "phase127_production_composition_shares_sync_serving_and_operator_authority",
      "renamed_anchor",
    ),
  ],
  [
    "FLOW-04 production composition anchor",
    "P129 FLOW-04: authoritative sync runtime to RPC/CLI/dashboard/support production composition anchor is missing",
    replace(
      "packages/open-bitcoin-rpc/tests/black_box_parity.rs",
      "phase127_production_composition_shares_sync_serving_and_operator_authority",
      "renamed_anchor",
    ),
  ],
  [
    "FLOW-02 live-fact fanout anchor",
    "P129 FLOW-02: handshake to bilateral negotiation to live header-aware announcement anchor is missing",
    replace(
      "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
      "production_announcement_transport_cases_fanout_uses_live_peer_facts",
      "renamed_anchor",
    ),
  ],
  [
    "FLOW-03 transport post-write anchor",
    "P129 FLOW-03: high-bandwidth decision to wire emission to post-write evidence anchor is missing",
    replace(
      "packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
      "production_announcement_transport_cases_partial_failure_credits_only_prefix_and_redacts",
      "renamed_anchor",
    ),
  ],
  [
    "FLOW-03 achieved-effect unit anchor",
    "P129 FLOW-03: post-write-only achieved-effect unit anchors are missing",
    replace(
      "packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs",
      "compact_success_receipt_records_achieved_effect_once",
      "renamed_anchor",
    ),
  ],
  [
    "FLOW-04 operator surface",
    "P129 FLOW-04: CLI operator surface test files are missing",
    (files: Map<string, string>) =>
      files.set("packages/open-bitcoin-cli/tests/operator_flows.rs", ""),
  ],
  [
    "verifier heredoc wiring",
    "P129 verifier heredoc: Phase 129 pair must run between Phase 128 and the Phase 117 gate",
    replace(
      "scripts/verify.sh",
      "bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts\nbun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
      "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
    ),
  ],
  [
    "verifier run_step wiring",
    "P129 verifier run_step: Phase 129 pair must run between Phase 128 and the Phase 117 gate",
    replace(
      "scripts/verify.sh",
      'run_step "check Phase 129 integration guardrails and milestone reconciliation" bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts\n',
      "",
    ),
  ],
  [
    "final-gate ordering",
    "P129 final gate run_step order must end with bun run scripts/check-phase117-parity-uat-release-boundary.ts",
    append(
      "scripts/verify.sh",
      'run_step "check Phase 130 placeholder" bun run scripts/check-phase130-placeholder.ts',
    ),
  ],
  [
    "composed Phase 127 shared-authority seam",
    "P127 production authority: daemon must compose sync, inbound, and RPC from one authoritative runtime",
    replace(
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      "authoritative_runtime.network.clone(),",
      [
        "ManagedNetworkHandle::transient_runtime(/* duplicate authority */),",
        "        // authoritative_runtime.network.clone(),",
      ].join("\n"),
    ),
  ],
  [
    "composed Phase 128 local-offer seam",
    "P128 local offer: production handshake must schedule sendcmpct(false, version 2)",
    replace(
      "packages/open-bitcoin-network/src/peer/compact_relay.rs",
      "announce: false,",
      "announce: true,",
    ),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);

  // Act
  const failures =
    checkPhase129IntegrationGuardrailsAndMilestoneReconciliation(root);

  // Assert
  expect(failures).toContain(expectedFailure);
});

function daemonHelperFiles(): string[] {
  return rustSourceFiles(path.join(REPO_ROOT, PHASE127_DAEMON_HELPER_DIR)).map(
    (absolutePath) =>
      path
        .join(
          PHASE127_DAEMON_HELPER_DIR,
          path.relative(
            path.join(REPO_ROOT, PHASE127_DAEMON_HELPER_DIR),
            absolutePath,
          ),
        )
        .split(path.sep)
        .join("/"),
  );
}

function rustSourceFiles(directory: string): string[] {
  const paths: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const entryPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      paths.push(...rustSourceFiles(entryPath));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith(".rs")) paths.push(entryPath);
  }
  return paths;
}

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase129-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const file of FIXTURE_FILES) {
    files.set(file, readFileSync(path.join(REPO_ROOT, file), "utf8"));
  }
  maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, text);
  }
  const archivedRoadmapPath = path.join(root, ARCHIVED_V21_ROADMAP);
  mkdirSync(path.dirname(archivedRoadmapPath), { recursive: true });
  writeFileSync(
    archivedRoadmapPath,
    readFileSync(path.join(REPO_ROOT, ARCHIVED_V21_ROADMAP), "utf8"),
  );
  return root;
}

function replace(file: string, needle: string, replacement: string): Mutator {
  return (files) => {
    const text = files.get(file) ?? "";
    if (!text.includes(needle)) {
      throw new Error(`fixture needle missing in ${file}: ${needle}`);
    }
    files.set(file, text.replace(needle, replacement));
  };
}

function append(file: string, value: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}\n${value}\n`);
}

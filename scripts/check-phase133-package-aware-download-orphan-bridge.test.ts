import { afterEach, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  PHASE133_TARGET_FILES,
  checkPhase133PackageAwareDownloadOrphanBridge,
} from "./check-phase133-package-aware-download-orphan-bridge";
import { readSourceCorpus } from "./source-corpus";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
type Mutator = (files: Map<string, string>) => void;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 133 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase133PackageAwareDownloadOrphanBridge(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "reject evidence capacity",
    "P133 PPKG-01: reject evidence must retain independent fixed-memory 120000/0.000001 domains",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs",
      "PHASE133_REJECT_FILTER_CAPACITY: usize = 120_000",
      "PHASE133_REJECT_FILTER_CAPACITY: usize = 119_999",
    ),
  ],
  [
    "reject evidence false-positive target",
    "P133 PPKG-01: reject evidence must retain independent fixed-memory 120000/0.000001 domains",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs",
      "PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE: f64 = 0.000_001",
      "PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE: f64 = 0.000_01",
    ),
  ],
  [
    "million-insertion allocation oracle",
    "P133 PPKG-01: reject-evidence stress and domain-separation regression tests must remain",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs",
      "fixed_memory_allocation_survives_one_million_unique_insertions",
      "removed_allocation_oracle",
    ),
  ],
  [
    "active-tip hard reset",
    "P133 PPKG-01: active-tip changes must reset both evidence domains",
    replace(
      "packages/open-bitcoin-network/src/peer.rs",
      "self.hard_reject_evidence.reset(new_tweak);",
      "",
    ),
  ],
  [
    "suppression without punishment",
    "P133 PPKG-01: semantic reject evidence must suppress work without peer punishment",
    replace(
      "packages/open-bitcoin-network/src/peer/tests.rs",
      "peer_manager_transaction_relay_semantic_reject_evidence_suppresses_without_punishment",
      "removed_suppression_oracle",
    ),
  ],
  [
    "announcer bound",
    "P133 PPKG-02: orphan bodies, announcers, peer/global totals, and traversal must remain independently bounded",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
      "PHASE133_MAX_ANNOUNCERS_PER_ORPHAN: usize = 8",
      "PHASE133_MAX_ANNOUNCERS_PER_ORPHAN: usize = 80",
    ),
  ],
  [
    "late-announcer peer-cap bypass",
    "P133 PPKG-02: late announcers must respect per-peer and aggregate retained-byte bounds",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
      "if self.peer_len(peer_id) >= self.policy.max_orphans_per_peer {",
      "if false && self.peer_len(peer_id) >= self.policy.max_orphans_per_peer {",
    ),
  ],
  [
    "late-announcer peer-cap oracle",
    "P133 PPKG-02: adversarial bounds and coherent cleanup regression tests must remain",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
      "late_announcer_respects_per_peer_orphan_cap",
      "removed_late_announcer_peer_cap_oracle",
    ),
  ],
  [
    "aggregate retained-byte oracle",
    "P133 PPKG-02: adversarial bounds and coherent cleanup regression tests must remain",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
      "candidate_cursor_creation_respects_aggregate_retained_byte_budget",
      "removed_aggregate_retained_byte_oracle",
    ),
  ],
  [
    "candidate privacy",
    "P133 PPKG-02: candidate proof must stay private, canonical, same-peer, single-parent, traversal-bounded, and byte-bounded",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
      "pub(super) members: [Transaction; 2]",
      "pub members: [Transaction; 2]",
    ),
  ],
  [
    "cursor child-body retention",
    "P133 PPKG-02: persistent candidate cursors must retain one parent body and child identities only",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
      "pub(super) child_wtxids: Box<[Wtxid]>,",
      "pub(super) child_wtxids: Box<[Wtxid]>,\n    pub(super) child_transactions: Box<[Transaction]>,",
    ),
  ],
  [
    "identity-only cursor oracle",
    "P133 PPKG-02: adversarial bounds and coherent cleanup regression tests must remain",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
      "persistent_candidate_cursor_retains_child_identities_not_child_bodies",
      "removed_identity_only_cursor_oracle",
    ),
  ],
  [
    "canonical child lookup",
    "P133 PPKG-02: candidate proof must stay private, canonical, same-peer, single-parent, traversal-bounded, and byte-bounded",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
      "let entry = self.orphans.get(&child_wtxid)?;",
      "let entry = self.orphans.values().next()?;",
    ),
  ],
  [
    "single-parent predicate",
    "P133 PPKG-02: candidate proof must stay private, canonical, same-peer, single-parent, traversal-bounded, and byte-bounded",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
      "entry.missing_parents.len() == 1 && ",
      "",
    ),
  ],
  [
    "same-peer aligned origins",
    "P133 PPKG-02: candidate proof must stay private, canonical, same-peer, single-parent, traversal-bounded, and byte-bounded",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
      "origins: [cursor.parent_peer; 2]",
      "origins: [cursor.parent_peer, PeerId::new(99)]",
    ),
  ],
  [
    "adversarial traversal oracle",
    "P133 PPKG-02: adversarial bounds and coherent cleanup regression tests must remain",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
      "bounded_parent_traversal_stops_before_an_older_eligible_child",
      "removed_traversal_oracle",
    ),
  ],
  [
    "duplicate authoritative admission",
    "P133 PPKG-03: each eligible peer candidate must have exactly one node-owned package admission call",
    replace(
      "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
      "let submitted = self.mempool.submit_package(",
      "let _duplicate = self.mempool.submit_package(\n                SubmitPackageCommand { package: package.clone(), context: AdmissionContext::peer(PolicyTime::from_unix_seconds(options.timestamp)) },\n                &chainstate,\n                options.verify_flags,\n                options.consensus_params,\n            )?;\n            let submitted = self.mempool.submit_package(",
    ),
  ],
  [
    "cached package fingerprint",
    "P133 PPKG-03: the bridge must cache identity and return the exact authoritative report and delta",
    replace(
      "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
      "let fingerprint = *checked.fingerprint().as_bytes();",
      "let fingerprint = [0_u8; 32];",
    ),
  ],
  [
    "exhaustive member feedback",
    "P133 PPKG-03: package status and every member-result variant must receive typed feedback",
    replace(
      "packages/open-bitcoin-node/src/network/admission_bridge.rs",
      "PackageMemberResult::PostTrimAbsent",
      "PackageMemberResult::FinallyPresent",
    ),
  ],
  [
    "no-projection oracle",
    "P133 PPKG-03: exact-call, no-projection, feedback, fallback, and multi-parent suppression tests must remain",
    replace(
      "packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs",
      "child_first_neutral_candidate_has_one_submit_exact_report_and_fingerprint_with_no_projection",
      "removed_no_projection_oracle",
    ),
  ],
  [
    "typed singleton rejection category",
    "P133 PPKG-03: singleton policy failures must preserve their typed rejection category",
    replace(
      "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
      "HardMemberFailure::Policy { category, .. } => *category",
      "HardMemberFailure::Policy { .. } => MempoolRejectionCategory::InternalInvariant",
    ),
  ],
  [
    "typed singleton rejection oracle",
    "P133 PPKG-03: exact-call, no-projection, feedback, fallback, and multi-parent suppression tests must remain",
    replace(
      "packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs",
      "singleton_policy_failures_preserve_exact_rejection_categories",
      "removed_singleton_rejection_category_oracle",
    ),
  ],
  [
    "network mempool dependency",
    "P133 architecture: the network crate must remain neutral and must not depend on mempool admission",
    append(
      "packages/open-bitcoin-network/Cargo.toml",
      '\nopen-bitcoin-mempool = { path = "../open-bitcoin-mempool" }\n',
    ),
  ],
  [
    "reject evidence Knots anchor",
    "P133 parity: network-transaction-reject-evidence must retain exact Phase 133 files and the opportunistic 1P1C Knots anchor",
    replaceNth(
      "docs/parity/source-breadcrumbs.json",
      '"packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py"',
      '"packages/bitcoin-knots/test/functional/p2p_orphan_handling.py"',
      2,
    ),
  ],
  [
    "machine requirement ownership",
    "P133 docs: machine index must close one exact done surface for PPKG-01 through PPKG-03",
    replace(
      "docs/parity/index.json",
      '"PPKG-03"',
      '"PACK-03"',
    ),
  ],
  [
    "human checklist closure",
    "P133 docs: human checklist must record the done Phase 133 surface and exact requirements",
    replace(
      "docs/parity/checklist.md",
      "| `v2-2-package-aware-download-orphan-bridge` | `done` |",
      "| `v2-2-package-aware-download-orphan-bridge` | `in_progress` |",
    ),
  ],
  [
    "broad package-wire claim",
    "P133 claims: the bounded 1P1C bridge must not become a general wire, later-phase, public, guaranteed, or production claim",
    append(
      "README.md",
      "\nPhase 133 implements a general package wire for all peers.\n",
    ),
  ],
  [
    "verifier wiring",
    "P133 verifier: Phase 133 test/check must run after Phase 132 and before the final Phase 117 gate in both command surfaces",
    replace(
      "scripts/verify.sh",
      "bun run scripts/check-phase133-package-aware-download-orphan-bridge.ts",
      "bun run scripts/check-phase117-parity-uat-release-boundary.ts",
    ),
  ],
  [
    "filesystem-only checker",
    "P133 checker: verification must remain deterministic and filesystem-only",
    append(
      "scripts/check-phase133-package-aware-download-orphan-bridge.ts",
      "\nBun." + "spawn([\"git\", \"status\"]);\n",
    ),
  ],
])("rejects mutation: %s", (_name, expectedFailure, mutate) => {
  // Arrange
  const root = createFixture(mutate);

  // Act
  const failures = checkPhase133PackageAwareDownloadOrphanBridge(root);

  // Assert
  expect(failures).toContain(expectedFailure);
});

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase133-check-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const relativePath of PHASE133_TARGET_FILES) {
    files.set(
      relativePath,
      readSourceCorpus(REPO_ROOT, relativePath),
    );
  }
  maybeMutate?.(files);
  for (const [relativePath, contents] of files) {
    const destination = path.join(root, relativePath);
    mkdirSync(path.dirname(destination), { recursive: true });
    writeFileSync(destination, contents);
  }
  return root;
}

function replace(
  relativePath: string,
  search: string,
  replacement: string,
): Mutator {
  return (files) => {
    const source = requireFile(files, relativePath);
    expect(source).toContain(search);
    files.set(relativePath, source.replace(search, replacement));
  };
}

function replaceNth(
  relativePath: string,
  search: string,
  replacement: string,
  occurrence: number,
): Mutator {
  return (files) => {
    let source = requireFile(files, relativePath);
    let cursor = 0;
    for (let index = 1; index <= occurrence; index += 1) {
      const found = source.indexOf(search, cursor);
      expect(found).toBeGreaterThanOrEqual(0);
      if (index === occurrence) {
        source =
          source.slice(0, found) +
          replacement +
          source.slice(found + search.length);
        files.set(relativePath, source);
        return;
      }
      cursor = found + search.length;
    }
  };
}

function append(relativePath: string, addition: string): Mutator {
  return (files) => {
    files.set(relativePath, requireFile(files, relativePath) + addition);
  };
}

function requireFile(files: Map<string, string>, relativePath: string): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}

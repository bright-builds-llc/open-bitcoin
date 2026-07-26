#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE132_CHECK =
  "bun run scripts/check-phase132-typed-package-staged-admission.ts";
const PHASE133_TEST =
  "bun test scripts/check-phase133-package-aware-download-orphan-bridge.test.ts";
const PHASE133_CHECK =
  "bun run scripts/check-phase133-package-aware-download-orphan-bridge.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

export const PHASE133_TARGET_FILES = [
  "README.md",
  "packages/README.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-network/Cargo.toml",
  "packages/open-bitcoin-network/src/peer.rs",
  "packages/open-bitcoin-network/src/peer/tests.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs",
  "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
  "packages/open-bitcoin-node/src/network/admission_bridge.rs",
  "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs",
  "scripts/check-phase133-package-aware-download-orphan-bridge.ts",
  "scripts/verify.sh",
] as const;

export function checkPhase133PackageAwareDownloadOrphanBridge(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE133_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  checkRejectEvidence(repoRoot, failures);
  checkTipResetAndSuppression(repoRoot, failures);
  checkBoundedOrphanCandidate(repoRoot, failures);
  checkAuthoritativeNodeBridge(repoRoot, failures);
  checkCrateBoundary(repoRoot, failures);
  checkParityEvidence(repoRoot, failures);
  checkDocumentation(repoRoot, failures);
  checkNarrowClaims(repoRoot, failures);
  checkVerifierWiring(repoRoot, failures);
  checkDeterministicScope(repoRoot, failures);
  return failures;
}

function checkRejectEvidence(repoRoot: string, failures: string[]): void {
  const source = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs",
  );
  requireAll(
    source,
    [
      "pub const PHASE133_REJECT_FILTER_CAPACITY: usize = 120_000;",
      "pub const PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE: f64 = 0.000_001;",
      "const GENERATION_COUNT: usize = 3;",
      "const PHASE133_REJECT_FILTER_WORD_COUNT: usize = 161_750;",
      "pub struct HardRejectEvidence",
      "pub struct ReconsiderableRejectEvidence",
      "Transaction(Wtxid)",
      "Package([u8; 32])",
    ],
    "P133 PPKG-01: reject evidence must retain independent fixed-memory 120000/0.000001 domains",
    failures,
  );

  const tests = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs",
  );
  requireAll(
    tests,
    [
      "fixed_memory_allocation_survives_one_million_unique_insertions",
      "typed_domains_keep_transactions_and_packages_separate",
      "generation_rotation_retains_the_guaranteed_window_and_reuses_labels",
    ],
    "P133 PPKG-01: reject-evidence stress and domain-separation regression tests must remain",
    failures,
  );
}

function checkTipResetAndSuppression(
  repoRoot: string,
  failures: string[],
): void {
  const peer = readTarget(repoRoot, "packages/open-bitcoin-network/src/peer.rs");
  const reset = sectionBetween(
    peer,
    "pub fn on_active_tip_changed",
    "pub fn header_store",
  );
  requireAll(
    reset,
    [
      "self.hard_reject_evidence.reset(new_tweak);",
      "self.reconsiderable_reject_evidence.reset(new_tweak);",
    ],
    "P133 PPKG-01: active-tip changes must reset both evidence domains",
    failures,
  );

  const lifecycle = readTarget(
    repoRoot,
    "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  );
  const lifecycleTests = readTarget(
    repoRoot,
    "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  );
  if (countMatches(lifecycle, /\.on_active_tip_changed\(/g) < 2) {
    failures.push(
      "P133 PPKG-01: both local and stored active-tip seams must invoke the evidence reset",
    );
  }
  requireAll(
    lifecycleTests,
    [
      "local_active_tip_change_resets_both_reject_evidence_domains",
      "stored_connected_active_tip_change_resets_both_reject_evidence_domains",
    ],
    "P133 PPKG-01: active-tip reset behavior must retain both managed regression proofs",
    failures,
  );

  const peerTests = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/tests.rs",
  );
  if (
    !peerTests.includes(
      "peer_manager_transaction_relay_semantic_reject_evidence_suppresses_without_punishment",
    )
  ) {
    failures.push(
      "P133 PPKG-01: semantic reject evidence must suppress work without peer punishment",
    );
  }
}

function checkBoundedOrphanCandidate(
  repoRoot: string,
  failures: string[],
): void {
  const orphanage = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
  );
  requireAll(
    orphanage,
    [
      "pub const PHASE102_MAX_ORPHAN_TRANSACTIONS: usize = 100;",
      "pub const PHASE102_MAX_ORPHANS_PER_PEER: usize = 25;",
      "pub const PHASE133_MAX_ANNOUNCERS_PER_ORPHAN: usize = 8;",
      "pub const PHASE102_MAX_RECONSIDERATIONS_PER_PARENT: usize = 32;",
      "struct BoundedOrphanAnnouncers",
      "orphans: BTreeMap<Wtxid, OrphanEntry>",
      "candidate_cursors: BTreeMap<(Wtxid, PeerId), SamePeerCandidateCursor>",
    ],
    "P133 PPKG-02: orphan bodies, announcers, peer/global totals, and traversal must remain independently bounded",
    failures,
  );

  const candidate = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
  );
  requireAll(
    candidate,
    [
      "pub struct SamePeerOneParentOneChildCandidate",
      "pub(super) members: [Transaction; 2]",
      "pub(super) origins: [PeerId; 2]",
      "pub(super) provenances: [ReceivedTransactionProvenance; 2]",
      "pub fn into_ordered_parts_with_provenance(",
      "entry.missing_parents.len() == 1 && entry.announcers.contains(parent_peer)",
      "origins: [cursor.parent_peer; 2]",
      "while cursor.visited < self.policy.max_reconsiderations_per_parent",
    ],
    "P133 PPKG-02: candidate proof must stay private, consumable, same-peer, single-parent, and traversal-bounded",
    failures,
  );

  const tests = readTarget(
    repoRoot,
    "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
  );
  requireAll(
    tests,
    [
      "announcer_cap_keeps_one_shared_body_under_adversarial_peer_churn",
      "bounded_parent_traversal_stops_before_an_older_eligible_child",
      "coherent_disconnect_expiry_eviction_cleanup_preserves_index_oracle",
    ],
    "P133 PPKG-02: adversarial bounds and coherent cleanup regression tests must remain",
    failures,
  );
}

function checkAuthoritativeNodeBridge(
  repoRoot: string,
  failures: string[],
): void {
  const packageBridge = readTarget(
    repoRoot,
    "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
  );
  const candidateSubmission = sectionBetween(
    packageBridge,
    "fn submit_same_peer_candidate",
    "pub(super) fn record_singleton_reject_evidence",
  );
  if (countMatches(candidateSubmission, /\.submit_package\(/g) !== 1) {
    failures.push(
      "P133 PPKG-03: each eligible peer candidate must have exactly one node-owned package admission call",
    );
  }
  requireAll(
    candidateSubmission,
    [
      "let fingerprint = *checked.fingerprint().as_bytes();",
      ".reconsiderable_package_contains(fingerprint)",
      "debug_assert_eq!(submitted.report.fingerprint().as_bytes(), &fingerprint);",
      "self.apply_package_feedback(&members, &provenances, &submitted, options.timestamp);",
      "ManagedPeerPackageAdmission { origins, submitted }",
    ],
    "P133 PPKG-03: the bridge must cache identity and return the exact authoritative report and delta",
    failures,
  );

  const admissionBridge = readTarget(
    repoRoot,
    "packages/open-bitcoin-node/src/network/admission_bridge.rs",
  );
  const feedback = sectionBetween(
    admissionBridge,
    "fn apply_package_status_feedback",
    "#[cfg(test)]\n    pub(super) fn apply_package_member_feedback_for_test",
  );
  requireAll(
    feedback,
    [
      "PackageStatus::Complete",
      "PackageStatus::Partial | PackageStatus::Failed",
      "PackageMemberResult::FinallyPresent",
      "PackageMemberResult::AlreadyPresent",
      "PackageMemberResult::SameTxidDifferentWitness",
      "PackageMemberResult::HardRejected",
      "ReconsiderableMemberFailure::MissingInputs",
      "ReconsiderableMemberFailure::PackageFee",
      "ReconsiderableMemberFailure::PackageReplacement",
      "PackageMemberResult::PostTrimAbsent",
    ],
    "P133 PPKG-03: package status and every member-result variant must receive typed feedback",
    failures,
  );

  const tests = readTarget(
    repoRoot,
    "packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs",
  );
  requireAll(
    tests,
    [
      "child_first_neutral_candidate_has_one_submit_exact_report_and_fingerprint_with_no_projection",
      "two_reconsiderable_parents_suppress_multi_parent_package_submission",
      "every_feedback_variant_keeps_hard_reconsiderable_and_failed_fingerprint_domains_separate",
      "newest_failed_fingerprint_falls_back_once_to_older_eligible_child",
    ],
    "P133 PPKG-03: exact-call, no-projection, feedback, fallback, and multi-parent suppression tests must remain",
    failures,
  );
}

function checkCrateBoundary(repoRoot: string, failures: string[]): void {
  const cargo = readTarget(repoRoot, "packages/open-bitcoin-network/Cargo.toml");
  if (cargo.includes("open-bitcoin-mempool")) {
    failures.push(
      "P133 architecture: the network crate must remain neutral and must not depend on mempool admission",
    );
  }
}

function checkParityEvidence(repoRoot: string, failures: string[]): void {
  const registry = JSON.parse(
    readTarget(repoRoot, "docs/parity/source-breadcrumbs.json"),
  ) as {
    groups: Array<{
      label: string;
      files: string[];
      breadcrumbs: string[];
    }>;
  };
  const expected = new Map([
    [
      "network-transaction-relay-download",
      [
        "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs",
        "packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs",
        "packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs",
      ],
    ],
    [
      "network-transaction-reject-evidence",
      [
        "packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs",
        "packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs",
      ],
    ],
    [
      "node-package-admission-bridge",
      [
        "packages/open-bitcoin-node/src/network/admission_bridge/package.rs",
        "packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs",
      ],
    ],
  ]);
  for (const [label, paths] of expected) {
    const maybeGroup = registry.groups.find((group) => group.label === label);
    if (
      maybeGroup === undefined ||
      paths.some((file) => !maybeGroup.files.includes(file)) ||
      !maybeGroup.breadcrumbs.includes(
        "packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py",
      )
    ) {
      failures.push(
        `P133 parity: ${label} must retain exact Phase 133 files and the opportunistic 1P1C Knots anchor`,
      );
    }
  }
}

function checkDocumentation(repoRoot: string, failures: string[]): void {
  const catalog = readTarget(
    repoRoot,
    "docs/parity/catalog/mempool-policy.md",
  );
  requireAll(
    catalog,
    [
      "## Package-Aware Download and Orphan Bridge",
      "PPKG-01",
      "PPKG-02",
      "PPKG-03",
      "120,000",
      "0.000001",
      "active-tip",
      "`[parent, child]`",
      "exactly one authoritative package-admission call",
      "Phase 134 owns",
      "Phase 136 owns",
      "Phase 137 owns",
    ],
    "P133 docs: catalog must state the exact PPKG-01/02/03 bounds, bridge, and deferred ownership",
    failures,
  );

  const index = JSON.parse(
    readTarget(repoRoot, "docs/parity/index.json"),
  ) as {
    surfaces: Array<{ name: string; status: string }>;
    checklist: {
      surfaces: Array<{
        id: string;
        status: string;
        requirements: string[];
        evidence: string[];
      }>;
    };
  };
  const maybeSurface = index.surfaces.find(
    (surface) =>
      surface.name === "v2-2-package-aware-download-orphan-bridge",
  );
  const maybeChecklist = index.checklist.surfaces.find(
    (surface) =>
      surface.id === "v2-2-package-aware-download-orphan-bridge",
  );
  if (
    maybeSurface?.status !== "done" ||
    maybeChecklist?.status !== "done" ||
    maybeChecklist.requirements.join(",") !== "PPKG-01,PPKG-02,PPKG-03" ||
    !maybeChecklist.evidence.includes(
      "scripts/check-phase133-package-aware-download-orphan-bridge.ts",
    )
  ) {
    failures.push(
      "P133 docs: machine index must close one exact done surface for PPKG-01 through PPKG-03",
    );
  }

  const checklist = readTarget(repoRoot, "docs/parity/checklist.md");
  if (
    !checklist.includes(
      "| `v2-2-package-aware-download-orphan-bridge` | `done` | `PPKG-01`, `PPKG-02`, `PPKG-03` |",
    )
  ) {
    failures.push(
      "P133 docs: human checklist must record the done Phase 133 surface and exact requirements",
    );
  }
}

function checkNarrowClaims(repoRoot: string, failures: string[]): void {
  const docs = [
    "README.md",
    "packages/README.md",
    "docs/parity/catalog/mempool-policy.md",
    "docs/parity/checklist.md",
  ]
    .map((file) => readTarget(repoRoot, file).toLowerCase())
    .join("\n");
  const forbidden = [
    /phase 133 (?:implements|ships|completes) (?:a )?general package wire/,
    /phase 133 (?:implements|ships|completes) arbitrary multi-parent/,
    /phase 134 (?:is |is now )?implemented/,
    /phase 136 (?:is |is now )?implemented/,
    /phase 137 (?:is |is now )?implemented/,
    /open bitcoin (?:now )?(?:provides|ships|supports) public\/default relay/,
    /open bitcoin (?:now )?(?:provides|ships|supports) guaranteed propagation/,
    /open bitcoin (?:is |is now )?production ready/,
  ];
  if (forbidden.some((pattern) => pattern.test(docs))) {
    failures.push(
      "P133 claims: the bounded 1P1C bridge must not become a general wire, later-phase, public, guaranteed, or production claim",
    );
  }
}

function checkVerifierWiring(repoRoot: string, failures: string[]): void {
  const verify = readTarget(repoRoot, "scripts/verify.sh");
  const expectedOrder = [
    PHASE132_CHECK,
    PHASE133_TEST,
    PHASE133_CHECK,
    PHASE117_TEST,
  ];
  if (
    countOrderedSequences(verify, expectedOrder) !== 2
  ) {
    failures.push(
      "P133 verifier: Phase 133 test/check must run after Phase 132 and before the final Phase 117 gate in both command surfaces",
    );
  }
}

function checkDeterministicScope(repoRoot: string, failures: string[]): void {
  const checker = readTarget(
    repoRoot,
    "scripts/check-phase133-package-aware-download-orphan-bridge.ts",
  );
  const forbidden = [
    "Bun." + "spawn",
    "child_" + "process",
    "fet" + "ch(",
    "http" + "://",
    "https" + "://",
  ];
  if (forbidden.some((needle) => checker.includes(needle))) {
    failures.push(
      "P133 checker: verification must remain deterministic and filesystem-only",
    );
  }
}

function readTarget(repoRoot: string, relativePath: string): string {
  return readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function sectionBetween(
  source: string,
  startMarker: string,
  endMarker: string,
): string {
  const start = source.indexOf(startMarker);
  const end = source.indexOf(endMarker, Math.max(start, 0));
  if (start < 0 || end < 0 || end <= start) {
    return "";
  }
  return source.slice(start, end);
}

function requireAll(
  source: string,
  needles: readonly string[],
  message: string,
  failures: string[],
): void {
  if (needles.some((needle) => !source.includes(needle))) {
    failures.push(message);
  }
}

function countMatches(source: string, pattern: RegExp): number {
  return source.match(pattern)?.length ?? 0;
}

function countOrderedSequences(
  source: string,
  needles: readonly string[],
): number {
  let cursor = 0;
  let count = 0;
  while (true) {
    for (const needle of needles) {
      const found = source.indexOf(needle, cursor);
      if (found < 0) {
        return count;
      }
      cursor = found + needle.length;
    }
    count += 1;
  }
}

if (import.meta.main) {
  const failures = checkPhase133PackageAwareDownloadOrphanBridge();
  if (failures.length > 0) {
    console.error(failures.map((failure) => `- ${failure}`).join("\n"));
    process.exit(1);
  }
  console.log("Phase 133 package-aware download/orphan bridge checks passed.");
}

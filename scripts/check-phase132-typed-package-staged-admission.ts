#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE131_CHECK =
  "bun run scripts/check-phase131-rolling-fee-expiry-pressure.ts";
const PHASE132_TEST =
  "bun test scripts/check-phase132-typed-package-staged-admission.test.ts";
const PHASE132_CHECK =
  "bun run scripts/check-phase132-typed-package-staged-admission.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

const REQUIRED_BREADCRUMB_PATHS = [
  "packages/open-bitcoin-mempool/src/package.rs",
  "packages/open-bitcoin-mempool/src/package/shape.rs",
  "packages/open-bitcoin-mempool/src/package/report.rs",
  "packages/open-bitcoin-mempool/src/package/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
  "packages/open-bitcoin-mempool/src/policy/ephemeral/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement/diagram.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement/diagram/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement/tests.rs",
  "packages/open-bitcoin-mempool/src/policy/truc.rs",
  "packages/open-bitcoin-mempool/src/policy/truc/tests.rs",
  "packages/open-bitcoin-mempool/src/pool/candidate.rs",
  "packages/open-bitcoin-mempool/src/pool/oracle.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs",
  "packages/open-bitcoin-mempool/src/pool/patch.rs",
  "packages/open-bitcoin-mempool/src/pool/patch/graph.rs",
  "packages/open-bitcoin-mempool/src/pool/prospective.rs",
  "packages/open-bitcoin-mempool/src/pool/prospective/limits.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/revision_cases.rs",
] as const;

export const PHASE132_TARGET_FILES = [
  "README.md",
  "packages/README.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-consensus/src/classify.rs",
  "packages/open-bitcoin-mempool/src/fee.rs",
  "packages/open-bitcoin-mempool/src/package.rs",
  "packages/open-bitcoin-mempool/src/package/shape.rs",
  "packages/open-bitcoin-mempool/src/package/report.rs",
  "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
  "packages/open-bitcoin-mempool/src/policy/output.rs",
  "packages/open-bitcoin-mempool/src/policy/replacement.rs",
  "packages/open-bitcoin-mempool/src/policy/truc.rs",
  "packages/open-bitcoin-mempool/src/pool.rs",
  "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  "packages/open-bitcoin-mempool/src/pool/prospective.rs",
  "packages/open-bitcoin-mempool/src/pool/tests.rs",
  "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
  "packages/open-bitcoin-mempool/src/types.rs",
  "scripts/check-phase132-typed-package-staged-admission.ts",
  "scripts/verify.sh",
] as const;

const NO_CLAIM_MARKERS = [
  "does not",
  "do not",
  "not ",
  "without",
  "outside",
  "deferred",
  "unsupported",
  "no claim",
] as const;

export function checkPhase132TypedPackageStagedAdmission(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE132_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  checkPack01Shape(repoRoot, failures);
  checkOpaquePublicApi(repoRoot, failures);
  checkPack02ThroughPack06(repoRoot, failures);
  checkPack07Policy(repoRoot, failures);
  checkParityClosure(repoRoot, failures);
  checkDocumentation(repoRoot, failures);
  checkNarrowClaims(repoRoot, failures);
  checkVerifierWiring(repoRoot, failures);
  checkDeterministicScope(repoRoot, failures);
  return failures;
}

function checkPack01Shape(repoRoot: string, failures: string[]): void {
  const packageSource = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package.rs",
  );
  const shape = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package/shape.rs",
  );
  if (
    !packageSource.includes("pub const MAX_PACKAGE_COUNT: usize = 25;") ||
    !packageSource.includes("pub const MAX_PACKAGE_WEIGHT: usize = 404_000;")
  ) {
    failures.push(
      "P132 PACK-01: package shape must retain the pinned 25-count and 404000-weight bounds",
    );
  }
  requireAll(
    shape,
    [
      "PackageShapeError::Empty",
      "PackageShapeError::TooManyTransactions",
      "PackageShapeError::TooHeavy",
      "PackageShapeError::DuplicateTxid",
      "PackageShapeError::DuplicateWtxid",
      "validate_topology_and_conflicts",
      "PackageFingerprint::from_members",
    ],
    "P132 PACK-01: shape refinement must prove nonempty bounds identities topology conflicts and fingerprint",
    failures,
  );
}

function checkOpaquePublicApi(repoRoot: string, failures: string[]): void {
  const packageSource = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package.rs",
  );
  const shape = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package/shape.rs",
  );
  const report = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package/report.rs",
  );

  if (
    !packageSource.includes("pub struct SubmissionPackage {") ||
    countMatches(shape, /\bpub fn try_from_package\(/g) !== 1 ||
    !packageSource.includes("pub enum SubmissionPackageKind {\n    Single,\n    ChildWithUnconfirmedParents,\n}")
  ) {
    failures.push(
      "P132 opacity: SubmissionPackage must remain a private-field refinement with one checked constructor",
    );
  }
  if (
    !hasPrivateFields(packageSource, "SubmissionPackage", [
      "package: WellFormedPackage",
      "kind: SubmissionPackageKind",
    ]) ||
    !packageSource.includes("pub fn package(&self) -> &WellFormedPackage") ||
    !packageSource.includes("pub fn kind(&self) -> SubmissionPackageKind")
  ) {
    failures.push(
      "P132 opacity: SubmissionPackage fields must remain private and read-only",
    );
  }
  if (
    /(new_unchecked|package_mut|kind_mut|set_package|set_kind)/.test(
      packageSource,
    ) ||
    /impl\s+(?:Default|From<[^>]+>)\s+for\s+SubmissionPackage/.test(
      packageSource,
    )
  ) {
    failures.push(
      "P132 opacity: SubmissionPackage must not gain unchecked/default/from/mutable construction",
    );
  }
  if (
    !near(
      packageSource,
      "pub struct SubmissionPackage",
      "```compile_fail,E0451",
      true,
    ) ||
    !packageSource.includes("SubmissionPackage { package, kind: kind }")
  ) {
    failures.push(
      "P132 opacity: SubmissionPackage must retain an E0451 direct-construction compile-fail proof",
    );
  }

  if (
    !hasPrivateFields(report, "PackageReport", [
      "fingerprint: PackageFingerprint",
      "status: PackageStatus",
      "members: Vec<PackageMemberResult>",
      "effective_fee_groups: Vec<EffectiveFeeGroup>",
    ])
  ) {
    failures.push(
      "P132 opacity: PackageReport fields must remain private and read-only",
    );
  }
  if (
    !hasPrivateFields(report, "EffectiveFeeGroup", [
      "id: EffectiveFeeGroupId",
      "ordered_wtxids: Vec<Wtxid>",
      "base_fee_sats: Amount",
      "modified_fee_sats: Amount",
      "virtual_size: TransactionVirtualSize",
      "effective_fee_rate: FeeRate",
    ])
  ) {
    failures.push(
      "P132 opacity: EffectiveFeeGroup fields must remain private and read-only",
    );
  }
  if (
    /(members_mut|effective_fee_groups_mut|ordered_wtxids_mut|set_status|set_members|set_effective)/.test(
      report,
    ) ||
    /impl\s+Default\s+for\s+(?:PackageReport|EffectiveFeeGroup)/.test(report)
  ) {
    failures.push(
      "P132 opacity: reports and fee groups must not gain setters or mutable accessors",
    );
  }
  if (
    countMatches(report, /```compile_fail,E0451/g) < 2 ||
    countMatches(report, /```compile_fail,E0616/g) < 2 ||
    !report.includes("PackageReport { fingerprint, status, members, effective_fee_groups }") ||
    !report.includes("report.status = status;")
  ) {
    failures.push(
      "P132 opacity: PackageReport must retain E0451 construction and E0616 mutation proofs",
    );
  }
}

function checkPack02ThroughPack06(
  repoRoot: string,
  failures: string[],
): void {
  const packageSource = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package.rs",
  );
  const report = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/package/report.rs",
  );
  const pool = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool.rs",
  );
  const admission = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  );
  const prospective = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/prospective.rs",
  );

  requireAll(
    packageSource,
    [
      "pub struct DryRunPackageCommand",
      "pub struct SubmitPackageCommand",
      "pub struct DryRunPackageResult",
      "pub struct SubmittedPackageResult",
      "pub delta: MempoolLifecycleDelta",
    ],
    "P132 PACK-02/PACK-03: dry-run and submit commands/results must remain capability-separated",
    failures,
  );
  requireOrdered(
    admission,
    [
      "pub fn dry_run_package(",
      "pub fn submit_package(",
      "fn evaluate_package(",
    ],
    "P132 PACK-02: dry-run and submit must share evaluation without conflating mutation capability",
    failures,
  );
  requireAll(
    admission,
    [
      "let before = self.complete_snapshot();",
      "assert_eq!(self.complete_snapshot(), before);",
      "for (index, (identity, transaction)) in request_members.iter().copied().enumerate()",
      "reconsiderable_indices.push(index);",
      "residual::evaluate(",
      "PackageMemberResult::SameTxidDifferentWitness",
    ],
    "P132 PACK-02/PACK-04: dry-run immutability and individual-first partial acceptance must remain explicit",
    failures,
  );
  if (
    !report.includes("pub fn try_new(") ||
    !report.includes("PackageReportError::MemberCountMismatch") ||
    !report.includes("PackageReportError::IdentityMismatch") ||
    !report.includes("PackageReportError::StatusMismatch")
  ) {
    failures.push(
      "P132 PACK-03: PackageReport::try_new must validate cardinality, order/identity, and status",
    );
  }
  requireAll(
    report,
    [
      "    FinallyPresent(NewlyPresent),",
      "    AlreadyPresent(ExistingMember),",
      "    SameTxidDifferentWitness(WitnessAlias),",
      "    HardRejected(HardMemberFailure),",
      "    Reconsiderable(ReconsiderableMemberFailure),",
      "    PostTrimAbsent(PostTrimAbsence),",
    ],
    "P132 PACK-03: ordered reports must retain the complete typed member-result vocabulary",
    failures,
  );
  requireAll(
    pool,
    [
      "pub(super) struct MempoolPatch",
      "base_revision: MempoolRevision",
      "pub(super) fn apply_prepared(",
      "if self.revision != patch.base_revision",
    ],
    "P132 PACK-05: sparse patch apply must remain crate-private and revision-first",
    failures,
  );
  requireAll(
    prospective,
    [
      "pub(super) struct ProspectiveMempool",
      "added_or_updated:",
      "removed:",
      "spent_updates:",
      "topology_updates:",
      "pub(super) fn prepare_patch(",
    ],
    "P132 PACK-05: prospective admission must remain a sparse guarded overlay",
    failures,
  );
  const testMaterialization = prospective.indexOf(
    "pub(super) fn materialize_for_test",
  );
  const productionProspective =
    testMaterialization === -1
      ? prospective
      : prospective.slice(0, testMaterialization);
  if (
    productionProspective.includes("MempoolState") ||
    productionProspective.includes("recompute_state(")
  ) {
    failures.push(
      "P132 PACK-05: production prospective admission must not clone MempoolState or fully recompute",
    );
  }
  if (
    !report.includes("pub fn try_new(") ||
    !report.includes("EffectiveFeeGroupError::EmptyMembership") ||
    !report.includes("EffectiveFeeGroupError::DuplicateMembership") ||
    !report.includes("EffectiveFeeGroupError::InconsistentEffectiveRate")
  ) {
    failures.push(
      "P132 PACK-06: EffectiveFeeGroup::try_new must validate nonempty unique membership and effective rate",
    );
  }
}

function checkPack07Policy(repoRoot: string, failures: string[]): void {
  const admission = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
  );
  const replacement = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/replacement.rs",
  );
  const truc = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/truc.rs",
  );
  const ephemeral = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
  );
  const output = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/policy/output.rs",
  );
  const types = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/types.rs",
  );
  const fee = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/fee.rs",
  );
  const classify = readTarget(
    repoRoot,
    "packages/open-bitcoin-consensus/src/classify.rs",
  );
  const singletonEvaluation = sectionBetween(
    admission,
    "fn evaluate_singleton",
    "fn remove_individual_groups",
  );

  requireOrdered(
    singletonEvaluation,
    [
      "RollingMempoolFeeRate::ZERO,",
      "evaluate_truc_package(",
      "prospective.rolling_mempool_fee_rate(),",
      "validate_candidate_limits(identity.txid)",
      "validate_ephemeral_spends(",
      "run_late_script_checks(",
    ],
    "P132 PACK-07: policy order must keep static/TRUC/rolling/limits/replacement/ephemeral/late scripts",
    failures,
  );
  if (
    !orderedOffsets(singletonEvaluation, [
      "validate_ephemeral_spends(",
      "run_late_script_checks(",
    ])
  ) {
    failures.push(
      "P132 PACK-07: policy order must keep ephemeral checks before late scripts",
    );
  }
  requireAll(
    replacement,
    [
      "MAX_REPLACEMENT_CANDIDATES: usize = 100",
      "enforce_conservative_candidate_bound(view, &direct_conflicts)?;",
      "potential_count",
      ".checked_add(descendant_count)",
      "collect_removal_union(view, &direct_conflicts)",
    ],
    "P132 PACK-07: limited replacement must retain conservative pre-union 100-candidate counting",
    failures,
  );
  requireAll(
    truc,
    [
      "direct_conflicts: &BTreeSet<Txid>",
      "maybe_sibling_eviction: Option<EligibleSiblingEviction>",
      "find_sibling_eviction(view, members, direct_conflicts)",
      "validate_parent_children(",
      "!direct_conflicts.contains(txid)",
      "intent.sibling",
    ],
    "P132 PACK-07: TRUC must evaluate direct conflicts and sibling intent against pre-replacement facts",
    failures,
  );
  requireAll(
    ephemeral,
    [
      "member.fees.base.to_sats() != 0 || member.fees.modified.to_sats() != 0",
      "missing.insert(OutPoint",
      "missing.remove(&input.previous_output);",
      "if missing.is_empty()",
    ],
    "P132 PACK-07: ephemeral policy must retain zero-fee and complete-spend predicates",
    failures,
  );
  requireAll(
    types,
    ["anchor: true", "send: false", "dust: false"],
    "P132 PACK-07: ephemeral defaults must remain anchor=true send=false dust=false",
    failures,
  );
  requireAll(
    output,
    [
      "ScriptPubKeyType::PayToAnchor",
      "permissions.anchor",
      "permissions.send",
      "output.value.to_sats() == 0 || permissions.dust",
    ],
    "P132 PACK-07: P2A/send/dust permission predicates must remain cumulative",
    failures,
  );
  requireAll(
    classify,
    [
      "const PAY_TO_ANCHOR_PROGRAM: [u8; 2] = [0x4e, 0x73];",
      "ScriptPubKeyType::PayToAnchor",
    ],
    "P132 PACK-07: exact pay-to-anchor witness-program bytes must remain classified",
    failures,
  );
  if (!fee.includes("FeeRate::from_sats_per_kvb(3_000)")) {
    failures.push(
      "P132 PACK-07: ephemeral dust threshold must retain the 3000 sat/kvB default",
    );
  }
  if (
    countMatches(admission, /trim_prospective_to_capacity\(/g) !== 1 ||
    !admission.includes("rewrite_final_membership(&prospective, &mut results);")
  ) {
    failures.push(
      "P132 PACK-07: package execution must retain one trim and final-membership rewrite",
    );
  }
}

function checkParityClosure(repoRoot: string, failures: string[]): void {
  const registry = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/tests.rs",
  );
  const breadcrumbsText = readTarget(
    repoRoot,
    "docs/parity/source-breadcrumbs.json",
  );
  if (
    !registry.includes("mod package_parity_cases;") ||
    !breadcrumbsText.includes(
      '"packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs"',
    )
  ) {
    failures.push(
      "P132 parity: package_parity_cases must remain registered and breadcrumbed",
    );
  }
  let manifest: BreadcrumbManifest | undefined;
  try {
    manifest = JSON.parse(breadcrumbsText) as BreadcrumbManifest;
  } catch {
    failures.push("P132 breadcrumbs: source-breadcrumbs.json must remain valid JSON");
    return;
  }
  for (const requiredPath of REQUIRED_BREADCRUMB_PATHS) {
    const maybeGroup = manifest.groups.find((group) =>
      group.files.includes(requiredPath),
    );
    if (
      !maybeGroup ||
      maybeGroup.breadcrumbs.length === 0 ||
      maybeGroup.breadcrumbs.includes("none")
    ) {
      failures.push(
        `P132 breadcrumbs: missing non-none Knots anchors for ${requiredPath}`,
      );
    }
  }
  const parityCases = readTarget(
    repoRoot,
    "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs",
  );
  requireAll(
    parityCases,
    [
      "MAX_PACKAGE_COUNT",
      "MAX_PACKAGE_WEIGHT",
      "stale_revision_sparse_patch_rejects_before_apply_without_mutation",
      "overlap_over_100_conservative_limited_rbf_count_precedes_union",
      "max_bound_generated_sparse_overlay_recompute_has_zero_clone_and_one_trim",
    ],
    "P132 parity: integrated package-policy matrix must retain bounds stale overlap and oracle cases",
    failures,
  );
}

function checkDocumentation(repoRoot: string, failures: string[]): void {
  const catalog = readTarget(
    repoRoot,
    "docs/parity/catalog/mempool-policy.md",
  );
  const readme = readTarget(repoRoot, "README.md");
  const packagesReadme = readTarget(repoRoot, "packages/README.md");
  requireAll(
    catalog,
    [
      "## Typed Package Vocabulary and Staged Admission",
      "PACK-01",
      "PACK-02",
      "PACK-03",
      "PACK-04",
      "PACK-05",
      "PACK-06",
      "PACK-07",
      "anchor=true",
      "send=false",
      "dust=false",
      "sparse overlay",
      "ordered",
      "one final trim",
      "general package wire",
      "guaranteed propagation",
    ],
    "P132 docs: catalog must map PACK-01 through PACK-07 and bounded intentional/deferred boundaries",
    failures,
  );
  for (const anchor of [
    "doc/policy/packages.md",
    "src/policy/packages.cpp",
    "src/validation.cpp",
    "src/test/txpackage_tests.cpp",
    "test/functional/mempool_package_rbf.py",
    "test/functional/mempool_truc.py",
    "test/functional/mempool_ephemeral_dust.py",
  ]) {
    if (!catalog.includes(anchor)) {
      failures.push(`P132 docs: catalog is missing pinned Knots anchor ${anchor}`);
    }
  }
  requireAll(
    `${readme}\n${packagesReadme}`.toLowerCase(),
    [
      "bounded local",
      "package admission",
      "peer package assembly",
      "general package wire",
      "guaranteed propagation",
    ],
    "P132 docs: READMEs must claim bounded local package admission and retain relay exclusions",
    failures,
  );
}

function checkNarrowClaims(repoRoot: string, failures: string[]): void {
  const claimFiles = [
    "README.md",
    "packages/README.md",
    "docs/parity/catalog/mempool-policy.md",
  ];
  const forbiddenClaims = [
    "general package wire",
    "peer package assembly",
    "rpc package adapter",
    "public package relay",
    "default package relay",
    "guaranteed propagation",
    "public-network package",
    "production readiness",
  ];
  for (const relativePath of claimFiles) {
    const text = readTarget(repoRoot, relativePath);
    for (const paragraph of text.split(/\r?\n\s*\r?\n/)) {
      const lower = paragraph.toLowerCase();
      for (const claim of forbiddenClaims) {
        if (!lower.includes(claim)) continue;
        if (NO_CLAIM_MARKERS.some((marker) => lower.includes(marker))) continue;
        failures.push(
          "P132 claims: bounded local admission must not become a general package-wire or production claim",
        );
        return;
      }
    }
  }
  for (const relativePath of REQUIRED_BREADCRUMB_PATHS) {
    const source = readTarget(repoRoot, relativePath);
    if (
      source.includes("open_bitcoin_network") ||
      source.includes("open_bitcoin_node") ||
      source.includes("open_bitcoin_rpc")
    ) {
      failures.push(
        `P132 architecture: pure package core must not import an adapter in ${relativePath}`,
      );
    }
  }
}

function checkVerifierWiring(repoRoot: string, failures: string[]): void {
  const verify = readTarget(repoRoot, "scripts/verify.sh");
  const visible = visibleCommandOrder(verify);
  const requiredVisible = [
    PHASE131_CHECK,
    PHASE132_TEST,
    PHASE132_CHECK,
    PHASE117_TEST,
  ];
  const requiredSteps = [
    `run_step "check Phase 131 rolling fee expiry pressure" ${PHASE131_CHECK}`,
    `run_step "test Phase 132 typed package staged admission checker" ${PHASE132_TEST}`,
    `run_step "check Phase 132 typed package staged admission" ${PHASE132_CHECK}`,
    `run_step "test Phase 117 parity UAT release boundary checker" ${PHASE117_TEST}`,
  ];
  if (
    !orderedLines(visible, requiredVisible) ||
    !orderedLines(verify, requiredSteps)
  ) {
    failures.push(
      "P132 verifier: checker test/run must follow Phase 131 and precede Phase 117 in both surfaces",
    );
  }
}

function checkDeterministicScope(repoRoot: string, failures: string[]): void {
  const checker = readTarget(
    repoRoot,
    "scripts/check-phase132-typed-package-staged-admission.ts",
  );
  const forbidden = [
    "fetch" + "(",
    "Bun." + "spawn",
    "node:" + "child_process",
    "http" + "://",
    "https" + "://",
  ];
  if (forbidden.some((needle) => checker.includes(needle))) {
    failures.push(
      "P132 deterministic scope: checker must remain local and network-free",
    );
  }
}

function readTarget(repoRoot: string, relativePath: string): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) return "";
  return readFileSync(absolutePath, "utf8");
}

function requireAll(
  text: string,
  needles: readonly string[],
  failure: string,
  failures: string[],
): void {
  if (!needles.every((needle) => text.includes(needle))) {
    failures.push(failure);
  }
}

function requireOrdered(
  text: string,
  needles: readonly string[],
  failure: string,
  failures: string[],
): void {
  if (!orderedOffsets(text, needles)) failures.push(failure);
}

function orderedOffsets(text: string, needles: readonly string[]): boolean {
  let cursor = -1;
  for (const needle of needles) {
    const next = text.indexOf(needle, cursor + 1);
    if (next === -1) return false;
    cursor = next;
  }
  return true;
}

function hasPrivateFields(
  text: string,
  name: string,
  fields: readonly string[],
): boolean {
  const start = text.indexOf(`pub struct ${name} {`);
  if (start === -1) return false;
  const end = text.indexOf("\n}", start);
  if (end === -1) return false;
  const body = text.slice(start, end);
  return (
    fields.every(
      (field) => body.includes(`    ${field},`) && !body.includes(`pub ${field}`),
    ) && !/\n\s+pub(?:\([^)]*\))?\s+\w+\s*:/.test(body)
  );
}

function near(
  text: string,
  anchor: string,
  needle: string,
  before: boolean,
): boolean {
  const anchorIndex = text.indexOf(anchor);
  const needleIndex = text.indexOf(needle);
  if (anchorIndex === -1 || needleIndex === -1) return false;
  return before ? needleIndex < anchorIndex : needleIndex > anchorIndex;
}

function countMatches(text: string, pattern: RegExp): number {
  return Array.from(text.matchAll(pattern)).length;
}

function sectionBetween(text: string, startNeedle: string, endNeedle: string): string {
  const start = text.indexOf(startNeedle);
  if (start === -1) return "";
  const end = text.indexOf(endNeedle, start + startNeedle.length);
  return end === -1 ? text.slice(start) : text.slice(start, end);
}

function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

function orderedLines(text: string, required: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const line of required) {
    const index = lines.indexOf(line, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

interface BreadcrumbManifest {
  groups: Array<{
    files: string[];
    breadcrumbs: string[];
  }>;
}

if (import.meta.main) {
  const failures = checkPhase132TypedPackageStagedAdmission();
  if (failures.length > 0) {
    for (const failure of failures) console.error(failure);
    process.exit(1);
  }
  console.log(
    "Phase 132 typed package staged admission checks passed: PACK-01 PACK-02 PACK-03 PACK-04 PACK-05 PACK-06 PACK-07.",
  );
}

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { DEFAULT_REPO_ROOT } from "./constants.ts";
import { checkPack07Policy } from "./policy.ts";
import { checkParityClosure, checkDocumentation, checkNarrowClaims } from "./parity.ts";
import { checkVerifierWiring, checkDeterministicScope } from "./claims.ts";
import { readTarget } from "./filesystem.ts";
import { requireAll, requireOrdered, hasPrivateFields, near, countMatches } from "./helpers.ts";

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

export function checkPack01Shape(repoRoot: string, failures: string[]): void {
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

export function checkOpaquePublicApi(repoRoot: string, failures: string[]): void {
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

export function checkPack02ThroughPack06(
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

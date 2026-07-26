import { afterEach, expect, test } from "bun:test";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  PHASE132_TARGET_FILES,
  checkPhase132TypedPackageStagedAdmission,
} from "./check-phase132-typed-package-staged-admission";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
type Mutator = (files: Map<string, string>) => void;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 132 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase132TypedPackageStagedAdmission(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "PACK-01 count bound",
    "P132 PACK-01: package shape must retain the pinned 25-count and 404000-weight bounds",
    replace(
      "packages/open-bitcoin-mempool/src/package.rs",
      "pub const MAX_PACKAGE_COUNT: usize = 25;",
      "pub const MAX_PACKAGE_COUNT: usize = 26;",
    ),
  ],
  [
    "SubmissionPackage raw payload enum",
    "P132 opacity: SubmissionPackage must remain a private-field refinement with one checked constructor",
    replace(
      "packages/open-bitcoin-mempool/src/package.rs",
      "pub struct SubmissionPackage {",
      "pub enum SubmissionPackage {\n    Raw(Vec<Transaction>),",
    ),
  ],
  [
    "SubmissionPackage public field",
    "P132 opacity: SubmissionPackage fields must remain private and read-only",
    replace(
      "packages/open-bitcoin-mempool/src/package.rs",
      "    package: WellFormedPackage,",
      "    pub package: WellFormedPackage,",
    ),
  ],
  [
    "SubmissionPackage unchecked constructor",
    "P132 opacity: SubmissionPackage must not gain unchecked/default/from/mutable construction",
    append(
      "packages/open-bitcoin-mempool/src/package.rs",
      "\nimpl SubmissionPackage { pub fn new_unchecked(package: WellFormedPackage, kind: SubmissionPackageKind) -> Self { Self { package, kind } } }\n",
    ),
  ],
  [
    "SubmissionPackage privacy proof error",
    "P132 opacity: SubmissionPackage must retain an E0451 direct-construction compile-fail proof",
    replace(
      "packages/open-bitcoin-mempool/src/package.rs",
      "```compile_fail,E0451",
      "```compile_fail,E0616",
    ),
  ],
  [
    "PackageReport public field",
    "P132 opacity: PackageReport fields must remain private and read-only",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "    status: PackageStatus,",
      "    pub status: PackageStatus,",
    ),
  ],
  [
    "EffectiveFeeGroup public field",
    "P132 opacity: EffectiveFeeGroup fields must remain private and read-only",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "    ordered_wtxids: Vec<Wtxid>,",
      "    pub ordered_wtxids: Vec<Wtxid>,",
    ),
  ],
  [
    "report mutable accessor",
    "P132 opacity: reports and fee groups must not gain setters or mutable accessors",
    append(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "\nimpl PackageReport { pub fn members_mut(&mut self) -> &mut Vec<PackageMemberResult> { &mut self.members } }\n",
    ),
  ],
  [
    "report mutation privacy proof",
    "P132 opacity: PackageReport must retain E0451 construction and E0616 mutation proofs",
    replaceNth(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "```compile_fail,E0616",
      "```compile_fail,E0451",
      2,
    ),
  ],
  [
    "report member cardinality",
    "P132 PACK-03: PackageReport::try_new must validate cardinality, order/identity, and status",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "PackageReportError::MemberCountMismatch",
      "PackageReportError::IdentityMismatch",
    ),
  ],
  [
    "report status consistency",
    "P132 PACK-03: PackageReport::try_new must validate cardinality, order/identity, and status",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "PackageReportError::StatusMismatch",
      "PackageReportError::IdentityMismatch",
    ),
  ],
  [
    "fee group nonempty",
    "P132 PACK-06: EffectiveFeeGroup::try_new must validate nonempty unique membership and effective rate",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "EffectiveFeeGroupError::EmptyMembership",
      "EffectiveFeeGroupError::ZeroVirtualSize",
    ),
  ],
  [
    "fee group duplicate",
    "P132 PACK-06: EffectiveFeeGroup::try_new must validate nonempty unique membership and effective rate",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "EffectiveFeeGroupError::DuplicateMembership",
      "EffectiveFeeGroupError::ZeroVirtualSize",
    ),
  ],
  [
    "fee group rate consistency",
    "P132 PACK-06: EffectiveFeeGroup::try_new must validate nonempty unique membership and effective rate",
    replace(
      "packages/open-bitcoin-mempool/src/package/report.rs",
      "EffectiveFeeGroupError::InconsistentEffectiveRate",
      "EffectiveFeeGroupError::ZeroVirtualSize",
    ),
  ],
  [
    "internal parity registry",
    "P132 parity: package_parity_cases must remain registered and breadcrumbed",
    replace(
      "packages/open-bitcoin-mempool/src/pool/tests.rs",
      "mod package_parity_cases;\n",
      "",
    ),
  ],
  [
    "internal parity breadcrumb",
    "P132 parity: package_parity_cases must remain registered and breadcrumbed",
    replace(
      "docs/parity/source-breadcrumbs.json",
      '        "packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs"\n',
      "",
    ),
  ],
  [
    "late scripts",
    "P132 PACK-07: policy order must keep ephemeral checks before late scripts",
    swap(
      "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
      "validate_ephemeral_spends",
      "run_late_script_checks",
    ),
  ],
  [
    "conservative overlap count",
    "P132 PACK-07: limited replacement must retain conservative pre-union 100-candidate counting",
    replace(
      "packages/open-bitcoin-mempool/src/policy/replacement.rs",
      "enforce_conservative_candidate_bound(view, &direct_conflicts)?;",
      "",
    ),
  ],
  [
    "TRUC direct conflicts",
    "P132 PACK-07: TRUC must evaluate direct conflicts and sibling intent against pre-replacement facts",
    replace(
      "packages/open-bitcoin-mempool/src/policy/truc.rs",
      "direct_conflicts: &BTreeSet<Txid>,",
      "_direct_conflicts: &BTreeSet<Txid>,",
    ),
  ],
  [
    "ephemeral zero fee",
    "P132 PACK-07: ephemeral policy must retain zero-fee and complete-spend predicates",
    replace(
      "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
      "member.fees.base.to_sats() != 0 || member.fees.modified.to_sats() != 0",
      "member.fees.base.to_sats() != 0",
    ),
  ],
  [
    "ephemeral complete spend",
    "P132 PACK-07: ephemeral policy must retain zero-fee and complete-spend predicates",
    replace(
      "packages/open-bitcoin-mempool/src/policy/ephemeral.rs",
      "missing.remove(&input.previous_output);",
      "",
    ),
  ],
  [
    "ephemeral permission defaults",
    "P132 PACK-07: ephemeral defaults must remain anchor=true send=false dust=false",
    replace(
      "packages/open-bitcoin-mempool/src/types.rs",
      "            anchor: true,",
      "            anchor: false,",
    ),
  ],
  [
    "one final trim",
    "P132 PACK-07: package execution must retain one trim and final-membership rewrite",
    replace(
      "packages/open-bitcoin-mempool/src/pool/package_admission.rs",
      "rewrite_final_membership(&prospective, &mut results);",
      "",
    ),
  ],
  [
    "narrow claim",
    "P132 claims: bounded local admission must not become a general package-wire or production claim",
    append(
      "README.md",
      "\nOpen Bitcoin provides general package wire relay and production readiness.\n",
    ),
  ],
  [
    "verifier run step",
    "P132 verifier: checker test/run must follow Phase 131 and precede Phase 117 in both surfaces",
    replace(
      "scripts/verify.sh",
      'run_step "check Phase 132 typed package staged admission" bun run scripts/check-phase132-typed-package-staged-admission.ts\n',
      "",
    ),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);

  // Act
  const failures = checkPhase132TypedPackageStagedAdmission(root);

  // Assert
  expect(failures).toContain(expectedFailure);
});

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase132-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const file of PHASE132_TARGET_FILES) {
    files.set(file, readFileSync(path.join(REPO_ROOT, file), "utf8"));
  }
  maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, text);
  }
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

function replaceNth(
  file: string,
  needle: string,
  replacement: string,
  occurrence: number,
): Mutator {
  return (files) => {
    const text = files.get(file) ?? "";
    let cursor = 0;
    let index = -1;
    for (let count = 0; count < occurrence; count += 1) {
      index = text.indexOf(needle, cursor);
      if (index === -1) {
        throw new Error(`fixture occurrence missing in ${file}: ${needle}`);
      }
      cursor = index + needle.length;
    }
    files.set(
      file,
      `${text.slice(0, index)}${replacement}${text.slice(index + needle.length)}`,
    );
  };
}

function append(file: string, value: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}${value}`);
}

function swap(file: string, left: string, right: string): Mutator {
  return (files) => {
    const text = files.get(file) ?? "";
    if (!text.includes(left) || !text.includes(right)) {
      throw new Error(`fixture swap needle missing in ${file}`);
    }
    files.set(
      file,
      text.replace(left, "__PHASE132_SWAP__").replace(right, left).replace(
        "__PHASE132_SWAP__",
        right,
      ),
    );
  };
}

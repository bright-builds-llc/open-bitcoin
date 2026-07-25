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
  PHASE130_TARGET_FILES,
  checkPhase130ResourceTimeFeePrimitives,
} from "./check-phase130-resource-time-fee-primitives";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
type Mutator = (files: Map<string, string>) => void;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 130 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase130ResourceTimeFeePrimitives(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "FEEP-01 resource types",
    "P130 FEEP-01: TransactionVirtualSize, AccountedMempoolMemory, and MempoolCapacity must remain distinct",
    replace(
      "packages/open-bitcoin-mempool/src/resource.rs",
      "pub struct AccountedMempoolMemory(usize);",
      "pub struct AccountedMempoolMemoryRenamed(usize);",
    ),
  ],
  [
    "FEEP-02 fee roles",
    "P130 FEEP-02: StaticRelayFeeRate, IncrementalRelayFeeRate, RollingMempoolFeeRate, and EffectiveAdmissionFeeRate must remain distinct",
    replace(
      "packages/open-bitcoin-mempool/src/fee.rs",
      "pub struct EffectiveAdmissionFeeRate(FeeRate);",
      "pub struct EffectiveAdmissionFeeRateRenamed(FeeRate);",
    ),
  ],
  [
    "FEEP-03 entry metadata",
    "P130 FEEP-03: MempoolEntryMetadata must retain acceptance time, origin, and relay intent",
    replace(
      "packages/open-bitcoin-mempool/src/context.rs",
      "pub struct MempoolEntryMetadata {",
      "pub struct MempoolEntryMetadataRenamed {",
    ),
  ],
  [
    "FEEP-04 explicit contexts",
    "P130 FEEP-04: operation-specific immutable contexts must remain present",
    replace(
      "packages/open-bitcoin-mempool/src/context.rs",
      "pub struct ReorgLifecycleContext",
      "pub struct ReorgLifecycleContextRenamed",
    ),
  ],
  [
    "FEEP-05 lifecycle delta",
    "P130 FEEP-05: MempoolLifecycleDelta must remain the committed-fact vocabulary",
    replace(
      "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
      "pub struct MempoolLifecycleDelta {",
      "pub struct MempoolLifecycleDeltaRenamed {",
    ),
  ],
  [
    "overflow checking",
    "P130 overflow: resource accounting must retain checked overflow failure",
    replace(
      "packages/open-bitcoin-mempool/src/resource.rs",
      "Overflow { component: &'static str }",
      "OverflowRenamed { component: &'static str }",
    ),
  ],
  [
    "incremental exclusion",
    "P130 incremental exclusion: incremental relay fee must not act as an ordinary admission floor",
    replace(
      "packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs",
      "incremental_relay_fee_is_not_an_admission_floor",
      "incremental_relay_fee_is_an_admission_floor",
    ),
  ],
  [
    "peer/local origin",
    "P130 origin: Peer and Local mempool origins must remain distinct",
    replace(
      "packages/open-bitcoin-mempool/src/context.rs",
      "    Local,\n    /// Received from a network peer.\n    Peer,",
      "    Local,\n    /// Received from a network peer.\n    PeerRenamed,",
    ),
  ],
  [
    "hidden clock/RNG token",
    "P130 hidden effects: pure mempool policy must not read wall-clock or randomness",
    append(
      "packages/open-bitcoin-mempool/src/context.rs",
      "use std::time::SystemTime;\nfn _hidden() { let _ = SystemTime::now(); }\n",
    ),
  ],
  [
    "cause-role split",
    "P130 cause-role: MempoolRemovalCause and MempoolRemovalRole must remain independent",
    replace(
      "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
      "pub enum MempoolRemovalRole {",
      "pub enum MempoolRemovalRoleRenamed {",
    ),
  ],
  [
    "legacy partial-metadata rejection",
    "P130 legacy compatibility: partial mempool entry metadata must fail closed as corruption",
    replace(
      "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs",
      "partial mempool entry metadata is corrupt",
      "partial mempool entry metadata is ignored",
    ),
  ],
  [
    "retry jitter bound",
    "P130 retry jitter: RetryJitterSeconds must enforce the inclusive 0..=300 bound",
    replace(
      "packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs",
      "const MAX_RETRY_JITTER_SECONDS: u64 = 300;",
      "const MAX_RETRY_JITTER_SECONDS: u64 = 301;",
    ),
  ],
  [
    "RPC usage mapping",
    "P130 RPC mapping: getmempoolinfo.usage must project accounted memory",
    replace(
      "packages/open-bitcoin-rpc/src/dispatch/node.rs",
      "usage: info.accounted_memory,",
      "usage: info.total_virtual_size,",
    ),
  ],
  [
    "Phase 131/134 boundary",
    "P130 deferred boundary: Phase 131 and Phase 134 ownership must remain explicit",
    replace(
      "docs/parity/catalog/mempool-policy.md",
      "**Phase 131** owns accounted-memory enforcement",
      "**Phase 130** owns accounted-memory enforcement",
    ),
  ],
  [
    "missing test-file breadcrumb",
    "P130 breadcrumbs: production and test resource files must both be registered",
    replace(
      "docs/parity/source-breadcrumbs.json",
      '"packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs",\n',
      "",
    ),
  ],
  [
    "identity leak",
    "P130 privacy: shared evidence must keep transaction identities on authenticated responses",
    replace(
      "docs/parity/catalog/mempool-policy.md",
      "transaction identities stay on authenticated direct responses.",
      "shared metrics may include transaction identities.",
    ),
  ],
  [
    "forbidden broad claim",
    "P130 no-claim: Phase 130 must not assert public or default relay",
    append(
      "docs/parity/catalog/mempool-policy.md",
      "\nPhase 130 enables public relay by default.\n",
    ),
  ],
  [
    "README root stale wording",
    "P130 README root freshness: README.md still advertises v2.1 active status",
    replace(
      "README.md",
      "> Status: Active milestone: v2.2 — Package Relay and Long-Lived Mempool Policy.",
      "> Status: Open Bitcoin v2.1",
    ),
  ],
  [
    "README packages stale wording",
    "P130 README packages freshness: packages/README.md still describes current v2.1 milestone",
    append("packages/README.md", "\nThis tracks the current v2.1 milestone.\n"),
  ],
  [
    "README parity stale wording",
    "P130 README parity freshness: docs/parity/README.md still presents v2.1 as the current claim",
    append(
      "docs/parity/README.md",
      "\nThe current v2.1 claim is intentionally narrow:\n",
    ),
  ],
  [
    "historical legacy enforcement docs",
    "P130 legacy enforcement: Phase 130 must retain historical legacy_vsize capacity enforcement documentation",
    replace(
      "docs/parity/catalog/mempool-policy.md",
      "fixed `legacy_vsize` during Phase 130",
      "fixed transitional capacity label during Phase 130",
    ),
  ],
  [
    "verifier heredoc wiring",
    "P130 verifier heredoc: Phase 130 pair must run between Phase 129 and the Phase 117 gate",
    replace(
      "scripts/verify.sh",
      "bun run scripts/check-phase130-resource-time-fee-primitives.ts\nbun test scripts/check-phase131-rolling-fee-expiry-pressure.test.ts",
      "bun test scripts/check-phase131-rolling-fee-expiry-pressure.test.ts",
    ),
  ],
  [
    "verifier run_step wiring",
    "P130 verifier run_step: Phase 130 pair must run between Phase 129 and the Phase 117 gate",
    replace(
      "scripts/verify.sh",
      'run_step "check Phase 130 resource time and fee primitives" bun run scripts/check-phase130-resource-time-fee-primitives.ts\n',
      "",
    ),
  ],
  [
    "final-gate ordering",
    "P130 final gate run_step order must end with bun run scripts/check-phase117-parity-uat-release-boundary.ts",
    append(
      "scripts/verify.sh",
      'run_step "check Phase 131 placeholder" bun run scripts/check-phase131-placeholder.ts',
    ),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);

  // Act
  const failures = checkPhase130ResourceTimeFeePrimitives(root);

  // Assert
  expect(failures).toContain(expectedFailure);
});

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase130-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const file of PHASE130_TARGET_FILES) {
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

function append(file: string, value: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}\n${value}\n`);
}

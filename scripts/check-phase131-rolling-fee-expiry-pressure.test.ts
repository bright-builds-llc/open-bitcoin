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
  PHASE131_TARGET_FILES,
  checkPhase131RollingFeeExpiryPressure,
} from "./check-phase131-rolling-fee-expiry-pressure";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
type Mutator = (files: Map<string, string>) => void;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with the complete Phase 131 corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase131RollingFeeExpiryPressure(root);

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "PRESS-01 accounted trim",
    "P131 PRESS-01: accounted-memory trim against MempoolCapacity must remain the active limiter",
    replace(
      "packages/open-bitcoin-mempool/src/pool/pressure.rs",
      "accounted_memory()",
      "total_virtual_size()",
    ),
  ],
  [
    "PRESS-02 track_package_removed",
    "P131 PRESS-02: track_package_removed bump must remain wired through pressure trim",
    replace(
      "packages/open-bitcoin-mempool/src/fee/rolling.rs",
      "fn track_package_removed",
      "fn track_removed_package",
    ),
  ],
  [
    "PRESS-03 halflife",
    "P131 PRESS-03: ROLLING_FEE_HALFLIFE and block-gated decay must remain present",
    replace(
      "packages/open-bitcoin-mempool/src/fee/rolling.rs",
      "ROLLING_FEE_HALFLIFE_SECONDS",
      "ROLLING_FEE_HALF_LIFE_SECS",
    ),
  ],
  [
    "PRESS-04 expire hours",
    "P131 PRESS-04: expire API and DEFAULT_MEMPOOL_EXPIRY_HOURS=336 must remain present",
    replace(
      "packages/open-bitcoin-mempool/src/types.rs",
      "pub const DEFAULT_MEMPOOL_EXPIRY_HOURS: u64 = 336",
      "pub const DEFAULT_MEMPOOL_EXPIRY_HOURS: u64 = 168",
    ),
  ],
  [
    "PRESS-05 oracle",
    "P131 PRESS-05: sustained-pressure oracle and restart-zero tests must remain present",
    replace(
      "packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",
      "sustained_pressure_oracle",
      "pressure_oracle_sustained",
    ),
  ],
  [
    "PRESS-05 bench threshold",
    "P131 PRESS-05: hermetic sustained-pressure bench threshold must remain verifier-reachable",
    replace(
      "packages/open-bitcoin-bench/src/cases/mempool.rs",
      "mempool-policy.sustained-pressure-trim",
      "mempool-policy.pressure-trim-sustained",
    ),
  ],
  [
    "evidence labels",
    "P131 evidence: capacityenforcement accounted_memory and rolling_fee_parity active must remain live",
    replace(
      "packages/open-bitcoin-mempool/src/pool/lifecycle.rs",
      '"accounted_memory"',
      '"legacy_vsize"',
    ),
  ],
  [
    "breadcrumb registration",
    "P131 breadcrumbs: Phase 131 first-party sources must remain registered",
    replace(
      "docs/parity/source-breadcrumbs.json",
      '"packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs",\n',
      "",
    ),
  ],
  [
    "public-network soak claim",
    "P131 no-claim: Phase 131 must not require public-network soak gates",
    append(
      "docs/parity/catalog/mempool-policy.md",
      "\nPhase 131 requires public-network soak validation before merge.\n",
    ),
  ],
  [
    "verifier heredoc wiring",
    "P131 verifier heredoc: Phase 131 pair must run between Phase 130 and the Phase 117 gate",
    replace(
      "scripts/verify.sh",
      "bun run scripts/check-phase131-rolling-fee-expiry-pressure.ts\nbun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
      "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
    ),
  ],
  [
    "verifier run_step wiring",
    "P131 verifier run_step: Phase 131 pair must run between Phase 130 and the Phase 117 gate",
    replace(
      "scripts/verify.sh",
      'run_step "check Phase 131 rolling fee expiry pressure" bun run scripts/check-phase131-rolling-fee-expiry-pressure.ts\n',
      "",
    ),
  ],
  [
    "final-gate ordering",
    "P131 final gate run_step order must end with bun run scripts/check-phase117-parity-uat-release-boundary.ts",
    append(
      "scripts/verify.sh",
      'run_step "check Phase 132 placeholder" bun run scripts/check-phase132-placeholder.ts',
    ),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);

  // Act
  const failures = checkPhase131RollingFeeExpiryPressure(root);

  // Assert
  expect(failures).toContain(expectedFailure);
});

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase131-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const file of PHASE131_TARGET_FILES) {
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
    files.set(file, text.split(needle).join(replacement));
  };
}

function append(file: string, value: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}\n${value}\n`);
}

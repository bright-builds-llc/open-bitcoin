import { createHash } from "node:crypto";

export const MANIFEST_FILE = "packages/testdata/phase164/historical-tooling/manifest.json";
const TOOLING_BINDINGS = [
  ["scripts/check-phase158-validated-reorg.ts", "packages/testdata/phase164/historical-tooling/phase158-checker.txt"],
  ["scripts/check-phase163-provenance.ts", "packages/testdata/phase164/historical-tooling/phase163-provenance.txt"],
  ["scripts/check-phase162-integrated-filter-proof.ts", "packages/testdata/phase164/historical-tooling/phase162-checker.txt"],
  ["scripts/check-phase162-integrated-filter-proof.test.ts", "packages/testdata/phase164/historical-tooling/phase162-tests.txt"],
  ["scripts/check-phase162-integrated-filter-proof/contracts.ts", "packages/testdata/phase164/historical-tooling/phase162-contracts.txt"],
  ["scripts/verify.sh", "packages/testdata/phase164/historical-tooling/verify.txt"],
  ["scripts/check-phase163-provenance/fixtures.ts", "packages/testdata/phase164/historical-tooling/phase163-fixtures.txt"],
  ["scripts/check-phase158-validated-reorg.test.ts", "packages/testdata/phase164/historical-tooling/phase158-tests.txt"],
  ["scripts/check-phase163-provenance/native.test.ts", "packages/testdata/phase164/historical-tooling/phase163-native-tests.txt"],
  ["scripts/check-phase159-filter-rpcs.test.ts", "packages/testdata/phase164/historical-tooling/phase159-tests.txt"],
  ["scripts/check-phase160-peer-filter-serving.test.ts", "packages/testdata/phase164/historical-tooling/phase160-tests.txt"],
  ["scripts/check-phase161-operator-index-evidence.test.ts", "packages/testdata/phase164/historical-tooling/phase161-tests.txt"],
] as const;
/** Guards whose pre-manifest text is frozen beside the manifest snapshots. */
const GUARD_SNAPSHOTS = [
  ["scripts/check-phase159-filter-rpcs.ts", "packages/testdata/phase164/historical-tooling/phase159-checker.txt", "563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32"],
  ["scripts/check-phase160-peer-filter-serving.ts", "packages/testdata/phase164/historical-tooling/phase160-checker.txt", "41f9054550790bf2f0263cedb4f0dd74c964a29a6fc4b8a492b1f17fa6cdbc05"],
  ["scripts/check-phase161-operator-index-evidence.ts", "packages/testdata/phase164/historical-tooling/phase161-checker.txt", "c3447ae4c4cc07c8a9d2de712b663520aab545f990730ec77e3e93ca7e93ce23"],
  ["scripts/check-phase163-provenance.test.ts", "packages/testdata/phase164/historical-tooling/phase163-tests.txt", "cdbffaf4922bdcd70a40d49f32552085519812a3c06b9ebda113054f3580cb5f"],
  ["scripts/check-phase163-provenance/fixture-controls.ts", "packages/testdata/phase164/historical-tooling/phase163-fixture-controls.txt", "0395ada68a3f154dacfeaea41dbd444bdf1864ee908931be5f4c3588ef761968"],
] as const;
export const CURRENT_TOOLING_FILES = TOOLING_BINDINGS.map(row => row[0]);
export const HISTORICAL_TOOLING_FILES = [MANIFEST_FILE, ...TOOLING_BINDINGS.map(row => row[1]), ...GUARD_SNAPSHOTS.map(row => row[1]),
  "scripts/check-phase164-canonical-identity/historical-tooling.ts", "scripts/check-phase164-canonical-identity/fixture-source.ts"];
export const EXPECTED_MANIFEST_SHA = "9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0";
const sha = (source: string) => createHash("sha256").update(source).digest("hex");
/** Gate steps the current checkers require exactly once. */
export const CURRENT_VERIFY_ADDITIONS = [
  'run_step "test Phase 68 active chain persistence" bun test ./scripts/check-phase68-active-chain-persistence.test.ts',
  'run_step "test Phase 165 shared exact work" bun test ./scripts/check-phase165-shared-exact-work.test.ts',
  'run_step "check Phase 165 shared exact work" bun run scripts/check-phase165-shared-exact-work.ts',
  'run_step "check Phase 166 source-derived genesis vectors" bun run scripts/phase166-genesis-vectors.ts --check',
  'run_step "test Phase 166 canonical bootstrap boundary" bun test ./scripts/check-phase166-canonical-bootstrap.test.ts',
  'run_step "check Phase 166 canonical bootstrap boundary" bun run scripts/check-phase166-canonical-bootstrap.ts',
  'run_step "check Phase 167 historical corpus" bun run scripts/phase167-historical-corpus.ts --check',
  'run_step "test Phase 167 historical corpus" bun test ./scripts/phase167-historical-corpus.test.ts',
  'run_step "check Phase 167 BIP30 fixtures" bun run scripts/phase167-bip30-fixtures.ts --check',
  'run_step "test Phase 167 BIP30 fixtures" bun test ./scripts/phase167-bip30-fixtures.test.ts',
  'run_step "check Phase 167 Regtest corpus" bun run scripts/phase167-regtest-corpus.ts --check',
  'run_step "test Phase 167 Regtest corpus" bun test ./scripts/phase167-regtest-corpus.test.ts',
  'run_step "test Phase 167 candidate history boundary" bun test ./scripts/check-phase167-candidate-history.test.ts',
  'run_step "check Phase 167 candidate history boundary" bun run scripts/check-phase167-candidate-history.ts',
  'run_step "test parity ledger" bun test ./scripts/check-parity-ledger.test.ts',
  'run_step "check parity ledger" bun run scripts/check-parity-ledger.ts',
];

export const CURRENT_TEMP_PREPARATION = '# Canonicalize the existing process-local temporary parent before any fixture work.\nTMPDIR="$(cd "${TMPDIR:-/tmp}" && pwd -P)"\nexport TMPDIR\n';

export const CONTROLLED_NATIVE_PREFIX = [
  "# Exercise the contended native paths at the original 16-thread harness setting.",
  "run_step \"stress node opened unlink 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase167_live_physical_opened_unlink -- --test-threads=16",
  "run_step \"stress node journal pressure 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority -- --test-threads=16",
  "run_step \"stress node runtime reopen 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase167_runtime_reopen -- --test-threads=16",
  "run_step \"stress node durable copies 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase167_durable -- --test-threads=16",
  "run_step \"stress node filter transport 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase160_filter -- --test-threads=16",
  "run_step \"stress node peer filter 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase160_peer -- --test-threads=16",
  "run_step \"stress node peer session 16 threads\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase167_peer_session -- --test-threads=16",
  "",
  "# Preserve the caller's test setting outside the complete native suites, which",
  "# together cover every workspace package once: node, rpc, then the remainder.",
  "saved_rust_test_threads=\"${RUST_TEST_THREADS-}\"",
  "had_rust_test_threads=\"${RUST_TEST_THREADS+x}\"",
  "export RUST_TEST_THREADS=4",
].join("\n") + "\n";
export const CONTROLLED_NATIVE_SUFFIX = [
  "if [[ \"$had_rust_test_threads\" == x ]]; then",
  "  export RUST_TEST_THREADS=\"$saved_rust_test_threads\"",
  "else",
  "  unset RUST_TEST_THREADS",
  "fi",
].join("\n") + "\n";
export const COMPLETE_NATIVE_SUITES = [
  "run_step \"cargo test node\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features",
  "run_step \"cargo test rpc\" cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features",
  "run_step \"cargo test remainder\" cargo test --manifest-path packages/Cargo.toml --workspace --exclude open-bitcoin-node --exclude open-bitcoin-rpc --all-features",
].join("\n") + "\n";
export const CONTROLLED_NATIVE_STEPS = CONTROLLED_NATIVE_PREFIX.split("\n").filter(line => line.startsWith("run_step "));

/** Original receipt qualification only: history reads frozen exact-byte snapshots, never live tooling. */
export function historicalToolingView(sources: ReadonlyMap<string, string>): Map<string, string> {
  if (sha(sources.get(MANIFEST_FILE) ?? "") !== EXPECTED_MANIFEST_SHA) throw new Error("fresh Phase163 VERIFICATION gate/lifecycle/digest: historical tooling manifest changed or missing");
  const manifest = JSON.parse(sources.get(MANIFEST_FILE)!) as {
    bindings: { file: string; snapshot: string; original_sha: string; current_sha: string }[];
  };
  if (JSON.stringify(manifest.bindings.map(row => [row.file, row.snapshot])) !== JSON.stringify(TOOLING_BINDINGS)) {
    throw new Error("historical tooling binding inventory changed");
  }
  const view = new Map(sources);
  for (const [file, snapshotFile, original] of GUARD_SNAPSHOTS) {
    const snapshot = sources.get(snapshotFile) ?? "";
    if (sha(snapshot) !== original) throw new Error(`fresh Phase163 VERIFICATION gate/lifecycle/digest: ${file}: original historical snapshot changed or missing`);
    view.set(file, snapshot);
  }
  for (const row of manifest.bindings) {
    const snapshot = sources.get(row.snapshot) ?? "";
    if (sha(snapshot) !== row.original_sha) throw new Error(`fresh Phase163 VERIFICATION gate/lifecycle/digest: ${row.file}: original historical snapshot changed or missing`);
    view.set(row.file, snapshot);
  }
  return view;
}

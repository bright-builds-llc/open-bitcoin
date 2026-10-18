import { createHash } from "node:crypto";
import { PHASE162_SOURCE_INVERSE, maybeRestoreReviewedSource } from "./contracts.ts";

export const MIGRATED_FIXTURE = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs";
const ORIGINAL_FIXTURE_SHA = "faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615";
const ORIGINAL_IMPORT = "use crate::{AuthoritativeNetworkRuntime, open_authoritative_network_runtime, open_runtime_store};";
const FIXTURE_IMPORT = "use crate::AuthoritativeNetworkRuntime;\nuse crate::tests::fixture_runtime::{\n    fixture_open_authoritative_network_runtime as open_authoritative_network_runtime,\n    fixture_open_runtime_store as open_runtime_store,\n};";
const ORDINARY_CONTEXT = "ManagedRpcContext::from_runtime_config_with_network_handle(";
const FIXTURE_CONTEXT = "ManagedRpcContext::from_fixture_runtime_config_with_network_handle(";
const NEW_INCLUDE = "pub(super) mod live_source {\n    include!(concat!(\n        env!(\"CARGO_MANIFEST_DIR\"),\n        \"/tests/support/phase165_live_source.rs\"\n    ));\n}\n";
const OLD_SEED = "        let runtime = DurableSyncRuntime::open(\n            FjallNodeStore::open(&path).expect(\"fresh real Fjall\"),\n            SyncRuntimeConfig {\n                network: SyncNetwork::Regtest,\n                ..Default::default()\n            },\n        )\n        .expect(\"unindexed actual runtime\");\n        let params = runtime.config().network.consensus_params();\n        let recipe = recipe::Recipe::new(params, ScriptVerifyFlags::P2SH, count);\n        let mut receipts = Vec::with_capacity(recipe.blocks.len());\n        for block in &recipe.blocks {\n            runtime\n                .store()\n                .save_block(block, PersistMode::Sync)\n                .expect(\"ordinary body persistence\");\n            let receipt = runtime\n                .network_handle()\n                .connect_local_block(block, recipe.flags, recipe.params)\n                .expect(\"normal-maturity managed acceptance\");\n            assert_eq!(receipt.height as usize, receipts.len());\n            assert_eq!(receipt.block_hash, block_hash(&block.header));\n            receipts.push(receipt);\n        }\n";
const NEW_SEED = "        let config = SyncRuntimeConfig {\n            network: SyncNetwork::Regtest,\n            ..Default::default()\n        };\n        let params = config.network.consensus_params();\n        let recipe = recipe::Recipe::new(params, ScriptVerifyFlags::P2SH, count);\n        let (runtime, genesis): (DurableSyncRuntime, ChainPosition) =\n            live_source::live_genesis_runtime(\n                &path,\n                &recipe.blocks[0],\n                config,\n                params,\n                recipe.flags,\n                open_bitcoin_node::chainstate::BasicFilterStartupMode::Disabled,\n            );\n        live_source::admit_descendant_headers(&runtime, &recipe.blocks[1..], recipe.flags, params);\n        let mut receipts = Vec::with_capacity(recipe.blocks.len());\n        for (height, block) in recipe.blocks.iter().enumerate() {\n            let receipt = if height == 0 {\n                genesis.clone()\n            } else {\n                runtime\n                    .store()\n                    .save_block(block, PersistMode::Sync)\n                    .expect(\"ordinary body persistence\");\n                runtime\n                    .network_handle()\n                    .connect_local_block(block, recipe.flags, recipe.params)\n                    .expect(\"normal-maturity managed acceptance\")\n            };\n            assert_eq!(receipt.height as usize, receipts.len());\n            assert_eq!(receipt.block_hash, block_hash(&block.header));\n            receipts.push(receipt);\n        }\n";

/** Complete reviewed wiring data only; no symbol stripping or unknown source normalization. */
function maybeHistoricalSource(source: string): string | undefined {
  const once = (needle: string) => source.split(needle).length === 2;
  if (!once(FIXTURE_IMPORT) || !once(FIXTURE_CONTEXT) || source.includes(ORIGINAL_IMPORT) || source.includes(ORDINARY_CONTEXT)) return undefined;
  const historical = source.replace(FIXTURE_IMPORT, ORIGINAL_IMPORT).replace(FIXTURE_CONTEXT, ORDINARY_CONTEXT);
  return createHash("sha256").update(historical).digest("hex") === ORIGINAL_FIXTURE_SHA ? historical : undefined;
}

/** Historical attribution only: exact reviewed wiring substitutions and original SHA required.
 * Raw current bytes retain a different digest and never earn ordinary qualification.
 */
export function fixtureSourceAttribution(source: string): {
  currentHash: string; historicalHash: string; evidenceClass: "original-historical" | "current-explicit-fixture" | "unproved"; historicalSource: string;
} {
  const sha = (value: string) => createHash("sha256").update(value).digest("hex");
  const currentHash = sha(source);
  if (currentHash === ORIGINAL_FIXTURE_SHA) return { currentHash, historicalHash: currentHash, evidenceClass: "original-historical", historicalSource: source };
  const normalized = currentHash === PHASE162_SOURCE_INVERSE?.currentRawSha
    ? maybeRestoreReviewedSource(source, PHASE162_SOURCE_INVERSE) : source;
  if (normalized === undefined) return { currentHash, historicalHash: currentHash, evidenceClass: "unproved", historicalSource: source };
  const once = (needle: string) => normalized.split(needle).length === 2;
  let candidate = normalized;
  if (once(NEW_INCLUDE) && once(NEW_SEED) && !normalized.includes(OLD_SEED)
    && normalized.includes("use super::{core_types, recipe};\n" + NEW_INCLUDE)
    && normalized.includes("        let started = Instant::now();\n" + NEW_SEED + "        assert!(\n            runtime")) {
    candidate = normalized.replace(NEW_INCLUDE, "").replace(NEW_SEED, OLD_SEED);
  }
  const maybeHistorical = maybeHistoricalSource(candidate);
  if (maybeHistorical !== undefined) return { currentHash, historicalHash: sha(maybeHistorical), evidenceClass: "current-explicit-fixture", historicalSource: maybeHistorical };
  return { currentHash, historicalHash: currentHash, evidenceClass: "unproved", historicalSource: source };
}

/** Copy only the historical claim input; executable current contracts retain raw source. */
export function historicalFixtureView(sources: ReadonlyMap<string, string>): Map<string, string> {
  const view = new Map(sources);
  if (sources.has(MIGRATED_FIXTURE)) view.set(MIGRATED_FIXTURE, fixtureSourceAttribution(sources.get(MIGRATED_FIXTURE)!).historicalSource);
  return view;
}

export const LEGACY_CONTRACT_BUNDLE = "packages/testdata/phase165/legacy-c-fixtures.json";


const LEGACY_BUNDLE_SHA = "1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c";
const LEGACY_BUNDLE_BYTES = 146752;
const LEGACY_METADATA = {
  "schemaVersion": 1,
  "evidence_class": "phase165-legacy-function-contract-bridge",
  "pre_c_capture": {
    "captured_at": "2026-10-17T05:20:52.045Z",
    "git_base": "d0104a997c0362bf65b81de1c8fec8ddc3012a1b",
    "scope": "pre-C workspace source; prospective legacy function-contract bridge only",
    "artifact_bytes": 124139,
    "artifact_sha256": "4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981"
  },
  "base_git_capture": {
    "captured_at": "2026-10-17T07:07:37.648Z",
    "scope": "Phase165 legacy structural contract baseline from authenticated pre-phase Git HEAD; not the seven-source pre-C snapshot or a runtime qualification receipt",
    "base_commit": "d0104a997c0362bf65b81de1c8fec8ddc3012a1b",
    "git_blob": "31dc52b18c51ee4a03e0aba0412443111fd153d7",
    "file": "packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs",
    "bytes": 20234,
    "sha256": "831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb",
    "current_inputs": [
      {
        "file": "packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs",
        "sha256": "e449aeed8ae0a65e23e7e654a20cccd4cc8609f982e2008b48aa372dbb777e8d"
      },
      {
        "file": "packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs",
        "sha256": "418404faac133e33635924f4881458a73bc9402f50df1d414dfd571ed12373ac"
      }
    ],
    "artifact_bytes": 22002,
    "artifact_sha256": "2f15d035bfc1e57bc9ef2a9e93f69607d8c46b24ba441938ae5236b24a9bf783"
  }
};
const LEGACY_VERSIONS: readonly (readonly [string, string, number, string])[] = [
  [
    "packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs",
    "623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580",
    14611,
    "677a3a5def6d6be380454ee029bfecf18b1d0de0fd3a25a0248d7f9ee4638160"
  ],
  [
    "packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs",
    "3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020",
    11654,
    "c35b11b189473843d2c34b95a75109c30bb064925019f8bf916c56fdf9241d39"
  ],
  [
    "packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs",
    "43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a",
    11533,
    "43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a"
  ],
  [
    "packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs",
    "3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813",
    18737,
    "3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813"
  ],
  [
    "packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs",
    "abe2651ff16c3d872695e8bde1222de63c2b5a28c519ffa9da299ac9166ab8b6",
    22182,
    "4c8250c11f1a2654b5c70c6c7594f238b5b68777a35992b6bb15933ff1e9951b"
  ],
  [
    "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs",
    "933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18",
    15320,
    "1c1e5c2fcba87ea6ffda69c3ecfbe569a309d74bc39c011a0648bdc3afa36513"
  ],
  [
    "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs",
    "1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60",
    23886,
    "d89e0e47cd23861c17706ab36be8d6efb92528fa6e82f9ddfd8a731d774652ca"
  ],
  [
    "packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs",
    "831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb",
    20234,
    "e449aeed8ae0a65e23e7e654a20cccd4cc8609f982e2008b48aa372dbb777e8d"
  ]
];
const CURRENT_HEADER_CHILD = "packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs";
const CURRENT_HEADER_CHILD_SHA = "418404faac133e33635924f4881458a73bc9402f50df1d414dfd571ed12373ac";
const legacySha = (source: string) => createHash("sha256").update(source).digest("hex");

/** Authenticate inert data before parsing; fixed code constants alone own the mapping. */
function authenticatedLegacySources(sources: ReadonlyMap<string, string>): readonly { path: string; sha256: string; source: string }[] {
  const maybeBundle = sources.get(LEGACY_CONTRACT_BUNDLE);
  if (maybeBundle === undefined || Buffer.byteLength(maybeBundle) > 512_000
    || Buffer.byteLength(maybeBundle) !== LEGACY_BUNDLE_BYTES || legacySha(maybeBundle) !== LEGACY_BUNDLE_SHA) {
    throw new Error(LEGACY_CONTRACT_BUNDLE + ": exact authenticated legacy bundle required");
  }
  const bundle = JSON.parse(maybeBundle);
  const { files, ...metadata } = bundle;
  if (JSON.stringify(metadata) !== JSON.stringify(LEGACY_METADATA) || !Array.isArray(files)
    || files.length !== LEGACY_VERSIONS.length) throw new Error(LEGACY_CONTRACT_BUNDLE + ": legacy bundle metadata/order required");
  for (const [index, [file, digest, bytes]] of LEGACY_VERSIONS.entries()) {
    const row = files[index];
    if (row === null || typeof row !== "object" || JSON.stringify(Object.keys(row)) !== '["path","sha256","source"]'
      || row.path !== file || row.sha256 !== digest || typeof row.source !== "string"
      || Buffer.byteLength(row.source) !== bytes || legacySha(row.source) !== digest) {
      throw new Error(LEGACY_CONTRACT_BUNDLE + ": exact legacy source row " + file + " required");
    }
  }
  return files;
}

/** Authenticated historical structural evidence only; no current authority. */
export function legacyFunctionContractView(sources: ReadonlyMap<string, string>): Map<string, string> {
  const rows = authenticatedLegacySources(sources);
  for (const [file, legacyDigest, , currentDigest] of LEGACY_VERSIONS) {
    const maybeSource = sources.get(file);
    if (maybeSource === undefined || ![legacyDigest, currentDigest].includes(legacySha(maybeSource))) {
      throw new Error(file + ": legacy function source version is not reviewed");
    }
  }
  const maybeChild = sources.get(CURRENT_HEADER_CHILD);
  if (maybeChild === undefined || legacySha(maybeChild) !== CURRENT_HEADER_CHILD_SHA) {
    throw new Error(CURRENT_HEADER_CHILD + ": legacy function source version is not reviewed");
  }
  const view = new Map(sources);
  for (const [index, [file]] of LEGACY_VERSIONS.entries()) view.set(file, rows[index].source);
  return view;
}

/** Explicit synthetic legacy-test baseline, separate from current acceptance. */
export function legacyFunctionSources(sources: ReadonlyMap<string, string>): Map<string, string> {
  return legacyFunctionContractView(sources);
}

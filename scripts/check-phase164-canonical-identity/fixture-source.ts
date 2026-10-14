import { createHash } from "node:crypto";

export const MIGRATED_FIXTURE = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs";
const ORIGINAL_FIXTURE_SHA = "faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615";
const ORIGINAL_IMPORT = "use crate::{AuthoritativeNetworkRuntime, open_authoritative_network_runtime, open_runtime_store};";
const FIXTURE_IMPORT = "use crate::AuthoritativeNetworkRuntime;\nuse crate::tests::fixture_runtime::{\n    fixture_open_authoritative_network_runtime as open_authoritative_network_runtime,\n    fixture_open_runtime_store as open_runtime_store,\n};";
const ORDINARY_CONTEXT = "ManagedRpcContext::from_runtime_config_with_network_handle(";
const FIXTURE_CONTEXT = "ManagedRpcContext::from_fixture_runtime_config_with_network_handle(";

/** Historical attribution only: exact reviewed wiring substitutions and original SHA required.
 * Raw current bytes retain a different digest and never earn ordinary qualification.
 */
export function fixtureSourceAttribution(source: string): {
  currentHash: string; historicalHash: string; evidenceClass: "original-historical" | "current-explicit-fixture" | "unproved"; historicalSource: string;
} {
  const sha = (value: string) => createHash("sha256").update(value).digest("hex");
  const currentHash = sha(source);
  if (currentHash === ORIGINAL_FIXTURE_SHA) return { currentHash, historicalHash: currentHash, evidenceClass: "original-historical", historicalSource: source };
  const once = (needle: string) => source.split(needle).length === 2;
  if (once(FIXTURE_IMPORT) && once(FIXTURE_CONTEXT) && !source.includes(ORIGINAL_IMPORT) && !source.includes(ORDINARY_CONTEXT)) {
    const historicalSource = source.replace(FIXTURE_IMPORT, ORIGINAL_IMPORT).replace(FIXTURE_CONTEXT, ORDINARY_CONTEXT);
    const historicalHash = sha(historicalSource);
    if (historicalHash === ORIGINAL_FIXTURE_SHA) return { currentHash, historicalHash, evidenceClass: "current-explicit-fixture", historicalSource };
  }
  return { currentHash, historicalHash: currentHash, evidenceClass: "unproved", historicalSource: source };
}

/** Copy only the historical claim input; executable current contracts retain raw source. */
export function historicalFixtureView(sources: ReadonlyMap<string, string>): Map<string, string> {
  const view = new Map(sources);
  if (sources.has(MIGRATED_FIXTURE)) view.set(MIGRATED_FIXTURE, fixtureSourceAttribution(sources.get(MIGRATED_FIXTURE)!).historicalSource);
  return view;
}

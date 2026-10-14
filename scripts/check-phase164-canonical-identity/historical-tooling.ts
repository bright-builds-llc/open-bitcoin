import { createHash } from "node:crypto";
import manifest from "../../packages/testdata/phase164/historical-tooling/manifest.json";

export const MANIFEST_FILE = "packages/testdata/phase164/historical-tooling/manifest.json";
export const CURRENT_TOOLING_FILES = manifest.bindings.map(row => row.file);
export const HISTORICAL_TOOLING_FILES = [MANIFEST_FILE, ...manifest.bindings.map(row => row.snapshot),
  "scripts/check-phase164-canonical-identity/historical-tooling.ts", "scripts/check-phase164-canonical-identity/fixture-source.ts"];
export const EVIDENCE_SCOPE = "historical/source-derived-regression";
export const EXPECTED_MANIFEST_SHA = "9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0";
const sha = (source: string) => createHash("sha256").update(source).digest("hex");

/** Original receipt qualification only: portable exact-byte snapshots and reviewed raw tooling.
 * Unknown current bytes cannot reuse historical authority, even with a regenerated report.
 */
export function historicalToolingView(sources: ReadonlyMap<string, string>): Map<string, string> {
  if (sha(sources.get(MANIFEST_FILE) ?? "") !== EXPECTED_MANIFEST_SHA) throw new Error("fresh Phase163 VERIFICATION gate/lifecycle/digest: historical tooling manifest changed or missing");
  const view = new Map(sources);
  for (const row of manifest.bindings) {
    const snapshot = sources.get(row.snapshot) ?? "";
    const current = sources.get(row.file) ?? "";
    if (sha(snapshot) !== row.original_sha) throw new Error(`fresh Phase163 VERIFICATION gate/lifecycle/digest: ${row.file}: original historical snapshot changed or missing`);
    if (![row.original_sha, row.current_sha].includes(sha(current))) throw new Error(`fresh Phase163 VERIFICATION gate/lifecycle/digest: ${row.file}: unreviewed current tooling fingerprint`);
    view.set(row.file, snapshot);
  }
  return view;
}

/** Make raw/current versus historical aggregate digests visible without relabeling receipts. */
export function toolingAttribution(sources: ReadonlyMap<string, string>, files: readonly string[]): {
  evidence_scope: string; rawCurrentDigest: string; historicalDigest: string; currentOrdinaryQualification: false;
} {
  const digest = (input: ReadonlyMap<string, string>) => sha(files.map(file => `${file}:${sha(input.get(file) ?? "")}`).join("\n"));
  return { evidence_scope: EVIDENCE_SCOPE, rawCurrentDigest: digest(sources), historicalDigest: digest(historicalToolingView(sources)), currentOrdinaryQualification: false };
}

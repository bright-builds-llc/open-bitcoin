import { afterEach, expect, test } from "bun:test";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { CHECK_FILES, CONTRACTS, HELPER_CONTRACTS, MODULE_EDGES, TEST_CONTRACTS, checkPhase160PeerFilterServing, maybeSelector } from "./check-phase160-peer-filter-serving.ts";
import { compact, maybeFunction, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";

const roots: string[] = [];
const repo = resolve(import.meta.dir, "..");
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "phase160-guard-"));
  roots.push(root);
  for (const file of CHECK_FILES) {
    mkdirSync(dirname(join(root, file)), { recursive: true });
    copyFileSync(join(repo, file), join(root, file));
  }
  return root;
}
function edit(root: string, file: string, change: (source: string) => string): void {
  const source = readFileSync(join(root, file), "utf8");
  const changed = change(source);
  expect(changed).not.toBe(source);
  writeFileSync(join(root, file), changed);
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

test("final scoped production/action/assertion links satisfy the lexical guard", () => {
  // Arrange / Act / Assert
  expect(checkPhase160PeerFilterServing(fixture())).toEqual([]);
});

test("content-identity caching observes mutation and restoration across checks", () => {
  // Arrange
  const root = fixture();
  const file = "packages/open-bitcoin-node/src/network/filter_transport/pending.rs";
  const original = readFileSync(join(root, file), "utf8");
  expect(checkPhase160PeerFilterServing(root)).toEqual([]);
  // Act / Assert
  edit(root, file, source => source.replace("registry.generic.len() >= 1024", "registry.generic.len() >= 102400"));
  expect(checkPhase160PeerFilterServing(root).some(failure => failure.includes("reserve_pending"))).toBe(true);
  writeFileSync(join(root, file), original);
  expect(checkPhase160PeerFilterServing(root)).toEqual([]);
});

for (const [file, symbol, anchors] of [...CONTRACTS, ...TEST_CONTRACTS, ...HELPER_CONTRACTS]) {
  for (const [index, anchor] of anchors.entries()) {
    test(`${symbol} rejects removed critical link ${index}`, () => {
      // Arrange
      const root = fixture();
      edit(root, file, source => {
        const body = maybeSelector(source, symbol);
        if (!body) throw new Error(`Missing ${symbol}`);
        const code = rustCode(source.slice(body.start, body.end));
        const positions = [...code.matchAll(/\S/g)].map(match => match.index);
        const needle = compact(anchor);
        const compacted = compact(code);
        const ranges: [number, number][] = [];
        for (let at = compacted.indexOf(needle); at >= 0; at = compacted.indexOf(needle, at + needle.length)) {
          ranges.push([body.start + positions[at], body.start + positions[at + needle.length - 1] + 1]);
        }
        expect(ranges.length).toBeGreaterThan(0);
        let changed = source;
        for (const [start, end] of ranges.reverse()) changed = changed.slice(0, start) + "removed_critical_link" + changed.slice(end);
        return changed;
      });
      // Act / Assert
      expect(checkPhase160PeerFilterServing(root).some(failure => failure.includes(file) && failure.includes(symbol))).toBe(true);
    });
  }
}

const N = "packages/open-bitcoin-node/src/";
const W = "packages/open-bitcoin-network/src/";
const DOC = "docs/parity/catalog/basic-compact-filters.md";
const mutations: [string, string, (source: string) => string][] = [
  ["caller snapshots replace atomic admission", N + "network/filter_transport.rs", source => source.replace("fn next_frame(&mut self)", "fn next_frame(&mut self, peer_pending: u64, aggregate_pending: u64)")],
  ["wrong exact stored source registration", "docs/parity/source-breadcrumbs.json", source => {
    const manifest = JSON.parse(source);
    for (const group of manifest.groups) if (group.files?.includes(N + "network/filter_transport/pending.rs")) group.files = group.files.filter((file: string) => file !== N + "network/filter_transport/pending.rs");
    return JSON.stringify(manifest);
  }],
  ["changed resource cap", W + "filter_serving/resource.rs", source => source.replace("BASIC_PEER_MAX_RECORD_READS: u64 = 4096", "BASIC_PEER_MAX_RECORD_READS: u64 = 40960")],
  ["lost all plus explicit corner", W + "filter_serving/capability/tests.rs", source => source.replace('(&["all", "blockfilters"], true, true)', '(&["all"], true, true)')],
  ["peer request cannot import serving implementation", N + "network/types.rs", source => source + "\nuse crate::network::runtime_authority::filter_index::serving::BasicFilterServeUnavailable;\n"],
  ["physical writer reencodes instead of exact bytes", N + "sync/tcp.rs", source => source.replace(".write_all(bytes)", ".write_all(&decode_wire(bytes).encode_wire(magic))")],
  ["enqueue substitutes for full physical write", N + "sync/session/filter_serving.rs", source => source.replace("session.send_encoded(frame.bytes(), magic)?", "session.enqueue(frame.bytes(), magic)?")],
  ["native test discovery is no longer explicit", "scripts/verify.sh", source => source.replaceAll("bun test ./scripts/check-phase160-peer-filter-serving.test.ts", "bun test scripts/check-phase160-peer-filter-serving.test.ts")],
  ["independent corpus check removed", "scripts/verify.sh", source => source.replaceAll("bun run scripts/generate-peer-filter-fixtures.ts --check", "bun run scripts/generate-peer-filter-fixtures.ts --write")],
  ["independent pin substituted", "docs/parity/fixtures/peer-filter-wire.json", source => source.replaceAll("a9aee730466ac67d35a3c03ee24676be5e045878", "0000000000000000000000000000000000000000")],
];
for (const claim of ["v2.5 is complete.", "v2.5 is now fully complete.", "CFOP-01 is complete.", "CFGR-01 passed.", "Phase 161 is complete.", "Phase 162 is now proven.", "BIP37 serving is supported.", "V0 enabled.", "production ready.", "production-funds safety is proven.", "archive proven.", "public peer serving enabled."]) {
  mutations.push([`unsupported ${claim}`, DOC, source => source + "\n" + claim + "\n"]);
}
for (const [name, file, change] of mutations) {
  test(name, () => {
    // Arrange
    const root = fixture();
    edit(root, file, change);
    // Act / Assert
    expect(checkPhase160PeerFilterServing(root).some(failure => failure.includes(file))).toBe(true);
  });
}
for (const { parent, symbol } of MODULE_EDGES) {
  test(`unreachable owner ${parent}:${symbol} cannot earn evidence`, () => {
    // Arrange
    const root = fixture();
    edit(root, parent, source => source.replace(new RegExp(`\\bmod\\s+${symbol}\\s*;`), `mod removed_${symbol};`));
    // Act / Assert
    expect(checkPhase160PeerFilterServing(root).some(failure => failure.includes(parent))).toBe(true);
  });
}

for (const [file, symbol] of TEST_CONTRACTS) {
  for (const mode of ["no-op", "ignored", "cfg-disabled"]) {
    test(`${symbol} rejects ${mode} evidence`, () => {
      // Arrange
      const root = fixture();
      edit(root, file, source => {
        const body = maybeFunction(source, symbol);
        if (!body) throw new Error(`Missing ${symbol}`);
        if (mode === "no-op") {
          const open = source.indexOf("{", body.start);
          return source.slice(0, open + 1) + " assert!(true); " + source.slice(body.end - 1);
        }
        const start = source.lastIndexOf("\n", body.start) + 1;
        return source.slice(0, start) + (mode === "ignored" ? "#[ignore]\n" : "#[cfg(any())]\n") + source.slice(start);
      });
      // Act / Assert
      expect(checkPhase160PeerFilterServing(root).some(failure => failure.includes(file) && failure.includes(symbol))).toBe(true);
    });
  }
}

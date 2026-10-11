import { beforeAll, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { mkdtempSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import {
  PIN, LIMITS, WIRE_PATH, PROOF_PATH, buildCorpus, generate, validateCorpus,
  validateResult, persistFixtures, runBounded,
} from "./generate-peer-filter-fixtures";

const ROOT = resolve(import.meta.dir, "..");
let fixtures: { wire: string; proof: string };
beforeAll(() => { fixtures = generate(); });
function oracleOutput(): string {
  const wire = JSON.parse(fixtures.wire);
  const proof = JSON.parse(fixtures.proof);
  return JSON.stringify({ wire: wire.cases, proof: proof.cases, handlers: wire.handler_cases, malformed: wire.malformed_cases });
}
function rowNamed(rows: Record<string, any>[], name: string): Record<string, any> {
  const maybeRow = rows.find(row => row.name === name);
  if (!maybeRow) throw new Error(`missing fixture ${name}`);
  return maybeRow;
}
function withFixtures(action: (root: string) => void): void {
  const root = mkdtempSync(resolve(tmpdir(), "peer-filter-fixtures-"));
  mkdirSync(resolve(root, "docs/parity/fixtures"), { recursive: true });
  try { action(root); } finally { rmSync(root, { recursive: true }); }
}

test("refuses a wrong pin before invoking the oracle", () => {
  // Arrange
  const calls: string[] = [];
  // Act / Assert
  expect(() => generate(undefined, (command) => {
    calls.push(command);
    return "0".repeat(40);
  })).toThrow("pin mismatch");
  expect(calls).toEqual(["git"]);
});

test("refuses a corpus exceeding the finite case budget", () => {
  // Arrange
  const corpus = buildCorpus();
  corpus.wire = Array(LIMITS.cases + 1).fill(corpus.wire[0]);
  // Act / Assert
  expect(() => validateCorpus(corpus)).toThrow("case budget");
});

test("refuses malformed oracle JSON before parsing contracts", () => {
  expect(() => validateResult("{}", buildCorpus())).toThrow();
});

test("refuses oversized oracle output before JSON parsing", () => {
  expect(() => validateResult(" ".repeat(LIMITS.stdout + 1), buildCorpus())).toThrow("output budget");
});

test("read-only check refuses drift without replacing either file", () => {
  // Arrange
  const root = mkdtempSync(resolve(tmpdir(), "peer-filter-check-"));
  mkdirSync(resolve(root, "docs/parity/fixtures"), { recursive: true });
  writeFileSync(resolve(root, WIRE_PATH), "old wire");
  writeFileSync(resolve(root, PROOF_PATH), "old proof");
  try {
    // Act / Assert
    expect(() => persistFixtures(root, { wire: "new wire", proof: "new proof" }, "--check")).toThrow("drift");
    expect(readFileSync(resolve(root, WIRE_PATH), "utf8")).toBe("old wire");
    expect(readFileSync(resolve(root, PROOF_PATH), "utf8")).toBe("old proof");
  } finally {
    rmSync(root, { recursive: true });
  }
});

test("pinned helper serializes all six families independently", () => {
  // Arrange / Act
  const fixtures = generate();
  const wire = JSON.parse(fixtures.wire);
  // Assert
  expect(wire.authority.pin).toBe(PIN);
  expect(new Set(wire.cases.map((row: { command: string }) => row.command)).size).toBe(6);
});

test.each([
  ["raw hash", (corpus: ReturnType<typeof buildCorpus>) => { corpus.wire[0].input.stop_hash_raw = "00"; }],
  ["unsafe height", (corpus: ReturnType<typeof buildCorpus>) => { corpus.wire[0].input.start_height = Number.MAX_SAFE_INTEGER + 1; }],
  ["lossy work number", (corpus: ReturnType<typeof buildCorpus>) => { corpus.proof[0].input.to_work = 1; }],
  ["oversized work", (corpus: ReturnType<typeof buildCorpus>) => { corpus.proof[0].input.to_work = (1n << 256n).toString(); }],
  ["unknown field", (corpus: ReturnType<typeof buildCorpus>) => { corpus.wire[0].input.extra = true; }],
  ["payload limit", (corpus: ReturnType<typeof buildCorpus>) => { corpus.wire[12].input.filter_data = "ff".repeat(LIMITS.payload); }],
])("corpus refuses %s violations", (_name, mutate) => {
  // Arrange
  const corpus = buildCorpus();
  mutate(corpus);
  // Act / Assert
  expect(() => validateCorpus(corpus)).toThrow();
});

test.each([
  ["unknown field", (result: Record<string, any>) => { result.wire[0].expected.extra = true; }],
  ["missing case", (result: Record<string, any>) => { result.wire.pop(); }],
  ["input mutation", (result: Record<string, any>) => { result.wire[0].input.filter_type = 1; }],
  ["frame byte mutation", (result: Record<string, any>) => { result.wire[0].expected.frame_hex = "00" + result.wire[0].expected.frame_hex.slice(2); }],
  ["payload byte mutation", (result: Record<string, any>) => { result.wire[0].expected.payload_hex = "ff" + result.wire[0].expected.payload_hex.slice(2); }],
  ["self-consistent raw digest mutation", (result: Record<string, any>) => {
    const row = result.wire[0];
    const payload = Buffer.from(row.expected.payload_hex, "hex");
    payload[5] ^= 255;
    const frame = Buffer.from(row.expected.frame_hex, "hex");
    payload.copy(frame, 24);
    createHash("sha256").update(createHash("sha256").update(payload).digest()).digest().copy(frame, 20, 0, 4);
    row.expected.payload_hex = payload.toString("hex");
    row.expected.frame_hex = frame.toString("hex");
  }],
  ["lossy proof number", (result: Record<string, any>) => { result.proof[0].expected.proof = 0; }],
  ["signed overflow", (result: Record<string, any>) => { result.proof[0].expected.equivalent_time = (1n << 63n).toString(); }],
  ["noncanonical sign", (result: Record<string, any>) => { result.proof[0].expected.sign = "-1"; }],
])("oracle refuses %s before fixture replacement", (_name, mutate) => {
  // Arrange
  const result = JSON.parse(oracleOutput());
  mutate(result);
  withFixtures(root => {
    persistFixtures(root, fixtures, "--write");
    // Act / Assert
    expect(() => persistFixtures(root, validateResult(JSON.stringify(result), buildCorpus()), "--write")).toThrow();
    expect(readFileSync(resolve(root, WIRE_PATH), "utf8")).toBe(fixtures.wire);
    expect(readFileSync(resolve(root, PROOF_PATH), "utf8")).toBe(fixtures.proof);
  });
});

test("pin mismatch leaves both existing goldens unchanged", () => {
  withFixtures(root => {
    // Arrange
    persistFixtures(root, fixtures, "--write");
    // Act / Assert
    expect(() => persistFixtures(root, generate(root, () => "bad-pin"), "--write")).toThrow("pin mismatch");
    expect(readFileSync(resolve(root, WIRE_PATH), "utf8")).toBe(fixtures.wire);
    expect(readFileSync(resolve(root, PROOF_PATH), "utf8")).toBe(fixtures.proof);
  });
});

test("dirty pinned source refuses before Python execution", () => {
  // Arrange
  let calls = 0;
  // Act / Assert
  expect(() => generate(ROOT, () => ++calls === 1 ? PIN : " M src/chain.cpp")).toThrow("tracked inputs");
  expect(calls).toBe(2);
});

test("atomic fixture publication rejects a symlink destination", () => {
  withFixtures(root => {
    // Arrange
    const target = resolve(root, "target.json");
    writeFileSync(target, "keep");
    symlinkSync(target, resolve(root, PROOF_PATH));
    // Act / Assert
    expect(() => persistFixtures(root, fixtures, "--write")).toThrow("symlink");
    expect(readFileSync(target, "utf8")).toBe("keep");
    expect(readdirSync(resolve(root, "docs/parity/fixtures"))).toEqual(["peer-filter-proof.json"]);
  });
});

test("two real regenerations reproduce identical goldens", () => {
  // Arrange / Act
  const second = generate();
  // Assert
  expect(second).toEqual(fixtures);
  withFixtures(root => {
    persistFixtures(root, fixtures, "--write");
    persistFixtures(root, second, "--write");
    expect(readFileSync(resolve(root, WIRE_PATH), "utf8")).toBe(fixtures.wire);
    expect(readFileSync(resolve(root, PROOF_PATH), "utf8")).toBe(fixtures.proof);
    expect(readdirSync(resolve(root, "docs/parity/fixtures"))).toHaveLength(2);
  });
});

test("successful check is read-only including modification times", () => {
  withFixtures(root => {
    // Arrange
    persistFixtures(root, fixtures, "--write");
    const before = [WIRE_PATH, PROOF_PATH].map(file => statSync(resolve(root, file)).mtimeMs);
    // Act
    persistFixtures(root, fixtures, "--check");
    // Assert
    expect([WIRE_PATH, PROOF_PATH].map(file => statSync(resolve(root, file)).mtimeMs)).toEqual(before);
  });
});

test.each([WIRE_PATH, PROOF_PATH])("check detects byte mutation in %s", path => {
  withFixtures(root => {
    // Arrange
    persistFixtures(root, fixtures, "--write");
    const mutated = readFileSync(resolve(root, path), "utf8").replace(PIN, "0".repeat(40));
    writeFileSync(resolve(root, path), mutated);
    // Act / Assert
    expect(() => persistFixtures(root, fixtures, "--check")).toThrow("drift");
    expect(readFileSync(resolve(root, path), "utf8")).toBe(mutated);
  });
});

test("wire uint256 bytes retain non-palindromic raw hash order", () => {
  // Arrange
  const row = rowNamed(JSON.parse(fixtures.wire).cases, "getcfilters-0");
  const raw = row.input.stop_hash_raw;
  // Act / Assert
  expect(raw).not.toBe(Buffer.from(raw, "hex").reverse().toString("hex"));
  expect(row.expected.payload_hex.slice(10)).toBe(raw);
});

test.each([[252, "fc"], [253, "fdfd00"], [65535, "fdffff"], [65536, "fe00000100"]])(
  "cfilter length %i uses pinned CompactSize transition", (length, prefix) => {
    // Arrange
    const row = rowNamed(JSON.parse(fixtures.wire).cases, `cfilter-length-${length}`);
    // Act / Assert
    expect(row.expected.payload_hex.slice(66).startsWith(prefix)).toBe(true);
  },
);

test("cfheaders genesis uses zero predecessor while later ranges retain raw predecessor", () => {
  // Arrange
  const rows = JSON.parse(fixtures.wire).cases;
  const genesis = rowNamed(rows, "cfheaders-genesis");
  const later = rowNamed(rows, "cfheaders-count-1");
  // Act / Assert
  expect(genesis.expected.payload_hex.slice(66, 130)).toBe("00".repeat(32));
  expect(later.expected.payload_hex.slice(66, 130)).toBe(later.input.prev_header_raw);
  expect(later.input.prev_header_raw).not.toBe("00".repeat(32));
});

test.each([[0, 0], [999, 0], [1000, 1], [1001, 1], [2000, 2], [3001, 3]])(
  "checkpoint stop %i excludes genesis and uses positive 1000 intervals", (height, count) => {
    const row = rowNamed(JSON.parse(fixtures.wire).cases, `cfcheckpt-${height}`);
    expect(row.input.headers_raw).toHaveLength(count);
    expect(row.expected.payload_hex.slice(66, 68)).toBe(count.toString(16).padStart(2, "0"));
  },
);

test.each([[2591999, true], [2592000, false], [2592001, false], [-1, true]])(
  "source stale timestamp boundary %i is strict", (age, allowed) => {
    const row = rowNamed(JSON.parse(fixtures.proof).cases, `timestamp-age-${age}`);
    expect(row.expected.allowed).toBe(allowed);
  },
);

test.each([[2591999, true], [2592000, false], [2592001, false]])(
  "source proof-equivalent boundary %i is strict", (age, allowed) => {
    const row = rowNamed(JSON.parse(fixtures.proof).cases, `proof-age-${age}`);
    expect(row.expected.equivalent_time).toBe(String(age));
    expect(row.expected.allowed).toBe(allowed);
  },
);

test("256-bit multiplication wrap remains observable", () => {
  const row = rowNamed(JSON.parse(fixtures.proof).cases, "wrap-256-multiply");
  expect(row.expected.equivalent_time).toBe("0");
  expect(row.expected.saturated).toBe(false);
});

test.each([["saturate-positive", "9223372036854775807"], ["saturate-negative", "-9223372036854775807"]])(
  "%s uses signed INT64_MAX saturation", (name, expected) => {
    const row = rowNamed(JSON.parse(fixtures.proof).cases, name);
    expect(row.expected.equivalent_time).toBe(expected);
    expect(row.expected.saturated).toBe(true);
  },
);

test("subprocess output budget stops an unbounded writer", () => {
  expect(() => runBounded("yes", [], ROOT)).toThrow(/maxBuffer|ENOBUFS/);
});

test("subprocess work budget terminates a stalled helper", () => {
  expect(() => runBounded("sleep", ["20"], ROOT)).toThrow(/timed out|ETIMEDOUT/);
}, 15_000);

test("subprocess input is refused before process launch", () => {
  expect(() => runBounded("missing-command", [], ROOT, " ".repeat(LIMITS.input + 1))).toThrow("input budget");
});

test("BASIC default oracle still reproduces its unchanged corpus", () => {
  // Arrange
  const path = resolve(ROOT, "packages/open-bitcoin-consensus/testdata/basic_filter_vectors.rs");
  const before = readFileSync(path, "utf8");
  // Act
  const output = runBounded(process.execPath, ["scripts/generate-basic-filter-vectors.ts", "--check"], ROOT);
  // Assert
  expect(output).toContain("ten pinned corpus agreements");
  expect(readFileSync(path, "utf8")).toBe(before);
});

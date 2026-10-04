import { afterEach, expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, symlinkSync, unlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { checkPhase154BasicFilters, CHECK_FILES } from "./check-phase154-basic-filters.ts";
import { checkVectors, foreignGitEnvironment, generate, readCorpus, runChild, FIXTURE, PIN } from "./generate-basic-filter-vectors.ts";

const REPO = resolve(import.meta.dir, "..");
const roots: string[] = [];
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "basic-filter-evidence-"));
  roots.push(root);
  for (const file of CHECK_FILES) {
    mkdirSync(dirname(join(root, file)), { recursive: true });
    writeFileSync(join(root, file), readFileSync(join(REPO, file)));
  }
  symlinkSync(join(REPO, "packages/bitcoin-knots"), join(root, "packages/bitcoin-knots"));
  return root;
}
function replace(root: string, file: string, before: string, after: string): void {
  const text = readFileSync(join(root, file), "utf8");
  expect(text.includes(before)).toBe(true);
  writeFileSync(join(root, file), text.replace(before, after));
}
function withEnvironment(values: Record<string, string>, operation: () => void): void {
  const previous = Object.entries(values).map(([name]) => [name, process.env[name]] as const);
  try {
    Object.assign(process.env, values);
    operation();
  } finally {
    for (const [name, maybeValue] of previous) {
      if (maybeValue === undefined) delete process.env[name];
      else process.env[name] = maybeValue;
    }
  }
}
afterEach(() => {
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

test("complete source fixture passes without phase completion artifacts", () => {
  // Arrange
  const root = fixture();
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures).toEqual([]);
});
test("missing exact baseline pin is actionable", () => {
  // Arrange
  const root = fixture();
  replace(root, FIXTURE, PIN, "wrong");
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures.some(item => item.includes("baseline pin"))).toBe(true);
});
test("missing source breadcrumb fails", () => {
  // Arrange
  const root = fixture();
  replace(root, "packages/open-bitcoin-consensus/tests/basic_filter.rs",
    "// - packages/bitcoin-knots/src/test/data/blockfilters.json", "// removed");
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures.some(item => item.includes("source breadcrumb"))).toBe(true);
});
test("missing parity evidence link fails", () => {
  // Arrange
  const root = fixture();
  replace(root, "docs/parity/index.json",
    '"packages/open-bitcoin-consensus/tests/basic_filter.rs"', '"missing-evidence.rs"');
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures.some(item => item.includes("surface evidence"))).toBe(true);
});
test("commenting default verifier command fails even when help retains command", () => {
  // Arrange
  const root = fixture();
  replace(root, "scripts/verify.sh",
    'run_step "reproduce Phase 154 independent BASIC vectors"',
    '# run_step "reproduce Phase 154 independent BASIC vectors"');
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures.some(item => item.includes("default verifier"))).toBe(true);
});
test("corpus count drift fails", () => {
  // Arrange
  const root = fixture();
  replace(root, FIXTURE, 'name: "corpus-0"', 'name: "renamed-genesis"');
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures.some(item => item.includes("ten corpus"))).toBe(true);
});
test("oracle cannot import Rust under test", () => {
  // Arrange
  const root = fixture();
  const file = join(root, "scripts/basic-filter-oracle.py");
  writeFileSync(file, readFileSync(file, "utf8") + "\nimport open_bitcoin_consensus\n");
  // Act
  const failures = checkPhase154BasicFilters(root);
  // Assert
  expect(failures.some(item => item.includes("independence"))).toBe(true);
});
for (const claim of [
  "Filter index activation is enabled.", "Compact filter RPC serving is supported.",
  "Compact filter peer serving is available.", "V0 is supported.", "BIP37 is enabled.",
  "GUI is shipped.", "Production readiness is ready.", "Production-funds use is safe.",
  "Filter index storage is available.", "Filter prune retention is supported.",
  "Compact-filter serving is enabled.", "Compact-filter catch-up is supported.",
]) {
  test(`rejects scoped overclaim: ${claim}`, () => {
    // Arrange
    const root = fixture();
    const file = join(root, "docs/parity/catalog/basic-compact-filters.md");
    writeFileSync(file, readFileSync(file, "utf8") + `\n${claim}\n`);
    // Act
    const failures = checkPhase154BasicFilters(root);
    // Assert
    expect(failures.some(item => item.includes("unsupported claim"))).toBe(true);
  });
}
test("altered expected fixture fails deterministic reproduction without rewriting it", () => {
  // Arrange
  const root = fixture();
  replace(root, FIXTURE, 'encoded: "019dfca8"', 'encoded: "00"');
  const altered = readFileSync(join(root, FIXTURE), "utf8");
  // Act / Assert
  expect(() => checkVectors(root)).toThrow("fixture drift");
  expect(readFileSync(join(root, FIXTURE), "utf8")).toBe(altered);
});
test("child nonzero exit propagates visibly", () => {
  // Arrange / Act / Assert
  expect(() => runChild("git", ["definitely-no-such-basic-filter-command"], REPO))
    .toThrow("failed");
});
test("oversized oracle input is refused before child execution", () => {
  // Arrange
  const input = "x".repeat(2_000_001);
  // Act / Assert
  expect(() => runChild("does-not-exist", [], REPO, input)).toThrow("input exceeds bound");
});
test("missing child executable fails visibly", () => {
  // Arrange / Act / Assert
  expect(() => runChild("no-such-basic-filter-oracle", [], REPO)).toThrow();
});

test("generator refuses wrong submodule pin before accepting expectations", () => {
  // Arrange
  const root = fixture();
  unlinkSync(join(root, "packages/bitcoin-knots"));
  symlinkSync(REPO, join(root, "packages/bitcoin-knots"));
  // Act / Assert
  expect(() => generate(root)).toThrow("Knots pin mismatch");
});
test("corpus parser refuses missing upstream row", () => {
  // Arrange
  const root = fixture();
  unlinkSync(join(root, "packages/bitcoin-knots"));
  const file = "packages/bitcoin-knots/src/test/data/blockfilters.json";
  mkdirSync(dirname(join(root, file)), { recursive: true });
  const rows = JSON.parse(readFileSync(join(REPO, file), "utf8"));
  writeFileSync(join(root, file), JSON.stringify(rows.slice(0, -1)));
  // Act / Assert
  expect(() => readCorpus(root)).toThrow("exactly ten corpus rows");
});
test("oracle disagreement with corpus is a blocker before rendering edges", () => {
  // Arrange
  const root = fixture();
  replace(root, "scripts/basic-filter-oracle.py", "encoded=encoded.hex()", 'encoded="00"');
  // Act / Assert
  expect(() => generate(root)).toThrow("independent oracle disagrees with pinned corpus");
});

test("hook relative index preserves real submodule pin and deterministic fixture", () => {
  // Arrange
  const root = fixture();
  const expected = readFileSync(join(root, FIXTURE), "utf8");
  withEnvironment({ GIT_INDEX_FILE: ".git/index" }, () => {
    // Act
    const actual = generate(root);
    checkVectors(root);
    // Assert
    expect(actual).toBe(expected);
    expect(readFileSync(join(root, FIXTURE), "utf8")).toBe(expected);
    expect(process.env.GIT_INDEX_FILE).toBe(".git/index");
  });
});
test("foreign Git ignores repository selectors and preserves global configuration", () => {
  // Arrange
  const root = fixture();
  const globalConfig = join(root, "global-git-config");
  writeFileSync(globalConfig, "[phase154]\n    authSentinel = preserved\n");
  const contaminated = {
    GIT_DIR: join(REPO, ".git"), GIT_WORK_TREE: REPO, GIT_COMMON_DIR: join(REPO, ".git"),
    GIT_OBJECT_DIRECTORY: join(REPO, ".git/objects"), GIT_INDEX_FILE: ".git/index",
    GIT_CONFIG_COUNT: "invalid-local-selector", GIT_PREFIX: "foreign/",
    GIT_CONFIG_GLOBAL: globalConfig, GIT_SSH_COMMAND: "phase154-auth-sentinel",
  };
  withEnvironment(contaminated, () => {
    // Act
    const actual = generate(root);
    const globalValue = runChild("git", ["config", "--global", "--get", "phase154.authSentinel"], root);
    const childEnv = foreignGitEnvironment();
    // Assert
    expect(actual).toBe(readFileSync(join(root, FIXTURE), "utf8"));
    expect(globalValue.trim()).toBe("preserved");
    expect(childEnv.GIT_SSH_COMMAND).toBe(contaminated.GIT_SSH_COMMAND);
    expect(childEnv.GIT_CONFIG_GLOBAL).toBe(globalConfig);
    expect(childEnv.PATH).toBe(process.env.PATH);
    expect(childEnv.GIT_DIR).toBeUndefined();
    expect(childEnv.GIT_INDEX_FILE).toBeUndefined();
    for (const [name, value] of Object.entries(contaminated)) expect(process.env[name]).toBe(value);
  });
});

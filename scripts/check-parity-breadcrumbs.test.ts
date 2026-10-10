import { afterEach, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

const script = resolve(import.meta.dir, "check-parity-breadcrumbs.ts");
const source = "packages/open-bitcoin-fixture/src/new.rs";
const anchor = "packages/bitcoin-knots/src/validation.cpp";
const roots: string[] = [];
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "breadcrumb-worktree-"));
  roots.push(root);
  const init = spawnSync("git", ["init", "--quiet", root], { encoding: "utf8" });
  expect(init.status).toBe(0);
  for (const file of [source, anchor, "docs/parity/source-breadcrumbs.json"]) mkdirSync(dirname(join(root, file)), { recursive: true });
  writeFileSync(join(root, anchor), "// pinned reference fixture\n");
  writeFileSync(join(root, source), `// Parity breadcrumbs:\n// - ${anchor}\n\npub fn retained() {}\n`);
  writeFileSync(join(root, "docs/parity/source-breadcrumbs.json"), JSON.stringify({ version: 1, noneReason: "No direct upstream anchor.", scope: { include: ["packages/open-bitcoin-*/src/**/*.rs"], exclude: ["packages/bitcoin-knots/**"] }, groups: [{ label: "fixture", files: [source], breadcrumbs: [anchor] }] }));
  return root;
}
function check(root: string) {
  return spawnSync(process.execPath, [script, "--check"], { cwd: root, encoding: "utf8" });
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true }); });

test("manifested untracked Rust is checked before staging", () => {
  // Arrange
  const root = fixture();
  expect(spawnSync("git", ["ls-files"], { cwd: root, encoding: "utf8" }).stdout).toBe("");
  // Act
  const valid = check(root);
  writeFileSync(join(root, source), "pub fn retained() {}\n");
  const missing = check(root);
  // Assert
  expect(valid.status).toBe(0);
  expect(valid.stdout).toContain("1 Rust file(s)");
  expect(missing.status).toBe(1);
  expect(missing.stderr).toContain("expected exactly one breadcrumb block");
});

test("unmanifested untracked Rust fails current worktree inventory", () => {
  // Arrange
  const root = fixture();
  const unknown = "packages/open-bitcoin-fixture/src/unknown.rs";
  writeFileSync(join(root, unknown), "pub fn unknown() {}\n");
  // Act
  const result = check(root);
  // Assert
  expect(result.status).toBe(1);
  expect(result.stderr).toContain(`missing breadcrumb mapping for ${unknown}`);
});

test("ignored untracked Rust stays outside current worktree inventory", () => {
  // Arrange
  const root = fixture();
  const ignored = "packages/open-bitcoin-fixture/src/generated.rs";
  writeFileSync(join(root, ".gitignore"), ignored + "\n");
  writeFileSync(join(root, ignored), "pub fn generated() {}\n");
  // Act
  const result = check(root);
  // Assert
  expect(result.status).toBe(0);
  expect(result.stdout).toContain("1 Rust file(s)");
});

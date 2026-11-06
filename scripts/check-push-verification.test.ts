import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  changedPathsAcrossCommits,
  executeMainPush,
  isSafeProsePath,
  parseNameStatus,
  parseRefUpdates,
  planMainPush,
  receiptMatches,
  type GitCommand,
  verifiedReceipt,
} from "./check-push-verification.ts";

const REMOTE = "a".repeat(40);
const LOCAL = "b".repeat(40);
const MAIN = "refs/heads/main";
const update = (local = LOCAL, remote = REMOTE) => `${MAIN} ${local} ${MAIN} ${remote}\n`;

type FakeState = {
  maybeBranch?: string;
  maybeHead?: string;
  maybeStatus?: Buffer;
  maybeAncestorStatus?: number;
  maybeChanged?: Buffer;
  maybeCommits?: string[];
  maybeSummary?: string;
  maybeTreeMode?: string;
};

function fakeGit(state: FakeState = {}): GitCommand {
  return (args) => {
    const key = args[0];
    const value = key === "symbolic-ref" ? state.maybeBranch ?? "main"
      : key === "rev-parse" ? state.maybeHead ?? LOCAL
      : key === "rev-list" ? (state.maybeCommits ?? [LOCAL]).join("\n")
      : "";
    const stdout = key === "status" ? state.maybeStatus ?? Buffer.alloc(0)
      : key === "diff-tree" ? args.includes("--summary")
        ? Buffer.from(state.maybeSummary ?? "")
        : state.maybeChanged ?? Buffer.from("M\0docs/notes.md\0")
      : key === "ls-tree" ? Buffer.from(`${state.maybeTreeMode ?? "100644"} blob ${"c".repeat(40)}\t${args.at(-1)}\0`)
      : Buffer.from(value);
    const status = key === "merge-base" ? state.maybeAncestorStatus ?? 0 : 0;
    return { status, stdout, stderr: "" };
  };
}

function gitIn(root: string): GitCommand {
  return (args) => {
    const result = spawnSync("git", ["-C", root, ...args], { encoding: "buffer" });
    if (result.error) throw result.error;
    return { status: result.status ?? -1, stdout: result.stdout, stderr: result.stderr.toString("utf8") };
  };
}

function runGit(root: string, ...args: string[]): string {
  const result = gitIn(root)(args);
  if (result.status !== 0) throw new Error(result.stderr);
  return result.stdout.toString("utf8").trim();
}

function docRepo(): { root: string; remote: string } {
  const root = mkdtempSync(join(tmpdir(), "open-bitcoin-push-mode-"));
  runGit(root, "init", "-q", "-b", "main");
  runGit(root, "config", "user.email", "test@example.test");
  runGit(root, "config", "user.name", "Push Check Test");
  runGit(root, "config", "core.filemode", "true");
  mkdirSync(join(root, "docs"));
  writeFileSync(join(root, "docs", "base.md"), "base\n");
  runGit(root, "add", "-A");
  runGit(root, "commit", "-qm", "base");
  return { root, remote: runGit(root, "rev-parse", "HEAD") };
}

test("Git pre-push updates require four fields and valid commit IDs", () => {
  // Arrange / Act / Assert
  expect(parseRefUpdates(update())).toHaveLength(1);
  expect(() => parseRefUpdates(`${MAIN} bad ${MAIN} ${REMOTE}\n`)).toThrow("malformed Git pre-push ref update");
});

test("NUL-delimited rename records keep both source and destination", () => {
  // Arrange
  const result = Buffer.from("R100\0docs/old.md\0packages/new.rs\0");
  // Act
  const paths = parseNameStatus(result);
  // Assert
  expect(paths).toEqual(["docs/old.md", "packages/new.rs"]);
});

test("all outgoing commits contribute paths, including both sides of a rename", () => {
  // Arrange
  const root = mkdtempSync(join(tmpdir(), "open-bitcoin-push-check-"));
  try {
    runGit(root, "init", "-q", "-b", "main");
    runGit(root, "config", "user.email", "test@example.test");
    runGit(root, "config", "user.name", "Push Check Test");
    mkdirSync(join(root, "docs", "parity"), { recursive: true });
    mkdirSync(join(root, "packages"));
    writeFileSync(join(root, "docs", "start.md"), "start\n");
    runGit(root, "add", "-A");
    runGit(root, "commit", "-qm", "base");
    const remote = runGit(root, "rev-parse", "HEAD");

    writeFileSync(join(root, "docs", "notes.md"), "notes\n");
    runGit(root, "add", "-A");
    runGit(root, "commit", "-qm", "docs");
    writeFileSync(join(root, "packages", "node.rs"), "fn main() {}\n");
    runGit(root, "add", "-A");
    runGit(root, "commit", "-qm", "code");
    runGit(root, "mv", "docs/notes.md", "docs/parity/notes.md");
    runGit(root, "commit", "-qm", "rename");
    const local = runGit(root, "rev-parse", "HEAD");

    // Act
    const paths = changedPathsAcrossCommits(gitIn(root), remote, local);
    const plan = planMainPush("origin", update(local, remote), gitIn(root));

    // Assert
    expect(paths).toEqual({ paths: ["docs/notes.md", "docs/parity/notes.md", "packages/node.rs"], requiresFull: true });
    expect(plan.mode).toBe("full");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("safe Markdown-only push gets quick verification", () => {
  // Arrange
  const git = fakeGit({ maybeChanged: Buffer.from("M\0docs/guide.md\0M\0.codex/tasks/todo.md\0") });
  // Act
  const plan = planMainPush("origin", update(), git);
  // Assert
  expect(plan.mode).toBe("quick");
});

test("parity, source, and unknown files require full verification", () => {
  // Arrange / Act / Assert
  for (const file of ["docs/parity/index.json", "packages/open-bitcoin-node/src/lib.rs", "new-file.xyz", "README.md"]) {
    const plan = planMainPush("origin", update(), fakeGit({ maybeChanged: Buffer.from(`M\0${file}\0`) }));
    expect(plan.mode).toBe("full");
  }
  expect(isSafeProsePath("docs/parity/scope.md")).toBe(false);
});

test("renames and file-type changes require full verification even within safe prose paths", () => {
  // Arrange / Act / Assert
  for (const changed of [
    "R100\0docs/old.md\0docs/new.md\0",
    "A\0docs/new.md\0D\0docs/old.md\0",
    "T\0docs/guide.md\0",
  ]) {
    expect(planMainPush("origin", update(), fakeGit({ maybeChanged: Buffer.from(changed) })).mode).toBe("full");
  }
});

test("a committed Markdown symlink requires full verification", () => {
  // Arrange
  const { root, remote } = docRepo();
  try {
    symlinkSync("base.md", join(root, "docs", "guide.md"));
    runGit(root, "add", "docs/guide.md");
    runGit(root, "commit", "-qm", "add guide link");
    const local = runGit(root, "rev-parse", "HEAD");
    // Act
    const plan = planMainPush("origin", update(local, remote), gitIn(root));
    // Assert
    expect(plan.changedPaths).toEqual(["docs/guide.md"]);
    expect(plan.mode).toBe("full");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("non-regular committed modes cannot use the safe prose route", () => {
  // Arrange / Act / Assert
  for (const mode of ["100755", "120000", "160000"]) {
    const git = fakeGit({ maybeChanged: Buffer.from("M\0docs/guide.md\0"), maybeTreeMode: mode });
    expect(planMainPush("origin", update(), git).mode).toBe("full");
  }
});

test("a committed executable-bit change requires full verification even when reverted before push", () => {
  // Arrange
  const { root, remote } = docRepo();
  try {
    const guide = join(root, "docs", "guide.md");
    writeFileSync(guide, "guide\n");
    runGit(root, "add", "docs/guide.md");
    runGit(root, "commit", "-qm", "add guide");
    chmodSync(guide, 0o755);
    runGit(root, "add", "docs/guide.md");
    runGit(root, "commit", "-qm", "make executable");
    chmodSync(guide, 0o644);
    runGit(root, "add", "docs/guide.md");
    runGit(root, "commit", "-qm", "restore regular mode");
    const local = runGit(root, "rev-parse", "HEAD");
    // Act
    const plan = planMainPush("origin", update(local, remote), gitIn(root));
    // Assert
    expect(plan.changedPaths).toEqual(["docs/guide.md"]);
    expect(plan.mode).toBe("full");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("outgoing edits to frozen GSD history refuse the push", () => {
  // Arrange
  const git = fakeGit({ maybeChanged: Buffer.from("M\0.planning/STATE.md\0") });
  // Act / Assert
  expect(() => planMainPush("origin", update(), git)).toThrow(".planning/ is frozen historical evidence");
  expect(() => planMainPush("origin", update(), fakeGit({ maybeChanged: Buffer.from("M\0.Planning/STATE.md\0") })))
    .toThrow(".planning/ is frozen historical evidence");
});

test("dirty worktree refuses a direct-main push", () => {
  // Arrange
  const git = fakeGit({ maybeStatus: Buffer.from("?? untracked.txt\0") });
  // Act / Assert
  expect(() => planMainPush("origin", update(), git)).toThrow("commit or remove worktree changes");
});

test("new or deleted remote main refuses the push", () => {
  // Arrange / Act / Assert
  expect(() => planMainPush("origin", update(LOCAL, "0".repeat(40)), fakeGit())).toThrow("creating or deleting origin/main");
  expect(() => planMainPush("origin", update("0".repeat(40), REMOTE), fakeGit())).toThrow("creating or deleting origin/main");
});

test("remote main must be an ancestor of local main", () => {
  // Arrange
  const git = fakeGit({ maybeAncestorStatus: 1 });
  // Act / Assert
  expect(() => planMainPush("origin", update(), git)).toThrow("not an ancestor");
});

test("branch and local ref must identify the current main HEAD", () => {
  // Arrange / Act / Assert
  expect(() => planMainPush("origin", update(), fakeGit({ maybeBranch: "dev" }))).toThrow("current local main HEAD");
  expect(() => planMainPush("origin", update(), fakeGit({ maybeHead: REMOTE }))).toThrow("current local main HEAD");
  expect(() => planMainPush("origin", `${MAIN} ${LOCAL} refs/heads/dev ${REMOTE}\n`, fakeGit())).toThrow("only a push of main");
});

test("an unchanged remote ref is a no-op", () => {
  // Arrange
  const git: GitCommand = () => { throw new Error("no Git inspection expected"); };
  // Act
  const plan = planMainPush("origin", update(LOCAL, LOCAL), git);
  // Assert
  expect(plan.mode).toBe("none");
});

test("a verifier failure blocks the push", () => {
  // Arrange
  const git = fakeGit();
  // Act / Assert
  expect(() => executeMainPush("origin", update(), git, () => { throw new Error("full verifier failed"); })).toThrow("full verifier failed");
});

test("verification refuses a changed HEAD or worktree afterwards", () => {
  // Arrange
  const state: FakeState = {};
  const git = fakeGit(state);
  // Act / Assert
  expect(() => executeMainPush("origin", update(), git, () => { state.maybeHead = REMOTE; })).toThrow("changed during verification");
  state.maybeHead = LOCAL;
  expect(() => executeMainPush("origin", update(), git, () => { state.maybeStatus = Buffer.from(" M docs/guide.md\0"); })).toThrow("changed during verification");
});

test("verification refuses a branch switch before Git publishes main", () => {
  // Arrange
  const state: FakeState = {};
  const git = fakeGit(state);
  // Act / Assert
  expect(() => executeMainPush("origin", update(), git, () => { state.maybeBranch = "dev"; }))
    .toThrow("main branch, HEAD or worktree changed during verification");
});

test("verification selects the full command for a code change", () => {
  // Arrange
  const git = fakeGit({ maybeChanged: Buffer.from("M\0packages/node.rs\0") });
  let mode = "";
  // Act
  executeMainPush("origin", update(), git, (selected) => { mode = selected; });
  // Assert
  expect(mode).toBe("full");
});

test("a transport retry reuses only the recent exact verified push", () => {
  // Arrange
  const plan = planMainPush("origin", update(), fakeGit({ maybeChanged: Buffer.from("M\0packages/node.rs\0") }));
  const receipt = verifiedReceipt(plan, update(), 1_000);
  // Act / Assert
  expect(receiptMatches(receipt, plan, update(), 2_000)).toBe(true);
  expect(receiptMatches(receipt, plan, update(LOCAL, "c".repeat(40)), 2_000)).toBe(false);
  expect(receiptMatches(receipt, { ...plan, localSha: "d".repeat(40) }, update(), 2_000)).toBe(false);
  expect(receiptMatches(receipt, { ...plan, changedPaths: ["docs/guide.md"] }, update(), 2_000)).toBe(false);
  expect(receiptMatches(receipt, plan, update(), 1_000 + 60 * 60 * 1_000)).toBe(false);
  expect(receiptMatches("not JSON", plan, update(), 2_000)).toBe(false);
});

test("pre-commit and pre-push hooks route to the intended commands", () => {
  // Arrange
  const root = join(import.meta.dir, "..");
  const preCommit = readFileSync(join(root, ".githooks/pre-commit"), "utf8");
  const prePush = readFileSync(join(root, ".githooks/pre-push"), "utf8");
  // Act / Assert
  expect(preCommit).toContain("OPEN_BITCOIN_LOC_REPORT_SOURCE=index bash scripts/verify.sh --quick");
  expect(prePush).toContain('bun run scripts/check-push-verification.ts "$@"');
});

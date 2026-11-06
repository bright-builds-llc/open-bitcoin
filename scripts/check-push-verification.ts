#!/usr/bin/env bun

import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

type GitResult = { status: number; stdout: Buffer; stderr: string };
export type GitCommand = (args: string[]) => GitResult;
export type VerificationMode = "none" | "quick" | "full";
export type PushPlan = { mode: VerificationMode; localSha: string; changedPaths: string[] };
type VerifiedReceipt = { version: 1; expiresAt: number; updatesText: string; plan: PushPlan };
type PathChanges = { paths: string[]; requiresFull: boolean };

type RefUpdate = {
  localRef: string;
  localSha: string;
  remoteRef: string;
  remoteSha: string;
};

const SHA = /^(?:[0-9a-f]{40}|[0-9a-f]{64})$/;
const MAIN_REF = "refs/heads/main";

function text(result: GitResult, command: string): string {
  if (result.status !== 0) throw new Error(`${command} failed: ${result.stderr.trim()}`);
  return result.stdout.toString("utf8").trim();
}

export function parseRefUpdates(source: string): RefUpdate[] {
  if (!source.trim()) return [];
  return source.trimEnd().split(/\r?\n/).map((line) => {
    const fields = line.split(" ");
    if (fields.length !== 4 || !SHA.test(fields[1]) || !SHA.test(fields[3])) {
      throw new Error("malformed Git pre-push ref update");
    }
    return { localRef: fields[0], localSha: fields[1], remoteRef: fields[2], remoteSha: fields[3] };
  });
}

function parseNameStatusChanges(output: Buffer): PathChanges {
  if (output.length === 0) return { paths: [], requiresFull: false };
  const fields = output.toString("utf8").split("\0");
  if (fields.pop() !== "") throw new Error("Git name-status output is not NUL-terminated");
  const paths: string[] = [];
  let requiresFull = false;
  for (let index = 0; index < fields.length;) {
    const status = fields[index++];
    const count = /^(?:R|C)[0-9]{1,3}$/.test(status) ? 2 : /^[A-Z]$/.test(status) ? 1 : 0;
    if (count === 0 || index + count > fields.length) throw new Error(`unexpected Git change status ${status}`);
    if (!["A", "M"].includes(status)) requiresFull = true;
    for (let offset = 0; offset < count; offset++) {
      const relativePath = fields[index++];
      if (!relativePath) throw new Error("Git change has an empty path");
      paths.push(relativePath);
    }
  }
  return { paths, requiresFull };
}

export function parseNameStatus(output: Buffer): string[] {
  return parseNameStatusChanges(output).paths;
}

export function changedPathsAcrossCommits(git: GitCommand, remoteSha: string, localSha: string): PathChanges {
  const commits = text(git(["rev-list", "--reverse", `${remoteSha}..${localSha}`]), "git rev-list").split("\n").filter(Boolean);
  if (commits.length === 0) throw new Error("no outgoing commits found for changed main ref");
  const paths = new Set<string>();
  let requiresFull = false;
  for (const commit of commits) {
    if (!SHA.test(commit)) throw new Error("git rev-list returned an invalid commit ID");
    const result = git(["diff-tree", "--root", "--no-commit-id", "--name-status", "-z", "-r", "-m", "-M", commit]);
    if (result.status !== 0) throw new Error(`git diff-tree failed: ${result.stderr.trim()}`);
    const changed = parseNameStatusChanges(result.stdout);
    for (const relativePath of changed.paths) paths.add(relativePath);
    requiresFull ||= changed.requiresFull;
    const summary = git(["diff-tree", "--root", "--no-commit-id", "--summary", "-r", "-m", commit]);
    if (summary.status !== 0) throw new Error(`git diff-tree summary failed: ${summary.stderr.trim()}`);
    if (/\bmode change [0-9]+ => [0-9]+\b/.test(summary.stdout.toString("utf8"))) requiresFull = true;
  }
  return { paths: [...paths].sort(), requiresFull };
}

export function isSafeProsePath(relativePath: string): boolean {
  if (/^\.codex\/tasks\/.+\.md$/.test(relativePath)) return true;
  return /^docs\/(?!parity\/).+\.md$/.test(relativePath);
}

function isRegularCommittedFile(git: GitCommand, localSha: string, relativePath: string): boolean {
  const result = git(["ls-tree", "-z", localSha, "--", relativePath]);
  if (result.status !== 0) throw new Error(`git ls-tree failed: ${result.stderr.trim()}`);
  const entry = result.stdout.toString("utf8");
  const maybeTab = entry.indexOf("\t");
  if (maybeTab < 0 || !entry.endsWith("\0") || entry.slice(maybeTab + 1, -1) !== relativePath) return false;
  const [mode, objectType, objectSha] = entry.slice(0, maybeTab).split(" ");
  return mode === "100644" && objectType === "blob" && SHA.test(objectSha);
}

export function planMainPush(remoteName: string, updatesText: string, git: GitCommand): PushPlan {
  const updates = parseRefUpdates(updatesText);
  if (updates.length === 0) return { mode: "none", localSha: "", changedPaths: [] };
  if (remoteName !== "origin") throw new Error("direct-main workflow requires the origin remote");
  if (updates.length !== 1 || updates[0].localRef !== MAIN_REF || updates[0].remoteRef !== MAIN_REF) {
    throw new Error("direct-main workflow supports only a push of main to origin/main");
  }
  const update = updates[0];
  if (/^0+$/.test(update.localSha) || /^0+$/.test(update.remoteSha)) {
    throw new Error("creating or deleting origin/main is outside the direct-main workflow");
  }
  if (update.localSha === update.remoteSha) return { mode: "none", localSha: update.localSha, changedPaths: [] };

  const branch = text(git(["symbolic-ref", "--quiet", "--short", "HEAD"]), "git symbolic-ref");
  const head = text(git(["rev-parse", "HEAD"]), "git rev-parse");
  if (branch !== "main" || head !== update.localSha) throw new Error("push must originate from the current local main HEAD");
  const status = git(["status", "--porcelain=v1", "-z", "--untracked-files=all"]);
  if (status.status !== 0) throw new Error(`git status failed: ${status.stderr.trim()}`);
  if (status.stdout.length !== 0) throw new Error("commit or remove worktree changes before pushing main");

  const ancestry = git(["merge-base", "--is-ancestor", update.remoteSha, update.localSha]);
  if (ancestry.status === 1) throw new Error("origin/main is not an ancestor of local main; sync before pushing");
  if (ancestry.status !== 0) throw new Error(`git merge-base failed: ${ancestry.stderr.trim()}`);

  const changes = changedPathsAcrossCommits(git, update.remoteSha, update.localSha);
  if (changes.paths.some((relativePath) => relativePath.toLowerCase().startsWith(".planning/"))) {
    throw new Error(".planning/ is frozen historical evidence; move new work to the parity ledger");
  }
  const safeProseOnly = !changes.requiresFull && changes.paths.every((relativePath) =>
    isSafeProsePath(relativePath) && isRegularCommittedFile(git, update.localSha, relativePath));
  return {
    mode: safeProseOnly ? "quick" : "full",
    localSha: update.localSha,
    changedPaths: changes.paths,
  };
}

export function executeMainPush(
  remoteName: string,
  updatesText: string,
  git: GitCommand,
  runVerifier: (mode: Exclude<VerificationMode, "none">, plan: PushPlan) => void,
): PushPlan {
  const plan = planMainPush(remoteName, updatesText, git);
  if (plan.mode === "none") return plan;
  runVerifier(plan.mode, plan);
  const branch = text(git(["symbolic-ref", "--quiet", "--short", "HEAD"]), "git symbolic-ref");
  const head = text(git(["rev-parse", "HEAD"]), "git rev-parse");
  const status = git(["status", "--porcelain=v1", "-z", "--untracked-files=all"]);
  if (status.status !== 0) throw new Error(`git status failed after verification: ${status.stderr.trim()}`);
  if (branch !== "main" || head !== plan.localSha || status.stdout.length !== 0) {
    throw new Error("main branch, HEAD or worktree changed during verification; retry from a clean commit");
  }
  return plan;
}

const RECEIPT_LIFETIME_MS = 60 * 60 * 1000;

export function verifiedReceipt(plan: PushPlan, updatesText: string, now = Date.now()): string {
  const receipt: VerifiedReceipt = { version: 1, expiresAt: now + RECEIPT_LIFETIME_MS, updatesText, plan };
  return JSON.stringify(receipt);
}

export function receiptMatches(source: string, plan: PushPlan, updatesText: string, now = Date.now()): boolean {
  try {
    const receipt = JSON.parse(source) as VerifiedReceipt;
    return receipt.version === 1 && receipt.expiresAt > now &&
      receipt.expiresAt <= now + RECEIPT_LIFETIME_MS &&
      receipt.updatesText === updatesText && JSON.stringify(receipt.plan) === JSON.stringify(plan);
  } catch {
    return false;
  }
}

function command(root: string): GitCommand {
  return (args) => {
    const result = spawnSync("git", ["-C", root, ...args], { encoding: "buffer", maxBuffer: 64 * 1024 * 1024 });
    if (result.error) throw result.error;
    return { status: result.status ?? -1, stdout: result.stdout, stderr: result.stderr.toString("utf8") };
  };
}

if (import.meta.main) {
  try {
    const rootResult = spawnSync("git", ["rev-parse", "--show-toplevel"], { encoding: "utf8" });
    if (rootResult.error) throw rootResult.error;
    if (rootResult.status !== 0) throw new Error(rootResult.stderr.trim());
    const root = rootResult.stdout.trim();
    const updatesText = readFileSync(0, "utf8");
    const git = command(root);
    const receiptPath = text(git(["rev-parse", "--git-path", "open-bitcoin/pre-push-receipt.json"]), "git rev-parse");
    let usedReceipt = false;
    const plan = executeMainPush(process.argv[2] ?? "", updatesText, git, (mode, selectedPlan) => {
      try {
        usedReceipt = receiptMatches(readFileSync(receiptPath, "utf8"), selectedPlan, updatesText);
      } catch {
        usedReceipt = false;
      }
      if (usedReceipt) {
        console.error("pre-push: reusing recent verification after interrupted transport");
        return;
      }
      const args = ["scripts/verify.sh", ...(mode === "quick" ? ["--quick"] : [])];
      console.error(`pre-push: running ${mode} verification for origin/main`);
      const result = spawnSync("bash", args, { cwd: root, stdio: "inherit" });
      if (result.error) throw result.error;
      if (result.status !== 0) throw new Error(`${mode} verification failed (${result.status ?? "signal"})`);
    });
    if (plan.mode !== "none" && !usedReceipt) {
      mkdirSync(dirname(receiptPath), { recursive: true });
      const temporaryPath = `${receiptPath}.${process.pid}`;
      writeFileSync(temporaryPath, verifiedReceipt(plan, updatesText), { mode: 0o600 });
      renameSync(temporaryPath, receiptPath);
    }
    console.error(`pre-push: ${plan.mode === "none" ? "no outgoing main commits" : `${plan.mode} verification passed for ${plan.changedPaths.length} changed path(s)`}`);
  } catch (error) {
    console.error(`pre-push: ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  }
}

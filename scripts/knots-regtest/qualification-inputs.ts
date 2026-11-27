import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const KNOTS = "packages/bitcoin-knots";
const INPUT_SCOPE = [
  "packages", ".cargo", "rust-toolchain.toml",
  "scripts/run-knots-regtest-differential.ts", "scripts/knots-regtest", "scripts/check-parity-ledger.ts",
  "scripts/command-timings.ts", "scripts/command-timing-lock.ts", "scripts/process-liveness.ts",
  "scripts/phase167-historical-corpus.ts", "scripts/phase167-regtest-serialization.ts",
  "docs/parity/fixtures/phase167-regtest-expectations.json",
  "scripts/run-knots-mainnet-prefix.ts", "scripts/mainnet-prefix.ts", "scripts/check-mainnet-prefix.ts",
  "docs/parity/fixtures/mainnet-prefix-200.hex", "docs/parity/fixtures/mainnet-block-201.hex",
  "docs/parity/fixtures/mainnet-retarget-201-2016.hex", "docs/parity/fixtures/mainnet-block-2017.hex",
] as const;
const MAX_FILE_BYTES = 128 * 1024 * 1024;

export const QUALIFICATION_INPUT_FORMAT = "open-bitcoin-regtest-qualification-inputs-v1";
export type QualificationBinding = {
  format: typeof QUALIFICATION_INPUT_FORMAT;
  source_commit: string;
  sha256: string;
};

type InputRow = readonly [file: string, mode: string, digest: string];

function git(root: string, args: readonly string[]): string {
  return execFileSync("git", ["-C", root, ...args], { encoding: "utf8", timeout: 30_000, maxBuffer: 8 * 1024 * 1024 });
}

const sha256 = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex");

export function digestQualificationInputs(entries: readonly InputRow[]): string {
  return sha256(JSON.stringify([QUALIFICATION_INPUT_FORMAT, ...entries]));
}

function knotsEntry(root: string): InputRow {
  const mode = /^160000 ([0-9a-f]{40}) 0\tpackages\/bitcoin-knots\s*$/.exec(git(root, ["ls-files", "--stage", "--", KNOTS]));
  if (!mode || git(resolve(root, KNOTS), ["rev-parse", "HEAD"]).trim() !== mode[1]) {
    throw new Error("Qualification requires the materialized Knots gitlink");
  }
  // The oracle builder exports this Git revision instead of copying the submodule worktree.
  return [KNOTS, "160000", mode[1]];
}

/** Read the applicable checkout bytes, including additions and staged or unstaged edits. */
export function currentQualificationInputs(root: string): string {
  const files = git(root, ["ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", ...INPUT_SCOPE]);
  const paths = [...new Set(files.split("\0").filter(Boolean))].sort();
  const entries = paths.map((file): InputRow => {
    if (file === KNOTS) return knotsEntry(root);
    const fullPath = resolve(root, file);
    const metadata = lstatSync(fullPath);
    if (!metadata.isFile() || metadata.size > MAX_FILE_BYTES) {
      throw new Error(`Expected a bounded regular qualification input: ${file}`);
    }
    const bytes = readFileSync(fullPath);
    if (bytes.length !== metadata.size) throw new Error(`Qualification input changed while reading: ${file}`);
    return [file, metadata.mode & 0o111 ? "100755" : "100644", sha256(bytes)];
  });
  return digestQualificationInputs(entries);
}

function requireCommittedInputs(root: string): string {
  const status = git(root, ["status", "--porcelain", "--untracked-files=all", "--ignore-submodules=dirty", "--", ...INPUT_SCOPE]);
  if (status.trim()) throw new Error("Commit the qualification inputs before running the actual daemons");
  return git(root, ["rev-parse", "HEAD"]).trim();
}

/** Capture before building either daemon and seal only receipts from the same inputs and build revision. */
export function beginQualification(root: string) {
  const sourceCommit = requireCommittedInputs(root);
  const binding = {
    format: QUALIFICATION_INPUT_FORMAT,
    source_commit: sourceCommit,
    sha256: currentQualificationInputs(root),
  } as const;
  return <T extends { open_bitcoin_build: { source_commit: string } }>(receipt: T) => {
    if (receipt.open_bitcoin_build.source_commit !== sourceCommit
      || requireCommittedInputs(root) !== sourceCommit
      || currentQualificationInputs(root) !== binding.sha256) {
      throw new Error("Qualification inputs changed between launch and receipt publication");
    }
    return { ...receipt, qualification_inputs: binding };
  };
}

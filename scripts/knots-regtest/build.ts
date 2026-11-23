import { createHash } from "node:crypto";
import { closeSync, createReadStream, mkdirSync, openSync, readFileSync } from "node:fs";
import { availableParallelism } from "node:os";
import { join } from "node:path";
import { BASELINE_COMMIT } from "../check-parity-ledger.ts";
import { resolveTarget } from "../command-timings.ts";

const KNOTS_PATH = "packages/bitcoin-knots";
const KNOTS_OPTIONS = [
  "-DCMAKE_BUILD_TYPE=Release",
  "-DBUILD_DAEMON=ON",
  "-DBUILD_GUI=OFF",
  "-DBUILD_CLI=OFF",
  "-DBUILD_TESTS=OFF",
  "-DENABLE_WALLET=OFF",
  "-DENABLE_TOR_SUBPROCESS=OFF",
  "-DWITH_CCACHE=OFF",
  "-DWITH_ZMQ=OFF",
];

export async function sha256File(file: string): Promise<string> {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(file)) hash.update(chunk);
  return hash.digest("hex");
}

function capture(cwd: string, command: string[], env = process.env): string {
  const result = Bun.spawnSync(command, { cwd, env, stdout: "pipe", stderr: "pipe" });
  if (!result.success) throw new Error(`${command[0]} ${command[1]} failed: ${result.stderr.toString().trim()}`);
  return result.stdout.toString().trim();
}

async function loggedCommand(cwd: string, command: string[], log: string, env = process.env) {
  const fd = openSync(log, "wx", 0o600);
  try {
    const child = Bun.spawn(command, { cwd, env, stdout: fd, stderr: fd });
    if (await child.exited !== 0) throw new Error(`${command[0]} ${command[1]} failed. See ${log}`);
  } finally {
    closeSync(fd);
  }
}

export async function buildKnots(root: string, run: string, label: string) {
  const submodule = join(root, KNOTS_PATH);
  const gitlink = capture(root, ["git", "rev-parse", `HEAD:${KNOTS_PATH}`]);
  const checkout = capture(submodule, ["git", "rev-parse", "HEAD"]);
  if (gitlink !== BASELINE_COMMIT || checkout !== BASELINE_COMMIT) {
    throw new Error(`Materialize the pinned Knots checkout at ${BASELINE_COMMIT} before qualifying it`);
  }
  const tree = capture(submodule, ["git", "rev-parse", `${BASELINE_COMMIT}^{tree}`]);
  const archive = join(run, "knots-source.tar");
  const source = join(run, "knots-src");
  const build = join(run, "knots-build");
  mkdirSync(source);
  // Export the pinned commit, so ignored files and worktree edits cannot become build inputs.
  capture(submodule, ["git", "archive", "--format=tar", `--output=${archive}`, BASELINE_COMMIT]);
  const archiveSha256 = await sha256File(archive);
  capture(source, ["tar", "-xf", archive]);

  const env = {
    PATH: process.env.PATH,
    HOME: run,
    LC_ALL: "C",
    BITCOIN_GENBUILD_NO_GIT: "1",
  };
  const configureLog = join(run, "knots-configure.log");
  const buildLog = join(run, "knots-build.log");
  const jobs = Math.min(availableParallelism(), 12);
  console.log(`${label}: configuring pinned Knots source`);
  await loggedCommand(root, ["cmake", "-S", source, "-B", build, "-G", "Ninja", ...KNOTS_OPTIONS], configureLog, env);
  console.log(`${label}: building bitcoind (see ${buildLog})`);
  await loggedCommand(root, ["cmake", "--build", build, "--target", "bitcoind", "--parallel", String(jobs)], buildLog, env);

  const executable = join(build, "bin", "bitcoind");
  const cache = join(build, "CMakeCache.txt");
  const compiler = /^CMAKE_CXX_COMPILER:FILEPATH=(.+)$/m.exec(readFileSync(cache, "utf8"))?.[1];
  if (!compiler) throw new Error(`CMake did not identify the C++ compiler in ${cache}`);
  const version = capture(run, [executable, "--version"], env).split("\n")[0];
  if (!/^Bitcoin Knots daemon version v29\.3(?:\.0)?\.knots20260210$/.test(version)) throw new Error(`Unexpected Knots version: ${version}`);
  const configureOutput = readFileSync(configureLog, "utf8");
  return {
    executable,
    sourceTree: tree,
    binarySha256: await sha256File(executable),
    build: {
      archive_sha256: archiveSha256,
      version,
      platform: capture(run, ["uname", "-sm"], env),
      cmake: capture(run, ["cmake", "--version"], env).split("\n")[0],
      ninja: capture(run, ["ninja", "--version"], env),
      cxx: capture(run, [compiler, "--version"], env).split("\n")[0],
      ...(process.platform === "darwin" ? { macos_sdk: capture(run, ["xcrun", "--show-sdk-version"], env) } : {}),
      dependencies: configureOutput.split("\n")
        .filter((line) => /^-- Found (Boost|[Ll]ibevent):/.test(line))
        .map((line) => line.replace(/(?:\/[^\s"']+)+/g, "<path>")),
      cmake_cache_sha256: await sha256File(cache),
      configure_log_sha256: await sha256File(configureLog),
      build_log_sha256: await sha256File(buildLog),
      git_build_info: "archive",
      configure_options: ["-G", "Ninja", ...KNOTS_OPTIONS],
      build_options: ["--target", "bitcoind", "--parallel", String(jobs)],
    },
  };
}

function productSource(root: string): string {
  const changes = capture(root, ["git", "status", "--porcelain", "--untracked-files=all", "--", "packages", "rust-toolchain.toml"]);
  if (changes) throw new Error("Open Bitcoin runtime source and toolchain must match the checked-out commit");
  return capture(root, ["git", "rev-parse", "HEAD"]);
}

export async function buildOpenBitcoin(root: string, run: string, label: string) {
  const commit = productSource(root);
  const targetDir = resolveTarget(root).directory;
  const options = ["--locked", "--manifest-path", "packages/Cargo.toml", "-p", "open-bitcoin-rpc", "--bin", "open-bitcoind"];
  const log = join(run, "open-bitcoin-build.log");
  const executable = join(run, "open-bitcoind");
  const builtExecutable = join(targetDir, "debug", "open-bitcoind");
  // Snapshot the binary before the timing wrapper releases the shared target lock.
  console.log(`${label}: building ordinary open-bitcoind (see ${log})`);
  await loggedCommand(
    root,
    ["bun", "run", "scripts/command-timings.ts", "run", "--key", "knots-regtest-build-open-bitcoind", "--",
      "bash", "-c", 'source_bin="$1"; dest_bin="$2"; shift 2; cargo build "$@" && cp "$source_bin" "$dest_bin"',
      "knots-regtest-copy-product", builtExecutable, executable, ...options],
    log,
    process.env.CARGO_TARGET_DIR ? { ...process.env, CARGO_TARGET_DIR: targetDir } : process.env,
  );
  if (productSource(root) !== commit) throw new Error("Open Bitcoin source changed during the build");

  return {
    executable,
    build: {
      source_commit: commit,
      binary_sha256: await sha256File(executable),
      rustc: capture(root, ["rustc", "--version"]),
      cargo: capture(root, ["cargo", "--version"]),
      cargo_options: options,
      build_log_sha256: await sha256File(log),
    },
  };
}

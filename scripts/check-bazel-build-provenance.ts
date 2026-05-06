#!/usr/bin/env bun

import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";

type AvailableString = {
  state: "available";
  value: string;
};

type UnavailableValue = {
  reason: string;
};

type UnavailableString = {
  state: "unavailable";
  value: UnavailableValue;
};

type StringAvailability = AvailableString | UnavailableString;

type StatusSnapshot = {
  build: {
    version: string;
    commit: StringAvailability;
    build_time: StringAvailability;
    target: StringAvailability;
    profile: StringAvailability;
  };
};

const ISO_UTC_TIMESTAMP_PATTERN = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/;

function repoRoot(): string {
  return execFileSync("git", ["rev-parse", "--show-toplevel"], {
    cwd: process.cwd(),
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  }).trim();
}

function workspaceVersion(repoRootPath: string): string {
  const cargoToml = readFileSync(
    path.join(repoRootPath, "packages", "Cargo.toml"),
    "utf8",
  );
  const match = cargoToml.match(/^\[workspace\.package\][\s\S]*?^version = "([^"]+)"/m);
  if (!match) {
    throw new Error("could not read workspace package version from packages/Cargo.toml");
  }

  return match[1];
}

function gitHeadCommit(repoRootPath: string): string {
  return execFileSync("git", ["rev-parse", "HEAD"], {
    cwd: repoRootPath,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  }).trim();
}

function bazelMakeEnv(repoRootPath: string): Map<string, string> {
  const output = execFileSync("bazel", ["info", "--show_make_env"], {
    cwd: repoRootPath,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  });
  const values = new Map<string, string>();

  for (const line of output.split("\n")) {
    const match = line.match(/^([A-Z0-9_]+):\s*(.*)$/);
    if (!match) {
      continue;
    }

    values.set(match[1], match[2]);
  }

  return values;
}

function assertAvailableString(
  field: StringAvailability,
  label: string,
): string {
  if (
    field.state !== "available" ||
    typeof field.value !== "string" ||
    field.value.trim() === ""
  ) {
    throw new Error(
      `${label} must be available, got ${JSON.stringify(field, null, 2)}`,
    );
  }

  return field.value;
}

function collectBazelStatus(repoRootPath: string): StatusSnapshot {
  const tempDir = mkdtempSync(
    path.join(os.tmpdir(), "open-bitcoin-bazel-provenance-"),
  );

  try {
    const output = execFileSync(
      "bazel",
      [
        "run",
        "//packages/open-bitcoin-cli:open_bitcoin",
        "--",
        "--network",
        "regtest",
        "--datadir",
        tempDir,
        "status",
        "--format",
        "json",
      ],
      {
        cwd: repoRootPath,
        encoding: "utf8",
        maxBuffer: 128 * 1024 * 1024,
      },
    );

    return JSON.parse(output) as StatusSnapshot;
  } finally {
    rmSync(tempDir, { force: true, recursive: true });
  }
}

function main(): void {
  const repoRootPath = repoRoot();
  const expectedVersion = workspaceVersion(repoRootPath);
  const expectedCommit = gitHeadCommit(repoRootPath);
  const makeEnv = bazelMakeEnv(repoRootPath);
  const expectedTarget = makeEnv.get("TARGET_CPU");
  const expectedProfile = makeEnv.get("COMPILATION_MODE");

  if (!expectedTarget || !expectedProfile) {
    throw new Error("missing TARGET_CPU or COMPILATION_MODE from `bazel info --show_make_env`");
  }

  const snapshot = collectBazelStatus(repoRootPath);
  const build = snapshot.build;

  if (build.version !== expectedVersion) {
    throw new Error(
      `expected build.version ${expectedVersion}, got ${build.version}`,
    );
  }

  const actualCommit = assertAvailableString(build.commit, "build.commit");
  const actualBuildTime = assertAvailableString(
    build.build_time,
    "build.build_time",
  );
  const actualTarget = assertAvailableString(build.target, "build.target");
  const actualProfile = assertAvailableString(build.profile, "build.profile");

  if (actualCommit !== expectedCommit) {
    throw new Error(`expected build.commit ${expectedCommit}, got ${actualCommit}`);
  }
  if (actualTarget !== expectedTarget) {
    throw new Error(`expected build.target ${expectedTarget}, got ${actualTarget}`);
  }
  if (actualProfile !== expectedProfile) {
    throw new Error(
      `expected build.profile ${expectedProfile}, got ${actualProfile}`,
    );
  }
  if (actualBuildTime.trim() === "") {
    throw new Error("build.build_time must be a non-empty string");
  }
  if (!ISO_UTC_TIMESTAMP_PATTERN.test(actualBuildTime)) {
    throw new Error(
      `build.build_time must be ISO-8601 UTC YYYY-MM-DDTHH:MM:SSZ, got ${actualBuildTime}`,
    );
  }

  console.log("Bazel build provenance check passed.");
}

main();

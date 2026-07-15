#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdtemp, mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import {
  acquireTargetLock,
  executeTimedCommand,
  normalizeCommandKey,
  percentile,
  readTimingRecords,
  renderTimingReport,
  softLimitMilliseconds,
  summarizeRecords,
  type TimingRecord,
  writeTimingRecord,
} from "./command-timings";

const tempRoots: string[] = [];

afterEach(async () => {
  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot !== undefined) {
      await rm(maybeRoot, { force: true, recursive: true });
    }
  }
});

async function tempRoot(): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-timings-"));
  tempRoots.push(root);
  return root;
}

function timingRecord(overrides: Partial<TimingRecord> = {}): TimingRecord {
  return {
    schemaVersion: 1,
    runId: crypto.randomUUID(),
    key: "cargo-test-workspace",
    source: "test",
    startedAt: "2026-07-14T12:00:00.000Z",
    endedAt: "2026-07-14T12:00:01.000Z",
    durationMs: 1_000,
    outcome: "success",
    exitStatus: 0,
    signal: null,
    pid: process.pid,
    git: { commit: "abc123", dirty: false },
    platform: { os: "darwin", arch: "arm64" },
    rustVersion: "rustc 1.94.1",
    verifyMode: null,
    target: { kind: "default" },
    ...overrides,
  };
}

test("normalizes_command_keys_without_preserving_shell_text", () => {
  // Arrange
  const input = " Cargo Test: Workspace / All Features ";

  // Act
  const key = normalizeCommandKey(input);

  // Assert
  expect(key).toBe("cargo-test-workspace-all-features");
});

test("calculates_nearest_rank_percentiles", () => {
  // Arrange
  const values = [10, 20, 30, 40, 50];

  // Act
  const median = percentile(values, 0.5);
  const p90 = percentile(values, 0.9);

  // Assert
  expect(median).toBe(30);
  expect(p90).toBe(50);
});

test("writes_records_atomically_and_retains_only_the_requested_count", async () => {
  // Arrange
  const root = await tempRoot();

  // Act
  for (let index = 0; index < 3; index += 1) {
    await writeTimingRecord(
      root,
      timingRecord({
        runId: `run-${index}`,
        startedAt: `2026-07-14T12:00:0${index}.000Z`,
      }),
      2,
    );
  }
  const directory = path.join(root, "command-timings", "cargo-test-workspace");
  const files = await readdir(directory);

  // Assert
  expect(files).toHaveLength(2);
  expect(files.every((file) => file.endsWith(".json"))).toBe(true);
  expect(files.some((file) => file.includes("run-0"))).toBe(false);
});

test("ignores_malformed_local_records", async () => {
  // Arrange
  const root = await tempRoot();
  const directory = path.join(root, "command-timings", "cargo-test-workspace");
  await mkdir(directory, { recursive: true });
  await writeFile(path.join(directory, "malformed.json"), "not-json\n");
  await writeTimingRecord(root, timingRecord());

  // Act
  const records = await readTimingRecords(root);

  // Assert
  expect(records).toHaveLength(1);
  expect(records[0]?.outcome).toBe("success");
});

test("summarizes_success_failure_and_interruption_history", () => {
  // Arrange
  const records = [
    timingRecord({ durationMs: 1_000 }),
    timingRecord({ runId: "second", durationMs: 3_000 }),
    timingRecord({ runId: "failure", outcome: "failure", exitStatus: 1 }),
    timingRecord({
      runId: "interrupt",
      outcome: "interrupted",
      exitStatus: 143,
      signal: "SIGTERM",
    }),
  ];

  // Act
  const [summary] = summarizeRecords(records);

  // Assert
  expect(summary?.sampleCount).toBe(2);
  expect(summary?.medianMs).toBe(1_000);
  expect(summary?.maximumMs).toBe(3_000);
  expect(summary?.failures).toBe(1);
  expect(summary?.interruptions).toBe(1);
});

test("reports_empty_history_clearly", () => {
  // Arrange
  const summaries = summarizeRecords([]);

  // Act
  const report = renderTimingReport(summaries);

  // Assert
  expect(report).toBe("No local command timing history yet.");
});

test("counts_only_live_running_records_as_current", () => {
  // Arrange
  const records = [
    timingRecord({
      runId: "live",
      outcome: "running",
      endedAt: null,
      durationMs: null,
      exitStatus: null,
      pid: process.pid,
    }),
    timingRecord({
      runId: "dead",
      outcome: "running",
      endedAt: null,
      durationMs: null,
      exitStatus: null,
      pid: 2_147_483_647,
    }),
  ];

  // Act
  const [summary] = summarizeRecords(records);

  // Assert
  expect(summary?.currentRuns).toBe(1);
});

test("uses_a_sixty_minute_initial_threshold_for_profile_verification", () => {
  // Arrange
  const command = ["bash", "scripts/verify.sh", "--profile"];

  // Act
  const threshold = softLimitMilliseconds([], "verify-profile", command, "default");

  // Assert
  expect(threshold).toBe(60 * 60_000);
});

test("uses_twice_p90_after_five_comparable_successes", () => {
  // Arrange
  const records = Array.from({ length: 5 }, (_, index) =>
    timingRecord({
      runId: `history-${index}`,
      durationMs: (16 + index) * 60_000,
      target: { kind: "default" },
    }),
  );
  records.push(
    timingRecord({
      runId: "isolated-outlier",
      durationMs: 200 * 60_000,
      target: { kind: "isolated" },
    }),
  );

  // Act
  const threshold = softLimitMilliseconds(
    records,
    "cargo-test-workspace",
    ["cargo", "test", "--workspace"],
    "default",
  );

  // Assert
  expect(threshold).toBe(40 * 60_000);
});

test("records_a_successful_child_status", async () => {
  // Arrange
  const root = await tempRoot();

  // Act
  const record = await executeTimedCommand(["bash", "-c", "exit 0"], {
    key: "fixture-success",
    stateRoot: root,
    heartbeatMs: 10_000,
  });

  // Assert
  expect(record.outcome).toBe("success");
  expect(record.exitStatus).toBe(0);
});

test("records_a_failed_child_status_without_rewriting_it", async () => {
  // Arrange
  const root = await tempRoot();

  // Act
  const record = await executeTimedCommand(["bash", "-c", "exit 7"], {
    key: "fixture-failure",
    stateRoot: root,
    heartbeatMs: 10_000,
  });

  // Assert
  expect(record.outcome).toBe("failure");
  expect(record.exitStatus).toBe(7);
});

test("local_storage_failure_does_not_mask_the_child_status", async () => {
  // Arrange
  const root = await tempRoot();
  const stateRoot = path.join(root, "state-is-a-file");
  await writeFile(stateRoot, "not a directory\n");
  let warnings = "";

  // Act
  const record = await executeTimedCommand(["bash", "-c", "exit 7"], {
    key: "fixture-storage-failure",
    stateRoot,
    heartbeatMs: 10_000,
    stderr: {
      write(value) {
        warnings += String(value);
        return true;
      },
    },
  });

  // Assert
  expect(record.exitStatus).toBe(7);
  expect(record.outcome).toBe("failure");
  expect(warnings).toContain("continuing without it");
  expect(warnings).toContain("could not persist local timing history");
});

test("records_a_signaled_child_as_interrupted", async () => {
  // Arrange
  const root = await tempRoot();

  // Act
  const record = await executeTimedCommand(["bash", "-c", "kill -TERM $$"], {
    key: "fixture-interrupted",
    stateRoot: root,
    heartbeatMs: 10_000,
  });

  // Assert
  expect(record.outcome).toBe("interrupted");
  expect(record.signal).toBe("SIGTERM");
});

test("does_not_persist_raw_child_arguments", async () => {
  // Arrange
  const root = await tempRoot();
  const secret = "never-persist-this-token";

  // Act
  await executeTimedCommand(["bash", "-c", "exit 0", secret], {
    key: "redacted-command",
    stateRoot: root,
    heartbeatMs: 10_000,
  });
  const records = await readTimingRecords(root);

  // Assert
  expect(JSON.stringify(records)).not.toContain(secret);
});

test("waits_for_the_active_target_owner_before_running_the_next_command", async () => {
  // Arrange
  const root = await tempRoot();
  const target = path.join(root, "target");
  const silent = { write: () => true };
  const first = await acquireTargetLock({
    key: "first",
    targetDirectory: target,
    stateRoot: root,
    heartbeatMs: 10,
    stderr: silent,
  });
  let secondAcquired = false;

  // Act
  const maybeSecond = acquireTargetLock({
    key: "second",
    targetDirectory: target,
    stateRoot: root,
    heartbeatMs: 10,
    stderr: silent,
  }).then((lock) => {
    secondAcquired = true;
    return lock;
  });
  await Bun.sleep(30);
  const acquiredWhileLocked = secondAcquired;
  await first.release();
  const second = await maybeSecond;
  await second.release();

  // Assert
  expect(acquiredWhileLocked).toBe(false);
  expect(secondAcquired).toBe(true);
});

test("marks_a_dead_owner_lock_abandoned_before_recovery", async () => {
  // Arrange
  const root = await tempRoot();
  const target = path.join(root, "target");
  const initial = await acquireTargetLock({
    key: "initial",
    targetDirectory: target,
    stateRoot: root,
  });
  await initial.release();
  await mkdir(initial.lockPath, { recursive: true });
  await writeFile(
    path.join(initial.lockPath, "owner.json"),
    `${JSON.stringify({
      pid: 2_147_483_647,
      runId: "dead-owner",
      key: "dead",
      startedAt: new Date().toISOString(),
    })}\n`,
  );

  // Act
  const recovered = await acquireTargetLock({
    key: "recovered",
    targetDirectory: target,
    stateRoot: root,
  });
  const abandoned = await readdir(path.join(root, "locks", "abandoned"));
  await recovered.release();

  // Assert
  expect(abandoned).toHaveLength(1);
});

test("does_not_abandon_a_just_created_ownerless_lock", async () => {
  // Arrange
  const root = await tempRoot();
  const target = path.join(root, "target");
  const initial = await acquireTargetLock({
    key: "initial",
    targetDirectory: target,
    stateRoot: root,
  });
  await initial.release();
  await mkdir(initial.lockPath, { recursive: true });
  let acquired = false;

  // Act
  const maybeRecovered = acquireTargetLock({
    key: "wait-for-owner-write",
    targetDirectory: target,
    stateRoot: root,
    heartbeatMs: 10,
    stderr: { write: () => true },
  }).then((lock) => {
    acquired = true;
    return lock;
  });
  await Bun.sleep(50);
  const acquiredDuringGrace = acquired;
  await rm(initial.lockPath, { force: true, recursive: true });
  const recovered = await maybeRecovered;
  await recovered.release();

  // Assert
  expect(acquiredDuringGrace).toBe(false);
});

test("wires_local_verifier_history_without_masking_status_and_profiles_ci", async () => {
  // Arrange
  const verifyScript = await readFile(path.join(import.meta.dir, "verify.sh"), "utf8");
  const ciWorkflow = await readFile(
    path.join(import.meta.dir, "..", ".github", "workflows", "ci.yml"),
    "utf8",
  );

  // Act
  const anchors = [
    '"verify-${verify_invocation}"',
    "record-batch",
    'if [[ -z "${CI:-}"',
    'if ! bun run scripts/command-timings.ts record-batch',
    "warning: failed to persist local verifier timing history",
    "bun test scripts/command-timings.test.ts",
    "bun test scripts/diagnose-rust-test-stall.test.ts",
  ];

  // Assert
  for (const anchor of anchors) {
    expect(verifyScript).toContain(anchor);
  }
  expect(ciWorkflow).toContain("bash scripts/verify.sh --profile");
});

#!/usr/bin/env bun

import { expect, test } from "bun:test";
import { EventEmitter } from "node:events";
import path from "node:path";
import {
  classifyDiagnosticAttempts,
  type HarnessSpawner,
  installChildSignalForwarding,
  isDirectScriptInvocation,
  observeHarnessStartup,
  parseAvailableBytes,
  parseDiagnosticOptions,
  selectExecutableFromCargoJson,
} from "./diagnose-rust-test-stall";
import { filterProcessEvidence } from "./process-liveness";

function delayedHarness(delayMs: number, exitStatus = 0): {
  spawn: HarnessSpawner;
  killedSignals: NodeJS.Signals[];
} {
  const killedSignals: NodeJS.Signals[] = [];
  let resolveExit: (status: number) => void = () => undefined;
  const exited = new Promise<number>((resolve) => {
    resolveExit = resolve;
  });
  const stdout = new ReadableStream<Uint8Array>({
    start(controller) {
      setTimeout(() => {
        controller.enqueue(new TextEncoder().encode("fixture: test\n"));
        controller.close();
        resolveExit(exitStatus);
      }, delayMs);
    },
  });
  const stderr = new ReadableStream<Uint8Array>({
    start(controller) {
      controller.close();
    },
  });
  return {
    killedSignals,
    spawn: () => ({
      pid: 4242,
      exited,
      stdout,
      stderr,
      kill(signal) {
        killedSignals.push(signal);
      },
    }),
  };
}

test("uses_the_cli_library_target_and_five_attempts_by_default", () => {
  // Arrange
  const root = "/tmp/open-bitcoin-fixture";

  // Act
  const options = parseDiagnosticOptions([], root);

  // Assert
  expect(options.packageName).toBe("open-bitcoin-cli");
  expect(options.targetName).toBe("open_bitcoin_cli");
  expect(options.targetKind).toBe("lib");
  expect(options.attempts).toBe(5);
  expect(options.startupThresholdMs).toBe(10_000);
  expect(options.maybeStopAfterMs).toBeNull();
});

test("runs_main_only_when_the_diagnostic_is_the_process_entrypoint", () => {
  const scriptPath = "/repo/scripts/diagnose-rust-test-stall.ts";

  expect(isDirectScriptInvocation(scriptPath, ["bun", scriptPath])).toBe(true);
  expect(isDirectScriptInvocation(scriptPath, ["bun", "test", `${scriptPath}.test.ts`])).toBe(
    false,
  );
});

test("rejects_a_stop_limit_that_precedes_diagnostic_capture", () => {
  // Arrange
  const args = ["--startup-threshold-ms", "100", "--stop-after-ms", "50"];

  // Act
  const parse = () => parseDiagnosticOptions(args);

  // Assert
  expect(parse).toThrow("--stop-after-ms must exceed --startup-threshold-ms");
});

test("selects_the_exact_emitted_test_executable", () => {
  // Arrange
  const output = [
    JSON.stringify({
      reason: "compiler-artifact",
      target: { name: "dependency", kind: ["lib"] },
      executable: "/tmp/dependency",
    }),
    JSON.stringify({
      reason: "compiler-artifact",
      target: { name: "open_bitcoin_cli", kind: ["lib"] },
      executable: "/tmp/open-bitcoin-cli-test",
    }),
  ].join("\n");

  // Act
  const executable = selectExecutableFromCargoJson(output, "open_bitcoin_cli", "lib");

  // Assert
  expect(executable).toBe("/tmp/open-bitcoin-cli-test");
});

test("parses_portable_df_available_space_in_bytes", () => {
  // Arrange
  const output = [
    "Filesystem 1024-blocks Used Available Capacity Mounted on",
    "/dev/disk3s5 100000 40000 60000 40% /System/Volumes/Data",
  ].join("\n");

  // Act
  const availableBytes = parseAvailableBytes(output);

  // Assert
  expect(availableBytes).toBe(60_000 * 1024);
});

test("classifies_five_prompt_harness_launches_as_host_pressure", () => {
  const attempts = Array.from({ length: 5 }, (_, index) => ({
    attempt: index + 1,
    exitStatus: 0,
    startupMs: 250,
    diagnosticDirectory: null,
    stoppedByLimit: false,
  }));

  expect(classifyDiagnosticAttempts(attempts, 10_000)).toBe(
    "disk-loader-concurrency-pressure",
  );
  expect(
    classifyDiagnosticAttempts([{ ...attempts[0]!, startupMs: null }], 10_000),
  ).toBe("pre-harness-stall-reproduced");
  expect(
    classifyDiagnosticAttempts([{ ...attempts[0]!, exitStatus: 1 }], 10_000),
  ).toBe("harness-list-failed");
});

test("captures_delayed_startup_evidence_without_terminating_the_harness", async () => {
  // Arrange
  const root = "/fixture/open-bitcoin";
  const harness = delayedHarness(50);
  let captures = 0;

  // Act
  const result = await observeHarnessStartup(
    "/fixture/test-harness",
    1,
    { repoRoot: root, startupThresholdMs: 10, maybeStopAfterMs: null },
    async () => {
      captures += 1;
      return path.join(root, "captured-evidence");
    },
    harness.spawn,
  );

  // Assert
  expect(captures).toBe(1);
  expect(result.exitStatus).toBe(0);
  expect(result.startupMs).toBeGreaterThanOrEqual(10);
  expect(result.diagnosticDirectory).toBe(path.join(root, "captured-evidence"));
  expect(result.stoppedByLimit).toBe(false);
  expect(harness.killedSignals).toEqual([]);
});

test("evidence_write_failure_does_not_replace_the_harness_status", async () => {
  // Arrange
  const harness = delayedHarness(50);

  // Act
  const result = await observeHarnessStartup(
    "/fixture/test-harness",
    1,
    { repoRoot: "/fixture/open-bitcoin", startupThresholdMs: 10, maybeStopAfterMs: null },
    async () => {
      throw new Error("fixture evidence write failure");
    },
    harness.spawn,
  );

  // Assert
  expect(result.exitStatus).toBe(0);
  expect(result.diagnosticDirectory).toBeNull();
  expect(harness.killedSignals).toEqual([]);
});

test("process_evidence_excludes_unrelated_command_rows", () => {
  // Arrange
  const output = [
    "PID PPID STATE ELAPSED %CPU %MEM COMMAND",
    "100 1 S 00:10 0.0 0.1 /tmp/test-binary --list",
    "101 100 S 00:09 0.0 0.1 helper-child",
    "200 1 S 10:00 0.0 0.1 unrelated --password secret-value",
    "300 1 S 00:05 1.0 0.1 cargo test --workspace",
  ].join("\n");

  // Act
  const evidence = filterProcessEvidence(output, 100);
  const stored = `${evidence.targetTree}${evidence.cargoJobs}`;

  // Assert
  expect(stored).toContain("/tmp/test-binary --list");
  expect(stored).toContain("helper-child");
  expect(stored).toContain("cargo test --workspace");
  expect(stored).not.toContain("secret-value");
});

test("forwards_cancellation_to_the_child_and_removes_signal_handlers", () => {
  // Arrange
  const signalSource = new EventEmitter();
  const forwarded: string[] = [];
  const forwarding = installChildSignalForwarding(
    {
      kill(signal) {
        forwarded.push(signal);
      },
    },
    signalSource,
  );

  // Act
  signalSource.emit("SIGTERM");
  const relayedSignal = forwarding.maybeSignal();
  forwarding.cleanup();
  signalSource.emit("SIGINT");

  // Assert
  expect(relayedSignal).toBe("SIGTERM");
  expect(forwarded).toEqual(["SIGTERM"]);
});

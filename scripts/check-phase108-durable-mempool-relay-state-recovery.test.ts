import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase108DurableMempoolRelayStateRecovery } from "./check-phase108-durable-mempool-relay-state-recovery";

const TARGET_FILES = [
  "README.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json",
  "packages/open-bitcoin-node/src/network/recovery.rs",
  "packages/open-bitcoin-node/src/network/relay_fanout.rs",
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs",
  "packages/open-bitcoin-node/src/network/tests/recovery_cases.rs",
  "packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs",
  "packages/open-bitcoin-node/src/status/relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/context/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/render/relay.rs",
  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_when_phase108_recovery_evidence_is_complete", () => {
  const root = createFixture();

  const failures = checkPhase108DurableMempoolRelayStateRecovery(root);

  expect(failures).toEqual([]);
});

test("fails_when_phase108_surface_is_missing", () => {
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "docs/parity/index.json", "v2-0-durable-mempool-relay-state-recovery");
    },
  });

  const failures = checkPhase108DurableMempoolRelayStateRecovery(root).join("\n");

  expect(failures).toContain("v2-0-durable-mempool-relay-state-recovery");
});

test("fails_when_a_required_requirement_is_missing", () => {
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "docs/parity/index.json", "REL-02");
    },
  });

  const failures = checkPhase108DurableMempoolRelayStateRecovery(root).join("\n");

  expect(failures).toContain("REL-02");
});

test("fails_when_core_recovery_symbol_is_missing", () => {
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "packages/open-bitcoin-node/src/network/recovery.rs", "recover_mempool_snapshot");
    },
  });

  const failures = checkPhase108DurableMempoolRelayStateRecovery(root).join("\n");

  expect(failures).toContain("recover_mempool_snapshot");
});

test("fails_when_verifier_order_omits_phase108_checker", () => {
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "scripts/verify.sh", "bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts");
    },
  });

  const failures = checkPhase108DurableMempoolRelayStateRecovery(root).join("\n");

  expect(failures).toContain("check-phase108-durable-mempool-relay-state-recovery.ts");
});

test("fails_on_positive_public_propagation_claim", () => {
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(files, "docs/operator/runtime-guide.md", "\nPhase 108 implements public propagation.\n");
    },
  });

  const failures = checkPhase108DurableMempoolRelayStateRecovery(root).join("\n");

  expect(failures).toContain("public propagation");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase108-check-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    files.set(file, readFileSync(path.join(process.cwd(), file), "utf8"));
  }
  options.maybeMutateFiles?.(files);
  for (const [file, contents] of files) {
    const target = path.join(root, file);
    mkdirSync(path.dirname(target), { recursive: true });
    writeFileSync(target, contents);
  }
  return root;
}

function removeFromFile(files: Map<TargetFile, string>, file: TargetFile, needle: string): void {
  files.set(file, (files.get(file) ?? "").replaceAll(needle, ""));
}

function appendToFile(files: Map<TargetFile, string>, file: TargetFile, text: string): void {
  files.set(file, `${files.get(file) ?? ""}${text}`);
}

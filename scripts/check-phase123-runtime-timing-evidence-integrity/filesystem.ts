import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { TargetFile } from "./constants.ts";

const SPLIT_CHILDREN = new Map<TargetFile, readonly string[]>([
  [
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    [
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/compact_timeout.rs",
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/idle_sessions.rs",
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/session_boundaries.rs",
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/tcp_read_semantics.rs",
    ],
  ],
  [
    "packages/open-bitcoin-rpc/src/inbound_listener/tests.rs",
    [
      "packages/open-bitcoin-rpc/src/inbound_listener/tests/admission_and_handshake.rs",
      "packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs",
      "packages/open-bitcoin-rpc/src/inbound_listener/tests/envelope_and_resource.rs",
      "packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs",
      "packages/open-bitcoin-rpc/src/inbound_listener/tests/preflight_and_advertisement.rs",
      "packages/open-bitcoin-rpc/src/inbound_listener/tests/reconnect_policy.rs",
    ],
  ],
  [
    "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs",
    [
      "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs",
      "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/inbound_runtime.rs",
    ],
  ],
]);

export function readText(repoRoot: string, file: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, file);
  if (!existsSync(absolutePath)) {
    failures.push(`P123 missing required corpus file: ${file}`);
    return "";
  }
  const texts = [readFileSync(absolutePath, "utf8")];
  for (const child of SPLIT_CHILDREN.get(file) ?? []) {
    const childPath = path.join(repoRoot, child);
    if (existsSync(childPath)) {
      texts.push(readFileSync(childPath, "utf8"));
    }
  }
  return texts.join("\n");
}

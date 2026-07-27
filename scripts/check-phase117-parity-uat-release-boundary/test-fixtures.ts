import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "../check-phase117-parity-uat-release-boundary";

export const SURFACES = {
  "v2-1-block-serving-activation-eligibility-boundary": [
    "BSRV-01",
    "BSRV-02",
    "BSRV-03",
    "BSRV-05",
    "BSRV-06",
  ],
  "v2-1-full-block-serving-request-path": ["BSRV-04", "GOV-01", "GOV-05"],
  "v2-1-bip152-wire-codec-message-semantics": ["CMP-01", "CMP-02", "CMP-03", "RCN-01"],
  "v2-1-compact-relay-negotiation-announcement-policy": ["CMP-04", "CMP-05", "CMP-06"],
  "v2-1-compact-block-reconstruction": ["RCN-02", "RCN-03", "GOV-04"],
  "v2-1-missing-transaction-fallback-validation-handoff": [
    "RCN-04",
    "RCN-05",
    "RCN-06",
    "RCN-07",
    "GOV-02",
    "GOV-03",
  ],
  "v2-1-operator-block-relay-evidence": ["OBS-01", "OBS-02", "OBS-03", "OBS-04", "OBS-05"],
  "v2-1-parity-uat-release-boundary": [
    "BOUND-01",
    "BOUND-02",
    "BOUND-03",
    "BOUND-04",
    "BOUND-05",
  ],
} as const;

export const TARGET_FILES = [
  "README.md",
  ".planning/REQUIREMENTS.md",
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/support-matrix.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase117-parity-uat-release-boundary.ts",
  "scripts/check-phase117-parity-uat-release-boundary.test.ts",
  "scripts/verify.sh",
] as const;

export type TargetFile = (typeof TARGET_FILES)[number];
export type FixtureOptions = {
  completedGapClosure?: boolean;
  gapClosureStage?: boolean;
  newerMilestoneRequirements?: boolean;
  postAuditGapPlanning?: boolean;
  maybeMutate?: (files: Map<TargetFile, string>) => void;
};

export const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

export function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase117-"));
  tempRoots.push(root);
  const commonText = [
    "Open Bitcoin provides bounded, explicit, default-off block serving and compact block relay.",
    "Package relay, BIP37 bloom-filter serving, compact-filter serving, public serving defaults, archive-node behavior, production full-node readiness, production service operation, and production-funds wallet use remain deferred.",
    ...requiredAnchors(),
  ].join("\n");
  const files = new Map<TargetFile, string>(TARGET_FILES.map((file) => [file, commonText]));
  files.set("docs/parity/index.json", JSON.stringify(createParityIndex(), null, 2));
  files.set("docs/parity/source-breadcrumbs.json", createBreadcrumbs());
  const v21Requirements = createRequirements(
    options.gapClosureStage ?? false,
    options.completedGapClosure ?? false,
    options.postAuditGapPlanning ?? false,
  );
  files.set(
    ".planning/REQUIREMENTS.md",
    options.newerMilestoneRequirements
      ? "# Requirements: Open Bitcoin\n\n**Milestone:** v2.2 Package Relay and Long-Lived Mempool Policy"
      : v21Requirements,
  );
  files.set("docs/operator/runtime-guide.md", `${commonText}\n${requiredCommands().join("\n")}`);
  files.set("scripts/verify.sh", createVerifyScript());
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  if (options.newerMilestoneRequirements) {
    const archivedPath = path.join(root, ".planning/milestones/v2.1-REQUIREMENTS.md");
    mkdirSync(path.dirname(archivedPath), { recursive: true });
    writeFileSync(archivedPath, `${v21Requirements}\n`);
  }
  return root;
}

export function createParityIndex(): {
  surfaces: Array<{ name: string; status: string }>;
  checklist: {
    surfaces: Array<{
      id: string;
      requirements: string[];
      status: string;
      upstream: { sources: string[]; tests: string[] };
    }>;
  };
} {
  return {
    surfaces: Object.keys(SURFACES).map((name) => ({ name, status: "done" })),
    checklist: {
      surfaces: Object.entries(SURFACES).map(([id, requirements]) => ({
        id,
        requirements: [...requirements],
        status: "done",
        upstream: {
          sources: requiredAnchors().filter((anchor) => anchor.includes("/src/")),
          tests: requiredAnchors().filter((anchor) => anchor.includes("/test/")),
        },
      })),
    },
  };
}

export function createBreadcrumbs(): string {
  const labels = [
    "network-block-serving-activation-boundary",
    "node-network-block-serving-adapter",
    "codec-bip152-compact-block",
    "network-compact-relay-peer-state",
    "network-compact-block-reconstruction",
    "network-compact-block-download",
    "node-network-block-relay-evidence-adapter",
    "node-status-contract",
    "node-observability-contracts",
    "rpc-surface",
    "cli-operator-onboarding-contracts",
    "cli-operator-dashboard-contracts",
    "cli-operator-support-bundles",
  ];
  return JSON.stringify({
    groups: labels.map((label) => ({ label, files: [`${label}.rs`], breadcrumbs: requiredAnchors() })),
  });
}

export function createRequirements(
  gapClosureStage: boolean,
  completedGapClosure: boolean,
  postAuditGapPlanning: boolean,
): string {
  const gapPhases = new Map([
    ["RCN-04", "125"],
    ["RCN-05", "125"],
    ["RCN-06", "125"],
    ["CMP-05", "126"],
    ["RCN-02", "126"],
    ["RCN-03", "126"],
    ["GOV-04", "126"],
    ["BOUND-01", "126"],
  ]);
  const postAuditGapPhases = new Map([
    ["BSRV-03", "127"],
    ["BSRV-04", "127"],
    ["OBS-02", "127"],
    ["OBS-04", "127"],
    ["CMP-04", "128"],
    ["CMP-05", "128"],
    ["OBS-03", "128"],
    ["OBS-01", "129"],
    ["BOUND-02", "129"],
  ]);
  return Object.entries(requirementPhases())
    .map(([requirement, phase]) => {
      const maybePostAuditGapPhase = postAuditGapPlanning
        ? postAuditGapPhases.get(requirement)
        : undefined;
      const maybeGapPhase =
        maybePostAuditGapPhase ?? (gapClosureStage || postAuditGapPlanning
          ? gapPhases.get(requirement)
          : undefined);
      const gapStatus = completedGapClosure ? "Complete" : "Pending";
      return `| ${requirement} | Phase ${maybeGapPhase ?? phase} | ${maybeGapPhase ? gapStatus : "Complete"} |`;
    })
    .join("\n");
}

export function createVerifyScript(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    "bun test scripts/check-phase116-operator-block-relay-evidence.test.ts",
    "bun run scripts/check-phase116-operator-block-relay-evidence.ts",
    "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
    "bun run scripts/check-phase117-parity-uat-release-boundary.ts",
    "bash scripts/check-pure-core-deps.sh",
    "VERIFY_COMMAND_ORDER",
    'run_step "test Phase 116 operator block-relay evidence checker" bun test scripts/check-phase116-operator-block-relay-evidence.test.ts',
    'run_step "check Phase 116 operator block-relay evidence" bun run scripts/check-phase116-operator-block-relay-evidence.ts',
    'run_step "test Phase 117 parity UAT release boundary checker" bun test scripts/check-phase117-parity-uat-release-boundary.test.ts',
    'run_step "check Phase 117 parity UAT release boundary" bun run scripts/check-phase117-parity-uat-release-boundary.ts',
    'run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh',
  ].join("\n");
}

export function requiredCommands(): string[] {
  return [
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -regtest openbitcoinnetworkstatus",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -regtest openbitcoinnetworkstatus",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-block-relay-support",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-block-relay-support",
    "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts",
    "bun run scripts/check-phase117-parity-uat-release-boundary.ts",
    "bash scripts/verify.sh",
  ];
}

export function requiredAnchors(): string[] {
  return [
    "packages/bitcoin-knots/src/protocol.h",
    "packages/bitcoin-knots/src/blockencodings.h",
    "packages/bitcoin-knots/src/blockencodings.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/src/net_processing.h",
    "packages/bitcoin-knots/src/net.h",
    "packages/bitcoin-knots/src/net_permissions.h",
    "packages/bitcoin-knots/src/validation.cpp",
    "packages/bitcoin-knots/src/node/blockstorage.cpp",
    "packages/bitcoin-knots/test/functional/p2p_getdata.py",
    "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
    "packages/bitcoin-knots/test/functional/p2p_permissions.py",
  ];
}

export function requirementPhases(): Record<string, string> {
  const result: Record<string, string> = {};
  for (const requirements of Object.values(SURFACES)) {
    for (const requirement of requirements) {
      if (requirement === "CMP-05") result[requirement] = "118";
      else if (["RCN-02", "RCN-03", "GOV-04"].includes(requirement)) result[requirement] = "119";
      else if (["RCN-07", "GOV-02", "GOV-03"].includes(requirement)) result[requirement] = "120";
      else if (requirement === "OBS-03") result[requirement] = "121";
      else if (requirement.startsWith("BSRV")) result[requirement] = requirement === "BSRV-04" ? "111" : "110";
      else if (["CMP-01", "CMP-02", "CMP-03", "RCN-01"].includes(requirement)) result[requirement] = "112";
      else if (requirement.startsWith("CMP")) result[requirement] = "113";
      else if (requirement.startsWith("RCN")) result[requirement] = "115";
      else if (requirement.startsWith("GOV")) result[requirement] = "111";
      else if (requirement.startsWith("OBS")) result[requirement] = "116";
      else result[requirement] = "117";
    }
  }
  return result;
}

export function mutateIndex(files: Map<TargetFile, string>, mutate: (index: any) => void): void {
  const index = JSON.parse(files.get("docs/parity/index.json") ?? "{}") as any;
  mutate(index);
  files.set("docs/parity/index.json", JSON.stringify(index, null, 2));
}

export function mutateBreadcrumbs(files: Map<TargetFile, string>, mutate: (breadcrumbs: any) => void): void {
  const breadcrumbs = JSON.parse(files.get("docs/parity/source-breadcrumbs.json") ?? "{}") as any;
  mutate(breadcrumbs);
  files.set("docs/parity/source-breadcrumbs.json", JSON.stringify(breadcrumbs, null, 2));
}

export function replace(files: Map<TargetFile, string>, file: TargetFile, needle: string, value: string): void {
  files.set(file, (files.get(file) ?? "").replace(needle, value));
}

export function append(files: Map<TargetFile, string>, file: TargetFile, value: string): void {
  files.set(file, `${files.get(file) ?? ""}\n\n${value}`);
}

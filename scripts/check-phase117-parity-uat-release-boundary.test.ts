import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "./check-phase117-parity-uat-release-boundary";

const SURFACES = {
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

const TARGET_FILES = [
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

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  completedGapClosure?: boolean;
  gapClosureStage?: boolean;
  postAuditGapPlanning?: boolean;
  maybeMutate?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_when_phase117_closeout_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();
  const gapRoot = createFixture({ gapClosureStage: true });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);
  const gapFailures = checkPhase117ParityUatReleaseBoundary(gapRoot);

  // Assert
  expect(failures).toEqual([]);
  expect(gapFailures).toEqual([]);
});

test("passes_when_completed_gap_closure_retains_phase125_and_phase126_ownership", () => {
  // Arrange
  const root = createFixture({
    completedGapClosure: true,
    gapClosureStage: true,
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_when_post_audit_gap_planning_uses_phase127_through_phase129_ownership", () => {
  // Arrange
  const root = createFixture({ postAuditGapPlanning: true });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_a_required_v2_1_surface_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.surfaces = index.surfaces.filter(
          (surface: { name?: string }) => surface.name !== "v2-1-compact-block-reconstruction",
        );
        index.checklist.surfaces = index.checklist.surfaces.filter(
          (surface: { id?: string }) => surface.id !== "v2-1-compact-block-reconstruction",
        );
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing v2.1 surface v2-1-compact-block-reconstruction");
});

test("fails_when_a_requirement_has_duplicate_surface_owners", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.checklist.surfaces[0].requirements.push("BOUND-01");
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("BOUND-01 must have exactly one parity surface owner");
});

test("fails_when_a_surface_entry_is_duplicated", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.surfaces.push({ ...index.surfaces[0] });
        index.checklist.surfaces.push({ ...index.checklist.surfaces[0] });
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("must have exactly one top-level and checklist entry");
});

test("fails_when_a_checklist_surface_is_not_done", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        index.checklist.surfaces[0].status = "blocked";
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("checklist v2.1 surface");
  expect(failures).toContain("must be done");
});

test("fails_when_a_required_knots_anchor_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      mutateIndex(files, (index) => {
        for (const surface of index.checklist.surfaces) {
          surface.upstream.sources = surface.upstream.sources.filter(
            (anchor: string) => anchor !== "packages/bitcoin-knots/src/blockencodings.cpp",
          );
          surface.upstream.tests = surface.upstream.tests.filter(
            (anchor: string) => anchor !== "packages/bitcoin-knots/src/blockencodings.cpp",
          );
        }
      });
      mutateBreadcrumbs(files, (breadcrumbs) => {
        for (const group of breadcrumbs.groups) {
          group.breadcrumbs = group.breadcrumbs.filter(
            (anchor: string) => anchor !== "packages/bitcoin-knots/src/blockencodings.cpp",
          );
        }
      });
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing Phase 117 parity-index Knots anchor");
  expect(failures).toContain("missing Phase 117 breadcrumb Knots anchor");
});

test("fails_when_a_required_breadcrumb_group_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(files, "docs/parity/source-breadcrumbs.json", "network-compact-block-download", "missing-group");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing breadcrumb group network-compact-block-download");
});

test("fails_when_a_required_cargo_command_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "docs/operator/runtime-guide.md",
        "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
        "missing cargo command",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing Phase 117 runtime guide command");
});

test("fails_when_a_required_bazel_command_is_missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "docs/operator/runtime-guide.md",
        "bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json",
        "missing bazel command",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("missing Phase 117 runtime guide command");
});

test("fails_when_visible_verifier_order_is_wrong", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        "bun run scripts/check-phase117-parity-uat-release-boundary.ts",
        "bun run scripts/missing-phase117.ts",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 visible commands");
});

test("fails_when_executable_verifier_order_is_wrong", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        'run_step "check Phase 117 parity UAT release boundary"',
        'run_step "missing Phase 117 parity UAT release boundary"',
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 executable commands");
});

test("fails_when_an_expected_run_step_label_executes_the_wrong_command", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        'run_step "check Phase 117 parity UAT release boundary" bun run scripts/check-phase117-parity-uat-release-boundary.ts',
        'run_step "check Phase 117 parity UAT release boundary" bash scripts/wrong-command.sh',
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 executable commands");
});

test("fails_when_visible_phase117_commands_follow_pure_core", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        "bun run scripts/check-phase117-parity-uat-release-boundary.ts\nbash scripts/check-pure-core-deps.sh",
        "bash scripts/check-pure-core-deps.sh\nbun run scripts/check-phase117-parity-uat-release-boundary.ts",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("exact Phase 117 visible commands");
});

test("fails_when_default_verifier_adds_a_generic_soak_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "scripts/verify.sh", 'run_step "external duration" bash scripts/run-soak-review.sh');
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("must not run soak workflows");
});

test("fails_when_default_verifier_hides_a_public_network_gate_on_a_continuation_line", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "scripts/verify.sh",
        ['run_step "external review" \\', "  bash scripts/public-network-review.sh"].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("default verifier must not run public-network");
});

test("fails_when_docs_claim_public_block_serving_by_default", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin supports public block serving by default.");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("forbidden positive Phase 117 claim");
});

test("fails_when_docs_claim_production_readiness", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "docs/parity/release-readiness.md", "v2.1 provides production full-node readiness.");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production full-node readiness");
});

test("fails_when_a_deferred_topic_masks_an_unrelated_positive_overclaim", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "docs/parity/release-readiness.md",
        "Package relay remains deferred, while production service operation is supported.",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production service operation");
});

test("fails_when_a_table_cell_mixes_a_deferred_topic_with_a_positive_overclaim", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(
        files,
        "docs/parity/support-matrix.md",
        "| Package relay remains deferred, but Open Bitcoin supports production service operation. | `supported` |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("production service operation");
});

test("fails_when_default_verifier_adds_a_public_network_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "scripts/verify.sh", 'run_step "live gate" bash scripts/run-live-mainnet-smoke.ts');
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("default verifier must not run run-live-mainnet-smoke");
});

test("allows_bounded_explicit_default_off_compact_relay_claims", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides bounded, explicit, default-off compact block relay.");
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("allows_deferred_and_optional_uat_wording", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Package relay remains deferred. Public-network compact-relay review is optional UAT.");
      append(
        files,
        "docs/parity/support-matrix.md",
        "| Open Bitcoin supports production service operation. | `deferred` | not allowed yet |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

function createFixture(options: FixtureOptions = {}): string {
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
  files.set(
    ".planning/REQUIREMENTS.md",
    createRequirements(
      options.gapClosureStage ?? false,
      options.completedGapClosure ?? false,
      options.postAuditGapPlanning ?? false,
    ),
  );
  files.set("docs/operator/runtime-guide.md", `${commonText}\n${requiredCommands().join("\n")}`);
  files.set("scripts/verify.sh", createVerifyScript());
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  return root;
}

function createParityIndex(): {
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

function createBreadcrumbs(): string {
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

test("fails_when_gap_closure_requirement_maps_to_stale_phase", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      const current = files.get(".planning/REQUIREMENTS.md") ?? "";
      files.set(
        ".planning/REQUIREMENTS.md",
        current.replace("| CMP-05 | Phase 118 |", "| CMP-05 | Phase 113 |"),
      );
    },
  });
  const gapRoot = createFixture({
    gapClosureStage: true,
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| CMP-05 | Phase 126 | Pending |",
        "| CMP-05 | Phase 118 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");
  const gapFailures = checkPhase117ParityUatReleaseBoundary(gapRoot).join("\n");

  // Assert
  expect(failures).toContain("CMP-05 must map to Phase 118 exactly once");
  expect(gapFailures).toContain("CMP-05 must map to Phase 126 exactly once");
});

test("fails_when_post_audit_gap_planning_retains_stale_requirement_ownership", () => {
  // Arrange
  const root = createFixture({
    postAuditGapPlanning: true,
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| BSRV-03 | Phase 127 | Pending |",
        "| BSRV-03 | Phase 110 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("BSRV-03 must map to Phase 127 exactly once");
});

function createRequirements(
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

function createVerifyScript(): string {
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

function requiredCommands(): string[] {
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

function requiredAnchors(): string[] {
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

function requirementPhases(): Record<string, string> {
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

function mutateIndex(files: Map<TargetFile, string>, mutate: (index: any) => void): void {
  const index = JSON.parse(files.get("docs/parity/index.json") ?? "{}") as any;
  mutate(index);
  files.set("docs/parity/index.json", JSON.stringify(index, null, 2));
}

function mutateBreadcrumbs(files: Map<TargetFile, string>, mutate: (breadcrumbs: any) => void): void {
  const breadcrumbs = JSON.parse(files.get("docs/parity/source-breadcrumbs.json") ?? "{}") as any;
  mutate(breadcrumbs);
  files.set("docs/parity/source-breadcrumbs.json", JSON.stringify(breadcrumbs, null, 2));
}

function replace(files: Map<TargetFile, string>, file: TargetFile, needle: string, value: string): void {
  files.set(file, (files.get(file) ?? "").replace(needle, value));
}

function append(files: Map<TargetFile, string>, file: TargetFile, value: string): void {
  files.set(file, `${files.get(file) ?? ""}\n\n${value}`);
}

import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase95NetworkParticipationReleaseBoundary } from "./check-phase95-network-participation-release-boundary";

const SURFACE_ID = "v1-9-network-participation-release-boundary";
const PHASE94_TEST_COMMAND =
  "bun test scripts/check-phase94-dos-resource-governance.test.ts";
const PHASE94_CHECKER_COMMAND =
  "bun run scripts/check-phase94-dos-resource-governance.ts";
const PHASE95_TEST_COMMAND =
  "bun test scripts/check-phase95-network-participation-release-boundary.test.ts";
const PHASE95_CHECKER_COMMAND =
  "bun run scripts/check-phase95-network-participation-release-boundary.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIRED_KNOTS_ANCHORS = [
  "packages/bitcoin-knots/src/net.cpp",
  "packages/bitcoin-knots/src/net_processing.cpp",
  "packages/bitcoin-knots/src/addrman.cpp",
  "packages/bitcoin-knots/src/banman.cpp",
  "packages/bitcoin-knots/src/net_permissions.cpp",
] as const;
const PHASE_REQUIREMENTS = {
  "v1-9-inbound-listener-admission-policy": [
    "INB-01",
    "INB-02",
    "INB-03",
    "INB-04",
    "INB-05",
  ],
  "v1-9-peer-permissions-connection-classes": [
    "PERM-01",
    "PERM-02",
    "PERM-03",
    "PERM-04",
  ],
  "v1-9-address-advertisement-discovery-boundaries": [
    "ADDR-01",
    "ADDR-02",
    "ADDR-03",
    "ADDR-04",
  ],
  "v1-9-eviction-ban-misbehavior-policy": [
    "EVICT-01",
    "EVICT-02",
    "EVICT-03",
    "EVICT-04",
  ],
  "v1-9-dos-resource-governance": [
    "DOS-01",
    "DOS-02",
    "DOS-03",
    "DOS-04",
    "DOS-05",
  ],
  [SURFACE_ID]: [
    "BOUND-01",
    "BOUND-02",
    "BOUND-03",
    "BOUND-04",
    "BOUND-05",
    "BOUND-06",
  ],
} as const;
const REQUIREMENT_PHASE_ASSIGNMENTS = {
  "INB-01": 98,
  "INB-02": 98,
  "INB-03": 98,
  "INB-04": 98,
  "INB-05": 97,
  "PERM-01": 91,
  "PERM-02": 91,
  "PERM-03": 91,
  "PERM-04": 91,
  "ADDR-01": 92,
  "ADDR-02": 92,
  "ADDR-03": 92,
  "ADDR-04": 92,
  "EVICT-01": 93,
  "EVICT-02": 93,
  "EVICT-03": 96,
  "EVICT-04": 96,
  "DOS-01": 94,
  "DOS-02": 94,
  "DOS-03": 96,
  "DOS-04": 97,
  "DOS-05": 94,
  "BOUND-01": 95,
  "BOUND-02": 95,
  "BOUND-03": 95,
  "BOUND-04": 95,
  "BOUND-05": 95,
  "BOUND-06": 98,
} as const;
const ROADMAP_TRACEABILITY_ROWS = [
  { phase: 90, requirements: [] },
  { phase: 91, requirements: ["PERM-01", "PERM-02", "PERM-03", "PERM-04"] },
  { phase: 92, requirements: ["ADDR-01", "ADDR-02", "ADDR-03", "ADDR-04"] },
  { phase: 93, requirements: ["EVICT-01", "EVICT-02"] },
  { phase: 94, requirements: ["DOS-01", "DOS-02", "DOS-05"] },
  { phase: 95, requirements: ["BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05"] },
  { phase: 96, requirements: ["EVICT-03", "EVICT-04", "DOS-03"] },
  { phase: 97, requirements: ["INB-05", "DOS-04"] },
  { phase: 98, requirements: ["INB-01", "INB-02", "INB-03", "INB-04", "BOUND-06"] },
] as const;
const TARGET_FILES = [
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  "README.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/operator/runtime-guide.md",
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

test("passes with complete Phase 95 release-boundary corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when any required Knots anchor is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromAllFiles(files, "packages/bitcoin-knots/src/net.cpp");
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-02");
});

test("fails positive deferred network participation claims", () => {
  // Arrange
  const claims = [
    "Phase 95 provides transaction relay support.",
    "Phase 95 provides compact block relay support.",
    "Phase 95 provides mempool propagation support.",
    "Phase 95 provides full address relay support.",
    "Phase 95 provides public inbound default behavior.",
    "Phase 95 says public-network CI is enabled.",
    "Phase 95 supports production service operation.",
    "Phase 95 has production full-node readiness.",
  ];
  const roots = claims.map((claim) =>
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/catalog/p2p.md") ?? "";
        files.set("docs/parity/catalog/p2p.md", `${current}\n${claim}\n`);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-01");
  }
});

test("fails same-unit positive claims with unrelated allowance wording", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/checklist.md") ?? "";
        files.set(
          "docs/parity/checklist.md",
          `${current}\n| \`future-mask\` | \`done\` | \`BOUND-01\` | Phase 95 provides transaction relay support. | Future scoped evidence required. |\n`,
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        const current = files.get("docs/parity/catalog/p2p.md") ?? "";
        files.set(
          "docs/parity/catalog/p2p.md",
          `${current}\nPhase 95 provides transaction relay support while a future scoped relay audit remains pending.\n`,
        );
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-01");
  }
});

test("fails positive README network participation claims", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      const current = files.get("README.md") ?? "";
      files.set("README.md", `${current}\nPhase 95 provides transaction relay support.\n`);
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-01");
});

test("fails when required Cargo or Bazel UAT command families are missing", () => {
  // Arrange
  const roots = [
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin",
  ].map((missingCommand) =>
    createFixture({
      maybeMutateFiles(files) {
        replaceInFile(files, "docs/operator/runtime-guide.md", missingCommand, "");
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-04");
  }
});

test("fails when resource-governance support redaction roots are missing", () => {
  // Arrange
  const roots = [
    "redact_inbound_resource_governance_evidence",
    "inbound_support_redacts_raw_phase94_resource_governance_material",
  ].map((missingRoot) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, missingRoot);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("BOUND-05");
  }
});

test("fails when v1.9 requirement IDs are duplicated or omitted", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "docs/parity/index.json", '"BOUND-06"', '"BOUND-05"');
      replaceInFile(files, "docs/parity/checklist.md", "`BOUND-06`", "`BOUND-05`");
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-06");
});

test("fails when gap-closure traceability maps requirements to stale phases", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/REQUIREMENTS.md",
        "| INB-05 | Phase 97 | Complete |",
        "| INB-05 | Phase 90 | Complete |",
      );
      replaceInFile(
        files,
        ".planning/ROADMAP.md",
        "| Phase 97 | INB-05, DOS-04 | 2 |",
        "| Phase 97 | — | 0 |",
      );
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain(
    "BOUND-06 requirements traceability missing INB-05 -> Phase 97",
  );
  expect(failures.join("\n")).toContain("BOUND-06 roadmap phase traceability");
});

test("fails when Phase 95 verifier commands exist only in VERIFY_COMMAND_ORDER", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "scripts/verify.sh",
        [
          "#!/usr/bin/env bash",
          "set -euo pipefail",
          ": <<'VERIFY_COMMAND_ORDER'",
          PHASE94_TEST_COMMAND,
          PHASE94_CHECKER_COMMAND,
          PHASE95_TEST_COMMAND,
          PHASE95_CHECKER_COMMAND,
          "VERIFY_COMMAND_ORDER",
          `run_step "Phase 94 DoS/resource governance checker tests" ${PHASE94_TEST_COMMAND}`,
          `run_step "Phase 94 DoS/resource governance checker" ${PHASE94_CHECKER_COMMAND}`,
          `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-03");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase95-"));
  tempRoots.push(root);

  const files = fixtureFiles();
  options.maybeMutateFiles?.(files);

  for (const [file, contents] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }

  return root;
}

function fixtureFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    [".planning/REQUIREMENTS.md", requirementsText()],
    [".planning/ROADMAP.md", roadmapText()],
    ["README.md", readmeText()],
    ["docs/parity/index.json", parityIndexText()],
    ["docs/parity/checklist.md", checklistText()],
    ["docs/parity/catalog/p2p.md", p2pCatalogText()],
    ["docs/parity/release-readiness.md", releaseReadinessText()],
    ["docs/parity/production-claim-boundary.md", productionBoundaryText()],
    ["docs/parity/support-matrix.md", supportMatrixText()],
    ["docs/operator/runtime-guide.md", runtimeGuideText()],
    ["packages/open-bitcoin-cli/src/operator/support/redaction.rs", redactionText()],
    ["packages/open-bitcoin-cli/src/operator/support/tests.rs", supportTestsText()],
    ["scripts/verify.sh", verifyScriptText()],
  ]);
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const [file, current] of files) {
    files.set(file, current.replaceAll(needle, ""));
  }
}

function replaceInFile(
  files: Map<TargetFile, string>,
  file: TargetFile,
  needle: string,
  replacement: string,
): void {
  files.set(file, (files.get(file) ?? "").replaceAll(needle, replacement));
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: Object.entries(PHASE_REQUIREMENTS).map(([id, requirements]) => ({
          id,
          status: "done",
          requirements,
          evidence:
            id === SURFACE_ID
              ? [
                  "docs/parity/catalog/p2p.md",
                  "docs/parity/checklist.md",
                  "docs/parity/release-readiness.md",
                  "docs/parity/production-claim-boundary.md",
                  "docs/parity/support-matrix.md",
                  "docs/operator/runtime-guide.md",
                  "packages/open-bitcoin-cli/src/operator/support/redaction.rs",
                  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
                  "scripts/check-phase95-network-participation-release-boundary.ts",
                  "scripts/check-phase95-network-participation-release-boundary.test.ts",
                  "scripts/verify.sh",
                  ".planning/REQUIREMENTS.md",
                  ".planning/ROADMAP.md",
                ]
              : ["docs/parity/catalog/p2p.md"],
          upstream: id === SURFACE_ID ? { sources: [...REQUIRED_KNOTS_ANCHORS] } : undefined,
        })),
      },
    },
    null,
    2,
  );
}

function readmeText(): string {
  return [
    "# Open Bitcoin",
    "v1.9 documents bounded opt-in inbound listener, admission, permission, address, eviction, ban, and resource-governance evidence.",
    "It does not claim transaction relay support, compact block relay support, mempool propagation support, public inbound defaults, production service operation, or production full-node readiness.",
  ].join("\n");
}

function checklistText(): string {
  const rows = Object.entries(PHASE_REQUIREMENTS).map(
    ([surface, requirements]) =>
      `| \`${surface}\` | \`done\` | ${requirements.map((id) => `\`${id}\``).join(", ")} | evidence | Scoped no-claim wording keeps deferred surfaces outside v1.9. | Future scoped evidence required. |`,
  );
  return [
    "# Parity Checklist",
    "| Surface | Status | Requirements | Evidence | Known Gaps | Suspected Unknowns |",
    "| --- | --- | --- | --- | --- | --- |",
    ...rows,
  ].join("\n");
}

function p2pCatalogText(): string {
  return [
    "# P2P Networking And Sync",
    `Phase 95 \`${SURFACE_ID}\` evidence keeps ${PHASE_REQUIREMENTS[SURFACE_ID].join(
      ", ",
    )} auditable.`,
    REQUIRED_KNOTS_ANCHORS.join(" "),
    "support redaction roots deterministic verification references 28/28 v1.9 requirement traceability",
    "Phase 95 does not claim transaction relay, compact block relay, mempool propagation, full address relay beyond Phase 92, public inbound defaults, public-network CI, production service operation, or production full-node readiness.",
  ].join("\n");
}

function releaseReadinessText(): string {
  return [
    "Surface id: `v1-9-network-participation-release-boundary`",
    "The v1.9 closeout links BOUND-01 through BOUND-06 to parity roots and runtime guide commands.",
    REQUIRED_KNOTS_ANCHORS.join(" "),
    "v1.9 does not claim transaction relay, compact block relay, mempool propagation, public inbound defaults, production service operation, or production full-node readiness.",
    "Requirement traceability stays exactly once across Phase 90 through Phase 98.",
  ].join("\n");
}

function productionBoundaryText(): string {
  return [
    "v1.9 bounded opt-in inbound evidence is a release-boundary only.",
    "This boundary does not claim public inbound defaults, transaction relay, compact block relay, mempool propagation, full address relay, production-service operation, or production full-node readiness.",
    "Future scoped milestones are required before deferred surfaces are promoted.",
  ].join("\n");
}

function supportMatrixText(): string {
  return [
    "Inbound serving is `opt-in UAT` evidence only.",
    "This evidence does not claim production full-node readiness, public inbound defaults, production network participation, transaction relay, compact block relay, mempool propagation, full address relay, or production-service operation.",
  ].join("\n");
}

function runtimeGuideText(): string {
  return [
    "# Phase 95 Network Participation Closeout Review",
    "The closeout is deterministic by default, public-network-free, service-manager-free, and not a production readiness claim.",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -regtest -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444",
    "bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -regtest -openbitcoininbound=1 -openbitcoinlisten=127.0.0.1:18444",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -regtest openbitcoinnetworkstatus",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -regtest openbitcoinnetworkstatus",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest status --format json",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest status --format json",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
    PHASE95_TEST_COMMAND,
    PHASE95_CHECKER_COMMAND,
    "bash scripts/verify.sh",
    "This evidence does not claim public inbound defaults, transaction relay, compact block relay, mempool propagation, full address relay, production-service operation, or production full-node readiness.",
  ].join("\n");
}

function redactionText(): string {
  return [
    'const INBOUND_ENDPOINT_REDACTION_SAFEGUARD: &str = "inbound peer endpoints bounded/redacted";',
    'const INBOUND_PERMISSION_REDACTION_SAFEGUARD: &str = "inbound permission labels bounded to machine classes/effects";',
    'const INBOUND_ADDRESS_REDACTION_SAFEGUARD: &str = "inbound address boundary evidence bounded/redacted";',
    'const INBOUND_PEER_POLICY_REDACTION_SAFEGUARD: &str = "inbound peer policy evidence bounded/redacted";',
    'const INBOUND_RESOURCE_GOVERNANCE_REDACTION_SAFEGUARD: &str = "inbound resource-governance evidence bounded/redacted";',
    'const REDACTED_RESOURCE_GOVERNANCE_LABEL: &str = "redacted_resource_governance_evidence";',
    "fn redact_inbound_resource_governance_evidence() {}",
    "fn sanitized_resource_governance_text() {}",
    "peer_id= raw_endpoint payload_bytes permission_string credential secret cookie= config=",
  ].join("\n");
}

function supportTestsText(): string {
  return [
    "fn inbound_support_redacts_raw_phase94_resource_governance_material() {}",
    "redacted_resource_governance_evidence",
    "127.0.0.1: 0.0.0.0: ::1 peer_id= peer- raw_endpoint payload_bytes raw_permission permission_string credential secret cookie= config=",
  ].join("\n");
}

function requirementsText(): string {
  return [
    "# Requirements",
    ...phaseTraceRows(),
    "**Coverage:**",
    "- v1.9 requirements: 28 total",
    "- Mapped to phases: 28",
    "- Unmapped: 0",
    "- Pending Phase 98 verification: 5",
  ].join("\n");
}

function roadmapText(): string {
  return [
    "# Roadmap",
    [
      "**Coverage:** 28/28 v1.9 requirements mapped, 0 unmapped.",
      "Five requirements are pending Phase 98 verification.",
    ].join(" "),
    "| Phase | Requirements | Count |",
    "| --- | --- | ---: |",
    ...ROADMAP_TRACEABILITY_ROWS.map(({ phase, requirements }) => {
      const requirementText = requirements.length === 0 ? "—" : requirements.join(", ");
      return `| Phase ${phase} | ${requirementText} | ${requirements.length} |`;
    }),
  ].join("\n");
}

function phaseTraceRows(): string[] {
  const rows = [
    "| Requirement | Phase | Status |",
    "| --- | --- | --- |",
  ];
  for (const [requirement, phase] of Object.entries(REQUIREMENT_PHASE_ASSIGNMENTS)) {
    rows.push(`| ${requirement} | Phase ${phase} | Complete |`);
  }
  return rows;
}

function verifyScriptText(): string {
  return [
    "#!/usr/bin/env bash",
    "set -euo pipefail",
    "# Phase 93 is followed by Phase 94, and Phase 94 is followed by Phase 95.",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE94_TEST_COMMAND,
    PHASE94_CHECKER_COMMAND,
    PHASE95_TEST_COMMAND,
    PHASE95_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "Phase 94 DoS/resource governance checker tests" ${PHASE94_TEST_COMMAND}`,
    `run_step "Phase 94 DoS/resource governance checker" ${PHASE94_CHECKER_COMMAND}`,
    `run_step "Phase 95 network participation release boundary checker tests" ${PHASE95_TEST_COMMAND}`,
    `run_step "Phase 95 network participation release boundary checker" ${PHASE95_CHECKER_COMMAND}`,
    `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
  ].join("\n");
}

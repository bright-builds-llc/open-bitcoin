import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase106ParityUatReleaseBoundary } from "./check-phase106-parity-uat-release-boundary";

const TARGET_FILES = [
  "README.md",
  ".planning/milestones/v2.0-REQUIREMENTS.md",
  ".planning/milestones/v2.0-ROADMAP.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/catalog/rpc-cli-config.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
  "docs/parity/release-readiness.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/check-phase106-parity-uat-release-boundary.ts",
  "scripts/check-phase106-parity-uat-release-boundary.test.ts",
  "scripts/verify.sh",
] as const;
const REQUIRED_REQUIREMENTS = ["BOUND-01", "BOUND-02", "BOUND-03", "BOUND-04", "BOUND-05"] as const;

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

test("passes_when_phase106_closeout_evidence_is_complete", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase106ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_any_phase106_requirement_is_missing", () => {
  // Arrange
  const roots = REQUIRED_REQUIREMENTS.map((requirement) =>
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, requirement);
      },
    }),
  );

  // Act
  const failureMessages = roots.map((root) => checkPhase106ParityUatReleaseBoundary(root).join("\n"));

  // Assert
  for (const [index, message] of failureMessages.entries()) {
    expect(message).toContain(REQUIRED_REQUIREMENTS[index]);
  }
});

test("fails_when_v2_requirement_has_duplicate_surface_owner", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      mutateParityIndex(files, (index) => {
        const surface = index.checklist.surfaces.find(
          (candidate: { id?: string }) => candidate.id === "v2-0-operator-rpc-metrics-logs-support-evidence",
        );
        surface.requirements.push("BOUND-01");
      });
    },
  });

  // Act
  const failures = checkPhase106ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("BOUND-01 must have exactly one parity surface owner");
});

test("fails_when_gap_closure_requirement_maps_to_stale_phase", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/milestones/v2.0-REQUIREMENTS.md",
        "| ACT-01 | Phase 107 | Complete |",
        "| ACT-01 | Phase 100 | Complete |",
      );
    },
  });

  // Act
  const failures = checkPhase106ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("ACT-01 must map to Phase 107 exactly once");
});

test("fails_when_uat_command_or_knots_anchor_is_missing", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        removeFromFile(
          files,
          "docs/operator/runtime-guide.md",
          "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human",
        );
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        removeFromAllFiles(files, "packages/bitcoin-knots/src/net_processing.cpp");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase106ParityUatReleaseBoundary(root).join("\n"));

  // Assert
  expect(failureMessages[0]).toContain("runtime guide command");
  expect(failureMessages[1]).toContain("Knots anchor");
});

test("fails_when_default_verifier_wiring_is_missing_or_out_of_order", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      removeFromFile(files, "scripts/verify.sh", "bun run scripts/check-phase106-parity-uat-release-boundary.ts");
    },
  });

  // Act
  const failures = checkPhase106ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("verifier-scope");
});

test("fails_when_default_verifier_adds_public_network_gate", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(files, "scripts/verify.sh", 'run_step "public-network relay CI" bash scripts/run-live-mainnet-smoke.ts');
    },
  });

  // Act
  const failures = checkPhase106ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("default verifier must not run public-network");
});

test("fails_when_docs_claim_deferred_public_or_production_scope", () => {
  // Arrange
  const roots = [
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "README.md", "Phase 106 supports compact block relay.");
      },
    }),
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, "docs/operator/runtime-guide.md", "Phase 106 proves production full-node readiness.");
      },
    }),
  ];

  // Act
  const failureMessages = roots.map((root) => checkPhase106ParityUatReleaseBoundary(root).join("\n"));

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("forbidden positive Phase 106 claim");
  }
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase106-"));
  tempRoots.push(root);
  const files = new Map<TargetFile, string>();
  for (const filePath of TARGET_FILES) {
    files.set(filePath, readFileSync(filePath, "utf8"));
  }
  options.maybeMutateFiles?.(files);
  for (const [filePath, content] of files.entries()) {
    const absolutePath = path.join(root, filePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, content);
  }

  return root;
}

function mutateParityIndex(files: Map<TargetFile, string>, mutate: (index: any) => void): void {
  const index = JSON.parse(files.get("docs/parity/index.json") ?? "{}");
  mutate(index);
  files.set("docs/parity/index.json", `${JSON.stringify(index, null, 2)}\n`);
}

function removeFromAllFiles(files: Map<TargetFile, string>, needle: string): void {
  for (const filePath of TARGET_FILES) {
    removeFromFile(files, filePath, needle);
  }
}

function removeFromFile(files: Map<TargetFile, string>, filePath: TargetFile, needle: string): void {
  files.set(filePath, (files.get(filePath) ?? "").replaceAll(needle, ""));
}

function replaceInFile(
  files: Map<TargetFile, string>,
  filePath: TargetFile,
  oldText: string,
  newText: string,
): void {
  files.set(filePath, (files.get(filePath) ?? "").replace(oldText, newText));
}

function appendToFile(files: Map<TargetFile, string>, filePath: TargetFile, text: string): void {
  files.set(filePath, `${files.get(filePath) ?? ""}\n${text}\n`);
}

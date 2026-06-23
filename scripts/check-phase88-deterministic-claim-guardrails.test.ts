#!/usr/bin/env bun

import { afterEach, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { checkPhase88DeterministicClaimGuardrails } from "./check-phase88-deterministic-claim-guardrails";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE88_REPO_ROOT";
const SURFACE_ID = "v1-8-deterministic-claim-guardrails";
const AUDIT_KEY = "v1_8_deterministic_claim_guardrails";
const PHASE87_CHECKER_COMMAND = "bun run scripts/check-phase87-release-readiness.ts";
const PHASE88_TEST_COMMAND =
  "bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts";
const PHASE88_CHECKER_COMMAND =
  "bun run scripts/check-phase88-deterministic-claim-guardrails.ts";
const PHASE88_REQUIREMENTS = ["REL-02", "REL-03", "REL-04"] as const;
const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
const REQUIRED_EVIDENCE = [
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/check-phase88-deterministic-claim-guardrails.ts",
  "scripts/check-phase88-deterministic-claim-guardrails.test.ts",
  "scripts/verify.sh",
] as const;
const tempRoots: string[] = [];

type FixtureOptions = {
  maybeDocAppend?: {
    file: string;
    text: string;
  };
  maybeParityIndexText?: string;
  maybeVerifyScript?: string;
};

afterEach(async () => {
  delete process.env[REPO_ROOT_OVERRIDE_ENV];

  while (tempRoots.length > 0) {
    const maybeRoot = tempRoots.pop();
    if (maybeRoot === undefined) {
      continue;
    }

    await rm(maybeRoot, { force: true, recursive: true });
  }
});

test("passes_when_phase88_fixture_contains_claim_guardrails_roots_and_verify_order", async () => {
  // Arrange
  const root = await createFixture({});
  process.env[REPO_ROOT_OVERRIDE_ENV] = root;

  // Act
  const failures = checkPhase88DeterministicClaimGuardrails();

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_release_docs_claim_production_full_node_readiness", async () => {
  // Arrange
  const root = await createFixture({
    maybeDocAppend: {
      file: "docs/parity/release-readiness.md",
      text: "Open Bitcoin is production full-node ready.",
    },
  });

  // Act
  const failures = checkPhase88DeterministicClaimGuardrails(root);

  // Assert
  expect(failures.join("\n")).toContain("production readiness claim");
});

test("fails_when_deferred_surface_is_promoted_with_positive_predicate", async () => {
  // Arrange
  const root = await createFixture({
    maybeDocAppend: {
      file: "docs/parity/support-matrix.md",
      text: "Inbound serving is fully supported for default releases.",
    },
  });

  // Act
  const failures = checkPhase88DeterministicClaimGuardrails(root);

  // Assert
  expect(failures.join("\n")).toContain("deferred surface promotion");
});

test("allows_scoped_no_claim_deferred_or_opt_in_uat_wording", async () => {
  // Arrange
  const root = await createFixture({
    maybeDocAppend: {
      file: "docs/parity/production-claim-boundary.md",
      text: [
        "This does not claim Open Bitcoin has production full-node readiness.",
        "| migration apply mode | `deferred` | not allowed yet | future gate before any production-ready language |",
        "The opt-in UAT wording may mention public-network default checks outside default verification.",
      ].join("\n"),
    },
  });

  // Act
  const failures = checkPhase88DeterministicClaimGuardrails(root);

  // Assert
  expect(failures).toEqual([]);
});

test("fails_when_phase88_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      ": <<'VERIFY_COMMAND_ORDER'",
      PHASE87_CHECKER_COMMAND,
      PHASE88_TEST_COMMAND,
      PHASE88_CHECKER_COMMAND,
      "VERIFY_COMMAND_ORDER",
      `run_step "check Phase 87 release readiness" ${PHASE87_CHECKER_COMMAND}`,
    ].join("\n"),
  });

  // Act
  const failures = checkPhase88DeterministicClaimGuardrails(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});

test("fails_when_default_verify_contains_public_network_service_or_multiday_drift", async () => {
  // Arrange
  const roots = await Promise.all(
    [
      "run-live-mainnet-smoke",
      "systemctl status open-bitcoin",
      "launchctl print gui/501/open-bitcoin",
      "sleep 259200",
      "sleep 86400",
      "--restart-after-progress",
      "brew services",
      "public-network CI",
      "public-network default checks",
      "release-blocking live sync",
      "automatic support-bundle upload",
      "destructive repair",
      "broad production-node readiness",
    ].map((forbiddenText) =>
      createFixture({ maybeVerifyScript: `${verifyScriptText()}\n${forbiddenText}\n` }),
    ),
  );

  // Act
  const failureMessages = roots.map((root) =>
    checkPhase88DeterministicClaimGuardrails(root).join("\n"),
  );

  // Assert
  for (const message of failureMessages) {
    expect(message).toContain("default verifier boundary");
  }
});

async function createFixture(options: FixtureOptions): Promise<string> {
  const root = await mkdtemp(path.join(os.tmpdir(), "open-bitcoin-phase88-"));
  tempRoots.push(root);

  const files = new Map<string, string>([
    ["docs/parity/index.json", options.maybeParityIndexText ?? parityIndexText()],
    ["scripts/verify.sh", options.maybeVerifyScript ?? verifyScriptText()],
  ]);

  for (const file of TARGET_FILES) {
    if (!files.has(file)) {
      files.set(file, humanPointerText(file));
    }
  }

  if (options.maybeDocAppend !== undefined) {
    const current = files.get(options.maybeDocAppend.file) ?? "";
    files.set(options.maybeDocAppend.file, `${current}\n${options.maybeDocAppend.text}\n`);
  }

  for (const [file, text] of files) {
    await writeFixtureFile(root, file, text);
  }

  return root;
}

async function writeFixtureFile(root: string, file: string, contents: string): Promise<void> {
  const absolutePath = path.join(root, file);
  await mkdir(path.dirname(absolutePath), { recursive: true });
  await writeFile(absolutePath, contents);
}

function parityIndexText(): string {
  return JSON.stringify(
    {
      surfaces: [{ name: SURFACE_ID, status: "done" }],
      checklist: {
        surfaces: [
          {
            id: SURFACE_ID,
            status: "done",
            requirements: PHASE88_REQUIREMENTS,
            evidence: REQUIRED_EVIDENCE,
          },
        ],
      },
      audit: {
        [AUDIT_KEY]: {
          path: "release-readiness.md",
          status: "done",
          requirements: PHASE88_REQUIREMENTS,
          evidence: REQUIRED_EVIDENCE,
        },
      },
    },
    null,
    2,
  );
}

function humanPointerText(file: string): string {
  if (file === "docs/parity/checklist.md") {
    return [
      "# Parity Checklist",
      `| \`${SURFACE_ID}\` | \`done\` | \`REL-02\`, \`REL-03\`, \`REL-04\` | guardrail evidence | The v1.8 deterministic claim guardrails do not claim production full-node readiness. | Future milestone required before deferred surfaces are promoted. |`,
    ].join("\n");
  }

  return [
    `# ${file}`,
    "The v1.8 deterministic claim guardrails define gates only and does not claim production full-node readiness.",
    "Deferred surfaces stay deferred unless a future milestone adds scoped evidence.",
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE87_CHECKER_COMMAND,
    PHASE88_TEST_COMMAND,
    PHASE88_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "check Phase 87 release readiness" ${PHASE87_CHECKER_COMMAND}`,
    `run_step "test Phase 88 deterministic claim guardrails checker" ${PHASE88_TEST_COMMAND}`,
    `run_step "check Phase 88 deterministic claim guardrails" ${PHASE88_CHECKER_COMMAND}`,
  ].join("\n");
}

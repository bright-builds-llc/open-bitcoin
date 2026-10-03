import { expect, test } from "bun:test";

import { checkPhase151ParityUatReleaseBoundary } from "../check-phase151-parity-uat-release-boundary.ts";
import { append, createFixture, replace } from "./test-fixtures.ts";
import {
  ARCHIVED_V24_AUDIT,
  ARCHIVED_V24_REQUIREMENTS,
  ARCHIVED_V24_ROADMAP,
} from "./constants.ts";

const AUDIT = ".planning/v2.4-MILESTONE-AUDIT.md";
const REQUIREMENTS = ".planning/REQUIREMENTS.md";
const ROADMAP = ".planning/ROADMAP.md";
const AUDIT_TEXT = `---
milestone: v2.4
status: gaps_found
gaps:
  requirements:
    - id: SNAP-01
      phase: 146
      status: partial
    - id: PRUN-01
      phase: 147
      status: partial
    - id: PRUN-02
      phase: 147
      status: unsatisfied
  integration:
    - id: INT-02
      requirements: [SNAP-01]
    - id: INT-01
      requirements: [PRUN-01, PRUN-02]
---
`;

function pendingFixture(maybeMutate?: (files: Map<string, string>) => void): string {
  return createFixture({
    maybeMutate(files) {
      files.set(AUDIT, AUDIT_TEXT);
      for (const id of ["SNAP-01", "PRUN-01", "PRUN-02"]) {
        replace(files, REQUIREMENTS, `- [x] **${id}**`, `- [ ] **${id}**`);
        const phase = id === "SNAP-01" ? 152 : 153;
        const row = `| ${id} | Phase ${phase} | Pending |`;
        append(files, REQUIREMENTS, row);
        append(files, ROADMAP, row);
      }
      append(files, ROADMAP, [
        "- [ ] **Phase 152: Post-Prune Wallet Rescan Eligibility**",
        "- [ ] **Phase 153: Automatic Prune Retention Integration**",
        "### Phase 152: Post-Prune Wallet Rescan Eligibility",
        "**Gap Closure**: INT-02; SNAP-01",
        "### Phase 153: Automatic Prune Retention Integration",
        "**Gap Closure**: INT-01; PRUN-01, PRUN-02",
      ].join("\n"));
      files.set(".planning/phases/152-post-prune-wallet-rescan-eligibility/README.md", "Pending");
      files.set(".planning/phases/153-automatic-prune-retention-integration/README.md", "Pending");
      maybeMutate?.(files);
    },
  });
}

test("accepts audited pending closure work without requiring false completion", () => {
  // Arrange
  const root = pendingFixture();

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("rejects unchecked requirements without an audit", () => {
  // Arrange
  const root = pendingFixture((files) => files.delete(AUDIT));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures.filter((failure) => failure.includes("must be checked"))).toHaveLength(3);
});

test("rejects pending closure backed by a passed audit", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, AUDIT, "status: gaps_found", "status: passed"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("audit must identify v2.4 gaps_found");
});

test("rejects malformed structured audit data", () => {
  // Arrange
  const root = pendingFixture((files) => files.set(AUDIT, "---\ninvalid: [\n---\n"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("invalid pending gap-closure audit");
});

test("rejects requirements not listed as audit gaps", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, AUDIT, "id: SNAP-01", "id: OTHER-01"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 must be checked");
});

test("rejects a structured audit status with the wrong data type", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, AUDIT, "status: partial", "status: [partial]"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 must be checked");
});

test("rejects closure rows that pretend to be complete", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, ROADMAP, "| SNAP-01 | Phase 152 | Pending |", "| SNAP-01 | Phase 152 | Complete |"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 must be checked");
});

test("rejects closure phase ownership mismatches", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, REQUIREMENTS, "| PRUN-02 | Phase 153 | Pending |", "| PRUN-02 | Phase 152 | Pending |"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("PRUN-02 must be checked");
});

test("rejects a closure phase listed as finished while its requirement is pending", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, ROADMAP, "- [ ] **Phase 152:", "- [x] **Phase 152:"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 must be checked");
});

test("rejects duplicate pending checklist entries", () => {
  // Arrange
  const root = pendingFixture((files) => append(files, REQUIREMENTS, "- [ ] **SNAP-01**"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 must be checked");
});

test("still rejects a public-default overclaim during pending closure", () => {
  // Arrange
  const root = pendingFixture((files) => append(files, "README.md", "Open Bitcoin enables public serving or relay by default."));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("public serving or relay by default");
});

test("still requires GRD-01 completion during pending product closure", () => {
  // Arrange
  const root = pendingFixture((files) => replace(files, REQUIREMENTS, "- [x] **GRD-01**", "- [ ] **GRD-01**"));

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("GRD-01");
});

function completeWalletClosure(files: Map<string, string>): void {
  replace(files, REQUIREMENTS, "- [ ] **SNAP-01**", "- [x] **SNAP-01**");
  for (const file of [REQUIREMENTS, ROADMAP]) {
    replace(files, file, "| SNAP-01 | Phase 152 | Pending |", "| SNAP-01 | Phase 152 | Complete |");
  }
  replace(files, ROADMAP, "- [ ] **Phase 152:", "- [x] **Phase 152:");
}

test("rejects claimed closure completion without the new owner's evidence", () => {
  // Arrange
  const root = pendingFixture(completeWalletClosure);

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 closure requires Phase 152 summary and lifecycle-valid verification");
});

function walletClosureEvidence(files: Map<string, string>, phase: number): void {
  const directory = phase === 152
    ? ".planning/phases/152-post-prune-wallet-rescan-eligibility"
    : ".planning/phases/146-wallet-leftover-snapshot-cutover";
  const lifecycle = "lifecycle_mode: yolo\nphase_lifecycle_id: fixture-closure";
  files.set(`${directory}/${phase}-CONTEXT.md`, `---\n${lifecycle}\n---\n`);
  files.set(`${directory}/${phase}-01-SUMMARY.md`, `---\n${lifecycle}\nrequirements-completed: [SNAP-01]\n---\n`);
  files.set(`${directory}/${phase}-VERIFICATION.md`, `---\n${lifecycle}\nstatus: passed\nlifecycle_validated: true\n---\nVerified SNAP-01.\n`);
}

test("accepts closure completion backed by the new owner's valid lifecycle", () => {
  // Arrange
  const root = pendingFixture((files) => {
    completeWalletClosure(files);
    walletClosureEvidence(files, 152);
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("historical completion cannot mask missing new-owner verification", () => {
  // Arrange
  const root = pendingFixture((files) => {
    completeWalletClosure(files);
    walletClosureEvidence(files, 146);
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("SNAP-01 closure requires Phase 152 summary and lifecycle-valid verification");
});

for (const variant of ["missing", "historical", "duplicate"] as const) {
  test(`rejects checked closure with ${variant} owner rows`, () => {
    // Arrange
    const root = pendingFixture((files) => {
      completeWalletClosure(files);
      const row = "| SNAP-01 | Phase 152 | Complete |";
      for (const file of [REQUIREMENTS, ROADMAP]) {
        if (variant === "duplicate") append(files, file, row);
        else replace(files, file, row, variant === "missing" ? "" : "| SNAP-01 | Phase 146 | Complete |");
      }
    });

    // Act
    const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

    // Assert
    expect(failures).toContain("SNAP-01 closure completion must map uniquely to Phase 152");
  });
}

function completedFixture(maybeMutate?: (files: Map<string, string>) => void): string {
  return pendingFixture((files) => {
    completeWalletClosure(files);
    walletClosureEvidence(files, 152);
    for (const id of ["PRUN-01", "PRUN-02"]) {
      replace(files, REQUIREMENTS, `- [ ] **${id}**`, `- [x] **${id}**`);
      for (const file of [REQUIREMENTS, ROADMAP]) {
        replace(files, file, `| ${id} | Phase 153 | Pending |`, `| ${id} | Phase 153 | Complete |`);
      }
    }
    replace(files, ROADMAP, "- [ ] **Phase 153:", "- [x] **Phase 153:");
    const directory = ".planning/phases/153-automatic-prune-retention-integration";
    const lifecycle = "lifecycle_mode: yolo\nphase_lifecycle_id: fixture-prune-closure";
    files.set(`${directory}/153-CONTEXT.md`, `---\n${lifecycle}\n---\n`);
    files.set(`${directory}/153-01-SUMMARY.md`, `---\n${lifecycle}\nrequirements-completed: [PRUN-01, PRUN-02]\n---\n`);
    files.set(`${directory}/153-VERIFICATION.md`, `---\n${lifecycle}\nstatus: passed\nlifecycle_validated: true\n---\nVerified PRUN-01 and PRUN-02.\n`);
    files.set(AUDIT, "---\nmilestone: v2.4\nstatus: tech_debt\ngaps:\n  requirements: []\n  integration: []\n---\n");
    maybeMutate?.(files);
  });
}

function archiveV24(files: Map<string, string>): void {
  for (const [live, archive] of [
    [REQUIREMENTS, ARCHIVED_V24_REQUIREMENTS],
    [ROADMAP, ARCHIVED_V24_ROADMAP],
    [AUDIT, ARCHIVED_V24_AUDIT],
  ]) {
    const maybeText = files.get(live);
    if (maybeText === undefined) throw new Error(`missing fixture control ${live}`);
    files.set(archive, maybeText);
    files.delete(live);
  }
}

test("accepts active v2.4 completion backed by both current closure owners", () => {
  // Arrange
  const root = completedFixture();

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("accepts archived-only v2.4 completion with retained phase lifecycles", () => {
  // Arrange
  const root = completedFixture(archiveV24);

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("later active milestone controls cannot override archived v2.4 evidence", () => {
  // Arrange
  const root = completedFixture((files) => {
    archiveV24(files);
    files.set(REQUIREMENTS, "**Milestone:** v2.5 Future Work\n- [ ] **FUT-01**");
    files.set(ROADMAP, "## Active Milestone: v2.5 Future Work\n| FUT-01 | Phase 154 | Pending |");
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

for (const archive of [ARCHIVED_V24_REQUIREMENTS, ARCHIVED_V24_ROADMAP, ARCHIVED_V24_AUDIT]) {
  test(`rejects missing archived v2.4 control ${archive}`, () => {
    // Arrange
    const root = completedFixture((files) => {
      archiveV24(files);
      files.delete(archive);
    });

    // Act
    const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

    // Assert
    expect(failures).toContain("missing target file");
  });
}

test("later active controls cannot mask incomplete archived v2.4 requirements", () => {
  // Arrange
  const root = completedFixture((files) => {
    archiveV24(files);
    replace(files, ARCHIVED_V24_REQUIREMENTS, "- [x] **PRUN-03**", "- [ ] **PRUN-03**");
    files.set(REQUIREMENTS, "**Milestone:** v2.5 Future Work\n- [x] **PRUN-03**");
    files.set(ROADMAP, "## Active Milestone: v2.5 Future Work");
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("PRUN-03 must be checked");
});

for (const phase of [152, 153]) {
  test(`archived v2.4 still requires current Phase ${phase} verification`, () => {
    // Arrange
    const root = completedFixture((files) => {
      archiveV24(files);
      const directory = phase === 152
        ? "152-post-prune-wallet-rescan-eligibility"
        : "153-automatic-prune-retention-integration";
      files.delete(`.planning/phases/${directory}/${phase}-VERIFICATION.md`);
      walletClosureEvidence(files, 146);
    });

    // Act
    const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

    // Assert
    expect(failures).toContain(`closure requires Phase ${phase} summary and lifecycle-valid verification`);
  });
}

test("archived v2.4 rejects mismatched current-owner lifecycle evidence", () => {
  // Arrange
  const root = completedFixture((files) => {
    archiveV24(files);
    replace(files, ".planning/phases/153-automatic-prune-retention-integration/153-VERIFICATION.md", "fixture-prune-closure", "other-lifecycle");
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("Phase 153 summary and lifecycle-valid verification");
});

test("archived pending gap audit cannot excuse unchecked closure requirements", () => {
  // Arrange
  const root = pendingFixture(archiveV24);

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("archived audit must identify completed v2.4");
  expect(failures).toContain("SNAP-01 must be checked");
});

test("resolves an archived pending audit while v2.4 controls remain active", () => {
  // Arrange
  const root = pendingFixture((files) => {
    files.set(ARCHIVED_V24_AUDIT, AUDIT_TEXT);
    files.delete(AUDIT);
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("matching active v2.4 controls take precedence over an older archive", () => {
  // Arrange
  const root = completedFixture((files) => {
    files.set(ARCHIVED_V24_REQUIREMENTS, "**Milestone:** v2.4 Old Copy\n- [ ] **GRD-01**");
    files.set(ARCHIVED_V24_ROADMAP, "# Milestone v2.4: Old Copy");
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root);

  // Assert
  expect(failures).toEqual([]);
});

test("archived roadmap completion rows remain required", () => {
  // Arrange
  const root = completedFixture((files) => {
    archiveV24(files);
    replace(files, ARCHIVED_V24_ROADMAP, "| GRD-01 | Phase 151 | Complete |", "| GRD-01 | Phase 151 | Pending |");
  });

  // Act
  const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("ROADMAP coverage row GRD-01 must be Complete");
});

for (const archive of [ARCHIVED_V24_REQUIREMENTS, ARCHIVED_V24_ROADMAP, ARCHIVED_V24_AUDIT]) {
  test(`rejects unrelated milestone content in versioned archive ${archive}`, () => {
    // Arrange
    const root = completedFixture((files) => {
      archiveV24(files);
      files.set(archive, (files.get(archive) ?? "").replaceAll("v2.4", "v2.5"));
    });

    // Act
    const failures = checkPhase151ParityUatReleaseBoundary(root).join("\n");

    // Assert
    expect(failures).toContain("identify");
    expect(failures).toContain("v2.4");
  });
}

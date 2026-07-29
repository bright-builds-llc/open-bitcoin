export const SCOPE_CLAIM_DIAGNOSTIC =
  "P134 scope: Phase 135-138 and broad relay/readiness claims must remain deferred";
export const PARITY_STATUS_DIAGNOSTIC =
  "P134 parity: pending requirements or verification gaps require in_progress status";

export const SCOPE_CLAIM_SOURCE_FILES = [
  "README.md",
  "packages/README.md",
  "docs/parity/catalog/mempool-policy.md",
  "docs/parity/checklist.md",
  "docs/parity/index.json",
] as const;

const PROHIBITED_CLAIM_VARIANTS = [
  "Phase 135 is implemented.",
  "PHASE-136 IS IMPLEMENTED!",
  "Phase 137 has shipped.",
  "Phase 138 is complete.",
  "Open Bitcoin supports a general package wire.",
  "The general-package wire is enabled.",
  "Open Bitcoin ships whole mempool rebroadcast.",
  "Whole-mempool rebroadcast is enabled.",
  "Open Bitcoin supports public/default relay.",
  "Public relay is enabled by default.",
  "Open Bitcoin guarantees transaction propagation.",
  "Transaction propagation is guaranteed.",
  "Open Bitcoin runs public-network CI.",
  "Public network CI is a default release gate.",
  "Open Bitcoin is production-ready.",
  "Open Bitcoin is ready for production.",
] as const;

type Mutator = (files: Map<string, string>) => void;

export type ScopeClaimMutation = {
  name: string;
  expectedFailure: string;
  mutate: Mutator;
};

export function scopeClaimMutations(): ScopeClaimMutation[] {
  return SCOPE_CLAIM_SOURCE_FILES.flatMap((relativePath) =>
    PROHIBITED_CLAIM_VARIANTS.map((claim) => ({
      name: `${relativePath}: ${claim}`,
      expectedFailure: SCOPE_CLAIM_DIAGNOSTIC,
      mutate: insertClaim(relativePath, claim),
    })),
  );
}

export function parityStatusMutations(): ScopeClaimMutation[] {
  return [
    {
      name: "human checklist claims done while MPLIFE remains pending",
      expectedFailure: PARITY_STATUS_DIAGNOSTIC,
      mutate: (files) => {
        setChecklistStatus(files, "done");
      },
    },
    {
      name: "machine surface claims done while MPLIFE remains pending",
      expectedFailure: PARITY_STATUS_DIAGNOSTIC,
      mutate: (files) => {
        setIndexStatus(files, "surfaces", "done");
      },
    },
    {
      name: "machine checklist claims done while MPLIFE remains pending",
      expectedFailure: PARITY_STATUS_DIAGNOSTIC,
      mutate: (files) => {
        setIndexStatus(files, "checklist", "done");
      },
    },
  ];
}

function insertClaim(relativePath: string, claim: string): Mutator {
  return (files) => {
    if (relativePath === "docs/parity/index.json") {
      const parsed = parseIndex(files);
      parsed.phase134MutationClaim = claim;
      files.set(relativePath, `${JSON.stringify(parsed, null, 2)}\n`);
      return;
    }

    files.set(relativePath, `${requireFile(files, relativePath)}\n${claim}\n`);
  };
}

function setChecklistStatus(
  files: Map<string, string>,
  status: "done" | "in_progress",
): void {
  const relativePath = "docs/parity/checklist.md";
  const source = requireFile(files, relativePath);
  const row =
    /^(\| `v2-2-authoritative-cross-cache-lifecycle-integration` \| `)(?:done|in_progress)(` \|)/m;
  if (!row.test(source)) {
    throw new Error("missing Phase 134 checklist row");
  }
  files.set(relativePath, source.replace(row, `$1${status}$2`));
}

function setIndexStatus(
  files: Map<string, string>,
  collection: "surfaces" | "checklist",
  status: "done" | "in_progress",
): void {
  const relativePath = "docs/parity/index.json";
  const parsed = parseIndex(files);
  const entries =
    collection === "surfaces" ? parsed.surfaces : parsed.checklist.surfaces;
  const maybeEntry = entries.find((entry) =>
    collection === "surfaces"
      ? entry.name === "v2-2-authoritative-cross-cache-lifecycle-integration"
      : entry.id === "v2-2-authoritative-cross-cache-lifecycle-integration",
  );
  if (maybeEntry === undefined) {
    throw new Error(`missing Phase 134 ${collection} record`);
  }
  maybeEntry.status = status;
  files.set(relativePath, `${JSON.stringify(parsed, null, 2)}\n`);
}

type ParityEntry = {
  id?: string;
  name?: string;
  status: string;
};

type ParityIndex = {
  surfaces: ParityEntry[];
  checklist: {
    surfaces: ParityEntry[];
  };
  phase134MutationClaim?: string;
};

function parseIndex(files: Map<string, string>): ParityIndex {
  const source = requireFile(files, "docs/parity/index.json");
  return JSON.parse(source) as ParityIndex;
}

function requireFile(files: Map<string, string>, relativePath: string): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}

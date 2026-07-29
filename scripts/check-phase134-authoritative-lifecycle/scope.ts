export const PHASE134_SCOPE_DIAGNOSTIC =
  "P134 scope: Phase 135-138 and broad relay/readiness claims must remain deferred";

export const PHASE134_PARITY_DIAGNOSTIC =
  "P134 parity: pending requirements or verification gaps require in_progress status";

type Phase134ScopeSources = {
  claimSurfaces: readonly string[];
  parityChecklist: string;
  parityIndex: string;
  requirements: string;
  gaps: string;
};

type Phase134ParityRecord = {
  status: string;
  knownGaps: string[];
};

type Phase134ParityStatuses = {
  surfaceStatus: string;
  checklistStatus: string;
  knownGaps: string[];
};

const PROHIBITED_SCOPE_CLAIMS = [
  /\bphase 135 (?:is )?(?:implemented|complete|done|shipped)\b/,
  /\bphase 136 (?:is )?(?:implemented|complete|done|shipped)\b/,
  /\bphase 137 (?:(?:is )?(?:implemented|complete|done)|has shipped|shipped)\b/,
  /\bphase 138 (?:is )?(?:implemented|complete|done|shipped)\b/,
  /\bsupports? (?:a )?general package wire\b/,
  /\bgeneral package wire is (?:enabled|implemented|available)\b/,
  /\bships? whole mempool rebroadcast\b/,
  /\bwhole mempool rebroadcast is (?:enabled|implemented|available)\b/,
  /\bsupports? public default relay\b/,
  /\bpublic relay is enabled by default\b/,
  /\bguarantees? (?:public )?transaction propagation\b/,
  /\btransaction propagation is guaranteed\b/,
  /\bruns? public network ci\b/,
  /\bpublic network ci is (?:a )?(?:default|release blocking)\b/,
  /\bis production ready\b/,
  /\bis ready for production\b/,
] as const;

export function phase134ScopeFailures(
  sources: Phase134ScopeSources,
): string[] {
  const failures: string[] = [];
  if (
    sources.claimSurfaces.some((surface) => {
      const normalized = normalizeClaimText(surface);
      return PROHIBITED_SCOPE_CLAIMS.some((claim) => claim.test(normalized));
    })
  ) {
    failures.push(PHASE134_SCOPE_DIAGNOSTIC);
  }

  const maybeParityStatuses = maybeParsePhase134ParityStatuses(
    sources.parityIndex,
  );
  const maybeHumanChecklistStatus = maybeChecklistPhase134Status(
    sources.parityChecklist,
  );
  const requirementsPending =
    (
      sources.requirements.match(
        /^- \[ \] \*\*MPLIFE-0[1-4]\*\*:/gm,
      ) ?? []
    ).length > 0;
  const verificationGapsPresent =
    /^status:\s*gaps_found\s*$/m.test(sources.gaps) ||
    /^gaps:\s*\n\s{2}-\s+truth:/m.test(sources.gaps);
  if (
    (requirementsPending || verificationGapsPresent) &&
    (maybeParityStatuses === null ||
      maybeParityStatuses.surfaceStatus !== "in_progress" ||
      maybeParityStatuses.checklistStatus !== "in_progress" ||
      maybeHumanChecklistStatus !== "in_progress")
  ) {
    failures.push(PHASE134_PARITY_DIAGNOSTIC);
  }

  const normalizedKnownGaps = normalizeClaimText(
    maybeParityStatuses?.knownGaps.join("\n") ?? "",
  );
  const requiredGapClaims = [
    "d 18 remains in force",
    "phase 135 owns",
    "phase 136 owns",
    "phase 137 owns",
    "phase 138 owns",
  ];
  if (
    requiredGapClaims.some((claim) => !normalizedKnownGaps.includes(claim)) &&
    !failures.includes(PHASE134_SCOPE_DIAGNOSTIC)
  ) {
    failures.push(PHASE134_SCOPE_DIAGNOSTIC);
  }

  return failures;
}

function normalizeClaimText(source: string): string {
  return source
    .normalize("NFKC")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, " ")
    .trim();
}

function maybeParsePhase134ParityStatuses(
  source: string,
): Phase134ParityStatuses | null {
  try {
    const parsed = JSON.parse(source) as unknown;
    if (!isRecord(parsed)) {
      return null;
    }
    const maybeSurface = maybeFindPhase134Record(parsed.surfaces, "name");
    if (!isRecord(parsed.checklist)) {
      return null;
    }
    const maybeChecklist = maybeFindPhase134Record(
      parsed.checklist.surfaces,
      "id",
    );
    if (maybeSurface === null || maybeChecklist === null) {
      return null;
    }
    return {
      surfaceStatus: maybeSurface.status,
      checklistStatus: maybeChecklist.status,
      knownGaps: maybeChecklist.knownGaps,
    };
  } catch {
    return null;
  }
}

function maybeFindPhase134Record(
  maybeEntries: unknown,
  identityField: "id" | "name",
): Phase134ParityRecord | null {
  if (!Array.isArray(maybeEntries)) {
    return null;
  }
  const maybeEntry = maybeEntries.find(
    (entry) =>
      isRecord(entry) &&
      entry[identityField] ===
        "v2-2-authoritative-cross-cache-lifecycle-integration",
  );
  if (!isRecord(maybeEntry) || typeof maybeEntry.status !== "string") {
    return null;
  }
  const knownGaps = Array.isArray(maybeEntry.known_gaps)
    ? maybeEntry.known_gaps.filter(
        (maybeGap): maybeGap is string => typeof maybeGap === "string",
      )
    : [];
  return { status: maybeEntry.status, knownGaps };
}

function isRecord(maybeValue: unknown): maybeValue is Record<string, unknown> {
  return typeof maybeValue === "object" && maybeValue !== null;
}

function maybeChecklistPhase134Status(source: string): string | null {
  const maybeMatch = source.match(
    /^\|\s*`v2-2-authoritative-cross-cache-lifecycle-integration`\s*\|\s*`([^`]+)`\s*\|/m,
  );
  return maybeMatch?.[1] ?? null;
}

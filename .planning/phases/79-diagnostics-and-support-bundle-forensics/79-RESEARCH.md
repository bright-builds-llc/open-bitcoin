---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 79-2026-06-17T13-53-04
generated_at: 2026-06-17T15:04:00.000Z
---

# Phase 79: Diagnostics and Support Bundle Forensics - Research

## Research Complete

Phase 79 should extend the existing support-bundle path instead of creating a
parallel diagnostics subsystem. The useful evidence already exists in typed
structures from Phases 72 through 78: `OpenBitcoinStatusSnapshot`, soak ledger
events, soak report projection, resource-bound support summaries, recovery
evidence, progress-guarantee support summaries, live-smoke summary projection,
and full-sync support verdicts.

## Current Implementation Shape

- `packages/open-bitcoin-cli/src/operator/support.rs` owns support bundle
  collection and JSON shape. `SupportEvidenceBundle` already includes
  redaction, config, status, recovery, store-health, live-smoke, full-sync,
  soak, and resource-bound evidence.
- `collect_soak_support_evidence` currently reads the latest datadir soak run,
  validates the run index path, reads `events.jsonl`, builds
  `SoakReportProjection`, and exposes only run id, final outcome, latest
  sequence, and report paths.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` provides the
  deterministic event source. `SoakLedgerEventEnvelope` includes
  `schema_version`, `run_id`, `sequence`, `recorded_at_unix_seconds`, and a
  typed `SoakLedgerEvent`.
- `SoakLedgerEvent` already carries the timeline events Phase 79 needs:
  `Started`, `Resume`, `Checkpoint`, `Stop`, and `Verdict`.
- `SoakCheckpointStatus` already contains the important diagnostic facts:
  network/lifecycle, recovery category/action/cause/next action, no-progress
  diagnosis, progress credit, rejected activity labels, expected progress
  window, last useful work, peer contribution, stall subsystem/confidence/basis
  and next action, resource-bound labels and next action, validated active-chain
  height, best-known tip height, and source status path.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` renders typed
  bundle fields into Markdown but does not yet render a forensic timeline,
  checkpoint chain, or final narrative verdict.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` already derives
  `SupportEvidenceVerdict` from shared status and live-smoke summaries. Phase 79
  should reuse this as one evidence source without overloading it into the soak
  narrative.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` already contains the
  support-bundle fixture style and is the right home for focused Phase 79 JSON
  and Markdown tests.
- Recent deterministic checkers in `scripts/check-phase75-soak-runner.ts`
  through `scripts/check-phase78-progress-guarantees.ts` use Bun, static anchors,
  phase plan frontmatter checks, docs/parity coverage checks, and
  `scripts/verify.sh` ordering checks.

## Recommended Implementation

### 1. Add a Support-Forensics Projection

Create a compact support-forensics sidecar in the existing support module tree,
preferably `packages/open-bitcoin-cli/src/operator/support/forensics.rs` plus a
`mod forensics;` declaration. The module should stay data-in/data-out where
possible and keep filesystem work in the existing support collector.

Recommended public-to-module types:

- `SupportForensicsEvidence`
  - `state: EvidenceState`
  - `timeline: Vec<ForensicTimelineEntry>`
  - `checkpoint_chain: CheckpointChainEvidence`
  - `narrative: ForensicNarrative`
  - `source: ForensicSourceEvidence`
  - `redaction: ForensicRedactionEvidence`
  - `maybe_unavailable_reason: Option<String>`
- `ForensicTimelineEntry`
  - `sequence: u64`
  - `recorded_at_unix_seconds: u64`
  - `kind: String`
  - `summary: String`
  - `evidence_basis: Vec<String>`
  - `next_action: Option<String>`
- `CheckpointChainEvidence`
  - `state: EvidenceState`
  - `algorithm: String` such as `sha256-json-v1`
  - `event_count: usize`
  - `first_sequence: Option<u64>`
  - `latest_sequence: Option<u64>`
  - `latest_hash: Option<String>`
  - `ordered: bool`
  - `missing_sequence_count: usize`
  - `truncated: bool`
  - `maybe_unavailable_reason: Option<String>`
- `ForensicNarrative`
  - `verdict: ForensicVerdict`
  - `likely_cause: String`
  - `evidence_basis: Vec<String>`
  - `next_action: String`
  - `confidence: ForensicConfidence`
- `ForensicVerdict`
  - `soak_stable`
  - `blocker_diagnosed`
  - `inconclusive`
  - `collection_failed`
- `ForensicConfidence`
  - `high`
  - `medium`
  - `low`

The projection can be built from the already-read ledger events inside
`collect_soak_support_evidence`. This avoids a second file read, keeps fallback
behavior explicit, and lets the support bundle serialize both the legacy soak
summary and the new Phase 79 forensic sidecar.

### 2. Use a Deterministic Checkpoint Chain

The checkpoint chain only needs ordering and truncation/missing-evidence
detection. It should not imply authenticity.

Use a deterministic per-event chain hash:

1. Start with a fixed seed such as `open-bitcoin-support-forensics-v1`.
2. For each event in sequence order, serialize a canonical hash input struct
   with `schema_version`, `run_id`, `sequence`, `recorded_at_unix_seconds`, and
   `event`.
3. Hash `previous_hash || canonical_event_json` using an already-available
   cryptographic hash dependency if one exists in the workspace. If no direct
   hash utility is available in the support crate, prefer a small deterministic
   Rust standard-library hasher only if the checker wording clearly calls it a
   chain digest rather than a cryptographic hash. Do not add signing or external
   trust roots.
4. Report `ordered=false` and `missing_sequence_count > 0` if event sequences
   skip numbers or regress.
5. Set `truncated=true` when `SoakLedgerReadResult.ignored_trailing_bytes > 0`.

Because the ledger read API already returns `ignored_trailing_bytes`, Phase 79
can expose partial-line truncation without treating it as fatal.

### 3. Derive Narrative From Typed Evidence Only

Narrative derivation should be conservative and enum-driven:

- `collection_failed`: support evidence could not collect a usable soak ledger.
- `soak_stable`: final soak outcome is clean completion and supporting
  checkpoint/status evidence includes progress or stay-current evidence.
- `blocker_diagnosed`: final outcome, recovery evidence, resource pressure,
  no-progress diagnosis, stall diagnosis, or peer-failure evidence indicates a
  diagnosed blocker.
- `inconclusive`: evidence is absent, conflicting, partial, or cannot justify a
  stronger outcome.

Likely cause and next action should prefer specific typed fields in this order:

1. recovery cause and recovery next action;
2. resource-bound state/labels and resource next action;
3. stall subsystem/confidence/evidence basis and stall next action;
4. no-progress diagnosis;
5. final soak outcome label;
6. conservative fallback text.

The renderer should format `likely_cause`, `evidence_basis`, `next_action`, and
`confidence`. It must not inspect Markdown, raw log lines, raw options, wallet
files, RPC secrets, or unbounded live-smoke payloads.

### 4. Preserve Shared Diagnostic Contract Boundaries

Only add fields to `OpenBitcoinStatusSnapshot` if the evidence is live or
durable node truth that all status consumers should share. The current Phase 79
needs can be satisfied by deriving support-forensics from existing status and
soak evidence, so the low-risk plan is to avoid changing node status contracts
unless implementation discovers a concrete missing typed field.

Dashboard, CLI status, RPC status, metrics, structured logs, live-smoke, and
support bundles already converge on `OpenBitcoinStatusSnapshot` and existing
summary projections. Phase 79 should document and verify this shared-source
contract rather than duplicating status classification in the support renderer.

### 5. Verification Strategy

Rust tests should cover:

- support bundle JSON includes `support_forensics`, `timeline`,
  `checkpoint_chain`, `narrative`, `likely_cause`, `evidence_basis`,
  `next_action`, `confidence`, source paths, event counts, redaction summary,
  and bundle-size facts;
- Markdown renders `## Forensic Timeline`, `## Checkpoint Chain`, and
  `## Failure Narrative`;
- ordered ledger events produce an available checkpoint chain with latest
  sequence and digest;
- skipped or regressed sequence numbers are detected without panicking;
- trailing partial ledger lines are reported as truncation;
- unavailable or incomplete evidence yields explicit unavailable or
  missing-evidence reasons;
- seeded sensitive strings such as RPC cookie contents, `rpcpassword`,
  `rpcauth`, wallet private material, and raw log text do not appear in JSON or
  Markdown output.

TypeScript checker should follow the Phase 78 checker style and verify:

- all DIAG requirement ids appear in Phase 79 plan frontmatter;
- source anchors exist for support-forensics types, support bundle fields,
  renderer sections, and tests;
- docs and parity roots mention the Phase 79 surface id and DIAG ids;
- `scripts/verify.sh` runs `bun test scripts/check-phase79-diagnostics-support-bundle.test.ts`
  followed by `bun run scripts/check-phase79-diagnostics-support-bundle.ts`;
- forbidden public-network/service-manager strings are absent from Phase 79
  default verification;
- forbidden sensitive material strings are absent from support-forensics tests
  except as redaction-negative fixtures.

Wire the checker and checker test into `scripts/verify.sh` immediately after
the Phase 78 checker.

## Documentation and Parity Updates

Update:

- `docs/operator/runtime-guide.md` with the support-bundle forensic story,
  fields, copy-pasteable repo-local UAT commands, and default-verification
  boundaries.
- `docs/architecture/status-snapshot.md` with the shared diagnostic contract and
  the distinction between runtime status truth and support-forensics sidecar
  provenance.
- `docs/architecture/operator-observability.md` with the cross-surface
  projection rule and bounded metrics/logging labels.
- `docs/parity/index.json`, `docs/parity/checklist.md`,
  `docs/parity/README.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` with a
  `phase79-diagnostics-support-bundle-forensics` surface and DIAG-01 through
  DIAG-04 traceability.
- `docs/parity/source-breadcrumbs.json` if any new Rust source or test file is
  added under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.
  A `none` breadcrumb is appropriate for Open Bitcoin-only support
  infrastructure.

## Key Risks

- Overclaiming root cause from partial evidence. Mitigate with conservative
  `inconclusive` fallback and confidence labels.
- Accidentally leaking raw support inputs. Mitigate with allowlisted projection
  types and fixture tests that seed sensitive strings.
- Turning Phase 79 into a status-contract rewrite. Mitigate by keeping
  bundle-only provenance in the support-forensics sidecar and using existing
  `OpenBitcoinStatusSnapshot` truth fields.
- Making default verification depend on public-network or multi-day behavior.
  Mitigate with deterministic fixtures and Bun/Rust checks only.

## Validation Architecture

The phase proof should be deterministic and local:

1. `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support::tests::phase79`
   or the aggregate test command inside `bash scripts/verify.sh` proves typed
   support-forensics behavior.
2. `bun test scripts/check-phase79-diagnostics-support-bundle.test.ts` proves
   the checker’s failure modes.
3. `bun run scripts/check-phase79-diagnostics-support-bundle.ts` proves source,
   test, docs, parity, and verify-script anchors.
4. `bash scripts/verify.sh` remains the final repo-native verification
   contract.

## RESEARCH COMPLETE

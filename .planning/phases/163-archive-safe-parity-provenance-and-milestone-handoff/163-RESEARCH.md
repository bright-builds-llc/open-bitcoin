---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-12T21:17:51Z"
requirements: [CFGR-01]
---

# Phase 163: Archive-Safe Parity Provenance and Milestone Handoff - Research

**Researched:** 2026-10-12. **Domain:** repository evidence selection, lifecycle claims and native verification. **Confidence:** HIGH for existing contracts; proposed decomposition is a recommendation. [VERIFIED: 163-CONTEXT.md; checker source inspected in this session]

<user-constraints>
## User Constraints (from CONTEXT.md)

The following implementation decisions, discretion and deferred ideas are copied verbatim; their source is the approved current context. [VERIFIED: 163-CONTEXT.md]

### Locked Decisions

- **D-01:** Preserve original Phase 162 certificates and six proof-source hashes; distinguish historical delivery from current closure ownership.
- **D-02:** Select fixed milestone-bound active/standard archived v2.5 requirements. Reject conflicting candidates, missing evidence, wrong milestone/baseline/owner and forged/inconsistent completion. An unrelated later active milestone cannot shadow archived v2.5.
- **D-03:** Accept CFGR-01 / Phase 163 / Pending only through linked audit/scope evidence. This transition grants no archive readiness or shipment claim.
- **D-04:** Test actual filesystem/CLI loaders and hermetic supplied maps. Missing supplied-map keys never fall back to disk.
- **D-05:** Preserve source/function, unique ownership, lifecycle, baseline, frozen hashes and excluded-capability checks. Audit/archive language requires narrow evidenced transitions.
- **D-06:** Reconcile current PROJECT/REQUIREMENTS/ROADMAP/UAT and both debug handoffs using actual native/CI receipts, preserving dated failures and proof limits.
- **D-07:** Require focused positive/refusal tests, independent review/security, full default native verification and renewed milestone audit before recommending archival. This invocation authorizes commit/push after clean verification.

### Agent Discretion

Choose the smallest robust shared provenance module and fixture arrangement. Add no dependencies or runtime scope. No pending GSD todos matched.

### Deferred Ideas (OUT OF SCOPE)

Actual archive/tag/shipment, new runtime behavior and unrelated v2.4 advisory cleanup remain separate.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID      | Description                                                                                                                                                                                                                                                    | Research Support                                                                                                                                                                              |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CFGR-01 | A contributor can audit pinned BASIC/protocol/RPC parity and intentional Fjall, BASIC-only and local resource-policy differences through source breadcrumbs, parity roots and deterministic native claim checks that retain the milestone's excluded surfaces. | Fixed v2.5 requirement selection; historical/current claim separation; actual loader tests; retained source/exclusion controls; truthful audit handoff. [VERIFIED: .planning/REQUIREMENTS.md] |

CFGR-01 is the only Pending active requirement, owned by Phase 163; CFPR-02 and CFGR-02 remain Complete under Phase 162. AG01/AF01 are the same archive handoff defect; TD01/TD02 are related record reconciliation. [VERIFIED: REQUIREMENTS.md; ROADMAP.md; 163-GAP-SCOPE.md; v2.5-MILESTONE-AUDIT.md]
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Use the existing Bun/TypeScript scripting surface; add no dependencies, Python automation, frontend or production Bitcoin library. Keep evidence decisions pure and filesystem/process work in adapters. Use early returns, typed boundary parsing and `maybe` for nullable internal names. [VERIFIED: AGENTS.md; standards/core/architecture.md; standards/core/code-shape.md; standards/languages/typescript-javascript.md; 163-CONTEXT.md]
- Preserve Bitcoin Knots `29.3.knots20260210`, parity ownership, breadcrumbs and intentional differences. Rust is pinned to `1.94.1`; no runtime change is authorized. [VERIFIED: AGENTS.md; rust-toolchain.toml; 163-CONTEXT.md]
- Use `bash scripts/verify.sh` as the full native contract; `--fast` is iteration only. Run `bun scripts/bright-builds-check.ts all`; keep managed standards/checkers unchanged. Overrides currently contain placeholders, not an applicable exemption. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; standards/core/verification.md]
- Use explicit `./` test-file paths. Run any ad-hoc Cargo/Bazel through `scripts/command-timings.ts`, serialize shared target jobs, poll resumable commands at least every 60 seconds and investigate liveness before termination. Preserve generated LOC freshness. [VERIFIED: AGENTS.md]
- Keep work inside the invoked GSD lifecycle and existing main checkout. Defer all nested commits until the strict wrapper clean gate; mandatory hooks run normally, then push upstream. Do not bypass the red planning transition. [VERIFIED: AGENTS.md; 162-CONTEXT.md D-14; parent task instructions]
- Tests cover behavior, one concern per unit test, with Arrange/Act/Assert. Keep shared ledgers append-only and Markdown frontmatter delimiters only at the top. [VERIFIED: supplied global instructions; standards/core/testing.md]
- Both active lesson files were read completely; no archives were loaded. The parent established 7,188 bytes / 2,397 estimated tokens and no audit trigger; this researcher loaded both active inputs. No project skill files were found in `.claude/skills` or `.agents/skills`. [VERIFIED: startup file reads and directory scan; parent task context]

Material guidance: root guidance, Bright Builds sidecar, placeholder overrides, architecture/code-shape/verification/testing/local-guidance and TypeScript standards. [VERIFIED: local instruction reads]

## Summary

The defect is in evidence interpretation, not filter runtime behavior. Phase 158 exports Phase 162's loader and claim validator. The loader reads only `.planning/REQUIREMENTS.md` and loads formal certificates only when all three original requirements are checked. The validator requires exactly one Phase 162 traceability row per ID and identical status across all three. Therefore honest CFGR-01 reassignment to Phase 163/Pending invalidates completed proof, and standard removal of active requirements breaks the filesystem path before archive selection is possible. [VERIFIED: scripts/check-phase158-validated-reorg.ts:36–118; scripts/check-phase162-integrated-filter-proof.ts:49–61; 163-GAP-SCOPE.md]

Use one small shared module to select fixed milestone evidence and return separate historical-delivery, current-closure and milestone-publication states. Preserve all existing function/source gates and original Phase 162 certificates. Expand actual native loader coverage; supplied maps must remain the complete universe of evidence. The requirement transition must pass before documentation activation or final full verification. [VERIFIED: 163-CONTEXT.md D-01–D-05; architecture standards; recommendation derived from inspected contracts]

**Primary recommendation:** repair historical/current provenance separation first, test the real archive sequence including its safety-copy overlap, then reconcile records and earn closure/re-audit receipts. [VERIFIED: approved work items; local complete-milestone workflow; recommendation]

## Standard Stack

| Component                                                   | Version / current contract                                                                                                                            | Use                                                                                                                                                                                                                       |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bun / `bun:test`                                            | Repo pin `1.3.9`; available executable `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64/bun` reports `1.3.9`; ordinary PATH currently reports `1.4.2`. | Existing TypeScript checkers and deterministic fixtures. Use pinned runtime when running verification. [VERIFIED: .bun-version; executable probes]                                                                        |
| `node:fs`, `node:path`, `node:crypto`, `node:child_process` | Runtime built-ins already imported by existing guards/tests.                                                                                          | Fixed-path loads, SHA256, temporary filesystem and real CLI subprocesses. [VERIFIED: Phase158/162 guards and tests]                                                                                                       |
| `Bun.YAML.parse` plus runtime shape checks                  | Existing Phase151 audit parsing pattern.                                                                                                              | Parse first frontmatter once into a narrow evidence shape; reject malformed/duplicate identity fields. [VERIFIED: scripts/check-phase151-parity-uat-release-boundary/planning-sources.ts; gap-closure.ts; recommendation] |

**Installation:** none. No packages are recommended, so registry version/publish-date checks are inapplicable. Use the repository pins rather than upgrading runtimes. [VERIFIED: 163-CONTEXT.md; AGENTS.md]

## Architecture Patterns

### Smallest robust decomposition

Recommendation: add `scripts/check-phase163-provenance.ts` (pure parsing/selection/state policy) and a focused test/fixture module, importing it from the existing Phase158 helper and Phase162 loader. Keep the Phase158 exported API compatible where practical; do not relocate its runtime contract arrays just to change provenance. New filenames are proposed, not existing files. [VERIFIED: existing importer/exporter edges; 163-CONTEXT.md Agent Discretion]

1. Filesystem adapter loads only a fixed allowlist of active/archive requirements, active/archive audit and roadmap, the approved gap scope/context, historical proof and required certificates. Missing fixed paths are represented as missing evidence. Never recursively discover a newer file or execute Git/network lookup to recover evidence. [VERIFIED: D-02/D-04; existing bounded loaders; recommendation]
1. Pure selector parses the supplied map and returns the selected v2.5 requirements with original path identity. It must not call filesystem helpers, and must not silently rewrite a later milestone's active key into historical evidence. [VERIFIED: D-02/D-04; recommendation]
1. Historical delivery validator checks Phase 162 proof, source bindings, unique parity owner and all formal gates independently of closure Pending/Complete. Closure validator recognizes only the linked Phase163 CFGR-01 transition and validates new-owner completion separately. [VERIFIED: D-01/D-03/D-05; recommendation]
1. Claim renderer exempts only exact authorized sentences for the validated evidence state. General exclusion scanning remains active, including in historical reports. [VERIFIED: phase162ClaimView; checkPhase162ExcludedClaims; D-05; recommendation]

### Fixed source and archive policy

| Source                                      | Accepted identity / transition                                                                                                                                                        |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.planning/REQUIREMENTS.md`                 | Exactly one v2.5 milestone identity and scoped IDs; unrelated later milestone ignored for v2.5 selection only when valid archive exists.                                              |
| `.planning/milestones/v2.5-REQUIREMENTS.md` | Standard archive envelope plus original v2.5 body; archive path alone and `Status: SHIPPED` alone confer no evidence.                                                                 |
| Both requirement paths                      | Accept equivalent parsed v2.5 requirement bodies during safety-copy overlap; reject differences in IDs, descriptions, ownership, status or baseline. Do not use first-wins selection. |
| Active/archive audit and roadmap            | Bind to fixed v2.5 identity. Require agreement if two relevant candidates exist; support moved audit and archived roadmap after active roadmap changes.                               |

All table policies are recommendations constrained by D-02/D-05 and the inspected archive workflow. [VERIFIED: 163-CONTEXT.md; GSD complete-milestone.md:380–477; bin/lib/milestone.cjs:160–171]

The standard CLI prepends a requirements archive header containing `Status: SHIPPED`, copies the original requirements bytes, moves the audit to `.planning/milestones/v2.5-MILESTONE-AUDIT.md`, and keeps phase directories unless optional phase archival is chosen. The workflow safety-commits while active and archive requirements coexist, then runs `git rm .planning/REQUIREMENTS.md`. Tests must model this actual sequence, not only deletion of an in-memory key. [VERIFIED: local bin/lib/milestone.cjs:160–171; complete-milestone.md:380–477]

Historical native guards require `.planning/phases/158-*` and `.planning/phases/162-*` fixed paths. Recommend explicitly retaining all historical phase directories during later milestone completion; moving them is outside the approved repair and would require another path migration. [VERIFIED: Phase158 CHECK_FILES; PHASE162 constants; complete-milestone.md optional phase archival; approved scope]

### Distinguish the state axes

| Historical delivery               | Current CFGR-01 owner/status                         | Required evidence / permitted conclusion                                                                                                                                                 |
| --------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Original pending / implementation | Phase162/Pending for all original IDs                | Retain current bootstrap/implementation tests and refusal behavior. No formal completion.                                                                                                |
| Original formal complete          | Phase162/Complete for all original IDs               | Existing six-source proof, four formal reports and unique done parity owner.                                                                                                             |
| Original formal complete          | Phase163/Pending, other two Phase162/Complete        | Same original formal evidence plus linked original gap audit, approved scope/context and unique current roadmap owner. No Phase163 completion or archive readiness.                      |
| Original formal complete          | Phase163/Complete                                    | New-owner SUMMARY and lifecycle-valid VERIFICATION with CFGR-01 coverage, actual review/security/native gates; matching current source binding. Original certificates remain historical. |
| Completed closure                 | Archived v2.5, with absent/later active requirements | Same evidence using fixed archive controls; renewed passing audit supports archive readiness. Archive metadata does not prove runtime capability or funds safety.                        |

This is the recommended minimal state machine, derived from approved transition/refusal requirements and existing Phase151 gap closure precedent. Keep delivery state and closure state separate rather than adding a blanket `complete-or-pending` exemption. [VERIFIED: 163-GAP-SCOPE.md; Phase151 gap-closure.ts; Phase158 claim validator]

Pending recognition must check audit CFGR-01 status/old phase, AG01/AF01 links, baseline, scope dependency 162, scope requirement CFGR-01, approved current lifecycle and exactly one Phase163 owner. Scope creation lifecycle `163-2026-10-12T21-03-37` is historical provenance; current context lifecycle is `163-2026-10-12T21-09-41`. Do not rewrite the scope to pretend both were one attempt. [VERIFIED: GAP-SCOPE and CONTEXT frontmatter; audit gap frontmatter; recommendation]

### Preserve source binding without freezing the defect forever

- Six proof roots are RPC `phase162.rs`, `phase162/retention.rs`, `phase162/fixtures.rs`, coins-flush `phase162.rs`, `packages/testdata/phase162/recipe.rs`, and `scripts/phase162-filter-oracle.ts`. Their ordered `path:sha256` digest is `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`. Preserve these bytes and original certificates. [VERIFIED: PHASE162_EVIDENCE; 162-12-SUMMARY.md; 162-VERIFICATION.md source_digest]
- Phase162 also freezes the entire Phase158 checker and mutation-test bytes with SHA256 constants in `checkFrozenGuards`. Editing those files necessarily requires a narrowly reviewed new checker binding, not changing the six proof roots. Record original/new byte hashes and the scoped diff; preserve every checkpoint contract/count and execute old mutations before accepting replacement hashes. [VERIFIED: check-phase162-integrated-filter-proof.ts:378–383; D-01/D-05; recommendation]
- Any extracted helper must enter Phase162's required source inventory and retained byte/structural checks, or its behavior could change outside the existing frozen-checker guard. Avoid merely freezing a thin wrapper while its new transitive dependency is unbound. [VERIFIED: CHECK_FILES and checkFrozenGuards; recommendation]
- Existing proof sentences are exact constants with “v2.5 remains unaudited and unshipped.” Retain dated historical statements, add narrowly gated current delivery/gap/audit wording, and run existing claim scanners over the remainder. A generic regex deleting audit/archive/complete prose would weaken the guard. [VERIFIED: PHASE162_COMPLETE; phase162ClaimView; checkPhase162ExcludedClaims; audit static-coupling note]

## Don't Hand-Roll

| Problem            | Use existing pattern / small extension                                                               | Avoid                                                                              |
| ------------------ | ---------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Archive lookup     | Fixed allowlisted v2.5 paths; semantic candidate comparison                                          | Recursive discovery, latest-file selection, first-wins conflicts                   |
| Source evidence    | `createHash("sha256")`, existing function/ownership/assertion guards                                 | A successful status string replacing source/certificate validation                 |
| Closure activation | Existing active-milestone lifecycle/coverage helpers where map-pure; adapter supplies their evidence | Inventing completion from unchecked scope, plan existence or old Phase162 coverage |
| Temporary layouts  | Existing `mkdtempSync`, explicit file writes and cleanup pattern                                     | Mutating this checkout's requirements to simulate archival                         |

Recommendations follow inspected Phase158/162 fixtures, Phase151 closure precedent and D-02–D-05. The old `resolvePlanningRequirementsSource` is a useful path precedent but accepts active matching text first and does not reject conflicting candidates; do not reuse it unchanged for this stricter contract. [VERIFIED: scripts/source-corpus.ts:92–111]

## Runtime State Inventory

| Category            | Items found in this phase scope                                                                      | Required action                                                                                                                                 |
| ------------------- | ---------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Stored data         | Requirement/claim records are tracked Markdown/JSON, not node database records.                      | Edit lifecycle documents; no Fjall data migration. [VERIFIED: scope; selected source roots]                                                     |
| Live service config | CI/hook routing executes native verification; no external service configuration change is requested. | Preserve workflow/hook behavior. [VERIFIED: .githooks/pre-commit; audit native/CI routing]                                                      |
| OS-registered state | None in the authorized provenance migration; no registration names are being changed.                | No launchd/systemd/task changes. [VERIFIED: approved work items]                                                                                |
| Secrets/env vars    | None renamed or required by the provenance resolver.                                                 | No secret/key migration; do not introduce repair/download/network fallback. [VERIFIED: source imports and scope]                                |
| Build artifacts     | Pinned Bun exists; LOC report is hook-generated and tracked.                                         | Use pinned runtime; retain hook-generated freshness. No installed package rename. [VERIFIED: executable probe; AGENTS.md; .githooks/pre-commit] |

Inventory is scoped to this repository evidence migration; it does not assert an exhaustive machine-wide service/secret search. [VERIFIED: approved scope]

## Common Pitfalls

1. **Formal evidence silently stops loading when CFGR-01 reopens.** Load reports according to historical proof publication state, then validate their presence. A new Pending closure cannot erase historical formal obligations. [VERIFIED: loadPhase162OptionalSources; D-01/D-03; recommendation]
1. **Tests synthesize states from the newly changed live tree.** Current Phase158/162 fixture builders rewrite Phase162 rows and some literal spacing; a live Phase163 row can survive those replacements. Build explicit canonical fixtures for each state, assert every mutation changed input, and preserve GFM spacing tolerance. [VERIFIED: transitionSources/formalSources and pending/implementation builders; recommendation]
1. **Supplied maps fill omissions from disk.** Keep selection, scope/audit parsing and lifecycle validation map-pure. Run map tests against a root with real valid evidence while intentionally omitting the key, proving no fallback. [VERIFIED: D-04; existing maybeSources boundary; recommendation]
1. **Renewed audit destroys the reason for reassignment.** Preserve an immutable copy and hash of the original gaps_found audit in the Phase163 evidence directory before renewed audit replaces the live report. Keep immutable gap lineage separate from current passing milestone audit. [VERIFIED: original audit path in scope; renewal requirement; recommendation]
1. **Circular native/formal/audit gates.** The native checker must accept a legitimate Pending closure without future Phase163 output. Earn implementation/review/native receipts first, then publish lifecycle-valid completion and rerun publication guards; only then renew the audit. Never require the upcoming successful full run or final passing audit merely to execute that full run. [VERIFIED: existing implementation/formal split; D-03/D-07; 162-CONTEXT.md D-14; recommendation]
1. **Circular self-hash.** Bind changed implementation/test inputs, not a report to its own bytes or a file containing its own required digest. Compute external manifests before publishing evidence; distinguish execution-byte scope from subsequent documentation publication. [VERIFIED: Phase162 source_digest construction; NATIVE-EVIDENCE execution/publication metadata; recommendation]
1. **False historical receipts become current claims.** Existing audit exact-head CI success belongs to `d47f6e09...`; Phase163 code needs its own full native/review/closure evidence. A fixture containing a synthetic passed report is test data only. [VERIFIED: v2.5 audit verification section; existing fixture comments; recommendation]

## Code Examples

Existing map isolation, to preserve when adding archive selection: [VERIFIED: check-phase162-integrated-filter-proof.ts:328–332]

```typescript
const sources = maybeSources ?? loadSources();
const text = (file: string) => sources.get(file) ?? "";
```

Existing source digest construction, to retain for original Phase162 evidence: [VERIFIED: check-phase158-validated-reorg.ts:109]

```typescript
const digest = createHash("sha256").update(PHASE162_EVIDENCE.map(file =>
  `${file}:${createHash("sha256").update(text(file)).digest("hex")}`
).join("\n")).digest("hex");
```

## Verification Design and Test Matrix

Nyquist Validation Architecture is intentionally omitted because `workflow.nyquist_validation` is false. The following is phase-specific regression planning, not a Nyquist activation. [VERIFIED: .planning/config.json]

| Test concern                                      | Supplied map                                                                                      | Temporary filesystem / actual CLI                       | Expected                                          |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------- |
| Original pending, implementation, formal complete | Explicit canonical fixtures                                                                       | Existing loader controls retained                       | Existing allowed transitions unchanged            |
| Legitimate Phase163/Pending                       | Original complete proof + real linked gap/scope fixtures                                          | Both Phase158 and Phase162 native loaders               | Pass historical proof; no closure/readiness grant |
| Completed Phase163 closure                        | Actual-shaped summary/verification/native/review/security fixtures                                | Both CLI commands                                       | Pass only with new-owner valid evidence           |
| Archive safety-copy overlap                       | Equal active + archive body, real envelope                                                        | Create copy before deleting active                      | Pass equal candidates; conflict refuses           |
| Standard archive                                  | Archive only, audit/roadmap moved                                                                 | Remove active file in isolated root                     | Pass completed closure                            |
| Archive + later milestone                         | Unrelated active requirements and roadmap                                                         | Both native CLI entrypoints                             | v2.5 archive remains authoritative                |
| Missing evidence                                  | Remove each requirement, proof, report, audit/scope or closure key individually                   | Missing file in fixture                                 | Nonzero/bounded targeted refusal; no disk rescue  |
| Conflicting/forged identity                       | Wrong version, baseline, owner, duplicate boxes/rows/candidates, incomplete status                | Mirror critical cases through CLI                       | Refuse even if another candidate is valid         |
| Historical tampering                              | Six-source change, stale lifecycle/digest, removed assertion/ownership/exclusion                  | Existing structural controls plus selected native cases | Existing targeted guard remains red               |
| Premature closure/readiness                       | Checkbox only, old-owner report, plan-only completion, gaps_found audit, synthetic SHIPPED header | CLI where relevant                                      | Refuse unauthorized claim                         |
| Scope/hash regression                             | Transitive helper mutation, changed checkpoint/count/IfNeeded/no-snapshot controls                | Existing mutation suites                                | Preserve original refusal semantics               |

All test cases derive from approved positive/refusal states, existing mutation suites and the inspected archive sequence. Report selection and claim publication should have separate assertions so a historical pass cannot accidentally be scored as closure completion. [VERIFIED: 163-GAP-SCOPE.md; existing checker/tests; recommendation]

Phase162 currently has no injected filesystem root: `loadSources()` anchors to `import.meta.dir`. Add a root parameter defaulting to the current repository, or copy its small import dependency tree into isolated fixtures and execute there. Prefer a backward-compatible root parameter for exported loader tests plus real CLI subprocess fixtures; verify default CLI routing itself still uses that same adapter. Merely testing a rewritten source map is insufficient. [VERIFIED: loadSources implementation; Phase158 existing maybeRoot API; D-04; recommendation]

Quick commands after implementation: [VERIFIED: existing scripts/test paths; proposed Phase163 file identified above]

```bash
bun test ./scripts/check-phase158-validated-reorg.test.ts
bun test ./scripts/check-phase162-integrated-filter-proof.test.ts
bun test ./scripts/check-phase163-provenance.test.ts
bun run scripts/check-phase158-validated-reorg.ts
bun run scripts/check-phase162-integrated-filter-proof.ts
bun scripts/bright-builds-check.ts all
bash scripts/verify.sh
```

Add new focused tests to both the documented `VERIFY_COMMAND_ORDER` and executed `run_step` sequence. Phase162's `HISTORICAL_COMMANDS` and existing ordering checks must retain all historical routes. Do not replace existing commands or use `--fast` as full evidence. Parent already reproduced current live Phase158/162 red; this research does not claim a new passing native run. [VERIFIED: verify.sh; Phase162 checkNative/contracts; parent evidence]

## Recommended Plan Split

1. **Provenance repair prerequisite:** shared fixed-path selector, separate historical/current states, original gap-audit preservation, Phase158/162 loader integration, reviewed checker-hash rebind and basic map transitions. Acceptance: actual current Phase163/Pending passes without altering six proof roots/certificates; forged transition still fails.
1. **Archive and refusal proof:** filesystem/CLI fixtures for the exact safety-copy → deletion → next-milestone sequence; complete negative matrix; retain historical mutations and wire new test route into verifier.
1. **Truthful handoff and earned gates:** reconcile PROJECT/REQUIREMENTS/ROADMAP/162-UAT, parity/contributor text and both debug records; independent review/security, full native receipt, valid Phase163 completion, renewed audit and strict final hook/commit/push.

This three-plan ordering is a recommendation derived from four approved work items: combine records and final closeout in the final plan to avoid publishing completion before its receipts. All nested commits remain deferred until the wrapper's final clean gate. Keep actual archival/tag/shipment outside these plans. [VERIFIED: approved scope; 162-CONTEXT.md D-14; parent instructions]

## Handoff Record Reconciliation

TD01 targets current PROJECT/REQUIREMENTS/ROADMAP and 162-UAT text. Preserve dated original Pending-stage evidence and describe CFGR-01's new owner accurately. TD02 targets `.planning/debug/phase156-automatic-writers.md` and `.planning/debug/bright-builds-file-lengths-ci.md`; both currently say awaiting human verification although later automated receipts supersede their runtime/CI uncertainty. Append dated evidence/disposition, update current focus, preserve original symptoms/red runs, and never invent a human confirmation. The Phase156 record explicitly preserves a debugger human-confirmation condition; distinguish automated closure evidence from that historical workflow condition. [VERIFIED: audit TD01/TD02; both debug files]

Existing full native receipt IDs are `6c7c6001-7fbd-42ec-875d-840c0c380707` and `b427fa6a-122a-4c28-9dcb-e58342ec3eac`; exact-head CI and Bright Builds runs are `38206596321` and `38206596267` at `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`. Verify cited local receipts/remote statuses before updating records; these are historical evidence pointers, not fresh Phase163 execution. [VERIFIED: v2.5-MILESTONE-AUDIT.md actual verification section]

Keep soft/unattained 550 MiB target, point allocation samples, synthetic-root/work/halving limits, body/wallet restrictions, BASIC/type0, single Fjall, default-off behavior and all deferred public/production/funds exclusions visible. Retain inherited v2.4 advisories as unfixed. [VERIFIED: 163-GAP-SCOPE.md; v2.5 audit evidence limits; integrated proof]

## Security Domain

Security enforcement is not explicitly false in config; the phase changes evidence trust boundaries. No node authentication/session/cryptography capability is being added. [VERIFIED: .planning/config.json; approved phase scope]

| ASVS-oriented category              | Scope applicability                        | Control                                                                                                                      |
| ----------------------------------- | ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| Authentication / Session Management | No new runtime auth/session work           | Preserve existing node boundaries; do not infer exposure from archive metadata.                                              |
| Access Control                      | Evidence claim authorization applies       | Unique owning phase, lifecycle identity and report generator; no disk rescue for supplied maps.                              |
| Input Validation                    | Yes, local Markdown/JSON/frontmatter input | Typed narrow parsing; duplicate/conflicting identity refusal; fixed file allowlist.                                          |
| Cryptography                        | Existing digest verification only          | Built-in SHA256 and existing source bindings; hashes supplement independent execution/review rather than proving authorship. |

Applicability is a scoped assessment using the researcher template's ASVS domains, not a claim of ASVS certification or a new standards-version adoption. Controls derive from the actual trust model. [VERIFIED: developer research template; existing hash/ownership checks; D-01–D-05]

| Threat                                               | Standard mitigation in this phase                                                             |
| ---------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Spoofed milestone or forged closure                  | Fixed identity, unique row/owner, actual lifecycle-bound formal evidence                      |
| Tampered or stale evidence                           | Preserved source digest, transitive checker binding, existing function/assertion gates        |
| Elevation from archive header to excluded capability | Separate publication state and narrow exact-sentence exemptions; unchanged exclusion scanning |
| Nondeterministic/incomplete input                    | Pure supplied-map validator, no fallback; bounded diagnostics and fixed filesystem candidates |

Threats/mitigations are recommendations derived from required refusal cases and preserved guards. [VERIFIED: CONTEXT D-02–D-05; GAP-SCOPE negative states; checker implementations]

## State of the Art

Existing Phase151 supports audited Pending reassignment and archived controls, while Phase158/162 still couple original completion to the active path. Reuse the repo vocabulary and lifecycle coverage concepts, but strengthen candidate conflict handling and map purity for this phase. No library modernization is needed. [VERIFIED: Phase151 planning-sources.ts/gap-closure.ts; Phase158/162 loaders; source-corpus.ts]

## Assumptions Log

No unverified external factual claims or dependency choices are used. Proposed filenames, decomposition and gate sequence are explicitly recommendations derived from inspected local contracts; they are not additional locked user decisions. [VERIFIED: sources listed below; approved discretion]

## Open Questions (RESOLVED)

- **Exact current audit/archive sentences — resolved by Plan01:** Gap: “Phase 162 scoped integrated proof remains historical delivery evidence; CFGR-01 is Pending under Phase 163 for AG01/AF01; v2.5 is audited with gaps, unarchived and unshipped.” Closure: “CFGR-01 archive-safe requirement closure is complete under Phase 163; v2.5 remains unarchived and unshipped.” Readiness: “v2.5 archive readiness is established by the renewed passing milestone audit; v2.5 remains unarchived and unshipped.” Archived controls: “v2.5 controls are archived under .planning/milestones/; this provenance does not establish new runtime capability or production/funds safety.” Plan01 tests exact authorized/premature use against linked Pending gap; earned canonical functional-closure verification plus SUMMARY activation; additionally renewed passed audit; additionally standard archived controls, respectively. Canonical functional-closure scope proves SC1–4 and SC5 readiness-gate enforcement while audit is pending/readiness unasserted; Plan04/Phase163 remain in progress until actual audit/readiness/final guards and whole-phase report refresh. No generic traceability-adapter change or missing-final audit exception is selected. Original historical sentence remains independent; no sentence asserts shipment. [RESOLVED: 163-01-PLAN.md interfaces/Task1; 163-04-PLAN.md staged gates]
- **Immutable original gap record — resolved by Plan01:** Preserve byte-for-byte `.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md` (SHA256 `f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8`) and `163-ORIGINAL-INTEGRATION-AUDIT.md` in that same directory (SHA256 `f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709`) before live audit renewal. Keep original creation-scope lifecycle distinct from current context; selection consumes immutable gap lineage and separately renewed active/archive audit. [RESOLVED: 163-01-PLAN.md Task1; 163-04-PLAN.md Task3]
- **Debug workflow status — resolved by Plan03:** Append dated actual automated receipt dispositions and update current focus in `.planning/debug/phase156-automatic-writers.md` and `.planning/debug/bright-builds-file-lengths-ci.md`; retain both files, original symptoms/failures and dates. Explicitly state human confirmation was not performed and retain Phase156's historical human-confirmation archival condition. Verify cited CI headSHA/native receipts; unavailable rechecks remain qualified historical evidence, never invented current success. [RESOLVED: 163-03-PLAN.md Task2]

## Environment Availability

Step 2.6: no new external dependencies identified; this is code/config/document provenance work. Existing pinned Bun was explicitly found; PATH Bun differs, so use the available pinned executable. No Cargo/Bazel, service or network execution was necessary for this research, and no claim is made that their full gates ran here. Full execution preparation remains the repo-native verifier's contract. [VERIFIED: tool probes; AGENTS.md; approved scope]

## Sources

### Primary (HIGH confidence)

- `scripts/check-phase158-validated-reorg.ts` and `.test.ts` — optional loader, requirement uniformity, six-source digest/formal gates, exact claim exemptions, filesystem fixture seam and existing refusal matrix. [VERIFIED: local reads]
- `scripts/check-phase162-integrated-filter-proof.ts`, `contracts.ts`, `.test.ts` — mandatory roots, actual loader, frozen checker hashes, function/ownership and verifier ordering controls. [VERIFIED: targeted local reads]
- `scripts/source-corpus.ts`; `scripts/check-phase151-parity-uat-release-boundary/{planning-sources,gap-closure}.ts` — historical archive and audited gap precedent. [VERIFIED: local reads]
- Current `163-CONTEXT.md`, `163-GAP-SCOPE.md`, REQUIREMENTS/ROADMAP/STATE; v2.5 milestone/integration audits and gap proposal — scope, blocker and receipt pointers. [VERIFIED: local reads and targeted searches]
- `docs/parity/v2-5-integrated-filter-proof.md`; Phase162 CONTEXT, VERIFICATION, NATIVE-EVIDENCE and Plan12 SUMMARY — source-bound original delivery and deferred-commit contract. [VERIFIED: relevant local sections]
- Local `/Users/peterryszkiewicz/.codex/get-shit-done/workflows/complete-milestone.md` and `bin/lib/milestone.cjs` — actual archive envelope, audit move, safety-copy overlap and requirements deletion. [VERIFIED: local installed workflow/source]
- `scripts/verify.sh`, `.githooks/pre-commit`, both TD02 debug files, AGENTS/sidecar/standards, both active lesson files — verification and truthful handoff requirements. [VERIFIED: local reads]

No web sources or new dependency research were needed for this repository-local phase. [VERIFIED: research scope]

## Metadata

**Confidence:** stack HIGH (pins and executable probes); architecture HIGH for existing behavior / MEDIUM for proposed decomposition; pitfalls HIGH (actual coupling and local archive sequence). **Research date:** 2026-10-12. **Validity:** recheck after any provenance/checker/archive workflow change; this is a source snapshot, not a 30-day guarantee. Research writing only; no implementation edits, commits or full verification were performed by this agent. [VERIFIED: session actions]

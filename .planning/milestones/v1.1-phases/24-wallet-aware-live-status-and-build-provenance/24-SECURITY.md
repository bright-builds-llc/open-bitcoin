---
phase: 24
slug: wallet-aware-live-status-and-build-provenance
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-06
generated_by: gsd-secure-phase
generated_at: 2026-05-06T02:26:20Z
---

# Phase 24 - Security

Per-phase security contract for Phase 24 wallet-aware live status and build
provenance. This audit verifies the Phase 24 implementation surfaces named by
the phase plans, summaries, verification report, and UAT gap closure. It does
not broaden into unrelated repository threats or a general vulnerability scan.

## Scope

This audit covers `24-01-PLAN.md` through `24-04-PLAN.md`, their summaries,
`24-VERIFICATION.md`, `24-UAT.md`, and the implementation surfaces for live
status wallet routing, credential-safe status rendering, and build provenance.
The Phase 24 plans did not include explicit `<threat_model>` blocks, and the
summaries did not include `## Threat Flags` sections, so the register below is
derived from the phase truths, residual risks, and security-relevant shipped
behavior.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Local wallet registry -> status RPC routing | Local durable wallet metadata chooses whether status uses root RPC, a selected wallet route, a sole wallet route, or wallet-unavailable status. | Wallet names, selected-wallet metadata, ambiguity state |
| Node RPC -> wallet status projection | Wallet RPC failures cross into operator-visible status while node-scoped RPC can remain healthy. | JSON-RPC error codes/messages, wallet balance/freshness fields, health signals |
| RPC auth config -> status surfaces | Credentials and cookie contents are consumed by the RPC client but must not appear in human or JSON status output. | Username/password auth, cookie file path, cookie contents, Authorization header |
| Build system -> operator status/dashboard | Cargo and Bazel compile-time metadata crosses into status JSON and dashboard rows. | Version, commit, build time, target, profile |
| Source date / workspace status -> build provenance | Build-time inputs can be user or build-system controlled and must retain a stable non-ambiguous operator-facing shape. | `SOURCE_DATE_EPOCH`, Bazel workspace status keys, UTC timestamp strings |
| GSD verification artifacts -> operator trust | Planning, UAT, and verification artifacts explain the shipped security posture and residual risks. | Phase plans, summaries, UAT evidence, verification commands |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status | Evidence |
|-----------|----------|-----------|-------------|------------|--------|----------|
| T-24-01 | Integrity | live status wallet routing | mitigate | `resolve_status_wallet_rpc_access()` uses selected-wallet metadata first, falls back to a sole managed wallet, and marks ambiguous multiwallet registries unavailable instead of guessing a wallet. | closed | `packages/open-bitcoin-cli/src/operator/status.rs:378`, `packages/open-bitcoin-cli/src/operator/status.rs:393`, `packages/open-bitcoin-cli/src/operator/status.rs:398`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:287`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:299`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:311` |
| T-24-02 | Availability / Integrity | live status collection | mitigate | Node reachability is derived from node-scoped RPC calls, while wallet failures only mark wallet fields unavailable and add wallet health diagnostics. | closed | `packages/open-bitcoin-cli/src/operator/status.rs:216`, `packages/open-bitcoin-cli/src/operator/status.rs:223`, `packages/open-bitcoin-cli/src/operator/status.rs:247`, `packages/open-bitcoin-cli/src/operator/status.rs:311`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:215` |
| T-24-03 | Information Disclosure | status and dashboard rendering | mitigate | Status projections carry credential source metadata without credential values, and regression coverage asserts rendered human and JSON output omit secrets, Authorization headers, passwords, and cookie contents. | closed | `packages/open-bitcoin-cli/src/operator/status.rs:72`, `packages/open-bitcoin-cli/src/operator/status.rs:77`, `packages/open-bitcoin-cli/src/operator/runtime.rs:293`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:467` |
| T-24-04 | Repudiation / Integrity | build provenance assembly | mitigate | Build provenance is assembled into the shared status snapshot from compile-time env vars with explicit unavailable states for absent metadata rather than silently omitting or inventing values. | closed | `packages/open-bitcoin-cli/src/operator/status.rs:251`, `packages/open-bitcoin-cli/src/operator/status.rs:418`, `packages/open-bitcoin-cli/src/operator/status.rs:428`, `packages/open-bitcoin-cli/src/operator/status/tests.rs:252` |
| T-24-05 | Integrity | Cargo build-time provenance | mitigate | Cargo `SOURCE_DATE_EPOCH` is formatted as UTC ISO-8601 using platform date commands and returns unavailable on formatting failure instead of emitting raw epoch seconds. | closed | `packages/open-bitcoin-cli/build.rs:21`, `packages/open-bitcoin-cli/build.rs:74`, `packages/open-bitcoin-cli/build.rs:85`, `packages/open-bitcoin-cli/build.rs:93`, `24-UAT.md` fixed-epoch evidence |
| T-24-06 | Integrity | Bazel build-time provenance | mitigate | Bazel uses the repo-owned `OPEN_BITCOIN_BUILD_TIME` workspace-status key instead of raw `{BUILD_TIMESTAMP}`, and the checker rejects non-ISO `build.build_time.value` values. | closed | `scripts/open-bitcoin-workspace-status.sh:11`, `scripts/open-bitcoin-workspace-status.sh:16`, `packages/open-bitcoin-cli/BUILD.bazel:8`, `packages/open-bitcoin-cli/BUILD.bazel:10`, `scripts/check-bazel-build-provenance.ts:34`, `scripts/check-bazel-build-provenance.ts:178` |
| T-24-07 | Repudiation | phase evidence and operator instructions | mitigate | Phase 24 UAT records concrete Cargo and Bazel reproduction commands and marks the build-time mismatch gap fixed with observed values; repo guidance now requires explicit Cargo/Bazel commands during UAT. | closed | `.planning/milestones/v1.1-phases/24-wallet-aware-live-status-and-build-provenance/24-UAT.md`, `AGENTS.md:39`, `.codex/tasks/lessons.md` |

*Status: open · closed*
*Disposition: mitigate (implementation required) · accept (documented risk) · transfer (third-party)*

## Open Threats

No open threats.

## Accepted Risks Log

No accepted risks.

## Unregistered Flags

None. The Phase 24 summary files do not contain `## Threat Flags` sections.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-06 | 7 | 7 | 0 | Codex / `gsd-secure-phase` |

## Verification Evidence

| Command / Input | Result |
|-----------------|--------|
| Phase 24 artifact review across `24-01-PLAN.md` through `24-04-PLAN.md` | Passed; no explicit `<threat_model>` blocks found, so the threat register was derived from shipped Phase 24 truths and residual risks. |
| Summary threat-flag review across `24-01-SUMMARY.md` through `24-04-SUMMARY.md` | Passed; no `## Threat Flags` sections found. |
| Cross-check against `24-VERIFICATION.md` and `24-UAT.md` | Passed; wallet-aware status, credential-safe rendering, build provenance, and gap closure evidence are represented in the register. |
| Targeted implementation audit across status, runtime, build, Bazel, and checker files named in the register | Passed; all seven mitigations have code or artifact evidence and no accepted risk is needed. |

## Residual Security Notes

- Phase 24 intentionally does not add an explicit user-facing wallet-selection
  flag to `open-bitcoin status` or `open-bitcoin dashboard`; it keeps ambiguous
  wallet state unavailable instead of guessing.
- Build provenance is operator-visible metadata, not an authenticity guarantee.
  It is useful for traceability, but it should not be treated as a signed
  supply-chain attestation.
- The archived Phase 24 lifecycle now includes the original yolo lifecycle and
  a later UAT gap-closure lifecycle. Active GSD lifecycle tooling reports that
  mix as invalid for strict lifecycle validation, but the security audit is
  scoped to the archived artifacts and the committed gap closure.

## Standards Inputs

Materially applied local `AGENTS.md`, `AGENTS.bright-builds.md`,
`standards-overrides.md`, Bright Builds pinned `standards/index.md`,
`standards/core/verification.md`, `standards/core/testing.md`, and the
`gsd-secure-phase` workflow. ASVS Level: 1.

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

**Approval:** verified 2026-05-06

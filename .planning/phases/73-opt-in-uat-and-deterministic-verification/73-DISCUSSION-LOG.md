# Phase 73: Opt-In UAT and Deterministic Verification - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md; this log preserves the
> alternatives considered.

**Date:** 2026-06-13T22:08:43.206Z
**Phase:** 73 - Opt-In UAT and Deterministic Verification
**Mode:** Yolo
**Areas discussed:** Default Hermetic Verification, Deterministic Coverage
Scope, Opt-In Public-Mainnet UAT Command Matrix, Parity And Evidence
Auditability

---

## Default Hermetic Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Existing-pattern deterministic checker in `verify.sh` | Matches Phase 61-72 Bun checker style, no new dependency, and can guard forbidden public-peer/service-manager strings. | Yes |
| Strict offline execution flags | Stronger post-bootstrap proof, but may fail fresh uncached clones and adds platform/tooling friction. | |
| Separate opt-in UAT wrapper and docs | Keeps public-mainnet workflows outside default verification while still documenting stable operator paths. | Yes, as a complement |
| Deterministic simulation harness for sync failure/recovery cases | Useful only for concrete VER-02 gaps; risks duplicating existing Phase 68-72 tests. | Conditional |

**User's choice:** Yolo-selected recommendation: extend the existing local Bun
checker and focused Rust-test pattern; keep public-mainnet UAT separate.

**Notes:** Normal `scripts/verify.sh` should not gain strict offline flags unless
Phase 73 deliberately adds a documented post-bootstrap audit mode.

---

## Deterministic Coverage Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Coverage manifest plus targeted hermetic Rust gap tests | Maps VER-02 behaviors to named deterministic tests and adds only missing narrow tests. | Yes |
| Checker-only audit over existing tests | Smallest change but can become hollow if durable UTXO/undo or crash semantics are indirect. | |
| Hermetic process-level crash harness | Stronger process-level proof but slower, more platform-sensitive, and likely too heavy for default verification. | |
| Opt-in public-mainnet UAT commands as proof | Appropriate for VER-03 operator review only, not VER-02 deterministic proof. | |

**User's choice:** Yolo-selected recommendation: audit existing Phase 68-72
coverage first; add targeted hermetic Rust gap tests only where required.

**Notes:** Crash recovery can be represented as durable reopen/recovery evidence
unless planning proves a process-level harness is short, hermetic, and stable.

---

## Opt-In Public-Mainnet UAT Command Matrix

| Option | Description | Selected |
|--------|-------------|----------|
| Central matrix in `docs/operator/runtime-guide.md` | One auditable command matrix with Cargo/Bazel forms and opt-in boundaries. | Yes |
| Workflow-specific runbook sections only | Easier in-context reading but higher drift and fragmented VER-03 proof. | |
| Script/CLI-generated UAT command printer | Reduces markdown drift but adds unnecessary implementation surface. | |
| Phase artifact only, such as `73-UAT.md` | Low blast radius but poor long-term discoverability. | |

**User's choice:** Yolo-selected recommendation: central matrix in
`docs/operator/runtime-guide.md`, with local pointers from existing sections.

**Notes:** Repo-local command forms are mandatory for operator UAT guidance:
Cargo and Bazel commands should be copy-pasteable rather than relying on an
installed alias.

---

## Parity And Evidence Auditability

| Option | Description | Selected |
|--------|-------------|----------|
| Phase-scoped evidence manifest plus validator | Useful if Phase 73 adds many non-Rust evidence/report surfaces. | Conditional |
| Extend existing breadcrumbs plus phase checker only | Lowest overhead and best fit for a narrow checker/docs/test closeout. | Yes |
| Generated v1.6 evidence index from repo scan | Broad freshness checks but risks false positives and generated-artifact churn. | |
| SLSA/in-toto-style local attestations | Strong provenance vocabulary but too heavy and out of scope. | |

**User's choice:** Yolo-selected recommendation: use the existing
checker-plus-breadcrumb approach unless planning introduces enough new
non-Rust evidence surfaces to justify a small local manifest.

**Notes:** New Rust source/tests still require `docs/parity/source-breadcrumbs.json`.
SLSA, in-toto, signed attestations, and broad provenance systems are deferred.

---

## the agent's Discretion

- The planner can choose whether the VER-02 coverage map is embedded in the
  Phase 73 checker or stored as a small manifest.
- The executor can add no new Rust tests if existing named tests explicitly
  satisfy every VER-02 behavior.
- The executor can keep Phase 73 docs/checker focused and avoid new
  dependencies.

## Deferred Ideas

- Public-network CI or release-blocking live sync.
- Current-tip timing thresholds in default verification.
- Process-level crash harness if durable reopen coverage is sufficient.
- SLSA/in-toto/signed provenance systems.

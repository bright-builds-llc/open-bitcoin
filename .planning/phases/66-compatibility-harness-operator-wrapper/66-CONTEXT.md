---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 66-2026-06-08T21-58-25
generated_at: 2026-06-08T21:58:25.000Z
---

# Phase 66: Compatibility Harness Operator Wrapper - Context

**Gathered:** 2026-06-08
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 66 gives operators a documented wrapper around the Phase 54 public-peer compatibility transcript harness. The wrapper must produce stable local JSON and Markdown reports with peer endpoint, network, negotiated capability summary, failing step, diagnosis, transcript summary, redaction boundaries, and next-action guidance.

This phase does not add default public-network verification, inbound serving, transaction relay, compact block relay, production-funds wallet use, migration apply mode, packaging, hosted dashboards, GUI work, or a broad production-node claim.

</domain>

<decisions>

## Implementation Decisions

### Wrapper Shape

- **D-01:** Add an operator-facing `open-bitcoin compatibility harness` command rather than asking operators to call Rust test or harness internals directly.
- **D-02:** Keep the wrapper deterministic by default: it runs built-in transcript scenarios through the existing Phase 54 pure harness and labels the peer endpoint supplied by the operator. It must not open public sockets from `bash scripts/verify.sh`.
- **D-03:** Write report files under an explicit local output directory, using stable names `compatibility-harness-report.json` and `compatibility-harness-report.md`.

### Report Contract

- **D-04:** The JSON report must include peer endpoint, selected network, scenario, negotiated capabilities, failing step, diagnosis, transcript summary, redaction boundaries, and next action.
- **D-05:** The Markdown report must expose the same facts in an operator-readable form and must not include raw wire payloads, daemon stdout/stderr tails, credentials, cookie contents, wallet material, or unbounded peer logs.
- **D-06:** Stable diagnosis strings must cover `compatible`, `version_rejected`, `network_mismatch`, `service_bit_mismatch`, `unsupported_message_order`, `timeout`, `peer_disconnect`, `malformed_payload`, and `local_configuration_failure`.

### Alignment And Boundaries

- **D-07:** The wrapper must reuse `open-bitcoin-network`'s `evaluate_transcript` and `CompatibilityDiagnosis` instead of duplicating peer lifecycle rules in the CLI layer.
- **D-08:** The wrapper's "failed peer does not count as useful progress" semantics must align with daemon peer-replacement behavior and Phase 54/55 compatibility decisions.
- **D-09:** Operator docs must provide copy-pasteable repo-local Cargo and Bazel commands and clearly state that compatibility wrapper reports are opt-in local evidence outside default deterministic verification.
- **D-10:** Add a deterministic Phase 66 checker only for local source/docs/verification-boundary drift, then wire it into `bash scripts/verify.sh`.

### Claude's Discretion

The planner may choose a single-plan split if it keeps code, docs, checker, and verification cohesive. The executor may add small CLI-only DTOs for stable report serialization and Markdown rendering, but pure diagnosis remains in `open-bitcoin-network`.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 66 goal and success criteria.
- `.planning/REQUIREMENTS.md` - COMPAT-01, COMPAT-02, COMPAT-03 and v1.5 default-verification boundaries.
- `.planning/PROJECT.md` - v1.5 scope, Knots baseline, and functional-core/imperative-shell constraints.
- `.planning/STATE.md` - Prior compatibility, support, and default-verification decisions.

### Prior Compatibility Evidence

- `.planning/phases/54-peer-compatibility-baseline-and-diagnostic-harness/54-CONTEXT.md` - Phase 54 harness decisions and report semantics.
- `.planning/phases/54-peer-compatibility-baseline-and-diagnostic-harness/54-01-SUMMARY.md` - Implemented pure compatibility harness behavior and diagnosis variants.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-01-SUMMARY.md` - Daemon peer-replacement and compatibility behavior that reports must align with.
- `packages/open-bitcoin-network/src/compatibility.rs` - Existing pure compatibility transcript harness.
- `packages/open-bitcoin-network/src/lib.rs` - Existing re-exports for compatibility types.
- `docs/parity/catalog/p2p.md` - Public-peer compatibility and opt-in evidence boundaries.

### Operator CLI And Docs

- `packages/open-bitcoin-cli/src/operator.rs` - Clap command contract for `open-bitcoin`.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - Operator command routing and runtime shell.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Operator binary regression style for JSON/Markdown outputs.
- `docs/operator/runtime-guide.md` - Repo-local operator command guidance.
- `scripts/check-phase65-support-review.ts` - Current deterministic checker pattern.
- `scripts/verify.sh` - Repo-native deterministic verification contract.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `evaluate_transcript` already executes deterministic transcript events and returns `CompatibilityReport` with steps, diagnosis, useful-progress status, and next action.
- `CompatibilityDiagnosis` already enumerates all required diagnosis categories.
- `open-bitcoin` already has global `--network`, `--format`, `--datadir`, and `--no-color` options plus report-writing patterns in `support bundle`.
- `operator_binary.rs` already exercises end-to-end binary commands and validates stable JSON/Markdown output.

### Established Patterns

- Pure protocol and peer decisions belong in `open-bitcoin-network`; CLI code should be a thin reporting shell.
- Operator-facing report commands use structured JSON for automation and Markdown/human text for review.
- Public-network work remains opt-in UAT evidence and must stay outside `bash scripts/verify.sh`.
- Repo-owned deterministic drift checks use Bun scripts with exact required strings and forbidden default-verification strings.

### Integration Points

- Add a CLI module under `packages/open-bitcoin-cli/src/operator/` for compatibility report DTOs and rendering.
- Add `open-bitcoin-network` as a CLI dependency for invoking the pure harness.
- Add focused operator-binary tests for report creation, diagnosis coverage, Markdown field coverage, and redaction boundary text.
- Update runtime docs, P2P parity wording, `scripts/verify.sh`, and the tracked LOC report as needed.

</code_context>

<specifics>

## Specific Ideas

- Prefer `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- compatibility harness ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- compatibility harness ...` examples in docs.
- Let operators pick a built-in scenario such as `compatible`, `timeout`, or `service-bit-mismatch` so every required diagnosis has deterministic local evidence.
- Keep endpoint values as operator-supplied labels in reports, not as proof that a public socket was contacted.

</specifics>

<deferred>

## Deferred Ideas

- Real live public-peer probing remains opt-in UAT evidence and is not added to default verification.
- Phase 67 owns final v1.5 threat-model, release-readiness, parity-root, and deterministic release-boundary closeout.
- Production-node support, inbound serving, relay behavior, production-funds wallet use, migration apply mode, packaging, hosted dashboards, Windows service integration, and GUI work remain future scope.

</deferred>

---

*Phase: 66-compatibility-harness-operator-wrapper*
*Context gathered: 2026-06-08 via yolo discussion*

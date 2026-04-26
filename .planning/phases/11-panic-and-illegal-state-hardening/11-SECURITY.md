---
phase: 11
slug: panic-and-illegal-state-hardening
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-26
updated: 2026-04-26
---

# Phase 11 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 11 delivers panic and illegal-state hardening for first-party production
Rust code. The phase inventories production panic-like sites, replaces reachable
caller-facing crash paths with typed errors or non-panicking control flow, adds
a repo-owned panic-site guard, keeps the guard allowlist empty at close, wires
the guard into `bash scripts/verify.sh`, and documents the residual review
process in parity audit docs.

The Phase 11 plans did not include explicit `<threat_model>` blocks, and the
Phase 11 summaries did not include `## Threat Flags` sections. The declared
threat register is therefore empty. This report still verifies the
security-relevant controls implied by the panic-hardening phase artifacts.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| First-party Rust source to panic-site scanner | Production Rust code under `packages/open-bitcoin-*/src` is scanned for unclassified panic-like constructs. | Source paths, line text, and panic-like token matches. |
| Panic allowlist to scanner result | The checked-in allowlist can suppress specific findings only when a path, needle, and rationale match. | Allowlist entries and local invariant rationales. |
| Panic-site guard to repo verification | `scripts/verify.sh` invokes the guard before Cargo format, lint, build, and test checks. | Guard exit code and diagnostic output. |
| Production APIs to callers | Mempool, consensus, chainstate, wallet, RPC, CLI, codec, network, benchmark, and report paths return typed failures instead of reachable panics. | Errors, reject reasons, CLI exits, codec failures, and report/write failures. |
| Phase artifacts to reviewers | Inventory, summaries, UAT, and parity docs explain the scan scope, exclusions, fixed clusters, and residual review trigger. | Human-readable audit evidence and residual-risk notes. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| _none declared_ | _n/a_ | _n/a_ | _n/a_ | Phase 11 plans declared no explicit STRIDE threat entries. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Reviewable production panic inventory | `11-INVENTORY.md` defines the first-party production scope, excluded vendored/generated/test-only code, searched panic-like forms, initial scan size, closeout categories, addressed clusters, and empty allowlist state. | verified |
| Reproducible panic-site scanner | `scripts/check-panic-sites.sh` scans `packages/open-bitcoin-*/src/*.rs`, skips `tests.rs` and inline `#[cfg(test)]` sections, ignores comment lines, detects `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!`, and `unimplemented!`, and prints the offending path plus allowlist format on failure. | verified |
| Empty production allowlist at close | `scripts/panic-sites.allowlist` contains only format comments and states entries are allowed only for narrow, locally proven invariants or accepted tooling boundaries. | verified |
| Guard wired into repo verification | `scripts/verify.sh` runs `bash scripts/check-panic-sites.sh` before Cargo format, clippy, build, tests, benchmark smoke, Bazel smoke build, coverage, and architecture checks. | verified |
| Reachable mempool invariant crashes converted to typed errors | Mempool limit validation returns `MempoolError::InternalInvariant` for missing candidate or ancestor state instead of panicking. | verified |
| Reachable consensus witness helper crashes converted to validation errors | Coinbase, witness reserved value, and commitment-output helper paths use `let...else` and return `BlockValidationError` with consensus reject reasons. | verified |
| Reachable wallet script push crash converted to typed error | Wallet push-data encoding returns `WalletError::Script` when the push exceeds the script element limit. | verified |
| Wire command and codec conversions return typed errors | Message command construction and wire decoding return `MessageCommandError`; codec readers return `CodecError::UnexpectedEof` instead of relying on unchecked array conversion. | verified |
| CLI and benchmark adapter failures are caller-visible | CLI stdin read failures return exit code 1, and benchmark parsing/report paths return `BenchError` rather than caller-facing panics. | verified |
| Regression behavior passed UAT | `11-UAT.md` records six passed checks for inventory reviewability, clean guard pass, helpful guard failure on temporary panic, verify wiring, typed-error regression tests, and parity residual-risk docs. | verified |
| Parity docs preserve residual risk | `docs/parity/deviations-and-unknowns.md` and `docs/parity/release-readiness.md` point reviewers to Phase 11 evidence and state that future production panic-like sites must be fixed or narrowly allowlisted. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-26 | 0 | 0 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| Targeted `rg` scan for `<threat_model>`, `Threat ID`, `T-11-`, `STRIDE`, `threat_id`, and `Threat Flags` across Phase 11 artifacts. | Found no explicit Phase 11 threat model entries and no summary threat flags. |
| Targeted review of Phase 11 plans, summaries, inventory, UAT, panic guard, allowlist, verify wiring, and parity docs. | Confirmed artifact-based controls for production panic-site inventory, fixed reachable crash clusters, future guard enforcement, and residual-risk documentation. |
| Source review of representative mempool, consensus witness, wallet script push, wire command, codec reader, CLI stdin, and benchmark parser/report paths. | Confirmed representative reachable crash paths now surface typed errors, consensus reject reasons, or caller-visible failures. |
| `bash -n scripts/check-panic-sites.sh` | Passed. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `git diff --check` | Passed. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed. |
| `bash scripts/verify.sh` | Passed. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-04-26

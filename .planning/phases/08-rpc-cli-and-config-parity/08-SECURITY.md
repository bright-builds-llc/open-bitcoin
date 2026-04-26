---
phase: 08
slug: rpc-cli-and-config-parity
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-26
updated: 2026-04-26
---

# Phase 08 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 08 delivers the first RPC, CLI, and configuration parity slice: first-class
RPC and CLI packages, managed node and wallet projections, shared RPC config and
method typing, an authenticated POST-only HTTP transport, `open-bitcoin-cli`
startup and request handling, operator-flow coverage, parity docs, and gap
closures for RPC safety and CLI behavior.

The Phase 08 plans declared 29 threats across package ownership, adapter
boundaries, transport/authentication, typed RPC dispatch, CLI parsing, local
credential handling, documentation accuracy, and verification provenance. No
additional open threat flags were found in the Phase 08 summaries.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Workspace manifests to shell packages | Cargo and Bazel package definitions make the RPC and CLI crates first-class build surfaces. | Workspace members, Bazel aliases, crate-universe lockfiles, and repo verification targets. |
| Managed node and wallet internals to RPC context | RPC handlers reach node and wallet state only through narrow managed projections. | Chain, mempool, networking, wallet snapshots, balances, descriptor records, and transaction submission commands. |
| Config files and CLI flags to runtime config | User-provided config files, datadirs, and CLI flags become typed RPC runtime and client startup configuration. | `bitcoin.conf` keys, include paths, auth fields, RPC host and port fields, wallet path flags, and datadir-derived cookie paths. |
| HTTP client to RPC transport | Remote or local HTTP requests cross the transport auth and JSON-RPC envelope parser before dispatch. | HTTP method, Basic auth header, request body, JSON-RPC id, notification and batch shape, and response status. |
| RPC params to typed dispatcher | JSON-RPC params become typed request structs before reaching managed node or wallet commands. | Named and positional params, method names, rescan ranges, raw transaction hex, fee limit fields, and helper requests. |
| CLI args and stdin to request builder | Terminal input becomes CLI startup state and RPC requests. | Flags, named params, positional args, `-stdin*` input, endpoint overrides, and auth material. |
| CLI binary to operator terminal | The CLI must not block or expose misleading behavior during normal operator workflows. | Open stdin handles, stderr diagnostics, stdout rendering, exit codes, and deferred-surface errors. |
| Cookie file to local authentication | Local RPC cookie files provide single-operator authentication. | Cookie username, generated password, filesystem path, Unix file mode, and existing cookie contents. |
| Parity docs and summaries to future reviewers | Phase artifacts describe what is supported, rejected, deferred, or accepted. | Catalog entries, lifecycle summaries, review findings, UAT notes, and verification reports. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-08-01-01 | T | `packages/Cargo.toml`, `BUILD.bazel` | mitigate | `open-bitcoin-rpc` and `open-bitcoin-cli` are workspace members and have root Bazel aliases, making the shell packages first-class build surfaces. | closed |
| T-08-01-02 | D | `scripts/verify.sh` | mitigate | The repo verification smoke build includes `//:rpc` and `//:cli`, so missing shell targets fail in the native verification path. | closed |
| T-08-01-03 | T | Package manifests | mitigate | The scaffolding remains limited to shell crates and does not add deferred wallet-routing, multi-user auth, or broad RPC dependency surfaces. | closed |
| T-08-02-01 | T | `packages/open-bitcoin-node/src/network.rs` | mitigate | RPC-facing network projections stay narrow and avoid coupling handlers to raw peer-manager, mempool, or chainstate internals. | closed |
| T-08-02-02 | T | `packages/open-bitcoin-node/src/wallet.rs` | mitigate | Wallet projections and commands expose only the Phase 8 wallet surface instead of widening handlers to raw wallet internals. | closed |
| T-08-02-03 | E | `packages/open-bitcoin-rpc/src/config.rs` | mitigate | RPC config supports cookie auth or explicit `rpcuser` plus `rpcpassword`, binds locally by default, and keeps `rpcauth`, whitelist scope, and wallet-path routing deferred. | closed |
| T-08-02-04 | T | `packages/open-bitcoin-rpc/src/method.rs` | mitigate | The shared method registry and request/response structs prevent ad hoc free-form JSON handling in later RPC slices. | closed |
| T-08-03-01 | E | `packages/open-bitcoin-rpc/src/http.rs` auth gate | mitigate | The HTTP server requires POST plus Basic auth, binds to local endpoints by default, and limits supported auth to the single-operator cookie or explicit credential model. | closed |
| T-08-03-02 | D | `packages/open-bitcoin-rpc/src/http.rs` envelope parser | mitigate | Batch parsing, notification detection, and HTTP status mapping are centralized in the transport module with direct transport tests. | closed |
| T-08-03-03 | T | `packages/open-bitcoin-rpc/src/config.rs` | mitigate | Config parsing canonicalizes `conf`, `datadir`, and `includeconf`, rejects `conf=` inside config files, and keeps unsupported auth and wallet-path features out of scope. | closed |
| T-08-03-04 | T | `packages/open-bitcoin-rpc/src/dispatch.rs` | mitigate | Dispatch normalizes params through typed requests, rejects unsupported ranges and deferred methods, and routes mutations through `ManagedRpcContext`. | closed |
| T-08-04-01 | T | `packages/open-bitcoin-cli/src/args.rs`, `startup.rs` | mitigate | CLI parsing preserves named parameters, rejects duplicates or positional collisions through the shared normalizer, and preserves stdin, config, and port-precedence rules. | closed |
| T-08-04-02 | S | `packages/open-bitcoin-cli/src/startup.rs` | mitigate | CLI startup reuses shared auth resolution, prefers cookie auth when no password is configured, and resolves endpoint precedence before transport. | closed |
| T-08-04-03 | T | `packages/open-bitcoin-cli/src/getinfo.rs` | mitigate | `-getinfo` helper batching is bound to shared RPC method names and rejects extra arguments. | closed |
| T-08-05-01 | S | `packages/open-bitcoin-cli/src/client.rs` | mitigate | The CLI client builds Basic auth from shared auth config, prefers cookie auth when appropriate, and sends requests through POST-only typed transport. | closed |
| T-08-05-02 | E | `packages/open-bitcoin-cli/tests/operator_flows.rs` | mitigate | Operator-flow tests exercise the supported headless workflow and explicit failures for deferred mutating or multiwallet surfaces. | closed |
| T-08-05-03 | R | `docs/parity/catalog/rpc-cli-config.md` | mitigate | The parity catalog documents baseline-backed methods, Open Bitcoin extensions, rejected behavior, and deferred interfaces. | closed |
| T-08-06-01 | T | `packages/open-bitcoin-rpc/src/dispatch.rs::rescan_blockchain` | mitigate | Partial or out-of-bounds rescan ranges are rejected with invalid params because the current wallet seam only executes truthful full-snapshot rescans. | closed |
| T-08-06-02 | T | `packages/open-bitcoin-rpc/src/dispatch.rs::send_raw_transaction` | mitigate | Explicit `maxfeerate` and `maxburnamount` values are rejected before transaction decoding or mempool submission. | closed |
| T-08-06-03 | E | `packages/open-bitcoin-rpc/src/http.rs::read_or_create_cookie_password` | mitigate | Cookie secrets are generated with `getrandom`, predictable fallback material was removed, and new Unix cookie files are created with mode `0600`. | closed |
| T-08-06-04 | R | `.planning/phases/08-rpc-cli-and-config-parity/08-06-SUMMARY.md` | mitigate | The 08-06 summary records lifecycle provenance, proof commands, and explicit handoff for the remaining gap categories. | closed |
| T-08-07-01 | S | `packages/open-bitcoin-rpc/src/config/loader/rpc_address.rs` | mitigate | RPC config preserves hostname endpoint strings without DNS resolution at config load and keeps explicit port precedence deterministic. | closed |
| T-08-07-02 | T | `packages/open-bitcoin-cli/src/args.rs`, `packages/open-bitcoin-rpc/src/method.rs` | mitigate | CLI parsing preserves duplicate named parameters and the shared normalizer rejects duplicates before HTTP transport. | closed |
| T-08-07-03 | D | `packages/open-bitcoin-cli/src/main.rs` | mitigate | Stdin reads are gated on enabled stdin flags, with an open-pipe subprocess regression for normal terminal use. | closed |
| T-08-07-04 | R | `.planning/phases/08-rpc-cli-and-config-parity/08-07-SUMMARY.md` | mitigate | The 08-07 summary records lifecycle provenance and targeted proof commands. | closed |
| T-08-08-01 | R | `docs/parity/catalog/rpc-cli-config.md` | mitigate | RPC/CLI/config docs explicitly state supported, rejected, and deferred semantics so parity claims remain auditable. | closed |
| T-08-08-02 | D | Full verification closeout | mitigate | Phase closeout requires targeted gap tests plus Cargo formatting, clippy, build, full tests, and `bash scripts/verify.sh`. | closed |
| T-08-08-03 | R | `.planning/phases/08-rpc-cli-and-config-parity/*-SUMMARY.md` | mitigate | The gap-closure and final summaries carry lifecycle provenance and requirement completion frontmatter. | closed |
| T-08-08-04 | I | Human verification notes | accept | Human verification notes record commands, expected shapes, and permission expectations only; they do not expose cookie secrets or sensitive values. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| First-class package and build ownership | `packages/Cargo.toml` includes the RPC and CLI crates, root Bazel aliases expose `//:rpc` and `//:cli`, and `scripts/verify.sh` builds both aliases. | verified |
| Narrow managed adapter seam | `ManagedRpcContext` and node wallet or network projections keep RPC dispatch behind explicit shell-facing methods instead of raw internals. | verified |
| Typed RPC method registry and normalization | `method.rs` maps supported methods to typed request/response shapes and rejects deferred methods, ranged descriptors, positional collisions, and duplicate named params. | verified |
| Config parser and auth resolution | Config loading handles datadir, config file, include, endpoint, and auth precedence while rejecting unsupported config-in-config, ambiguous password syntax, and deferred auth modes. | verified |
| POST-only authenticated transport | `http.rs` rejects non-POST requests, enforces Basic auth before dispatch, centralizes JSON-RPC batch/notification behavior, and maps transport statuses deterministically. | verified |
| Cookie-auth hardening | New cookie files use direct random secret generation and Unix owner-only mode while preserving reads of existing cookie files. | verified |
| RPC dispatcher safety limits | `rescanblockchain` rejects unsupported ranges and `sendrawtransaction` rejects explicit fee or burn safety parameters that are not yet enforced. | verified |
| CLI startup and argument safety | CLI parsing preserves endpoint and auth precedence, rejects invalid RPC ports, rejects duplicate named args before transport, and consumes `-stdinrpcpass` before stdin arguments. | verified |
| Stdin blocking regression guard | `main.rs` reads stdin only for enabled stdin flags, and the operator-flow test proves normal CLI invocation does not wait on an open stdin pipe. | verified |
| CLI transport and output behavior | The CLI client sends typed POST requests with shared auth and surfaces JSON-RPC failures through stable stderr and exit-code behavior. | verified |
| Operator-flow and deferred-surface coverage | Integration tests cover the descriptor/rescan/balance/build/sign/send workflow, deferred surfaces, parity catalog tracking, and stdin behavior. | verified |
| Honest parity documentation | `docs/parity/catalog/rpc-cli-config.md` records supported, rejected, extended, and deferred RPC/CLI/config behavior. | verified |
| Lifecycle provenance | Phase 08 summaries record lifecycle identifiers, proof commands, requirement completion, and handoff state. | verified |

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-08-01 | T-08-08-04 | Human verification notes are documentation-only and record commands, expected result shapes, and permission expectations rather than secrets or sensitive cookie values. | Codex | 2026-04-26 |

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-26 | 29 | 29 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| Targeted `rg` scan for `<threat_model>` and `## Threat Flags` across Phase 08 plan and summary artifacts. | Found 29 declared threats, all with closed dispositions after implementation review. Summary threat flags did not add open threats beyond the declared boundaries. |
| Targeted `rg` and source review across RPC, CLI, node adapter, docs, and Phase 08 review or verification artifacts. | Controls for auth, config precedence, typed dispatch, duplicate params, unsupported rescan ranges, explicit tx safety params, stdin gating, docs, and lifecycle provenance reviewed. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features` | Passed: 26 RPC library tests, 2 black-box parity tests, binary tests, and doc-tests passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features` | Passed: 10 CLI library tests, 2 CLI binary tests, 4 operator-flow integration tests, and doc-tests passed. |
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

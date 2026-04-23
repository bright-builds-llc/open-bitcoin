# Phase 8: RPC, CLI, and Config Parity - Research

**Researched:** 2026-04-22
**Domain:** Operator-facing RPC, CLI, and Bitcoin-style config surfaces over the existing managed node and wallet adapters. [VERIFIED: repo grep]
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

This section is copied verbatim from `08-CONTEXT.md`. [VERIFIED: repo grep]

### Locked Decisions

### Interface boundary and ownership
- **D-01:** Keep RPC, CLI, and config parsing in adapter-owned shell crates or
  modules layered over the existing managed node and wallet facades in
  `open-bitcoin-node`; do not leak transport, process, or config concerns into
  the pure-core crates.
- **D-02:** Model supported RPC methods with typed request/response and error
  mapping instead of free-form JSON plumbing spread across handlers, so the
  parity surface stays auditable and easier to extend in later phases.

### Supported RPC surface
- **D-03:** Limit the initial RPC slice to methods that the existing managed
  node and wallet facades can back honestly: node/chainstate/mempool/network
  info, raw transaction submission over the managed mempool path, wallet info,
  descriptor import, rescan against chainstate snapshots, address derivation,
  balance/UTXO inspection, and deterministic transaction build/sign flows.
- **D-04:** Keep unsupported or not-yet-owned baseline RPC areas explicitly out
  of scope for this phase, including mining admin surfaces, external signer
  RPCs, multiwallet persistence semantics beyond the supported adapter-owned
  slice, index-dependent RPCs, and any baseline behavior that requires runtime
  facilities the repo does not yet own.

### CLI and config surface
- **D-05:** Expose the Phase 8 shell through baseline-shaped operator tools: a
  node/server entrypoint plus a client-style CLI for RPC access, rather than a
  single app-specific subcommand tree that hides the baseline mental model.
- **D-06:** Config-file parsing and option precedence must follow the supported
  baseline rules: explicit CLI flags override config-file values; explicit
  config-file location and data-directory handling follow the Knots
  `feature_config_args.py` expectations for the supported slice; config is
  parsed at the shell boundary and converted into typed runtime config before
  reaching domain code.
- **D-07:** The AI-agent-friendly CLI todo is folded into this phase rather than
  treated as a separate capability: every important CLI command in scope should
  have deterministic non-interactive behavior, stable machine-readable output
  where it materially helps automation, explicit exit codes, and actionable
  error output instead of human-only prose.

### Verification and operator flows
- **D-08:** End-to-end tests should prove headless operator workflows through
  CLI and RPC only, using hermetic in-memory or repo-owned local runtime
  fixtures rather than external services.
- **D-09:** Phase 8 summaries and parity tracking should state the supported RPC
  and CLI/config surface explicitly, so unsupported baseline methods are listed
  as deferred rather than silently omitted.

### the agent's Discretion
- Exact crate/module names for the RPC server, CLI client, and config parser are
  at the agent's discretion as long as the pure-core / imperative-shell
  boundary stays intact.
- The specific supported RPC and CLI method list can stay narrow if it is
  justified by the current managed node and wallet capabilities and is captured
  explicitly in parity docs and plan artifacts.

### Deferred Ideas (OUT OF SCOPE)
- Full Knots RPC coverage beyond the supported headless slice
- Mining admin/control RPCs and advanced index-dependent methods
- External signer RPCs, PSBT orchestration, and richer wallet admin flows
- Multiwallet persistence semantics broader than the adapter-owned slice
- GUI surfaces
- Phase 9 black-box parity harnesses and process-isolation work
- Phase 10 benchmarks and audit-readiness reporting
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RPC-01 | In-scope RPC methods, result payloads, and error semantics match the pinned baseline. [VERIFIED: repo grep] | `Architecture Patterns` defines a typed method registry and one shared error mapper; `Common Pitfalls` captures JSON-RPC 1.1 vs 2.0 HTTP semantics, batch behavior, and named-argument rules from upstream sources. [VERIFIED: repo grep] |
| CLI-01 | In-scope CLI commands, config-file parsing, and option precedence match the pinned baseline. [VERIFIED: repo grep] | `Standard Stack` recommends `clap` for CLI parsing but a repo-owned Bitcoin config parser for `bitcoin.conf`; `Common Pitfalls` captures `-rpcconnect`/`-rpcport` precedence, `-stdinrpcpass`, `-rpcwait`, and `feature_config_args.py` edge cases. [VERIFIED: repo grep][CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html] |
| CLI-02 | Operators can run the node and wallet headlessly through CLI and RPC surfaces only. [VERIFIED: repo grep] | `Architecture Patterns` keeps CLI and RPC on one typed command path over `Managed*` facades, and `Open Questions` calls out the only wallet-send surface that may need explicit scoping. [VERIFIED: repo grep][ASSUMED] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Keep transport, process, config, filesystem, and auth concerns in shell-owned adapters and keep pure-core crates free of direct I/O and runtime side effects. [VERIFIED: repo grep]
- Match Bitcoin Knots `29.3.knots20260210` on externally visible in-scope behavior and record intentional differences in `docs/parity/index.json` plus companion docs. [VERIFIED: repo grep]
- Keep dependencies minimal and security-conscious, and do not introduce third-party Rust Bitcoin libraries into the production path. [VERIFIED: repo grep]
- Use `rust-toolchain.toml` as the Rust version source of truth; the pinned toolchain is `1.94.1`. [VERIFIED: repo grep]
- Use Bazel/Bzlmod plus the Cargo workspace together; new packages need Cargo workspace wiring and Bazel targets. [VERIFIED: repo grep]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [VERIFIED: repo grep]
- Rust modules should favor `foo.rs` plus `foo/`, early returns, `let ... else`, and `maybe_` naming for internal optional values. [VERIFIED: repo grep]
- Unit tests for pure and business logic should stay focused, headless, and clearly structured as Arrange/Act/Assert. [VERIFIED: repo grep]

## Summary

Phase 8 should be planned as a new shell-only interface layer on top of the already-exported `ManagedChainstate`, `ManagedMempool`, `ManagedPeerNetwork`, and `ManagedWallet` adapters, because those are the only current seams that already bridge pure-core behavior into mutable runtime state. [VERIFIED: repo grep]

The hard part is not HTTP routing by itself; it is reproducing Knots's operator contract across five separate areas at once: JSON-RPC request parsing, JSON-RPC 1.1 versus 2.0 error and HTTP status behavior, CLI argument conversion and helper flags, Bitcoin-style config precedence, and the honest scoping of wallet methods that the current descriptor wallet can actually back. [VERIFIED: repo grep]

Open Bitcoin currently has no first-party RPC server, no client CLI, no config parser, and no interface parity catalog entry beyond `rpc` and `cli-config` being marked `planned`, so the phase plan should budget for new workspace packages or modules, Bazel wiring, parity-doc updates, and test fixtures rather than treating this as an incremental patch to an existing interface layer. [VERIFIED: repo grep]

**Primary recommendation:** Build one shared typed command layer over `open-bitcoin-node`, expose it through an `axum`/`tokio` RPC server and a `clap`/`ureq` client CLI, and keep Bitcoin config parsing repo-owned and test-led by upstream `feature_config_args.py` cases. [CITED: https://docs.rs/axum/latest/axum/][CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html][CITED: https://docs.rs/ureq/latest/ureq/][VERIFIED: repo grep][ASSUMED]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `open-bitcoin-node` | `0.1.0` | Existing managed chainstate, mempool, network, and wallet facades. | It already exports the shell-owned runtime seams Phase 8 is required to build on instead of bypassing. [VERIFIED: repo grep] |
| `serde` | `1.0.228` | Typed request/response serialization for RPC envelopes and CLI JSON output. | Strong typing is the cleanest way to satisfy D-02 without free-form JSON plumbing. [VERIFIED: crates.io API][CITED: https://docs.rs/serde_json/latest/serde_json/] |
| `serde_json` | `1.0.149` | Parse JSON-RPC bodies and render JSON responses or CLI output. | The official docs cover typed, untyped, `from_slice`, and `from_reader` flows that fit HTTP request handling and machine-readable CLI output. [VERIFIED: crates.io API][CITED: https://docs.rs/serde_json/latest/serde_json/] |
| `clap` | `4.6.1` | Baseline-shaped CLI flag parsing, validation, help, and subcommand modeling. | The derive tutorial shows stable struct-driven parsing, defaults, validation, and subcommands without hand-written `argv` parsing. [VERIFIED: crates.io API][CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html] |
| `axum` | `0.8.9` | Maintained HTTP routing and request extraction for the RPC server transport. | The official docs make clear that `axum` is built for routing and request handling and is designed to work with `tokio` and `hyper`. [VERIFIED: crates.io API][CITED: https://docs.rs/axum/latest/axum/] |
| `tokio` | `1.52.1` | Runtime required by `axum` for the RPC server binary only. | `axum` explicitly targets `tokio` and `hyper`, so the async runtime should stay isolated to the transport crate or binary. [VERIFIED: crates.io API][CITED: https://docs.rs/axum/latest/axum/] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `ureq` | `3.3.0` | Blocking HTTP client for the `bitcoin-cli`-style binary. | The official docs describe it as a simple, safe blocking HTTP client with JSON, proxies, HTTPS, and minimal dependencies, which fits a thin CLI wrapper well. [VERIFIED: crates.io API][CITED: https://docs.rs/ureq/latest/ureq/] |
| `base64` | `0.22.1` | Decode and encode HTTP Basic auth values. | Upstream RPC auth is HTTP Basic auth over POST requests; use the standard base64 engine instead of a custom codec. [VERIFIED: repo grep][VERIFIED: crates.io API][CITED: https://docs.rs/base64/latest/base64/] |
| `percent-encoding` | `2.3.2` | Encode wallet names for `/wallet/<name>` endpoint paths if wallet scoping is supported. | Upstream `-rpcwallet` changes the RPC endpoint path, so path-segment encoding should use a library helper if this slice supports wallet selection. [VERIFIED: repo grep][VERIFIED: crates.io API][ASSUMED] |
| `repo-owned config parser` | `n/a` | Parse `bitcoin.conf`, `includeconf`, section scopes, and precedence into typed runtime config. | The upstream config rules are Bitcoin-specific and are not safely representable as generic INI or TOML parsing. [VERIFIED: repo grep] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `axum` + `tokio` | `hyper` directly | `hyper` is current and well-supported, but it leaves more low-level request extraction and response wiring in Phase 8 code. [VERIFIED: crates.io API][CITED: https://docs.rs/axum/latest/axum/] |
| `axum` + `tokio` | `tiny_http` | `tiny_http` keeps a blocking model but its crates.io metadata is much older (`0.12.0`, updated 2022-10-06), so it is a weaker maintenance signal for a new transport layer. [VERIFIED: crates.io API] |
| `clap` | hand-rolled `argv` parsing | This would repeat the same class of duplicate-flag, mutual-exclusion, and help-text bugs that upstream CLI tests already exercise. [VERIFIED: repo grep][CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html] |
| repo-owned Bitcoin config parser | generic INI parser crate | Generic parsers do not model `conf=` rejection in config files, `includeconf` recursion warnings, datadir relocation, or ignored-conf errors. [VERIFIED: repo grep] |

**Installation:** The exact manifest paths depend on final package naming; if Phase 8 introduces `packages/open-bitcoin-rpc` and `packages/open-bitcoin-cli`, use the following adds. [ASSUMED]

```bash
cargo add --manifest-path packages/open-bitcoin-rpc/Cargo.toml serde serde_json base64 percent-encoding axum
cargo add --manifest-path packages/open-bitcoin-rpc/Cargo.toml tokio --features macros,rt-multi-thread,net
cargo add --manifest-path packages/open-bitcoin-cli/Cargo.toml clap --features derive
cargo add --manifest-path packages/open-bitcoin-cli/Cargo.toml ureq serde serde_json percent-encoding
```

**Version verification:** Current registry metadata as of 2026-04-22 is `clap 4.6.1` (`2026-04-15T18:59:05Z`), `serde 1.0.228` (`2025-09-27T16:51:35Z`), `serde_json 1.0.149` (`2026-01-06T16:23:34Z`), `base64 0.22.1` (`2024-04-30T23:16:40Z`), `ureq 3.3.0` (`2026-03-21T12:35:05Z`), `axum 0.8.9` (`2026-04-14T07:55:20Z`), `tokio 1.52.1` (`2026-04-16T21:29:03Z`), and `percent-encoding 2.3.2` (`2025-08-21T08:46:50Z`). [VERIFIED: crates.io API]

## Architecture Patterns

### Recommended Project Structure

```text
packages/
├── open-bitcoin-node/          # Existing managed facades; stays the shell/domain bridge
├── open-bitcoin-rpc/           # Typed RPC methods, dispatch, auth, and server transport
│   ├── src/lib.rs
│   ├── src/dispatch.rs
│   ├── src/error.rs
│   ├── src/config.rs
│   ├── src/methods/
│   └── src/bin/open-bitcoind.rs
└── open-bitcoin-cli/           # Client-style CLI wrapper over the same typed method layer
    ├── src/main.rs
    ├── src/args.rs
    ├── src/client.rs
    └── src/getinfo.rs
```

This split keeps the current managed adapters untouched, isolates async transport to the RPC package, and gives the CLI a thin client surface instead of duplicating business logic. [VERIFIED: repo grep][ASSUMED]

### Pattern 1: Shared Typed Method Registry

**What:** Define one method-spec layer that owns method names, typed params, typed results, and RPC error-code mapping, then call it from both HTTP handlers and the CLI. [VERIFIED: repo grep][ASSUMED]

**When to use:** Every supported RPC and every CLI command that maps to an RPC method. [VERIFIED: repo grep]

**Example:**

```rust
// Source pattern: packages/bitcoin-knots/src/rpc/server.cpp
// + packages/open-bitcoin-node/src/{chainstate,mempool,network,wallet}.rs
enum RpcMethod {
    GetBlockchainInfo,
    GetMempoolInfo(GetMempoolInfoRequest),
    GetNetworkInfo,
    SendRawTransaction(SendRawTransactionRequest),
    ImportDescriptors(ImportDescriptorsRequest),
}

fn dispatch<S, W>(
    method: RpcMethod,
    managed_node: &mut ManagedNodeContext<S, W>,
) -> Result<serde_json::Value, RpcError> {
    match method {
        RpcMethod::GetBlockchainInfo => Ok(serde_json::to_value(blockchain_info(managed_node)?)?),
        RpcMethod::GetMempoolInfo(request) => Ok(serde_json::to_value(mempool_info(managed_node, request)?)?),
        RpcMethod::GetNetworkInfo => Ok(serde_json::to_value(network_info(managed_node)?)?),
        RpcMethod::SendRawTransaction(request) => Ok(serde_json::to_value(send_raw_transaction(managed_node, request)?)?),
        RpcMethod::ImportDescriptors(request) => Ok(serde_json::to_value(import_descriptors(managed_node, request)?)?),
    }
}
```

### Pattern 2: Parse Config Once, Then Freeze Runtime Config

**What:** Parse `bitcoin.conf`, `includeconf`, chain sections, `-conf`, `-datadir`, and CLI overrides into typed runtime structs before server startup or CLI RPC execution. [VERIFIED: repo grep]

**When to use:** Node startup, CLI startup, and any helper command like `-getinfo` that must discover auth, datadir, or RPC endpoint settings. [VERIFIED: repo grep]

**Example:**

```rust
// Source pattern: packages/bitcoin-knots/src/common/config.cpp
struct RuntimeConfig {
    chain: Chain,
    data_dir: PathBuf,
    rpc: RpcServerConfig,
    wallet: WalletRuntimeConfig,
}

fn load_runtime_config(cli: CliArgs) -> Result<RuntimeConfig, ConfigError> {
    let file_settings = parse_bitcoin_conf(cli.maybe_conf_path.as_deref(), cli.maybe_data_dir.as_deref())?;
    let merged = merge_with_cli_precedence(file_settings, cli)?;
    RuntimeConfig::try_from(merged)
}
```

### Pattern 3: CLI Helpers Aggregate RPCs, But Normal Commands Stay RPC-Shaped

**What:** Keep the default CLI path as `bitcoin-cli`-style RPC invocation with method-specific argument conversion, and treat `-getinfo` as a thin aggregator over `getnetworkinfo`, `getblockchaininfo`, `getwalletinfo`, and `getbalances`. [VERIFIED: repo grep]

**When to use:** For the main CLI binary and for any helper command that exists only to reduce repeated operator steps. [VERIFIED: repo grep]

**Example:**

```rust
// Source pattern: packages/bitcoin-knots/src/bitcoin-cli.cpp
fn build_getinfo_batch() -> Vec<JsonRpcEnvelope<serde_json::Value>> {
    vec![
        request("getnetworkinfo", serde_json::Value::Null, 0),
        request("getblockchaininfo", serde_json::Value::Null, 1),
        request("getwalletinfo", serde_json::Value::Null, 2),
        request("getbalances", serde_json::Value::Null, 3),
    ]
}
```

### Anti-Patterns to Avoid

- **Free-form JSON handlers everywhere:** This violates D-02 and makes RPC error codes, CLI output, and method scoping drift apart. Use one typed command layer plus one error mapper. [VERIFIED: repo grep]
- **Reaching through `Managed*` facades into pure-core internals from transport code:** The current project boundary is the managed shell layer, not the pure-core crates. [VERIFIED: repo grep]
- **Treating `bitcoin.conf` like generic INI:** Upstream behavior includes `includeconf`, chain sections, datadir relocation, and ignored-conf failures that generic parsers will miss. [VERIFIED: repo grep]
- **Shipping `-netinfo` before `getpeerinfo`:** Upstream `-netinfo` is a human dashboard built from both `getpeerinfo` and `getnetworkinfo`; current Open Bitcoin only has enough state for `getnetworkinfo`. [VERIFIED: repo grep]
- **Promising HD keypool or multiwallet semantics:** The current wallet slice is descriptor import, address derivation, rescan, balance, UTXO inspection, and deterministic build/sign; it does not own keypool or broad multiwallet persistence yet. [VERIFIED: repo grep]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| CLI argument parsing | custom `std::env::args()` scanner | `clap` derive parser | Required flags, mutually exclusive switches, generated help, and typed parsing are already solved. [CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html] |
| JSON parsing and rendering | manual string slicing or bespoke JSON AST | `serde` + `serde_json` | This keeps request/response models typed and makes machine-readable CLI output cheap. [CITED: https://docs.rs/serde_json/latest/serde_json/] |
| HTTP client behavior in the CLI | raw `TcpStream` and handwritten HTTP | `ureq::Agent` | The official docs call out blocking I/O, connection reuse, JSON support, and minimal dependencies, which match a thin client binary. [CITED: https://docs.rs/ureq/latest/ureq/] |
| Basic auth base64 encoding | custom base64 codec | `base64` standard engine | Upstream RPC auth is HTTP Basic auth, so the only safe reason to touch base64 is through a library helper. [VERIFIED: repo grep][CITED: https://docs.rs/base64/latest/base64/] |
| Wallet-name path encoding | hand-built `%` escaping | `percent-encoding` | `/wallet/<name>` path handling is easy to get wrong if wallet selection lands in scope. [VERIFIED: repo grep][ASSUMED] |

**Key insight:** HTTP, JSON, CLI parsing, and encoding should come from libraries, but Bitcoin config parsing should stay repo-owned because the baseline semantics are domain-specific and are already specified by upstream source plus tests. [VERIFIED: repo grep]

## Common Pitfalls

### Pitfall 1: Config Parser Looks Simpler Than It Is

**What goes wrong:** A generic parser accepts settings that upstream rejects or misses side effects like config-file-driven datadir relocation. [VERIFIED: repo grep]
**Why it happens:** `feature_config_args.py` covers `conf=` being illegal inside config files, leading `-` being illegal in config lines, `#` being ambiguous in `rpcpassword`, `includeconf` recursion warnings, ignored-conf errors, and CLI-over-conf precedence for `-datadir`. [VERIFIED: repo grep]
**How to avoid:** Write a targeted Bitcoin config parser and port the relevant upstream functional cases directly into Phase 8 tests. [VERIFIED: repo grep][ASSUMED]
**Warning signs:** Tests pass for happy-path config loading but do not assert startup failures for invalid `conf`, ignored-conf, or non-existent datadir cases. [VERIFIED: repo grep][ASSUMED]

### Pitfall 2: JSON-RPC HTTP Behavior Depends on the Request Version

**What goes wrong:** The server returns the wrong HTTP status or emits responses to notifications when upstream would return `204 No Content`. [VERIFIED: repo grep]
**Why it happens:** Knots treats legacy 1.0/1.1 failures as HTTP errors, but JSON-RPC 2.0 errors return `200 OK` with an RPC error object; notifications still execute but return `204`, and mixed-version batches are accepted for compatibility. [VERIFIED: repo grep]
**How to avoid:** Keep envelope parsing, notification detection, and HTTP-status mapping in one transport module with direct test coverage from `interface_rpc.py`. [VERIFIED: repo grep][ASSUMED]
**Warning signs:** A single "RPC error means HTTP 500" rule appears anywhere in the server code or tests only cover JSON-RPC 2.0 happy paths. [VERIFIED: repo grep][ASSUMED]

### Pitfall 3: Named Arguments Are Not Just a JSON Object

**What goes wrong:** CLI named arguments and JSON named parameters accept duplicates or collide silently with positional args, producing payloads that differ from Knots. [VERIFIED: repo grep]
**Why it happens:** Upstream transforms named args into positional arrays using method-specific argument metadata, rejects duplicate keys, and errors when a parameter is provided both positionally and by name. [VERIFIED: repo grep]
**How to avoid:** Centralize argument conversion tables and reuse them for both `-named` CLI input and HTTP named-parameter requests. [VERIFIED: repo grep][ASSUMED]
**Warning signs:** Method handlers see raw `serde_json::Map<String, Value>` objects directly or duplicate parameters are only caught by ad hoc handler logic. [VERIFIED: repo grep][ASSUMED]

### Pitfall 4: CLI Connection and Auth Precedence Is Easy To Get Wrong

**What goes wrong:** The CLI talks to the wrong port, fails to fall back to cookie auth, or handles `-stdinrpcpass` and `-stdin` in the wrong order. [VERIFIED: repo grep]
**Why it happens:** Upstream resolves the port in this order: `-rpcport`, then the port embedded in `-rpcconnect`, then the default chain port; it also prefers cookie auth when `-rpcpassword` is empty and lets `-stdinrpcpass` consume the first stdin line before `-stdin` appends more args. [VERIFIED: repo grep]
**How to avoid:** Mirror `bitcoin-cli.cpp` request setup rules and reuse the upstream `interface_bitcoin_cli.py` cases as direct acceptance tests. [VERIFIED: repo grep][ASSUMED]
**Warning signs:** The CLI parses `-rpcconnect` and `-rpcport` independently in different modules, or auth lookup and stdin handling are tested only through manual runs. [VERIFIED: repo grep][ASSUMED]

### Pitfall 5: Current Wallet Capability Is Narrower Than Full Wallet RPC Parity

**What goes wrong:** The phase plan commits to `getnewaddress`, `getrawchangeaddress`, or broader wallet-admin semantics that require HD keypool, key reservation, or multiwallet runtime behavior the current wallet does not own. [VERIFIED: repo grep]
**Why it happens:** The current wallet core exposes descriptor import, address derivation from fixed descriptors, rescan, balances, UTXOs, and deterministic build/sign, but the vendored upstream wallet RPC surface assumes richer runtime wallet state. [VERIFIED: repo grep]
**How to avoid:** Scope wallet RPCs to descriptor import, derivation, balance/UTXO inspection, rescan, and raw transaction submission first, and gate any broader wallet-send or keypool-compatible RPC on an explicit parity review. [VERIFIED: repo grep][ASSUMED]
**Warning signs:** Planning documents list `getnewaddress` or broad multiwallet flows without a corresponding design note explaining how current descriptors become new addresses over time. [VERIFIED: repo grep][ASSUMED]

### Pitfall 6: New Interface Packages Must Satisfy Cargo and Bazel Together

**What goes wrong:** The Rust workspace builds locally but Bazel aliases or `crate_universe` targets are missing, so repo-native verification fails late. [VERIFIED: repo grep]
**Why it happens:** The workspace is driven by `packages/Cargo.toml` and `packages/Cargo.lock`, while Bazel also needs `BUILD.bazel` targets and root aliases for first-party packages. [VERIFIED: repo grep]
**How to avoid:** Plan interface packages as first-class workspace members from day one and budget explicit tasks for Cargo manifests, `Cargo.lock`, package `BUILD.bazel` files, and any top-level aliases that should expose the new crates. [VERIFIED: repo grep][ASSUMED]
**Warning signs:** A plan adds a crate name in prose but never mentions `packages/Cargo.toml`, `packages/Cargo.lock`, or a new `BUILD.bazel`. [VERIFIED: repo grep][ASSUMED]

## Code Examples

Verified patterns from local and official sources:

### Typed RPC Envelope Parsing

```rust
// Source pattern:
// - packages/bitcoin-knots/src/rpc/request.cpp
// - https://docs.rs/serde_json/latest/serde_json/
#[derive(serde::Deserialize)]
struct JsonRpcEnvelope<T> {
    #[serde(default)]
    jsonrpc: Option<String>,
    #[serde(default)]
    id: Option<serde_json::Value>,
    method: String,
    #[serde(default)]
    params: T,
}
```

### Thin Wallet Mapping Over Managed Adapters

```rust
// Source pattern:
// - packages/open-bitcoin-node/src/wallet.rs
// - packages/bitcoin-knots/src/wallet/rpc/coins.cpp
fn get_balances<S: WalletStore>(
    wallet: &ManagedWallet<S>,
    coinbase_maturity: u32,
) -> Result<GetBalancesResponse, RpcError> {
    let balance = wallet.wallet().balance(coinbase_maturity)?;
    Ok(GetBalancesResponse::from(balance))
}
```

### CLI `-getinfo` As a Batch Over Real RPC Methods

```rust
// Source pattern:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
fn getinfo_batch() -> [(&'static str, u64); 4] {
    [
        ("getnetworkinfo", 0),
        ("getblockchaininfo", 1),
        ("getwalletinfo", 2),
        ("getbalances", 3),
    ]
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Blocking micro HTTP crates with weaker maintenance signals | `axum 0.8.9` on `tokio 1.52.1` for the server transport | Crates.io metadata current as of 2026-04-14 and 2026-04-16. [VERIFIED: crates.io API] | Adds an async runtime only at the shell edge while keeping the managed node and wallet APIs synchronous. [CITED: https://docs.rs/axum/latest/axum/][ASSUMED] |
| Free-form JSON blobs as handler inputs | `serde`/`serde_json` typed structs with `Value` only at the outer envelope | `serde_json 1.0.149` current as of 2026-01-06. [VERIFIED: crates.io API] | Makes payload parity and error mapping easier to audit and share between CLI and RPC. [CITED: https://docs.rs/serde_json/latest/serde_json/] |
| Ad hoc CLI parsing | derive-based `clap 4.6.1` parser definitions | `clap 4.6.1` current as of 2026-04-15. [VERIFIED: crates.io API] | Keeps help text, validation, defaults, and mutual exclusion out of interface business logic. [CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html] |

**Deprecated/outdated:**

- `tiny_http` as the default server recommendation for new work here: the latest crates.io metadata is `0.12.0` with an older `updated_at` timestamp than the maintained async stack, so it is a poor default unless Phase 8 explicitly rejects async transport. [VERIFIED: crates.io API]
- Treating `-netinfo` as a stable automation surface: upstream documents it as a human-readable dashboard that changes regularly and depends on `getpeerinfo` plus `getnetworkinfo`. [VERIFIED: repo grep]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `axum` + `tokio` for the RPC server and `ureq` for the CLI is the best-maintained fit for this repo's minimal-dependency shell layer. | `Standard Stack` | Rework of the transport package and its tests if the project prefers a blocking server stack instead. |
| A2 | New interface work should land as `open-bitcoin-rpc` and `open-bitcoin-cli` packages rather than only new modules under `open-bitcoin-node`. | `Architecture Patterns` | Extra workspace churn if the planner later chooses module-only integration. |
| A3 | The honest wallet-mutating Phase 8 slice may need to stop at raw transaction build/sign plus `sendrawtransaction`, instead of claiming full `sendtoaddress` parity immediately. | `Common Pitfalls`, `Open Questions` | Operator-flow planning could either over-scope Phase 8 or under-scope a feasible send path. |
| A4 | `percent-encoding` should be added only if `/wallet/<name>` path scoping lands inside the supported slice. | `Standard Stack`, `Don't Hand-Roll` | One small dependency may be unnecessary if Phase 8 stays single-wallet only. |

## Open Questions (RESOLVED)

1. **Should Phase 8 expose `sendtoaddress`, or keep wallet send flows as CLI-local build/sign plus `sendrawtransaction`? (RESOLVED)**
   - Resolution: Keep the initial send surface at deterministic `buildtransaction` / `buildandsigntransaction` plus `sendrawtransaction`. Do not require `sendtoaddress` parity in Phase 8 because the current wallet slice does not yet own the broader fee-estimation, wallet-policy, and comment/label semantics that Knots exposes there. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md; VERIFIED: repo grep]
   - Planning impact: Plans should treat `sendtoaddress` as an explicitly deferred surface in docs and tests, while proving the end-to-end headless send flow through build/sign plus raw submission. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md]

2. **Does the supported auth/config slice need `rpcauth` and RPC whitelists now, or only single-operator cookie and explicit user/password auth? (RESOLVED)**
   - Resolution: Limit Phase 8 to the honest local-operator auth slice: cookie-file auth plus explicit `rpcuser` / `rpcpassword` handling and the related connection/config precedence. Do not include `rpcauth`, method whitelists, or broader multi-user policy in this phase. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md; VERIFIED: repo grep]
   - Planning impact: Config parsing and CLI/client behavior must cover the supported local auth paths explicitly, and parity docs must mark `rpcauth` and whitelist controls as deferred rather than silently omitting them. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md]

3. **Should `/wallet/<name>` routing exist at all in the initial slice? (RESOLVED)**
   - Resolution: No. Keep Phase 8 on a single active wallet slice and defer `/wallet/<name>` routing with the broader multiwallet persistence semantics that the phase context already leaves out of scope. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md; VERIFIED: repo grep]
   - Planning impact: CLI and RPC planning should reject or explicitly defer `-rpcwallet` and path-scoped multiwallet behavior, and should not add wallet-path encoding or routing dependencies in this phase. [VERIFIED: .planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` | Rust build, test, and dependency management | ✓ | `1.94.1` | — |
| `rustc` | Rust compilation | ✓ | `1.94.1` | — |
| `clippy` | Repo verification contract | ✓ | `0.1.94` | — |
| `rustfmt` | Repo verification contract | ✓ | `1.8.0-stable` | — |
| `bazel` | `bash scripts/verify.sh` smoke build | ✓ | `8.6.0` | no full repo-native fallback |
| `cargo-llvm-cov` | `bash scripts/verify.sh` coverage gate | ✓ | `0.8.5` | partial manual cargo checks only |
| `node` | `bash scripts/verify.sh` timing and GSD tooling | ✓ | `v24.13.0` | no repo-native fallback |
| `npm` | Node runtime support | ✓ | `11.6.2` | — |
| `git` | source control and some repo scripts | ✓ | `2.53.0` | — |
| `rg` | fast repo search during implementation and review | ✓ | `15.1.0` | `grep` |

All availability and version values in the table above came from local command probes captured on 2026-04-22. [VERIFIED: local command output]

**Missing dependencies with no fallback:**

- None. [VERIFIED: local command output]

**Missing dependencies with fallback:**

- None. [VERIFIED: local command output]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes | Keep RPC POST-only, require auth on non-public endpoints, and match upstream-style Basic auth plus cookie or explicit credentials for the supported slice. [VERIFIED: repo grep][ASSUMED] |
| V3 Session Management | no | The upstream RPC surface shown here is request-authenticated and does not use server sessions or cookies for session state. [VERIFIED: repo grep] |
| V4 Access Control | yes | Keep method registration explicit, gate wallet-scoped methods through the dispatcher, and only widen auth scope when the supported surface requires it. [VERIFIED: repo grep][ASSUMED] |
| V5 Input Validation | yes | Parse JSON into typed request structs and then parse hex, amounts, descriptors, heights, and booleans into domain types at the boundary. [VERIFIED: repo grep][CITED: https://docs.rs/serde_json/latest/serde_json/] |
| V6 Cryptography | yes | Do not hand-roll password hashing, HMAC, or base64 helpers; use existing crypto or encoding libraries when auth parity needs them. [VERIFIED: repo grep][CITED: https://docs.rs/base64/latest/base64/][ASSUMED] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Unauthenticated or weakly authenticated RPC access | Elevation of Privilege | Bind locally by default, require auth, and do not expose mutating methods without the supported auth layer in place. [VERIFIED: repo grep][ASSUMED] |
| Parameter smuggling through mixed named and positional args | Tampering | Normalize parameters in one place and reject duplicates or unknown names exactly once before handler execution. [VERIFIED: repo grep] |
| Malformed JSON or notification handling bugs | Denial of Service | Centralize envelope parsing, enforce POST-only requests, and mirror upstream `200`/`204`/legacy error semantics through dedicated tests. [VERIFIED: repo grep] |
| Path confusion through `-conf`, `-datadir`, or `/wallet/<name>` | Tampering | Canonicalize config paths, keep datadir handling at startup only, and encode wallet path segments if they are supported. [VERIFIED: repo grep][ASSUMED] |
| Descriptor, hex, or amount parsing bugs leaking into business logic | Tampering | Parse boundary inputs into typed domain values before calling managed facades. [VERIFIED: repo grep] |

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local workflow, verification, parity-doc, toolchain, and dependency constraints. [VERIFIED: repo grep]
- `AGENTS.bright-builds.md` and canonical Bright Builds standards pages under `../coding-and-architecture-requirements/standards/` - architecture, code-shape, verification, and testing rules. [VERIFIED: repo grep]
- `.planning/phases/08-rpc-cli-and-config-parity/08-CONTEXT.md` - locked decisions and out-of-scope items for Phase 8. [VERIFIED: repo grep]
- `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md` - phase requirements and current project position. [VERIFIED: repo grep]
- `packages/open-bitcoin-node/src/{lib,chainstate,mempool,network,wallet}.rs` - current managed adapter surface. [VERIFIED: repo grep]
- `packages/open-bitcoin-wallet/src/{lib,address,descriptor,wallet,error}.rs`, `packages/open-bitcoin-mempool/src/{pool,types,error}.rs`, `packages/open-bitcoin-chainstate/src/{lib,types}.rs`, `packages/open-bitcoin-network/src/{lib,message,peer}.rs` - current interface-capable domain data and runtime state. [VERIFIED: repo grep]
- `packages/bitcoin-knots/src/bitcoin-cli.cpp` - CLI flags, request building, auth precedence, `-getinfo`, `-rpcwait`, `-named`, and port precedence. [VERIFIED: repo grep]
- `packages/bitcoin-knots/src/rpc/{server.cpp,request.cpp,client.cpp,blockchain.cpp,mempool.cpp,net.cpp,output_script.cpp}` - request parsing, named arg transformation, and core RPC contracts. [VERIFIED: repo grep]
- `packages/bitcoin-knots/src/wallet/rpc/{wallet.cpp,coins.cpp,addresses.cpp,transactions.cpp,backup.cpp,spend.cpp}` - wallet RPC result shapes and argument surfaces. [VERIFIED: repo grep]
- `packages/bitcoin-knots/src/common/config.cpp` - config-file parsing, `includeconf`, `-conf`, and datadir handling. [VERIFIED: repo grep]
- `packages/bitcoin-knots/test/functional/{interface_rpc.py,interface_bitcoin_cli.py,feature_config_args.py,rpc_blockchain.py,rpc_net.py,rpc_getgeneralinfo.py,rpc_deriveaddresses.py}` - executable contract tests for the supported interface slice. [VERIFIED: repo grep]
- `https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html` - current `clap` derive API usage. [CITED: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html]
- `https://docs.rs/serde_json/latest/serde_json/` - current `serde_json` typed and untyped parsing model. [CITED: https://docs.rs/serde_json/latest/serde_json/]
- `https://docs.rs/ureq/latest/ureq/` - current blocking HTTP client model. [CITED: https://docs.rs/ureq/latest/ureq/]
- `https://docs.rs/axum/latest/axum/` - current HTTP server stack and runtime expectations. [CITED: https://docs.rs/axum/latest/axum/]
- `https://docs.rs/base64/latest/base64/` - current base64 engine usage. [CITED: https://docs.rs/base64/latest/base64/]
- Crates.io API endpoints for `clap`, `serde`, `serde_json`, `base64`, `ureq`, `axum`, `tokio`, and `percent-encoding` - current versions and registry timestamps. [VERIFIED: crates.io API]

### Secondary (MEDIUM confidence)

- None.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: MEDIUM - crate versions and library capabilities were verified, but the choice of `axum`/`tokio` plus `ureq` is still a phase recommendation rather than an existing repo convention. [VERIFIED: crates.io API][CITED: https://docs.rs/axum/latest/axum/][CITED: https://docs.rs/ureq/latest/ureq/]
- Architecture: HIGH - the managed adapter boundary, minimal-dependency policy, and pure-core/shell split are locked by project instructions and current code structure. [VERIFIED: repo grep]
- Pitfalls: HIGH - the major failure modes are backed directly by upstream source and functional tests. [VERIFIED: repo grep]

**Research date:** 2026-04-22
**Valid until:** 2026-05-06

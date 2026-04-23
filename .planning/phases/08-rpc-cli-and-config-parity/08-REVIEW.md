---
phase: 08-rpc-cli-and-config-parity
reviewed: 2026-04-23T05:26:55Z
depth: standard
files_reviewed: 39
files_reviewed_list:
  - BUILD.bazel
  - scripts/verify.sh
  - packages/Cargo.toml
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/wallet.rs
  - packages/open-bitcoin-rpc/Cargo.toml
  - packages/open-bitcoin-rpc/BUILD.bazel
  - packages/open-bitcoin-rpc/src/lib.rs
  - packages/open-bitcoin-rpc/src/error.rs
  - packages/open-bitcoin-rpc/src/envelope.rs
  - packages/open-bitcoin-rpc/src/envelope/tests.rs
  - packages/open-bitcoin-rpc/src/method.rs
  - packages/open-bitcoin-rpc/src/method/tests.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/tests.rs
  - packages/open-bitcoin-rpc/src/config/loader.rs
  - packages/open-bitcoin-rpc/src/config/loader/rpc_address.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - packages/open-bitcoin-rpc/src/http.rs
  - packages/open-bitcoin-rpc/src/http/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-cli/Cargo.toml
  - packages/open-bitcoin-cli/BUILD.bazel
  - packages/open-bitcoin-cli/src/lib.rs
  - packages/open-bitcoin-cli/src/args.rs
  - packages/open-bitcoin-cli/src/args/tests.rs
  - packages/open-bitcoin-cli/src/startup.rs
  - packages/open-bitcoin-cli/src/startup/tests.rs
  - packages/open-bitcoin-cli/src/getinfo.rs
  - packages/open-bitcoin-cli/src/getinfo/tests.rs
  - packages/open-bitcoin-cli/src/client.rs
  - packages/open-bitcoin-cli/src/client/tests.rs
  - packages/open-bitcoin-cli/src/output.rs
  - packages/open-bitcoin-cli/src/main.rs
  - packages/open-bitcoin-cli/tests/operator_flows.rs
findings:
  critical: 2
  warning: 4
  info: 0
  total: 6
status: issues_found
---

# Phase 08: Code Review Report

**Reviewed:** 2026-04-23T05:26:55Z
**Depth:** standard
**Files Reviewed:** 39
**Status:** issues_found

## Summary

Reviewed the Phase 8 RPC/CLI/config surface against the repo-local guidance in `AGENTS.md`, the Bright Builds sidecar in `AGENTS.bright-builds.md`, the placeholder `standards-overrides.md`, and the Bright Builds canonical architecture, code-shape, verification, testing, and Rust standards loaded from the upstream rules repo because the local `standards/` tree was absent.

The scoped Phase 8 test surface currently passes:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc -p open-bitcoin-cli`

The review still found two high-severity issues in RPC authentication and transaction broadcast safety, plus four behavioral regressions in CLI/config and wallet rescan handling that the current tests do not cover.

## Critical Issues

### CR-01: RPC Cookie Auth Secrets Are Weakly Generated And Weakly Stored

**File:** `packages/open-bitcoin-rpc/src/http.rs:403-447`
**Issue:** `read_or_create_cookie_password()` creates the RPC cookie with default file permissions and falls back to a PID-plus-timestamp secret when `/dev/urandom` is unavailable. On a typical `umask 022` system that leaves the cookie readable by other local users, and on platforms without `/dev/urandom` the generated password is predictable. That turns cookie auth into a local credential disclosure / local auth bypass risk.
**Fix:**
```rust
fn random_hex_secret() -> std::io::Result<String> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    Ok(hex_encode(&bytes))
}

#[cfg(unix)]
let mut file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .mode(0o600)
    .open(path)?;
```

### CR-02: `sendrawtransaction` Ignores Its Fee/Burn Safety Limits

**File:** `packages/open-bitcoin-rpc/src/dispatch.rs:289-313`
**Issue:** The Phase 8 request surface accepts `maxfeerate` and `maxburnamount` in `SendRawTransactionRequest`, but `send_raw_transaction()` only decodes and submits the transaction. The safety guardrails are never enforced, so a caller can pass low caps and still broadcast a transaction with an excessive fee or an accidental burn output. That is an irreversible operator-safety break on a money-moving RPC.
**Fix:**
```rust
if let Some(max_fee_rate_sat_per_kvb) = request.maybe_max_fee_rate_sat_per_kvb {
    validate_fee_rate(&transaction, &context.blockchain_snapshot(), max_fee_rate_sat_per_kvb)?;
}
if let Some(max_burn_amount_sats) = request.maybe_max_burn_amount_sats {
    validate_burn_amount(&transaction, max_burn_amount_sats)?;
}
```
If those checks are not ready yet, reject non-default values explicitly instead of silently ignoring them.

## Warnings

### WR-01: `rescanblockchain` Echoes The Requested Range But Always Scans The Full Chain

**File:** `packages/open-bitcoin-rpc/src/dispatch.rs:260-286`
**Issue:** `rescan_blockchain()` validates `start_height` and `stop_height`, but then passes the full `context.blockchain_snapshot()` into `context.rescan_wallet()`. The wallet scan walks every UTXO in that snapshot, so the requested height window only changes the response payload, not the actual rescan scope. Range-limited rescans therefore rebuild balances from the entire chain.
**Fix:** Filter the snapshot to the requested height interval before calling `rescan_wallet()`, or extend the wallet scan API to accept and enforce `start_height` and `stop_height` directly.

### WR-02: Interactive `bitcoin-cli` Invocations Block Until EOF Even Without `-stdin`

**File:** `packages/open-bitcoin-cli/src/main.rs:12-16`
**Issue:** `main()` unconditionally drains all of `stdin` before parsing arguments. In a normal interactive terminal, `open-bitcoin-cli getnetworkinfo` waits for EOF and never reaches config resolution or transport setup unless the operator manually sends `Ctrl-D`. This differs from Knots and makes ordinary CLI usage hang.
**Fix:** Add a lightweight `requires_stdin(&cli_args)` pre-check and only call `read_to_string()` when `-stdin` or `-stdinrpcpass` is actually present.

### WR-03: `rpcconnect` Accepts Hostname Syntax In The CLI But Rejects It In Config Resolution

**File:** `packages/open-bitcoin-rpc/src/config/loader.rs:559-566`
**Issue:** CLI parsing only validates the optional port fragment, but `parse_socket_address()` later rejects anything that is not an `IpAddr` or `SocketAddr`. `-rpcconnect=localhost` therefore parses successfully and then fails as `invalid rpc address: localhost`, even though the HTTP client itself can use hostnames.
**Fix:** Resolve hostnames with `ToSocketAddrs` / `lookup_host`, or store the host as a string plus port instead of forcing `SocketAddr` during config loading.

### WR-04: Duplicate Named CLI Parameters Are Silently Overwritten

**File:** `packages/open-bitcoin-cli/src/args.rs:356-404`
**Issue:** `set_named_value()` replaces an earlier `name=value` pair with the last one seen. That means `-named deriveaddresses descriptor=a descriptor=b` reaches transport instead of failing argument validation, and it bypasses the duplicate/collision checks already implemented in the shared RPC normalizer.
**Fix:** Preserve duplicate named arguments so `normalize_method_call()` can reject them, or reject duplicate keys directly in `parse_named_parameters()` before encoding the request.

---

_Reviewed: 2026-04-23T05:26:55Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_

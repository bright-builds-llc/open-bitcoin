# RPC, CLI, And Config Operator Surface

This entry tracks the supported Phase 8 operator interface slice implemented in
Open Bitcoin. The behavioral baseline remains Bitcoin Knots
`29.3.knots20260210`, but this document is intentionally scoped to the
supported RPC, CLI, and config surface that the current managed node and wallet
adapters can back honestly.

## Coverage

- authenticated local JSON-RPC over HTTP POST only
- baseline-shaped `bitcoin-cli` argument parsing for the supported operator
  slice, including `-named`, `-stdin`, `-stdinrpcpass`, `-conf`, `-datadir`,
  `-rpcconnect`, `-rpcport`, `-rpcuser`, `-rpcpassword`, `-rpccookiefile`,
  `-getinfo`, and `-color`
- supported baseline-backed RPC methods:
  `getblockchaininfo`, `getmempoolinfo`, `getnetworkinfo`, `sendrawtransaction`,
  `deriveaddresses`, `getwalletinfo`, `getbalances`, `listunspent`,
  `importdescriptors`, and `rescanblockchain`
- supported Open Bitcoin extension RPC methods:
  `buildtransaction`, `buildandsigntransaction`, `openbitcoinsyncstatus`,
  `openbitcoinsyncpause`, and `openbitcoinsyncresume`
- deterministic machine-readable CLI output for `-getinfo --json` and JSON
  result rendering for object or array RPC responses
- hermetic single-wallet operator workflow:
  `importdescriptors` -> `rescanblockchain` -> `getbalances` ->
  `listunspent` -> `buildandsigntransaction` -> `sendrawtransaction`

## Knots sources

- [`packages/bitcoin-knots/src/bitcoin-cli.cpp`](../../../packages/bitcoin-knots/src/bitcoin-cli.cpp)
- [`packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py`](../../../packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py)
- [`packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py`](../../../packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py)

## First-party implementation

- [`packages/open-bitcoin-cli/src/main.rs`](../../../packages/open-bitcoin-cli/src/main.rs)
- [`packages/open-bitcoin-cli/src/client.rs`](../../../packages/open-bitcoin-cli/src/client.rs)
- [`packages/open-bitcoin-cli/src/output.rs`](../../../packages/open-bitcoin-cli/src/output.rs)
- [`packages/open-bitcoin-cli/src/args.rs`](../../../packages/open-bitcoin-cli/src/args.rs)
- [`packages/open-bitcoin-cli/src/startup.rs`](../../../packages/open-bitcoin-cli/src/startup.rs)
- [`packages/open-bitcoin-rpc/src/http.rs`](../../../packages/open-bitcoin-rpc/src/http.rs)
- [`packages/open-bitcoin-rpc/src/method.rs`](../../../packages/open-bitcoin-rpc/src/method.rs)

## Supported behaviors

- CLI auth reuses the shared runtime-config loader and prefers local cookie
  auth when no explicit password is set.
- RPC transport stays POST-only with HTTP Basic auth and explicit exit-code `1`
  failures for bad credentials, unsupported methods, and actionable RPC errors.
- `-getinfo` remains a thin four-call batch over `getnetworkinfo`,
  `getblockchaininfo`, `getwalletinfo`, and `getbalances`.
- `buildtransaction` and `buildandsigntransaction` are repo-owned extension
  methods that provide deterministic build or sign flows over the managed
  wallet slice.
- `openbitcoinsyncstatus`, `openbitcoinsyncpause`, and
  `openbitcoinsyncresume` are repo-owned extension methods that let the
  operator CLI inspect or update daemon-owned durable sync control without
  opening the Fjall store from a second process.
- `openbitcoinnetworkstatus` is a repo-owned extension method that exposes
  Open Bitcoin network status evidence including bounded inbound status,
  metrics availability, and Phase 105 relay/mempool evidence under `relay`.
- Operator `status --format human`, `status --format json`, dashboard rows,
  and support bundles consume the same shared `mempool.relay` evidence
  contract instead of renderer-local relay summaries.
- `rescanblockchain` supports full active-snapshot rescans and rejects partial height ranges with invalid params because bounded wallet rescans are outside the Phase 8 adapter surface.
- `sendrawtransaction` explicit `maxfeerate` and `maxburnamount` values are rejected because those safety limits are not enforced by the supported dispatcher surface.
- `-rpcconnect=localhost` and other client-path hostname endpoints are supported, with explicit `-rpcport` taking precedence over embedded `-rpcconnect` ports and embedded ports taking precedence over the chain-default RPC port.
- `-stdin` and `-stdinrpcpass` are the only CLI flags that trigger stdin reads; normal no-stdin-flag invocations proceed to startup or transport without waiting for EOF.
- duplicate named parameters are rejected before transport through the shared
  method normalizer instead of being overwritten by CLI parsing.
- open-stdin regression coverage proves a normal CLI invocation without stdin flags does not wait on an open stdin pipe.
- cookie-auth creation uses a generated `__cookie__:<64 lowercase hex chars>`
  secret and owner-only Unix file mode for newly created cookie files.

## Phase 105 relay evidence classification

Phase 105 classifies operator-facing relay and mempool evidence as
implemented, unavailable, deferred, or intentionally different. The supported
RPC and CLI surfaces expose fixed counters only: `accepted_count`,
`rejected_count`, `orphaned_count`, `requested_count`, `served_count`,
`announced_count`, `suppressed_count`, `evicted_count`, `expired_count`, and
`rebroadcast_deferred_count`.

Support-bundle redaction protects raw transaction hex, txids, wtxids,
endpoints, socket-address shapes, peer identifiers, permission strings,
credentials, cookies, secrets, suspicious long hex, and dynamic labels. The
Phase 105 RPC and CLI surface does not claim public propagation, compact block
relay, package relay, bloom/filter serving, public relay defaults,
public-network relay CI, production service operation, production full-node
readiness, or production-funds wallet use.

## Phase 106 UAT and release boundary commands

Phase 106 records the repo-local UAT command boundary for the Open
Bitcoin-owned operator and RPC evidence surfaces. The runtime guide keeps the
canonical Cargo and Bazel forms for `status --format human`, `status --format
json`, `openbitcoinnetworkstatus`, and redacted support bundle collection.

The Phase 106 RPC and CLI closeout does not claim public propagation, compact
block relay, package relay, bloom/filter serving, public relay defaults,
public-network relay CI, production service operation, production full-node
readiness, production-service proof, production full-node readiness proof,
production-funds wallet use, or production-funds wallet safety proof.

## Phase 107 runtime relay activation and eligibility UAT

Phase 107 records the RPC and CLI evidence for
`v2-0-runtime-relay-activation-download-eligibility`. The daemon runtime uses
the resolved `RuntimeConfig.relay` value when constructing managed network
state, and the deterministic inbound-serving input is resolved
`config.inbound.enabled`. Operator review uses repo-local Cargo and Bazel
commands from `docs/operator/runtime-guide.md` for default-off status, explicit
`-openbitcoinrelay=1`, `openbitcoinnetworkstatus`, redacted support bundle
review, and `bash scripts/verify.sh`.

`openbitcoinnetworkstatus.relay`, operator `status --format json`, dashboard
rows, and support bundles expose aggregate sanitized activation and download
eligibility evidence only. Granular scheduler labels remain internal typed
test evidence unless reduced to fixed counters. `sendrawtransaction` success
does not guarantee public propagation.

Phase 107 does not claim public relay by default, compact block relay, package
relay, bloom/filter serving, public-network relay CI, production service
operation, production full-node readiness, production-funds wallet safety,
production-funds wallet use, or durable mempool recovery.

## Deferred surfaces

- deferred `sendtoaddress` and richer wallet-send RPC ergonomics beyond the current
  deterministic build or sign extensions
- deferred `getpeerinfo` and the human `-netinfo` dashboard that depends on it
- deferred `rpcwallet` / `-rpcwallet` and broader multiwallet endpoint selection
- deferred `rpcauth`, `rpcwhitelist`, and other remote-operator auth or ACL
  features
- deferred `rpcwait`, daemon supervision, and broader process-control CLI
  helpers

## Notes

- Phase 8 currently documents a single-wallet, local-operator slice; this
  entry is intentionally explicit so omitted Knots behaviors are treated as
  deferred rather than implied parity.
- Future updates should expand this ledger when new RPC methods, CLI helpers,
  or config semantics become supported.

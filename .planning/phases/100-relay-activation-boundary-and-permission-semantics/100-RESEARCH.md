# Phase 100: Relay Activation Boundary and Permission Semantics - Research

**Researched:** 2026-06-29
**Domain:** Default-off transaction relay activation, peer relay eligibility, scoped permission effects, config parsing, no-claim guardrails, and deterministic local verification
**Confidence:** HIGH for repo-local implementation patterns; MEDIUM for exact config spelling because Phase 100 leaves final key names to planning discretion.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Activation Contract

- **D-01:** Transaction relay must remain default-off through an explicit Open Bitcoin-owned activation setting. Default config, default daemon startup, and existing inbound listener enablement must not make Open Bitcoin a public transaction-relay participant.
- **D-02:** Activation should be represented as a typed policy decision, not scattered boolean checks. Prefer a pure data-in/data-out relay activation module that can be unit tested without socket, RPC, mempool, filesystem, or public-network side effects.
- **D-03:** Service bits must not change in Phase 100. If version-message relay preference is touched, it must be a deliberate output of the relay activation policy with matrix tests proving default-off behavior and no accidental public relay claim.
- **D-04:** Operator-facing naming should stay Open Bitcoin-owned. Reuse existing JSONC and `-openbitcoin...` CLI conventions rather than accepting Knots `whitelist` or `whitebind` compatibility inputs.

#### Peer Eligibility Matrix

- **D-05:** Add one explicit eligibility matrix for default config, outbound peers, inbound peers, manual/operator-configured peers, protected slots, and permissioned peers. The matrix should be pure and emit stable machine labels.
- **D-06:** Outbound and manual peers can become relay-eligible only after explicit relay activation. Inbound peers require both the existing inbound-serving boundary and an explicit v2.0 relay-eligible permission or class signal; ordinary public inbound peers are not relay-eligible by default.
- **D-07:** Protected admission is not transaction-relay eligibility. Existing `forceinbound` and `noban` effects may protect admission, eviction, or misbehavior policy, but they must not activate transaction relay unless paired with a scoped relay-like permission effect.
- **D-08:** Existing `download` and `addr` effects remain bounded policy inputs. Phase 100 must not reinterpret them as transaction inventory, mempool query, rebroadcast, or public relay permission.

#### Scoped Permission Effects

- **D-09:** Promote `relay`, `forcerelay`, and `mempool` from fully inactive labels into explicit v2.0 relay-permission policy effects, but only as eligibility evidence and policy inputs for later v2.0 transaction relay plans. Phase 100 itself should not mutate mempool state or perform socket relay actions.
- **D-10:** `relay` means peer eligibility for normal transaction inventory/request/send paths once Phase 101+ wires those paths. It does not imply compact blocks, package relay, bloom/filter serving, full address relay, public defaults, or production readiness.
- **D-11:** `forcerelay` implies the scoped `relay` eligibility signal and should be modeled as a separate force-relay policy input for later suppression/bypass rules.
- **D-12:** `mempool` means eligibility for scoped v2.0 mempool-related peer behavior once later phases own exact message and serving rules. It must not make the current `mempool` P2P command serve arbitrary transactions in Phase 100.
- **D-13:** `bloomfilter`, `blockfilters`, compact-filter-like behavior, and compact-block behavior remain inactive/deferred. Tests and docs must prove that `all` does not activate these surfaces.

#### Evidence, Docs, and Guardrails

- **D-14:** Status/support/log/metric evidence should use low-cardinality labels only. Do not expose raw permission class names, raw permission strings, peer ids, endpoints, transaction ids, raw transaction hex, credentials, or dynamic labels.
- **D-15:** Add deterministic no-claim guardrails for Phase 100 if docs, parity roots, or verifier order are updated. The checker should fail on claims that Phase 100 supports default public relay, compact block relay, bloom/filter serving, package relay, production service operation, production full-node readiness, production-funds wallet use, or public-network relay CI.
- **D-16:** Verification must stay local and deterministic. Public-network relay review, if documented, is opt-in UAT evidence outside `bash scripts/verify.sh`.
- **D-17:** New first-party Rust files under `packages/open-bitcoin-*/src` or tests under `packages/open-bitcoin-*/tests` need source-breadcrumb entries unless they use a defensible `none` breadcrumb.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| ACT-01 | Operator can enable transaction relay only through explicit relay activation settings that keep public relay off by default. | Add a typed default-off `RelayActivationConfig` or equivalent under Open Bitcoin-owned config, with CLI override if needed. Defaults must leave `LocalPeerConfig::default().relay` and service bits unchanged unless tests prove an intentional policy output. |
| ACT-02 | Node classifies peer relay eligibility across outbound, inbound, manual, protected, and permissioned peers without changing service bits or public defaults accidentally. | Add a pure eligibility function over `PeerConnectionClass`, activation state, and scoped permission effects. Matrix tests should cover ordinary inbound, permissioned inbound, protected inbound, outbound, manual configured, default disabled, and activated cases. |
| ACT-03 | Permission effects for `relay`, `forcerelay`, and `mempool` activate only scoped behavior documented for v2.0. | Extend or replace the current inactive relay labels with typed scoped relay policy effects for those three tokens, and keep them as policy inputs/evidence only in Phase 100. Do not wire them to peer socket actions or mempool mutation. |
| ACT-04 | Bloom/filter permissions, compact-block behavior, and unrelated peer permissions remain inactive unless later requirements explicitly activate them. | Preserve `inactive_bloomfilter` and `inactive_blockfilters`, add tests for `all`, compact block inventory, service bits, and no public/default relay claims. Keep `download`, `addr`, `noban`, and `forceinbound` semantics unchanged. |
</phase_requirements>

## Summary

Phase 100 should be planned as a narrow policy/config/docs phase. The core deliverable is a pure relay activation and eligibility boundary that later v2.0 phases can consume when transaction inventory, download, mempool admission, serving, fanout, RPC, metrics, logs, and support evidence are implemented.

The safest shape is:

1. Introduce a typed relay activation policy in `open-bitcoin-network`, near but separate from the existing inbound permission model.
2. Add Open Bitcoin-owned config parsing in `open-bitcoin-rpc` so activation remains default-off and explicit.
3. Convert `relay`, `forcerelay`, and `mempool` permission evidence from purely inactive labels into scoped v2.0 relay policy inputs, without connecting those inputs to socket I/O or mempool mutation in Phase 100.
4. Preserve inactive bloom/filter and compact-block boundaries with tests and optional deterministic docs/checker updates.

**Primary recommendation:** split planning into three plans: pure policy and tests, config/parser wiring, and docs/parity/no-claim verification. This keeps the risky semantic change (`relay`/`forcerelay`/`mempool`) separate from operator surface and guardrail work.

## Project Constraints

- `AGENTS.md` requires `bash scripts/verify.sh` as the repo-native verification contract before marking work complete.
- Rust source follows Rust `1.94.1` from `rust-toolchain.toml` and Rust 2024 workspace conventions.
- Pure Bitcoin domain behavior belongs in functional-core crates. For Phase 100 that means `open-bitcoin-network` should own relay activation and eligibility decisions, while RPC/config/runtime/doc layers remain adapters.
- New Rust source or test files under first-party packages need parity breadcrumbs through `docs/parity/source-breadcrumbs.json`, unless the file is Open Bitcoin-only support/infrastructure and uses the standard `none` breadcrumb reason.
- TypeScript checker work should use Bun, fixed-file reads, exported checker functions that return `string[]`, and fixture tests.
- UAT docs must use repo-local Cargo and Bazel commands, not only a bare installed `open-bitcoin` alias.
- Default verification must not add public-network, service-manager, long-running, or production-deployment gates.

## Existing Code

### `open-bitcoin-network`

- `packages/open-bitcoin-network/src/inbound/permissions.rs` already defines `PeerPermissionToken`, `PeerPermissionSet`, `PermissionEffectLabel`, `InactivePermissionEffectLabel`, `PeerConnectionClass`, `InboundPermissionDecision`, and `PeerPermissionClassRegistry`.
- Current active effects are admission protection, eviction policy input, misbehavior policy input, address-response policy input, and download-serving policy input.
- Current inactive effects are `inactive_relay`, `inactive_forcerelay`, `inactive_mempool`, `inactive_bloomfilter`, and `inactive_blockfilters`.
- `PeerConnectionClass` already includes the class labels needed for the eligibility matrix: `ordinary_inbound`, `permissioned_inbound`, `protected_inbound`, `outbound`, and `manual_configured`.
- `packages/open-bitcoin-network/src/message.rs` defines `ServiceFlags`, `VersionMessage`, and `LocalPeerConfig`. `LocalPeerConfig::default()` currently sets `services = NETWORK | WITNESS` and `relay = true`; Phase 100 must test whatever policy/config path it chooses so default daemon behavior still does not become public transaction relay.
- `packages/open-bitcoin-network/src/peer/tests.rs` already has negative tests proving relay-like labels are inactive for transaction paths and `all` does not activate service bits or compact blocks. Phase 100 should revise these tests deliberately rather than deleting the negative coverage.

### `open-bitcoin-rpc`

- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` owns the JSONC config contract. `OpenBitcoinConfig` currently has sections for `onboarding`, `metrics`, `logs`, `service`, `dashboard`, `migration`, `storage`, `sync`, and `inbound`.
- `InboundConfig` already owns `enabled`, `listen_addresses`, `max_peers`, `reserved_slots`, `allow_public`, and `permission_classes`.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` resolves JSONC plus CLI settings into `InboundListenerConfig`, including permission class parsing.
- `packages/open-bitcoin-rpc/src/config/loader/inbound.rs` shows the Open Bitcoin-prefixed CLI pattern, such as `-openbitcoininbound` and `-openbitcoininboundpermissionclass`.

### `open-bitcoin-node`

- `packages/open-bitcoin-node/src/network.rs` owns `ManagedPeerNetwork`, `ManagedNetworkInfo`, `local_config`, peer counts, mempool wrapper, and current in-memory transaction paths.
- `ManagedNetworkInfo` already exposes `local_services_bits` and `relay`; Phase 100 can add relay activation status only if needed, but should avoid broad status work that Phase 105 owns.
- `packages/open-bitcoin-node/src/network/inbound.rs` projects inbound permission decisions and is the right adapter if Phase 100 needs managed evidence for scoped permission effects.
- `packages/open-bitcoin-node/src/status/inbound.rs` owns shared inbound permission evidence fields and should be extended only if Phase 100 truly needs status exposure before Phase 105.

### Docs and Checkers

- `scripts/check-phase91-peer-permissions.ts` and `.test.ts` are the best local pattern for a permission/no-claim checker.
- `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/operator/runtime-guide.md`, `docs/parity/catalog/p2p.md`, `docs/parity/index.json`, and `docs/parity/checklist.md` are the most likely docs touched if Phase 100 exposes activation semantics.
- `scripts/verify.sh` includes both executable `run_step` ordering and a visible `VERIFY_COMMAND_ORDER` block; checkers that assert ordering should inspect both.

## Recommended Architecture

### Policy Types

Add a pure policy surface in `open-bitcoin-network`. Suggested names are examples; the planner can refine them.

```rust
pub struct RelayActivationConfig {
    pub enabled: bool,
}

pub enum RelayPermissionEffectLabel {
    TransactionRelayPolicyInput,
    ForceRelayPolicyInput,
    MempoolPolicyInput,
}

pub enum RelayEligibilityReason {
    Eligible,
    RelayDisabled,
    PermissionRequired,
    ActivationRequired,
    ProtectedNotRelay,
    PermissionEffectInactive,
}

pub struct RelayEligibilityInput {
    pub activation: RelayActivationConfig,
    pub connection_class: PeerConnectionClass,
    pub relay_effects: Vec<RelayPermissionEffectLabel>,
}
```

Keep the policy function deterministic:

```rust
pub fn classify_relay_eligibility(input: &RelayEligibilityInput) -> RelayEligibilityDecision
```

The policy should avoid direct references to sockets, runtime peer managers, mempool state, RPC context, wall-clock time, random values, filesystem state, or public-network state.

### Permission Effect Split

Do not keep Phase 100 semantics as raw strings. The existing `PermissionEffectLabel` and `InactivePermissionEffectLabel` split is the right pattern, but Phase 100 needs a third explicit category or expanded active-effect vocabulary:

- Keep `bloomfilter` and `blockfilters` inactive.
- Keep unrelated current active effects unchanged: `forceinbound`, `noban`, `download`, and `addr`.
- Promote `relay`, `forcerelay`, and `mempool` into scoped relay policy effects, not socket actions.
- Preserve `forcerelay` implying `relay`, matching existing `PeerPermissionSet::insert_expanded_permission`.
- Preserve `all` expansion, with tests proving it activates only v2.0-scoped relay policy effects plus existing bounded effects while keeping bloom/filter and compact-block behavior inactive.

### Config

Recommended JSONC shape:

```jsonc
{
  "relay": {
    "enabled": false
  }
}
```

Recommended CLI override pattern, if the planner decides Phase 100 needs CLI:

```text
-openbitcoinrelay=1
```

Rationale:

- `relay.enabled` mirrors existing `sync.network_enabled` and `inbound.enabled` explicit activation style.
- `-openbitcoinrelay` matches the repo's Open Bitcoin-prefixed operator flag style.
- This avoids accepting baseline `-whitelist`, `-whitebind`, or other Knots flags as full compatibility.

### Tests

Pure policy tests should cover:

- Default config: relay disabled, no public relay eligibility.
- Outbound peer: not eligible until activation is enabled.
- Manual peer: not eligible until activation is enabled.
- Ordinary inbound peer: not eligible by default, even if inbound listener is enabled.
- Permissioned inbound peer with no `relay`/`forcerelay`/`mempool`: not eligible for transaction relay.
- Protected inbound peer with only `forceinbound`/`noban`: protected admission remains separate from relay eligibility.
- Permissioned inbound peer with `relay`: eligible only when activation is enabled.
- Permissioned inbound peer with `forcerelay`: carries force-relay policy input and relay eligibility only when activation is enabled.
- Permissioned inbound peer with `mempool`: carries mempool policy input only when activation is enabled and does not serve the P2P `mempool` command in Phase 100.
- `all`: includes scoped `relay`, `forcerelay`, and `mempool` policy effects, keeps bloom/filter inactive, and does not alter compact-block/service-bit behavior.

Config/parser tests should cover:

- JSONC default yields `relay.enabled = false`.
- JSONC `relay.enabled = true` enables activation in the resolved runtime config.
- CLI override wins over JSONC if `-openbitcoinrelay` is added.
- Unknown relay config keys fail under `deny_unknown_fields`.
- Config error messages use existing `Error reading open-bitcoin.jsonc: ...` style.

Docs/checker tests should cover:

- Allowed wording: default-off, scoped v2.0 relay policy input, future relay serving, no public relay default.
- Forbidden wording: public relay by default, compact block relay support, bloom/filter serving support, package relay support, production full-node readiness, production service operation, production-funds wallet support, public-network relay CI.

## Threat Model Inputs for Plans

Phase 100 plans should include `<threat_model>` blocks because security enforcement is enabled.

High-signal threats:

- **Elevation of Privilege:** A protected or permissioned peer accidentally becomes relay-eligible without explicit activation or without a relay-like permission effect.
- **Information Disclosure:** Status/support/logs expose raw permission classes, endpoints, peer IDs, transaction IDs, raw transaction hex, or credentials.
- **Denial of Service:** `all` or `forcerelay` accidentally bypasses queue/resource governance or raises request caps before later phases define those controls.
- **Tampering:** CLI or JSONC parsing silently accepts unsupported Knots compatibility flags and changes relay behavior outside Open Bitcoin-owned config.
- **Repudiation:** Docs or parity roots claim relay is supported without deterministic evidence roots and no-claim guardrails.

## Recommended Plan Split

### Plan 100-01: Pure Relay Activation and Eligibility Policy

Build the functional core in `open-bitcoin-network`: activation config type, scoped relay permission effect labels, relay eligibility input/decision types, matrix tests, and breadcrumb updates. This plan should not touch RPC config, docs, or runtime socket paths except as needed for tests.

### Plan 100-02: Config and Parser Wiring

Add `relay.enabled` and optional `-openbitcoinrelay` parsing in `open-bitcoin-rpc`, resolve it into a typed config, and add parser/precedence tests. If a managed runtime field is needed, thread the typed activation config without changing peer socket behavior.

### Plan 100-03: Docs, Parity, and No-Claim Guardrails

Update parity/operator docs and add or update a deterministic Bun checker only if docs or parity roots change. Wire through `scripts/verify.sh`, update source breadcrumbs if new Rust files were added, and create `100-VERIFICATION.md` after full verification passes.

## Pitfalls

- Do not implement Phase 101 transaction inventory scheduling in Phase 100.
- Do not mutate mempool admission or serving behavior in Phase 100.
- Do not let `mempool` permission make the P2P `mempool` command serve transactions yet.
- Do not make `forceinbound` or `noban` imply relay eligibility.
- Do not use service bits as proof of transaction relay activation.
- Do not add public-network checks to default verification.
- Do not introduce a new external dependency for IP matching, config parsing, or checkers.
- Do not forget `docs/parity/source-breadcrumbs.json` if a new Rust source/test file is added.

## Verification Recommendations

Focused local iteration:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network relay_activation
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc relay
bun test scripts/check-phase100-relay-activation-boundary.test.ts
bun run scripts/check-phase100-relay-activation-boundary.ts
```

Final phase verification:

```bash
bash scripts/verify.sh
```

If docs add UAT commands, use repo-local forms:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -openbitcoinrelay=1
bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -openbitcoinrelay=1
```

The final `100-VERIFICATION.md` should record `status: passed`, map ACT-01 through ACT-04 to implementation evidence, and state the residual boundary: Phase 100 defines activation and eligibility only; transaction download, mempool admission, relay serving/fanout, compact blocks, bloom/filter serving, package relay, public relay defaults, public-network CI, production service operation, production full-node readiness, and production-funds wallet use remain deferred.

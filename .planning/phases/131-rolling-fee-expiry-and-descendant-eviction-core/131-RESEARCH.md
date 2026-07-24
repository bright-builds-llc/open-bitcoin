# Phase 131: Rolling Fee, Expiry, and Descendant Eviction Core - Research

**Researched:** 2026-07-24
**Domain:** Mempool sustained-pressure policy (accounted-capacity trim, descendant-package eviction, rolling-fee bump/decay, expiry cleanup)
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Capacity enforcement

- **D-01:** Enforce configured mempool capacity from accounted memory usage against `MempoolCapacity`. Virtual size remains a distinct fee and reporting measure and must not drive trim.
- **D-02:** Retire `PolicyConfig.legacy_vsize_trim_limit` as the active trim limiter. Update capacity-enforcement evidence from `legacy_vsize` to accounted-memory enforcement.
- **D-03:** Trim continues until accounted usage is within capacity. Over-capacity classification and ledger aggregates stay consistent with the Phase 130 recomputation oracle.

#### Descendant-package eviction and rolling bump

- **D-04:** Pressure selects the lowest Knots-compatible descendant-score package (existing `descendant_score` ordering, then txid tie-break) and removes that victim plus all descendants as one package.
- **D-05:** On each pressure package removal, raise `RollingMempoolFeeRate` from the actual evicted package feerate plus incremental relay fee, matching Knots `trackPackageRemoved` / `TrimToSize` bump semantics. Clear any block-since-last-bump gate so decay cannot start until a later block connect.
- **D-06:** Preserve Phase 130 fee-role boundaries: incremental remains a replacement and pressure-bump input; effective admission stays `max(static, rolling)`; do not store effective as mutable state; do not let package aggregates bypass the wrong floor.

#### Block-gated rolling decay

- **D-07:** Rolling-floor decay is block-gated only. No wall-clock decay without a connected-block lifecycle context. Pure policy uses `BlockLifecycleContext.connected_at` (and occupancy) rather than reading clocks.
- **D-08:** Match pinned Knots half-life and rounding behavior: 12-hour default half-life, shortened to 6-hour when usage is below half capacity and 3-hour when below quarter capacity; floor rolling to zero below the incremental/2 boundary per Knots `GetMinFee` semantics where those rules affect the rolling state.
- **D-09:** Operator evidence continues to expose static, incremental, rolling, and effective roles separately. Rolling-fee parity status must leave `Deferred` once bump and decay are live.

#### Expiry and index cleanup

- **D-10:** Add a pure mempool expiry API that takes explicit `PolicyTime` (or a narrow expiry context) and removes entries whose acceptance age exceeds policy, emitting `MempoolRemovalCause::Expiry` with Direct/Descendant roles.
- **D-11:** Expiry and pressure removals must leave no stale descendants or derived indexes; always remove through existing topology helpers and `recompute_state` / resource-ledger recompute so graph and fee-aggregate invariants hold.
- **D-12:** Shell adapters sample current time and invoke expiry through `ManagedNetworkHandle` / managed network authority. Pure core never reads wall-clock time. Do not invent acceptance times for `LegacyUnknown` entries—fail closed or skip per Phase 130 metadata rules.

#### Determinism, oracle, and bounds

- **D-13:** Deterministic fill, trim, block, decay, expiry, refill, and reorg scenarios must agree with recomputation oracles for membership, accounted usage, and rolling fee after each committed transition.
- **D-14:** Document resource and performance bounds for sustained-pressure sequences and enforce them with hermetic tests in the default verifier. No public-network or non-deterministic soak gates.
- **D-15:** Rolling fee remains non-durable for this phase: restart baseline stays zero unless a later durability phase redesigns persistence (MPDUR / Phase 135 territory).

### Claude's Discretion

- Exact module split for trim/bump/decay/expiry helpers inside `open-bitcoin-mempool`, provided the public semantic contracts stay clear.
- Internal representation of block-since-last-bump and last-rolling-fee-update state, provided decay stays block-gated and occupancy-sensitive.
- Whether pressure trim accepts `PressureDecisionContext` as a required argument or threads occupancy through existing ledger + capacity fields, provided pure code still never samples clocks.
- Exact performance threshold numbers, provided they are documented, hermetic, and fail the default verifier when exceeded.
- Temporary retention of `set_rolling_mempool_fee_rate` for tests until internal bump/decay fully own the state machine.

### Deferred Ideas (OUT OF SCOPE)

- Typed package vocabulary, staged admission, TRUC/ephemeral-dust package exceptions — Phase 132.
- Package-aware download / same-peer 1P1C orphan bridge — Phase 133.
- Authoritative cross-cache lifecycle projection for every dependent cache — Phase 134.
- Snapshot schema, checkpointing, and recovery of durable mempool records — Phase 135 (rolling fee remains non-durable).
- Receive-independent maintenance loops and transport receipts beyond the minimum expiry/decay wiring — Phase 136.
- Broader RPC/operator evidence expansion beyond correcting enforcement and rolling-fee labels needed for PRESS — Phase 137.

None — discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PRESS-01 | Enforce configured mempool capacity using deterministic accounted-memory usage; keep vsize as a separate fee/reporting measure | Switch `trim_to_size` limiter from `legacy_vsize_trim_limit` to `mempool_capacity` vs `resource_ledger.accounted_memory()`; flip `MempoolCapacityEnforcement` evidence; keep `bytes`/vsize reporting unchanged |
| PRESS-02 | Pressure selects complete descendant packages by descendant-score order and raises rolling minimum from the actual evicted package | Extend existing `select_eviction_candidate` + `collect_descendants`; add Knots `trackPackageRemoved` bump (`package feerate + incremental`, strict greater-than) |
| PRESS-03 | Rolling decay is block-gated, occupancy-sensitive, clock-safe; match 12h/6h/3h half-life and rounding | Port Knots `GetMinFee` decay math with injected `PolicyTime`; gate on block-since-bump; zero below incremental/2; expose raw rolling separately from effective |
| PRESS-04 | Expiry and pressure leave no stale descendants/indexes; preserve graph and fee-aggregate invariants | Pure `Expire`-shaped API; remove via topology helpers + `recompute_state`; emit `Expiry`/`Pressure` lifecycle removals with Direct/Descendant roles |
| PRESS-05 | Sustained fill/trim/block/decay/expiry/refill/reorg sequences stay bounded and agree with recomputation oracle and perf thresholds | Hermetic scenario tests + oracle assertions + `open-bitcoin-bench` / verifier-gated thresholds; no public-network gates |
</phase_requirements>

## Summary

Phase 131 activates the Phase 130 primitives: capacity enforcement must switch from transitional `legacy_vsize_trim_limit` to accounted memory against `MempoolCapacity`, while virtual size remains fee/RPC `bytes` only. Eviction already selects the lowest `descendant_score` package with txid tie-break and removes victims plus descendants through `recompute_state`; the missing Knots behaviors are (1) bumping the rolling floor from the **descendant-package** feerate plus incremental relay fee via `trackPackageRemoved` semantics, and (2) block-gated occupancy-sensitive decay matching `GetMinFee`. Expiry is not yet a production pure-core path—`MempoolRemovalCause::Expiry` exists but nothing emits it—so Phase 131 must add a `PolicyTime`-driven expiry API and minimal `ManagedNetworkHandle` wiring.

Knots anchors are concrete and verified in-tree: `TrimToSize`, `trackPackageRemoved`, `GetMinFee`, `Expire`, `removeForBlock` (sets `blockSinceLastRollingFeeBump = true`), default expiry 336 hours, and `ROLLING_FEE_HALFLIFE = 12h` with `/2` and `/4` occupancy shortening. Open Bitcoin already has fee-role wrappers, resource ledger + recomputation oracle, lifecycle delta vocabulary, and shell cause→serving maps. The planner should follow the CONTEXT natural commit order and treat floating-point decay + `llround` boundaries as first-class differential fixtures, while preserving Phase 130’s intentional rule that ordinary admission/effective reporting use `max(static, rolling)` and never let incremental contaminate those surfaces.

**Primary recommendation:** Implement a pure rolling-fee state machine (bump + block-gated decay) beside accounted-capacity `trim_to_size`, then expiry cleanup, then evidence-label flip / legacy seam removal, then hermetic oracle+perf gates—wiring only the minimum shell hooks through `ManagedNetworkHandle`.

## Project Constraints (from .cursor/rules/ and AGENTS)

No `.cursor/rules/` directory is present in this repo. Applicable constraints from `AGENTS.md` / `AGENTS.bright-builds.md` / standards:

- Functional core / imperative shell: pure mempool never reads wall-clock or randomness. [VERIFIED: AGENTS.md, standards/core/architecture.md]
- No Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md]
- Verification contract: `bash scripts/verify.sh` (default pre-commit/release). [VERIFIED: AGENTS.md]
- Parity breadcrumbs required for new first-party Rust sources under `packages/open-bitcoin-*/src` or `tests`. [VERIFIED: AGENTS.md]
- Intentional Knots differences must be recorded in `docs/parity/`. [VERIFIED: AGENTS.md]
- Unit-test pure business logic (Arrange/Act/Assert). [VERIFIED: standards/core/testing.md]
- Prefer early returns; avoid `unwrap()`. [VERIFIED: AGENTS.bright-builds.md, user code-styling rules]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|-------------------|---------|---------|--------------|
| Rust toolchain | `1.94.1` | Language/runtime pin | `rust-toolchain.toml` / Cargo workspace pin [VERIFIED: rustc/cargo --version] |
| `open-bitcoin-mempool` | workspace | Pure pressure, rolling fee, expiry | Existing domain owner for trim/lifecycle/fee/resource [VERIFIED: codebase] |
| `open-bitcoin-node` | workspace | Minimal shell wiring via `ManagedNetworkHandle` | Sole mutation authority [VERIFIED: Phase 127/130 decisions] |
| Bitcoin Knots baseline | `29.3.knots20260210` | Behavioral oracle for trim/bump/decay/expiry | Pinned submodule under `packages/bitcoin-knots` [VERIFIED: AGENTS.md + source] |
| `f64::powf` + Knots `llround` | std | Rolling decay math | Matches Knots `double`/`pow`/`llround` in `GetMinFee` [VERIFIED: txmempool.cpp; CITED: .planning/research/STACK.md] |

### Supporting

| Library / Surface | Version | Purpose | When to Use |
|-------------------|---------|---------|-------------|
| `open-bitcoin-bench` | workspace | Hermetic sustained-pressure thresholds | PRESS-05 perf bounds in default verifier [VERIFIED: packages/open-bitcoin-bench] |
| Bun | `1.3.9` (local) | Parity checkers / verify automation | Breadcrumb and claim-check scripts [VERIFIED: bun --version] |
| Existing test modules under `pool/tests/` | workspace | Unit/oracle fixtures | Extend rather than invent parallel harness [VERIFIED: codebase] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Accounted-memory trim | Keep legacy vsize trim | Forbidden by D-01/D-02; would block PRESS-01 parity |
| Wall-clock decay in core | Shell-only decay timer | Violates functional-core and D-07 |
| Persist rolling fee | Restart-zero (locked) | Would diverge from Knots dump/load and D-15 |
| Criterion / public soak | Hermetic `open-bitcoin-bench` | Public/network/wall-clock gates forbidden by D-14 |
| Exact C++ `DynamicUsage` | Rust-owned ledger v1 | Already intentional Phase 130 difference [VERIFIED: mempool-policy.md] |

**Installation:** none — first-party crates and std only. No new Cargo dependencies required for the locked design.

**Version verification:** Rust `1.94.1` and Bun `1.3.9` probed on the research host (2026-07-24). No third-party crates added.

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-mempool/src/
├── fee.rs                 # keep role wrappers; add rolling state helpers or thin re-exports
├── fee/rolling.rs         # RECOMMENDED: RollingFeeState bump/decay (f64 internal, FeeRate external)
├── resource.rs            # accounted ledger + oracle (consume, do not redesign)
├── context.rs             # PressureDecisionContext / BlockLifecycleContext / PolicyTime
├── types.rs               # PolicyConfig: retire active legacy_vsize seam; add expiry duration
├── pool.rs                # Mempool fields + public APIs
├── pool/admission.rs      # post-admit trim calls accounted trim + bump
├── pool/lifecycle.rs      # evidence labels: AccountedMemory + live rolling parity
├── pool/pressure.rs       # RECOMMENDED: trim_to_size, select_eviction, track_package_removed
├── pool/expiry.rs         # RECOMMENDED: expire(PolicyTime) / expire_before cutoff
└── pool/tests/
    ├── pressure_cases.rs  # NEW: trim/bump/oracle
    ├── rolling_fee_cases.rs # NEW: block gate, half-lives, rounding, zero
    └── expiry_cases.rs    # NEW: age + descendant cleanup

packages/open-bitcoin-node/src/network/
├── mempool_lifecycle.rs   # call decay after connect cleanup
├── admission_bridge.rs    # already maps Pressure/Expiry → serving
└── runtime_authority.rs   # expose expire_mempool(PolicyTime) mutate path
```

### Pattern 1: Accounted-capacity trim loop (Knots `TrimToSize`)

**What:** While `accounted_memory > capacity`, select lowest descendant-score entry, compute package feerate from descendant aggregates, bump rolling (`package + incremental`), remove victim+descendants, recompute indexes.
**When to use:** After every committing admission that can exceed capacity (existing admission trim hook).
**Example (algorithmic, from Knots):**

```cpp
// Source: packages/bitcoin-knots/src/txmempool.cpp TrimToSize / trackPackageRemoved
while (!mapTx.empty() && DynamicMemoryUsage() > sizelimit) {
    auto it = mapTx.get<descendant_score>().begin();
    CFeeRate removed(it->GetModFeesWithDescendants(), it->GetSizeWithDescendants());
    removed += m_opts.incremental_relay_feerate;
    trackPackageRemoved(removed); // bumps only if removed > rolling; clears block gate
    CalculateDescendants(...);
    RemoveStaged(stage, false, MemPoolRemovalReason::SIZELIMIT);
}
```

Open Bitcoin mapping [VERIFIED: pool.rs]:
- Limiter: `state.resource_ledger.accounted_memory()` vs `config.mempool_capacity` (not `total_virtual_size` / `legacy_vsize_trim_limit`).
- Package feerate for bump: `FeeRate::from_fee_sats_and_vbytes(descendant_stats.total_fee_sats, descendant_stats.virtual_size)` then add incremental (saturating/checked as appropriate).
- Selection: keep existing `descendant_score()` + txid tie-break (D-04).

### Pattern 2: Block-gated rolling decay (Knots `GetMinFee` + `removeForBlock`)

**What:** After a pressure bump, `block_since_last_rolling_fee_bump = false` and decay is disabled. On connected-block lifecycle, set the gate true and refresh `last_rolling_fee_update` from `BlockLifecycleContext.connected_at`. Subsequent decay applications (on block context and/or when reading/applying min fee with an injected time after the gate is open) use occupancy-adjusted half-life and the 10-second update gate.
**When to use:** Block connect path and any pure API that materializes the current rolling floor for admission/evidence after a block has opened the gate.
**Knots constants** [VERIFIED: txmempool.h / txmempool.cpp / mempool_tests.cpp]:

| Constant | Value | Role |
|----------|------:|------|
| `ROLLING_FEE_HALFLIFE` | `60 * 60 * 12` (43200 s) | Default half-life |
| Half capacity | `usage < sizelimit / 2` | Half-life `/= 2` → 6h |
| Quarter capacity | `usage < sizelimit / 4` | Half-life `/= 4` → 3h |
| Update gate | `time > lastRollingFeeUpdate + 10` | Skip tiny steps |
| Zero threshold | `rolling < incremental/2` | Collapse to 0 |
| Rounding | `llround(rollingMinimumFeeRate)` | External integer sat/kvB |

```cpp
// Source: packages/bitcoin-knots/src/txmempool.cpp GetMinFee
if (!blockSinceLastRollingFeeBump || rollingMinimumFeeRate == 0)
    return CFeeRate(llround(rollingMinimumFeeRate));
// else if time advanced > 10s:
//   rolling /= pow(2.0, (time - last) / halflife);
//   zero if < incremental/2
```

**Open Bitcoin fee-role caveat (intentional, locked):** Knots `GetMinFee` may return `max(llround(rolling), incremental)` during mid-decay, and RPC `mempoolminfee` is `max(GetMinFee, min_relay)`. Phase 130 / D-06 keep ordinary admission and `mempoolminfee` as `max(static, rolling)` and keep incremental out of that derivation. Decay must still **zero the rolling state** below `incremental/2`, but must **not** rewrite effective admission to incremental. Differential fixtures should compare rolling raw state and `llround` steps to Knots, then assert Open Bitcoin effective separately. [VERIFIED: txmempool.cpp + rpc/mempool.cpp + Phase 130 CONTEXT]

### Pattern 3: Expiry (`Expire`) with injected time

**What:** Given explicit `now: PolicyTime` and configured expiry duration (Knots default 336 hours), remove every entry with known acceptance time `< now - expiry`, plus all descendants, as `MempoolRemovalCause::Expiry`.
**When to use:** Shell-sampled maintenance / authority mutate; also callable from pure tests with fake clocks.
**Knots call site** [VERIFIED: validation.cpp `LimitMempoolSize`]:

```cpp
int expired = pool.Expire(GetTime<std::chrono::seconds>() - pool.m_opts.expiry);
pool.TrimToSize(pool.m_opts.max_size_bytes, ...);
```

**LegacyUnknown policy (discretion recommendation):** Skip expiry for `MempoolAcceptanceTime::LegacyUnknown` / recovery-unknown records (do not invent times). Document that those entries are retained until replaced, confirmed, conflicted, or pressure-evicted. Prefer skip over failing the whole sweep so mixed recovery pools remain operable. [ASSUMED: choosing skip over fail-closed within D-12]

### Pattern 4: Minimal shell wiring

**What:** 
1. Admission path already trims inside pure commit — bump comes for free once trim is upgraded.
2. `apply_connected_block_mempool_lifecycle` should open the decay gate (and apply/update rolling state) using `BlockLifecycleContext` after connect removals—mirroring `removeForBlock` setting `blockSinceLastRollingFeeBump = true` and `lastRollingFeeUpdate = GetTime()`.
3. Add `ManagedNetworkHandle` mutate for `expire_mempool(PolicyTime)` that samples time in the shell and projects Expiry removals through existing serving maps (Phase 134 still owns full cross-cache completeness).

### Anti-Patterns to Avoid

- **Vsize-driven trim presented as Knots capacity parity:** Forbidden by D-01; update all tests that set `legacy_vsize_trim_limit` as the active limiter.
- **Wall-clock decay without a post-bump block:** Knots and D-07 both forbid this; `mempool_tests.cpp` asserts the floor stays until `removeForBlock`.
- **Bump from individual victim feerate:** Must use descendant-package feerate + incremental.
- **Mutating effective fee state:** Derive only.
- **Inventing acceptance times for LegacyUnknown:** D-12.
- **Persisting rolling fee:** D-15 / Phase 135.
- **Public-network or sleep-based soak in default verify:** D-14.
- **Full cross-cache claims:** Phase 134 territory; Phase 131 only needs truthful mempool core + minimum authority wiring.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rolling exponential decay | Custom discrete ladders / integer-only approximations without Knots fixtures | Local `f64` state + `pow(2.0, dt/halflife)` + `llround` | Boundary bugs at 12h/6h/3h and zero threshold [VERIFIED: mempool_tests.cpp] |
| Capacity estimation | C++ allocator mimicry | Phase 130 `MempoolResourceLedger` / `recompute_resource_ledger` | Intentional Rust-owned formula already documented |
| Descendant package selection | New graph crate / random eviction | Existing `descendant_score` + `collect_descendants` | Already Knots-shaped; D-04 locks tie-break to txid |
| Lifecycle vocabulary | New removal enums | `MempoolRemovalCause::{Pressure,Expiry}` + roles | Already defined; wire producers |
| Perf harness | Criterion-only / wall-clock CI | Extend `open-bitcoin-bench` + hermetic unit scenarios | Matches STACK and D-14 |
| Time sources in core | `SystemTime::now` in mempool | `PolicyTime` / contexts from shell | FEEP-04 / D-07 / D-12 |

**Key insight:** Phase 131 is mostly **activating and completing** existing seams, not greenfield policy invention. The risky novelty is faithful rolling-state math and accounted-capacity switching without breaking Phase 130 evidence contracts.

## Common Pitfalls

### Pitfall 1: Decay starts immediately after bump
**What goes wrong:** Rolling floor falls before any block, re-admitting just-evicted feerates.
**Why it happens:** Treating decay as pure wall-clock; forgetting `blockSinceLastRollingFeeBump`.
**How to avoid:** Clear gate on bump; set gate only on connected-block lifecycle; unit-test the `mempool_tests.cpp` sequence.
**Warning signs:** Rolling decreases between two admissions with no block context.

### Pitfall 2: Wrong bump numerator/denominator
**What goes wrong:** Floor rises too little/too much; CPFP packages mis-priced.
**Why it happens:** Using single-tx feerate or `descendant_score` max(self, package) instead of Knots `GetModFeesWithDescendants` / `GetSizeWithDescendants`.
**How to avoid:** Bump from descendant aggregate fee/size of the selected victim; add incremental; bump only if strictly greater than current rolling.
**Warning signs:** Parent+child eviction bumps equal parent-only rate.

### Pitfall 3: Incremental contaminates effective admission
**What goes wrong:** Operator/`mempoolminfee` disagree with Phase 130 contract.
**Why it happens:** Copying Knots `GetMinFee` return `max(rolling, incremental)` into Open Bitcoin effective.
**How to avoid:** Keep raw rolling for decay/zeroing; derive effective as `max(static, rolling)` only; expose incremental separately.
**Warning signs:** `rollingmempoolfee` and `mempoolminfee` collapse to incremental while raw rolling is mid-decay below static.

### Pitfall 4: Stale descendants after expiry/pressure
**What goes wrong:** Child remains after parent expiry/eviction; spent-outpoint / aggregates diverge.
**Why it happens:** Deleting map entries without `collect_descendants` + `recompute_state`.
**How to avoid:** One removal helper shared by pressure and expiry; oracle-check ledger and parent/child sets after every transition.
**Warning signs:** `recompute_resource_ledger` != cached ledger; child with missing parent.

### Pitfall 5: Legacy vsize tests silently keep old limiter
**What goes wrong:** PRESS-01 “passes” while production still trims on vsize.
**Why it happens:** Many fixtures set `legacy_vsize_trim_limit` (pool tests, recovery tests, RPC tests).
**How to avoid:** Migrate fixtures to small `mempool_capacity`; delete or gut the active legacy field; assert `capacityenforcement == "accounted_memory"` (or chosen stable label).
**Warning signs:** Tests still construct `PolicyConfig { legacy_vsize_trim_limit: ... }`.

### Pitfall 6: Sustained trim clones/recomputes unboundedly
**What goes wrong:** Hermetic perf gate fails; admission latency explodes under fill/trim loops.
**Why it happens:** Prospective `entries.clone()` + full `recompute_state` per eviction iteration (current design).
**How to avoid:** Document bounds for N-entry / M-trim scenarios now; keep oracle recompute in tests; defer wholesale incremental indexes if thresholds can be met; do not claim Phase 134-scale cache work.
**Warning signs:** Nonlinear time as trim iterations grow; verifier timeouts.

### Pitfall 7: Tie-break / modified-fee mismatch vs Knots
**What goes wrong:** Different victim chosen when scores equal.
**Why it happens:** Knots uses entry-time tie-break and modified fees; Open Bitcoin uses txid and base fees (no `prioritisetransaction`).
**How to avoid:** Keep D-04 txid tie-break; document as intentional difference in parity catalog; fixture equal-score pairs explicitly.
**Warning signs:** Differential tests vs Knots multi-index order on equal scores.

## Code Examples

Verified patterns from in-repo sources:

### Accounted trim limiter switch

```rust
// Target shape for pool trim (adapt existing trim_to_size).
// Source mapping: packages/open-bitcoin-mempool/src/pool.rs + Knots TrimToSize
while state.resource_ledger.accounted_memory() > config.mempool_capacity.as_accounted() {
    let Some(victim_txid) = select_eviction_candidate(&state.entries) else {
        break;
    };
    let package_rate = package_descendant_feerate(&state.entries, victim_txid);
    rolling.track_package_removed(package_rate.saturating_add(incremental));
    // remove victim + descendants, recompute_state, record Pressure Direct/Descendant
}
```

### Rolling bump (strict greater-than)

```rust
// Source: packages/bitcoin-knots/src/txmempool.cpp trackPackageRemoved
fn track_package_removed(state: &mut RollingFeeState, rate: FeeRate) {
    if rate > state.rolling_fee_rate() {
        state.set_rolling(rate);
        state.block_since_last_bump = false;
    }
}
```

### Block-gated decay step

```rust
// Source: packages/bitcoin-knots/src/txmempool.cpp GetMinFee + removeForBlock
// On connected block:
state.block_since_last_bump = true;
state.last_update = context.connected_at;

// When applying decay with injected `now` and occupancy:
if !state.block_since_last_bump || state.rolling_as_f64 == 0.0 {
    return Rounding::llround(state.rolling_as_f64);
}
if now.unix_seconds() > state.last_update.unix_seconds() + 10 {
    let mut halflife = 12.0 * 3600.0;
    if usage < capacity / 4 { halflife /= 4.0; }
    else if usage < capacity / 2 { halflife /= 2.0; }
    let dt = (now.unix_seconds() - state.last_update.unix_seconds()) as f64;
    state.rolling_as_f64 /= 2.0_f64.powf(dt / halflife);
    state.last_update = now;
    if state.rolling_as_f64 < (incremental_sats_per_kvb as f64) / 2.0 {
        state.rolling_as_f64 = 0.0;
    }
}
```

### Expiry selection

```rust
// Source: packages/bitcoin-knots/src/txmempool.cpp Expire
// cutoff = now - expiry_duration (default 336 hours)
// for each entry with Known(accepted_at) if accepted_at < cutoff: stage victim
// CalculateDescendants for each; RemoveStaged(..., EXPIRY)
```

## State of the Art

| Old Approach (Phase 130 / current) | Current Target (Phase 131) | When Changed | Impact |
|------------------------------------|----------------------------|--------------|--------|
| Trim on `legacy_vsize_trim_limit` | Trim on accounted memory vs `MempoolCapacity` | Phase 131 | PRESS-01 |
| Rolling fee set only via test setter; parity `Deferred` | Live bump/decay state machine; parity leaves Deferred | Phase 131 | PRESS-02/03 |
| No production Expiry producer | Pure expire API + shell invoke | Phase 131 | PRESS-04 |
| Capacity evidence `legacy_vsize` | Accounted-memory enforcement label | Phase 131 | Operator truth |
| Rolling non-durable (implicit zero) | Explicit restart-zero contract retained | Unchanged / D-15 | Aligns with Phase 135 |

**Deprecated/outdated:**
- Active use of `PolicyConfig.legacy_vsize_trim_limit` as trim limiter (retire per D-02).
- `RollingFeeParityStatus::Deferred` as the live production label once bump/decay ship.
- Any docs claiming Phase 130 still owns enforcement (catalog already points to Phase 131).

## Recommended Plan Breakdown

Follow CONTEXT natural commit order (planner should produce roughly these plans):

1. **Accounted trim + rolling bump** — switch limiter; `trackPackageRemoved`; Pressure deltas include rolling change; unit tests for package bump and capacity.
2. **Block-gated decay** — rolling state fields; connect-path gate; half-life/rounding/zero fixtures mirroring `mempool_tests.cpp`.
3. **Expiry cleanup** — pure API + descendant cleanup + Expiry roles; LegacyUnknown skip; shell authority hook.
4. **Evidence / seam retirement** — `MempoolCapacityEnforcement::AccountedMemory` (label `accounted_memory`); rolling parity live status; remove active legacy vsize seam; update RPC/node tests and `docs/parity/catalog/mempool-policy.md` + breadcrumbs/index as needed.
5. **Sustained-pressure oracle + perf** — fill/trim/block/decay/expiry/refill/reorg scenario; ledger/rolling oracle agreement; documented thresholds in verifier/`open-bitcoin-bench`.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Prefer **skip** (not fail-closed) for `LegacyUnknown` during expiry sweeps | Expiry pattern | Fail-closed would block expiry on any legacy pool; skip retains undated entries longer |
| A2 | Evidence label string for enforcement should be `accounted_memory` | Evidence flip | RPC/checker fixtures must match whatever string is chosen |
| A3 | Decay may be applied on block connect and/or when materializing rolling for admission after the gate opens, as long as no pre-block wall-clock decay occurs | Decay pattern | Over-eager connect-only decay without 10s gate fidelity could diverge from Knots step timing |
| A4 | Current clone+recompute trim can meet Phase 131 hermetic thresholds without a new eviction index | Perf pitfall | May need an incremental ordering structure inside the phase if benches fail |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

## Open Questions

1. **Exact live `RollingFeeParityStatus` variant name**
   - What we know: must leave `Deferred` (D-09).
   - What's unclear: `Active` vs `Live` vs `Enforced` string for metrics/RPC.
   - Recommendation: `Active` with `as_str() -> "active"`; update checkers/tests in the evidence plan.

2. **Whether `legacy_vsize_trim_limit` field is deleted or retained as dead/unused**
   - What we know: must not be the active limiter (D-02).
   - What's unclear: delete vs `#[deprecated]` stub for one release of fixtures.
   - Recommendation: delete from `PolicyConfig` in the evidence/seam plan once all call sites migrate—cleaner than a zombie field.

3. **How closely to mirror Knots modified-fee / entry-time ordering**
   - What we know: D-04 locks existing score + txid tie-break; no modified-fee surface exists.
   - What's unclear: whether any PRESS differential fixture expects Knots multi-index equality cases.
   - Recommendation: document intentional difference; do not add prioritisation in Phase 131.

4. **Expiry scheduling cadence in the node**
   - What we know: shell must sample time and call through authority (D-12); full maintenance loops are Phase 136.
   - What's unclear: whether Phase 131 only exposes the API + test/authority hook, or also a minimal call from an existing tick.
   - Recommendation: ship pure API + `ManagedNetworkHandle` method + unit/integration invocation; optional call from an existing receive/maintenance seam only if already present—do not build Phase 136 timers.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | Implementation + tests | ✓ | 1.94.1 | — |
| Bun | Parity/verify scripts | ✓ | 1.3.9 | — |
| Bitcoin Knots submodule sources | Behavioral anchors | ✓ | in-tree `packages/bitcoin-knots` | — |
| Public network / live soak | Not required | n/a | — | Hermetic fixtures only |

**Missing dependencies with no fallback:** none identified.

**Missing dependencies with fallback:** none identified.

Step 2.6 note: Phase is code/config + hermetic tests; no external services required.

## Validation Architecture

> Included for planner / future Nyquist VALIDATION.md generation. Repo `workflow.nyquist_validation` is currently `false` in `.planning/config.json`; keep these mappings ready when validation docs are generated.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust `#[test]` / Cargo workspace tests (`open-bitcoin-mempool`, `open-bitcoin-node`, `open-bitcoin-rpc`) + `open-bitcoin-bench` |
| Config file | `packages/Cargo.toml` workspace; `rust-toolchain.toml` |
| Quick run command | `bun run scripts/command-timings.ts run --key mempool-press-quick -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib` |
| Full suite command | `bash scripts/verify.sh` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PRESS-01 | Trim uses accounted memory vs capacity; vsize distinct | unit | `cargo test -p open-bitcoin-mempool accounted_capacity_trim` | ❌ Wave 0 — extend/replace `legacy_vsize` tests |
| PRESS-02 | Descendant-score package eviction + rolling bump | unit | `cargo test -p open-bitcoin-mempool pressure_bump` | ⚠️ Partial — eviction exists; bump missing |
| PRESS-03 | Block-gated 12h/6h/3h decay + rounding/zero | unit | `cargo test -p open-bitcoin-mempool rolling_fee_decay` | ❌ Wave 0 |
| PRESS-04 | Expiry/pressure cleanup invariants | unit | `cargo test -p open-bitcoin-mempool expiry_descendant_cleanup` | ❌ Wave 0 — cause enum only |
| PRESS-05 | Multi-step scenario + oracle + perf threshold | unit + bench | `cargo test -p open-bitcoin-mempool sustained_pressure_oracle`; bench case in `open-bitcoin-bench` | ❌ Wave 0 |
| Evidence | `capacityenforcement` / rolling parity labels | unit/RPC | `cargo test -p open-bitcoin-rpc capacityenforcement` | ⚠️ Exists asserting `legacy_vsize` — must flip |
| Shell | Expire/decay via authority | unit | `cargo test -p open-bitcoin-node expire_mempool` / lifecycle decay | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** targeted `cargo test -p open-bitcoin-mempool --lib <filter>`
- **Per wave merge:** mempool + node + rpc packages affected by labels
- **Phase gate:** `bash scripts/verify.sh` green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs` — PRESS-01/02
- [ ] `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs` — PRESS-03 (port vectors from `mempool_tests.cpp`)
- [ ] `packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs` — PRESS-04
- [ ] Sustained scenario + oracle assertions — PRESS-05
- [ ] Hermetic perf threshold case in `packages/open-bitcoin-bench/src/cases/mempool.rs` (or sibling) — PRESS-05
- [ ] Update existing fixtures that depend on `legacy_vsize_trim_limit` / `capacityenforcement: legacy_vsize` / `RollingFeeParityStatus::Deferred`
- [ ] Parity doc/breadcrumb updates for live rolling-fee and accounted enforcement

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | N/A — no new auth surface |
| V3 Session Management | no | N/A |
| V4 Access Control | partial | Expire/decay mutate only via `ManagedNetworkHandle`; no second authority |
| V5 Input Validation | yes | Validate `PolicyTime`, capacity, expiry duration; reject/skip illegal metadata combinations |
| V6 Cryptography | no | No new crypto; fee math is policy arithmetic |

### Known Threat Patterns for mempool pressure

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Fee-floor oscillation / churn DoS after trim | Denial of Service | Rolling bump + block-gated decay (Knots trackPackageRemoved/GetMinFee) |
| Unbounded mempool memory growth | Denial of Service | Accounted-capacity trim to `MempoolCapacity` |
| Stale child retention after parent removal | Tampering / DoS | Mandatory descendant collection + recompute oracle |
| Clock manipulation in core | Spoofing | Injected `PolicyTime` only; shell samples clocks |
| Evidence label injection | Information Disclosure | Fixed low-cardinality enums (`pressure`, `expiry`, enforcement strings) |
| Cross-cache stale serve after eviction | Elevation / Spoofing of availability | Minimal serving cleanup now; complete projection in Phase 134 |

## Sources

### Primary (HIGH confidence)

- `packages/bitcoin-knots/src/txmempool.cpp` — `Expire`, `GetMinFee`, `trackPackageRemoved`, `TrimToSize`, `removeForBlock` rolling gate
- `packages/bitcoin-knots/src/txmempool.h` — `ROLLING_FEE_HALFLIFE`, rolling state fields, `CompareTxMemPoolEntryByDescendantScore`
- `packages/bitcoin-knots/src/kernel/mempool_options.h` — `DEFAULT_MEMPOOL_EXPIRY_HOURS = 336`
- `packages/bitcoin-knots/src/validation.cpp` — `LimitMempoolSize` (Expire then TrimToSize)
- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `mempoolminfee` / `usage` / `maxmempool` meanings
- `packages/bitcoin-knots/src/test/mempool_tests.cpp` — rolling half-life differential expectations
- `packages/open-bitcoin-mempool/src/{pool.rs,fee.rs,resource.rs,context.rs,pool/lifecycle.rs,pool/admission.rs}` — current seams
- `packages/open-bitcoin-node/src/network/{mempool_lifecycle.rs,admission_bridge.rs,runtime_authority.rs}` — shell authority
- `docs/parity/catalog/mempool-policy.md` — Phase 130 contracts and Phase 131 ownership boundary
- `.planning/phases/131-.../131-CONTEXT.md`, `.planning/phases/130-.../130-CONTEXT.md`, `.planning/REQUIREMENTS.md`, `.planning/research/{STACK,PITFALLS,FEATURES}.md`

### Secondary (MEDIUM confidence)

- `.planning/research/STACK.md` — f64 decay containment and non-persistence of rolling fee
- `.planning/research/PITFALLS.md` — pressure/fee/eviction hazard catalogue

### Tertiary (LOW confidence)

- Exact hermetic performance threshold numbers (discretion; must be measured during implementation) [ASSUMED until benches land]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — first-party crates + verified Knots anchors; no new deps
- Architecture: HIGH — seams and commit order locked by CONTEXT; algorithms verified in Knots sources
- Pitfalls: HIGH — mirrored by in-repo PITFALLS.md and concrete current code gaps

**Research date:** 2026-07-24
**Valid until:** 2026-08-23 (30 days; stable baseline, revisit if Knots pin or Phase 130 ledger formula changes)

---

## RESEARCH COMPLETE

**Phase:** 131 - Rolling Fee, Expiry, and Descendant Eviction Core
**Confidence:** HIGH

### Key Findings

- Knots algorithms are fully pinned in-tree: accounted/dynamic usage trim loop, descendant-package bump via `trackPackageRemoved`, block-gated `GetMinFee` decay (12h/6h/3h, 10s gate, incremental/2 zero), and `Expire` at 336h default.
- Open Bitcoin already has selection/removal topology, fee roles, resource oracle, and lifecycle vocabulary; Phase 131 must switch the trim limiter, add bump/decay state, emit Expiry, and flip evidence labels.
- Preserve Phase 130 fee-role split: bump uses incremental; effective admission stays `max(static, rolling)`—do not import Knots’ mid-decay `max(rolling, incremental)` into effective.
- Natural plan order: accounted trim+bump → block-gated decay → expiry → evidence/legacy seam removal → sustained oracle/perf gates; package admission and full cross-cache projection remain deferred.
- Highest risks: pre-block decay, wrong bump package rate, leftover legacy_vsize fixtures, and clone/recompute cost under sustained trim.

### File Created

`.planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-RESEARCH.md`

### Confidence Assessment

| Area | Level | Reason |
|------|-------|--------|
| Standard Stack | HIGH | No new libraries; verified Rust 1.94.1 and existing crates |
| Architecture | HIGH | Locked CONTEXT + verified Knots/Open Bitcoin seams |
| Pitfalls | HIGH | Cross-checked against Knots tests, PITFALLS.md, and current code gaps |

### Open Questions

- Live rolling-parity label string (`active` recommended)
- Delete vs temporarily keep `legacy_vsize_trim_limit` field after limiter retirement
- How much node-side expiry scheduling beyond an authority API belongs in 131 vs 136

### Ready for Planning

Research complete. Planner can now create PLAN.md files.

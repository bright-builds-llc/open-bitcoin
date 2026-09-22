# Phase 147: Pure Prune Policy and Lock Windows - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-22T12:07:11.819Z
**Phase:** 147-Pure Prune Policy and Lock Windows
**Mode:** Yolo
**Areas discussed:** Typed prune mode, Automatic height window, Manual keep-window refusal, Lock buffer, Plan boundary

---

## Typed prune mode

| Option | Description | Selected |
|--------|-------------|----------|
| Typed enum plus Knots integer contract | Disabled / manual-only / automatic MiB, parsed as `0`, `1`, or `>= 550`; default disabled; no RPC/CLI in this phase | ✓ |
| Raw integer left in the decision core | Keep Knots `-prune=N` as the working value everywhere | |
| New Open Bitcoin-only config with no Knots integer mapping | Friendlier names, diverge from operator-visible Knots values | |

**User's choice:** [auto] Typed enum plus Knots integer contract
**Notes:** [auto-select] Selected all gray areas. Recommended default matches `.planning/research/FEATURES.md` and PRUN-01. Operator commands stay in Phase 150.

---

## Automatic height window

| Option | Description | Selected |
|--------|-------------|----------|
| Injected tip, prune-after, and per-height sizes | Pure plan keeps 288 blocks, waits for network prune-after, and budgets bytes from caller-supplied sizes | ✓ |
| Mainnet prune-after constant inside the function | Hard-code one network height | |
| Shell measures disk inside chainstate | Policy stats Fjall or the filesystem | |

**User's choice:** [auto] Injected tip, prune-after, and per-height sizes
**Notes:** Empty plan before prune-after is not an error. Constants 288 and 550 MiB are locked; off-by-one stays with Knots `GetPruneRange`.

---

## Manual keep-window refusal

| Option | Description | Selected |
|--------|-------------|----------|
| Typed refusal inside the 288-block window | Manual target height must match Knots; no automatic byte budget in manual mode | ✓ |
| Clamp the manual height down into the legal window | Silently prune less than requested | |
| Share one formula for manual and automatic without checking Knots | Faster, risks a parity miss | |

**User's choice:** [auto] Typed refusal inside the 288-block window
**Notes:** Researcher must cite the manual-versus-automatic difference before planning codes the comparison.

---

## Lock buffer

| Option | Description | Selected |
|--------|-------------|----------|
| Injected ranges plus Knots 10-block buffer | Pure forbid check; no persistence; omit protected heights rather than inventing a new failure if Knots skips them | ✓ |
| Persist locks in Fjall in this phase | Combines LOCK-01 and LOCK-02 | |
| Fixed 10-block buffer in a direction chosen without reading Knots | Risks protecting the wrong side | |

**User's choice:** [auto] Injected ranges plus Knots 10-block buffer
**Notes:** LOCK-02 operator list/set stays in Phase 150. Buffer direction and inclusivity must match `DoPruneLocksForbidPruning`.

---

## Plan boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Pure plan or typed refusal only | No deletes, no have-pruned, no `Pruned` label, no `FlushMode` change | ✓ |
| Also set a flush-for-prune fact and orchestration order | Pulls Phase 148 unlink sequencing forward | |
| Also advertise `NODE_NETWORK_LIMITED` | Pulls Phase 149 serving forward | |

**User's choice:** [auto] Pure plan or typed refusal only
**Notes:** Single active chainstate. No new crate. Verification stays `bash scripts/verify.sh`.

---

## Claude's Discretion

- Module split inside `open-bitcoin-chainstate`.
- Exact keep-window and buffer comparisons after research quotes Knots.
- Collection type for injected per-height sizes.
- Unit-test fixture style.

## Deferred Ideas

- Payload unlink, have-pruned, and interrupted prune — Phase 148.
- Limited serving and honest `Pruned` labels — Phase 149.
- Operator prune surfaces and lock list/set — Phase 150.
- Parity roots and no-claim guardrails — Phase 151.
- `-pruneduringinit` — FUT-27.
- txindex combined with prune — out of scope.
- Flush-before-unlink — Phase 148.

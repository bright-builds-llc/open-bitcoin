# Validated reorg and retained BASIC branch identity (Phase 158)

CFIX-03 is Complete for the internal production validated reorg path. The full
native contract passed in 19m24.571s, independent source/security review closed
all findings and 25 mitigations, and formal lifecycle verification passed 21/21
truths. [Final verification](../../.planning/phases/158-validated-reorg-and-retained-branch-identity/158-VERIFICATION.md)
records the earned scope; external filter serving remains pending.

Pinned Knots `29.3.knots20260210` anchors are
[`index/base.cpp`](../../packages/bitcoin-knots/src/index/base.cpp),
[`index/blockfilterindex.cpp`](../../packages/bitcoin-knots/src/index/blockfilterindex.cpp),
[`validation.cpp`](../../packages/bitcoin-knots/src/validation.cpp) and
[`undo.h`](../../packages/bitcoin-knots/src/undo.h). Open Bitcoin uses immutable
hash-keyed Fjall records, active-height projections and guarded SyncAll batches;
it does not reproduce Knots' filter flat-file layout.

## Acceptance, publication and recovery

The private staged reorg exposes borrowed facts. Genuine absorption mints a
sealed accepted receipt before persistence; preview freezes publication without
earning acceptance. The serialized manager binds generation, branch, revision,
frontier and same-store authority. Every nonempty ordinary replacement or
recovered-prefix turn requires accepted positions from the live manager.
Replacement filter headers follow the exact common ancestor's predecessor.

Accepted target, processed cursor and safe durable checkpoint are separate.
Equal-height, longer, shorter, disconnect-only and A→B→A followed by a normal
validated connect exercise the existing production caller. Full disconnect has
an explicit paused NoTip target, never a fabricated accepted block. A displaced
coins fence cannot earn safe replacement progress. Only an ordinary own coins
write followed by compatible metadata earns the replacement fence; index turns
do not force coins writes. No-pressure IfNeeded preserves ordinary cadence.

Rewind atomically replaces ownership/checkpoint/protection and masks incompatible
physical suffix rows. Hash-keyed displaced and already-written replacement
records remain immutable. After dropping every owner, configured Enabled reopen
uses recovered coins and compatible metadata to select canonical active records;
it resumes conflicting physical projections through sealed accepted positions.
Coins/metadata disagreement refuses explicitly. Software fault/reopen tests do
not establish hardware power-loss resilience.

Required retained body or genuine undo loss refuses before preview, mempool and
index effects. Native byte/allocation admission precedes decode, and decoded
undo must equal genuine historical undo exactly. Current coins, another branch,
snapshot payloads, empty/invented history, skipping, downloads and repair are
not substitutes. A valid immutable filter may be reused without its source only
when consensus does not require that input. Reserved prune locks and fresh
generation/frontier checks protect still-required inputs before paired deletion.

## Executed evidence and limits

The continuous fixture and required branch tests live in
[`branches.rs`](../../packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs),
[`retention.rs`](../../packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs),
[`failures.rs`](../../packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs),
[`protection.rs`](../../packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs)
and [`measurements.rs`](../../packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs).
The standard case retains runtime maturity 100, mined PoW and actual flags,
spending from height 101 and forking at 100 through 106 below P2SH height 150.
Compact fixtures explicitly use maturity one and easy PoW. Runtime halving
210,000 differs from Knots regtest 150; these tests do not claim full regtest
parameter parity. Genuine 401-height and 1,025-height histories prove retained
source protection, interrupted-intent refusal before deletion and actual paired
pruning across the unchanged automatic threshold 1,000. Automatic pressure uses
test-only target zero; production minimum-target parsing is separate evidence.

[The current measurements](../../.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md)
preserve 54 preparation configurations and 149 complete ordinary turns at
prefixes 16/128/512 and depths 1/8/32. Full eight-body turns observe 98 record,
44 projection and 7,621 checkpoint operations with 56 BASIC point reads in the
fixed-depth case. The first captured-fact turn instead observes seven body
reads, eight generations, 72 record / 42 projection operations and 44 point
reads. Historical Phase 157 observations remain unchanged.

**Intentional local resource-policy difference:** independently existing finite
storage, source and fact caps can refuse even fully retained deep/shared-gap
forks before effects. The required 401-block over-budget case preserves old
prefix/protection. Arbitrary retained-fork liveness, unlimited lag, whole-runtime
constant memory/work, isolated publication latency, archive-scale performance
and hard timing guarantees are not claimed. Stage-inclusive preparation and
whole reorg/turn durations include their existing callers; reservations are not
RSS. No new production cap was derived from an observed average.

## Scope and reproduction

[Repo-local UAT](../../.planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md)
provides timed Cargo/Bazel forms and exact named tests. The deterministic guard
supplements executed behavior and the default full native contract. Independent
source/security review and formal phase/lifecycle verification remain root
gates before completion.

Filter/index RPC (159), peer filter serving (160), operator projections (161)
and integrated retained-client proof (162) remain pending and are not shipped.
V0/type 2, BIP37, public serving defaults, public-network verification,
production readiness, production-funds safety and hardware crash guarantees
remain deferred. There is no public reorg CLI/RPC in this phase.

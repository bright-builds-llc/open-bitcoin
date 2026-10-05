# BASIC compact filter construction

Phase 154 implements pure BASIC construction and commitments for CFIL-01 and
CFIL-02 against Bitcoin Knots `29.3.knots20260210`, exact commit
`a9aee730466ac67d35a3c03ee24676be5e045878`. [Phase verification](../../../.planning/phases/154-basic-generation-and-commitment-parity/154-VERIFICATION.md)
passed all 11 must-haves and three roadmap criteria; the full native verifier,
including coverage and Bazel smoke, passed on 2026-10-04.

This scope is BASIC-only: V0 and BIP37 serving are excluded. Filter index
public activation, RPC serving, peer serving, catch-up,
GUI, production readiness and production-funds claims remain deferred.

## Algorithm and inputs

- [blockfilter.cpp](../../../packages/bitcoin-knots/src/blockfilter.cpp) anchors
  `BuildFilterElements`, `HashToRange` and `BuildHashedSet`: omit empty scripts;
  omit outputs whose first byte is OP_RETURN; include nonempty historical spent
  scripts even when their first byte is OP_RETURN; deduplicate raw scripts before
  mapping while retaining collisions between distinct scripts.
- [blockfilter.h](../../../packages/bitcoin-knots/src/blockfilter.h) pins P=19,
  M=784931 and header composition; the first 16 raw block-hash bytes are two
  little-endian SipHash keys. Mapping takes the high half of the unsigned
  128-bit product with N\*M.
- [golombrice.h](../../../packages/bitcoin-knots/src/util/golombrice.h),
  [streams.h](../../../packages/bitcoin-knots/src/streams.h) and
  [serialize.h](../../../packages/bitcoin-knots/src/serialize.h) anchor unary
  quotients, 19-bit remainders, MSB-first bits, zero padding and CompactSize N.
  Filter hashes are SHA256d(encoded bytes); headers are SHA256d(raw filter hash
  || raw predecessor header). Display hex reverses each raw digest.

`BasicFilter::from_script_facts` is a low-level fact API, without a completeness
claim. `BasicFilterInputs` accepts validated chain position and complete
historical per-transaction undo. It refuses missing earlier history and
identity/cardinality mismatches instead of consulting the current UTXO set.
The encoder is block-bounded; it is not a general unbounded GCS implementation.
Contextual commitments require explicit genesis or a matching predecessor
height/hash and reuse the common ancestor when a branch is replaced.

## Independent evidence and limits

[generate-basic-filter-vectors.ts](../../../scripts/generate-basic-filter-vectors.ts)
checks the exact clean Knots pin, reads all ten
[blockfilters.json](../../../packages/bitcoin-knots/src/test/data/blockfilters.json)
rows and compares independently generated block identities, bytes and headers
before accepting additional edges. It derives the missing filter-hash column
independently. Sparse upstream vectors flatten spent scripts into a synthetic
undo; Rust corpus tests deliberately use the low-level script-fact API and do
not relax the strict historical adapter.

[basic-filter-oracle.py](../../../scripts/basic-filter-oracle.py) imports pinned
`test_framework.crypto.siphash.siphash` and `messages.CBlock`,
`ser_compact_size`, `hash256`. This narrow Python compatibility exception
provides independent pinned protocol helpers. Bun owns filesystem orchestration,
pin checks, bounded input/output, the ten-second child timeout, corpus validation
and dependency-free Rust fixture rendering. No Rust-under-test results supply
expected outputs. This is a source-audited, corpus-cross-validated independent
oracle, **not a directly linked Knots binary**.

The frozen collision uses zero keys, N=2, range=1569862 and scripts `dd040000`
and `82060000`, both mapping to 1560268. Discovery scanned little-endian u32
candidates below 10000 and found the first collision at candidate 1666. Default
verification reproduces the frozen pair without a search.

[basic_filter_vectors.rs](../../../packages/open-bitcoin-consensus/testdata/basic_filter_vectors.rs)
contains empty/raw duplicate/mapped collision/OP_RETURN/empty script/malformed
script/count-252/253 cases; pinned witness and malformed-output corpus cases;
SipHash lengths 64/255/256/257; and six ordered/replacement branch blocks.
The validated spend fixture uses coinbase scripts `0051`, `010151`,
`010251`, timestamps 1000/1100/1200, easy target `207fffff`, nonce zero and
historical scripts `51` then `52`. Its raw spending identity is
`4f281baf8875d9c7ab412e10e71926df2fad3688fd04404034c1869a9498ed15`;
its filter is `0444047c0618719703318c90`. The Rust tests validate and stage
this three-transaction spend, preserve exact historical/same-block undo assertions,
then prove retained-history commitments after both spent coins disappear.
Independent constants also cover a replacement spend and each branch successor.

## Reproduction

Materialize the pinned submodule with `git submodule update --init --recursive`.
Use repo-pinned Bun and Python 3 with its standard library; no added dependencies,
Rust Bitcoin library or full Knots build is needed. From the repository root:

```bash
bun run scripts/generate-basic-filter-vectors.ts --check
bun run scripts/generate-basic-filter-vectors.ts --write
bun test ./scripts/check-phase154-basic-filters.test.ts
bun run scripts/check-phase154-basic-filters.ts
bun run scripts/command-timings.ts run --key phase154-independent-basic-parity -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --test basic_filter
bun run scripts/command-timings.ts run --key phase154-historical-basic-parity -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate block_filter
```

`--check` regenerates in memory and fails on drift without writing tracked
files. `--write` is explicit regeneration and still first requires all ten
upstream agreements. The default `bash scripts/verify.sh` runs checker tests,
the structural claim/provenance checker and oracle check before Rust checks.
Structural checks supplement exact behavioral tests; they do not prove parity
alone. Full coverage, native verification and Bazel smoke passed as Phase 154 gates.

## Recoverable internal index and pre-prune startup protection

Phase 155 addresses CFIX-02, CFIX-04 and CFPR-03 through existing-crate internal
storage and startup foundations. Its [formal verification](../../../.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-VERIFICATION.md)
records 10/10 distinct truths after full native verification; the v2.5 milestone
is not complete. The unique parity owner is
`v2-5-recoverable-basic-index-and-startup-protection` in
[index.json](../index.json).

Pinned [index/base.cpp](../../../packages/bitcoin-knots/src/index/base.cpp)
anchors `Commit`, `Rewind`, `ChainStateFlushed` and prune-lock ordering.
[index/blockfilterindex.cpp](../../../packages/bitcoin-knots/src/index/blockfilterindex.cpp)
anchors `CustomCommit`/`CustomRewind`, contextual commitments and retained branch
records. [node/blockstorage.cpp](../../../packages/bitcoin-knots/src/node/blockstorage.cpp)
anchors buffered protection and interrupted deletion behavior.

Open Bitcoin uses additive versioned BASIC envelopes in Fjall's existing
schema-2 BlockIndex namespace instead of Knots flat files/LevelDB. Immutable
block-hash rows are separate from checkpoint-visible projection and saved
progress. SyncAll batches publish records/projection/checkpoint/protection
together; record-only commits cannot advance authority. Saved higher fences
record historical publication provenance, not current authority. Recovered coins
B and independently compatible contiguous durable metadata decide safe progress;
same-height saved fence/endpoint conflicts refuse. B/meta disagreement after
coins H/B replay conservatively refuses before prune resume. Missing/corrupt/weak
reserved protection is never silently recreated. The direct startup input guard
also protects heights 0/1, beyond the existing ordinary buffered helper.

Real Fjall close/reopen tests use actual validated historical and same-block
spends and alternative branches, retaining immutable rows after deferred/failed
coins/meta persistence. They separately inspect cursor, fence, projection,
protection, bodies, non-genesis undo and live intent. Sparse codec-valid long
history fixtures serve deletion-order proof only and do not claim consensus
validation. Before-record/checkpoint/protection and after-commit software faults
exercise the real store and production runtime. A concrete metadata fault stops
the actual flush adapter after successful coins B publication; compatible H/B
replay succeeds while stale metadata stops startup and preserves required input.

The full startup integrity scan performs linear retained-history work with
bounded additional record memory. The deterministic 256-record/256-projection
corpus measures exactly 1,023 key/prefix reads (`4N - 1`), without per-row ancestor
walks or a full index cache. This is not a total runtime/memory cap, latency
promise, archive-scale benchmark or hardware power-loss simulation. Category-only
failures do not dump filter/script payloads. No implicit repair or download occurs.

Phase 156 extends ordinary prune ownership, lock CRUD and disable/re-enable below;
157 owns activation and scheduled catch-up; 158 owns validated runtime reorg;
159–162 own RPC/peer/operator serving and integrated retained-client proof. These
product surfaces, GUI, production readiness and production-funds claims remain deferred.

```bash
bun run scripts/command-timings.ts run --key phase155-production-recovery-matrix -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib filter_index
bun test ./scripts/check-phase155-filter-index.test.ts
bun run scripts/check-phase155-filter-index.ts
bash scripts/verify.sh
```

The native checker validates source/manifest/parity links and guard ordering;
its mutation tests supplement actual durable Rust evidence. The default native
workflow includes the existing Bazel smoke; run Cargo/Bazel work through the
repo timing wrapper and serialize builds. No public-network run is required.

## Index-owned manual and automatic prune coordination

Phase 156 implements the internal CFPR-01 ownership and application contract.
Evidence for the full native gate and formal phase/lifecycle proof is recorded in
[Phase 156 verification](../../../.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-VERIFICATION.md). The unique parity owner is
`v2-5-index-owned-manual-and-automatic-prune-coordination` in [index.json](../index.json).
The [runtime evidence summary](../../../.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-07-SUMMARY.md)
records targeted node prune 129/129, authenticated RPC prune 38/38, legal-target
daemon 1/1 and inherited daemon 5/5 passes. The root-owned
[shutdown repair artifact](../../../.planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-FJALL-SHUTDOWN-FIX.md)
separately records the final filter-index 102/102 pass after the inherited Fjall
shutdown regression was repaired. These selections are not the full native gate.

Pinned [index/base.cpp](../../../packages/bitcoin-knots/src/index/base.cpp)
anchors `Commit`, `ChainStateFlushed`, `Stop` and `SetBestBlockIndex`: committed
progress owns prune release and stopping work precedes releasing protection.
[index/blockfilterindex.cpp](../../../packages/bitcoin-knots/src/index/blockfilterindex.cpp)
anchors `CustomCommit`/`CustomRewind` and retained filter records;
[node/blockstorage.cpp](../../../packages/bitcoin-knots/src/node/blockstorage.cpp)
anchors manual/automatic selection and buffered locks. Open Bitcoin retains the
schema-2 Fjall layout and paired-key logical accounting rather than adopting
Knots blk/rev file pruning or LevelDB index storage. Its direct input checks also
cover heights 0/1, preserving the ordinary buffered helper's existing arithmetic.

The additive 11-byte `basic_filter:v1:owner` envelope records Active or Disabled
and a checked u64 generation. Existing v1 immutable, projection and checkpoint
bytes stay unchanged. All-artifact absence preserves legacy no-index behavior;
valid saved state with no owner is legacy Active generation zero. The real
runtime constructor materializes that owner without resetting a nonempty saved
prefix. Corrupt, partial, weak or inconsistent ownership refuses with bounded
categories and Repair guidance. Disabled retains records/checkpoint and may
retain extra conservative protection after an interrupted release.

Authoritative handle and authenticated RPC set/clear refuse the reserved BASIC
identity even when absent. Public whole-map replacement can preserve only the
exact fresh reserved entry under the shared publication guard; forged, omitted,
changed or stale entries refuse. Ordinary unrelated named CRUD still persists
through reopen. Authentication remains before parsing/dispatch and does not
grant index ownership. These are existing prune-lock routes, not filter RPCs.

Manual application reloads validated protection per candidate. Intent creation,
concrete paired deletion and resumed intent use fresh owned protection under
publication-before-payload synchronization. Applicable release requires recovered
coins B, compatible current durable ancestry, the saved fence/checkpoint and the
candidate height/hash bound to that ancestry. A caller snapshot cannot authorize
deletion. Required candidates skip at application; direct unauthorized deletion
refuses before usage, intent or payload effects, including when mates are absent.
Only `DeletedLiveMate` earns receipts, cache eviction and undo cleanup. Unsupported
generic protection snapshots refuse; known transient no-index stores are explicit.

Opaque work is bound to the store incarnation, exact lifecycle/generation,
checkpoint/fence and effective protection. Both record-only and checkpoint
writers reject stale same-generation frontier, changed branch, foreign reopened
incarnation and Disabled work. Ahead rows alone never release input. Complete
immutable-prefix/projection proof plus the current recovered coins/metadata fence
precedes atomic SyncAll publication of rows, projection, checkpoint and protection.
Before-commit failures retain conservative proof; an after-commit failed reply may
reopen with the complete new proof. Ambiguous failures poison live guarded work
and deletion until actual reopen rather than pretending disk rolled back.

Trusted parameter-free Rust host disable durably advances Disabled generation
before a second SyncAll batch releases the reserved lock. It preserves history
and ordinary locks; retry with retained protection is idempotent. Re-enable
preflights every required retained body and non-genesis undo, reconciles against
the current recovered fence and atomically acquires Active ownership/protection
before work can be minted. Missing history refuses without acquisition. No
worker scheduler or worker join/stop behavior is inferred from this lifecycle.

Automatic reuse and the independent Periodic timer compare fresh normalized
ownership before either gate. Same-second clone progress, disable/re-enable and
ordinary lock changes invalidate both gates; order-only changes still coalesce.
Required heights are excluded from candidate sizes while their bytes remain in
total usage. A stalled index may keep the logical 550 MiB soft target unattainable
without forcing an unnecessary coins checkpoint. The actual offline daemon test
starts at 578,571,326 bytes; after explicit trusted disable it deletes 714 paired
heights, earns one batch and retains 578,420,026 bytes. Its 235 nonactive bulk pairs
still account for 578,358,970 bytes. Actual encoded body/undo sizes supply these
totals; no lowered target or production accounting override supplies the evidence.

The dense daemon and sparse deletion histories are codec-valid ancestry and
historical-correspondence fixtures, not consensus-accepted full daemon chains.
Separate small `ValidatedHistory` controls stage real engine spends/forks with
coinbase maturity one and synthetic supplied chain work. Software fault tests
drop all handles and reopen the actual store; they do not establish hardware
power-loss resilience, exhaustive migration safety or archive-scale performance.
The official Fjall shutdown pin and matching format-marker observations are
documented in the repair artifact; it does not adopt every later upstream fix.

The three v2.4 advisories remain: stale durable metadata at interrupted prune
may refuse Repair; the generic paired-unlink no-op default remains separate from
unsupported protection refusal; the separate support-summary crash undercount
can lose evidence after a durable delete. No repair, redownload, source datadir
migration, service/config mutation or secret migration is implied here.

Public configuration/activation and scheduled catch-up (157), runtime reorg (158),
filter/index RPC (159), peers (160), operator projections (161), complete
client-after-prune proof (162), V0, BIP37, GUI, production readiness and
production-funds claims remain deferred.

```bash
bun test ./scripts/check-phase156-prune-coordination.test.ts
bun run scripts/check-phase156-prune-coordination.ts
bun run scripts/command-timings.ts run --key phase156-cross-boundary-node -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib prune
bun run scripts/command-timings.ts run --key phase156-cross-boundary-rpc -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib prune
bun run scripts/command-timings.ts run --key phase156-daemon-prune-protection -- cargo test --locked --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind index_protection -- --nocapture
```

Structural checks supplement actual behavioral tests: named ordinary production
bodies, registered test functions, parity roots, breadcrumbs and current scoped
claims are mutation-tested source contracts, not execution or safety proof.
Default `bash scripts/verify.sh` runs the Phase 156 checker/tests after Phase 155;
root must complete formatting, strict Clippy, builds/tests/doctests, coverage,
Bazel/provenance, final source/security/lifecycle review and Git finalization.

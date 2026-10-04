# BASIC compact filter construction

Phase 154 implements pure BASIC construction and commitments for CFIL-01 and
CFIL-02 against Bitcoin Knots `29.3.knots20260210`, exact commit
`a9aee730466ac67d35a3c03ee24676be5e045878`. [Phase verification](../../../.planning/phases/154-basic-generation-and-commitment-parity/154-VERIFICATION.md)
passed all 11 must-haves and three roadmap criteria; the full native verifier,
including coverage and Bazel smoke, passed on 2026-10-04.

This scope is BASIC-only: V0 and BIP37 serving are excluded. Filter index
activation, ordinary prune retention of filters, RPC serving, peer serving, catch-up,
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
storage and startup foundations. Its formal phase/full native gate remains
pending; the v2.5 milestone is not complete. The unique parity owner is
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

Phase 156 owns ordinary prune ownership, lock CRUD and disable/re-enable;
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

---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T20:40:16Z
measurement_started_at: 2026-10-05T20:36:57.917000Z
measurement_ended_at: 2026-10-05T20:37:23.326000Z
---

# Phase 157 complete bounded turn measurements

The actual configured recovered runtime executes one first turn. Its ordinary parameter-free handle method executes subsequent turns under the same authority and consumes the budgeted opaque append preparation/completion. Measurements below include the whole admitted driver ledger, with the adapter's 5,367 checkpoint reservation plus 48 admitted undo structural examinations for eight spending blocks. No full history vector, recovery, metadata reload, full-prefix projection or ancestor reader enters the turn.

## Reproduction and provenance

```bash
PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH bun run scripts/command-timings.ts run --key phase157-turn-measure-final -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase157_measure_turns -- --ignored --nocapture
```

Actual command: **2026-10-05T20:36:57.917Z–20:37:23.326Z**, wrapper duration **25,409 ms**, exit **0**; **1 registered measurement test passed**, harness execution **18.65 s**. Rust **1.94.1**, Bun **1.3.9**, macOS **26.6.2**, arm64, pinned Fjall **3.1.10**. Timing metadata identifies base commit `26bc454a66d6d7a241fc01be84dc6fbf0416d15a` with uncommitted Phase 157 implementation; root owns final Git provenance. Local detailed output was captured at `/tmp/phase157-turn-final-measure.log`; reproducible results are recorded here rather than relying on that temporary file.

`TurnHistory` starts with the existing genuinely staged/committed genesis, funding and historical plus same-block spending fixture, then continuously validates every successor. Each later block spends the preceding reward and a same-block output; its final unspent output carries an 8,192-byte script. Fixtures use easy synthetic proof-of-work headers and test-only coinbase maturity one. They use actual Fjall stores, full consensus stage/commit, real durable coins/metadata, and actual drop/reopen; they are not public-mainnet histories. Prefixes contain 16, 128 or 512 records and have 32 still-required successor blocks.

“Cold” means an actual closed database/runtime reopening, not an OS page-cache purge. “Warm” is a subsequent closed database/runtime reopen after catch-up. Startup totals include full recovery and required-history preflight, loaded baseline chain/undo ownership and the first bounded turn; they cannot establish bounded total startup latency.

## Observed times

All figures are microseconds, rounded down by `Duration::as_micros`. Authority hold includes decoding, body binding, generation, capability validation and synchronous publication. Storage time is the complete append completion call, including its checks and SyncAll batch, not isolated hardware fsync latency.

| Prefix | Cold configured open total | First turn hold / storage | Subsequent holds (three turns) | Subsequent storage (three turns) | Idle hold / storage | Warm reopen total |
| ------ | -------------------------: | ------------------------- | ------------------------------ | -------------------------------- | ------------------- | ----------------: |
| 16     |                     56,423 | 9,319 / 5,042             | 9,144; 8,837; 9,129            | 4,308; 4,369; 4,829              | 288 / 64            |            29,901 |
| 128    |                    110,891 | 11,094 / 6,585            | 8,672; 8,628; 9,106            | 4,546; 4,346; 4,966              | 256 / 58            |            85,507 |
| 512    |                    319,583 | 8,952 / 4,637             | 8,602; 9,071; 9,461            | 4,146; 4,827; 5,226              | 296 / 65            |           287,982 |

External subsequent-call elapsed times were 9,148/8,842/9,135; 8,678/8,635/9,113; and 8,609/9,076/9,466 µs respectively. The first turn is observed inside the actual configured-open call; its external total is inseparable from the separately reported startup total. No zero elapsed observation is invented.

## Complete ledger and actual operation observations

| Work / observation                                     |    First 8-block turn | Subsequent 8-block turn | Caught-up idle turn |
| ------------------------------------------------------ | --------------------: | ----------------------: | ------------------: |
| Actual selected blocks / generations                   |                 8 / 8 |                   8 / 8 |               0 / 0 |
| Actual native body reads / allocating decodes          |                 8 / 8 |                   8 / 8 |               0 / 0 |
| Actual borrowed undo objects / undo wire reads in turn |                 8 / 0 |                   8 / 0 |               0 / 0 |
| Body wire bytes read                                   |         67,680–67,688 |           67,680–67,688 |                   0 |
| Logical borrowed undo bytes                            |                 1,360 |                   1,360 |                   0 |
| Actual indexed point reads                             |                    31 |                      40 |                  18 |
| Record operations, including native body probes        |                    55 |                      70 |                  20 |
| Projection operations                                  |                    30 |                      36 |                   8 |
| Checkpoint / map / undo structural reservation         |                 5,415 |                   5,415 |               3,231 |
| Logical clone/allocation reservation                   |   4,505,961–4,506,260 |     4,507,026–4,507,324 |         5,180–5,184 |
| Script item-work reservation                           |             1,936,752 |               1,936,752 |                   0 |
| Script byte-work reservation                           | 17,397,872–17,399,928 |   17,397,872–17,399,928 |                   0 |
| Actual examined scripts / script bytes                 |           40 / 65,568 |             40 / 65,568 |               0 / 0 |
| Actual unique hashed / sorted elements                 |               16 / 16 |                 16 / 16 |               0 / 0 |
| Actual encoded record envelopes                        |           1,411–1,415 |             1,412–1,416 |                   0 |
| Actual effect batch key+value bytes                    |           2,811–2,815 |             2,812–2,816 |                   0 |
| Actual persistence batches                             |                     1 |                       1 |                   0 |

Indexed reads count actual BASIC state/owner/record/projection probes; body reads are counted independently at their native caller. H/B and variable lock-map checks are separately charged in the complete ledger. They are not silently included in the indexed-read observation. Record/projection operations combine reads, parsing/commitment validation, edge checks, encodes and writes; they are not presented as a measured filesystem syscall count. Undo bytes describe already-owned decoded facts borrowed from the baseline chainstate, not fresh turn-side disk reads or history copies.

Logical allocation and CPU reservations are deliberately conservative software limits, not RSS, allocation-profiler observations, measured comparison counts or hard latency limits. Before even the wire count scanner, a borrowed native length probe reserves `32 × wire_bytes + size_of(Block)` decoded storage, `wire_bytes / 9` possible output items and `wire_bytes` possible output-script bytes. The scanner then validates canonical counts against remaining minimum envelopes before the existing allocating decoder. Authoritative undo vector-length passes and restored-script length passes are separately admitted/charged before examination.

Before generation, the ledger reserves txid/body and merkle buffers, script-reference vectors/BTree nodes, mapped values, GCS buffers/output copies and identity vectors. Item/byte work uses a conservative 256-fold comparison allowance in addition to the examined bound. This is an admission metric over bounded data, not a claim to have counted every stdlib comparison. Unique hashed/sorted element counts come from the achieved canonical filter count, and examined source counts come from actual scanner/undo facts. No allocator or comparison profiler was used.

GCS output admission uses `4 × prospective_items + 179` bytes, including the 170-byte immutable envelope. For BASIC P=19 and M=784931, total quotient bits are bounded by the final mapped range; quotient, terminator and remainder bits total less than 22 per item. Four bytes per item plus the maximum nine-byte CompactSize count safely reserve the encoded output before sorting/hashing. Actual achieved envelopes are still counted only once by the append adapter.

The Plan 05 variable operator-map reservation remains `1024 × N × (N+1)/2 + E + N + 1` work, with `E + N × (4 × size_of(PruneLockInfo) + 1024)` allocation reservation. The private append acquisition, preparation and completion share one ledger. The capability is acquired once before inputs using the componentwise maximum of the normal and singleton policies. Its captured acquisition cost participates in every admission decision. After selecting input/generation work, `with_remaining_budget` only decreases each cap to the selected policy minus driver cost and refuses budget inflation or loss of captured work. Capability identity is unchanged. Acquisition is removed once from the driver-only ledger and remains once in achieved append accounting; selected blocks and actual encoded output also enter the merged outcome exactly once.

## Chosen production policy and legal singleton evidence

The normal policy preserves eight-block turns observed above and leaves explicit headroom over the representative workload. These are software admission budgets consumed by the actual ordinary method, with a single oversized candidate ending its turn.

| Counter                        | Normal aggregate |            Absolute singleton |
| ------------------------------ | ---------------: | ----------------------------: |
| Blocks                         |                8 |                             1 |
| Body logical/wire bytes        |            1 MiB |                   128,000,000 |
| Undo logical bytes             |            4 MiB |                       256 MiB |
| Copy/allocation reservation    |           16 MiB |                         1 GiB |
| Item-work reservation          |       4 Mi units |                  128 Mi units |
| Byte-work reservation          |      32 Mi units |                   64 Gi units |
| Encoded record envelopes       |            1 MiB | `MAX_SIZE + 170` = 33,554,602 |
| Record operations              |              512 |                           512 |
| Checkpoint/map/structural work |        1,000,000 |                     1,000,000 |
| Projection operations          |              256 |                           256 |

The independent hard adapter envelope ceilings remain 128 candidates and `MAX_SIZE + 128 × 170` aggregate bytes. The body adapter additionally rejects any native wire row above **4,000,000 bytes** before scanning/allocating. A first candidate above its absolute bound explicitly refuses/pauses; it never yields an endless empty turn. Later aggregate saturation yields before allocating/decoding/hashing the next body. Dynamic operator-map exhaustion is an explicit conservative refusal; existing operator CRUD and schema are unchanged.

The legal singleton fixture adds **99 additional 9,985-byte outputs** to a genuinely consensus-staged/committed block. Its exact wire size is **989,871 bytes**, below the 1,000,000-byte stripped-body ceiling and four-million weight ceiling; all scripts are within the existing 10,000-byte bound. Production defaults admitted it as one oversized singleton and reached the accepted target:

| Observation                                                |            Actual result |
| ---------------------------------------------------------- | -----------------------: |
| Authority hold / storage completion                        |        43,230 / 5,098 µs |
| Blocks / body reads / decodes / undo borrows / generations |        1 / 1 / 1 / 1 / 1 |
| Examined scripts / script bytes                            |            104 / 988,520 |
| Unique hashed / sorted elements                            |                    2 / 2 |
| Body wire / logical undo bytes                             |            989,871 / 170 |
| Copy/allocation reservation                                |               65,560,780 |
| Item-work / byte-work reservation                          | 28,266,659 / 254,397,361 |
| Record / checkpoint / projection operations                |           20 / 5,373 / 9 |
| Indexed point reads                                        |                       17 |
| Encoded envelope / effect batch bytes / batches            |            177 / 485 / 1 |

Absolute input ceilings use existing legal representation limits: weight ≤4,000,000, stripped body ≤1,000,000, minimum input wire 41 bytes and output wire 9 bytes, and scripts ≤10,000 bytes. At most `floor(1,000,000 / 41)` historical spent scripts can contribute less than 244 MB of script payload plus decoded container/coin overhead; 256 MiB bounds this logical undo. The row-length preflight conservatively permits up to `4,000,000 / 9` prospective output slots plus actual authoritative undo counts, explaining the deliberately larger comparison/admission ceilings even though witness bytes cannot all be outputs. BASIC fact capture strips witnesses while preserving every txid-bearing field, header/merkle binding and genuine staged undo, so witness Vec expansion is not copied into retained live facts. Its combined retained allocation cap is now **384 MiB**, retaining the existing one-million raw-script-item cap; these are explicit adapter bounds, not measured default memory use.

The 10,000-byte `ScriptBuf` limit is the existing first-party primitive/codec representation limit; it is not asserted to be the maximum of every Bitcoin-consensus-legal output script. This plan preserves that existing representation contract and tests a genuine legal large payload within it. It does not claim indexing support for every otherwise legal Bitcoin block outside that representation/validation surface. Historical spent scripts also obey the validated executable-script bound; current coins never replace them.

The measured singleton proves this concrete legal payload can progress. The arithmetic bounds prevent blind claims about every imaginable payload or allocator: turn policy can still refuse an excessive variable operator map, corrupted/untrusted stored envelope, incompatible authority, unsupported validation surface or exhausted resource bound. This report makes no all-payload, hard fsync-latency, hardware power-loss, public-mainnet, archive-scale or production-readiness claim.

## Deterministic evidence and release cadence

The ordinary suite uses production defaults and lower injected limits for exact/one-under body and output admission, item yields and absolute refusal. It checks ordered first/multiple/idle turns, actual prefix-independent operations at 16/128/512, full-ancestry positive detection, malformed pre-decode counts, mutation pause/same-authority resume, lifecycle invalidation, accepted writer-error retained facts, accepted-unflushed indexing, genuine later flush/no-record release, stronger effective protection and actual append faults/reopen. Exact final suite/regression counts are in `157-07-SUMMARY.md`.

Safe publication releases only the exact authenticated durable tip. Intermediate processed suffixes retain the previous checkpoint and actual stronger covering lock; sustained durable growth while behind can prolong retention. The normal Periodic/Always coins owner remains responsible for coins durability and automatic deletion. The index turn never forces that flush or deletes payloads. The serialized generation implementation has bounded admitted work and measured hold times above; those observations cannot promise an upper wall-clock duration. Ordinary daemon timer wiring remains Plan 08.

# Stack Research

**Domain:** Open Bitcoin v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)
**Researched:** 2026-10-03
**Confidence:** HIGH for source behavior and existing stack; MEDIUM for proposed integration

## Recommendation

Add **no third-party production dependencies and no new workspace crate**. Extend existing pure-core, codec, network and Fjall adapters with arbitrary-byte SipHash-2-4, a small Golomb–Rice codec, BASIC generation/commitments, durable index records and bounded serving.

This is planned capability. Keep indexing and P2P serving explicitly enabled and the shipped v2.4 single-chainstate prune contract intact. BIP37, Knots V0 filters, assumeutxo, archive product modes, public defaults, public-network CI and production claims remain outside scope. `.planning/PROJECT.md` is the current scope authority; historical runtime prose in `.planning/STACK.md` must not override the shipped milestone record.

Material guidance: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` (no substantive exception), and architecture, verification and Rust standards require first-party Bitcoin ownership, pure decisions with shell effects, Bun automation and the existing serialized authority. Context7 was unavailable; pinned source and version-specific official documentation supplied primary evidence.

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
| --- | --- | --- | --- |
| Rust | `1.94.1`, edition `2024` | Typed filters, index transitions and serving policy | Existing pins; `u128` supports exact multiply-high range mapping without a big-integer dependency. [S1] |
| First-party workspace | `0.1.0` | Domain types, hashing, serialization and policy | Extend existing modules; preserve owned Bitcoin implementation. [S1, S2] |
| Bitcoin Knots | `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878` | Parity oracle | Pinned implementation and fixtures settle construction and index/prune behavior. [S3–S8] |
| Fjall | Existing `3.1.4`, `default-features = false` | Filter bytes, commitments, branch mapping and cursor | Existing durable store; atomic batches span keyspaces in one database. [S9, S10] |
| Bazel/Bzlmod | Bazel `8.6.0`, `rules_rust 0.69.0` | First-party smoke builds | Keep existing Cargo-derived graph and Rust toolchain alignment. [S1] |
| Bun | `1.3.9` | Parity and release-boundary automation | Canonical existing runtime; no package-install bootstrap. [S1] |

### Supporting Facilities

| Facility | Version | Reuse / required change |
| --- | --- | --- |
| First-party SHA256d | Workspace `0.1.0` | Reuse `crypto::double_sha256` for filter hash/header. [S2] |
| First-party SipHash | Workspace `0.1.0` | Extend `siphash_uint256`, currently Wtxid-only, with arbitrary-byte SipHash-2-4. Preserve BIP152 outputs. [S2, S3] |
| First-party CompactSize | Workspace `0.1.0` | Reuse canonical bounded serialization; add message-specific bounds before allocation. [S2] |
| Bit/Golomb codec | New first-party code | No first-party implementation was found in the scoped Rust search; implement a small bounded bit reader/writer and Golomb–Rice routine. [S3] |
| Standard collections | Rust `1.94.1` | A bounded `BTreeSet` of raw scripts plus sorted `Vec<u64>` is sufficient. Deduplicate scripts before hashing; preserve mapped collisions. [S3] |
| serde / serde_json | Existing `1.0.228` / `1.0.149` | Small versioned metadata, operator status and support evidence; keep filters and P2P binary. [S1] |

### Module Ownership

| Owner | Proposed change | Boundary |
| --- | --- | --- |
| `open-bitcoin-primitives` | Distinct BASIC type/filter hash/filter header types where useful | Keep raw bytes distinct from display hex and block hashes. |
| `open-bitcoin-codec` | Bit codec, filter serialization, BIP157 payload shapes | Codec depends only on primitives; do not import consensus hashing and create a cycle. [S2] |
| `open-bitcoin-consensus` pure modules | Byte SipHash, element hashing, generation and commitments | Consume validated spent-script facts supplied by chainstate, without adding a consensus-to-chainstate dependency for `BlockUndo`. Derived filters do not become consensus validity rules. [S2, S5] |
| `open-bitcoin-chainstate` | Pure progress/reorg/protection facts | Existing undo supplies spent scripts; persistence remains in adapters. [S5] |
| `open-bitcoin-node` | Durable index, catch-up/recovery, branch selection, serving reads | Reuse the same Fjall database and serialized prune/runtime authority. [S9, S12] |
| `open-bitcoin-network` | Request policy, service bit, framing integration | Existing service flags omit compact filters; `blockfilters` is currently inactive evidence. Add behavior before advertising support. [S13] |
| Existing RPC/CLI/dashboard/support | Enabled/catching-up/ready/missing-history/failure evidence | Derive status from persisted authority; no new dashboard or server framework. |

## Algorithm Contract

Use BASIC `0`, `P = 19`, `M = 784931`; SipHash keys come from the first 16 raw little-endian block-hash bytes. Range mapping is the high half of the 128-bit product, not modulo. Encode sorted deltas as unary quotient and big-endian remainder, prepend CompactSize element count, zero-pad the final byte. Empty encoding is `[0]`. [S3, S14]

The set includes nonempty output scripts except those starting with `OP_RETURN`, plus nonempty spent-prevout scripts. Do not apply the output exclusion to spent scripts. Knots uses block undo for spent scripts; current UTXOs cannot supply coins already spent. Genesis needs no undo. [S3, S5]

Hash serialized filter bytes with SHA256d. Header is SHA256d(filter hash || previous header); genesis predecessor is 32 zero bytes. Hash raw bytes, not display hex/reversed encodings. [S3, S11]

## Durable Index and Prune Integration

Recommend a dedicated filter keyspace inside the existing Fjall database: versioned binary records by `(type, block hash)`, an active height mapping, and cursor `(height, block hash, header)`. This is a proposed schema. Keep filters independently readable after paired block/undo deletion and account their growth separately from v2.4's soft block/undo retention target. [S9]

Commit bytes, hash/header and active mapping durably, while distinguishing indexed progress from the safe restart cursor constrained by the durable chainstate checkpoint. Advance prune protection only from that safe prefix. If protection is a separate commit, persist the safe filter checkpoint first and protection second: a crash may retain extra history but must never release required history. Install protection under the shared authority before catch-up or startup prune recovery can delete inputs. Provide an adapter method for this transition rather than composing unlocked public calls. Cross-keyspace atomic batches support this design, but application crash/reopen proof is still required. [S9, S10, S12]

Reserve internal index protection from ordinary operator deletion, or require a safe index-disable transition before releasing it. Existing generic lock CRUD alone does not prove that invariant. Reuse the buffered prune policy and reload locks under the deletion owner. Generic sink defaults and accepted interrupted-prune advisories must not yield a success cursor without concrete filter persistence. [S12]

Keep completed records by block hash across reorgs; atomically rewind the active mapping/cursor/header. Knots preserves disconnected filters under hash keys before height overwrite. Requests follow the validated stop hash's ancestry; permitted stale branches require a deliberate parity decision rather than height-only lookup. [S6, S7]

**Already-pruned bootstrap:** refuse fresh or lagged indexing when required historical block/undo inputs are absent. Keep serving unready, report missing history, and never silently start a suffix with a zero predecessor. Knots refuses catch-up beyond pruned block data; append also requires undo. Reindex/redownload can be explained as future operator work, but do not automatically mutate or redownload the datadir. Retained snapshots/current coins cannot reconstruct every historical spent script. [S5, S8]

## Serving Contract and Bounds

Reuse peer transport/framing for all six BIP157 messages. Cap inclusive ranges at 1,000 filters and 2,000 hashes. Checkpoints are at heights 1,000, 2,000, etc., excluding genesis. Budget storage reads, queued bytes and checkpoint count before allocation; protocol caps alone are not a complete DoS budget. [S7]

`cfilter` carries a CompactSize byte-vector length around filter bytes, which themselves begin with CompactSize element count. `cfheaders` carries the preceding header and filter hashes. Keep these counts/shapes distinct. [S11]

Index enablement and serving enablement are separate. Knots requires BASIC indexing for `-peerblockfilters`, advertises bit `1 << 6` at initialization and checks explicit `blockfilters` permission against the index. Its request preparation disconnects unsupported types/invalid ranges or hashes; the BIP permits ignoring some of these. Match pinned behavior where in scope. [S4, S7]

**Selected service semantics:** match Knots' enablement-based advertisement after valid activation; advertise configured BASIC capability without claiming initial catch-up is complete. Serve entirely available indexed ranges during catch-up, and report index progress separately. Explicit `blockfilters` permission grants capability only to that peer when the BASIC prerequisite holds. Missing records or storage failure cannot produce an invented successful response. The body-serving 288+2 window must not restrict durable filter reads: limited-body and filter service bits describe separate availability. [S4, S13]

## Development Tools and Verification

| Tool / fixture | Purpose |
| --- | --- |
| Pinned `src/test/data/blockfilters.json` and `blockfilter_tests.cpp:199–244` | Exact filter-byte and header parity; compare commitments, not only membership. [S15] |
| Pinned `src/test/crypto_tests.cpp` SipHash vectors | Tail/block-boundary byte hashing and preservation of BIP152 behavior |
| Pinned `test/functional/p2p_blockfilters.py` | Range/disconnect, stale branch and enablement oracle [S7] |
| Pinned `test/functional/feature_index_prune.py` | Filter reads after prune, disabled-index catch-up refusal, restart/reorg protection [S8] |
| Existing first-party harness/benchmarks | Deterministic generation, durable faults, prune/reorg/restart/serving and bounded-resource evidence |
| `bash scripts/verify.sh` | Final implementation contract; root owns verification/commit hooks for this research task |

No Cargo jobs or new tests were run for this document-only research. Future ad-hoc Cargo/Bazel jobs must use `scripts/command-timings.ts`; public-network checks remain opt-in.

## Installation

No new installs/dependencies are recommended. Preserve existing manifests and lockfile. There is no `package.json` or `bun install` step. If the pinned reference is absent, use the existing bootstrap:

```bash
git submodule update --init --recursive
```

## Alternatives Considered

| Recommended | Alternative | When Alternative Would Fit |
| --- | --- | --- |
| Owned byte SipHash | `siphasher` | Only after deliberate ownership-policy change and maintenance/security review; unnecessary here |
| Small owned Golomb codec | Generic bit-stream crate | Many unrelated binary formats justify broader abstraction; this fixed format does not |
| Same-database Fjall keyspace | LevelDB metadata plus flat filter files | Direct Knots storage interoperability or proven scale needs, outside v2.5 |
| Existing serialized owner | Independent index/deletion owner | Profiling may justify parallel preparation later; durable branch/protection authority must remain shared |
| Missing-history refusal | Suffix-only index or foreign filter downloads | Requires separate partial-service/trust scope; cannot silently satisfy full BASIC parity |

## What NOT to Use

| Avoid | Reason | Use Instead |
| --- | --- | --- |
| Rust Bitcoin/BIP158 production libraries | Conflicts with domain/dependency policy | Owned modules and verified fixtures |
| `DefaultHasher` | No required explicit SipHash-2-4/key contract | Owned keyed byte SipHash |
| BIP152 masked short IDs | Wrong key derivation and only 48 output bits | Full 64-bit hash with block-hash keys |
| Per-request generation | Undo loss after pruning and request cost amplification | Durable once-per-block generation |
| `Pruned` for absent filters | Deleted bodies do not imply deleted filters | Separate filter readiness/availability |
| Knots V0/type `2` | Locally supported upstream but not selected here; P2P preparation accepts BASIC | BASIC-only scope |

## Version Compatibility and Open Decisions

| Boundary | Recommendation / remaining decision |
| --- | --- |
| Rust/Cargo/Bazel | Keep Rust `1.94.1`/edition 2024 and current `rules_rust`; no upgrades |
| Fjall | Use 3.1.4 `Database::batch`, `durability(Some(SyncAll))`, atomic `commit`; do not copy 2.x partition APIs. [S10] |
| Dependency graph | Codec cannot import consensus; pure generation consumes spent-script facts without importing chainstate. Final module placement needs phase design. [S2] |
| Existing store schema `2` | Decide additive namespace/envelope versus global schema bump and reopen compatibility before writes. [S9] |
| Readiness/advertisement | Selected: Knots enablement-based bit, available-range serving during catch-up and separate progress evidence; verify lag/reorg/failure/existing-peer cases |
| Missing history | Explicit refusal recommended; restoration is separate future work |

## Sources

Inspected 2026-10-03. Local paths are repo-relative; line citations refer to the pinned checkout. HIGH marks source-confirmed behavior; proposed layouts/transition protocols are MEDIUM until verified. Official BIPs are deployed specifications assigned in 2017; assignment age does not invalidate their normative rules.

- **S1 — HIGH:** `rust-toolchain.toml:2`; `packages/Cargo.toml:19–25`; `MODULE.bazel:3–16`; `.bazelversion:1`; `.bun-version:1`; `packages/open-bitcoin-node/Cargo.toml:12–23`; `packages/Cargo.lock:678–679` — existing pins.
- **S2 — HIGH:** `packages/open-bitcoin-consensus/src/crypto.rs:22–31,54–56`; `crypto/siphash.rs:12–87`; `packages/open-bitcoin-codec/src/compact_size.rs:8–60`; consensus/codec manifest dependency sections — reusable APIs and fixed-size SipHash limitation. Scoped first-party searches found no Golomb/bit-stream or filter implementation.
- **S3 — HIGH:** `packages/bitcoin-knots/src/blockfilter.cpp:27–43,75–102,187–224,258–282`; `blockfilter.h:89–96`; `util/golombrice.h:14–42`; `streams.h:270–364`; `crypto/siphash.cpp:62` onward — generation, keys, constants, bit encoding and commitments.
- **S4 — HIGH:** `packages/bitcoin-knots/src/init.cpp:1075–1104,2358–2361`; `src/net_processing.cpp:3156–3206` — configuration, permission and service dependencies.
- **S5 — HIGH:** `packages/open-bitcoin-chainstate/src/types.rs:15–40`; `packages/bitcoin-knots/src/index/blockfilterindex.cpp:268–285` — undo data and genesis case.
- **S6 — HIGH:** `packages/bitcoin-knots/src/index/blockfilterindex.cpp:336–370`; `src/index/base.cpp:430–446` — branch preservation and prune-lock ordering.
- **S7 — HIGH:** `packages/bitcoin-knots/src/net_processing.cpp:153–155,3215–3315`; `src/index/blockfilterindex.h:20`; `test/functional/p2p_blockfilters.py:49–50,109–124,265–273` — caps/checkpoints/serving behavior.
- **S8 — HIGH:** `packages/bitcoin-knots/src/init.cpp:2485–2496`; `test/functional/feature_index_prune.py:90–93,123–155` — retained filters and missing-history catch-up refusal.
- **S9 — HIGH:** `packages/open-bitcoin-node/src/storage.rs:28–61`; `storage/fjall_store.rs:59–95`; `storage/fjall_store/prune.rs:79–103` — schema/keyspaces and durable-delete pattern. [Fjall 3.1.4 Database](https://docs.rs/fjall/3.1.4/fjall/struct.Database.html).
- **S10 — HIGH:** [Fjall 3.1.4 OwnedWriteBatch](https://docs.rs/fjall/3.1.4/fjall/struct.OwnedWriteBatch.html) — atomic cross-keyspace writes/durability selection. Cached published `fjall-3.1.4/src/journal/writer.rs:35–49` confirms SyncAll data/metadata `fsync`; no hardware-failure guarantee inferred.
- **S11 — HIGH:** [Official BIP157](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki), Filter Headers and New Messages — commitments/wire shapes, checked against S3/S7.
- **S12 — HIGH existing / MEDIUM proposal:** `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs:60–115`; `automatic_prune.rs:132–173`; `storage/fjall_store/prune.rs:211–244` — owner/lock behavior and recovery refusal; combined filter persistence is proposed.
- **S13 — HIGH:** `packages/open-bitcoin-network/src/message.rs:39–47`; `limited_serve.rs:19–35`; `inbound/permissions.rs:157–164` — existing flags/body policy/inactive permission.
- **S14 — HIGH:** [Official BIP158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki), Hashing Data Objects, Golomb–Rice Coding, Contents, Construction and Signaling — checked against S3. [Official test vectors](https://github.com/bitcoin/bips/tree/master/bip-0158).
- **S15 — HIGH:** `packages/bitcoin-knots/src/test/blockfilter_tests.cpp:199–244`; `src/test/data/blockfilters.json` — exact filter/header assertions.

*Active v2.5 research supersedes v2.4 stack guidance; historical evidence remains in Git.*

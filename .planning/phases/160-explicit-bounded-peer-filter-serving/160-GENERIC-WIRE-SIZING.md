---
phase: 160-explicit-bounded-peer-filter-serving
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T21:23:57Z"
artifact_type: planning-research-supplement
---

# Generic Wire Sizing Source Audit

This parent-workflow planning supplement records facts from a scoped read-only audit of the existing network serializers, primitive accessors and checksum implementation. No production code, tests or plans were changed during that audit, and no Cargo verification was run. Plan14 owns implementation and independent serializer-equality evidence; Plans09/10 own actual transport integration. These formulas describe logical simultaneously retained heap-buffer lengths, not allocator capacity or RSS.

## Checked Building Blocks

Use checked addition, multiplication and integer conversion throughout, rejecting overflow and the local 4,000,000-byte payload envelope before allocation.

- `C(n) = 1` for `n <= 252`, `3` for `n <= 65,535`, and `5` otherwise **only through the codec's `MAX_SIZE = 33,554,432`**. Larger values return `CompactSizeTooLarge`. The existing writer never emits a nine-byte CompactSize. Source: `packages/open-bitcoin-codec/src/compact_size.rs`, `write_compact_size`.
- `B(s) = C(s.len()) + s.len()` for borrowed byte slices.
- `T(tx) = 8 + C(inputs.len()) + sum[40 + B(script_sig)] + C(outputs.len()) + sum[8 + B(script_pubkey)]`, plus `2 + sum[C(stack.len()) + sum B(item)]` if any input has a nonempty witness stack. Source: `packages/open-bitcoin-codec/src/transaction.rs`, `encode_transaction`, `encode_inputs`, `encode_outputs`, `encode_witness`.
- A stack containing one empty item activates witness. Empty stacks on the other inputs then each serialize their zero item-count byte. Witness activation follows stack nonemptiness, not whether any item contains bytes.
- Final frame length is `F = P + 24`, where `P` is payload length. Source: `packages/open-bitcoin-network/src/message.rs`, `WireNetworkMessage::encode_wire`.

## Exhaustive Variant Payload Formulas

| Outgoing variant | Exact payload length `P` | Source path and serializer |
| -- | -- | -- |
| `Version` | `85 + C(user_agent.as_bytes().len()) + user_agent.as_bytes().len()` | `packages/open-bitcoin-network/src/message.rs`, `encode_version_payload` |
| `Verack`, `WtxidRelay`, `SendHeaders`, `GetAddr` | `0` | `packages/open-bitcoin-network/src/message.rs`, `encode_payload` |
| `Ping`, `Pong` | `8` | `packages/open-bitcoin-network/src/message.rs`, `encode_payload` |
| `SendCompact` | `9` | `packages/open-bitcoin-codec/src/compact_block.rs`, `encode_send_compact_payload` |
| `GetHeaders` | `36 + C(locator.block_hashes.len()) + 32*n` | `packages/open-bitcoin-codec/src/network.rs`, `encode_block_locator`; `packages/open-bitcoin-network/src/message.rs` adds the stop hash |
| `Headers` | `C(n) + 81*n` | `packages/open-bitcoin-network/src/message.rs`, `encode_headers_payload`; each 80-byte header includes a zero transaction-count byte |
| `Addr` | `C(n) + 30*n` | `packages/open-bitcoin-network/src/message.rs`, `encode_addr_payload`; reject `n > PHASE92_ADDR_BATCH_LIMIT` |
| `Inv`, `GetData`, `NotFound` | `C(n) + 36*n` | `packages/open-bitcoin-network/src/message.rs`, `encode_inventory_payload`; reject `n > 50,000` |
| `Tx` | `T(tx)` | `packages/open-bitcoin-codec/src/transaction.rs`, witness encoding |
| `Block` | `80 + C(n) + sum T(tx)` | `packages/open-bitcoin-codec/src/block.rs`, `encode_block` |
| `CompactBlock` | `88 + C(short_ids.len()) + 6*s + C(prefilled.len()) + sum[C(index_delta) + T(tx)]` | `packages/open-bitcoin-codec/src/compact_block.rs`, `encode_compact_block_payload` |
| `GetBlockTxn` | `32 + C(n) + sum C(index_delta)` | `packages/open-bitcoin-codec/src/compact_block.rs`, `encode_get_block_transactions_payload` |
| `BlockTxn` | `32 + C(n) + sum T(tx)` | `packages/open-bitcoin-codec/src/compact_block.rs`, `encode_block_transactions_payload` |
| `GetCFilters`, `GetCFHeaders` | `37` | `packages/open-bitcoin-network/src/message/filter.rs`, `encode_ranged` |
| `GetCFCheckpt` | `33` | `packages/open-bitcoin-network/src/message/filter.rs`, `encode_checkpoint` |
| `CFilter` | `33 + C(filter.len()) + filter.len()` | `packages/open-bitcoin-network/src/message/filter.rs`, `CFilter::encode`; existing codec limit 4,000,000 |
| `CFHeaders` | `65 + C(n) + 32*n` | `packages/open-bitcoin-network/src/message/filter.rs`, `CFHeaders::encode`; existing codec limit 4,000,000 |
| `CFCheckpt` | `33 + C(n) + 32*n` | `packages/open-bitcoin-network/src/message/filter.rs`, `CFCheckpt::encode`; existing codec limit 4,000,000 |
| `MalformedCFRequest` | Always typed rejection | `packages/open-bitcoin-network/src/message.rs`, `encode_payload` returns `UnexpectedEof`; receive-only marker |

Here `n` is the relevant vector count and `s` is the short-ID count. Count user-agent UTF-8 bytes, not characters. Do not add an outgoing header-count restriction that the current encoder does not enforce.

There is no raw-block or raw-wire enum variant. All variable-length fields are accessible without cloning: `ScriptBuf::as_bytes()` and `ScriptWitness::stack()` in `packages/open-bitcoin-primitives/src/script.rs`; public transaction fields in `packages/open-bitcoin-primitives/src/transaction.rs`; public block, locator and CF vectors; and the six-byte `ShortId::as_wire_bytes()` accessor in `packages/open-bitcoin-codec/src/compact_block.rs`. There is no existing public exact transaction/message encoded-length helper.

## Actual Encoder Scratch and Checksum Peaks

| Encoder stage | Simultaneously retained logical heap-buffer lengths |
| -- | -- |
| Payload-only transaction/request/CF encoding | Up to `P` |
| Version address temporary | At most `max(P, 98)` |
| Headers temporary encoded header | `P + 79` when nonempty |
| Addr temporary encoded address | `P + 26` when nonempty |
| Inventory temporary encoded vector | `P + 36` when nonempty |
| Block/BlockTxn nested transaction temporary | Maximum over transactions of `prefix_before_tx + 2*T(tx)` |
| CompactBlock nested prefilled transaction | Maximum of `prefix_before_delta + C(delta) + 2*T(tx)` |
| CompactBlock structure validation | `10*prefilled_count`: collected `Vec<u64>` deltas and expanded `Vec<u16>` positions coexist before payload construction |
| GetBlockTxn validation | `2*n` expanded-index bytes, before payload construction |
| First checksum SHA256 | `P + D(P)`, where `D(P) = 64*ceil((P + 9)/64)` |
| Second checksum SHA256 | `P + 64`; the first digest is a stack array |
| Final frame construction | `P + F + L = 2*P + 24 + L`, where `L = command_name().len()` for the owned command string retained in `MessageHeader` |

The temporary address/inventory/header vectors are allocated by `packages/open-bitcoin-codec/src/network.rs` and `packages/open-bitcoin-codec/src/block.rs`, then appended by `packages/open-bitcoin-network/src/message.rs`. Nested transaction vectors coexist with their appended parent payload until the loop iteration ends in `packages/open-bitcoin-codec/src/block.rs` and `packages/open-bitcoin-codec/src/compact_block.rs`.

SHA256 allocates `bytes.to_vec()` and pads that vector in `packages/open-bitcoin-consensus/src/crypto/sha256.rs:93`. The two calls in `packages/open-bitcoin-consensus/src/crypto.rs:55`, `double_sha256`, are sequential. The first SHA padded buffer is dropped before the second SHA, and checksum padded buffers are dropped before final frame construction. Hash scratch and the final frame do not coexist. Implementation source review additionally identified `self.command()` calling `MessageCommand::new` and `command.to_owned()` after checksum calculation: that string remains owned by the header during frame construction and payload append. Count its logical length (at most12), rather than excluding it as descriptor metadata; this corrects the audit's original final-stage formula.

Reserve the maximum stage peak before calling the existing encoder; shrink to `F` only after `encode_wire` returns and its actual scratch has dropped. The listed payload-construction temporary peaks are bounded by the wire/checksum peak for valid structures, but implementation should retain explicit formulas or document that bound. Do not sum mutually exclusive SHA and frame stages.

- Empty messages: payload `0`, frame `24`, peak **64**.
- Ping/Pong: payload `8`, frame `32`, peak **72**.
- Empty-locator GetHeaders: payload `37`, frame `61`, command string `10`, peak **108**; final construction dominates its101-byte SHA stage.
- Payload `4,000,000`: frame `4,000,024`, SHA-padded input `4,000,064`, peak **8,000,064**, fitting the 8MiB aggregate limit.

An unconditional 4M scratch reservation would falsely reject tiny messages with otherwise sufficient CF headroom. Existing codec encoding may remain the one actual encoding operation; do not encode to measure or encode again to send.

## No-Allocation Structural Validation and Traversal Bounds

Do not call `validate_compact_block_structure`, `expand_prefilled_positions`, or `expand_block_transaction_indexes` during measurement: these helpers allocate vectors. They live in `packages/open-bitcoin-codec/src/compact_block.rs`. Reproduce their scalar checks narrowly:

- Cumulative differential indexes must fit `u16`, with checked delta addition and checked next-position increment.
- Compact total transaction count must be nonzero and at most 65,535.
- Each prefilled position must be at most `short_id_count + prefilled_ordinal`.
- Each prefilled transaction must have nonempty inputs and outputs.
- CompactSize inputs themselves must obey the existing codec `MAX_SIZE`, including index deltas.

The existing serializer still validates independently after reservation. Apply O(1) vector-count/minimum-size checks before traversing large inputs, then stop checked accumulation immediately above the local 4M payload envelope. Avoid an unbounded preliminary witness scan before rejecting obviously excessive transaction counts. Measurement only borrows raw scripts, witness items and field vectors.

## CF Factory Categorization

The generic guarded encoding factory must reject `CFilter`, `CFHeaders`, and `CFCheckpt`. Their formulas can remain available for independent codec tests, but production CF responses must cross `PreparedFilterServe` and its request-bound lease/receipt path. `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs` constructs those prepared CF responses. Generic encoding must not bypass prepared-request authorization or produce filter success receipts.

CF request variants can remain generic outgoing messages. `MalformedCFRequest` is never outgoing and always receives typed refusal.

## Independent Regression Evidence Required by Plan14

- Exhaustive enum coverage: measured `P` and `F` equal existing serializer lengths for each encodable variant.
- CompactSize transitions 252/253 and 65,535/65,536; reject `MAX_SIZE + 1` differential delta instead of assigning a nominal nine-byte width.
- Empty transaction vectors, multiple inputs with only one witness-bearing input, and a witness stack containing an empty item.
- Multibyte user agent; Addr limit/one-over; inventory 50,000/50,001.
- Compact empty/count overflow, differential overflow/out-of-bounds and null prefilled transaction.
- SHA payload lengths 55/56 and 63/64; Ping peak 72 and Verack peak 64 with exact available headroom/one-under.
- Genuine near-maximum valid witness block; independent codec-only 4M/one-over local-envelope controls.
- Nested transaction temporary accounting, including a large final transaction.
- All three malformed CF kinds rejected without encoding.
- Generic factory refusal for each CF response family; generic output never substitutes for a prepared filter request or receipt.

No implementation, socket-integration or passing-test claim is earned by this source audit alone.

# Core Domain and Serialization Foundations

This is the Phase 2 seed entry for `REF-03`. It catalogs the major domain and byte-level serialization surfaces that later consensus, chainstate, mempool, networking, wallet, RPC, and CLI work will reuse. The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Coverage

- `CAmount` and fixed-width identifier types
- shared serialization primitives and stream adapters
- script byte containers and script-size limits
- transaction, witness, and generic transaction-identifier handling
- block, block-header, and block-locator serialization
- P2P message framing and inventory identity boundaries

## Amounts and fixed-width identifiers

**Major features**

- `CAmount`, `COIN`, and the consensus-critical `MoneyRange` ceiling
- `uint160` and `uint256` as fixed-width opaque blobs
- txid and wtxid ordering implications that later code must not collapse away

**Knots sources**

- [`packages/bitcoin-knots/src/consensus/amount.h`](../../../packages/bitcoin-knots/src/consensus/amount.h)
- [`packages/bitcoin-knots/src/uint256.h`](../../../packages/bitcoin-knots/src/uint256.h)
- [`packages/bitcoin-knots/src/primitives/transaction.h`](../../../packages/bitcoin-knots/src/primitives/transaction.h)

**Knots tests and vectors**

- [`packages/bitcoin-knots/src/test/txpackage_tests.cpp`](../../../packages/bitcoin-knots/src/test/txpackage_tests.cpp)
- [`packages/bitcoin-knots/src/test/transaction_tests.cpp`](../../../packages/bitcoin-knots/src/test/transaction_tests.cpp)

**Quirks to preserve**

- `MAX_MONEY` is a consensus-critical sanity ceiling, not a statement of current supply; `MoneyRange` depends on that exact limit.
- `base_blob` hex is displayed in reverse byte order, which means `GetHex()` and normal byte-array hex editors do not agree by default.
- `SetHexDeprecated()` accepts truncated, oversized, and odd-length hex inputs; that behavior is fragile and deprecated upstream, but any in-scope interface that still depends on it must be called out explicitly instead of being normalized away.
- Hash and witness-hash ordering can diverge from raw `uint256` ordering; `txpackage_tests.cpp` explicitly checks that comparisons use displayed identity semantics instead of plain integer ordering.

**Known bugs**

- No confirmed Phase 2 baseline bug is cataloged in this area yet.

**Suspected unknowns**

- Audit later which user-facing surfaces still require deprecated hex acceptance versus strict `FromHex()` or padded `FromUserHex()` parsing.
- Audit later which downstream features depend on hex-string ordering versus raw blob ordering so typed Rust identifiers do not accidentally erase that distinction.

## Serialization primitives and stream adapters

**Major features**

- little-endian and big-endian integer writers and readers
- `CompactSize`, `VARINT`, and formatter-driven serialization
- `SpanReader`, `VectorWriter`, and `DataStream` as shared byte transports

**Knots sources**

- [`packages/bitcoin-knots/src/serialize.h`](../../../packages/bitcoin-knots/src/serialize.h)
- [`packages/bitcoin-knots/src/streams.h`](../../../packages/bitcoin-knots/src/streams.h)

**Knots tests and vectors**

- [`packages/bitcoin-knots/src/test/serialize_tests.cpp`](../../../packages/bitcoin-knots/src/test/serialize_tests.cpp)
- [`packages/bitcoin-knots/src/test/fuzz/deserialize.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/deserialize.cpp)

**Quirks to preserve**

- `CompactSize` must reject non-canonical encodings and reject values above `MAX_SIZE` when range checks are enabled.
- `VARINT` uses Knots' one-to-one MSB base-128 scheme, not Bitcoin's CompactSize encoding.
- Vector deserialization reserves memory in 5 MiB batches rather than trusting claimed lengths up front; Phase 2 codecs should keep that denial-of-service boundary visible.
- Serialization parameters are part of the baseline design, not an implementation detail; downstream codecs should keep context explicit instead of smuggling format choices through global state.

**Known bugs**

- No confirmed Phase 2 baseline bug is cataloged in this area yet.

**Suspected unknowns**

- Audit later which stream parameters besides transaction witness mode are required for in-scope parity, especially once disk and networking adapters land.
- Audit later whether any legacy serializer edge cases outside the imported unit and fuzz coverage need dedicated repo-owned fixtures.

## Script byte containers and limits

**Major features**

- `CScript` as a byte-faithful script container
- opcode and numeric-script boundaries reused by validation later
- unspendable-output detection that depends on script bytes and script length

**Knots sources**

- [`packages/bitcoin-knots/src/script/script.h`](../../../packages/bitcoin-knots/src/script/script.h)

**Knots tests and vectors**

- [`packages/bitcoin-knots/src/test/script_tests.cpp`](../../../packages/bitcoin-knots/src/test/script_tests.cpp)
- [`packages/bitcoin-knots/src/test/data/script_tests.json`](../../../packages/bitcoin-knots/src/test/data/script_tests.json)

**Quirks to preserve**

- Script limits are explicit and reused across later phases: `MAX_SCRIPT_ELEMENT_SIZE`, `MAX_OPS_PER_SCRIPT`, `MAX_PUBKEYS_PER_MULTISIG`, `MAX_PUBKEYS_PER_MULTI_A`, `MAX_SCRIPT_SIZE`, and `MAX_STACK_SIZE`.
- `LOCKTIME_THRESHOLD` and `LOCKTIME_MAX` are domain-visible constants that later validation logic must interpret exactly.
- `IsUnspendable()` treats either `OP_RETURN` or an oversized script as instantly prunable.
- `script_tests.cpp` keeps witness-related flag dependencies explicit by forcing `P2SH` and `WITNESS` when `CLEANSTACK` is present.

**Known bugs**

- No confirmed Phase 2 baseline bug is cataloged in this area yet.

**Suspected unknowns**

- Audit later which script helpers are consensus-critical versus policy-only convenience so Phase 3 and Phase 5 do not over-share one boundary type.
- Audit later whether Open Bitcoin needs dedicated regression fixtures for tapscript-only key-count and annex edge cases beyond the imported upstream corpora.

## Transactions, witnesses, and generic transaction identifiers

**Major features**

- outpoints, inputs, outputs, locktime, and sequence fields
- witness-aware versus witness-free transaction serialization
- `GenTxid` and P2P-facing txid versus wtxid distinctions

**Knots sources**

- [`packages/bitcoin-knots/src/primitives/transaction.h`](../../../packages/bitcoin-knots/src/primitives/transaction.h)
- [`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h)

**Knots tests and vectors**

- [`packages/bitcoin-knots/src/test/transaction_tests.cpp`](../../../packages/bitcoin-knots/src/test/transaction_tests.cpp)
- [`packages/bitcoin-knots/src/test/script_tests.cpp`](../../../packages/bitcoin-knots/src/test/script_tests.cpp)
- [`packages/bitcoin-knots/src/test/data/tx_valid.json`](../../../packages/bitcoin-knots/src/test/data/tx_valid.json)
- [`packages/bitcoin-knots/src/test/data/tx_invalid.json`](../../../packages/bitcoin-knots/src/test/data/tx_invalid.json)
- [`packages/bitcoin-knots/src/test/data/sighash.json`](../../../packages/bitcoin-knots/src/test/data/sighash.json)
- [`packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp)

**Quirks to preserve**

- Transactions have two explicit serialization modes: `TX_WITH_WITNESS` and `TX_NO_WITNESS`.
- Witness serialization uses the dummy-empty-input-vector plus flag-byte envelope; treating it as a generic optional field would lose wire compatibility.
- Knots rejects "superfluous witness" encodings where every witness stack is empty and rejects unknown optional-data flags outright.
- `GenTxid` exists because later networking and mempool paths must preserve the txid versus wtxid distinction instead of collapsing them into one opaque hash.
- `transaction_tests.cpp` and `script_tests.cpp` deserialize transactions with witness enabled, so Phase 2 fixtures should keep witness-aware round trips as the default compatibility path unless a test intentionally uses the non-witness mode.

**Known bugs**

- No confirmed Phase 2 baseline bug is cataloged in this area yet.

**Suspected unknowns**

- Audit later whether any legacy transaction parsers in RPC or CLI surfaces still depend on deprecated or ambiguous hex-path behavior that Phase 2 should expose as typed boundary APIs.
- Audit later whether all in-scope witness-related policy and relay behaviors can be covered by the current upstream vectors, or whether repo-owned regression cases are needed once mempool and networking work begins.

## Blocks, headers, and locators

**Major features**

- block header fields reused across consensus, networking, and chainstate
- full block serialization as header plus transactions
- block locator serialization reused by sync and persistence flows

**Knots sources**

- [`packages/bitcoin-knots/src/primitives/block.h`](../../../packages/bitcoin-knots/src/primitives/block.h)

**Knots tests and vectors**

- [`packages/bitcoin-knots/src/test/fuzz/deserialize.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/deserialize.cpp)

**Quirks to preserve**

- `CBlock` serializes only the header plus `vtx`; expensive-check caches such as `fChecked`, `m_checked_witness_commitment`, and `m_checked_merkle_root` are memory-only.
- `CBlockLocator` still serializes a hard-coded `DUMMY_VERSION` even though the comment says the value has never been used. That historical write shape should stay explicit in Rust rather than being optimized away silently.

**Known bugs**

- No confirmed Phase 2 baseline bug is cataloged in this area yet.

**Suspected unknowns**

- Audit later how block-locator compatibility interacts with chainstate, index, and wallet persistence code paths that remain outside Phase 2.
- Audit later whether any block-header time or merkle-root helper behavior needs dedicated Phase 2 fixtures before consensus execution starts.

## Protocol framing and inventory identities

**Major features**

- 24-byte P2P message header structure
- named message-type catalog for handshake, relay, compact blocks, filters, and transaction-reconciliation messages
- inventory type handling that crosses into txid versus wtxid identity

**Knots sources**

- [`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h)

**Knots tests and vectors**

- [`packages/bitcoin-knots/src/test/fuzz/protocol.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/protocol.cpp)
- [`packages/bitcoin-knots/src/test/fuzz/deserialize.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/deserialize.cpp)

**Quirks to preserve**

- `CMessageHeader` is byte-structured as message-start, 12-byte message type, payload size, then checksum.
- `WTXIDRELAY` is a first-class message-type constant, which means Phase 6 networking work must preserve wtxid-aware relay negotiation as a baseline concern.
- `CInv` is not only a hash plus type tag; `IsGenTxMsg()` and `ToGenTxid()` show that some inventory types carry transaction-identity flavor that later relay code must preserve.
- Address serialization in `protocol.h` already carries distinct network and disk formats plus V1 versus V2 encoding rules; that complexity should stay visible in the catalog even though full address parity is a later-phase concern.

**Known bugs**

- No confirmed Phase 2 baseline bug is cataloged in this area yet.

**Suspected unknowns**

- Audit later exactly which `protocol.h` payload types belong in the first networking catalog slice versus a separate address-and-services slice, especially around `CAddress` V1 or V2 encoding.
- Audit later whether any message-header acceptance or rejection edge cases need repo-owned fixtures beyond the current fuzz coverage.

## Follow-up triggers

Update this entry when Phase 2 or later work:

- imports a new upstream vector corpus
- proves a previously suspected unknown
- discovers a confirmed Knots compatibility bug or intentional Open Bitcoin deviation
- splits one of the areas above into a more specific subsystem catalog page

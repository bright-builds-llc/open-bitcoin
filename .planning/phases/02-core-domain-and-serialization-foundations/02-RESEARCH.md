# Phase 2: Core Domain and Serialization Foundations - Research

**Researched:** 2026-04-11
**Domain:** Rust Bitcoin domain modeling, byte-level parsing/serialization, and
reference-catalog seeding against the vendored Knots baseline
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Keep Phase 2 entirely on the pure-core side of the workspace; runtime code
  remains a downstream consumer.
- Split the phase around reusable domain/codec seams instead of mirroring Knots
  file-for-file.
- Parse raw inputs once at the boundary into invariant-bearing domain types.
- Preserve byte-exact wire-visible distinctions such as CompactSize rules,
  witness vs non-witness encoding, and message header boundaries.
- Use Knots vectors plus repo-owned golden fixtures for deterministic coverage.
- Extend `docs/parity/` into a living reference catalog and record unknowns
  explicitly.

### the agent's Discretion
- Exact crate names and internal module boundaries.
- Which upstream vectors are copied verbatim versus normalized into smaller
  repo-owned fixtures, as long as provenance remains explicit.

### Deferred Ideas (OUT OF SCOPE)
- Live black-box parity harnesses against running nodes.
- Consensus execution, chainstate mutation, mempool policy, wallet behavior,
  and operator interfaces beyond shared domain/codecs.

</user_constraints>

<research_summary>
## Summary

Phase 2 should treat the vendored Knots tree as a behavioral specification, not
as a shape to copy mechanically. The cleanest path in this repo is a small
first-party pure-core library surface that separates invariant-carrying domain
types from byte-level codecs, then re-exports those libraries through the
existing `open-bitcoin-core` crate or uses it as the umbrella crate. This keeps
later consensus, chainstate, networking, and wallet code from having to parse
or validate raw primitives repeatedly.

The most important implementation pattern is boundary parsing with lossless
domain types. Amounts, hashes, scripts, outpoints, transactions, blocks,
headers, message headers, and selected payload envelopes should deserialize from
bytes into strongly typed Rust structures, serialize back to identical bytes,
and expose fallible constructors where baseline invariants matter. The phase
should use the vendored Knots source, unit tests, and fuzz targets as canonical
inputs for fixtures, while keeping the actual verification contract repo-native
through `bash scripts/verify.sh`.

The reference-catalog requirement does not need a separate documentation system.
The repo already established `docs/parity/index.json` as the machine-readable
root, so Phase 2 should expand that surface with subsystem catalog artifacts
under `docs/parity/` that point at upstream source files, tests, quirks, and
unknowns. That satisfies `REF-03` while keeping parity tracking in one place.

**Primary recommendation:** implement Phase 2 as a pure-core domain/codec layer
backed by Knots-derived fixtures, with newtypes plus fallible constructors at
the boundary, and extend `docs/parity/` into a subsystem reference catalog
instead of inventing a second audit surface.
</research_summary>

<standard_stack>
## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust standard library | 1.85 / edition 2024 workspace baseline | Core data structures, byte arrays, collections, formatting, and errors | Matches the repo's minimal-dependency policy and is sufficient for the initial domain/codec surface |
| First-party pure-core crates under `packages/` | workspace-local | Home for domain values and codecs | Required by repo policy: no third-party Rust Bitcoin library in the production path |
| Vendored Knots source and tests | `29.3.knots20260210` baseline | Behavioral specification and fixture source | The repo's parity contract is explicitly anchored to this pinned upstream baseline |
| `scripts/verify.sh` | repo-native | Format, lint, build, test, coverage, and purity verification | Already wired into contributor workflow and CI |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `scripts/check-pure-core-deps.sh` | repo-native | Pure-core dependency/import enforcement | Update when introducing additional pure-core crates |
| Cargo workspace manifests and BUILD files | workspace-local | Keep Cargo and Bazel surfaces aligned | Update whenever new first-party crates or packages are introduced |
| `docs/parity/index.json` and companion docs | repo-local | Reference catalog and parity tracking | Extend as subsystem docs and unknown tracking land |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party domain/codec libraries | `bitcoin` or other third-party Rust Bitcoin crates | Faster bootstrap, but violates the repo's production-path ownership constraint |
| Reusable domain/codec split | Single `open-bitcoin-core` monolith | Less initial ceremony, but weaker reuse boundaries for later phases |
| Knots-derived vectors | Handwritten fixtures only | Simpler authorship, but much weaker provenance and edge-case coverage |

**Installation:**
```bash
# No new external packages are recommended for Phase 2 by default.
# Use the existing workspace and verification contract:
bash scripts/verify.sh
```
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Recommended Project Structure
```text
packages/
├── open-bitcoin-core/          # umbrella crate or re-export surface
├── open-bitcoin-primitives/    # invariant-bearing values and shared structs
├── open-bitcoin-codec/         # byte-level parse/serialize entrypoints
└── open-bitcoin-node/          # downstream runtime consumer

docs/parity/
├── index.json
└── catalog/
   ├── primitives.md
   ├── transactions.md
   └── protocol.md
```

The planner may collapse the first two new crates into modules under
`open-bitcoin-core` if that is measurably simpler, but the code should still
preserve the same domain-vs-codec boundary.

### Pattern 1: Invariants At The Boundary
**What:** Raw bytes and primitive inputs become checked domain types exactly
once, at parse or constructor boundaries.
**When to use:** Amounts, fixed-width hashes, script byte containers, outpoints,
transactions, blocks, headers, and message envelopes.
**Example:**
```rust
pub struct Amount(i64);

impl Amount {
    pub fn from_sats(value: i64) -> Result<Self, AmountError> {
        if !(0..=21_000_000_i64 * 100_000_000).contains(&value) {
            return Err(AmountError::OutOfRange(value));
        }
        Ok(Self(value))
    }
}
```

### Pattern 2: Lossless Codec APIs With Explicit Parameters
**What:** Serialization entrypoints should make wire-format choices explicit
instead of baking them into ambiguous helpers.
**When to use:** Witness vs non-witness transaction encoding, CompactSize/VARINT
handling, network message envelopes, and stream limits.
**Example:**
```rust
pub enum TransactionEncoding {
    WithWitness,
    WithoutWitness,
}

pub fn encode_transaction(
    tx: &Transaction,
    encoding: TransactionEncoding,
) -> Vec<u8> {
    // encode exactly according to the chosen wire mode
}
```

### Pattern 3: Fixture Provenance First
**What:** Every shared fixture should point back to an upstream Knots source,
test, or fuzz target.
**When to use:** Transaction, block, script, and protocol vectors.
**Example:**
```text
tests/fixtures/transactions/tx_valid_001.hex
  source = packages/bitcoin-knots/src/test/data/tx_valid.json
  notes = "byte-exact copy of upstream valid transaction vector"
```

### Anti-Patterns to Avoid
- **C++ shape mirroring:** Do not copy Knots class layout or mutable cache flags
  just because they exist upstream; keep only behaviorally relevant surfaces.
- **Validation everywhere:** Do not re-check amount ranges, hash lengths, or
  script size rules at every call site once boundary parsing already enforced
  them.
- **Lossy abstractions:** Do not replace raw wire-visible distinctions with a
  normalized type that cannot serialize back to the original bytes.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Fixture corpus selection | Brand-new ad hoc transaction/script vectors | Knots unit-test JSON and fuzz targets | Upstream already encodes edge cases and consensus-sensitive byte forms |
| Parity catalog storage | TODO comments scattered through code | `docs/parity/index.json` plus subsystem docs | Keeps parity tracking auditable and visible to contributors |
| Domain validation strategy | Repeated primitive checks in every consumer | Fallible constructors and boundary parsers | Makes illegal states unrepresentable and keeps later phases simpler |

**Key insight:** the project already owns its production implementation surface,
so the main leverage in Phase 2 comes from reusing upstream evidence and repo
contracts, not from inventing new local conventions for fixtures, parity
tracking, or validation.
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Normalizing Away Wire Differences
**What goes wrong:** The core model hides differences between witness and
non-witness transactions, message headers and payloads, or txid/wtxid-style
identity.
**Why it happens:** A single "convenient" abstraction is treated as more
important than byte-exact compatibility.
**How to avoid:** Keep encoding mode and identity distinctions explicit in the
domain and codec APIs.
**Warning signs:** Round-trip tests pass only through one serialization mode, or
two different upstream byte streams produce the same in-memory representation.

### Pitfall 2: Primitive Leakage
**What goes wrong:** Later code keeps receiving `Vec<u8>`, `String`, `i64`, or
tuple primitives and re-validates them ad hoc.
**Why it happens:** The first parse layer stops at syntactic decoding instead of
producing checked types.
**How to avoid:** Give each critical concept a domain type with a narrow
constructor and use those types pervasively.
**Warning signs:** Multiple modules each implement their own amount-range check,
hash-length check, or script-size rule.

### Pitfall 3: Fixture Provenance Drift
**What goes wrong:** Tests pass against locally invented examples but stop
covering actual Knots edge cases.
**Why it happens:** Fixtures are copied or rewritten without recording where
they came from.
**How to avoid:** Record source paths for every imported vector and keep
upstream-derived fixtures separate from repo-authored regression cases.
**Warning signs:** A fixture file has no source note, or reviewers cannot trace
it back to Knots source/tests.

### Pitfall 4: Catalog Debt Hidden As TODOs
**What goes wrong:** Unknown subsystem behavior stays buried in code comments and
never becomes visible to later planners or verifiers.
**Why it happens:** The reference catalog is treated as optional documentation
instead of a requirement artifact.
**How to avoid:** Add subsystem entries and unknown tracking as part of Phase 2
deliverables, not as follow-up cleanup.
**Warning signs:** Code merges introduce "investigate later" comments without a
matching `docs/parity/` update.
</common_pitfalls>

<code_examples>
## Code Examples

Verified patterns from repo and baseline sources:

### Fixed-width domain wrapper
```rust
pub struct Hash32([u8; 32]);

impl TryFrom<&[u8]> for Hash32 {
    type Error = HashError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let array: [u8; 32] = bytes.try_into().map_err(|_| HashError::Length)?;
        Ok(Self(array))
    }
}
```

### Lossless script container
```rust
#[derive(Clone, PartialEq, Eq)]
pub struct ScriptBuf(Vec<u8>);

impl ScriptBuf {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ScriptError> {
        if bytes.len() > 10_000 {
            return Err(ScriptError::TooLarge(bytes.len()));
        }
        Ok(Self(bytes))
    }
}
```

### Fixture metadata shape
```text
fixture_id: tx_valid_001
source_path: packages/bitcoin-knots/src/test/data/tx_valid.json
codec_surface: transaction
expectation: parses and re-serializes byte-for-byte
```
</code_examples>

<sota_updates>
## State of the Art (2024-2026)

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Primitive-heavy domain APIs | Strongly typed newtypes/enums with fallible constructors | Ongoing modern Rust practice | Fits the repo's "illegal states unrepresentable" direction and reduces repeated validation |
| Parser correctness proven mostly by happy-path tests | Parser correctness backed by round-trip, corpus, and fuzz-derived edge cases | Mature baseline in current Bitcoin projects | Phase 2 should seed deterministic corpus coverage early, even before Phase 9 black-box parity |
| Documentation tracked in prose-only notes | Machine-readable parity/capability indexes with companion docs | Common audit/readiness pattern | `docs/parity/index.json` should become the stable root for later audit and parity status |

**New tools/patterns to consider:**
- Keep pure-core libraries dependency-light and use explicit `TryFrom`/builder
  boundaries rather than macro-heavy magic.
- Reuse upstream test/fuzz corpora as fixed regression seeds before introducing
  larger fuzz/property harnesses in later phases.

**Deprecated/outdated:**
- Depending on third-party Rust Bitcoin crates in the production path for domain
  ownership or serialization semantics in this repo.
</sota_updates>

<open_questions>
## Open Questions

1. **Exact crate split vs umbrella modules**
   - What we know: the repo wants reusable pure-core boundaries, and success
     criteria say "crates" expose the shared surface.
   - What's unclear: whether the first implementation should add dedicated
     `open-bitcoin-primitives` / `open-bitcoin-codec` crates immediately or use
     modules under `open-bitcoin-core` first.
   - Recommendation: choose the smallest split that still makes later phases
     consume a stable domain/codec API, then reflect that split consistently in
     Cargo, Bazel, and `scripts/pure-core-crates.txt`.

2. **How much network payload surface belongs in Phase 2**
   - What we know: the roadmap includes "network payloads" in the shared
     domain/serialization foundation.
   - What's unclear: whether Phase 2 should implement only message headers and
     a few foundational payloads or all baseline message variants.
   - Recommendation: cover message headers plus the payload types directly
     reused by later parsing work, and record any still-unknown payloads in the
     reference catalog instead of silently skipping them.

3. **Catalog file format details**
   - What we know: `docs/parity/index.json` already exists and should remain the
     root.
   - What's unclear: whether subsystem detail is best stored as JSON, Markdown,
     or a hybrid.
   - Recommendation: keep `index.json` as the machine-readable index and add
     human-readable subsystem docs that can cite upstream files/tests precisely.
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- `AGENTS.md` — Repo-local workflow contract, verification entrypoint, parity
  recording rules.
- `.planning/PROJECT.md` — Core value, constraints, and architectural intent.
- `.planning/REQUIREMENTS.md` — Phase-mapped requirements and traceability.
- `.planning/phases/02-core-domain-and-serialization-foundations/02-CONTEXT.md`
  — Locked decisions for this phase.
- `packages/bitcoin-knots/src/consensus/amount.h` — Consensus-critical amount
  range baseline.
- `packages/bitcoin-knots/src/serialize.h` — Serialization primitives and size
  limits.
- `packages/bitcoin-knots/src/streams.h` — Stream model and serialization
  context patterns.
- `packages/bitcoin-knots/src/script/script.h` — Script byte semantics and
  limits.
- `packages/bitcoin-knots/src/primitives/transaction.h` — Transaction and
  witness serialization rules.
- `packages/bitcoin-knots/src/primitives/block.h` — Block/header domain shape.
- `packages/bitcoin-knots/src/protocol.h` — Message-header and payload type
  baseline.
- `packages/bitcoin-knots/src/test/serialize_tests.cpp`,
  `packages/bitcoin-knots/src/test/transaction_tests.cpp`,
  `packages/bitcoin-knots/src/test/script_tests.cpp`,
  `packages/bitcoin-knots/src/test/data/script_tests.json`,
  `packages/bitcoin-knots/src/test/data/tx_valid.json`,
  `packages/bitcoin-knots/src/test/data/tx_invalid.json`,
  `packages/bitcoin-knots/src/test/data/sighash.json` — Upstream vector and
  unit-test corpus for deterministic fixtures.
- `packages/bitcoin-knots/src/test/fuzz/deserialize.cpp`,
  `packages/bitcoin-knots/src/test/fuzz/protocol.cpp`,
  `packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp` — Upstream
  fuzz targets that highlight edge cases worth capturing in deterministic tests.

### Secondary (MEDIUM confidence)
- `packages/README.md` — Current workspace boundary contract.
- `scripts/verify.sh` and `scripts/check-pure-core-deps.sh` — Existing local
  verification/purity enforcement surfaces that new code must integrate with.

### Tertiary (LOW confidence - needs validation)
- None.
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: Rust pure-core domain types and byte-level codecs
- Ecosystem: vendored Knots source/tests, repo-native verification, parity docs
- Patterns: boundary parsing, lossless serialization APIs, fixture provenance,
  catalog seeding
- Pitfalls: wire-format normalization, primitive leakage, fixture drift, hidden
  catalog debt

**Confidence breakdown:**
- Standard stack: HIGH — driven by repo policy and existing workspace
- Architecture: HIGH — aligned with current repo constraints and baseline files
- Pitfalls: HIGH — directly visible from the parity and purity goals
- Code examples: MEDIUM — representative patterns, to be validated during
  implementation

**Research date:** 2026-04-11
**Valid until:** stable until the repo changes its baseline or dependency policy
</metadata>

---

*Phase: 02-core-domain-and-serialization-foundations*
*Research completed: 2026-04-11*
*Ready for planning: yes*

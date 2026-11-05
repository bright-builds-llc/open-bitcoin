# Plan167-06 ownership/provenance amendment

Amended at2026-10-21T18:57:48Z within the existing gsd-plan-phase revision workflow. Original lifecycle167-2026-10-21T16-38-23/yolo and generated_at2026-10-21T16:55:59Z remain unchanged. Executor checkpoint:0/2 tasks completed, no06 source changes. This is a bounded plan amendment, not implementation or renewed phase acceptance. Root's independent checker reviews it before release.

## Original and authorized scope

Original plan snapshot: .local/open-bitcoin-dev/phase167/plan06-amendment/before-PLAN.md, SHA256 c11ed18b662e416b8e611f351b030e906881bfa04c04b4643391e6999af16234. Before amendment, current06 PLAN matched that exact digest.

Exactly two existing production owners are added to06 Task1 files_modified/files/read-first:

- packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
- packages/open-bitcoin-chainstate/src/engine/work_authority.rs

Task1 retains the same objective and strengthens initialization, clone/rebind transfer, admission, negative tests and acceptance. The two existing tasks, Task2 in full, CVLD-01/02/03/04 requirements, wave4 and dependencies167-04/05 remain. Other accepted plans/checks remain accepted. No signature/export/error/registrar owner beyond the listed files is authorized; the executor must report an actual need before adding one. Existing tests.rs is already owned; put new behavioral controls in phase167_context.rs. Existing canonical_genesis/tests.rs is read-only and its accepted01 controls remain.

The planning edit scope is only06 PLAN, targeted06 decision-coverage notes and this amendment. No source, shared state/roadmap/requirements/config/task metadata, Git operation, build/test, subagent, acquisition or historical artifact change is included. Independent02 acquisition remains active under its own authority and is not a consumer prerequisite of06.

## Source/contract reasoning

Current engine.rs::Default creates WorkAuthority::new(WorkOrigin::Live) for a generic empty root. The generic connect paths can grow synthetic validated roots. canonical_genesis.rs::from_canonical_genesis also uses Live after sealed proof and complete empty effective-coins/root checks. Therefore Live alone, a root label/hash or matching ConsensusParams cannot prove the selected canonical origin. Matching numerical/structural facts are not a sealed initialization path.

work_authority.rs owns the private owner/generation/seal boundary. Clone for Chainstate<MemoryCoinsView> rebuilds a snapshot owner then explicitly restores origin and maybe_params. Consuming rebind_into first validates live origin and complete effective coins/active-chain/undo/count equality, then creates a new Live owner and restores maybe_params. A marker stored elsewhere or inferred later would either disappear at these legitimate transfers or incorrectly promote generic Live imports. These exact existing owners are necessary to initialize and preserve private selected facts while keeping fresh identities and existing foreign/stale/preview protections.

The design places private maybe_canonical_rules and private admission state inside the existing WorkAuthority. Generic construction leaves them absent. Only the sealed CanonicalGenesisProof constructor initializes selection from proof.spec().rules() after existing root checks. Clone/rebind preserve immutable selected facts only for genuinely canonical usable live sources; they do not transfer pending receipts or StageBindings. Invalidation of mutable coins removes usable canonical authority, and generation changes clear pending admissions. Accept/preview logic must not promote a Live label into canonical selection.

Plan01's complete monetary/difficulty/height-zero activation mapping, empty genesis UTXOs and exact work remain. Generic fixture defaults and intentional small-height fixture compatibility remain. There is no duplicate work engine, public caller boolean/setter, implementable proof trait, raw-snapshot promotion, new dependency or workspace crate.

Accepted03 CandidateHistory/CandidateRules prove structural facts only, including strict checked suffixes; IndependentlySeeded always stays conditional. Accepted04 HeaderStore owns actual private header admission and retains original shell_observed_now, while its public history/context getters return data. Accepted05's selected body wrapper needs the original admission clock and exact header; its transaction constructor and sigop budget are the shared kernels. A copied history/context cannot transfer HeaderStore membership or canonical coins authority.

Chainstate's Cargo.toml depends only on consensus and primitives. It cannot import a HeaderStore authority type from network/node without reversing the existing architecture.06 therefore supplies a pure explicit trusted-clock admission operation on its own canonical owner. From exact headers, explicit trusted-shell observations and the requested disconnect/branch identity, it derives owner-held history, validates shared rules and records private exact-header,parent,rules,owner,generation,original-clock evidence. This operation validates input; it does not certify that an arbitrary caller clock/context came from HeaderStore.09/10 retain responsibility for coordinating the actual held HeaderStore and its original admission observation. No claim that06 alone completes product wiring is introduced.

Pending admission is a single finite owner/generation-scoped batch with checked bounds, cleared on generation changes/acceptance/transfer; it is not another unbounded header registry. Valid duplicate pending admission preserves the first successful original observation. Body staging requires that matching private receipt and uses its original clock, independently rebuilding each actual coins branch after staged disconnect/connect. Neither candidate.time nor a new current replay clock substitutes for admission. Previously rejected future headers gain nothing from that rejection. Branch/parent/header mismatch, missing/stale/foreign/replayed receipt, synthetic Live with matching labels, raw snapshots/config/params/copied context and preview transfers all require refusal controls before accepted mutation.

The bounded seeded-historical adapter still validates exact serialized identity/prerequisites and executes the same real coins/interpreter kernel. Its fixture origin remains tainted and cannot yield ordinary product proof/runtime, even when scalar checks or a genesis-shaped prefix pass. Task2's real interpreter flags, actual coins/creating MTP,80000/80001 budget,rewards+fees,maturity and unchanged-state tests remain byte-for-byte unchanged.

## Metadata and verification handoff

The former conceptual key link engine.rs→CandidateRules is replaced with concrete source paths/calls: canonical_genesis.rs→consensus chain_spec/rules.rs via proof.spec().rules(), planned engine/candidate_rules.rs→consensus candidate_history.rs via CandidateHistory::check, and engine/apply.rs→consensus transaction.rs via validate_transaction_with_context. The planned edge must become executable wiring; no fake comment/string or premature pass claim is permitted.

AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md and local architecture/code-shape/testing/verification/Rust standards informed this revision. Both active lesson inputs were read completely:8856bytes,2953 conservative tokens. No project skills were present. No new external API/library research was needed: this is an existing-owner contract correction, grounded in current code plus accepted01/03/04/05 summaries and167 CONTEXT/RESEARCH.

Root's independent checker must verify exact ownership delta, unchanged Task2/lifecycle/dependencies/requirements, proof-only initialization, clone/rebind freshness, bounded admission, original-clock behavior, fixture taint and retained Task2 evidence. Implementation still owes actual RED/GREEN/final focused checks and attributable summary. This artifact does not assert those future checks passed or relax native/coverage/Bazel/security/goal/lifecycle acceptance.

Planner artifact checks completed: gsd-tools frontmatter validate returned valid:true; verify plan-structure returned valid:true,2tasks,0errors/0warnings. Direct snapshot comparison confirmed exact original SHA256, byte-identical Task2, unchanged lifecycle/generated_at/dependencies/requirements/task count and exactly the two ownership additions above. Reviewed the owned amendment diff and verified only the two opening/closing frontmatter delimiters occur in06 PLAN. No source implementation, Cargo/Bazel/build/test or Git operation was run. The planned candidate_rules.rs key link is pending implementation, not a claimed passing current-source check.

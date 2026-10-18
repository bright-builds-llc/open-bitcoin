# Plan 32 bounded stale-header serving replan

Lifecycle: `165-2026-10-14T16-09-59Z`. Same14 owners, three tasks, full commands, requirements and graph. No source expansion or current/historical pin work is authorized by this design alone.

## Actual new failure

After the cap amendment actually formats to626, the peer corpus passes4/4. The normal100 integrated lifecycle actually passes h1001 extension, ancestor998 reorg999..1002, manual and automatic retention, and authenticated RPC. It then fails inbound exchange4 at the original unowned inbound.rs196 with a closed incomplete frame: old active1000 GetCFilters expects four exact frames. The actual command exits101 with0 passed/1 failed in247 seconds. This is no complete lifecycle GREEN.

Six exact source bodies are preserved in `.local/open-bitcoin-dev/phase165-plan32-inbound-failure-checkpoint.json`, SHA-256 `7be36f07b8849f3e84076bc3839ae96713f66cbb507612c1a03d5812888d9fc9`. Actual log `.local/open-bitcoin-dev/phase165-plan32-normal-lifecycle.log` SHA-256 `7a49d0108cb193b62101c95847d9382b750a831ee751c7184dbd46777cc99b13`. The preceding checked Plan32 is preserved at `.local/open-bitcoin-dev/phase165-plan32-before-stale-header-amendment.md`, SHA-256 `7e3a3c59f0b5af8c0601c02641bfe8b2852b6da099da23e8e341b7fbc0adc3ae`. Original884307 plan and cap14910 checkpoint remain attributable.

## Independently traced cause

Raw initialized startup seeds active position headers without checked proof, selected header params or stale branches. The peer branch validator requires a stop HeaderStore entry before consulting persisted ScriptsValid provenance; stale serving additionally needs checked stop/best proof, and transport independently checks stop membership. Four common-active exchanges succeed before old active1000 fails. Persistent filters and ScriptsValid facts alone do not reconstruct a checked stale header graph. The original inbound source remains exactly `d89e0e47cd23861c17706ab36be8d6efb92528fa6e82f9ddfd8a731d774652ca`.

## Bounded owning-fixture correction

Keep raw open/open_policy(None) unchanged. In the already owned phase162 fixtures.rs open_serving, after raw startup, build an inert HeaderStore from actual saved active positions plus only absent genuine preclose receipt headers. For missing-from-active original999/1000, require actual retained physical body bytes/hash/header to match the receipt. Never restore missing input or replay pruned bodies. Invoke the existing public try_admit_fixture_headers once with complete normal100 params and actual maximum header time. It independently validates every header proof/context/ancestry and numerical work; it does not qualify the pruned body core.

Capture the real pre-admission missing checked stale proof when applicable and verify checked stop/best facts after admission. Require empty eligible_body_candidates and no maybe_body_eligibility for header-only entries; these observations prove no body grant, not whole-core qualification. Compare physical metadata/coins/B/heads/counts before/after and retain every original settled identity, immutable filter/checkpoint, retention,20-frame/byte-oracle, auth/capability, resource, timeout and join assertion. The original102-block denial fixture has no missing stale receipts and must continue to work without invented rows.

The header API is not generically read-only: its publisher can write new KnownOnly records. Here only previously accepted active/receipt identities are permitted. The existing publisher checks each persisted ScriptsValid identity for equality and skips it; an empty known_only vector yields zero publication batches. It does not persist a header snapshot. Unknown/new synthetic headers, borrowed body authority, extra startup, private APIs, production edits, unowned inbound/lifecycle edits,15th owner and cap exceptions are excluded. Current phase162513 leaves115 lines; actual formatting/cap and runtime remain unproved.

## Verification and release

Independent tooling causal review approves this precise same-owner design. An independent plan checker must verify the amendment and unchanged other31 plans/graph/commands/source checkpoint before root GO. Execute actual original integrated lifecycle and complete daemon/coins commands; retain the failed diagnostic and do not lower expected frames or increase deadlines. Independent six-Rust review precedes every Task3 descriptor/version/SOURCE/input pin derivation. Add source controls through existing owned data/loops. Exact two-stage source/adapter inverse, immutable original evidence, copied-module CLI controls, full guard matrices and closing raw hashes remain mandatory. Final14 native/coverage/code/security/formal/Git gates remain pending.

Guidance: repo AGENTS/Bright Builds sidecar/overrides and architecture/code-shape/verification/testing/Rust standards; root-cause and evidence guardrails require this bounded replan after the newly reachable consumer failure.

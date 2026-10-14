# Phase 164: Discussion Log

Audit trail only; planning uses CONTEXT.md.

**Date:** 2026-10-13
**Mode:** Yolo, recommended defaults auto-selected under user authorization.

## Canonical root identity

**Question:** Which root identities are accepted?

| Option | Selected |
| --- | --- |
| Only exact pinned genesis header/body for mainnet, testnet3, signet and regtest; reject any other null-parent root. | Yes (recommended) |
| Accept any PoW-valid null-parent root | No |

**Question:** How is canonical identity exposed?

| Option | Selected |
| --- | --- |
| Use one pure existing-network specification with inspectable genesis and support facts reused by adapters. | Yes (recommended) |
| Duplicate network tables per adapter | No |

## Compatibility refusal

**Question:** When must incompatible state refuse?

| Option | Selected |
| --- | --- |
| Inspect before schema/coins migration, initialization, validation-history recovery, prune resume and BASIC writes across ordinary daemon/operator paths. | Yes (recommended) |
| Check after normal store open | No |

**Question:** How are legacy or incomplete identities treated?

| Option | Selected |
| --- | --- |
| Fail closed with a bounded typed diagnostic when root/network proof is absent, contradictory or noncanonical; preserve prior durable records and provide a seam for Phase 165 work classification. | Yes (recommended) |
| Infer identity from configuration and rewrite state | No |

**Question:** What recovery does refusal offer?

| Option | Selected |
| --- | --- |
| Describe the reason and safe separate-datadir/review steps; no automatic repair, conversion or destructive reindex. | Yes (recommended) |
| Automatically repair | No |

## Validation support and evidence

**Question:** What does selecting a network prove?

| Option | Selected |
| --- | --- |
| Identity selection is separate from validation support. Classify all four existing modes and refuse unsupported ordinary history before acceptance, especially Signet without challenge validation. | Yes (recommended) |
| Apply generic defaults to all networks | No |

**Question:** What intermediate acceptance is claimed?

| Option | Selected |
| --- | --- |
| Only implemented and verified capabilities; missing canonical bootstrap, exact work and candidate-history prerequisites remain explicit gates until Phases 165–167. | Yes (recommended) |
| Claim complete canonical chain support now | No |

**Question:** How are old fixtures retained?

| Option | Selected |
| --- | --- |
| Keep synthetic/source-derived fixture evidence explicitly separate from ordinary canonical runtime acceptance; do not rewrite historical receipts or silently bypass production gates. | Yes (recommended) |
| Present fixture acceptance as canonical parity | No |

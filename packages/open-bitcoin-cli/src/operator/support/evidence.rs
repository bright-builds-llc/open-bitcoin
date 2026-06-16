// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Full-sync support evidence derivation.

use open_bitcoin_node::{
    OpenBitcoinStatusSnapshot,
    status::{
        BestKnownTipStatus, FieldAvailability, NoProgressDiagnosis, StayCurrentStatus,
        SyncProgress, SyncReconcileProgressStatus, SyncRecoveryCategory, SyncResourcePressure,
        SyncStatus,
    },
};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct LiveSmokeEvidence {
    pub(crate) state: EvidenceState,
    pub(crate) report_path: Option<String>,
    pub(crate) summary: Option<Value>,
    pub(crate) reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct FullSyncEvidence {
    pub(crate) initial_tip: SummaryEvidence,
    pub(crate) final_tip: TipEvidence,
    pub(crate) connected_active_chain: ActiveChainEvidence,
    pub(crate) validated_active_chain: ActiveChainEvidence,
    pub(crate) restart_resume_checkpoints: SummaryEvidence,
    pub(crate) stay_current_window: SummaryEvidence,
    pub(crate) peer_contribution: SummaryEvidence,
    pub(crate) no_progress_or_reorg_events: SummaryEvidence,
    pub(crate) resource_pressure: SummaryEvidence,
    pub(crate) recovery: SummaryEvidence,
    pub(crate) verdict: EvidenceVerdictSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct TipEvidence {
    pub(crate) height: Option<u64>,
    pub(crate) hash: Option<String>,
    pub(crate) work: Option<String>,
    pub(crate) freshness: Option<String>,
    pub(crate) maybe_unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ActiveChainEvidence {
    pub(crate) height: Option<u64>,
    pub(crate) hash: Option<String>,
    pub(crate) work: Option<String>,
    pub(crate) maybe_unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SummaryEvidence {
    pub(crate) state: EvidenceState,
    pub(crate) summary: Option<String>,
    pub(crate) maybe_unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct EvidenceVerdictSummary {
    pub(crate) label: SupportEvidenceVerdict,
    pub(crate) justifications: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SupportEvidenceVerdict {
    SyncToTipProven,
    StayCurrentProven,
    DiagnosedBlocker,
    Inconclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EvidenceState {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct EvidenceAvailability {
    pub(crate) state: EvidenceState,
    pub(crate) reason: Option<String>,
}

impl EvidenceAvailability {
    pub(crate) const fn available() -> Self {
        Self {
            state: EvidenceState::Available,
            reason: None,
        }
    }

    pub(crate) fn unavailable(reason: impl Into<String>) -> Self {
        Self {
            state: EvidenceState::Unavailable,
            reason: Some(reason.into()),
        }
    }

    pub(crate) const fn is_available(&self) -> bool {
        matches!(self.state, EvidenceState::Available)
    }
}

pub(crate) fn derive_full_sync_evidence(
    status: &OpenBitcoinStatusSnapshot,
    live_smoke: &LiveSmokeEvidence,
) -> FullSyncEvidence {
    let final_status = live_smoke_final_status(live_smoke);
    let mut final_tip = final_tip_evidence(&status.sync.best_known_tip);
    if !tip_can_prove_sync_to_tip(&final_tip)
        && let Some(live_tip) = live_smoke_final_tip_evidence(final_status)
    {
        final_tip = live_tip;
    }
    let mut connected_active_chain = connected_active_chain_evidence(&status.sync.sync_progress);
    if !active_chain_can_prove_sync_to_tip(&connected_active_chain)
        && let Some(live_chain) = live_smoke_connected_active_chain_evidence(final_status)
    {
        connected_active_chain = live_chain;
    }
    let mut validated_active_chain = validated_active_chain_evidence(&status.sync.sync_progress);
    if !active_chain_can_prove_sync_to_tip(&validated_active_chain)
        && let Some(live_chain) = live_smoke_validated_active_chain_evidence(final_status)
    {
        validated_active_chain = live_chain;
    }
    let mut justifications = Vec::new();
    let sync_to_tip_proven =
        sync_to_tip_proven(&final_tip, &connected_active_chain, &validated_active_chain);
    if sync_to_tip_proven {
        justifications.push("validated_active_chain_matches_best_known_tip".to_string());
    }
    let stay_current_proven = sync_to_tip_proven
        && matches!(
            status.sync.stay_current,
            FieldAvailability::Available(StayCurrentStatus::CurrentAtBestKnownTip)
        );
    if stay_current_proven {
        justifications.push("stay_current_current_at_best_known_tip".to_string());
    }
    let mut blocker_justifications = diagnosed_blocker_justifications(&status.sync);
    let label = if stay_current_proven {
        SupportEvidenceVerdict::StayCurrentProven
    } else if sync_to_tip_proven {
        SupportEvidenceVerdict::SyncToTipProven
    } else if !blocker_justifications.is_empty() {
        justifications.append(&mut blocker_justifications);
        SupportEvidenceVerdict::DiagnosedBlocker
    } else {
        justifications.push("missing_required_sync_to_tip_evidence".to_string());
        SupportEvidenceVerdict::Inconclusive
    };

    FullSyncEvidence {
        initial_tip: initial_tip_evidence(live_smoke),
        final_tip,
        connected_active_chain,
        validated_active_chain,
        restart_resume_checkpoints: live_smoke_summary_field(
            live_smoke,
            "restartResumeEvidence",
            "restart/resume checkpoint evidence unavailable",
        ),
        stay_current_window: stay_current_summary(&status.sync),
        peer_contribution: peer_contribution_summary(&status.sync),
        no_progress_or_reorg_events: no_progress_or_reorg_summary(&status.sync),
        resource_pressure: resource_pressure_summary(&status.sync),
        recovery: recovery_summary(status),
        verdict: EvidenceVerdictSummary {
            label,
            justifications,
        },
    }
}

fn final_tip_evidence(value: &FieldAvailability<BestKnownTipStatus>) -> TipEvidence {
    match value {
        FieldAvailability::Available(value) => TipEvidence {
            height: Some(value.height),
            hash: Some(value.block_hash.clone()),
            work: Some(value.work.clone()),
            freshness: Some(serialized_label(&value.freshness)),
            maybe_unavailable_reason: None,
        },
        FieldAvailability::Unavailable { reason } => TipEvidence {
            height: None,
            hash: None,
            work: None,
            freshness: None,
            maybe_unavailable_reason: Some(reason.clone()),
        },
    }
}

fn connected_active_chain_evidence(value: &FieldAvailability<SyncProgress>) -> ActiveChainEvidence {
    match value {
        FieldAvailability::Available(value) => {
            let hash = value.maybe_connected_block_hash.clone();
            let connected_and_validated_match = value.connected_block_height
                == value.validated_active_chain_height
                && value.maybe_connected_block_hash == value.maybe_validated_active_chain_hash;
            let work = connected_and_validated_match
                .then(|| value.maybe_validated_active_chain_work.clone())
                .flatten();
            let maybe_unavailable_reason = if hash.is_none() {
                Some("connected active-chain hash unavailable".to_string())
            } else if work.is_none() {
                Some("connected active-chain work unavailable".to_string())
            } else {
                None
            };
            ActiveChainEvidence {
                height: Some(value.connected_block_height),
                hash,
                work,
                maybe_unavailable_reason,
            }
        }
        FieldAvailability::Unavailable { reason } => ActiveChainEvidence {
            height: None,
            hash: None,
            work: None,
            maybe_unavailable_reason: Some(reason.clone()),
        },
    }
}

fn validated_active_chain_evidence(value: &FieldAvailability<SyncProgress>) -> ActiveChainEvidence {
    match value {
        FieldAvailability::Available(value) => {
            let hash = value.maybe_validated_active_chain_hash.clone();
            let work = value.maybe_validated_active_chain_work.clone();
            let maybe_unavailable_reason = if hash.is_none() {
                Some("validated active-chain hash unavailable".to_string())
            } else if work.is_none() {
                Some("validated active-chain work unavailable".to_string())
            } else {
                None
            };
            ActiveChainEvidence {
                height: Some(value.validated_active_chain_height),
                hash,
                work,
                maybe_unavailable_reason,
            }
        }
        FieldAvailability::Unavailable { reason } => ActiveChainEvidence {
            height: None,
            hash: None,
            work: None,
            maybe_unavailable_reason: Some(reason.clone()),
        },
    }
}

fn sync_to_tip_proven(
    final_tip: &TipEvidence,
    connected_active_chain: &ActiveChainEvidence,
    validated_active_chain: &ActiveChainEvidence,
) -> bool {
    final_tip.freshness.as_deref() == Some("fresh")
        && chains_match_tip(connected_active_chain, final_tip)
        && chains_match_tip(validated_active_chain, final_tip)
}

fn chains_match_tip(chain: &ActiveChainEvidence, tip: &TipEvidence) -> bool {
    chain.height.is_some()
        && chain.hash.is_some()
        && chain.work.is_some()
        && chain.height == tip.height
        && chain.hash == tip.hash
        && chain.work == tip.work
}

fn tip_can_prove_sync_to_tip(tip: &TipEvidence) -> bool {
    tip.height.is_some()
        && tip.hash.is_some()
        && tip.work.is_some()
        && tip.freshness.as_deref() == Some("fresh")
}

fn active_chain_can_prove_sync_to_tip(chain: &ActiveChainEvidence) -> bool {
    chain.height.is_some() && chain.hash.is_some() && chain.work.is_some()
}

fn live_smoke_final_status(live_smoke: &LiveSmokeEvidence) -> Option<&Value> {
    live_smoke
        .summary
        .as_ref()
        .and_then(|summary| summary.get("finalStatus"))
}

fn live_smoke_final_tip_evidence(final_status: Option<&Value>) -> Option<TipEvidence> {
    let tip = final_status?.get("bestKnownTip")?;
    Some(TipEvidence {
        height: json_u64(tip, "height"),
        hash: json_string(tip, "blockHash"),
        work: json_string(tip, "work"),
        freshness: json_string(tip, "freshness"),
        maybe_unavailable_reason: None,
    })
}

fn live_smoke_connected_active_chain_evidence(
    final_status: Option<&Value>,
) -> Option<ActiveChainEvidence> {
    let final_status = final_status?;
    Some(ActiveChainEvidence {
        height: json_u64(final_status, "connectedBlockHeight")
            .or_else(|| json_u64(final_status, "blockHeight"))
            .or_else(|| json_u64(final_status, "validatedActiveChainHeight")),
        hash: json_string(final_status, "maybeValidatedActiveChainHash"),
        work: json_string(final_status, "maybeValidatedActiveChainWork"),
        maybe_unavailable_reason: None,
    })
}

fn live_smoke_validated_active_chain_evidence(
    final_status: Option<&Value>,
) -> Option<ActiveChainEvidence> {
    let final_status = final_status?;
    Some(ActiveChainEvidence {
        height: json_u64(final_status, "validatedActiveChainHeight"),
        hash: json_string(final_status, "maybeValidatedActiveChainHash"),
        work: json_string(final_status, "maybeValidatedActiveChainWork"),
        maybe_unavailable_reason: None,
    })
}

fn json_u64(value: &Value, key: &str) -> Option<u64> {
    value.get(key).and_then(Value::as_u64)
}

fn json_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn diagnosed_blocker_justifications(sync: &SyncStatus) -> Vec<String> {
    let mut justifications = Vec::new();
    if matches!(
        sync.no_progress_diagnosis,
        FieldAvailability::Available(NoProgressDiagnosis::StorageOrResourceBlocked)
    ) {
        justifications.push("blocking_diagnosis_available".to_string());
    }
    if matches!(
        sync.recovery_category,
        FieldAvailability::Available(SyncRecoveryCategory::ResourceExhaustion)
    ) || matches!(
        &sync.recovery_action,
        FieldAvailability::Available(action)
            if action.to_ascii_lowercase().contains("free storage")
                || action.to_ascii_lowercase().contains("resource")
    ) {
        justifications.push("resource_exhaustion_recovery_available".to_string());
    }
    if matches!(
        &sync.resource_pressure,
        FieldAvailability::Available(pressure) if resource_pressure_indicates_blocker(pressure)
    ) {
        justifications.push("resource_pressure_blocking".to_string());
    }
    if matches!(
        sync.latest_reorg,
        FieldAvailability::Available(ref evidence) if !evidence.fully_persisted
    ) {
        justifications.push("blocking_reorg_evidence_available".to_string());
    }
    if matches!(
        sync.reconcile_progress,
        FieldAvailability::Available(SyncReconcileProgressStatus::BranchCompetitionAwaitingBodies {
            missing_block_count,
            ..
        }) if missing_block_count > 0
    ) {
        justifications.push("blocking_reconcile_evidence_available".to_string());
    }
    justifications
}

fn resource_pressure_indicates_blocker(value: &SyncResourcePressure) -> bool {
    value.max_blocks_in_flight_total > 0
        && value.blocks_in_flight >= value.max_blocks_in_flight_total
}

fn initial_tip_evidence(live_smoke: &LiveSmokeEvidence) -> SummaryEvidence {
    live_smoke_summary_field(
        live_smoke,
        "firstHeaderProgress",
        "initial tip evidence unavailable",
    )
}

fn stay_current_summary(sync: &SyncStatus) -> SummaryEvidence {
    match (&sync.stay_current, &sync.stay_current_next_action) {
        (FieldAvailability::Available(status), FieldAvailability::Available(next_action)) => {
            SummaryEvidence::available(format!(
                "status={} next_action={}",
                serialized_label(status),
                next_action
            ))
        }
        (FieldAvailability::Available(status), FieldAvailability::Unavailable { reason }) => {
            SummaryEvidence::available(format!(
                "status={} next_action=Unavailable: {}",
                serialized_label(status),
                reason
            ))
        }
        (FieldAvailability::Unavailable { reason }, _) => {
            SummaryEvidence::unavailable(reason.clone())
        }
    }
}

fn peer_contribution_summary(sync: &SyncStatus) -> SummaryEvidence {
    match &sync.attempt_counters {
        FieldAvailability::Available(counters) => SummaryEvidence::available(format!(
            "connected={} failed={} attempted={} max_sync_rounds={}",
            counters.connected_peers,
            counters.failed_peers,
            counters.attempted_peers,
            counters.max_sync_rounds
        )),
        FieldAvailability::Unavailable { reason } => SummaryEvidence::unavailable(reason.clone()),
    }
}

fn no_progress_or_reorg_summary(sync: &SyncStatus) -> SummaryEvidence {
    let diagnosis = availability_label(&sync.no_progress_diagnosis);
    let next_action = availability_string(&sync.no_progress_next_action);
    let latest_reorg = availability_json(&sync.latest_reorg);
    let reconcile = availability_json(&sync.reconcile_progress);
    SummaryEvidence::available(format!(
        "diagnosis={diagnosis} next_action={next_action} latest_reorg={latest_reorg} reconcile_progress={reconcile}"
    ))
}

fn resource_pressure_summary(sync: &SyncStatus) -> SummaryEvidence {
    match &sync.resource_pressure {
        FieldAvailability::Available(value) => SummaryEvidence::available(format!(
            "blocks_in_flight={} outbound_peers={} target_outbound_peers={} max_blocks_in_flight_total={}",
            value.blocks_in_flight,
            value.outbound_peers,
            value.target_outbound_peers,
            value.max_blocks_in_flight_total
        )),
        FieldAvailability::Unavailable { reason } => SummaryEvidence::unavailable(reason.clone()),
    }
}

fn recovery_summary(status: &OpenBitcoinStatusSnapshot) -> SummaryEvidence {
    match &status.recovery_evidence {
        FieldAvailability::Available(evidence) => {
            return SummaryEvidence::available(format!(
                "category={} cause={} action_class={} next_action={}",
                evidence.category.as_str(),
                serialized_label(&evidence.cause),
                serialized_label(&evidence.action_class),
                evidence.next_action
            ));
        }
        FieldAvailability::Unavailable { .. } => {}
    }

    recovery_summary_from_legacy_sync(&status.sync)
}

fn recovery_summary_from_legacy_sync(sync: &SyncStatus) -> SummaryEvidence {
    match (&sync.recovery_category, &sync.recovery_action) {
        (FieldAvailability::Available(category), FieldAvailability::Available(action)) => {
            SummaryEvidence::available(format!("category={} action={}", category.as_str(), action))
        }
        (FieldAvailability::Available(category), FieldAvailability::Unavailable { reason }) => {
            SummaryEvidence::available(format!(
                "category={} action=Unavailable: {}",
                category.as_str(),
                reason
            ))
        }
        (FieldAvailability::Unavailable { reason }, _) => {
            SummaryEvidence::unavailable(reason.clone())
        }
    }
}

fn live_smoke_summary_field(
    live_smoke: &LiveSmokeEvidence,
    key: &str,
    unavailable_reason: &str,
) -> SummaryEvidence {
    live_smoke
        .summary
        .as_ref()
        .and_then(|summary| summary.get(key))
        .map(|value| SummaryEvidence::available(value.to_string()))
        .unwrap_or_else(|| SummaryEvidence::unavailable(unavailable_reason))
}

impl SummaryEvidence {
    fn available(summary: impl Into<String>) -> Self {
        Self {
            state: EvidenceState::Available,
            summary: Some(summary.into()),
            maybe_unavailable_reason: None,
        }
    }

    fn unavailable(reason: impl Into<String>) -> Self {
        Self {
            state: EvidenceState::Unavailable,
            summary: None,
            maybe_unavailable_reason: Some(reason.into()),
        }
    }
}

fn availability_label<T>(value: &FieldAvailability<T>) -> String
where
    T: Serialize,
{
    match value {
        FieldAvailability::Available(value) => serialized_label(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn availability_string(value: &FieldAvailability<String>) -> String {
    match value {
        FieldAvailability::Available(value) => value.clone(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn availability_json<T>(value: &FieldAvailability<T>) -> String
where
    T: Serialize,
{
    match value {
        FieldAvailability::Available(value) => serde_json::to_value(value)
            .map(|value| value.to_string())
            .unwrap_or_else(|_| "unknown".to_string()),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn serialized_label<T>(value: &T) -> String
where
    T: Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

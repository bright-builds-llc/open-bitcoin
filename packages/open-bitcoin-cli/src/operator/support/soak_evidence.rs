// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Compact support-bundle soak and forensic collection.

use std::{fs, path::Path};

use serde::Serialize;

use crate::operator::{
    config::OperatorConfigResolution,
    soak::{
        ledger::{SoakLedger, SoakLedgerLayout, SoakRunIndex},
        report::SoakReportProjection,
    },
};

use super::{
    EvidenceState, RedactionSummary, SOAK_LEDGER_UNAVAILABLE_REASON,
    forensics::SupportForensicsEvidence, path_to_string, soak_outcome_label,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct SoakSupportEvidence {
    pub(super) state: EvidenceState,
    pub(super) maybe_run_id: Option<String>,
    pub(super) maybe_final_outcome: Option<String>,
    pub(super) maybe_latest_sequence: Option<u64>,
    pub(super) maybe_source_ledger_path: Option<String>,
    pub(super) maybe_json_report_path: Option<String>,
    pub(super) maybe_markdown_report_path: Option<String>,
    pub(super) maybe_unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SoakSupportCollection {
    pub(super) soak_evidence: SoakSupportEvidence,
    pub(super) support_forensics: SupportForensicsEvidence,
}

impl SoakSupportCollection {
    fn unavailable() -> Self {
        Self {
            soak_evidence: SoakSupportEvidence::unavailable(),
            support_forensics: SupportForensicsEvidence::unavailable(
                SOAK_LEDGER_UNAVAILABLE_REASON,
            ),
        }
    }
}

impl SoakSupportEvidence {
    fn available(
        run_id: String,
        maybe_final_outcome: Option<String>,
        latest_sequence: u64,
        source_ledger_path: &Path,
        json_report_path: &Path,
        markdown_report_path: &Path,
    ) -> Self {
        Self {
            state: EvidenceState::Available,
            maybe_run_id: Some(run_id),
            maybe_final_outcome,
            maybe_latest_sequence: Some(latest_sequence),
            maybe_source_ledger_path: Some(path_to_string(source_ledger_path)),
            maybe_json_report_path: Some(path_to_string(json_report_path)),
            maybe_markdown_report_path: Some(path_to_string(markdown_report_path)),
            maybe_unavailable_reason: None,
        }
    }

    fn unavailable() -> Self {
        Self {
            state: EvidenceState::Unavailable,
            maybe_run_id: None,
            maybe_final_outcome: None,
            maybe_latest_sequence: None,
            maybe_source_ledger_path: None,
            maybe_json_report_path: None,
            maybe_markdown_report_path: None,
            maybe_unavailable_reason: Some(SOAK_LEDGER_UNAVAILABLE_REASON.to_string()),
        }
    }
}

pub(super) fn collect_soak_support_evidence(
    config_resolution: &OperatorConfigResolution,
    redaction: &RedactionSummary,
) -> SoakSupportCollection {
    let Some(data_dir) = config_resolution.maybe_data_dir.as_ref() else {
        return SoakSupportCollection::unavailable();
    };

    let layout = SoakLedgerLayout::for_datadir(data_dir);
    let index = match fs::read_to_string(layout.run_index_path())
        .ok()
        .and_then(|text| serde_json::from_str::<SoakRunIndex>(&text).ok())
    {
        Some(index) => index,
        None => return SoakSupportCollection::unavailable(),
    };
    let Some(latest_run) = index.runs.first() else {
        return SoakSupportCollection::unavailable();
    };

    let run_paths = layout.paths_for_run(&latest_run.run_id);
    if latest_run.ledger_path != run_paths.events_path {
        return SoakSupportCollection::unavailable();
    }

    let read = match SoakLedger::read_events(&run_paths.events_path) {
        Ok(read) => read,
        Err(_) => return SoakSupportCollection::unavailable(),
    };
    let projection =
        match SoakReportProjection::from_ledger_events(read.events.clone(), &run_paths.events_path)
        {
            Ok(projection) => projection,
            Err(_) => return SoakSupportCollection::unavailable(),
        };
    let maybe_final_outcome = projection
        .verdict
        .as_ref()
        .or(projection.stop.as_ref())
        .map(|event| soak_outcome_label(event.outcome));

    SoakSupportCollection {
        soak_evidence: SoakSupportEvidence::available(
            projection.run_id.as_str().to_string(),
            maybe_final_outcome,
            projection.latest_sequence,
            &run_paths.events_path,
            &run_paths.report_json_path,
            &run_paths.report_markdown_path,
        ),
        support_forensics: SupportForensicsEvidence::available(
            &read,
            &projection,
            &run_paths.events_path,
            &run_paths.report_json_path,
            &run_paths.report_markdown_path,
            redaction,
        ),
    }
}

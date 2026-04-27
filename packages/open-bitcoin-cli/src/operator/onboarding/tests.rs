// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::path::PathBuf;

use super::{
    OnboardingMessage, OnboardingMessageLevel, OnboardingPlan, OnboardingPromptAnswers,
    OnboardingRequest, OnboardingWriteDecision, ProposedConfigWrite,
};
use crate::operator::NetworkSelection;

#[test]
fn onboarding_write_decisions_distinguish_no_proposed_and_approved_writes() {
    // Arrange
    let path = PathBuf::from("/tmp/open-bitcoin.jsonc");
    let proposed = ProposedConfigWrite {
        path: path.clone(),
        contents: "{ \"schema_version\": 1 }".to_string(),
        replaces_existing: false,
    };

    // Act
    let decisions = [
        OnboardingWriteDecision::NoWrite {
            reason: "dry run".to_string(),
        },
        OnboardingWriteDecision::ProposedWrite {
            write: proposed.clone(),
        },
        OnboardingWriteDecision::ApprovedWrite { write: proposed },
    ];

    // Assert
    assert!(matches!(
        decisions[0],
        OnboardingWriteDecision::NoWrite { .. }
    ));
    assert!(matches!(
        decisions[1],
        OnboardingWriteDecision::ProposedWrite { .. }
    ));
    assert!(matches!(
        decisions[2],
        OnboardingWriteDecision::ApprovedWrite { .. }
    ));
}

#[test]
fn onboarding_plan_keeps_prompt_answers_and_messages_explicit() {
    // Arrange
    let answers = OnboardingPromptAnswers {
        maybe_network: Some(NetworkSelection::Regtest),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_config_path: Some(PathBuf::from("/tmp/open-bitcoin.jsonc")),
        detect_existing_installations: true,
        metrics_enabled: true,
        logs_enabled: true,
        approve_write: false,
    };

    // Act
    let plan = OnboardingPlan {
        request: OnboardingRequest::NonInteractive {
            answers: answers.clone(),
            force_overwrite: false,
        },
        write_decision: OnboardingWriteDecision::NoWrite {
            reason: "approval required".to_string(),
        },
        messages: vec![OnboardingMessage {
            level: OnboardingMessageLevel::Info,
            text: "approval required".to_string(),
        }],
    };

    // Assert
    let OnboardingRequest::NonInteractive {
        answers: stored,
        force_overwrite,
    } = plan.request
    else {
        panic!("expected non-interactive request");
    };
    assert_eq!(stored, answers);
    assert!(!force_overwrite);
    assert_eq!(plan.messages[0].level, OnboardingMessageLevel::Info);
}

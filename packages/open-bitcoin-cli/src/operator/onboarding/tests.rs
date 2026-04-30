// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use super::{
    OnboardingExistingState, OnboardingMessage, OnboardingMessageLevel, OnboardingPlan,
    OnboardingPromptAnswers, OnboardingPrompter, OnboardingRequest, OnboardingWriteDecision,
    ProposedConfigWrite, apply_onboarding_plan, plan_onboarding, prompt_onboarding_answers,
    render_onboarding_plan,
};
use crate::operator::{
    NetworkSelection,
    config::OperatorConfigResolution,
    detect::{
        DetectedInstallation, DetectionConfidence, DetectionSourcePath, DetectionSourcePathKind,
        DetectionUncertainty, ProductFamily, WalletCandidate, WalletCandidateKind,
        WalletChainScope,
    },
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-onboarding-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }

    fn child(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

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
        existing: OnboardingExistingState::default(),
        proposed_answers: answers.clone(),
        detected_installations: Vec::new(),
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
    assert_eq!(plan.proposed_answers, answers);
}

#[test]
fn interactive_prompt_collects_practical_first_run_answers() {
    // Arrange
    let mut prompter = FakePrompter::new([
        "regtest",
        "/tmp/open-bitcoin",
        "/tmp/open-bitcoin/open-bitcoin.jsonc",
        "yes",
        "no",
        "yes",
        "yes",
    ]);
    let defaults = OnboardingPromptAnswers {
        maybe_network: Some(NetworkSelection::Mainnet),
        maybe_data_dir: Some(PathBuf::from("/tmp/default")),
        maybe_config_path: Some(PathBuf::from("/tmp/default/open-bitcoin.jsonc")),
        detect_existing_installations: false,
        metrics_enabled: true,
        logs_enabled: true,
        approve_write: false,
    };

    // Act
    let answers = prompt_onboarding_answers(&mut prompter, &defaults).expect("prompt answers");

    // Assert
    assert_eq!(prompter.calls, 7);
    assert_eq!(answers.maybe_network, Some(NetworkSelection::Regtest));
    assert_eq!(
        answers.maybe_config_path,
        Some(PathBuf::from("/tmp/open-bitcoin/open-bitcoin.jsonc"))
    );
    assert!(answers.metrics_enabled);
    assert!(!answers.logs_enabled);
    assert!(answers.detect_existing_installations);
    assert!(answers.approve_write);
}

#[test]
fn non_interactive_onboarding_writes_jsonc_and_never_bitcoin_conf() {
    // Arrange
    let sandbox = TestDirectory::new("write");
    let data_dir = sandbox.child("open-bitcoin");
    let config_path = data_dir.join("open-bitcoin.jsonc");
    let answers = complete_answers(&data_dir, &config_path, true);
    let request = OnboardingRequest::NonInteractive {
        answers,
        force_overwrite: false,
    };

    // Act
    let plan = plan_onboarding(
        &resolution(&data_dir, &config_path),
        OnboardingExistingState::default(),
        vec![detected_installation()],
        request,
    )
    .expect("onboarding plan");
    let written = apply_onboarding_plan(&plan).expect("apply onboarding");
    let contents = fs::read_to_string(&config_path).expect("config contents");

    // Assert
    assert_eq!(written, Some(config_path.clone()));
    assert!(contents.contains("\"onboarding\""));
    assert!(contents.contains("\"wizard_answers\""));
    assert!(contents.contains("\"network\""));
    assert!(contents.contains("\"datadir\""));
    assert!(contents.contains("\"metrics\""));
    assert!(contents.contains("\"logs\""));
    assert!(contents.contains("\"migration\""));
    assert!(!data_dir.join("bitcoin.conf").exists());
}

#[test]
fn onboarding_render_surfaces_wallet_metadata_and_read_only_caution() {
    // Arrange
    let sandbox = TestDirectory::new("render");
    let data_dir = sandbox.child("open-bitcoin");
    let config_path = data_dir.join("open-bitcoin.jsonc");
    let request = OnboardingRequest::NonInteractive {
        answers: complete_answers(&data_dir, &config_path, false),
        force_overwrite: false,
    };

    // Act
    let plan = plan_onboarding(
        &resolution(&data_dir, &config_path),
        OnboardingExistingState::default(),
        vec![detected_installation()],
        request,
    )
    .expect("onboarding plan");
    let rendered = render_onboarding_plan(&plan);

    // Assert
    assert!(rendered.contains("Wallet candidate:"));
    assert!(rendered.contains("name=primary"));
    assert!(rendered.contains("chain=mainnet"));
    assert!(rendered.contains("format=legacy wallet.dat"));
    assert!(rendered.contains("Read-only inspection only"));
    assert!(!rendered.contains("__cookie__:secret"));
    assert!(!rendered.contains("legacy wallet bytes"));
}

#[test]
fn existing_jsonc_is_unchanged_without_force_overwrite() {
    // Arrange
    let sandbox = TestDirectory::new("existing");
    let data_dir = sandbox.child("open-bitcoin");
    let config_path = data_dir.join("open-bitcoin.jsonc");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(&config_path, "{ \"schema_version\": 1 }\n").expect("existing config");
    let before = fs::read(&config_path).expect("before");
    let request = OnboardingRequest::NonInteractive {
        answers: complete_answers(&data_dir, &config_path, true),
        force_overwrite: false,
    };

    // Act
    let plan = plan_onboarding(
        &resolution(&data_dir, &config_path),
        OnboardingExistingState::default(),
        Vec::new(),
        request,
    )
    .expect("onboarding plan");
    let written = apply_onboarding_plan(&plan).expect("apply onboarding");
    let after = fs::read(&config_path).expect("after");

    // Assert
    assert_eq!(written, None);
    assert_eq!(before, after);
    assert!(matches!(
        plan.write_decision,
        OnboardingWriteDecision::NoWrite { .. }
    ));
}

#[test]
fn non_interactive_missing_values_fail_without_prompting() {
    // Arrange
    let prompter = FakePrompter::new(["regtest"]);
    let answers = OnboardingPromptAnswers {
        maybe_network: None,
        maybe_data_dir: None,
        maybe_config_path: None,
        detect_existing_installations: false,
        metrics_enabled: true,
        logs_enabled: true,
        approve_write: true,
    };

    // Act
    let error = plan_onboarding(
        &OperatorConfigResolution::default(),
        OnboardingExistingState::default(),
        Vec::new(),
        OnboardingRequest::NonInteractive {
            answers,
            force_overwrite: false,
        },
    )
    .expect_err("missing values must fail");

    // Assert
    assert!(
        error
            .to_string()
            .contains("missing required onboarding value")
    );
    assert_eq!(prompter.calls, 0);
}

fn complete_answers(
    data_dir: &std::path::Path,
    config_path: &std::path::Path,
    approve_write: bool,
) -> OnboardingPromptAnswers {
    OnboardingPromptAnswers {
        maybe_network: Some(NetworkSelection::Regtest),
        maybe_data_dir: Some(data_dir.to_path_buf()),
        maybe_config_path: Some(config_path.to_path_buf()),
        detect_existing_installations: true,
        metrics_enabled: true,
        logs_enabled: true,
        approve_write,
    }
}

fn resolution(
    data_dir: &std::path::Path,
    config_path: &std::path::Path,
) -> OperatorConfigResolution {
    OperatorConfigResolution {
        maybe_config_path: Some(config_path.to_path_buf()),
        maybe_data_dir: Some(data_dir.to_path_buf()),
        maybe_network: Some(NetworkSelection::Regtest),
        ..OperatorConfigResolution::default()
    }
}

fn detected_installation() -> DetectedInstallation {
    DetectedInstallation {
        product_family: ProductFamily::Unknown,
        confidence: DetectionConfidence::Low,
        uncertainty: vec![DetectionUncertainty::ProductAmbiguous],
        source_paths: vec![DetectionSourcePath {
            kind: DetectionSourcePathKind::ConfigFile,
            path: PathBuf::from("/tmp/core/.bitcoin/bitcoin.conf"),
            present: true,
        }],
        maybe_data_dir: Some(PathBuf::from("/tmp/core/.bitcoin")),
        maybe_config_file: Some(PathBuf::from("/tmp/core/.bitcoin/bitcoin.conf")),
        maybe_cookie_file: None,
        wallet_candidates: vec![WalletCandidate {
            kind: WalletCandidateKind::LegacyWalletFile,
            path: PathBuf::from("/tmp/core/.bitcoin/wallets/primary/wallet.dat"),
            maybe_name: Some("primary".to_string()),
            present: true,
            product_family: ProductFamily::Unknown,
            product_confidence: DetectionConfidence::Low,
            chain_scope: WalletChainScope::Mainnet,
        }],
    }
}

#[derive(Debug)]
struct FakePrompter {
    answers: Vec<String>,
    calls: usize,
}

impl FakePrompter {
    fn new<const N: usize>(answers: [&str; N]) -> Self {
        Self {
            answers: answers
                .iter()
                .map(|value| value.to_string())
                .rev()
                .collect(),
            calls: 0,
        }
    }
}

impl OnboardingPrompter for FakePrompter {
    fn prompt(
        &mut self,
        _question: &str,
        _maybe_default: Option<&str>,
    ) -> Result<String, super::OnboardingError> {
        self.calls += 1;
        self.answers
            .pop()
            .ok_or_else(|| super::OnboardingError::new("missing fake prompt answer"))
    }
}

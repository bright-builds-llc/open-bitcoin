// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    DetectionScan, OnboardArgs, OperatorCli, OperatorCommandOutcome, OperatorRuntimeError,
};
use crate::operator::{
    config::OperatorConfigResolution,
    onboarding::{
        OnboardingError, OnboardingPromptAnswers, OnboardingRequest, StdIoOnboardingPrompter,
        apply_onboarding_plan, plan_onboarding, prompt_onboarding_answers,
        read_existing_open_bitcoin_config, render_onboarding_plan,
    },
};

pub(super) fn execute_onboarding(
    args: &OnboardArgs,
    cli: &OperatorCli,
    config_resolution: OperatorConfigResolution,
    detections: DetectionScan,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let defaults = OnboardingPromptAnswers {
        maybe_network: cli.maybe_network,
        maybe_data_dir: cli.maybe_data_dir.clone(),
        maybe_config_path: cli.maybe_config_path.clone(),
        detect_existing_installations: args.detect_existing,
        metrics_enabled: !args.disable_metrics,
        logs_enabled: !args.disable_logs,
        approve_write: args.approve_write,
    };
    let answers = if args.non_interactive {
        defaults
    } else {
        let mut prompter = StdIoOnboardingPrompter;
        prompt_onboarding_answers(&mut prompter, &defaults).map_err(onboarding_error)?
    };
    let request = if args.non_interactive {
        OnboardingRequest::NonInteractive {
            answers,
            force_overwrite: args.force_overwrite,
        }
    } else {
        OnboardingRequest::Interactive { answers }
    };
    let existing = read_existing_open_bitcoin_config(config_resolution.maybe_config_path.as_ref())
        .map_err(onboarding_error)?;
    let plan = plan_onboarding(
        &config_resolution,
        existing,
        detections.installations,
        request,
    )
    .map_err(onboarding_error)?;
    apply_onboarding_plan(&plan).map_err(onboarding_error)?;
    Ok(OperatorCommandOutcome::success(format!(
        "{}\n",
        render_onboarding_plan(&plan)
    )))
}

fn onboarding_error(error: OnboardingError) -> OperatorRuntimeError {
    OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
    }
}

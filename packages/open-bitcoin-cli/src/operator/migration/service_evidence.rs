// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

use std::{
    fs,
    path::{Path, PathBuf},
};

use super::MigrationInstallationSummary;
use crate::operator::detect::{DetectedInstallation, ServiceCandidate, ServiceManager};

pub(super) const SERVICE_REVIEW_AMBIGUOUS: &str = "service_review_ambiguous";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ServiceAssociation {
    Matched(Vec<ServiceCandidate>),
    Ambiguous,
    NoneDetected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CandidateAssociation {
    Matched,
    Ambiguous,
    OtherInstallation,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ServiceOwnershipEvidence {
    maybe_data_dir: Option<PathBuf>,
    maybe_config_file: Option<PathBuf>,
}

pub(super) fn associate_service_candidates(
    installation: &DetectedInstallation,
) -> ServiceAssociation {
    let mut matched = Vec::new();
    let mut saw_ambiguous = false;

    for candidate in installation
        .service_candidates
        .iter()
        .filter(|candidate| candidate.present)
    {
        match associate_candidate(candidate, installation) {
            CandidateAssociation::Matched => matched.push(candidate.clone()),
            CandidateAssociation::Ambiguous => saw_ambiguous = true,
            CandidateAssociation::OtherInstallation => {}
        }
    }

    if !matched.is_empty() {
        return ServiceAssociation::Matched(matched);
    }
    if saw_ambiguous {
        return ServiceAssociation::Ambiguous;
    }

    ServiceAssociation::NoneDetected
}

pub(super) fn summary_service_review_is_ambiguous(
    installation: &MigrationInstallationSummary,
) -> bool {
    installation
        .uncertainty
        .iter()
        .any(|uncertainty| uncertainty == SERVICE_REVIEW_AMBIGUOUS)
}

fn associate_candidate(
    candidate: &ServiceCandidate,
    installation: &DetectedInstallation,
) -> CandidateAssociation {
    let Some(evidence) = read_service_ownership_evidence(candidate) else {
        return CandidateAssociation::Ambiguous;
    };
    if !evidence_is_specific(&evidence) {
        return CandidateAssociation::Ambiguous;
    }
    if evidence_matches_installation(installation, &evidence) {
        return CandidateAssociation::Matched;
    }

    CandidateAssociation::OtherInstallation
}

fn read_service_ownership_evidence(
    candidate: &ServiceCandidate,
) -> Option<ServiceOwnershipEvidence> {
    let content = fs::read_to_string(&candidate.path).ok()?;

    Some(match candidate.manager {
        ServiceManager::Systemd => parse_systemd_service_ownership(&content),
        ServiceManager::Launchd => parse_launchd_service_ownership(&content),
        ServiceManager::Unknown => return None,
    })
}

fn evidence_is_specific(evidence: &ServiceOwnershipEvidence) -> bool {
    evidence.maybe_data_dir.is_some() || evidence.maybe_config_file.is_some()
}

fn evidence_matches_installation(
    installation: &DetectedInstallation,
    evidence: &ServiceOwnershipEvidence,
) -> bool {
    let maybe_data_dir_matches = installation
        .maybe_data_dir
        .as_deref()
        .zip(evidence.maybe_data_dir.as_deref())
        .is_some_and(|(installation_data_dir, service_data_dir)| {
            paths_match(installation_data_dir, service_data_dir)
        });
    if maybe_data_dir_matches {
        return true;
    }

    installation
        .maybe_config_file
        .as_deref()
        .zip(evidence.maybe_config_file.as_deref())
        .is_some_and(|(installation_config_file, service_config_file)| {
            paths_match(installation_config_file, service_config_file)
        })
}

fn parse_systemd_service_ownership(unit_content: &str) -> ServiceOwnershipEvidence {
    let tokens = parse_systemd_exec_tokens(unit_content);
    ServiceOwnershipEvidence {
        maybe_data_dir: extract_path_argument(&tokens, &["-datadir", "--datadir"]),
        maybe_config_file: extract_path_argument(
            &tokens,
            &["-conf", "--conf", "-config", "--config"],
        ),
    }
}

fn parse_systemd_exec_tokens(unit_content: &str) -> Vec<String> {
    collect_systemd_exec_start(unit_content)
        .map(|command| split_shellish(&command))
        .unwrap_or_default()
}

fn collect_systemd_exec_start(unit_content: &str) -> Option<String> {
    let mut command = String::new();
    let mut collecting = false;

    for line in unit_content.lines() {
        let trimmed = line.trim();
        let maybe_segment = if collecting {
            Some(trimmed)
        } else {
            trimmed.strip_prefix("ExecStart=")
        };
        let Some(segment) = maybe_segment else {
            continue;
        };

        collecting = append_exec_segment(&mut command, segment);
        if !collecting {
            break;
        }
    }

    (!command.is_empty()).then_some(command)
}

fn append_exec_segment(command: &mut String, segment: &str) -> bool {
    let continues = segment.trim_end().ends_with('\\');
    let trimmed = segment.trim().trim_end_matches('\\').trim();
    if !trimmed.is_empty() {
        if !command.is_empty() {
            command.push(' ');
        }
        command.push_str(trimmed);
    }
    continues
}

fn parse_launchd_service_ownership(plist_content: &str) -> ServiceOwnershipEvidence {
    let tokens = parse_launchd_program_arguments(plist_content);
    ServiceOwnershipEvidence {
        maybe_data_dir: extract_path_argument(&tokens, &["-datadir", "--datadir"]),
        maybe_config_file: extract_path_argument(
            &tokens,
            &["-conf", "--conf", "-config", "--config"],
        ),
    }
}

fn parse_launchd_program_arguments(plist_content: &str) -> Vec<String> {
    let mut in_program_arguments = false;
    let mut in_program_arguments_array = false;
    let mut arguments = Vec::new();

    for line in plist_content.lines() {
        let trimmed = line.trim();
        if trimmed == "<key>ProgramArguments</key>" {
            in_program_arguments = true;
            continue;
        }
        if in_program_arguments && trimmed == "<array>" {
            in_program_arguments_array = true;
            continue;
        }
        if in_program_arguments_array && trimmed == "</array>" {
            break;
        }
        if !in_program_arguments_array {
            continue;
        }

        let Some(value) = trimmed
            .strip_prefix("<string>")
            .and_then(|rendered| rendered.strip_suffix("</string>"))
        else {
            continue;
        };
        arguments.push(xml_unescape(value));
    }

    arguments
}

fn xml_unescape(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn extract_path_argument(tokens: &[String], flags: &[&str]) -> Option<PathBuf> {
    for (index, token) in tokens.iter().enumerate() {
        for flag in flags {
            if token == flag {
                let value = tokens.get(index + 1)?;
                return Some(PathBuf::from(value));
            }

            let prefix = format!("{flag}=");
            if let Some(value) = token.strip_prefix(&prefix) {
                return Some(PathBuf::from(value));
            }
        }
    }

    None
}

fn split_shellish(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut maybe_quote = None;

    for character in input.chars() {
        match maybe_quote {
            Some(quote) if character == quote => maybe_quote = None,
            Some(_) => current.push(character),
            None if character == '"' || character == '\'' => maybe_quote = Some(character),
            None if character.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            None => current.push(character),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

fn paths_match(left: &Path, right: &Path) -> bool {
    left.components().eq(right.components())
}

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const BUILD_COMMIT_ENV: &str = "OPEN_BITCOIN_BUILD_COMMIT";
const BUILD_TIME_ENV: &str = "OPEN_BITCOIN_BUILD_TIME";
const BUILD_TARGET_ENV: &str = "OPEN_BITCOIN_BUILD_TARGET";
const BUILD_PROFILE_ENV: &str = "OPEN_BITCOIN_BUILD_PROFILE";

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .unwrap_or(manifest_dir.as_path());

    emit_git_rerun_instructions(workspace_root);
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");

    emit_env_var(BUILD_COMMIT_ENV, git_commit(workspace_root));
    emit_env_var(BUILD_TIME_ENV, build_time());
    emit_env_var(BUILD_TARGET_ENV, env::var("TARGET").ok());
    emit_env_var(BUILD_PROFILE_ENV, env::var("PROFILE").ok());
}

fn emit_env_var(name: &str, maybe_value: Option<String>) {
    let Some(value) = maybe_value.map(|value| value.trim().to_string()) else {
        return;
    };
    if value.is_empty() {
        return;
    }

    println!("cargo:rustc-env={name}={value}");
}

fn emit_git_rerun_instructions(workspace_root: &Path) {
    let git_dir = workspace_root.join(".git");
    let head_path = git_dir.join("HEAD");
    println!("cargo:rerun-if-changed={}", head_path.display());

    let Ok(head_contents) = fs::read_to_string(&head_path) else {
        return;
    };
    let Some(reference) = head_contents.trim().strip_prefix("ref: ") else {
        return;
    };

    println!(
        "cargo:rerun-if-changed={}",
        git_dir.join(reference).display()
    );
}

fn git_commit(workspace_root: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(workspace_root)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn build_time() -> Option<String> {
    if let Ok(source_date_epoch) = env::var("SOURCE_DATE_EPOCH") {
        let trimmed = source_date_epoch.trim();
        if !trimmed.is_empty() {
            return format_epoch_seconds(trimmed);
        }
    }

    current_utc_timestamp()
}

fn current_utc_timestamp() -> Option<String> {
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()?;
    command_output(output)
}

fn format_epoch_seconds(epoch_seconds: &str) -> Option<String> {
    let bsd_output = Command::new("date")
        .args(["-u", "-r", epoch_seconds, "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok();
    if let Some(timestamp) = bsd_output.and_then(command_output) {
        return Some(timestamp);
    }

    let gnu_output = Command::new("date")
        .args([
            "-u",
            "-d",
            &format!("@{epoch_seconds}"),
            "+%Y-%m-%dT%H:%M:%SZ",
        ])
        .output()
        .ok()?;
    command_output(gnu_output)
}

fn command_output(output: std::process::Output) -> Option<String> {
    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

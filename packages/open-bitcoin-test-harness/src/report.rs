// Parity breadcrumbs:
// - packages/bitcoin-knots/test/functional/test_framework
// - packages/bitcoin-knots/test/functional/interface_rpc.py
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde_json::json;

use crate::case::SuiteReport;

#[derive(Debug)]
pub enum ReportError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "report I/O failed: {error}"),
            Self::Json(error) => write!(f, "report JSON failed: {error}"),
        }
    }
}

impl std::error::Error for ReportError {}

impl From<std::io::Error> for ReportError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for ReportError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn write_reports_from_env(report: &SuiteReport) -> Result<Option<Vec<PathBuf>>, ReportError> {
    let Ok(dir) = std::env::var("OPEN_BITCOIN_PARITY_REPORT_DIR") else {
        return Ok(None);
    };
    let paths = write_reports(report, Path::new(&dir))?;
    Ok(Some(paths))
}

pub fn write_reports(report: &SuiteReport, dir: &Path) -> Result<Vec<PathBuf>, ReportError> {
    fs::create_dir_all(dir)?;
    let file_stem = safe_file_stem(&report.suite, &report.target);
    let json_path = dir.join(format!("{file_stem}.json"));
    let md_path = dir.join(format!("{file_stem}.md"));

    write_json_report(report, &json_path)?;
    write_markdown_report(report, &md_path)?;
    Ok(vec![json_path, md_path])
}

fn write_json_report(report: &SuiteReport, path: &Path) -> Result<(), ReportError> {
    let outcomes = report
        .outcomes
        .iter()
        .map(|outcome| {
            json!({
                "case": outcome.case_name,
                "passed": outcome.passed,
                "detail": outcome.detail,
            })
        })
        .collect::<Vec<_>>();
    let body = json!({
        "suite": report.suite,
        "target": report.target,
        "skipped": report.skipped,
        "skip_reason": report.skip_reason,
        "passed": report.passed(),
        "passed_count": report.passed_count(),
        "case_count": report.outcomes.len(),
        "outcomes": outcomes,
    });
    fs::write(path, serde_json::to_string_pretty(&body)?)?;
    Ok(())
}

fn write_markdown_report(report: &SuiteReport, path: &Path) -> Result<(), ReportError> {
    let mut file = fs::File::create(path)?;
    writeln!(file, "# Parity Suite Report")?;
    writeln!(file)?;
    writeln!(file, "- Suite: `{}`", report.suite)?;
    writeln!(file, "- Target: `{}`", report.target)?;
    if report.skipped {
        writeln!(
            file,
            "- Status: skipped ({})",
            report.skip_reason.as_deref().unwrap_or("no reason")
        )?;
        return Ok(());
    }
    writeln!(
        file,
        "- Status: {}",
        if report.passed() { "passed" } else { "failed" }
    )?;
    writeln!(
        file,
        "- Cases: {}/{} passed",
        report.passed_count(),
        report.outcomes.len()
    )?;
    writeln!(file)?;
    writeln!(file, "| Case | Status | Detail |")?;
    writeln!(file, "| --- | --- | --- |")?;
    for outcome in &report.outcomes {
        writeln!(
            file,
            "| {} | {} | {} |",
            outcome.case_name,
            if outcome.passed { "passed" } else { "failed" },
            outcome.detail.replace('|', "\\|")
        )?;
    }
    Ok(())
}

fn safe_file_stem(suite: &str, target: &str) -> String {
    format!("{}-{}", sanitize(suite), sanitize(target))
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::case::{CaseOutcome, SuiteReport};

    use super::write_reports;

    #[test]
    fn write_reports_creates_json_and_markdown_files() {
        // Arrange
        let dir =
            std::env::temp_dir().join(format!("open-bitcoin-report-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let report = SuiteReport {
            suite: "rpc".to_string(),
            target: "open-bitcoin".to_string(),
            skipped: false,
            skip_reason: None,
            outcomes: vec![CaseOutcome {
                case_name: "shape".to_string(),
                passed: true,
                detail: "ok".to_string(),
            }],
        };

        // Act
        let paths = write_reports(&report, &dir).expect("reports should write");

        // Assert
        assert_eq!(paths.len(), 2);
        assert!(
            fs::read_to_string(&paths[0])
                .expect("json")
                .contains("\"suite\"")
        );
        assert!(
            fs::read_to_string(&paths[1])
                .expect("markdown")
                .contains("# Parity Suite Report")
        );
        let _ = fs::remove_dir_all(dir);
    }
}

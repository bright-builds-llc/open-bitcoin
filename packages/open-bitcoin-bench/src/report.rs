// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::BenchError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchReport {
    pub schema_version: u32,
    pub baseline: String,
    pub mode: String,
    pub profile: BenchProfileReport,
    pub generated_at_unix_seconds: u64,
    pub groups: Vec<BenchGroupReport>,
    pub optional_knots_source: Option<KnotsSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchProfileReport {
    pub iterations_per_case: u64,
    pub binary_profile: String,
    pub threshold_free: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchGroupReport {
    pub id: String,
    pub description: String,
    pub cases: Vec<BenchCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchCaseReport {
    pub id: String,
    pub group: String,
    pub description: String,
    pub measurement: BenchMeasurementReport,
    pub iterations: u64,
    pub total_elapsed_nanos: u64,
    pub average_elapsed_nanos: u64,
    pub knots_mapping: KnotsMappingReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchMeasurementReport {
    pub focus: String,
    pub fixture: String,
    pub durability: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnotsMappingReport {
    pub benchmark_names: Vec<String>,
    pub source_files: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnotsSource {
    pub baseline: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_json_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_bin_path: Option<String>,
    pub generated_at_unix_seconds: u64,
}

pub fn to_json_string(report: &BenchReport) -> Result<String, BenchError> {
    Ok(serde_json::to_string_pretty(report)?)
}

pub fn write_json_report(report: &BenchReport, path: &Path) -> Result<PathBuf, BenchError> {
    fs::write(path, to_json_string(report)?)?;
    Ok(path.to_path_buf())
}

pub fn write_markdown_report(report: &BenchReport, path: &Path) -> Result<PathBuf, BenchError> {
    let mut file = fs::File::create(path)?;
    file.write_all(to_markdown(report).as_bytes())?;
    Ok(path.to_path_buf())
}

pub fn to_markdown(report: &BenchReport) -> String {
    let mut markdown = String::new();
    markdown.push_str("# Open Bitcoin Benchmark Report\n\n");
    markdown.push_str(&format!("- Schema version: `{}`\n", report.schema_version));
    markdown.push_str(&format!(
        "- Baseline: `{}`\n",
        escape_markdown_table_cell(&report.baseline)
    ));
    markdown.push_str(&format!(
        "- Mode: `{}`\n",
        escape_markdown_table_cell(&report.mode)
    ));
    markdown.push_str(&format!(
        "- Iterations per case: `{}`\n",
        report.profile.iterations_per_case
    ));
    markdown.push_str(&format!(
        "- Binary profile: `{}`\n",
        escape_markdown_table_cell(&report.profile.binary_profile)
    ));
    markdown.push_str(&format!(
        "- Timing thresholds: `{}`\n",
        if report.profile.threshold_free {
            "disabled"
        } else {
            "enabled"
        }
    ));
    markdown.push_str(&format!(
        "- Generated: `{}`\n",
        report.generated_at_unix_seconds
    ));
    if let Some(knots_source) = &report.optional_knots_source {
        markdown.push_str(&format!(
            "- Knots baseline source: `{}`\n",
            escape_markdown_table_cell(&knots_source.baseline)
        ));
        if let Some(path) = &knots_source.maybe_json_path {
            markdown.push_str(&format!(
                "- Knots JSON report path: `{}`\n",
                escape_markdown_table_cell(path)
            ));
        }
        if let Some(path) = &knots_source.maybe_bin_path {
            markdown.push_str(&format!(
                "- Knots binary path: `{}`\n",
                escape_markdown_table_cell(path)
            ));
        }
    }
    markdown.push('\n');
    markdown.push_str("| Group | Case | Focus | Fixture | State | Iterations | Total ns | Avg ns | Knots benchmarks | Sources |\n");
    markdown.push_str("| --- | --- | --- | --- | --- | ---: | ---: | ---: | --- | --- |\n");

    for group in &report.groups {
        for case in &group.cases {
            markdown.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                escape_markdown_table_cell(&group.id),
                escape_markdown_table_cell(&case.id),
                escape_markdown_table_cell(&case.measurement.focus),
                escape_markdown_table_cell(&case.measurement.fixture),
                escape_markdown_table_cell(&case.measurement.durability),
                case.iterations,
                case.total_elapsed_nanos,
                case.average_elapsed_nanos,
                escape_markdown_table_cell(&joined_or_dash(&case.knots_mapping.benchmark_names)),
                escape_markdown_table_cell(&joined_or_dash(&case.knots_mapping.source_files)),
            ));
        }
    }

    markdown
}

fn joined_or_dash(values: &[String]) -> String {
    if values.is_empty() {
        "-".to_string()
    } else {
        values.join(", ")
    }
}

fn escape_markdown_table_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        BenchCaseReport, BenchGroupReport, BenchMeasurementReport, BenchProfileReport, BenchReport,
        KnotsMappingReport, to_json_string, to_markdown,
    };

    #[test]
    fn report_serializes_stable_json_schema_fields() {
        // Arrange
        let report = sample_report();

        // Act
        let serialized = serde_json::to_value(&report);

        // Assert
        let Ok(Value::Object(object)) = serialized else {
            panic!("report should serialize to a JSON object");
        };
        for key in [
            "schema_version",
            "baseline",
            "mode",
            "profile",
            "groups",
            "optional_knots_source",
        ] {
            assert!(object.contains_key(key), "missing report key {key}");
        }
    }

    #[test]
    fn json_report_uses_pretty_typed_serialization() {
        // Arrange
        let report = sample_report();

        // Act
        let serialized = to_json_string(&report);

        // Assert
        let Ok(serialized) = serialized else {
            panic!("report should serialize");
        };
        assert!(serialized.contains("\"schema_version\""));
        assert!(serialized.contains("\"profile\""));
        assert!(serialized.contains("\"optional_knots_source\""));
    }

    #[test]
    fn markdown_table_cells_escape_pipe_characters() {
        // Arrange
        let mut report = sample_report();
        report.groups[0].cases[0].id = "case|with|pipes".to_string();

        // Act
        let markdown = to_markdown(&report);

        // Assert
        assert!(markdown.contains("case\\|with\\|pipes"));
    }

    #[test]
    fn markdown_uses_dash_for_unmapped_knots_columns() {
        // Arrange
        let mut report = sample_report();
        report.groups[0].cases[0]
            .knots_mapping
            .benchmark_names
            .clear();
        report.groups[0].cases[0].knots_mapping.source_files.clear();

        // Act
        let markdown = to_markdown(&report);

        // Assert
        assert!(markdown.contains("| - | - |"));
    }

    fn sample_report() -> BenchReport {
        BenchReport {
            schema_version: 2,
            baseline: "Bitcoin Knots 29.3.knots20260210".to_string(),
            mode: "smoke:1".to_string(),
            profile: BenchProfileReport {
                iterations_per_case: 1,
                binary_profile: "debug".to_string(),
                threshold_free: true,
            },
            generated_at_unix_seconds: 1,
            groups: vec![BenchGroupReport {
                id: "consensus-script".to_string(),
                description: "Consensus script validation".to_string(),
                cases: vec![BenchCaseReport {
                    id: "consensus-script.registry".to_string(),
                    group: "consensus-script".to_string(),
                    description: "Registry contract".to_string(),
                    measurement: BenchMeasurementReport {
                        focus: "script_validation".to_string(),
                        fixture: "shared_static_fixtures".to_string(),
                        durability: "pure".to_string(),
                    },
                    iterations: 1,
                    total_elapsed_nanos: 10,
                    average_elapsed_nanos: 10,
                    knots_mapping: KnotsMappingReport {
                        benchmark_names: vec!["VerifyScriptBench".to_string()],
                        source_files: vec![
                            "packages/bitcoin-knots/src/bench/verify_script.cpp".to_string(),
                        ],
                        notes: "Knots mapping".to_string(),
                    },
                }],
            }],
            optional_knots_source: None,
        }
    }
}

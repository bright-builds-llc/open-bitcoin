use std::{
    env, fs,
    path::PathBuf,
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_bench::{
    error::BenchError,
    registry::{benchmark_groups, list_output},
    report::{KnotsSource, to_json_string, to_markdown, write_json_report, write_markdown_report},
    runner::{RunConfig, RunMode, run_benchmarks},
};

const BASELINE: &str = "Bitcoin Knots 29.3.knots20260210";
const DEFAULT_OUTPUT_DIR: &str = "packages/target/benchmark-reports";

enum Command {
    List,
    Run {
        config: RunConfig,
        output_dir: PathBuf,
        maybe_knots_json: Option<PathBuf>,
        maybe_knots_bin: Option<PathBuf>,
        output_format: OutputFormat,
    },
}

enum OutputFormat {
    Summary,
    Json,
    Markdown,
}

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();

    match run(&args) {
        Ok(stdout) => {
            print!("{stdout}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &[String]) -> Result<String, BenchError> {
    match parse_command(args)? {
        Command::List => Ok(list_output()),
        Command::Run {
            config,
            output_dir,
            maybe_knots_json,
            maybe_knots_bin,
            output_format,
        } => {
            let generated_at = unix_timestamp()?;
            let report = run_benchmarks(
                benchmark_groups(),
                config,
                BASELINE,
                generated_at,
                knots_source(maybe_knots_json, maybe_knots_bin, generated_at),
            )?;
            fs::create_dir_all(&output_dir)?;
            let stem = report_stem(config);
            let json_path = write_json_report(&report, &output_dir.join(format!("{stem}.json")))?;
            let markdown_path =
                write_markdown_report(&report, &output_dir.join(format!("{stem}.md")))?;

            let mut stdout = format!(
                "wrote {}\nwrote {}\n",
                json_path.display(),
                markdown_path.display()
            );
            match output_format {
                OutputFormat::Summary => {}
                OutputFormat::Json => {
                    stdout.push_str(&to_json_string(&report)?);
                    stdout.push('\n');
                }
                OutputFormat::Markdown => stdout.push_str(&to_markdown(&report)),
            }
            Ok(stdout)
        }
    }
}

fn parse_command(args: &[String]) -> Result<Command, BenchError> {
    if args.is_empty() {
        return Err(BenchError::InvalidArgument(usage()));
    }

    if args == ["--list"] {
        return Ok(Command::List);
    }

    let mut maybe_mode: Option<&str> = None;
    let mut mode_count = 0;
    let mut maybe_iterations: Option<u64> = None;
    let mut output_dir = PathBuf::from(DEFAULT_OUTPUT_DIR);
    let mut maybe_knots_json: Option<PathBuf> = None;
    let mut maybe_knots_bin: Option<PathBuf> = None;
    let mut output_format = OutputFormat::Summary;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--smoke" => {
                maybe_mode = Some("smoke");
                mode_count += 1;
                index += 1;
            }
            "--full" => {
                maybe_mode = Some("full");
                mode_count += 1;
                index += 1;
            }
            "--iterations" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(BenchError::InvalidArgument(
                        "--iterations requires a numeric value".to_string(),
                    ));
                };
                maybe_iterations = Some(value.parse::<u64>().map_err(|_| {
                    BenchError::InvalidArgument(format!("invalid iteration count `{value}`"))
                })?);
                index += 2;
            }
            "--format" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(BenchError::InvalidArgument(
                        "--format requires json or markdown".to_string(),
                    ));
                };
                output_format = match value.as_str() {
                    "json" => OutputFormat::Json,
                    "markdown" => OutputFormat::Markdown,
                    _ => {
                        return Err(BenchError::InvalidArgument(format!(
                            "unsupported output format `{value}`"
                        )));
                    }
                };
                index += 2;
            }
            "--output-dir" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(BenchError::InvalidArgument(
                        "--output-dir requires a path".to_string(),
                    ));
                };
                output_dir = PathBuf::from(value);
                index += 2;
            }
            "--knots-json" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(BenchError::InvalidArgument(
                        "--knots-json requires a path".to_string(),
                    ));
                };
                maybe_knots_json = Some(PathBuf::from(value));
                index += 2;
            }
            "--knots-bin" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(BenchError::InvalidArgument(
                        "--knots-bin requires a path".to_string(),
                    ));
                };
                maybe_knots_bin = Some(PathBuf::from(value));
                index += 2;
            }
            "--help" | "-h" => return Err(BenchError::InvalidArgument(usage())),
            unknown => {
                return Err(BenchError::InvalidArgument(format!(
                    "unknown benchmark argument `{unknown}`\n{}",
                    usage()
                )));
            }
        }
    }

    let mode = if let (1, Some(mode)) = (mode_count, maybe_mode) {
        mode
    } else {
        return Err(BenchError::InvalidArgument(
            "choose exactly one of --smoke or --full".to_string(),
        ));
    };

    let config = match mode {
        "smoke" => RunConfig::smoke(maybe_iterations.unwrap_or(1))?,
        "full" => {
            let Some(iterations) = maybe_iterations else {
                return Err(BenchError::MissingFullIterations);
            };
            RunConfig::full(iterations)?
        }
        _ => return Err(BenchError::InvalidArgument(usage())),
    };

    Ok(Command::Run {
        config,
        output_dir,
        maybe_knots_json,
        maybe_knots_bin,
        output_format,
    })
}

fn knots_source(
    maybe_knots_json: Option<PathBuf>,
    maybe_knots_bin: Option<PathBuf>,
    generated_at_unix_seconds: u64,
) -> Option<KnotsSource> {
    if maybe_knots_json.is_none() && maybe_knots_bin.is_none() {
        return None;
    }

    Some(KnotsSource {
        baseline: BASELINE.to_string(),
        maybe_json_path: maybe_knots_json.map(|path| path.display().to_string()),
        maybe_bin_path: maybe_knots_bin.map(|path| path.display().to_string()),
        generated_at_unix_seconds,
    })
}

fn report_stem(config: RunConfig) -> &'static str {
    match config.mode {
        RunMode::Smoke => "open-bitcoin-bench-smoke",
        RunMode::Full { .. } => "open-bitcoin-bench-full",
    }
}

fn unix_timestamp() -> Result<u64, BenchError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn usage() -> String {
    "usage: open-bitcoin-bench --list | --smoke [--iterations N] [--output-dir PATH] [--knots-json PATH] [--knots-bin PATH] [--format json|markdown] | --full --iterations N [--output-dir PATH] [--knots-json PATH] [--knots-bin PATH] [--format json|markdown]".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::Value;

    use super::run;

    #[test]
    fn smoke_run_writes_json_and_markdown_reports_to_output_dir() {
        // Arrange
        let output_dir = std::env::temp_dir().join(format!(
            "open-bitcoin-bench-smoke-report-{}",
            std::process::id()
        ));
        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).expect("remove old benchmark report directory");
        }
        let args = vec![
            "--smoke".to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
        ];

        // Act
        let stdout = run(&args).expect("smoke report run should succeed");

        // Assert
        let json_path = output_dir.join("open-bitcoin-bench-smoke.json");
        let markdown_path = output_dir.join("open-bitcoin-bench-smoke.md");
        assert!(stdout.contains(json_path.to_string_lossy().as_ref()));
        assert!(stdout.contains(markdown_path.to_string_lossy().as_ref()));
        let json = fs::read_to_string(&json_path).expect("read JSON benchmark report");
        let value: Value = serde_json::from_str(&json).expect("benchmark report JSON");
        assert!(
            value["groups"]
                .as_array()
                .is_some_and(|groups| groups.len() >= 7)
        );
        assert!(markdown_path.exists());
        fs::remove_dir_all(&output_dir).expect("remove benchmark report directory");
    }

    #[test]
    fn optional_knots_paths_are_recorded_without_reading_inputs() {
        // Arrange
        let output_dir = std::env::temp_dir().join(format!(
            "open-bitcoin-bench-knots-source-{}",
            std::process::id()
        ));
        if output_dir.exists() {
            fs::remove_dir_all(&output_dir).expect("remove old benchmark report directory");
        }
        let missing_json = output_dir.join("missing-knots-report.json");
        let missing_bin = output_dir.join("missing-knots");
        let args = vec![
            "--smoke".to_string(),
            "--output-dir".to_string(),
            output_dir.display().to_string(),
            "--knots-json".to_string(),
            missing_json.display().to_string(),
            "--knots-bin".to_string(),
            missing_bin.display().to_string(),
        ];

        // Act
        run(&args).expect("smoke report run should accept optional Knots paths");

        // Assert
        let json_path = output_dir.join("open-bitcoin-bench-smoke.json");
        let json = fs::read_to_string(&json_path).expect("read JSON benchmark report");
        let value: Value = serde_json::from_str(&json).expect("benchmark report JSON");
        assert_eq!(
            value["optional_knots_source"]["maybe_json_path"].as_str(),
            Some(missing_json.to_string_lossy().as_ref())
        );
        assert_eq!(
            value["optional_knots_source"]["maybe_bin_path"].as_str(),
            Some(missing_bin.to_string_lossy().as_ref())
        );
        fs::remove_dir_all(&output_dir).expect("remove benchmark report directory");
    }

    #[test]
    fn run_rejects_conflicting_modes() {
        // Arrange
        let args = vec!["--smoke".to_string(), "--full".to_string()];

        // Act
        let error = run(&args).expect_err("conflicting modes should fail");

        // Assert
        assert_eq!(error.to_string(), "choose exactly one of --smoke or --full");
    }
}

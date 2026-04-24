use std::{
    env,
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_bench::{
    error::BenchError,
    registry::{benchmark_groups, list_output},
    report::{to_json_string, to_markdown},
    runner::{RunConfig, run_benchmarks},
};

const BASELINE: &str = "Bitcoin Knots 29.3.knots20260210";

enum Command {
    List,
    Run {
        config: RunConfig,
        output_format: OutputFormat,
    },
}

enum OutputFormat {
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
            output_format,
        } => {
            let report = run_benchmarks(
                benchmark_groups(),
                config,
                BASELINE,
                unix_timestamp()?,
                None,
            )?;
            match output_format {
                OutputFormat::Json => Ok(to_json_string(&report)? + "\n"),
                OutputFormat::Markdown => Ok(to_markdown(&report)),
            }
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
    let mut maybe_iterations: Option<u64> = None;
    let mut output_format = OutputFormat::Json;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--smoke" => {
                maybe_mode = Some("smoke");
                index += 1;
            }
            "--full" => {
                maybe_mode = Some("full");
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
            "--help" | "-h" => return Err(BenchError::InvalidArgument(usage())),
            unknown => {
                return Err(BenchError::InvalidArgument(format!(
                    "unknown benchmark argument `{unknown}`\n{}",
                    usage()
                )));
            }
        }
    }

    let Some(mode) = maybe_mode else {
        return Err(BenchError::InvalidArgument(usage()));
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
        output_format,
    })
}

fn unix_timestamp() -> Result<u64, BenchError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn usage() -> String {
    "usage: open-bitcoin-bench --list | --smoke [--iterations N] [--format json|markdown] | --full --iterations N [--format json|markdown]".to_string()
}

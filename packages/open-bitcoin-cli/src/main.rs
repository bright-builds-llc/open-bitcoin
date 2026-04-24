mod client;
mod output;

use open_bitcoin_cli::args::stdin_required_for_args;

use std::{
    env,
    io::{self, Read},
    path::PathBuf,
    process::ExitCode,
};

fn main() -> ExitCode {
    let cli_args = env::args_os().skip(1).collect::<Vec<_>>();
    let stdin = if stdin_required_for_args(&cli_args) {
        let mut stdin = String::new();
        io::stdin().read_to_string(&mut stdin).expect("stdin");
        stdin
    } else {
        String::new()
    };

    match client::run_cli(&cli_args, &stdin, &default_data_dir()) {
        Ok(stdout) => {
            if !stdout.is_empty() {
                print!("{stdout}");
            }
            ExitCode::SUCCESS
        }
        Err(failure) => {
            eprintln!("{}", failure.stderr);
            ExitCode::from(failure.exit_code as u8)
        }
    }
}

fn default_data_dir() -> PathBuf {
    if cfg!(target_os = "macos")
        && let Some(home) = env::var_os("HOME")
    {
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("Bitcoin");
    }

    if cfg!(windows)
        && let Some(app_data) = env::var_os("APPDATA")
    {
        return PathBuf::from(app_data).join("Bitcoin");
    }

    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home).join(".bitcoin");
    }

    PathBuf::from(".bitcoin")
}

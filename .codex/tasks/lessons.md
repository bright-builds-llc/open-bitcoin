## lesson-use-repo-local-cli-instructions | 2026-05-05 04:49 CDT

Date: 2026-05-05 04:49 CDT
What went wrong: I presented UAT commands using the `open-bitcoin` alias instead of the repo-local `cargo` or `bazel` invocations the user wanted for this project workflow.
Preventive rule: In this repo, prefer explicit `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...` examples when giving operator command instructions unless the user explicitly asks for the installed alias.
Trigger signal to catch it earlier: Any instruction that tells the user to run `open-bitcoin ...` directly during local verification or UAT.

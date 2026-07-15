## lesson-use-repo-local-cli-instructions | 2026-05-05 04:49 CDT

Date: 2026-05-05 04:49 CDT
What went wrong: I presented UAT commands using the `open-bitcoin` alias instead of the repo-local `cargo` or `bazel` invocations the user wanted for this project workflow.
Preventive rule: In this repo, prefer explicit `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...` examples when giving operator command instructions unless the user explicitly asks for the installed alias.
Trigger signal to catch it earlier: Any instruction that tells the user to run `open-bitcoin ...` directly during local verification or UAT.

## lesson-promote-uat-command-lessons-to-rules | 2026-05-05 05:34 CDT

Date: 2026-05-05 05:34 CDT
What went wrong: The UAT checkpoint still made explicit Cargo and Bazel commands secondary even after the repo had a lesson about avoiding bare `open-bitcoin` alias instructions.
Preventive rule: When a lesson captures a recurring repo-specific UAT workflow rule, promote it into `AGENTS.md` Repo-Local Guidance so future sessions see it before generating checkpoints.
Trigger signal to catch it earlier: A UAT prompt says "or the matching Cargo/Bazel command" without immediately showing the exact copy-pasteable commands.

## lesson-confirm-guard-direction | 2026-07-14 22:49 CDT

Date: 2026-07-14 22:49 CDT
What went wrong: I interpreted "increase the 50 GiB guard" as raising the minimum-free-space threshold, while the user meant relaxing the guard by lowering that threshold.
Preventive rule: When a request changes a guard without naming the target value, restate whether the user wants stricter enforcement or easier passage before proposing a number.
Trigger signal to catch it earlier: The user says increase, decrease, tighten, or relax a threshold but does not provide the replacement value.

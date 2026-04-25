#!/usr/bin/env bash
set -euo pipefail

verify_start_milliseconds=""
coverage_report=""

format_elapsed_duration() {
  local total_milliseconds="$1"
  local hours=0
  local minutes=0
  local seconds=0
  local milliseconds=0

  hours=$((total_milliseconds / 3600000))
  minutes=$(((total_milliseconds % 3600000) / 60000))
  seconds=$(((total_milliseconds % 60000) / 1000))
  milliseconds=$((total_milliseconds % 1000))

  if [[ "$hours" -gt 0 ]]; then
    printf '%sh %sm %s.%03ds' "$hours" "$minutes" "$seconds" "$milliseconds"
  elif [[ "$minutes" -gt 0 ]]; then
    printf '%sm %s.%03ds' "$minutes" "$seconds" "$milliseconds"
  elif [[ "$seconds" -gt 0 ]]; then
    printf '%s.%03ds' "$seconds" "$milliseconds"
  else
    printf '%sms' "$milliseconds"
  fi
}

current_time_milliseconds() {
  local maybe_epoch_realtime="${EPOCHREALTIME:-}"
  local maybe_milliseconds=""
  local fractional=""

  if [[ "$maybe_epoch_realtime" =~ ^([0-9]+)\.([0-9]+)$ ]]; then
    fractional="${BASH_REMATCH[2]}000"
    printf '%s%s\n' "${BASH_REMATCH[1]}" "${fractional:0:3}"
    return
  fi

  if command -v python3 >/dev/null 2>&1; then
    maybe_milliseconds="$(python3 -c 'import time; print(int(time.time() * 1000))' 2>/dev/null || true)"
    maybe_milliseconds="${maybe_milliseconds%%$'\n'*}"
    if [[ "$maybe_milliseconds" =~ ^[0-9]+$ ]]; then
      printf '%s\n' "$maybe_milliseconds"
      return
    fi
  fi

  printf '%s000\n' "$(date +%s)"
}

finish_verify() {
  local exit_status="$1"
  local verify_end_milliseconds=0
  local elapsed_milliseconds=0
  local elapsed_display=""

  if [[ -n "$coverage_report" ]]; then
    rm -f "$coverage_report"
  fi

  verify_end_milliseconds="$(current_time_milliseconds)"

  if [[ -z "$verify_start_milliseconds" ]]; then
    verify_start_milliseconds="$verify_end_milliseconds"
  fi

  elapsed_milliseconds=$((verify_end_milliseconds - verify_start_milliseconds))
  elapsed_display="$(format_elapsed_duration "$elapsed_milliseconds")"

  if [[ "$exit_status" -eq 0 ]]; then
    echo "verify.sh completed in ${elapsed_display} (${elapsed_milliseconds}ms)" >&2
  else
    echo "verify.sh failed after ${elapsed_display} (${elapsed_milliseconds}ms)" >&2
  fi
}

trap 'finish_verify $?' EXIT

require_command() {
  local command_name="$1"
  local maybe_install_hint="${2:-}"

  if command -v "$command_name" >/dev/null 2>&1; then
    return
  fi

  echo "error: ${command_name} is required" >&2
  if [[ -n "$maybe_install_hint" ]]; then
    echo "$maybe_install_hint" >&2
  fi
  exit 1
}

require_command cargo
require_command cargo-llvm-cov "install it with: cargo install cargo-llvm-cov --locked"
require_command bazel "install Bazelisk or Bazel and ensure \`bazel\` is on PATH"
require_command git
require_command grep
require_command bun

verify_start_milliseconds="$(current_time_milliseconds)"
export OPEN_BITCOIN_PARITY_REPORT_DIR="${OPEN_BITCOIN_PARITY_REPORT_DIR:-$PWD/packages/target/parity-reports}"
export OPEN_BITCOIN_BENCHMARK_REPORT_DIR="${OPEN_BITCOIN_BENCHMARK_REPORT_DIR:-$PWD/packages/target/benchmark-reports}"
export OPEN_BITCOIN_LOC_REPORT_SOURCE="${OPEN_BITCOIN_LOC_REPORT_SOURCE:-worktree}"
mkdir -p "$OPEN_BITCOIN_PARITY_REPORT_DIR"
mkdir -p "$OPEN_BITCOIN_BENCHMARK_REPORT_DIR"

bun run scripts/generate-loc-report.ts --source="$OPEN_BITCOIN_LOC_REPORT_SOURCE" --output=docs/metrics/lines-of-code.md --check
bash scripts/check-pure-core-deps.sh
bash scripts/check-file-lengths.sh
bash scripts/check-panic-sites.sh
cargo fmt --manifest-path packages/Cargo.toml --all --check
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bash scripts/run-benchmarks.sh --smoke --output-dir "$OPEN_BITCOIN_BENCHMARK_REPORT_DIR"
bazel build //:core //:node //:rpc //:cli //:test_harness //:bench

pure_core_crates=()
while IFS= read -r crate_name; do
  [[ -z "$crate_name" ]] && continue
  pure_core_crates+=("$crate_name")
done <scripts/pure-core-crates.txt
llvm_cov_args=()
for crate_name in "${pure_core_crates[@]}"; do
  llvm_cov_args+=(--package "$crate_name")
done

cargo llvm-cov clean --manifest-path packages/Cargo.toml --workspace
coverage_report="$(mktemp)"

cargo llvm-cov --manifest-path packages/Cargo.toml "${llvm_cov_args[@]}" --show-missing-lines --text >"$coverage_report"
if grep -q "^Uncovered Lines:" "$coverage_report"; then
  sed -n '/Uncovered Lines:/,$p' "$coverage_report" >&2
  exit 1
fi

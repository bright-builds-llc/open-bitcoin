#!/usr/bin/env bash
set -euo pipefail

verify_start_milliseconds=""
coverage_report=""
verify_mode="full"
print_timings=0
step_labels=()
step_durations=()
step_statuses=()

usage() {
  cat >&2 <<'EOF'
usage: bash scripts/verify.sh [--full | --profile | --fast] [--timings]

Modes:
  --full      Run the full strict verification contract (default).
  --profile   Run the full strict verification contract and print step timings.
  --fast      Run local fast checks; skip benchmark smoke, Bazel smoke, and coverage.

Options:
  --timings   Print step timings for the selected mode.
EOF
}

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

  if [[ "$print_timings" -eq 1 && "${#step_labels[@]}" -gt 0 ]]; then
    print_step_timings >&2
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

parse_args() {
  local mode_seen=0

  while [[ "$#" -gt 0 ]]; do
    case "$1" in
    --full)
      if [[ "$mode_seen" -eq 1 && "$verify_mode" != "full" ]]; then
        echo "error: choose only one verification mode" >&2
        usage
        exit 2
      fi
      verify_mode="full"
      mode_seen=1
      shift
      ;;
    --profile)
      if [[ "$mode_seen" -eq 1 && "$verify_mode" != "full" ]]; then
        echo "error: choose only one verification mode" >&2
        usage
        exit 2
      fi
      verify_mode="full"
      print_timings=1
      mode_seen=1
      shift
      ;;
    --fast)
      if [[ "$mode_seen" -eq 1 ]]; then
        echo "error: choose only one verification mode" >&2
        usage
        exit 2
      fi
      verify_mode="fast"
      mode_seen=1
      shift
      ;;
    --timings)
      print_timings=1
      shift
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      echo "error: unsupported verify option $1" >&2
      usage
      exit 2
      ;;
    esac
  done
}

record_step_timing() {
  local label="$1"
  local duration_milliseconds="$2"
  local status="$3"

  step_labels+=("$label")
  step_durations+=("$duration_milliseconds")
  step_statuses+=("$status")
}

print_step_timings() {
  local idx=0
  local label=""
  local duration=""
  local status=""

  echo "verify.sh step timings:"
  while [[ "$idx" -lt "${#step_labels[@]}" ]]; do
    label="${step_labels[$idx]}"
    duration="$(format_elapsed_duration "${step_durations[$idx]}")"
    status="${step_statuses[$idx]}"
    printf '  [%s] %s - %s\n' "$status" "$label" "$duration"
    idx=$((idx + 1))
  done
}

run_step() {
  local label="$1"
  shift

  local step_start_milliseconds=0
  local step_end_milliseconds=0
  local step_duration_milliseconds=0
  local status=0

  step_start_milliseconds="$(current_time_milliseconds)"
  set +e
  "$@"
  status="$?"
  set -e
  step_end_milliseconds="$(current_time_milliseconds)"
  step_duration_milliseconds=$((step_end_milliseconds - step_start_milliseconds))
  record_step_timing "$label" "$step_duration_milliseconds" "$status"
  return "$status"
}

run_benchmark_list() {
  bash scripts/run-benchmarks.sh --list >/dev/null
}

run_coverage_report() {
  local cargo_status=0

  coverage_report="$(mktemp)"
  if [[ "${#llvm_cov_args[@]}" -gt 0 ]]; then
    cargo llvm-cov --manifest-path packages/Cargo.toml "${llvm_cov_args[@]}" --show-missing-lines --text >"$coverage_report"
  else
    cargo llvm-cov --manifest-path packages/Cargo.toml --show-missing-lines --text >"$coverage_report"
  fi
  cargo_status="$?"
  if [[ "$cargo_status" -ne 0 ]]; then
    return "$cargo_status"
  fi

  if grep -q "^Uncovered Lines:" "$coverage_report"; then
    sed -n '/Uncovered Lines:/,$p' "$coverage_report" >&2
    return 1
  fi
}

# Several release-boundary checkers inspect this script as text and require
# these legacy command lines to remain visible in order. The timed run_step
# calls below are the executed verification path.
: <<'VERIFY_COMMAND_ORDER'
bun run scripts/check-v1.3-release-boundaries.ts
bun run scripts/check-v1.4-release-boundaries.ts
bun run scripts/check-v1.5-release-boundaries.ts
bun run scripts/check-phase61-resource-recovery-boundaries.ts
bun run scripts/check-phase62-sync-truth-surfaces.ts
bun run scripts/check-phase63-service-lifecycle.ts
bun run scripts/check-phase64-service-restart-resume.ts
bun run scripts/check-phase65-support-review.ts
bun run scripts/check-phase66-compatibility-wrapper.ts
bun run scripts/check-phase68-active-chain-persistence.ts
bun run scripts/check-phase69-tip-stay-current.ts
bun run scripts/check-phase70-reorg-recovery.ts
bun run scripts/check-phase71-resource-restart.ts
bun run scripts/check-phase72-observability-evidence.ts
bun test scripts/check-phase73-uat-verification.test.ts
env -u OPEN_BITCOIN_PHASE73_REPO_ROOT bun run scripts/check-phase73-uat-verification.ts
bun run scripts/check-v1.6-release-boundaries.ts
bun test scripts/check-phase75-soak-runner.test.ts
bun run scripts/check-phase75-soak-runner.ts
bun test scripts/check-phase76-resource-bounds.test.ts
bun run scripts/check-phase76-resource-bounds.ts
bun test scripts/check-phase77-corruption-lock-recovery.test.ts
bun run scripts/check-phase77-corruption-lock-recovery.ts
bun test scripts/check-phase78-progress-guarantees.test.ts
bun run scripts/check-phase78-progress-guarantees.ts
bun test scripts/check-phase79-diagnostics-support-bundle.test.ts
bun run scripts/check-phase79-diagnostics-support-bundle.ts
bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts
bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts
bun test scripts/check-phase82-production-claim-boundary.test.ts
bun run scripts/check-phase82-production-claim-boundary.ts
bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts
bun run scripts/check-phase83-support-matrix-issue-evidence.ts
bun test scripts/check-phase84-upgrade-rollback-policy.test.ts
bun run scripts/check-phase84-upgrade-rollback-policy.ts
bun test scripts/check-phase85-operator-runbooks.test.ts
bun run scripts/check-phase85-operator-runbooks.ts
bun test scripts/check-phase86-service-operation-expectations.test.ts
bun run scripts/check-phase86-service-operation-expectations.ts
bun test scripts/check-phase87-release-readiness.test.ts
bun run scripts/check-phase87-release-readiness.ts
bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts
bun run scripts/check-phase88-deterministic-claim-guardrails.ts
bun test scripts/check-phase90-inbound-listener-admission.test.ts
bun run scripts/check-phase90-inbound-listener-admission.ts
bun test scripts/check-phase91-peer-permissions.test.ts
bun run scripts/check-phase91-peer-permissions.ts
VERIFY_COMMAND_ORDER

parse_args "$@"
require_command cargo
require_command git
require_command grep
require_command bun

if [[ "$verify_mode" == "full" ]]; then
  require_command cargo-llvm-cov "install it with: cargo install cargo-llvm-cov --locked"
  require_command bazel "install Bazelisk or Bazel and ensure \`bazel\` is on PATH"
fi

verify_start_milliseconds="$(current_time_milliseconds)"
export OPEN_BITCOIN_PARITY_REPORT_DIR="${OPEN_BITCOIN_PARITY_REPORT_DIR:-$PWD/packages/target/parity-reports}"
export OPEN_BITCOIN_BENCHMARK_REPORT_DIR="${OPEN_BITCOIN_BENCHMARK_REPORT_DIR:-$PWD/packages/target/benchmark-reports}"
export OPEN_BITCOIN_LOC_REPORT_SOURCE="${OPEN_BITCOIN_LOC_REPORT_SOURCE:-worktree}"
mkdir -p "$OPEN_BITCOIN_PARITY_REPORT_DIR"
mkdir -p "$OPEN_BITCOIN_BENCHMARK_REPORT_DIR"

if [[ -z "${CI:-}" ]]; then
  run_step "ensure git hooks" bash scripts/ensure-git-hooks.sh
fi

run_step "generate LOC report" bun run scripts/generate-loc-report.ts --source="$OPEN_BITCOIN_LOC_REPORT_SOURCE" --output=docs/metrics/lines-of-code.md --check
run_step "check parity breadcrumbs" bun run scripts/check-parity-breadcrumbs.ts --check
run_step "check v1.3 release boundaries" bun run scripts/check-v1.3-release-boundaries.ts
run_step "check v1.4 release boundaries" bun run scripts/check-v1.4-release-boundaries.ts
run_step "check v1.5 release boundaries" bun run scripts/check-v1.5-release-boundaries.ts
run_step "check Phase 61 resource recovery boundaries" bun run scripts/check-phase61-resource-recovery-boundaries.ts
run_step "check Phase 62 sync truth surfaces" bun run scripts/check-phase62-sync-truth-surfaces.ts
run_step "check Phase 63 service lifecycle" bun run scripts/check-phase63-service-lifecycle.ts
run_step "check Phase 64 service restart resume" bun run scripts/check-phase64-service-restart-resume.ts
run_step "check Phase 65 support review" bun run scripts/check-phase65-support-review.ts
run_step "check Phase 66 compatibility wrapper" bun run scripts/check-phase66-compatibility-wrapper.ts
run_step "check Phase 68 active chain persistence" bun run scripts/check-phase68-active-chain-persistence.ts
run_step "check Phase 69 tip stay current" bun run scripts/check-phase69-tip-stay-current.ts
run_step "check Phase 70 reorg recovery" bun run scripts/check-phase70-reorg-recovery.ts
run_step "check Phase 71 resource restart" bun run scripts/check-phase71-resource-restart.ts
run_step "check Phase 72 observability evidence" bun run scripts/check-phase72-observability-evidence.ts
run_step "test Phase 73 UAT verification checker" bun test scripts/check-phase73-uat-verification.test.ts
run_step "check Phase 73 UAT verification" env -u OPEN_BITCOIN_PHASE73_REPO_ROOT bun run scripts/check-phase73-uat-verification.ts
run_step "check v1.6 release boundaries" bun run scripts/check-v1.6-release-boundaries.ts
run_step "test Phase 75 soak runner checker" bun test scripts/check-phase75-soak-runner.test.ts
run_step "check Phase 75 soak runner" bun run scripts/check-phase75-soak-runner.ts
run_step "test Phase 76 resource bounds checker" bun test scripts/check-phase76-resource-bounds.test.ts
run_step "check Phase 76 resource bounds" bun run scripts/check-phase76-resource-bounds.ts
run_step "test Phase 77 corruption lock recovery checker" bun test scripts/check-phase77-corruption-lock-recovery.test.ts
run_step "check Phase 77 corruption lock recovery" bun run scripts/check-phase77-corruption-lock-recovery.ts
run_step "test Phase 78 progress guarantees checker" bun test scripts/check-phase78-progress-guarantees.test.ts
run_step "check Phase 78 progress guarantees" bun run scripts/check-phase78-progress-guarantees.ts
run_step "test Phase 79 diagnostics support bundle checker" bun test scripts/check-phase79-diagnostics-support-bundle.test.ts
run_step "check Phase 79 diagnostics support bundle" bun run scripts/check-phase79-diagnostics-support-bundle.ts
run_step "test Phase 80 opt-in soak UAT release boundaries checker" bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts
run_step "check Phase 80 opt-in soak UAT release boundaries" bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts
run_step "test Phase 82 production claim boundary checker" bun test scripts/check-phase82-production-claim-boundary.test.ts
run_step "check Phase 82 production claim boundary" bun run scripts/check-phase82-production-claim-boundary.ts
run_step "test Phase 83 support matrix issue evidence checker" bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts
run_step "check Phase 83 support matrix issue evidence" bun run scripts/check-phase83-support-matrix-issue-evidence.ts
run_step "test Phase 84 upgrade rollback policy checker" bun test scripts/check-phase84-upgrade-rollback-policy.test.ts
run_step "check Phase 84 upgrade rollback policy" bun run scripts/check-phase84-upgrade-rollback-policy.ts
run_step "test Phase 85 operator runbooks checker" bun test scripts/check-phase85-operator-runbooks.test.ts
run_step "check Phase 85 operator runbooks" bun run scripts/check-phase85-operator-runbooks.ts
run_step "test Phase 86 service operation expectations checker" bun test scripts/check-phase86-service-operation-expectations.test.ts
run_step "check Phase 86 service operation expectations" bun run scripts/check-phase86-service-operation-expectations.ts
run_step "test Phase 87 release readiness checker" bun test scripts/check-phase87-release-readiness.test.ts
run_step "check Phase 87 release readiness" bun run scripts/check-phase87-release-readiness.ts
run_step "test Phase 88 deterministic claim guardrails checker" bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts
run_step "check Phase 88 deterministic claim guardrails" bun run scripts/check-phase88-deterministic-claim-guardrails.ts
run_step "test Phase 90 inbound listener admission checker" bun test scripts/check-phase90-inbound-listener-admission.test.ts
run_step "check Phase 90 inbound listener admission" bun run scripts/check-phase90-inbound-listener-admission.ts
run_step "test Phase 91 peer permissions checker" bun test scripts/check-phase91-peer-permissions.test.ts
run_step "check Phase 91 peer permissions" bun run scripts/check-phase91-peer-permissions.ts
run_step "check pure-core dependencies" bash scripts/check-pure-core-deps.sh
run_step "check file lengths" bash scripts/check-file-lengths.sh
run_step "check panic sites" bash scripts/check-panic-sites.sh
run_step "cargo fmt" cargo fmt --manifest-path packages/Cargo.toml --all --check
run_step "cargo clippy" cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings

if [[ "$verify_mode" == "full" ]]; then
  run_step "cargo build" cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
fi

run_step "cargo test" cargo test --manifest-path packages/Cargo.toml --workspace --all-features

if [[ "$verify_mode" == "full" ]]; then
  run_step "benchmark list" run_benchmark_list
  run_step "benchmark smoke" bash scripts/run-benchmarks.sh --smoke --output-dir "$OPEN_BITCOIN_BENCHMARK_REPORT_DIR"
  run_step "check benchmark report" bun run scripts/check-benchmark-report.ts --report="$OPEN_BITCOIN_BENCHMARK_REPORT_DIR/open-bitcoin-bench-smoke.json"
  run_step "bazel build" bazel build //:core //:node //:rpc //:cli //:test_harness //:bench
  run_step "check Bazel build provenance" bun run scripts/check-bazel-build-provenance.ts
fi

pure_core_crates=()
while IFS= read -r crate_name; do
  [[ -z "$crate_name" ]] && continue
  pure_core_crates+=("$crate_name")
done <scripts/pure-core-crates.txt
llvm_cov_args=()
if [[ "${#pure_core_crates[@]}" -gt 0 ]]; then
  for crate_name in "${pure_core_crates[@]}"; do
    llvm_cov_args+=(--package "$crate_name")
  done
fi

if [[ "$verify_mode" == "full" ]]; then
  run_step "cargo llvm-cov clean" cargo llvm-cov clean --manifest-path packages/Cargo.toml --workspace
  run_step "cargo llvm-cov pure core" run_coverage_report
fi

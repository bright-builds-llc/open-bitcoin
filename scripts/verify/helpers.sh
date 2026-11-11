#!/usr/bin/env bash
# Sourced by scripts/verify.sh. Do not execute directly.

usage() {
  cat >&2 <<'EOF'
usage: bash scripts/verify.sh [--full | --profile | --fast | --quick] [--timings]
                             [--shard <name>] [--parallel] [--plan] [--list-shards]

Modes:
  --full      Run the full strict verification contract (default).
  --profile   Run the full strict verification contract and print step timings.
  --fast      Run local fast checks; skip benchmark smoke, Bazel smoke, and coverage.
  --quick     Run the short pre-commit integrity and formatting checks.

Options:
  --timings   Print step timings for the selected mode.
  --shard <name>
              Run only the steps of one shard of the full contract. The shards
              partition the contract, so running every shard equals a full run.
  --parallel  Run the full contract with the checker-test shards concurrent
              beside the serial native shards. Same steps as a serial full run.
  --plan      Print each step as "<shard><TAB><label>" without running it.
  --list-shards
              Print the shard names, one per line, and exit.
EOF
}

# Bun's 5s default fails correct mutation cases on slow runners; these suites
# assert refusal behavior, not speed. Files that set their own default keep it.
readonly BUN_TEST_DEFAULT_TIMEOUT_MILLISECONDS=60000

# Phase-checker suites loop one test over large tables; the preload keeps a
# spread of rows per call site without editing the text-pinned test files.
# OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS=1 runs every case.
readonly BUN_TEST_SAMPLING_PRELOAD="./scripts/verify/sample-checker-tests.ts"

# Every step belongs to exactly one shard; unmatched steps fall into "checks"
# or "checker-tests-other", so a new step can never be left out of CI.
readonly VERIFY_SHARDS=(
  checks
  native-test
  native-stress
  native-release
  checker-tests-158
  checker-tests-159-163
  checker-tests-164
  checker-tests-165
  checker-tests-other
)
maybe_verify_shard=""
verify_plan_only=0
verify_parallel=0
readonly PARALLEL_POLL_SECONDS=1

is_verify_shard() {
  local candidate="$1"
  local shard=""

  for shard in "${VERIFY_SHARDS[@]}"; do
    if [[ "$shard" == "$candidate" ]]; then
      return 0
    fi
  done
  return 1
}

bun_test_shard() {
  local argument=""

  for argument in "$@"; do
    case "$argument" in
    *check-phase158-validated-reorg.test.ts) echo "checker-tests-158" && return ;;
    *check-phase159-filter-rpcs.test.ts | *check-phase163-provenance/native.test.ts) echo "checker-tests-159-163" && return ;;
    *check-phase164-canonical-identity.test.ts) echo "checker-tests-164" && return ;;
    *check-phase165-shared-exact-work.test.ts) echo "checker-tests-165" && return ;;
    esac
  done
  echo "checker-tests-other"
}

step_shard() {
  local label="$1"
  shift

  if [[ "${1:-}" == bun && "${2:-}" == test ]]; then
    bun_test_shard "$@"
    return
  fi

  case "$label" in
  "stress "*) echo "native-stress" ;;
  "cargo build" | "cargo test" | "cargo test "*) echo "native-test" ;;
  "benchmark "* | "check benchmark report" | "bazel build" | "check Bazel build provenance" | "cargo llvm-cov "*)
    echo "native-release"
    ;;
  *) echo "checks" ;;
  esac
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
  # Keep a failing step from aborting this EXIT-trap reporter before its summary.
  set +e
  local verify_end_milliseconds=0
  local elapsed_milliseconds=0
  local elapsed_display=""
  if [[ -n "$current_step_label" && -n "$current_step_started_milliseconds" ]]; then
    verify_end_milliseconds="$(current_time_milliseconds)"
    record_step_timing \
      "$current_step_label" \
      "$((verify_end_milliseconds - current_step_started_milliseconds))" \
      "$exit_status" \
      "$current_step_started_milliseconds"
    current_step_label=""
    current_step_started_milliseconds=""
  fi
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

  if [[ -n "$timing_batch_file" && -s "$timing_batch_file" ]]; then
    if ! bun run scripts/command-timings.ts record-batch \
      --file "$timing_batch_file" \
      --source verify \
      --verify-mode "$verify_invocation"; then
      echo "warning: failed to persist local verifier timing history" >&2
    fi
  fi
  if [[ -n "$timing_batch_file" ]]; then
    rm -f "$timing_batch_file"
  fi

  return "$exit_status"
}

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
      verify_invocation="full"
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
      verify_invocation="profile"
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
      verify_invocation="fast"
      mode_seen=1
      shift
      ;;
    --quick)
      if [[ "$mode_seen" -eq 1 ]]; then
        echo "error: choose only one verification mode" >&2
        usage
        exit 2
      fi
      verify_mode="quick"
      verify_invocation="quick"
      mode_seen=1
      shift
      ;;
    --timings)
      print_timings=1
      shift
      ;;
    --shard)
      if ! is_verify_shard "${2:-}"; then
        echo "error: --shard requires one of: ${VERIFY_SHARDS[*]}" >&2
        exit 2
      fi
      maybe_verify_shard="$2"
      shift 2
      ;;
    --parallel)
      verify_parallel=1
      shift
      ;;
    --plan)
      verify_plan_only=1
      shift
      ;;
    --list-shards)
      printf '%s\n' "${VERIFY_SHARDS[@]}"
      exit 0
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

  if [[ "$verify_parallel" -eq 1 ]]; then
    if [[ "$verify_mode" != "full" || -n "$maybe_verify_shard" ]]; then
      echo "error: --parallel applies only to the whole full verification contract" >&2
      exit 2
    fi
    verify_invocation="parallel"
    return
  fi
  if [[ -z "$maybe_verify_shard" ]]; then
    return
  fi
  if [[ "$verify_mode" != "full" ]]; then
    echo "error: --shard applies only to the full verification contract" >&2
    exit 2
  fi
  # Keep shard runs out of the full-run local timing history.
  verify_invocation="shard-${maybe_verify_shard}"
}

record_step_timing() {
  local label="$1"
  local duration_milliseconds="$2"
  local status="$3"
  local started_at_milliseconds="${4:-0}"
  local timing_key=""

  step_labels+=("$label")
  step_durations+=("$duration_milliseconds")
  step_statuses+=("$status")

  if [[ -n "$timing_batch_file" ]]; then
    timing_key="$(printf '%s' "$label" |
      tr '[:upper:]' '[:lower:]' |
      sed -E 's/[^a-z0-9._-]+/-/g; s/^-+//; s/-+$//')"
    printf 'verify-step-%s\t%s\t%s\t%s\n' \
      "$timing_key" \
      "$started_at_milliseconds" \
      "$duration_milliseconds" \
      "$status" >>"$timing_batch_file"
  fi
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

is_background_shard() {
  [[ "$1" == checker-tests-* ]]
}

stop_background_shards() {
  local process_group=""

  for process_group in "$@"; do
    kill -TERM -- "-${process_group}" 2>/dev/null || true
  done
}

# Prints the first background shard whose status file records a failure.
failed_background_shard() {
  local log_directory="$1"
  local status_file=""

  for status_file in "$log_directory"/*.status; do
    [[ -f "$status_file" ]] || continue
    if [[ "$(<"$status_file")" != 0 ]]; then
      basename "$status_file" .status
      return 0
    fi
  done
  return 1
}

report_failed_shard() {
  local log_directory="$1"
  local shard="$2"

  echo "verify.sh: shard ${shard} failed" >&2
  if [[ -f "$log_directory/$shard.log" ]]; then
    echo "verify.sh: last lines of ${shard} (full log: $log_directory/$shard.log)" >&2
    tail -n 60 "$log_directory/$shard.log" >&2
  fi
}

# Runs every shard of the full contract: the Bun checker-test shards in the
# background, the Cargo and Bazel shards serially because they share one
# target directory. Children inherit the timing wrapper and its build lock.
run_parallel_shards() {
  local log_directory=""
  local shard=""
  local maybe_failed=""
  local background_shards=()
  local background_groups=()
  local pending=0

  log_directory="$(mktemp -d)"
  # Job control gives each background shard its own process group, so a
  # failure elsewhere can stop its whole process tree.
  set -m
  for shard in "${VERIFY_SHARDS[@]}"; do
    is_background_shard "$shard" || continue
    (
      status=0
      bash scripts/verify.sh --shard "$shard" >"$log_directory/$shard.log" 2>&1 || status="$?"
      echo "$status" >"$log_directory/$shard.status"
    ) &
    background_shards+=("$shard")
    background_groups+=("$!")
  done
  set +m
  echo "verify.sh: background shards ${background_shards[*]} (logs: $log_directory)" >&2

  for shard in "${VERIFY_SHARDS[@]}"; do
    is_background_shard "$shard" && continue
    echo "verify.sh: running shard ${shard}" >&2
    if ! bash scripts/verify.sh --shard "$shard"; then
      maybe_failed="$shard"
    elif maybe_failed="$(failed_background_shard "$log_directory")"; then
      :
    fi
    if [[ -n "$maybe_failed" ]]; then
      stop_background_shards "${background_groups[@]}"
      report_failed_shard "$log_directory" "$maybe_failed"
      return 1
    fi
  done

  echo "verify.sh: native shards passed; waiting for ${background_shards[*]}" >&2
  while :; do
    if maybe_failed="$(failed_background_shard "$log_directory")"; then
      stop_background_shards "${background_groups[@]}"
      report_failed_shard "$log_directory" "$maybe_failed"
      return 1
    fi
    pending=0
    for shard in "${background_shards[@]}"; do
      [[ -f "$log_directory/$shard.status" ]] || pending=$((pending + 1))
    done
    [[ "$pending" -eq 0 ]] && break
    sleep "$PARALLEL_POLL_SECONDS"
  done
  rm -rf "$log_directory"
  echo "verify.sh: all ${#VERIFY_SHARDS[@]} shards passed" >&2
}

run_step() {
  local label="$1"
  shift

  local step_start_milliseconds=0
  local step_end_milliseconds=0
  local step_duration_milliseconds=0
  local status=0
  local command_args=("$@")
  local argument_index=0
  local shard=""

  shard="$(step_shard "$label" "$@")"
  if [[ "$verify_plan_only" -eq 1 ]]; then
    printf '%s\t%s\n' "$shard" "$label"
    return 0
  fi
  # The first step of a parallel run hands the whole contract to the shards.
  # It starts here, not in parse_args, so it runs inside the timing wrapper.
  if [[ "$verify_parallel" -eq 1 ]]; then
    run_parallel_shards || exit "$?"
    exit 0
  fi
  if [[ -n "$maybe_verify_shard" && "$shard" != "$maybe_verify_shard" ]]; then
    return 0
  fi

  if [[ "${command_args[0]:-}" == bun && "${command_args[1]:-}" == test ]]; then
    for ((argument_index = 2; argument_index < ${#command_args[@]}; argument_index++)); do
      case "${command_args[$argument_index]}" in
      -*) break ;;
      /* | ./* | ../*) continue ;;
      *.test.ts)
        if [[ -f "${command_args[$argument_index]}" ]]; then
          command_args[$argument_index]="./${command_args[$argument_index]}"
        fi
        ;;
      esac
    done
    command_args+=(--timeout "$BUN_TEST_DEFAULT_TIMEOUT_MILLISECONDS" --preload "$BUN_TEST_SAMPLING_PRELOAD")
  fi

  step_start_milliseconds="$(current_time_milliseconds)"
  current_step_label="$label"
  current_step_started_milliseconds="$step_start_milliseconds"
  set +e
  "${command_args[@]}"
  status="$?"
  set -e
  step_end_milliseconds="$(current_time_milliseconds)"
  step_duration_milliseconds=$((step_end_milliseconds - step_start_milliseconds))
  record_step_timing "$label" "$step_duration_milliseconds" "$status" "$step_start_milliseconds"
  current_step_label=""
  current_step_started_milliseconds=""
  return "$status"
}

run_quick_mode() {
  run_step "test parity ledger" bun test ./scripts/check-parity-ledger.test.ts
  run_step "check parity ledger" bun run scripts/check-parity-ledger.ts
  run_step "test push verification" bun test ./scripts/check-push-verification.test.ts
  run_step "test verify shards" bun test ./scripts/verify-shards.test.ts
  run_step "test checker case sampling" bun test ./scripts/verify/sample-checker-tests.test.ts
  run_step "check Bright Builds rules" bun scripts/bright-builds-check.ts all
  run_step "check Phase 164 canonical identity boundary" bun run scripts/check-phase164-canonical-identity.ts
  run_step "check Phase 167 candidate history boundary" bun run scripts/check-phase167-candidate-history.ts
  run_step "check current documentation reconciliation" bun run scripts/check-current-documentation-reconciliation.ts
  run_step "cargo fmt" cargo fmt --manifest-path packages/Cargo.toml --all --check
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

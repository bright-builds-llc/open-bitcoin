#!/usr/bin/env bash
# Sourced by scripts/verify.sh. Do not execute directly.

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
  local started_at_milliseconds="${4:-0}"
  local timing_key=""

  step_labels+=("$label")
  step_durations+=("$duration_milliseconds")
  step_statuses+=("$status")

  if [[ -n "$timing_batch_file" ]]; then
    timing_key="$(printf '%s' "$label" \
      | tr '[:upper:]' '[:lower:]' \
      | sed -E 's/[^a-z0-9._-]+/-/g; s/^-+//; s/-+$//')"
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

run_step() {
  local label="$1"
  shift

  local step_start_milliseconds=0
  local step_end_milliseconds=0
  local step_duration_milliseconds=0
  local status=0

  step_start_milliseconds="$(current_time_milliseconds)"
  current_step_label="$label"
  current_step_started_milliseconds="$step_start_milliseconds"
  set +e
  "$@"
  status="$?"
  set -e
  step_end_milliseconds="$(current_time_milliseconds)"
  step_duration_milliseconds=$((step_end_milliseconds - step_start_milliseconds))
  record_step_timing "$label" "$step_duration_milliseconds" "$status" "$step_start_milliseconds"
  current_step_label=""
  current_step_started_milliseconds=""
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

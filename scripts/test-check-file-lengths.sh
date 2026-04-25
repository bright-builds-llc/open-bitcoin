#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
helper_script="${script_dir}/check-file-lengths.sh"
verify_script="${script_dir}/verify.sh"
readonly pi="3.141592653589793"
tau="$(awk -v pi="$pi" 'BEGIN { printf "%.15f", 2 * pi }')"
readonly tau
expected_max_file_lines="$(awk -v tau="$tau" 'BEGIN { printf "%d", int(tau * 100) }')"
readonly expected_max_file_lines

tmp_root="$(mktemp -d)"
trap 'rm -rf "$tmp_root"' EXIT

assert_contains() {
  local haystack="$1"
  local needle="$2"

  if [[ "$haystack" != *"$needle"* ]]; then
    echo "expected output to contain: $needle" >&2
    exit 1
  fi
}

assert_not_contains() {
  local haystack="$1"
  local needle="$2"

  if [[ "$haystack" == *"$needle"* ]]; then
    echo "expected output not to contain: $needle" >&2
    exit 1
  fi
}

write_rust_file() {
  local path="$1"
  local line_count="$2"

  mkdir -p "$(dirname "$path")"
  awk -v count="$line_count" 'BEGIN { for (i = 1; i <= count; i++) print "fn line_" i "() {}"; }' >"$path"
}

init_repo() {
  local repo_dir="$1"

  mkdir -p "$repo_dir"
  (
    cd "$repo_dir"
    git init -q
    git config user.name "Codex Test"
    git config user.email "codex@example.com"
  )
}

write_verify_test_fixture() {
  local repo_dir="$1"
  local fake_bin="$2"

  mkdir -p "${repo_dir}/scripts" "$fake_bin"
  cp "$helper_script" "${repo_dir}/scripts/check-file-lengths.sh"
  chmod +x "${repo_dir}/scripts/check-file-lengths.sh"

  cat >"${repo_dir}/scripts/check-pure-core-deps.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "Pure-core dependency and import checks passed."
EOF
  chmod +x "${repo_dir}/scripts/check-pure-core-deps.sh"

  cat >"${repo_dir}/scripts/check-panic-sites.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "check-panic-sites.sh: no unclassified production panic-like sites"
EOF
  chmod +x "${repo_dir}/scripts/check-panic-sites.sh"

  cat >"${repo_dir}/scripts/run-benchmarks.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
exit 0
EOF
  chmod +x "${repo_dir}/scripts/run-benchmarks.sh"

  touch "${repo_dir}/scripts/pure-core-crates.txt"

  cat >"${fake_bin}/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
touch "${VERIFY_MARKER_DIR:?}/cargo-called"
exit 0
EOF
  cat >"${fake_bin}/cargo-llvm-cov" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
touch "${VERIFY_MARKER_DIR:?}/cargo-llvm-cov-called"
exit 0
EOF
  cat >"${fake_bin}/bazel" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
touch "${VERIFY_MARKER_DIR:?}/bazel-called"
exit 0
EOF
  cat >"${fake_bin}/bun" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
touch "${VERIFY_MARKER_DIR:?}/bun-called"
if [[ "${1:-}" == "--print" ]]; then
  printf '%s' "0"
  exit 0
fi
if [[ "${1:-}" == "run" && "${2:-}" == "scripts/generate-loc-report.ts" ]]; then
  touch "${VERIFY_MARKER_DIR:?}/loc-report-called"
  exit 0
fi
echo "unexpected bun invocation: $*" >&2
exit 1
EOF
  chmod +x "${fake_bin}/cargo" "${fake_bin}/cargo-llvm-cov" "${fake_bin}/bazel" "${fake_bin}/bun"
}

run_positive_case() {
  local repo_dir="${tmp_root}/positive"
  local output=""

  init_repo "$repo_dir"
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/lib.rs" 40

  (
    cd "$repo_dir"
    git add packages/open-bitcoin-foo/src/lib.rs
    output="$("$helper_script" 2>&1)"
    printf '%s' "$output" >positive-output.txt
  )

  output="$(cat "${repo_dir}/positive-output.txt")"
  assert_contains "$output" "Production Rust file-length check passed"
}

run_negative_case() {
  local repo_dir="${tmp_root}/negative"
  local output=""

  init_repo "$repo_dir"
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/oversized.rs" "$expected_max_file_lines"

  (
    cd "$repo_dir"
    git add packages/open-bitcoin-foo/src/oversized.rs
    set +e
    output="$("$helper_script" 2>&1)"
    status=$?
    set -e
    printf '%s' "$output" >negative-output.txt
    printf '%s' "$status" >negative-status.txt
  )

  output="$(cat "${repo_dir}/negative-output.txt")"
  status="$(cat "${repo_dir}/negative-status.txt")"
  if [[ "$status" -eq 0 ]]; then
    echo "negative case should fail" >&2
    exit 1
  fi

  assert_contains "$output" "packages/open-bitcoin-foo/src/oversized.rs"
  assert_contains "$output" "${expected_max_file_lines} lines"
  assert_contains "$output" "Move inline tests into a sibling tests.rs file."
  assert_contains "$output" "Split the file into foo.rs plus foo/ child modules."
}

run_scope_case() {
  local repo_dir="${tmp_root}/scope"
  local output=""

  init_repo "$repo_dir"
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/lib.rs" 20
  write_rust_file "${repo_dir}/packages/bitcoin-knots/src/ignored.rs" 900
  write_rust_file "${repo_dir}/packages/target/debug/generated.rs" 900
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/tests.rs" 900
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/tests/helper.rs" 900

  (
    cd "$repo_dir"
    git add packages
    output="$("$helper_script" 2>&1)"
    printf '%s' "$output" >scope-output.txt
  )

  output="$(cat "${repo_dir}/scope-output.txt")"
  assert_contains "$output" "Production Rust file-length check passed"
}

run_verify_integration_case() {
  local repo_dir="${tmp_root}/integration"
  local fake_bin="${repo_dir}/fake-bin"
  local output=""

  init_repo "$repo_dir"
  write_verify_test_fixture "$repo_dir" "$fake_bin"
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/oversized.rs" "$expected_max_file_lines"

  (
    cd "$repo_dir"
    git add packages scripts
    set +e
    output="$(PATH="${fake_bin}:$PATH" VERIFY_MARKER_DIR="$repo_dir" bash "$verify_script" 2>&1)"
    status=$?
    set -e
    printf '%s' "$output" >integration-output.txt
    printf '%s' "$status" >integration-status.txt
  )

  output="$(cat "${repo_dir}/integration-output.txt")"
  status="$(cat "${repo_dir}/integration-status.txt")"
  if [[ "$status" -eq 0 ]]; then
    echo "integration case should fail" >&2
    exit 1
  fi

  assert_contains "$output" "production Rust files must stay below ${expected_max_file_lines} lines"
  assert_contains "$output" "verify.sh failed after "
  assert_contains "$output" "ms)"
  if [[ -e "${repo_dir}/cargo-called" || -e "${repo_dir}/cargo-llvm-cov-called" || -e "${repo_dir}/bazel-called" ]]; then
    echo "verify.sh should stop before expensive cargo/bazel work" >&2
    exit 1
  fi
}

run_verify_success_timing_case() {
  local repo_dir="${tmp_root}/verify-success"
  local fake_bin="${repo_dir}/fake-bin"
  local output=""

  init_repo "$repo_dir"
  write_verify_test_fixture "$repo_dir" "$fake_bin"
  write_rust_file "${repo_dir}/packages/open-bitcoin-foo/src/lib.rs" 40

  (
    cd "$repo_dir"
    git add packages scripts
    set +e
    output="$(PATH="${fake_bin}:$PATH" VERIFY_MARKER_DIR="$repo_dir" bash "$verify_script" 2>&1)"
    status=$?
    set -e
    printf '%s' "$output" >success-output.txt
    printf '%s' "$status" >success-status.txt
  )

  output="$(cat "${repo_dir}/success-output.txt")"
  status="$(cat "${repo_dir}/success-status.txt")"
  if [[ "$status" -ne 0 ]]; then
    echo "verify success case should pass" >&2
    exit 1
  fi

  assert_contains "$output" "verify.sh completed in "
  assert_contains "$output" "("
  assert_contains "$output" "ms)"
  assert_not_contains "$output" "verify.sh failed after "
}

run_positive_case
run_negative_case
run_scope_case
run_verify_integration_case
run_verify_success_timing_case

echo "check-file-lengths tests passed."

#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
helper_script="${script_dir}/ensure-git-hooks.sh"
install_script="${script_dir}/install-git-hooks.sh"

tmp_root="$(mktemp -d)"
trap 'rm -rf "$tmp_root"' EXIT

assert_eq() {
  local actual="$1"
  local expected="$2"
  if [[ "$actual" != "$expected" ]]; then
    echo "expected '$expected' but got '$actual'" >&2
    exit 1
  fi
}

assert_contains() {
  local haystack="$1"
  local needle="$2"
  if [[ "$haystack" != *"$needle"* ]]; then
    echo "expected output to contain: $needle" >&2
    exit 1
  fi
}

init_repo() {
  local repo_dir="$1"
  mkdir -p "${repo_dir}/scripts" "${repo_dir}/.githooks"
  cp "$helper_script" "${repo_dir}/scripts/ensure-git-hooks.sh"
  cp "$install_script" "${repo_dir}/scripts/install-git-hooks.sh"
  chmod +x "${repo_dir}/scripts/ensure-git-hooks.sh" "${repo_dir}/scripts/install-git-hooks.sh"
  cat >"${repo_dir}/.githooks/pre-commit" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
exit 0
EOF
  chmod +x "${repo_dir}/.githooks/pre-commit"
  (
    cd "$repo_dir"
    git init -q
    git config user.name "Codex Test"
    git config user.email "codex@example.com"
  )
}

run_missing_config_case() {
  local repo_dir="${tmp_root}/missing-config"
  local output=""

  init_repo "$repo_dir"
  (
    cd "$repo_dir"
    output="$(bash scripts/ensure-git-hooks.sh 2>&1)"
    printf '%s' "$output" >output.txt
    git config --local --get core.hooksPath >hooks-path.txt
  )

  output="$(cat "${repo_dir}/output.txt")"
  assert_contains "$output" "Configured git hooks: core.hooksPath=.githooks"
  assert_eq "$(cat "${repo_dir}/hooks-path.txt")" ".githooks"
}

run_idempotent_case() {
  local repo_dir="${tmp_root}/idempotent"
  local output=""

  init_repo "$repo_dir"
  (
    cd "$repo_dir"
    git config --local core.hooksPath .githooks
    output="$(bash scripts/ensure-git-hooks.sh 2>&1)"
    printf '%s' "$output" >output.txt
  )

  output="$(cat "${repo_dir}/output.txt")"
  assert_contains "$output" "Git hooks already configured: core.hooksPath=.githooks"
}

run_wrong_config_case() {
  local repo_dir="${tmp_root}/wrong-config"

  init_repo "$repo_dir"
  (
    cd "$repo_dir"
    git config --local core.hooksPath .git/hooks
    bash scripts/ensure-git-hooks.sh >/dev/null
    git config --local --get core.hooksPath >hooks-path.txt
  )

  assert_eq "$(cat "${repo_dir}/hooks-path.txt")" ".githooks"
}

run_missing_hook_case() {
  local repo_dir="${tmp_root}/missing-hook"
  local output=""
  local status=""

  init_repo "$repo_dir"
  rm "${repo_dir}/.githooks/pre-commit"
  (
    cd "$repo_dir"
    set +e
    output="$(bash scripts/ensure-git-hooks.sh 2>&1)"
    status=$?
    set -e
    printf '%s' "$output" >output.txt
    printf '%s' "$status" >status.txt
  )

  output="$(cat "${repo_dir}/output.txt")"
  status="$(cat "${repo_dir}/status.txt")"
  if [[ "$status" -eq 0 ]]; then
    echo "missing hook case should fail" >&2
    exit 1
  fi
  assert_contains "$output" "missing required executable hook: .githooks/pre-commit"
}

run_install_wrapper_case() {
  local repo_dir="${tmp_root}/install-wrapper"
  local output=""

  init_repo "$repo_dir"
  (
    cd "$repo_dir"
    output="$(bash scripts/install-git-hooks.sh 2>&1)"
    printf '%s' "$output" >output.txt
    git config --local --get core.hooksPath >hooks-path.txt
  )

  output="$(cat "${repo_dir}/output.txt")"
  assert_contains "$output" "Git hooks are configured from .githooks"
  assert_contains "$output" "Re-running this installer is safe and idempotent."
  assert_eq "$(cat "${repo_dir}/hooks-path.txt")" ".githooks"
}

run_missing_config_case
run_idempotent_case
run_wrong_config_case
run_missing_hook_case
run_install_wrapper_case

echo "ensure-git-hooks tests passed."

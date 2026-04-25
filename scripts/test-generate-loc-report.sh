#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
generator_script="${script_dir}/generate-loc-report.ts"
report_path="docs/metrics/lines-of-code.md"

tmp_root="$(mktemp -d)"
trap 'rm -rf "$tmp_root"' EXIT

assert_contains() {
  local file_path="$1"
  local needle="$2"

  if ! grep -Fq "$needle" "$file_path"; then
    echo "expected ${file_path} to contain: ${needle}" >&2
    exit 1
  fi
}

assert_not_contains() {
  local file_path="$1"
  local needle="$2"

  if grep -Fq "$needle" "$file_path"; then
    echo "expected ${file_path} not to contain: ${needle}" >&2
    exit 1
  fi
}

write_lines() {
  local file_path="$1"
  local line_count="$2"
  local prefix="$3"

  mkdir -p "$(dirname "$file_path")"
  awk -v count="$line_count" -v prefix="$prefix" 'BEGIN { for (i = 1; i <= count; i++) print prefix " " i; }' >"$file_path"
}

init_repo() {
  local repo_dir="$1"

  mkdir -p "${repo_dir}/scripts"
  cp "$generator_script" "${repo_dir}/scripts/generate-loc-report.ts"
  chmod +x "${repo_dir}/scripts/generate-loc-report.ts"
  (
    cd "$repo_dir"
    git init -q
    git config user.name "Codex Test"
    git config user.email "codex@example.com"
  )
}

run_scope_and_structure_case() {
  local repo_dir="${tmp_root}/scope"

  init_repo "$repo_dir"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/src/lib.rs" 30 "pub fn alpha_"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/src/tests.rs" 5 "// test"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/tests/parity.rs" 4 "// parity"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/Cargo.toml" 3 "[package]"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/BUILD.bazel" 3 "rust_library("
  write_lines "${repo_dir}/packages/open-bitcoin-codec/testdata/vector.hex" 2 "abcdef"
  write_lines "${repo_dir}/scripts/tool.sh" 4 "echo tool"
  write_lines "${repo_dir}/.github/workflows/ci.yml" 3 "name: ci"
  write_lines "${repo_dir}/.githooks/pre-commit" 3 "bash scripts/verify.sh"
  write_lines "${repo_dir}/BUILD.bazel" 2 "alias("
  write_lines "${repo_dir}/packages/bitcoin-knots/src/ignored.rs" 60 "vendored"
  write_lines "${repo_dir}/packages/target/generated.rs" 60 "generated"
  write_lines "${repo_dir}/.planning/STATE.md" 60 "planning"
  write_lines "${repo_dir}/docs/notes.md" 60 "docs"
  write_lines "${repo_dir}/${report_path}" 60 "old report"

  (
    cd "$repo_dir"
    git add .
    bun run scripts/generate-loc-report.ts --source=worktree --output="$report_path"
  )

  assert_contains "${repo_dir}/${report_path}" "# Lines Of Code Report"
  assert_contains "${repo_dir}/${report_path}" "## Aggregate"
  assert_contains "${repo_dir}/${report_path}" "## Per-Crate Modules"
  assert_contains "${repo_dir}/${report_path}" "## Language And Category Breakdown"
  assert_contains "${repo_dir}/${report_path}" "## Largest Included Files"
  assert_contains "${repo_dir}/${report_path}" "## Metadata"
  assert_contains "${repo_dir}/${report_path}" "open-bitcoin-alpha"
  assert_contains "${repo_dir}/${report_path}" "open-bitcoin-codec"
  assert_contains "${repo_dir}/${report_path}" "Rust production"
  assert_contains "${repo_dir}/${report_path}" "Rust tests"
  assert_contains "${repo_dir}/${report_path}" "Shell scripts"
  assert_contains "${repo_dir}/${report_path}" "YAML"
  assert_contains "${repo_dir}/${report_path}" "Hooks"
  assert_contains "${repo_dir}/${report_path}" "Fixture/data"
  assert_not_contains "${repo_dir}/${report_path}" "packages/bitcoin-knots/src/ignored.rs"
  assert_not_contains "${repo_dir}/${report_path}" "packages/target/generated.rs"
  assert_not_contains "${repo_dir}/${report_path}" ".planning/STATE.md"
  assert_not_contains "${repo_dir}/${report_path}" "docs/notes.md"
}

run_index_mode_case() {
  local repo_dir="${tmp_root}/index"

  init_repo "$repo_dir"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/src/lib.rs" 2 "pub fn staged_alpha_"

  (
    cd "$repo_dir"
    git add .
    bun run scripts/generate-loc-report.ts --source=index --output="$report_path"
    git add "$report_path"
    write_lines "packages/open-bitcoin-alpha/src/lib.rs" 25 "pub fn unstaged_alpha_"
    bun run scripts/generate-loc-report.ts --source=index --output="$report_path" --check

    write_lines "packages/open-bitcoin-beta/src/lib.rs" 4 "pub fn staged_beta_"
    git add packages/open-bitcoin-beta/src/lib.rs
    set +e
    bun run scripts/generate-loc-report.ts --source=index --output="$report_path" --check >check-output.txt 2>&1
    status=$?
    set -e
    if [[ "$status" -eq 0 ]]; then
      echo "index check should fail after staging a new counted file" >&2
      exit 1
    fi

    bun run scripts/generate-loc-report.ts --source=index --output="$report_path"
    git add "$report_path"
    bun run scripts/generate-loc-report.ts --source=index --output="$report_path" --check
  )

  assert_contains "${repo_dir}/${report_path}" "open-bitcoin-beta"
}

run_worktree_check_case() {
  local repo_dir="${tmp_root}/worktree"

  init_repo "$repo_dir"
  write_lines "${repo_dir}/packages/open-bitcoin-alpha/src/lib.rs" 3 "pub fn alpha_"

  (
    cd "$repo_dir"
    git add .
    bun run scripts/generate-loc-report.ts --source=worktree --output="$report_path"
    bun run scripts/generate-loc-report.ts --source=worktree --output="$report_path" --check
    write_lines "packages/open-bitcoin-alpha/src/lib.rs" 12 "pub fn changed_alpha_"

    set +e
    bun run scripts/generate-loc-report.ts --source=worktree --output="$report_path" --check >check-output.txt 2>&1
    status=$?
    set -e
    if [[ "$status" -eq 0 ]]; then
      echo "worktree check should fail after a counted file changes" >&2
      exit 1
    fi

    bun run scripts/generate-loc-report.ts --source=worktree --output="$report_path"
    bun run scripts/generate-loc-report.ts --source=worktree --output="$report_path" --check
  )
}

run_real_repo_smoke_case() {
  local smoke_report="packages/target/loc-report-smoke.md"

  (
    cd "$repo_root"
    bun run scripts/generate-loc-report.ts --source=worktree --output="$smoke_report"
  )

  assert_contains "${repo_root}/${smoke_report}" "open-bitcoin-consensus"
  assert_contains "${repo_root}/${smoke_report}" "Input fingerprint"
}

run_scope_and_structure_case
run_index_mode_case
run_worktree_check_case
run_real_repo_smoke_case

echo "generate-loc-report tests passed."

#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
cd "$repo_root"

default_output_dir="${OPEN_BITCOIN_BENCHMARK_REPORT_DIR:-$PWD/packages/target/benchmark-reports}"

usage() {
	printf '%s\n' \
		"usage: scripts/run-benchmarks.sh --list" \
		"       scripts/run-benchmarks.sh (--smoke | --full) [--iterations N] [--output-dir PATH] [--knots-json PATH] [--knots-bin PATH] [--format json|markdown]"
}

require_value() {
	local option="$1"
	local maybe_value="${2:-}"

	if [[ -n "$maybe_value" ]]; then
		return
	fi

	echo "error: ${option} requires a value" >&2
	usage >&2
	exit 2
}

benchmark_args=()
mode_count=0
output_dir_set=0
list_requested=0
maybe_mode=""

while [[ "$#" -gt 0 ]]; do
	case "$1" in
	--smoke | --full)
		maybe_mode="${1#--}"
		benchmark_args+=("$1")
		mode_count=$((mode_count + 1))
		shift
		;;
	--iterations | --output-dir | --knots-json | --knots-bin | --format)
		option="$1"
		value="${2:-}"
		require_value "$option" "$value"
		if [[ "$option" == "--output-dir" ]]; then
			output_dir_set=1
		fi
		benchmark_args+=("$option" "$value")
		shift 2
		;;
	--list)
		benchmark_args+=("$1")
		list_requested=1
		shift
		;;
	*)
		echo "error: unsupported benchmark option $1" >&2
		usage >&2
		exit 2
		;;
	esac
done

if [[ "$list_requested" -eq 1 ]]; then
	if [[ "${#benchmark_args[@]}" -ne 1 ]]; then
		echo "error: --list cannot be combined with run options" >&2
		usage >&2
		exit 2
	fi

	exec "${cargo_args[@]}" "${benchmark_args[@]}"
fi

if [[ "$mode_count" -ne 1 ]]; then
	echo "error: choose exactly one of --smoke or --full" >&2
	usage >&2
	exit 2
fi

cargo_args=(
	cargo
	run
)
if [[ "$maybe_mode" == "full" ]]; then
	cargo_args+=(--release)
fi
cargo_args+=(
	--manifest-path
	packages/Cargo.toml
	-p
	open-bitcoin-bench
	--
)

if [[ "$output_dir_set" -eq 0 ]]; then
	benchmark_args+=(--output-dir "$default_output_dir")
fi

exec "${cargo_args[@]}" "${benchmark_args[@]}"

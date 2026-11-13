#!/usr/bin/env bash

# macOS turns every durable write of the storage engine into F_FULLFSYNC, about
# 12 ms each on an internal disk, and the store-backed tests issue thousands of
# them. A RAM-backed TMPDIR makes those syncs free. Linux and CI already use a
# plain fsync and keep their normal temporary directory.

readonly VERIFY_RAMDISK_VOLUME_PREFIX="open-bitcoin-verify-"
# 512-byte sectors; RAM is committed only as files are written.
readonly VERIFY_RAMDISK_SECTORS=16777216
verify_ramdisk_device=""
# Verifier logs that must outlive the run stay in the caller's temporary parent.
verify_host_tmpdir="$TMPDIR"

wants_verify_ramdisk() {
  [[ "$(uname -s)" == "Darwin" ]] || return 1
  [[ -z "${CI:-}" && "$verify_mode" == "full" && "$verify_plan_only" -eq 0 ]] || return 1
  [[ "${OPEN_BITCOIN_VERIFY_RAMDISK:-1}" != "0" ]] || return 1
  # A shard started by a parallel run inherits its parent's volume.
  [[ "$TMPDIR" != "/Volumes/${VERIFY_RAMDISK_VOLUME_PREFIX}"* ]]
}

# A killed run cannot detach its volume; reclaim any whose owner is gone.
detach_stale_verify_ramdisks() {
  local volume=""
  local owner=""

  for volume in "/Volumes/${VERIFY_RAMDISK_VOLUME_PREFIX}"*; do
    [[ -d "$volume" ]] || continue
    owner="${volume##*-}"
    [[ "$owner" =~ ^[0-9]+$ ]] || continue
    kill -0 "$owner" 2>/dev/null && continue
    hdiutil detach "$volume" -force >/dev/null 2>&1 ||
      echo "warning: could not detach stale ${volume}" >&2
  done
}

# Points TMPDIR at a fresh RAM-backed volume. The speed-up is optional, so a
# failure keeps the caller's TMPDIR and says so.
start_verify_ramdisk() {
  local volume="${VERIFY_RAMDISK_VOLUME_PREFIX}$$"
  local output=""

  wants_verify_ramdisk || return 0
  detach_stale_verify_ramdisks
  if ! output="$(hdiutil attach -nomount "ram://${VERIFY_RAMDISK_SECTORS}" 2>&1)"; then
    echo "warning: no RAM disk for test temporary files: ${output}" >&2
    return 0
  fi
  verify_ramdisk_device="$(awk '/^\/dev\// {print $1; exit}' <<<"$output")"
  if ! output="$(diskutil erasevolume APFS "$volume" "$verify_ramdisk_device" 2>&1)"; then
    echo "warning: no RAM disk for test temporary files: ${output}" >&2
    stop_verify_ramdisk
    return 0
  fi
  TMPDIR="$(cd "/Volumes/${volume}" && pwd -P)"
  export TMPDIR
  echo "verify.sh: test temporary files on RAM disk ${TMPDIR}" >&2
}

stop_verify_ramdisk() {
  [[ -n "$verify_ramdisk_device" ]] || return 0
  TMPDIR="$verify_host_tmpdir"
  export TMPDIR
  hdiutil detach "$verify_ramdisk_device" -force >/dev/null 2>&1 ||
    echo "warning: could not detach RAM disk ${verify_ramdisk_device}" >&2
  verify_ramdisk_device=""
}

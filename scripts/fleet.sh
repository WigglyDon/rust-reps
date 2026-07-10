#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
binary=$(mktemp "${TMPDIR:-/tmp}/rust-reps-fleet.XXXXXX")

cleanup() {
    rm -f "$binary"
}

trap cleanup EXIT HUP INT TERM

rustc --edition=2021 "$repo_root/tools/fleet/main.rs" -o "$binary"

set +e
"$binary" "$@"
status=$?
set -e

exit "$status"

#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
source "${project_root}/scripts/env.sh"
cd "$project_root"

# 僅解譯兩個pure Rust targets；故意UB從不交給普通cargo run／test。
miri_toolchain="nightly-2026-10-01"
cargo "+${miri_toolchain}" miri setup
cargo "+${miri_toolchain}" miri test -p ch14-model-checking --test miri_contracts --locked
cargo "+${miri_toolchain}" miri test -p ch14-model-checking --test miri_owner_solution --locked

miri_log="$(mktemp -t learn-rust-miri-XXXXXX)"
trap 'rm -f -- "$miri_log"' EXIT
if cargo "+${miri_toolchain}" miri run \
    --manifest-path examples/ch14-model-checking/fixtures/miri-fail/Cargo.toml \
    --locked > "$miri_log" 2>&1; then
    cat "$miri_log"
    printf '%s\n' 'Miri故意失效pointer案例應以指定UB失敗' >&2
    exit 1
fi
if ! grep -Fq -- 'Undefined Behavior' "$miri_log" || \
   ! grep -Fq --file=examples/ch14-model-checking/fixtures/miri-fail/src/main.expected "$miri_log"; then
    cat "$miri_log"
    printf '%s\n' 'Miri非零結果沒有指定freed allocation診斷' >&2
    exit 1
fi
printf '%s\n' 'Miri兩個正常targets通過，故意UB命中指定freed allocation原因'

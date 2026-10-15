#!/usr/bin/env bash
set -euo pipefail
cd /workspace/learn_rust
source scripts/env.sh
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_BUILD_JOBS=2
for lesson_manifest in \
    examples/intro-workspace/Cargo.toml \
    examples/ch09-engineering/lesson-workspace/Cargo.toml \
    examples/ch09-content/lesson-workspace/Cargo.toml; do
    cargo fmt --manifest-path "$lesson_manifest" --all -- --check
    cargo clippy --manifest-path "$lesson_manifest" --workspace --all-targets --locked -- -D warnings
    cargo test --manifest-path "$lesson_manifest" --workspace --all-targets --locked
    cargo test --manifest-path "$lesson_manifest" --workspace --doc --locked
done
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --workspace --doc --locked
uv run --locked --python "3.14+gil" python scripts/check-fixtures.py
uv run --locked --python "3.14+gil" python scripts/check-c-examples.py

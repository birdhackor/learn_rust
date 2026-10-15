#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"
source "${project_root}/scripts/env.sh"

uv run --locked --python "3.14+gil" zensical build --strict
uv run --locked --python "3.14+gil" python scripts/check-tutorial-structure.py --final --site-dir site
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

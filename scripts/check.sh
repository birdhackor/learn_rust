#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"
export PATH="${HOME}/.cargo/bin:${PATH}:${HOME}/.local/bin"

uv run --locked --python "3.14+gil" zensical build --strict
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked

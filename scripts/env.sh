#!/usr/bin/env bash

# 以 source 載入；安裝腳本的 export 不會沿用到下一個 shell 或 Cloud 任務。
project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
project_parent="$(dirname -- "$project_root")"

export LEARN_RUST_TOOLS_DIR="${LEARN_RUST_TOOLS_DIR:-${project_parent}/.learn-rust-tools}"
export CARGO_HOME="${CARGO_HOME:-${LEARN_RUST_TOOLS_DIR}/cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-${LEARN_RUST_TOOLS_DIR}/rustup}"
export UV_CACHE_DIR="${UV_CACHE_DIR:-${LEARN_RUST_TOOLS_DIR}/uv-cache}"
export UV_PYTHON_INSTALL_DIR="${UV_PYTHON_INSTALL_DIR:-${LEARN_RUST_TOOLS_DIR}/python}"
export PATH="${LEARN_RUST_TOOLS_DIR}/bin:${CARGO_HOME}/bin:${PATH}"
hash -r

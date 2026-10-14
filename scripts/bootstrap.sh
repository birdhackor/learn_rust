#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"
export PATH="${HOME}/.cargo/bin:${PATH}:${HOME}/.local/bin"

if ! command -v uv >/dev/null 2>&1; then
  curl --fail --silent --show-error --location https://astral.sh/uv/0.13.0/install.sh \
    | env UV_NO_MODIFY_PATH=1 sh
fi

if ! command -v rustup >/dev/null 2>&1; then
  curl --fail --silent --show-error --location https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain none --no-modify-path
fi

# 保留與 pyenv 相容的 .python-version；只在 uv 指令中指定 GIL 版本。
uv sync --locked --python "3.14+gil"
# 自動安裝 rust-toolchain.toml 指定的工具鏈及元件。
rustup show
cargo fetch --locked

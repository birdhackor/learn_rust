#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"
source "${project_root}/scripts/env.sh"

uv_minimum_version="0.13.0"
uv_installed_version=""
if command -v uv >/dev/null 2>&1 && uv_version_output="$(uv --version 2>/dev/null)"; then
  uv_installed_version="${uv_version_output#uv }"
  uv_installed_version="${uv_installed_version%% *}"
fi

if [[ -z "$uv_installed_version" ]] || \
   [[ "$(printf '%s\n%s\n' "$uv_minimum_version" "$uv_installed_version" | sort -V | head -n 1)" != "$uv_minimum_version" ]]; then
  curl --fail --silent --show-error --location "https://astral.sh/uv/${uv_minimum_version}/install.sh" \
    | env UV_UNMANAGED_INSTALL="${LEARN_RUST_TOOLS_DIR}/bin" sh
  hash -r
fi

uv --version

if ! command -v rustup >/dev/null 2>&1; then
  curl --fail --silent --show-error --location https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain none --no-modify-path
fi

# 保留與 pyenv 相容的 .python-version；只在 uv 指令中指定 GIL 版本。
uv sync --locked --python "3.14+gil"
# 自動安裝 rust-toolchain.toml 指定的工具鏈及元件。
rustup show
cargo fetch --locked

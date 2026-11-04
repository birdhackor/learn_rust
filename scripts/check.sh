#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"
source "${project_root}/scripts/env.sh"

if [[ "$#" -gt 1 ]] || [[ "$#" -eq 1 && "$1" != --force ]]; then
  printf '%s\n' 'Usage: bash scripts/check.sh [--force]' >&2
  exit 2
fi

uv run --locked --no-default-groups --group site --python "3.14+gil" \
  python -m unittest discover -s tests -v
bash scripts/build-site.sh
uv run --locked --no-default-groups --python "3.14+gil" \
  python scripts/check-execution.py ensure --kind rust "$@"

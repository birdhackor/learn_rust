#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"
source "${project_root}/scripts/env.sh"

uv run --locked --no-default-groups --group site --python "3.14+gil" zensical build --strict
uv run --locked --no-default-groups --group site --python "3.14+gil" \
  python scripts/check-tutorial-structure.py --self-test
uv run --locked --no-default-groups --group site --python "3.14+gil" \
  python scripts/check-tutorial-structure.py --final --site-dir outputs/site --write-build-info

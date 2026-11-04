"""保守重用完整 Rust／Miri 成功結果；輸入不明、快取缺損就重新實跑。"""

import argparse
from datetime import UTC, datetime
import hashlib
import json
import os
from pathlib import Path
import platform
import shlex
import shutil
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[1]
COMMANDS = {"rust": ["bash", "scripts/check-rust.sh"], "miri": ["bash", "scripts/check-miri.sh"]}
INPUT_DIRECTORIES = ("examples", "scripts", "tests", ".cargo", ".github")
INPUT_FILES = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "pyproject.toml", "uv.lock", ".python-version")
IGNORED = {"target", "__pycache__", ".git", ".venv", ".cache"}


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def probe(arguments, root):
    return subprocess.check_output(arguments, cwd=root, stderr=subprocess.STDOUT, text=True).strip()


def external_inputs(root):
    """Cargo also reads ancestor/global config and optional compiler wrappers."""
    paths = set()
    for parent in (root, *root.parents):
        paths.update(parent / ".cargo" / name for name in ("config", "config.toml"))
    cargo_home = os.environ.get("CARGO_HOME")
    if cargo_home:
        paths.update(Path(cargo_home) / name for name in ("config", "config.toml"))
    for name in ("CC", "RUSTC", "RUSTDOC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"):
        value = os.environ.get(name)
        if value:
            command = shlex.split(value)
            executable = shutil.which(command[0]) if command else None
            if executable:
                paths.add(Path(executable).resolve())
    return {str(path): digest(path.read_bytes()) for path in sorted(paths) if path.is_file()}


def runtime_identity(root, kind):
    # Values affect the digest only; credentials/environment values never enter the receipt.
    environment = {
        name: value for name, value in os.environ.items()
        if name.startswith(("CARGO_", "RUST", "MIRI", "UV_"))
        or name in {"CC", "CXX", "CFLAGS", "CPPFLAGS", "LDFLAGS", "CPATH", "LIBRARY_PATH", "LD_LIBRARY_PATH", "SDKROOT", "MACOSX_DEPLOYMENT_TARGET", "PATH", "LANG", "LC_ALL"}
    }
    identity = {
        "root": str(root.resolve()),
        "platform": platform.platform(),
        "libc": platform.libc_ver(),
        "python": sys.version,
        "rustc": probe(["rustc", "-vV"], root),
        "cargo": probe(["cargo", "-vV"], root),
        "environment": environment,
        "external_inputs": external_inputs(root),
    }
    if kind == "rust":
        identity["cc"] = probe([*shlex.split(os.environ.get("CC", "cc")), "--version"], root)
    else:
        identity["miri"] = probe(["cargo", "+nightly-2026-10-01", "miri", "--version"], root)
        identity["nightly"] = probe(["rustc", "+nightly-2026-10-01", "-vV"], root)
    cpu = Path("/proc/cpuinfo")
    if cpu.is_file():
        identity["cpu_features"] = sorted({line for line in cpu.read_text().splitlines() if line.startswith(("flags", "Features", "model name"))})
    return identity


def execution_key(root=ROOT, kind="rust", identity=None):
    if kind not in COMMANDS:
        raise ValueError(f"未知檢查種類：{kind}")
    paths = {root / name for name in INPUT_FILES}
    for directory in INPUT_DIRECTORIES:
        paths.update(path for path in (root / directory).rglob("*") if not IGNORED.intersection(path.relative_to(root).parts))
    # Non-Markdown documentation may contain executable fixtures or their data.
    paths.update(path for path in (root / "docs").rglob("*") if path.suffix != ".md")
    hasher = hashlib.sha256()
    for path in sorted(paths):
        if path.is_symlink():
            raise ValueError(f"執行輸入不可為 symlink：{path}")
        if path.is_file():
            name = path.relative_to(root).as_posix()
            hasher.update(name.encode() + b"\0" + hashlib.sha256(path.read_bytes()).digest())
    runtime = identity if identity is not None else runtime_identity(root, kind)
    hasher.update(json.dumps({"schema_version": 1, "kind": kind, "command": COMMANDS[kind], "runtime": runtime}, sort_keys=True).encode())
    return hasher.hexdigest()


def valid_receipt(directory, key, kind):
    try:
        receipt = json.loads((directory / "success.json").read_bytes())
        stamp = datetime.fromisoformat(receipt["executed_at"])
        log = (directory / "check.log").read_bytes()
        if (
            receipt.get("schema_version") != 1 or receipt.get("key") != key
            or receipt.get("kind") != kind or receipt.get("command") != COMMANDS[kind]
            or receipt.get("status") != "passed" or stamp.tzinfo is None
            or receipt.get("log") != {"bytes": len(log), "sha256": digest(log)}
        ):
            return None
        return receipt
    except (OSError, ValueError, KeyError, TypeError):
        return None


def source_identity(root):
    try:
        revision = probe(["git", "rev-parse", "HEAD"], root)
        modified = bool(probe(["git", "status", "--porcelain", "--untracked-files=normal"], root))
        return {"revision": revision, "modified": modified}
    except (OSError, subprocess.CalledProcessError):
        return {"revision": None, "modified": None}


def ensure(root=ROOT, kind="rust", force=False, identity=None, runner=subprocess.run):
    key = execution_key(root, kind, identity)
    directory = root / "outputs/checks" / kind
    receipt = None if force else valid_receipt(directory, key, kind)
    if receipt:
        print(f"重用相同輸入的完整 {kind} 成功結果：{receipt['executed_at']}", flush=True)
        return receipt
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "success.json").unlink(missing_ok=True)
    log_path = directory / "check.log"
    print(f"正在實跑完整 {kind} 檢查；輸出：{log_path}", flush=True)
    with log_path.open("wb") as stream:
        result = runner(COMMANDS[kind], cwd=root, stdout=stream, stderr=subprocess.STDOUT, check=False)
    if result.returncode:
        print(log_path.read_text(errors="replace")[-12000:], file=sys.stderr)
        raise subprocess.CalledProcessError(result.returncode, COMMANDS[kind])
    if execution_key(root, kind, identity) != key:
        raise ValueError("檢查期間執行輸入已變動，沒有保存成功快取；請重新執行")
    log = log_path.read_bytes()
    receipt = {
        "schema_version": 1, "kind": kind, "key": key, "command": COMMANDS[kind],
        "status": "passed", "executed_at": datetime.now(UTC).isoformat(),
        "source": source_identity(root), "log": {"bytes": len(log), "sha256": digest(log)},
    }
    with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", dir=directory, delete=False) as stream:
        temporary = Path(stream.name)
        json.dump(receipt, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
    try:
        temporary.replace(directory / "success.json")
    finally:
        temporary.unlink(missing_ok=True)
    print(f"完整 {kind} 檢查通過，已保存結果。", flush=True)
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["key", "ensure"])
    parser.add_argument("--kind", choices=COMMANDS, default="rust")
    parser.add_argument("--force", action="store_true", help="忽略成功快取，重新實跑全部檢查")
    parser.add_argument("--github-output", type=Path)
    args = parser.parse_args()
    if args.command == "ensure":
        ensure(kind=args.kind, force=args.force)
    else:
        key = execution_key(kind=args.kind)
        print(key)
        if args.github_output:
            with args.github_output.open("a", encoding="utf-8") as stream:
                stream.write(f"key={key}\n")


if __name__ == "__main__":
    main()

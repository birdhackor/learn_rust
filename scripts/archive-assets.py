"""明確下載固定歷史封存，或離線核對；教材建置與測試不會自動下載。"""

import argparse
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import tarfile
import tempfile
import urllib.request


ROOT = Path(__file__).resolve().parents[1]
CATALOG = "assets/archives/catalog.json"
REPOSITORY = "birdhackor/learn_rust"


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def relative_path(name):
    path = PurePosixPath(name)
    if not name or "\\" in name or "\0" in name or path.is_absolute() or ".." in path.parts or path.as_posix() != name:
        raise ValueError(f"封存路徑不合法：{name}")
    return path


def safe_path(root, name):
    relative_path(name)
    target = root / name
    if target.is_symlink() or not target.resolve().is_relative_to(root.resolve()):
        raise ValueError(f"封存路徑離開指定目錄：{name}")
    return target


def checked(raw, spec, name):
    if len(raw) != spec["bytes"] or sha(raw) != spec["sha256"]:
        raise ValueError(f"封存大小／SHA-256 不符：{name}")
    return raw


def load_catalog(root=ROOT):
    data = json.loads((root / CATALOG).read_bytes())
    if data.get("schema_version") != 1 or data.get("repository") != REPOSITORY or not data.get("groups"):
        raise ValueError("封存清單版本／來源不符")
    for group, spec in data["groups"].items():
        revision = spec.get("source_revision", "")
        if (
            not re.fullmatch(r"[a-z0-9-]+", group) or not re.fullmatch(r"[0-9a-f]{40}", revision)
            or spec.get("url") != f"https://codeload.github.com/{REPOSITORY}/tar.gz/{revision}"
            or spec.get("prefix") != f"learn_rust-{revision}/" or spec.get("format") != "tar.gz"
            or not isinstance(spec.get("files"), dict) or not spec["files"]
        ):
            raise ValueError(f"封存必須使用完整固定提交與指定公開來源：{group}")
        for name, item in [(group, spec), *spec["files"].items()]:
            if type(item.get("bytes")) is not int or item["bytes"] < 0 or not re.fullmatch(r"[0-9a-f]{64}", str(item.get("sha256", ""))):
                raise ValueError(f"封存大小／SHA 格式不符：{name}")
        for name in spec["files"]:
            relative_path(name)
            if not name.startswith("planning/"):
                raise ValueError(f"不是允許的歷史審閱輸入：{name}")
    return data


def destination(root, group):
    # A symlinked output directory must not redirect extraction outside the repository.
    return safe_path(root, f"outputs/archives/{group}")


def verify(root=ROOT, group="review-history", spec=None):
    spec = spec or load_catalog(root)["groups"][group]
    output = destination(root, group)
    for name, item in spec["files"].items():
        path = safe_path(output, name)
        if not path.is_file():
            raise ValueError(f"缺少 {name}；先執行 python scripts/archive-assets.py fetch --group {group}")
        checked(path.read_bytes(), item, name)


def unpack(raw, spec, output):
    checked(raw, spec, "壓縮包")
    verified = {}
    with tarfile.open(fileobj=io.BytesIO(raw), mode="r:gz") as archive:
        seen = set()
        for entry in archive:
            # GitHub's source archive also holds the unchanged course and examples.
            # Inspect every path/type, but materialize only the catalog's exact planning tree.
            if entry.name == spec["prefix"].rstrip("/") and entry.isdir():
                continue
            if not entry.name.startswith(spec["prefix"]):
                raise ValueError("封存項目不在固定提交的根目錄")
            name = entry.name.removeprefix(spec["prefix"])
            relative_path(name)
            if entry.issym() or entry.islnk() or not (entry.isfile() or entry.isdir()):
                raise ValueError(f"封存不接受 symlink 或特殊檔案：{name}")
            if entry.isdir():
                continue
            if name in seen:
                raise ValueError(f"封存含重複檔案：{name}")
            seen.add(name)
            if not name.startswith("planning/"):
                continue
            if name not in spec["files"] or entry.size != spec["files"][name]["bytes"]:
                raise ValueError(f"封存檔案集合／大小不符：{name}")
            target = safe_path(output, name)
            verified[name] = checked(archive.extractfile(entry).read(), spec["files"][name], name)
            if target.exists():
                checked(target.read_bytes(), spec["files"][name], name)
    if set(verified) != set(spec["files"]):
        raise ValueError("封存缺少清單指定檔案")
    # All files and existing local changes have been checked before any file is written.
    for name, content in verified.items():
        target = safe_path(output, name)
        if target.exists():
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=target.parent, delete=False) as stream:
            temporary = Path(stream.name)
            stream.write(content)
        try:
            # Atomic creation without overwriting a file created after the preflight.
            os.link(temporary, target)
        finally:
            temporary.unlink(missing_ok=True)


def fetch(root=ROOT, group="review-history", opener=urllib.request.urlopen):
    catalog = load_catalog(root)
    if group not in catalog["groups"]:
        raise ValueError(f"未知封存組：{group}")
    spec = catalog["groups"][group]
    output = destination(root, group)
    missing = False
    for name, item in spec["files"].items():
        path = safe_path(output, name)
        if path.exists():
            checked(path.read_bytes(), item, name)
        else:
            missing = True
    if not missing:
        print(f"已核對本機封存：{group}")
        return
    cache = safe_path(root, f"outputs/archive-cache/{group}.tar.gz")
    raw = cache.read_bytes() if cache.is_file() else b""
    if len(raw) != spec["bytes"] or sha(raw) != spec["sha256"]:
        with opener(spec["url"], timeout=60) as response:
            raw = response.read(spec["bytes"] + 1)
        checked(raw, spec, "下載壓縮包")
        cache.parent.mkdir(parents=True, exist_ok=True)
        cache.write_bytes(raw)
    unpack(raw, spec, output)
    verify(root, group, spec)
    print(f"已取得並逐檔核對 {len(spec['files'])} 個封存檔：{output}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["fetch", "verify"])
    parser.add_argument("--group", default="review-history")
    args = parser.parse_args()
    if args.command == "fetch":
        fetch(group=args.group)
    else:
        verify(group=args.group)
        print("固定封存離線核對通過")


if __name__ == "__main__":
    main()

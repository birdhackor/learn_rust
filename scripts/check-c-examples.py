"""驗證第12章實際C11程式及指定的宣告失敗，僅執行正常案例。"""
from pathlib import Path
import os
import shlex
import subprocess
import tempfile

root = Path(__file__).resolve().parent.parent
package = root / "examples/ch12-c-basics"
compiler = shlex.split(os.environ.get("CC", "cc"))
flags = ["-std=c11", "-Wall", "-Wextra", "-Werror", "-pedantic"]
expected_output = {
    "main": (
        "amount=100\nlocal total=240; empty total=0\n"
        "text=Rust; text bytes=4; storage bytes=5\n"
        "buffer length=3; nonzero=2\nowned total=240\n"
    ),
    "checks": "",
    "solution": "original=240; updated=260\n",
}
with tempfile.TemporaryDirectory(prefix="learn-rust-c-") as temporary:
    for name, expected in expected_output.items():
        executable = Path(temporary) / name
        build = subprocess.run(
            [*compiler, *flags, str(package / f"{name}.c"),
             str(package / "counts.c"), "-o", str(executable)],
            capture_output=True, text=True, cwd=root,
        )
        if build.returncode:
            raise SystemExit(f"C正常案例未能建置：{name}\n{build.stderr}")
        result = subprocess.run([str(executable)], capture_output=True, text=True)
        if result.returncode or result.stdout != expected or result.stderr:
            raise SystemExit(f"C正常案例結果不符：{name}\n{result}")
    sources = sorted((package / "fixtures/compile-fail").glob("*.c"))
    if not sources:
        raise SystemExit("缺少C宣告負例")
    for index, source in enumerate(sources):
        expectation = source.with_suffix(".expected")
        if not expectation.is_file() or not expectation.read_text().strip():
            raise SystemExit(f"缺少C失敗契約：{source.name}")
        result = subprocess.run(
            [*compiler, *flags, "-c", str(source), "-o",
             str(Path(temporary) / f"negative-{index}.o")],
            capture_output=True, text=True, cwd=root,
            env={**os.environ, "LC_ALL": "C"},
        )
        if not result.returncode or expectation.read_text().strip() not in result.stderr:
            raise SystemExit(f"C失敗原因不符：{source.name}\n{result.stderr}")
print(f"已驗證3個C11正常程式及{len(sources)}個指定編譯失敗")

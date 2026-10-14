"""Check the intentional compiler/runtime failures shown in the tutorial."""
from pathlib import Path
import os
import subprocess
import tempfile

project_root = Path(__file__).resolve().parent.parent
fixtures = sorted(project_root.glob("examples/**/fixtures/compile-fail/*.rs"))
fixtures += sorted(project_root.glob("examples/**/fixtures/runtime-fail/*.rs"))
count = 0
with tempfile.TemporaryDirectory(prefix="rust-tutorial-fixtures-") as temporary:
    for index, source in enumerate(fixtures):
        expectation = source.with_suffix(".expected")
        if not expectation.is_file():
            raise SystemExit(f"缺少失敗契約：{expectation.relative_to(project_root)}")
        expected = expectation.read_text().strip()
        if not expected:
            raise SystemExit(f"空白失敗契約：{expectation.relative_to(project_root)}")
        executable = Path(temporary) / f"fixture_{index}"
        compile_result = subprocess.run(
            ["rustc", "--edition=2024", "--crate-name", f"fixture_{index}",
             str(source), "-o", str(executable)],
            capture_output=True, text=True, cwd=project_root,
        )
        if source.parent.name == "compile-fail":
            result = compile_result
        else:
            if compile_result.returncode:
                raise SystemExit(f"執行失敗範例未能先通過編譯：{source}\n{compile_result.stderr}")
            result = subprocess.run([str(executable)], capture_output=True, text=True,
                                    env={**os.environ, "RUST_BACKTRACE": "0"})
        if not result.returncode or expected not in result.stderr:
            raise SystemExit(f"失敗契約不符：{source.relative_to(project_root)}\n"
                             f"預期非零結束並包含 {expected!r}\n{result.stderr}")
        count += 1
print(f"已驗證 {count} 個故意失敗的教學範例")

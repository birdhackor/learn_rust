"""發布流程的失效與離線契約；不把網路或真正的 Rust 編譯藏在單元測試。"""

import copy
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]


def script(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), ROOT / "scripts" / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


execution = script("check-execution")
assets = script("archive-assets")
structure = script("check-tutorial-structure")


def write(root, name, raw):
    path = root / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(raw)


class ExecutionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        write(self.root, "examples/lesson/src/lib.rs", b"pub fn answer() -> u8 { 42 }\n")
        write(self.root, "Cargo.toml", b"[workspace]\n")
        write(self.root, "Cargo.lock", b"version = 4\n")
        write(self.root, "scripts/check-rust.sh", b"cargo test\n")
        self.identity = {"toolchain": "fixed", "platform": "test"}
        self.calls = 0

    def runner(self, command, *, cwd, stdout, stderr, check):
        self.assertEqual(command, ["bash", "scripts/check-rust.sh"])
        self.calls += 1
        stdout.write(b"all required checks passed\n")
        return SimpleNamespace(returncode=0)

    def ensure(self, **kwargs):
        return execution.ensure(self.root, identity=self.identity, runner=self.runner, **kwargs)

    def key(self):
        return execution.execution_key(self.root, identity=self.identity)

    def test_prose_and_optional_archives_reuse_the_complete_result(self):
        receipt = self.ensure()
        write(self.root, "docs/lesson.md", b"updated explanation\n")
        write(self.root, "outputs/archives/review-history/planning/old.md", b"old review\n")
        self.assertEqual(receipt, self.ensure())
        self.assertEqual(self.calls, 1)

    def test_each_executable_input_change_invalidates_the_result(self):
        for name in ["examples/lesson/src/lib.rs", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "uv.lock", "scripts/check-rust.sh", "examples/lesson/fixtures/compile-fail/bad.expected", "docs/input.json", ".cargo/config.toml"]:
            with self.subTest(name=name):
                previous = self.key()
                write(self.root, name, b"changed input\n")
                self.assertNotEqual(previous, self.key())

    def test_environment_change_and_force_each_rerun(self):
        self.ensure()
        self.identity["toolchain"] = "new"
        self.ensure()
        self.ensure(force=True)
        self.assertEqual(self.calls, 3)

    def test_global_cargo_config_and_compiler_wrapper_changes_are_bound(self):
        cargo_home = self.root / "tool-config"
        wrapper = self.root / "compiler-wrapper"
        wrapper.write_text("#!/bin/sh\nexec cc \"$@\"\n")
        wrapper.chmod(0o755)
        with patch.dict("os.environ", {"CARGO_HOME": str(cargo_home), "RUSTC_WRAPPER": str(wrapper)}):
            before = execution.external_inputs(self.root)
            write(cargo_home, "config.toml", b'[build]\nrustflags = ["-C", "opt-level=1"]\n')
            configured = execution.external_inputs(self.root)
            self.assertNotEqual(before, configured)
            wrapper.write_text("#!/bin/sh\nexit 1\n")
            self.assertNotEqual(configured, execution.external_inputs(self.root))

    def test_missing_or_corrupt_receipt_and_log_rerun(self):
        self.ensure()
        directory = self.root / "outputs/checks/rust"
        (directory / "success.json").write_text("{}")
        self.ensure()
        (directory / "check.log").write_text("changed")
        self.ensure()
        (directory / "check.log").unlink()
        self.ensure()
        self.assertEqual(self.calls, 4)

    def test_failure_never_leaves_a_success_receipt(self):
        self.ensure()
        def failure(*args, **kwargs):
            kwargs["stdout"].write(b"specific failing check\n")
            return SimpleNamespace(returncode=19)
        with self.assertRaises(subprocess.CalledProcessError):
            execution.ensure(self.root, force=True, identity=self.identity, runner=failure)
        self.assertFalse((self.root / "outputs/checks/rust/success.json").exists())

    def test_input_changed_during_execution_cannot_be_cached(self):
        def changing(*args, **kwargs):
            write(self.root, "examples/lesson/src/lib.rs", b"concurrent edit\n")
            return self.runner(*args, **kwargs)
        with self.assertRaisesRegex(ValueError, "期間"):
            execution.ensure(self.root, identity=self.identity, runner=changing)
        self.assertFalse((self.root / "outputs/checks/rust/success.json").exists())


class ArchiveTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.name = "planning/reviews/report.md"
        self.raw_file = b"original independent observations\n"
        self.spec = {
            "source_revision": "a" * 40,
            "url": f"https://codeload.github.com/{assets.REPOSITORY}/tar.gz/" + "a" * 40,
            "format": "tar.gz", "prefix": "learn_rust-" + "a" * 40 + "/",
            "files": {self.name: {"bytes": len(self.raw_file), "sha256": assets.sha(self.raw_file)}},
        }
        self.raw = self.archive([(self.name, self.raw_file, None)])
        self.spec.update(bytes=len(self.raw), sha256=assets.sha(self.raw))
        self.catalog = {"schema_version": 1, "repository": assets.REPOSITORY, "groups": {"review-history": self.spec}}
        self.save_catalog()

    def save_catalog(self):
        write(self.root, assets.CATALOG, json.dumps(self.catalog).encode())

    def archive(self, rows):
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode="w:gz") as archive:
            for name, raw, kind in rows:
                entry = tarfile.TarInfo(self.spec["prefix"] + name)
                entry.size = len(raw)
                if kind:
                    entry.type = kind
                    entry.linkname = "../../outside"
                    entry.size = 0
                archive.addfile(entry, io.BytesIO(raw))
        return output.getvalue()

    def test_download_then_offline_verify_and_reuse(self):
        calls = []
        def opener(url, timeout):
            calls.append(url)
            return io.BytesIO(self.raw)
        assets.fetch(self.root, opener=opener)
        assets.verify(self.root)
        self.assertEqual(len(calls), 1)
        path = self.root / "outputs/archives/review-history" / self.name
        self.assertEqual(path.read_bytes(), self.raw_file)
        assets.fetch(self.root, opener=lambda *a, **k: self.fail("verified files must not use network"))
        path.unlink()
        assets.fetch(self.root, opener=lambda *a, **k: self.fail("valid archive cache must work offline"))
        assets.verify(self.root)

    def test_local_changes_are_never_overwritten_or_downloaded(self):
        path = self.root / "outputs/archives/review-history" / self.name
        write(self.root, str(path.relative_to(self.root)), b"my notes")
        with self.assertRaisesRegex(ValueError, "SHA-256"):
            assets.fetch(self.root, opener=lambda *a, **k: self.fail("local conflict must fail before network"))
        self.assertEqual(path.read_bytes(), b"my notes")

    def test_wrong_archive_hash_is_rejected_before_any_extraction(self):
        with self.assertRaisesRegex(ValueError, "SHA-256"):
            assets.fetch(self.root, opener=lambda *a, **k: io.BytesIO(b"wrong archive"))
        self.assertFalse((self.root / "outputs/archives").exists())

    def test_bad_members_are_rejected_before_any_file_is_written(self):
        for damage in ["missing", "duplicate", "extra", "traversal", "symlink", "hardlink", "bytes"]:
            with self.subTest(damage=damage):
                rows = [(self.name, self.raw_file, None)]
                if damage == "missing":
                    rows = []
                elif damage == "duplicate":
                    rows *= 2
                elif damage in {"extra", "traversal"}:
                    rows.append(("planning/extra" if damage == "extra" else "../escape", b"x", None))
                elif damage in {"symlink", "hardlink"}:
                    rows = [(self.name, b"", tarfile.SYMTYPE if damage == "symlink" else tarfile.LNKTYPE)]
                else:
                    rows = [(self.name, b"changed", None)]
                raw = self.archive(rows)
                spec = copy.deepcopy(self.spec)
                spec.update(bytes=len(raw), sha256=assets.sha(raw))
                output = self.root / "output"
                with self.assertRaises(ValueError):
                    assets.unpack(raw, spec, output)
                self.assertFalse(output.exists())

    def test_missing_inputs_verify_without_attempting_network(self):
        with self.assertRaisesRegex(ValueError, "fetch"):
            assets.verify(self.root)

    def test_catalog_rejects_moving_refs_and_nonarchive_files(self):
        self.spec["source_revision"] = "main"
        self.save_catalog()
        with self.assertRaises(ValueError):
            assets.load_catalog(self.root)
        self.spec["source_revision"] = "a" * 40
        self.spec["files"] = {"Cargo.toml": {"bytes": 1, "sha256": "a" * 64}}
        self.save_catalog()
        with self.assertRaisesRegex(ValueError, "歷史"):
            assets.load_catalog(self.root)

    def test_symlinked_output_cannot_escape_the_repository(self):
        with tempfile.TemporaryDirectory() as outside:
            (self.root / "outputs").symlink_to(outside, target_is_directory=True)
            with self.assertRaises(ValueError):
                assets.fetch(self.root, opener=lambda *a, **k: self.fail("unsafe output must fail first"))


class StructureTests(unittest.TestCase):
    def test_required_current_source_is_checked_without_archived_reviews(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            write(root, "lesson.md", b"# 1.1 Lesson\n")
            report = structure.Report(final=True)
            checker = structure.Checker(root, report)
            checker.positions = {"1.1": 0}
            row = {"id": "1.1", "path": "lesson.md", "first_available_at": "1.1", "loaded_teaching_sources": ["missing.rs"]}
            checker.check_sources(row)
            self.assertTrue(any("missing.rs" in message for message in report.pending))
            write(root, "missing.rs", b"fn main() {}\n")
            report.pending.clear()
            checker.check_sources(row)
            self.assertEqual(report.errors + report.pending, [])


if __name__ == "__main__":
    unittest.main()

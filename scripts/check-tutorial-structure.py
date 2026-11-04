#!/usr/bin/env python3
"""Validate the current curriculum, executable source references, and built HTML.

Run with the repository's .venv/bin/python to use its locked Markdown/snippets
engine. This is structural evidence, never a pedagogical or compilation review.
"""

from __future__ import annotations

import argparse
from collections import Counter
from html.parser import HTMLParser
import json
from datetime import UTC, datetime
from importlib.metadata import version
from pathlib import Path
import re
import sys
import subprocess
import tempfile
import tomllib
from urllib.parse import unquote, urlsplit


LEGACY_PAGES = 109
LEGACY_ANCHORS = 890


def normalized(value):
    return " ".join(str(value).split())


class HTML(HTMLParser):
    def __init__(self, content):
        super().__init__(convert_charrefs=True)
        self.ids = set()
        self.duplicates = set()
        self.links = []
        self.h1s = []
        self._h1 = None
        self.feed(content)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        ident = attrs.get("id") or (attrs.get("name") if tag == "a" else None)
        if ident:
            if ident in self.ids:
                self.duplicates.add(ident)
            self.ids.add(ident)
        if tag == "h1":
            self._h1 = []
        for attr in ("href", "xlink:href", "src", "poster"):
            if attrs.get(attr):
                self.links.append((self.getpos()[0], attrs[attr]))
        # Data URLs may contain commas; they are external to this file check.
        if attrs.get("srcset") and "data:" not in attrs["srcset"]:
            self.links.extend(
                (self.getpos()[0], item.strip().split()[0])
                for item in attrs["srcset"].split(",") if item.strip()
            )

    def handle_endtag(self, tag):
        if tag == "h1" and self._h1 is not None:
            self.h1s.append(normalized("".join(self._h1)))
            self._h1 = None

    def handle_data(self, data):
        if self._h1 is not None:
            self._h1.append(data)


class Report:
    def __init__(self, final=False, limit=35):
        self.final = final
        self.limit = limit
        self.errors = []
        self.pending = []

    def error(self, message):
        self.errors.append(message)

    def incomplete(self, message):
        self.pending.append(message)

    def finish(self):
        for label, rows in (("ERROR", self.errors), ("PENDING", self.pending)):
            for row in rows[: self.limit]:
                print(f"{label}: {row}")
            if len(rows) > self.limit:
                print(f"{label}: … {len(rows) - self.limit} more; use --max-messages")
        mode = "final" if self.final else "draft"
        print(f"{mode}: {len(self.errors)} error(s), {len(self.pending)} pending check(s).")
        if not self.errors and not self.pending:
            print("Structural checks complete; teaching quality, execution, build freshness, and visual review remain separate.")
        # A partial draft is never a passing check. 2 distinguishes unfinished
        # migration work from a structurally invalid manifest or broken link.
        return 1 if self.errors or (self.final and self.pending) else 2 if self.pending else 0


def extension_config(tree, prefix=""):
    """Expand TOML dotted extension names without hard-coding their options."""
    result = {}
    for name, value in tree.items():
        full = f"{prefix}.{name}" if prefix else name
        if not isinstance(value, dict):
            raise ValueError(f"Markdown extension {full} must be a table")
        if not value or any(not isinstance(v, dict) for v in value.values()):
            result[full] = value
        else:
            result.update(extension_config(value, full))
    return result


class Checker:
    def __init__(self, root, report, site_dir=None):
        self.root = root.resolve()
        self.report = report
        self.curriculum = self.root / "curriculum"
        self.site_arg = site_dir
        self.rendered = {}
        self.built = {}
        self.snippet_sources = {}

    def load(self, path):
        try:
            return json.loads(path.read_text(encoding="utf-8"))
        except (OSError, ValueError) as exc:
            self.report.error(f"{path}: {exc}")
            return None

    def file(self, value, context):
        if not isinstance(value, str) or not value or "#" in value:
            self.report.error(f"{context}: expected a repository-relative file path, got {value!r}")
            return None
        path = (self.root / value).resolve()
        if Path(value).is_absolute() or not path.is_relative_to(self.root):
            self.report.error(f"{context}: path must stay inside the repository: {value}")
            return None
        if not path.is_file():
            self.report.incomplete(f"{context}: missing file {value}")
            return None
        return path

    def unique(self, rows, field, context):
        values = [row.get(field) for row in rows]
        wanted_type = int if field == "number" else str
        if any(type(x) is not wanted_type or x == "" for x in values):
            self.report.error(f"{context}: every {field} must be a nonempty {wanted_type.__name__}")
            return
        duplicates = [x for x, count in Counter(values).items() if count > 1]
        if duplicates:
            self.report.error(f"{context}: invalid/duplicate {field}: {duplicates}")

    def run(self):
        manifest = self.load(self.curriculum / "sections.json")
        mappings = self.load(self.curriculum / "migration-map.json")
        anchors = self.load(self.curriculum / "legacy-anchors.json")
        try:
            config = tomllib.loads((self.root / "zensical.toml").read_text(encoding="utf-8"))["project"]
        except (OSError, ValueError, KeyError) as exc:
            self.report.error(f"zensical.toml: {exc}")
            return
        if not isinstance(manifest, dict) or not all(isinstance(x, list) for x in (mappings, anchors)):
            self.report.error("Expected sections.json object and migration/anchors arrays")
            return
        sections, chapters = manifest.get("sections"), manifest.get("chapters")
        if not all(isinstance(x, list) for x in (sections, chapters)) or not all(
            isinstance(row, dict) for rows in (sections, chapters, mappings, anchors) for row in rows
        ):
            self.report.error("Manifest entries must be objects in chapters/sections/migration/anchors arrays")
            return
        if manifest.get("schema_version") != 1 or not chapters or not sections:
            self.report.error("Expected schema_version=1 and nonempty chapters/sections")
            return
        if len(mappings) != LEGACY_PAGES:
            self.report.error(f"Expected {LEGACY_PAGES} legacy mappings, found {len(mappings)}")
        for rows, fields, label in ((sections, ("id", "path"), "sections"),
                                    (chapters, ("number",), "chapters"),
                                    (mappings, ("old_id", "old_path"), "migration-map"),
                                    (anchors, ("old_path", "url"), "legacy-anchors")):
            for field in fields:
                self.unique(rows, field, label)
        if self.report.errors:
            return
        self.config = config
        self.sections = {row["id"]: row for row in sections}
        self.positions = {row["id"]: i for i, row in enumerate(sections)}
        self.old = {row["old_id"]: row for row in mappings}
        extra = {str(p.relative_to(self.root)) for p in (self.root / "docs/chapters").glob("*-lesson.md")} - {r["path"] for r in sections}
        if extra:
            self.report.error(f"Canonical lesson files absent from sections.json: {sorted(extra)}")
        self.check_manifest(chapters, sections, mappings, anchors)
        self.check_navigation(chapters)
        for row in sections:
            path = self.file(row["path"], row["id"])
            if path:
                page = self.render(path)
                expected = normalized(f"{row['id']} {row['title']}")
                if page and page.h1s != [expected]:
                    self.report.error(f"{row['path']}: expected one H1 {expected!r}; found {page.h1s!r}")
                if page:
                    self.check_links(path, page, built=False)
            self.check_sources(row)
        if self.report.final or self.site_arg is not None:
            self.check_site(sections, anchors)
        else:
            self.report.incomplete("Built HTML not checked; provide --site-dir after a fresh strict build")

    def check_manifest(self, chapters, sections, mappings, anchors):
        expected_ids = []
        if [c["number"] for c in chapters] != list(range(1, len(chapters) + 1)):
            self.report.error("chapters: expected consecutive numeric order starting at 1")
        for chapter in chapters:
            ids = chapter.get("section_ids", [])
            if not isinstance(ids, list):
                self.report.error(f"chapter {chapter['number']}: section_ids must be a list")
                continue
            expected_ids.extend(ids)
            for index, ident in enumerate(ids, 1):
                if ident != f"{chapter['number']}.{index}":
                    self.report.error(f"chapter {chapter['number']}: expected section {chapter['number']}.{index}, got {ident}")
        if expected_ids != list(self.sections):
            self.report.error("sections order/coverage differs from chapters[].section_ids")
        mapping_paths = {row["old_path"] for row in mappings}
        if mapping_paths != {row["old_path"] for row in anchors}:
            self.report.error("migration-map old_path coverage differs from independently captured legacy-anchors")
        anchor_count = sum(len(row.get("headings", [])) for row in anchors)
        if len(anchors) != LEGACY_PAGES or anchor_count != LEGACY_ANCHORS:
            self.report.error(f"Expected 109 legacy pages / 890 captured heading anchors; found {len(anchors)} / {anchor_count}")
        for old in mappings:
            self.file(old["old_path"], f"legacy {old['old_id']}")
            old_match = re.match(r"docs/chapters/(\d+)-(\d+)-[^/]+\.md$", old["old_path"])
            if not old_match or old["old_id"] != ".".join(str(int(x)) for x in old_match.groups()):
                self.report.error(f"{old['old_id']}: legacy id does not match old_path {old['old_path']}")
            targets = old.get("new_ids", [])
            if not isinstance(targets, list) or not targets or any(not isinstance(x, str) for x in targets) or len(targets) != len(set(targets)):
                self.report.error(f"{old['old_id']}: new_ids must contain unique canonical targets")
                continue
            for ident in targets:
                if ident not in self.sections:
                    self.report.error(f"{old['old_id']}: unknown canonical target {ident}")
                elif old["old_id"] not in self.sections[ident].get("old_ids", []):
                    self.report.error(f"{old['old_id']} → {ident}: target does not list this old_id")
        for row in sections:
            ident = row["id"]
            match = re.fullmatch(r"([1-9][0-9]*)\.([1-9][0-9]*)", ident)
            if not match:
                self.report.error(f"Invalid canonical id: {ident}")
                continue
            chapter, order = map(int, match.groups())
            if (row.get("chapter"), row.get("order"), row["path"]) != (
                chapter, order, f"docs/chapters/{chapter:02d}-{order:02d}-lesson.md"
            ):
                self.report.error(f"{ident}: chapter/order/path do not match canonical id")
            if not row.get("title"):
                self.report.error(f"{ident}: missing title")
            for old_id in row.get("old_ids", []):
                if old_id not in self.old or ident not in self.old[old_id].get("new_ids", []):
                    self.report.error(f"{ident}: old_id {old_id} has no reverse migration mapping")
            for prereq in row.get("prerequisites", []):
                if prereq not in self.positions or self.positions[prereq] >= self.positions[ident]:
                    self.report.error(f"{ident}: prerequisite {prereq} is unknown or not earlier")
            for prereq in row.get("prerequisite_chapters", []):
                if not isinstance(prereq, int) or not 1 <= prereq < chapter:
                    self.report.error(f"{ident}: prerequisite chapter {prereq} is not earlier")

    def check_navigation(self, chapters):
        nav = self.config.get("nav", [])
        actual = []
        for entry in nav:
            if not isinstance(entry, dict):
                self.report.error("nav entries must be tables")
                continue
            for label, value in entry.items():
                if isinstance(value, list):
                    paths = []
                    for item in value:
                        if not isinstance(item, dict) or len(item) != 1 or not isinstance(next(iter(item.values())), str):
                            self.report.error(f"nav {label}: expected flat section label/path entries")
                            continue
                        label_text, target = next(iter(item.items()))
                        paths.append((normalized(label_text), target))
                    actual.append((label, paths))
        wanted = [
            (f"第 {c['number']} 章：{c['title']}",
             [(normalized(f"{ident} {self.sections[ident]['title']}"), self.sections[ident]["path"].removeprefix("docs/"))
              for ident in c["section_ids"] if ident in self.sections])
            for c in chapters
        ]
        if actual != wanted:
            self.report.incomplete("zensical.toml nav: chapter labels/order/section paths must match all 18 manifest chapters")

    def render(self, path):
        if path in self.rendered:
            return self.rendered[path]
        self.rendered[path] = None
        try:
            import markdown
            configs = extension_config(self.config.get("markdown_extensions", {}))
            snippet = dict(configs.get("pymdownx.snippets", {}))
            snippet.update(base_path=[str(self.root)], check_paths=True, url_download=False)
            configs["pymdownx.snippets"] = snippet
            # Keep the heading permalink outside the H1 text used for identity.
            configs["toc"] = {**configs.get("toc", {}), "permalink": False}
            md = markdown.Markdown(extensions=list(configs), extension_configs=configs)
            processor = md.preprocessors["snippet"]
            sources = set()
            original = processor.get_snippet_path

            def tracked(value):
                result = original(value)
                if result:
                    sources.add(Path(result).resolve())
                return result

            processor.get_snippet_path = tracked
            # The real snippets engine validates files/regions, recursively
            # expands includes, and implements its exact line-slice semantics.
            result = HTML(md.convert(path.read_text(encoding="utf-8")))
            self.snippet_sources[path] = sources
            self.rendered[path] = result
            return result
        except ImportError as exc:
            self.report.incomplete(f"{path.relative_to(self.root)}: Markdown/snippets unavailable ({exc}); run .venv/bin/python")
        except Exception as exc:
            self.report.error(f"{path.relative_to(self.root)}: Markdown/snippet expansion failed: {exc}")
        return None

    def check_sources(self, row):
        """Check the current source contract without replaying historical reviews."""
        ident = row["id"]
        available = row.get("first_available_at")
        if not isinstance(available, str) or available not in self.positions or self.positions[available] > self.positions[ident]:
            self.report.error(f"{ident}: first_available_at must be an existing section no later than itself")
        loaded = row.get("loaded_teaching_sources")
        if not isinstance(loaded, list) or any(not isinstance(p, str) for p in loaded):
            self.report.error(f"{ident}: loaded_teaching_sources must list actual repository file paths")
            return
        if len(loaded) != len(set(loaded)):
            self.report.error(f"{ident}: duplicate loaded_teaching_sources")
        for value in loaded:
            self.file(value, f"{ident} teaching source")
        if row.get("manifest"):
            self.file(row["manifest"], f"{ident} Cargo manifest")
            if not isinstance(row.get("command"), str) or not row["command"].strip():
                self.report.error(f"{ident}: manifest requires a concrete command")
        for source in self.snippet_sources.get((self.root / row["path"]).resolve(), set()):
            if source.is_relative_to(self.root) and str(source.relative_to(self.root)) not in loaded:
                self.report.error(f"{ident}: included source missing from loaded_teaching_sources: {source.relative_to(self.root)}")

    def built_page(self, path):
        if path not in self.built:
            try:
                self.built[path] = HTML(path.read_text(encoding="utf-8"))
            except (OSError, UnicodeError) as exc:
                self.report.error(f"{path}: cannot read built HTML/SVG: {exc}")
                self.built[path] = None
        return self.built[path]

    def local_url(self, value, built):
        split = urlsplit(value)
        if split.scheme or split.netloc:
            site = urlsplit(self.config.get("site_url", ""))
            if not built or split.scheme not in ("", "http", "https") or split.netloc != site.netloc:
                return None
            prefix = site.path.rstrip("/")
            if split.path != prefix and not split.path.startswith(prefix + "/"):
                return None
        return split

    def check_links(self, path, page, built):
        base = self.site if built else self.root / self.config.get("docs_dir", "docs")
        for line, value in page.links:
            split = self.local_url(value, built)
            if split is None:
                continue
            raw_path = unquote(split.path)
            if built and raw_path.startswith("/"):
                prefix = urlsplit(self.config.get("site_url", "")).path.rstrip("/")
                if prefix and raw_path != prefix and not raw_path.startswith(prefix + "/"):
                    self.report.error(f"{path.relative_to(base)}:{line}: URL escapes deployed site prefix: {value}")
                    continue
                target = base / raw_path.removeprefix(prefix).lstrip("/")
            elif raw_path.startswith("/"):
                target = base / raw_path.lstrip("/")
            else:
                target = path.parent / raw_path if raw_path else path
            target = target.resolve()
            if not target.is_relative_to(base.resolve()):
                self.report.error(f"{path.relative_to(base)}:{line}: link escapes {'site' if built else 'docs'}: {value}")
                continue
            if target.is_dir():
                target /= "index.html" if built else "index.md"
            elif built and not target.is_file() and not target.suffix:
                target = target / "index.html" if (target / "index.html").is_file() else target.with_suffix(".html")
            if not target.is_file():
                self.report.error(f"{path.relative_to(base)}:{line}: missing link/image target: {value}")
                continue
            fragment = unquote(split.fragment).split(":~:text=", 1)[0]
            if fragment:
                other = self.render(target) if not built and target.suffix == ".md" else (
                    self.built_page(target) if target.suffix in (".html", ".svg") else None
                )
                if other and fragment not in other.ids:
                    self.report.error(f"{path.relative_to(base)}:{line}: missing anchor #{fragment}: {value}")

    def check_site(self, sections, anchors):
        self.site = (self.site_arg or self.root / self.config.get("site_dir", "site")).resolve()
        if not self.site.is_dir():
            self.report.incomplete(f"Missing built site: {self.site}; run the strict site build first")
            return
        pages = sorted(self.site.rglob("*.html"))
        if not pages:
            self.report.incomplete(f"Built site contains no HTML: {self.site}")
        for path in pages:
            page = self.built_page(path)
            if page:
                if page.duplicates:
                    self.report.error(f"{path.relative_to(self.site)}: duplicate HTML ids {sorted(page.duplicates)}")
                self.check_links(path, page, built=True)
        for row in sections:
            relative = Path(row["path"]).relative_to(self.config.get("docs_dir", "docs"))
            options = [self.site / relative.with_suffix("") / "index.html", self.site / relative.with_suffix(".html")]
            target = next((p for p in options if p.is_file()), None)
            if target is None:
                self.report.incomplete(f"{row['id']}: canonical page absent from built site")
            else:
                page = self.built_page(target)
                expected = normalized(f"{row['id']} {row['title']}")
                # Built permalink glyphs are present as H1 text; compare their
                # removal only at the end, preserving the actual lesson title.
                if page and [h.rstrip(" ¶§#🔗") for h in page.h1s] != [expected]:
                    self.report.error(f"{target.relative_to(self.site)}: built H1 differs from canonical manifest")
        for row in anchors:
            target = self.site / row["url"] / "index.html"
            if not target.is_file():
                self.report.incomplete(f"Legacy URL missing from built site: {row['url']}")
                continue
            page = self.built_page(target)
            if page:
                missing = [h["id"] for h in row.get("headings", []) if h["id"] not in page.ids]
                if missing:
                    self.report.error(f"{row['old_path']}: legacy heading anchors missing in rendered page: {missing}")
                # Compatibility entrances must point at every mapped destination.
                old = next((m for m in self.old.values() if m["old_path"] == row["old_path"]), None)
                if old:
                    for ident in old["new_ids"]:
                        stem = Path(self.sections[ident]["path"]).stem
                        if not any(stem in unquote(urlsplit(url).path).split("/") or
                                   f"{stem}.html" in unquote(urlsplit(url).path).split("/")
                                   for _, url in page.links):
                            self.report.error(f"{row['old_path']}: compatibility entrance omits destination {ident}")


def self_test():
    """Regression harness for links, source declarations, and real snippets."""
    with tempfile.TemporaryDirectory(prefix="tutorial-structure-") as temp:
        root = Path(temp)
        (root / "site").mkdir()
        source = root / "site/index.html"
        target = root / "site/target.html"
        source.write_text('<a href="target.html#ok">ok</a><img src="picture.svg">', encoding="utf-8")
        target.write_text('<h1 id="ok">Target</h1>', encoding="utf-8")
        (root / "site/picture.svg").write_text('<svg id="picture"></svg>', encoding="utf-8")
        report = Report()
        check = Checker(root, report)
        check.config = {"site_url": "https://example.org/book/"}
        check.site = root / "site"
        check.check_links(source, HTML(source.read_text()), built=True)
        assert not report.errors, report.errors
        check.check_links(source, HTML('<a href="target.html#absent">bad</a><img src="lost.svg">'), built=True)
        assert len(report.errors) == 2 and any("missing anchor" in x for x in report.errors)
        check.check_links(source, HTML('<a href="/elsewhere/">outside prefix</a>'), built=True)
        assert "deployed site prefix" in report.errors[-1]
        assert HTML('<p id="same"></p><p id="same"></p>').duplicates == {"same"}
        lesson = root / "lesson.md"
        lesson.write_text("# 1.1 Lesson\n", encoding="utf-8")
        row = {"id": "1.1", "path": "lesson.md", "first_available_at": "1.1", "loaded_teaching_sources": []}
        clean = Report()
        ready = Checker(root, clean)
        ready.positions = {"1.1": 0}
        ready.check_sources(row)
        assert not clean.errors and not clean.pending, (clean.errors, clean.pending)
        # Prose edits are valid without old review files or reviewer identities.
        lesson.write_text("# 1.1 Changed\n", encoding="utf-8")
        ready.check_sources(row)
        assert not clean.errors and not clean.pending
        row["loaded_teaching_sources"] = ["missing.rs"]
        ready.check_sources(row)
        assert any("missing.rs" in message for message in clean.pending)
        row["loaded_teaching_sources"] = []
        try:
            import markdown  # noqa: F401 — optional locked engine for exact snippets
        except ImportError:
            raise RuntimeError("self-test requires the locked site dependency group") from None
        else:
            snippet = root / "code.rs"
            snippet.write_text('// --8<-- [start:ok]\nlet value = 1;\n// --8<-- [end:ok]\n', encoding="utf-8")
            ready.config = {"markdown_extensions": {"toc": {}, "pymdownx": {"snippets": {}}}}
            good = root / "good.md"
            good.write_text('# 1.1 Lesson\n\n--8<-- "code.rs:ok"\n', encoding="utf-8")
            assert ready.render(good) is not None
            assert snippet in ready.snippet_sources[good]
            bad = root / "bad.md"
            bad.write_text('--8<-- "code.rs:absent"\n', encoding="utf-8")
            assert ready.render(bad) is None
            assert "Snippet section 'absent'" in clean.errors[-1]
            lost = root / "lost.md"
            lost.write_text('--8<-- "missing.rs"\n', encoding="utf-8")
            assert ready.render(lost) is None
            assert "missing.rs" in clean.errors[-1]
    print("self-test: link/image/anchor, deployment prefix, duplicate id, current source declarations, and exact snippets passed")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--draft", action="store_true", help="report unfinished migration work (exit 2), never claim a partial pass")
    mode.add_argument("--final", action="store_true", help="require current source declarations, navigation, and built-site checks")
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--site-dir", type=Path, help="check this existing built site; final defaults to configured site_dir")
    parser.add_argument("--max-messages", type=int, default=35, help="maximum detailed messages per severity")
    parser.add_argument("--write-build-info", action="store_true", help="write build-info.json after successful final site validation")
    parser.add_argument("--self-test", action="store_true", help="run temporary-fixture regression checks only")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return 0
    report = Report(final=args.final, limit=max(1, args.max_messages))
    checker = Checker(args.root, report, args.site_dir)
    checker.run()
    result = report.finish()
    if args.write_build_info:
        if result or not args.final or not hasattr(checker, "site"):
            parser.error("--write-build-info requires successful --final site validation")
        try:
            revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=checker.root, stderr=subprocess.DEVNULL, text=True).strip()
            modified = bool(subprocess.check_output(["git", "status", "--porcelain", "--untracked-files=normal"], cwd=checker.root, text=True).strip())
        except (OSError, subprocess.CalledProcessError):
            revision, modified = None, None
        info = {
            "schema_version": 1,
            "source_revision": revision,
            "source_modified": modified,
            "validated_at": datetime.now(UTC).isoformat(),
            "builder": "zensical",
            "builder_version": version("zensical"),
            "sections": len(checker.sections),
            "legacy_pages": LEGACY_PAGES,
            "legacy_anchors": LEGACY_ANCHORS,
            "structural_validation": "passed",
        }
        (checker.site / "build-info.json").write_text(json.dumps(info, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return result


if __name__ == "__main__":
    sys.exit(main())

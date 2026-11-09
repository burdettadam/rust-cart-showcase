"""Exercise release metadata with real Git checkouts; WASM execution is separate."""

import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


VERIFIER = Path(__file__).resolve().parents[1] / "scripts/verify_dist.py"


class ReleaseSourceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for directory in ("web", "vendor", "scripts", "dist/pkg"):
            (self.root / directory).mkdir(parents=True)
        (self.root / ".gitignore").write_text("/dist/\n")
        (self.root / "web/index.html").write_text("<!doctype html><title>Fixture</title>")
        (self.root / "vendor/provenance.json").write_text(json.dumps({"source_commit": "1" * 40}))
        shutil.copyfile(VERIFIER, self.root / "scripts/verify_dist.py")
        self.git("init", "-q")
        self.git("add", ".")
        self.git("commit", "-qm", "Initial fixture")
        self.head = self.git("rev-parse", "HEAD")
        self.info = {"source_commit": self.head, "source_dirty": False, "core_commit": "1" * 40}
        shutil.copyfile(self.root / "web/index.html", self.root / "dist/index.html")
        (self.root / "dist/pkg/cart_wasm.js").write_text("// Static validator fixture.\n")
        (self.root / "dist/pkg/cart_wasm_bg.wasm").write_bytes(b"\x00asm\x01\x00\x00\x00")
        (self.root / "dist/.nojekyll").touch()
        self.write_info()

    def git(self, *args):
        return subprocess.check_output(
            ["git", "-c", "user.name=Test fixture", "-c", "user.email=fixture@example.invalid",
             "-c", "commit.gpgsign=false", *args],
            cwd=self.root, text=True, stderr=subprocess.PIPE,
        ).strip()

    def write_info(self, **changes):
        (self.root / "dist/build-info.json").write_text(json.dumps({**self.info, **changes}))

    def verify(self, *args):
        return subprocess.run(
            [sys.executable, "-B", str(self.root / "scripts/verify_dist.py"), *args],
            cwd=self.root, text=True, capture_output=True,
        )

    def assert_rejected(self, message):
        result = self.verify("--release")
        self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertIn(message, result.stderr)

    def test_clean_release_matches_the_actual_git_commit(self):
        result = self.verify("--release")
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report["source_commit"], self.head)
        self.assertIs(report["source_dirty"], False)

    def test_release_rejects_a_fabricated_source_commit(self):
        self.write_info(source_commit="0" * 40)
        self.assert_rejected("Build/source commit mismatch")

    def test_release_rejects_an_artifact_from_the_previous_commit(self):
        (self.root / "web/index.html").write_text("<!doctype html><title>Changed</title>")
        self.git("add", "web/index.html")
        self.git("commit", "-qm", "Update source after the artifact was built")
        self.assertNotEqual(self.git("rev-parse", "HEAD"), self.head)
        self.assert_rejected("Build/source commit mismatch")

    def test_release_rejects_a_build_marked_dirty_even_if_checkout_is_clean(self):
        self.write_info(source_dirty=True)
        self.assertEqual(self.git("status", "--porcelain"), "")
        self.assert_rejected("Release artifact was built with local changes")

    def test_release_rejects_uncommitted_source_changes_after_a_clean_build(self):
        (self.root / "web/index.html").write_text("<!doctype html><title>Uncommitted</title>")
        self.assert_rejected("Release checkout has local changes")

    def test_release_rejects_untracked_source_after_a_clean_build(self):
        (self.root / "new-source.js").write_text("// Untracked source.\n")
        self.assert_rejected("Release checkout has local changes")

    def test_local_preview_allows_explicitly_dirty_builds(self):
        self.write_info(source_dirty=True)
        (self.root / "web/index.html").write_text("<!doctype html><title>Local edit</title>")
        result = self.verify()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIs(json.loads(result.stdout)["source_dirty"], True)

    def test_build_source_fields_require_their_actual_json_types(self):
        for changes in ({"source_commit": None}, {"source_commit": 123},
                        {"source_commit": "not-a-commit"}, {"source_dirty": "false"},
                        {"source_dirty": 0}, {"source_dirty": None}):
            with self.subTest(changes=changes):
                self.write_info(**changes)
                for mode in ((), ("--release",)):
                    result = self.verify(*mode)
                    self.assertNotEqual(result.returncode, 0, result.stdout)
                    self.assertIn("Invalid build source metadata", result.stderr)


if __name__ == "__main__":
    unittest.main()

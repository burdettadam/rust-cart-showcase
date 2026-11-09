#!/usr/bin/env python3
"""Export an explicit, clean library commit for this independently hosted demo."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def git(repository, *args):
    return subprocess.check_output(["git", "-C", str(repository), *args])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("repository", type=Path)
    parser.add_argument("commit", help="Explicit full 40-character source commit")
    args = parser.parse_args()
    if len(args.commit) != 40 or any(c not in "0123456789abcdef" for c in args.commit):
        parser.error("use a full lowercase commit SHA, not a moving branch")
    actual = git(args.repository, "rev-parse", args.commit + "^{commit}").decode().strip()
    if actual != args.commit:
        parser.error("commit did not resolve exactly")
    files = git(args.repository, "ls-tree", "-r", "--name-only", actual).decode().splitlines()
    selected = sorted(p for p in files if p in {"Cargo.toml", "Cargo.lock"} or p.startswith("src/") or p.startswith("tests/"))
    destination = ROOT / "vendor" / "shopping-cart"
    if destination.exists():
        existing = {p.relative_to(destination).as_posix() for p in destination.rglob("*") if p.is_file()}
        unexpected = existing - set(selected)
        if unexpected:
            parser.error(f"unexpected existing vendor files; inspect before export: {sorted(unexpected)}")
    hashes = {}
    with tempfile.TemporaryDirectory(prefix="cart-export-") as directory:
        stage = Path(directory)
        for path in selected:
            content = git(args.repository, "show", f"{actual}:{path}")
            hashes[path] = hashlib.sha256(content).hexdigest()
            output = stage / path
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(content)
        for path in selected:
            output = destination / path
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes((stage / path).read_bytes())
    manifest = {"source_commit": actual, "source_tree": git(args.repository, "rev-parse", actual + "^{tree}").decode().strip(), "algorithm": "sha256", "files": hashes}
    (ROOT / "vendor" / "provenance.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"Exported {len(selected)} exact source files from {actual}")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Fail if a pinned library file was edited, removed or added."""

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def verify():
    manifest = json.loads((ROOT / "vendor/provenance.json").read_text())
    directory = ROOT / "vendor/shopping-cart"
    actual = {p.relative_to(directory).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in directory.rglob("*") if p.is_file()}
    if actual != manifest["files"]:
        raise SystemExit("Vendored source differs from its pinned manifest; re-export a reviewed core commit.")
    print(f"Verified {len(actual)} library files from {manifest['source_commit']}")
    return manifest


if __name__ == "__main__":
    verify()

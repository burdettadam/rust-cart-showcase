#!/usr/bin/env python3
"""Check the static allowlist, relative assets and binary WASM before uploading."""

import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
DIST = ROOT / "dist"


class Assets(HTMLParser):
    def handle_starttag(self, tag, attrs):
        values = dict(attrs)
        for attribute in ("src", "href"):
            path = values.get(attribute)
            if not path or tag == "a":
                continue
            if not path.startswith("./"):
                raise SystemExit(f"Expected a relative static asset: {path}")
            resolved = (DIST / path).resolve()
            if not resolved.is_relative_to(DIST.resolve()) or not resolved.is_file():
                raise SystemExit(f"Missing or escaping static asset: {path}")


def main():
    expected = {p.relative_to(ROOT / "web").as_posix() for p in (ROOT / "web").rglob("*") if p.is_file()}
    expected |= {"pkg/cart_wasm.js", "pkg/cart_wasm_bg.wasm", "build-info.json", ".nojekyll"}
    files = {p.relative_to(DIST).as_posix(): p for p in DIST.rglob("*") if p.is_file()}
    if set(files) != expected:
        raise SystemExit(f"Unexpected/missing deployment files: {sorted(set(files) ^ expected)}")
    if any(p.is_symlink() for p in DIST.rglob("*")):
        raise SystemExit("Pages artifact contains a symlink")
    forbidden = {".md", ".toml", ".rs", ".zip", ".bundle", ".log", ".py"}
    for name, path in files.items():
        if path.suffix in forbidden or any(part in {".git", ".evidence", "src", "vendor"} for part in Path(name).parts):
            raise SystemExit(f"Development material in deployment: {name}")
        if path.suffix == ".js":
            content = path.read_text()
            for match in re.finditer(r'(?:from\s*|import\s*)[\'\"]([^\'\"]+)[\'\"]', content):
                specifier = match.group(1)
                if not specifier.startswith(".") or not (path.parent / specifier).is_file():
                    raise SystemExit(f"Unresolved or nonrelative browser module: {name}: {specifier}")
    Assets().feed((DIST / "index.html").read_text())
    wasm = (DIST / "pkg/cart_wasm_bg.wasm").read_bytes()
    if wasm[:8] != b"\x00asm\x01\x00\x00\x00":
        raise SystemExit("Missing actual WebAssembly binary")
    info = json.loads((DIST / "build-info.json").read_text())
    if info["core_commit"] != json.loads((ROOT / "vendor/provenance.json").read_text())["source_commit"]:
        raise SystemExit("Build/core provenance mismatch")
    print(json.dumps({"files": len(files), "wasm_bytes": len(wasm), "wasm_sha256": hashlib.sha256(wasm).hexdigest(), "source_commit": info["source_commit"], "source_dirty": info["source_dirty"]}, indent=2))


if __name__ == "__main__":
    main()

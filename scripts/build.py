#!/usr/bin/env python3
"""Build only the static showcase artifact with the pinned Rust/WASM toolchain."""

import json
import os
from pathlib import Path
import shutil
import subprocess

from verify_vendor import verify

ROOT = Path(__file__).resolve().parents[1]


def output(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def main():
    provenance = verify()
    if output("wasm-bindgen", "--version") != "wasm-bindgen 0.2.130":
        raise SystemExit("Install wasm-bindgen-cli exactly 0.2.130 to match Cargo.toml.")
    subprocess.run(["cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "rust-cart-showcase"], cwd=ROOT, check=True)
    metadata = json.loads(output("cargo", "metadata", "--format-version", "1", "--no-deps"))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/cart_wasm.wasm"
    dist = ROOT / "dist"
    if dist.is_symlink():
        raise SystemExit("Refusing to replace a symlink at dist.")
    if dist.exists():
        shutil.rmtree(dist)
    shutil.copytree(ROOT / "web", dist)
    subprocess.run(["wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(dist / "pkg"), "--out-name", "cart_wasm", "--no-typescript"], cwd=ROOT, check=True)
    info = {
        "source_commit": output("git", "rev-parse", "HEAD"),
        "source_dirty": bool(output("git", "status", "--porcelain")),
        "core_commit": provenance["source_commit"],
        "rust": output("rustc", "--version"),
        "wasm_bindgen": "0.2.130",
        "target": "wasm32-unknown-unknown",
    }
    (dist / "build-info.json").write_text(json.dumps(info, indent=2) + "\n")
    (dist / ".nojekyll").touch()
    print(f"Built {dist}; source dirty: {info['source_dirty']}")


if __name__ == "__main__":
    os.chdir(ROOT)
    main()

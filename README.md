# Rust Cart Lab

A functional WebAssembly showcase of the Rust shopping-cart library, ready to
build for GitHub Pages. Add products, run the published example, compare sample
and live prices, and inspect validation and repricing behavior.

**The actual Rust cart runs in the browser.** JavaScript handles the interface and
HTTP; Rust handles response parsing, quantities, cart state, exact money, tax and
totals. This is a separate companion to the library-only challenge submission.

## Run locally

Install [rustup](https://rustup.rs/), Node 22 or later, and Python 3. The tracked
`rust-toolchain.toml` installs Rust 1.97.1 and the WASM target automatically.

```sh
cargo install wasm-bindgen-cli --locked --version 0.2.130
python3 scripts/build.py
node scripts/verify-wasm.mjs
python3 scripts/verify_dist.py
python3 -m http.server 8080 --directory dist
```

Open `http://localhost:8080`. Serve the directory over HTTP; opening HTML directly
with `file://` does not provide the module-loading environment required by WASM.
There is no npm dependency installation or JavaScript bundler.

## What to try

- Load the example: two cornflakes and one weetabix produce **15.02 subtotal,
  1.88 tax and 16.90 total**.
- Add the same product again; quantities combine and the latest unit price
  applies to all units of that product.
- Explore a one-cent price: the 12.5% tax rounds upward to **0.01**.
- Submit an invalid price or quantity; an error appears and the cart stays intact.
- Switch to live prices to request the public price API. Mode changes start a
  fresh cart, making the source of each cart's prices explicit.

Values have no currency symbol because the original contract does not name a
currency. The sample prices are fixed examples; live prices are fetched on each
addition. There are no accounts, payments, checkout, storage or analytics.

## GitHub Pages

The workflow `.github/workflows/pages.yml` builds and tests pull requests. On
`main`, it uploads **only `dist`** and deploys that verified artifact with the
GitHub Pages environment. All asset URLs are relative, including WASM loading,
so both a user site and a `/repository-name/` project site are supported.

1. Review this **companion repository** at
   [`burdettadam/rust-cart-showcase`](https://github.com/burdettadam/rust-cart-showcase).
   Feature work preserves its actual GitHub ancestry.
2. In Settings → Pages, select **GitHub Actions** as the publishing source.
   This repository was configured that way on 2026-11-09.
3. Open the feature PR, review its checks and both independent reviews, and make
   the human merge decision. A merge into `main` triggers the Pages workflow.
4. Observe a successful deployment and use the URL returned by GitHub. Do not
   infer success merely from a green local build.

The repository was created on 2026-11-09, and the user authorized the GitHub
connection for it and the private library fork. `feat/wasm-showcase` is the review
branch. The current handoff and actual PR/CI records identify publication and
verification status. A repository or configured workflow alone does not establish
a successful build, human approval or live Pages site.

## Quality and source provenance

```sh
python3 scripts/verify_vendor.py
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace --all-targets
cargo test --locked --workspace --doc
cargo test --locked -p equal-experts-shopping-cart --no-default-features --all-targets
node --test tests/*.test.mjs
cargo install cargo-llvm-cov --locked --version 0.6.21
cargo llvm-cov --locked --workspace --all-targets --ignore-filename-regex '/tests/' --fail-under-lines 91
python3 scripts/build.py
node scripts/verify-wasm.mjs
python3 scripts/verify_dist.py
```

The coverage gate includes the bridge and all production library Rust in the
workspace; tests and third-party dependencies are excluded. Native line coverage,
compiled-WASM execution, browser behavior, and deployment health are separate
observations. See `HANDOFF.md` and the retained evidence for measured outcomes.

`vendor/provenance.json` pins the library commit and SHA-256 of every exported
source file. These are exact copies, not a reimplementation. The exporter includes
only Cargo inputs, Rust source and tests. It excludes the original assignment,
private completion link, governance and review evidence. The original project
remains the canonical place to change business rules.

The current export uses published library commit
`c0f6aa90e06c1dfb7692b14d287e9d95da6221dc`. That repository is private; the
companion remains independently buildable without its credentials.

To update the snapshot after reviewing a new library candidate:

```sh
python3 scripts/export_core.py /path/to/library FULL_40_CHARACTER_COMMIT_SHA
cargo generate-lockfile
python3 scripts/verify_vendor.py
```

Review dependency changes and rerun the full checks. Do not edit the vendor copy
by hand. Cargo uses this repository's root profiles; the exported library's own
profiles remain byte-identical and Cargo reports that the workspace overrides them.

## Read the design

- [Architecture and interview Q&A](docs/architecture/README.md)
- [AI use and human oversight](docs/AI_USAGE.md)
- [Process and evidence](docs/PROCESS.md)
- [Development workflow](DEVELOPMENT.md)
- [Current handoff](HANDOFF.md)

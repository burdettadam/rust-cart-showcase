# Rust Cart Lab

[**Open the live Rust Cart Lab**](https://burdettadam.github.io/rust-cart-showcase/)

A functional WebAssembly showcase of the Rust shopping-cart library on GitHub
Pages. Add products, run the published example, compare sample and live prices,
and inspect validation and repricing behavior. [HANDOFF.md](HANDOFF.md) links
current source, verification, review and release records.

**The actual Rust cart runs in the browser.** JavaScript handles the interface and
HTTP; Rust handles response parsing, quantities, cart state, exact money, tax and
totals. This is a separate companion to the library-only challenge submission.

## Run locally

Install Git, [rustup](https://rustup.rs/), Node 24 and Python 3.12. These are the
maintained Node/Python versions used for verification. The tracked
`rust-toolchain.toml` installs Rust 1.97.1 and the WASM target automatically.

```sh
git clone https://github.com/burdettadam/rust-cart-showcase.git
cd rust-cart-showcase
cargo install wasm-bindgen-cli --locked --version 0.2.130
python3 scripts/build.py
node scripts/verify-wasm.mjs
python3 scripts/verify_dist.py
python3 -m http.server 8080 --directory dist
```

Open `http://localhost:8080`. Serve the directory over HTTP; opening HTML directly
with `file://` does not provide the module-loading environment required by WASM.
There is no npm dependency installation or JavaScript bundler. Build from a Git
checkout: the build records its commit and whether local changes exist. A normal
GitHub source ZIP lacks that history. Local previews may contain changes and show
`+ local changes`; release verification requires a clean checkout.

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

## Reproduce the GitHub Pages deployment

The workflow `.github/workflows/pages.yml` builds and tests pull requests. On
`main`, it uploads **only `dist`** and deploys that verified artifact with the
GitHub Pages environment. All asset URLs are relative, including WASM loading,
so both a user site and a `/repository-name/` project site are supported.

1. In repository Settings → Pages, select **GitHub Actions** as the publishing
   source. This repository was configured that way on 2026-11-09.
2. Prepare a feature PR into `main`, run the quality gate and obtain the configured
   independent reviews and user merge decision.
3. A merge into `main` triggers the Pages workflow. Observe the deployment's
   actual result and URL; a local build does not establish deployment health.

The existing live site is linked above. [HANDOFF.md](HANDOFF.md) is the single
current review/release entry point; the steps here describe how to reproduce a
release, not outstanding setup work.

## Quality and source provenance

```sh
python3 scripts/verify_vendor.py
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace --all-targets
cargo test --locked --workspace --doc
cargo test --locked -p equal-experts-shopping-cart --no-default-features --all-targets
node --test tests/*.test.mjs
python3 -B -m unittest discover -s tests -p 'test_*.py' -v
cargo install cargo-llvm-cov --locked --version 0.6.21
cargo llvm-cov --locked --workspace --all-targets --ignore-filename-regex '/tests/' --fail-under-lines 91
python3 scripts/build.py
node scripts/verify-wasm.mjs
python3 scripts/verify_dist.py --release
```

Run the release checks from a clean Git checkout. `--release` requires the artifact
metadata to identify the checked-out commit, a clean build and a clean current
checkout. The default `verify_dist.py` command in the quickstart remains suitable
for labeled local previews. Python tests use only the standard library.

The coverage gate includes the bridge and all production library Rust in the
workspace; tests and third-party dependencies are excluded. Native line coverage,
compiled-WASM execution, browser behavior, and deployment health are separate
observations. See `HANDOFF.md` and the retained evidence for measured outcomes.

`vendor/provenance.json` pins the library commit and SHA-256 of every exported
source file. These are exact copies, not a reimplementation. The exporter includes
only Cargo inputs, Rust source and tests. It excludes the original assignment,
private completion link, governance and review evidence. The original project
remains the canonical place to change business rules.

[The provenance manifest](vendor/provenance.json) identifies the exact exported
library commit and files. Its source repository is private; this companion
remains independently buildable without its credentials.

To update the snapshot after reviewing a new library candidate:

```sh
python3 scripts/export_core.py /path/to/library FULL_40_CHARACTER_COMMIT_SHA
cargo generate-lockfile
python3 scripts/verify_vendor.py
```

Review dependency changes and rerun the full checks. Do not edit the vendor copy
by hand. Cargo uses this repository's root profiles; the exported library's own
profiles remain byte-identical and Cargo reports that the workspace overrides them.

## Review path

1. Try the live example and validation scenarios above.
2. Read the [Rust/WASM bridge](src/lib.rs) and the
   [exported library source](vendor/shopping-cart/src).
3. Inspect [browser session handling](web/session.js),
   [transport](web/transport.js) and the corresponding tests.
4. Use the [architecture and interview Q&A](docs/architecture/README.md) to explain
   the boundaries, exact money and source provenance.
5. Read [AI use and human oversight](docs/AI_USAGE.md) and
   [PROCESS](docs/PROCESS.md), then follow [HANDOFF](HANDOFF.md) to actual reviews.

[DEVELOPMENT.md](DEVELOPMENT.md) describes the maintained workflow and recovery.
The live page's source and architecture links follow its displayed build commit
when metadata is available.

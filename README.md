# Rust Cart Lab

[**Open the live demo**](https://burdettadam.github.io/rust-cart-showcase/) ·
[Architecture and Q&A](ARCHITECTURE.md) · [Development guide](DEVELOPMENT.md)

A shopping-cart library running in your browser through WebAssembly. Rust owns
validation, cart state, exact money, tax and totals; JavaScript handles the
interface and HTTP. This public companion is separate from the library-only
challenge submission.

## Try it

Run the example: **two cornflakes and one weetabix produce 15.02 subtotal,
1.88 tax and 16.90 total**. Then try rounding, repricing or a rejected price.
The custom response form lets you send raw price JSON to Rust and inspect the
resulting cart snapshot.

Sample prices give repeatable results. Live mode fetches a new quote on each
addition; switching sources starts a fresh cart. Amounts have no currency symbol
because the contract does not specify one. The demo has no accounts, payments or
saved baskets.

## Run locally

Install Git, [rustup](https://rustup.rs/) and Python 3.12. The tracked
`rust-toolchain.toml` installs Rust 1.97.1 and the WASM target.

```sh
git clone https://github.com/burdettadam/rust-cart-showcase.git
cd rust-cart-showcase
cargo install wasm-bindgen-cli --locked --version 0.2.130
python3 scripts/build.py
python3 scripts/verify_dist.py
python3 -m http.server 8080 --directory dist
```

Open `http://localhost:8080`. Use a Git checkout and serve it over HTTP: a source
ZIP lacks the Git history used for build identity, and `file://` does not provide
the required module-loading environment. Local previews may show `+ local changes`.
The [development guide](DEVELOPMENT.md) contains the full checks, Node requirement,
release verification, deployment and library-update commands.

## Review and verification

| Area | Start here |
| --- | --- |
| Cart rules and exact prices | [Cart](vendor/shopping-cart/src/cart.rs), [price parser](vendor/shopping-cart/src/price_response.rs) |
| Rust/WASM boundary | [Bridge](src/lib.rs) and [bridge tests](tests/bridge.rs) |
| Browser requests and state | [Transport](web/transport.js), [session](web/session.js) and [tests](tests) |
| Design decisions and interview preparation | [Architecture and Q&A](ARCHITECTURE.md) |
| Change history and reviews | [Pull requests](https://github.com/burdettadam/rust-cart-showcase/pulls?q=is%3Apr) |

[CI and Pages runs](https://github.com/burdettadam/rust-cart-showcase/actions/workflows/pages.yml)
record the checks and deployments. Production Rust line coverage must be at least
**91%**; measured results belong to their actual CI runs.

The [provenance manifest](vendor/provenance.json) identifies the exact library
export. Its canonical repository is private, but this companion builds without
those credentials. The live page's source and architecture links follow its
displayed build commit when metadata is available.

## AI contribution

ChatGPT produced the first drafts of the implementation, tests and documentation.
The user directs the work and retains design and acceptance judgment.
[AI usage and development history](docs/AI_USAGE.md) records contributions,
consequential corrections and the distinction between executed checks and personal
review. Publishing this companion does not submit the challenge.

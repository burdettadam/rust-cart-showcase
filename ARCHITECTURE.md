# Architecture authority

The maintained design and interview Q&A is
[docs/architecture/README.md](docs/architecture/README.md).

The companion calls the original Rust shopping-cart library through a small WASM
binding. Rust owns validation, quantities, pricing, state and totals. JavaScript
owns DOM rendering and asynchronous browser HTTP. Raw response text crosses the
boundary; decimal amounts return as strings.

`vendor/shopping-cart` is a pinned exact source export. The original project
remains the canonical owner of business rules. Update it through the exporter,
verify its manifest, and rerun checks. The showcase is separate from the
library-only challenge submission. Accepted scope and actual verification state
are recorded in `PLANS.md` and `HANDOFF.md` respectively.

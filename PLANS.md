# Accepted goal: Rust cart WebAssembly showcase

The user requested a functional WebAssembly showcase on GitHub Pages. The
assignment remains a separate library project; the companion makes its behavior
available for review. The user owns scope, design judgment, acceptance and merges.
ChatGPT supplies implementation and technical evidence.

## Accepted behavior

- Run the existing Rust cart rules in WebAssembly; JavaScript does no money math.
- Export a fixed, verified library revision into `vendor/shopping-cart`, recording
  its commit and file hashes. The original library owns business-rule changes.
- Offer repeatable sample prices and optional live API requests.
- Preserve raw price text until Rust parses it; display Rust-formatted totals.
- Show addition, quantity combination, latest-price behavior, upward tax rounding
  and errors that preserve state. Reset discards only the local demo cart.
- Serialize operations; a late request must not repopulate a reset or switched cart.
- Use relative static assets that work under a GitHub Pages repository path.
- Deploy only `dist`; exclude assignment instructions, the private completion
  link, review evidence and development files from the artifact.

## Work records

| Work | Authoritative record |
| --- | --- |
| Initial Rust/WASM bridge, browser interface, verified source export and Pages release | [PR #1](https://github.com/burdettadam/rust-cart-showcase/pull/1) |
| Exact-price/HTTP corrections, source links, clean release verification and initial documentation cleanup | [PR #2](https://github.com/burdettadam/rust-cart-showcase/pull/2) |
| Documentation and navigation consolidation | [PR #3](https://github.com/burdettadam/rust-cart-showcase/pull/3) |
| Scenario result navigation and remaining documentation refinements | [PR #4](https://github.com/burdettadam/rust-cart-showcase/pull/4) |

Those PRs retain their actual candidates, findings, checks and release outcomes.
[AI usage and history](docs/AI_USAGE.md) records attribution and consequential
corrections without repeating volatile test counts.

Final challenge submission requires a separate explicit user instruction.

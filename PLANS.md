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

## Every third unit free

The approved offer repeats across products in addition order, including bulk
adds. The showcase enables the offer. Library callers select it with
`ShoppingCart::with_every_third_free`. Failed adds preserve the next award,
and a reset starts a new group. The displayed receipt identifies free units,
their savings at the latest prices, and the taxable subtotal.

The fresh clone contained the verified export of original commit
`a637640a3ffee1e7cabcfcb9671cdd011d5c9869`. Following the user's recovery
instruction, the library development checkout was reconstructed from its nine
hash-verified files. Its baseline commit is `3119f7d76e568aa570fbceae0b35febdf5faf815`.
The library feature commit in `vendor/provenance.json` descends from this
reconstructed history. The earlier private Git history was unavailable.

## Decimal money arithmetic

The user requested a separate branch and PR to calculate cart money with
`rust_decimal::Decimal`. This builds on the repeating promotion in PR #6,
so `feat/decimal-money` targets `feat/every-third-free`.

Use checked Decimal multiplication, addition and subtraction throughout receipts.
Apply the existing 12.5% rate with explicit upward rounding to two decimal places.
Keep exact-cent validation, the established maximum amount, quantity bounds and
atomic errors. Verify existing results and rounding boundaries at small and
large amounts. The corresponding library source is retained on
`library/decimal-money`.

## Cart readability

The user approved naming the money scale, tax rate and maximum amount,
extracting `calculate_tax`, and moving empty receipt initialization into
`CartReceipt::empty`. Implement on `refactor/cart-readability` with the
verified library source retained on `library/cart-readability`. Existing
pricing, promotion, validation and two-decimal formatting tests provide the
regression checks. The user authorized merging this improvement after checks
and independent reviews.

# Architecture and questions to be comfortable answering

## Boundaries

The browser loads a static HTML interface and a real WebAssembly module. The
Rust bridge owns one `ShoppingCart` from the pinned library. A browser request
retrieves raw price JSON; the shared Rust parser validates that text without
passing money through JavaScript's binary floating-point number representation.
The bridge returns an owned snapshot with monetary values formatted as strings.

| Concern | Owner | Evidence path |
| --- | --- | --- |
| Product, quantity and amount invariants | Original cart library | `vendor/shopping-cart/src/cart.rs` and its tests |
| Exact JSON numeric token validation | Shared library parser | `vendor/shopping-cart/src/price_response.rs` |
| Native HTTP request and error mapping | Optional library HTTP adapter | `vendor/shopping-cart/src/http_price_client.rs` |
| Browser/WASM boundary and formatted state | Thin Rust adapter | `src/lib.rs`, `tests/bridge.rs` |
| Browser HTTP, response limits, cancellation | JavaScript transport/session | `web/transport.js`, `web/session.js`, Node tests |
| Rendering and accessible interaction | HTML/CSS/JavaScript interface | `web/index.html`, `web/app.js`, `web/style.css` |
| Compile, artifact checks and Pages deployment | Build scripts and GitHub Actions | `scripts/`, `.github/workflows/pages.yml` |

The core library is portable with `default-features = false`. Its normal default
still enables the native HTTP adapter. The showcase builds only its own package
for WASM; native workspace tests also exercise the original HTTP adapter against
controlled local servers.

## Why a separate showcase?

The challenge explicitly asks for a library and excludes application submissions.
The user separately requested an interactive showcase. Keeping two projects
preserves both purposes. The companion is independently buildable and contains a
pinned export of the library's Cargo inputs, source and tests. The original
assignment and private completion link are not exported or deployed.

This is deliberate source vendoring from the private library fork into the
independently buildable public companion.
A full source commit and file hashes make the relationship auditable. It does
create an explicit update step; `verify_vendor.py` rejects missing, added or
changed files relative to that manifest. The manifest is a consistency check,
not a signature or independent proof of trust. Reviewers compare it to the
retained source commit. A pinned Git dependency is an alternative if the Pages
build has appropriate private repository access. Neither
approach justifies force-pushing unrelated Git history.

## Questions and answers

### 1. Is this really executing Rust?

Yes. `cargo build --target wasm32-unknown-unknown` creates a binary module from
our Rust bridge and the existing library. `wasm-bindgen --target web` generates
the JavaScript glue. The interface instantiates `BrowserCart` from that module.
The artifact tests instantiate the built WASM bytes in Node and call the same
exports. A normal-looking interface by itself would not prove this boundary.

### 2. Why not calculate totals in JavaScript too?

Two implementations of money rules can disagree. Rust is the one implementation
of subtotal, tax, total, repricing and validation. JavaScript renders strings and
coordinates requests. Even a live response arrives as `response` text; parsing
it first as JavaScript JSON would risk rounding large numeric prices before Rust
sees them. The bridge's precision test uses `90071992547409.91` specifically to
exercise a value beyond safe integer cents in JavaScript.

### 3. What exactly does Rust guarantee here?

The compiler provides memory safety for safe Rust and rejects many invalid
ownership/type relationships. The crate forbids authored unsafe code. Business
correctness still needs explicit validation and tests. Rust does not automatically
choose the correct tax rule, prevent every integer overflow, ensure a correct
price service, or eliminate asynchronous browser races. Checked arithmetic,
atomic state updates, raw numeric parsing and cancellation policy provide those
application-specific protections. Third-party crates may have their own unsafe
internals; this project does not claim a transitive unsafe-code audit.

### 4. How does tax rounding work?

The core uses checked integer cents. At 12.5%, tax in cents is one eighth of the
subtotal in cents, rounded toward positive infinity. It computes quotient plus
one when the remainder is nonzero. An exact cent stays unchanged. Tax is applied
once to the complete subtotal, not independently to each line. For 1502 cents,
187.75 tax cents becomes 188, making the total 1690 cents.

### 5. Why does the WASM method take quantity as a string?

A JavaScript-to-WASM unsigned integer conversion can coerce unexpected numbers.
The adapter accepts text, requires ASCII decimal digits, parses `u32`, then lets
the core require a positive value and check combined quantity. Negative numbers,
fractions and out-of-range values produce ordinary errors rather than wrapping
or truncating. Prices also cross as raw text. Returned quantities fit `u32`, so
JavaScript can represent each returned quantity exactly.

### 6. What happens when adding a product already in the cart?

The core combines quantity and uses the newest unit price for all units of that
product. One unit at 2.52 followed by one at 3.25 produces 6.50 subtotal, 0.82 tax
and 7.32 total. The UI calls this behavior out in its repricing scenario. This is
an accepted rule, not a universal retail policy; preserving individual purchase
price lots would require a different model and a user decision.

### 7. What if the API or validation fails?

The transport rejects unsuccessful HTTP status, limits response bytes and applies
a request time budget. Rust rejects invalid response shapes and unsupported
prices. The cart constructs and validates the next snapshot before assigning it,
so any failed addition preserves the prior visible state. Native adapter errors
continue to distinguish request, status, response and invalid-price failures.
The browser reports a useful message and keeps the previous cart visible.

### 8. How are reset and overlapping requests handled?

The browser session serializes additions. Reset and mode changes invalidate the
current operation and abort its request. Before applying a response, the session
checks that it still belongs to the current generation. An abort is not assumed
to guarantee that every already-resolved promise disappears. Tests deliberately
resolve stale requests after reset to ensure they cannot repopulate the cart.
The Rust cart itself is synchronous and does not need a concurrent mutation model.

### 9. Why offer both sample and live prices?

Sample mode gives a repeatable, network-independent demonstration of known
results. Live mode shows actual browser integration with the supplied public
price service. Switching sources clears the cart and changes its label, so an
observer knows which source supplied prices. A failed live request stays a
visible failure; it does not silently substitute sample data.

### 10. What are the deliberate limits?

Quantities are positive `u32` values. Every monetary amount fits `u64` cents.
Prices must be nonnegative exact cents; extra significant fractional digits and
exponent notation are rejected. Harmless trailing zeroes are accepted. Product
IDs use lowercase ASCII slug syntax. The demo has no persistence, inventory,
authentication, checkout, discounts or multi-currency behavior. Refreshing the
page creates a new cart. These limits preserve the original accepted rules.

### 11. Does over 90% coverage prove correctness?

No. The gate is at least 91% production Rust source line coverage. It covers the
native build of the bridge and original library, including the optional HTTP
adapter. Tests and dependencies do not inflate the denominator. Separate tests
execute the actual release WASM, and JavaScript tests exercise request timing,
response limits and session cancellation. Coverage does not imply branch,
mutation, cross-browser or hosted deployment coverage. The original project
already demonstrated why: independent review found a JSON-shape defect despite
high coverage, and concrete regression tests were needed.

### 12. How does GitHub Pages run an application without a server?

Pages serves static HTML, CSS, JavaScript and WASM. The browser executes the cart
and calls the public price service directly. No server-side Rust process runs on
Pages. Relative imports and the generated WASM initializer keep assets under the
current project path. The workflow verifies and uploads only `dist`, then deploys
that same artifact through the Pages environment. Settings must select GitHub
Actions as the publishing source. Release verification also requires the artifact's
recorded showcase commit to match the checked-out commit, with a clean build and
checkout. Local previews may include changes and are labeled accordingly. These
checks make source identity explicit; a local build is not a hosted deployment.

The page's View source and Architecture links use the recorded full showcase
commit when valid metadata is available. Their public repository fallback keeps
code inspection available without access to the original private library.

### 13. What makes the process honest and reviewable?

Commit authorship identifies AI contributions. The user selected Rust, requested
WASM and GitHub Pages, accepted the business assumptions, and retains judgment.
AI-generated implementation and independent AI reviews are identified as such.
Checks report their actual candidate, commands and outcomes. Missing GitHub
access, human review or hosted verification remains visible. No AI report grants
human approval and no fabricated PR, CI result or publication claim is used.

## Primary technical references

- [wasm-bindgen: no-bundler browser loading](https://wasm-bindgen.github.io/wasm-bindgen/examples/without-a-bundler.html)
- [Rust wasm32-unknown-unknown target](https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html)
- [GitHub Pages custom workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
- [GitHub Pages publishing source](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)

The Cargo manifests/lockfiles, exported source hash manifest, tests and actual
build evidence define this project's concrete behavior and dependency versions.

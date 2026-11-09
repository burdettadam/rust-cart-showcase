# WebAssembly showcase handoff

## Implemented outcome

Rust Cart Lab is a separate, independently buildable companion to the shopping-cart
library. It runs the real library through WebAssembly, supplies sample/live price
modes, shows exact cart snapshots and demonstrates rounding, repricing, validation
and reset. GitHub Pages workflow and static artifact are prepared.

The original library source is pinned to
`7b2513643e51668e9caae9d88a6b3a7ba03e34f3`. Its cart algorithm remains unchanged
from the original reviewed candidate. The source export contains nine exact files;
`vendor/provenance.json` and the exporter make that relationship inspectable.

The source is on `feat/wasm-showcase`. `main` is the companion's initial workflow
baseline; feature topics have not been merged into it. Git is authoritative for
current source identity. The final delivery report and build-info.json bind the
reviewed source, actual artifact and retained evidence to full commit identities.

## Executed verification

The complete corrected implementation passed eleven maintained checks on clean
commit `4fb2bd009b189fb3ccb85463dfc3964c19269feb`. The final documentation checkpoint
is verified again and identified by the delivered verification.json; no claim
that a dirty source tree has the identity of HEAD is used.

| Evidence | Observed result |
| --- | --- |
| Native workspace behavior tests | 53 passed: 18 cart, 18 HTTP, 8 parser, 9 bridge |
| Documentation test | 1 passed |
| Feature-disabled library tests | 26 passed |
| JavaScript transport/session tests | 24 passed |
| Actual release WASM checks | 8 passed |
| Production Rust source line coverage | 213/219 = 97.26%; gate >=91% |
| Static artifact | 9 files; 206649-byte actual WASM binary |
| Formatting, Clippy, provenance and artifact checks | Passed |

Coverage includes bridge 60/60, core cart 85/85, shared parser 29/29 and native HTTP 39/45.
The six uncovered native constructor-wrapper lines remain in the denominator.
Tests and third-party dependencies are excluded. This is native line coverage,
not branch coverage, JavaScript coverage or exhaustive browser coverage.

Actual browser checks confirmed the example 15.02/1.88/16.90, upward tax, repricing,
fractional-cent rejection, invalid quantity handling and large exact prices.
A real live cornflakes request succeeded in the browser and returned 2.52, with
Rust total 2.84 for one unit. Mobile 375px content and a project subpath loaded the
module and showed the same sample results. The final QA report records any later
layout recheck and its source identity. Cross-browser/device testing is not claimed.

## Review findings and corrections

The core received two independent round 2 PASS reports after adversarial review
exposed an existing exact 16 KiB HTTP boundary discrepancy. A native regression failed
before bounded EOF lookahead and passed afterward, including one-over rejection.
Both initial findings and the final core reviews are retained.

Showcase contract review found that the browser decoder stripped a leading UTF-8
BOM before shared Rust validation. Three regressions failed before preserving the
BOM; 24 Node tests passed afterward. Native and compiled-WASM rejection cases now
include the original BOM text. Mobile QA also exposed long totals splitting
mid-number; the rendering fix uses string length to select compact, unbroken
monetary output. Ordinary totals retain their original layout.

The separate final contract and adversarial reports identify the precise final
showcase candidate and dispositions. They are AI reviews and do not grant human
approval. No review requirement, coverage gate or business rule was weakened.

## Verification environment

One renewed coverage export failed when managed browser-preview synchronization
created transient files inside Cargo's output directory. Behavior tests passed
in that run; the coverage export did not. Its failed log is retained. Moving
compiler output outside the synchronized tree via CARGO_TARGET_DIR eliminated
the race; the complete eleven-check run then passed. Build scripts obtain the
target directory from Cargo metadata and support this without source changes.

## External state and next action

The GitHub connection returned 404 for the inferred personal fork and exposes only
the ElevenID installation, with no personal repositories in the current inventory.
No showcase remote, hosted PR, hosted CI, main merge or Pages deployment has
occurred. This does not prove that the user's fork does not exist.

The concrete remaining input is the intended showcase repository URL with access
through the GitHub connection. Inspect existing ancestry, push a feature branch,
open the prepared PR, and let the user make the merge decision. Enable Settings
→ Pages → GitHub Actions, then observe deployment and its actual URL. Never
force-push this independently initialized history over an existing repository.
The original challenge completion link has not been activated.

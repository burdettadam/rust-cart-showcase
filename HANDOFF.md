# WebAssembly showcase handoff

## Current GitHub integration: 2026-11-09

The public repository is
[`burdettadam/rust-cart-showcase`](https://github.com/burdettadam/rust-cart-showcase),
created with initial `main` commit `0ec8238570b597cee798229eaae3aab446cdb057`.
The user authorized the GitHub connection for this repository and the private
library fork. `feat/wasm-showcase` is the combined review branch on that actual
GitHub base, importing the archived companion checkpoint
`fb486cd99b99a8a34e476aded6af3a5f5d72eeb7` and its meaningful increments.

GitHub API import messages identify the AI-generated work and original source
author/SHA. Their authorized account identity does not imply human coding or
review. The saved archives preserve the original full local histories. The
public replay omits the transient Python bytecode files from an early historical
commit; those generated files were already absent from the reviewed final tree.

Publication is being reconciled with the real repositories. Fresh exact-candidate
checks, independent reviews, hosted PR/CI results and human merge review must be
recorded separately as integration completes. No new measured result, hosted PR
number or live Pages deployment is claimed by this documentation checkpoint.
The sections below retain evidence for the archived source; those results do not
automatically apply to a new GitHub commit.

The prepared current PR description is `docs/pull-requests/01-wasm-showcase.md`.
Pages remains configured in source to deploy only after the full quality gate on
`main`; the user retains the merge decision. Settings → Pages was set to GitHub
Actions on 2026-11-09. The live URL must still come from an observed successful
deployment.

The current source export pins the published private library candidate
`c0f6aa90e06c1dfb7692b14d287e9d95da6221dc`, tree
`273b3daf46c960c3635072f77e7af45a322a42d1`. Its nine exported Cargo/source/test
files are byte-for-byte identical to the archived export. Only the provenance
commit and tree changed; the publication documentation is outside that export.

## Implemented outcome

Rust Cart Lab is a separate, independently buildable companion to the shopping-cart
library. It runs the real library through WebAssembly, supplies sample/live price
modes, shows exact cart snapshots and demonstrates rounding, repricing, validation
and reset. GitHub Pages workflow and static artifact are prepared.

The archived companion pinned its original library source to
`7b2513643e51668e9caae9d88a6b3a7ba03e34f3`. Its cart algorithm remains unchanged
from the original reviewed candidate. The source export contains nine exact files;
`vendor/provenance.json` and the exporter make that relationship inspectable.

In the archived project, source was on `feat/wasm-showcase` and `main` held the
initial workflow baseline. The GitHub repository has the separate actual base
identified above. Git is authoritative for current source identity. The retained
final archive evidence identifies `fb486cd99b99a8a34e476aded6af3a5f5d72eeb7`;
build-info.json identifies the source and core used for each generated artifact.

## Retained archive verification

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

## Historical access limitation and next action

Before 2026-11-09, the GitHub connection returned 404 for the inferred personal
fork and exposed only the ElevenID installation. At that checkpoint no showcase
remote, hosted PR, hosted CI, main merge or Pages deployment had occurred. The
browser discovery and selected-repository authorization resolved the access
limitation; they did not grant merge approval or establish deployment health.

Finish the integrated candidate, bind current checks and reviews to the actual
PR, and let the user decide its merge into `main`. Then observe the authorized
Pages deployment and its real URL. The original challenge completion action
remains separate and has not been activated.

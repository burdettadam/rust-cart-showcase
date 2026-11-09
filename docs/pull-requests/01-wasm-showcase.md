# Proposed PR: demonstrate the Rust cart through WebAssembly on GitHub Pages

## Why

The user wants an interactive way to explore and explain the library's behavior.
The challenge submission remains a library, so this separate companion makes the
same Rust implementation visible in the browser without submitting an app as the
challenge solution.

## Changes and resulting behavior

- Pin exact library source with commit identity and file hashes. Compile its
  existing cart and shared response parser to real WebAssembly.
- Add a thin Rust binding with string inputs and exact monetary string outputs.
- Provide sample/live prices, the reference example, cart inspection, edge-case
  scenarios and custom responses. Reset/mode changes invalidate stale requests.
- Bound live transport by time/bytes and preserve UTF-8 text, including a BOM,
  so shared Rust validation remains authoritative.
- Build relative static assets for project subpaths. Upload only dist and deploy
  on main after the full quality gate. PRs run checks without deployment.
- Retain the actual git-agent skills/roles, architecture Q&A, honest AI disclosure
  and native Git checkpoints.

## Evidence for review

The retained archive verification identifies source
`fb486cd99b99a8a34e476aded6af3a5f5d72eeb7` and every command/log/hash. Its corrected
implementation checks passed 53 native tests, 1 doctest,
24 JavaScript tests and 8 actual-WASM checks. Production Rust coverage is 213/219
(97.26%), with tests/dependencies excluded and all production included.
Browser QA exercised the reference result, errors, live requests, narrow layout
and a project subpath. These are historical results. Fresh integrated-candidate
verification, independent reviews and hosted CI must be bound to the real PR
head/base; no new hosted result is claimed by this draft.

Initial independent findings and red/green regressions are retained: the library's
native response limit and browser BOM handling were corrected. Final contract and
adversarial reports bind the reviewed candidate; automated evidence and AI reviews
do not replace the user's human merge judgment.

## Publication and human decision

Repository: `burdettadam/rust-cart-showcase`. Review branch: `feat/wasm-showcase`;
target: `main`. Initial GitHub base:
`0ec8238570b597cee798229eaae3aab446cdb057`.

The review branch imports the saved increments on the actual repository ancestry.
GitHub API import messages identify AI origin and original author/source SHA;
the authorized account identity does not imply human coding. The saved archives
preserve the original full histories. Public replay omits temporary Python
bytecode from one historical intermediate commit; the reviewed final source
already excluded those files.

This remains a prepared description until linked to an actual GitHub PR. The user
reviews the current checks and two independent reviews and decides the merge.
Pages must use GitHub Actions as its source; deployment stays restricted to
`main`, and its URL must come from a successful observed deployment. Neither
source publication nor Pages deployment submits the challenge.

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

The delivered verification report identifies the exact final candidate and every
command/log/hash. Corrected implementation checks passed 53 native tests, 1 doctest,
24 JavaScript tests and 8 actual-WASM checks. Production Rust coverage is 213/219
(97.26%), with tests/dependencies excluded and all production included.
Browser QA exercised the reference result, errors, live requests, narrow layout
and a project subpath. Hosted CI and Pages publication remain unverified.

Initial independent findings and red/green regressions are retained: the library's
native response limit and browser BOM handling were corrected. Final contract and
adversarial reports bind the reviewed candidate; automated evidence and AI reviews
do not replace the user's human merge judgment.

## Publication and human decision

This is a prepared description, not an existing GitHub PR. The connection cannot
currently resolve the personal fork. Supply the intended accessible companion
repository, preserve its ancestry, and open the real feature PR. The user reviews
and decides the merge. Pages must use GitHub Actions as its source, and the actual
deployment URL must come from a successful observed deployment.

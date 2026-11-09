# Accepted goal: Rust cart WebAssembly showcase

The user requested a functional WebAssembly showcase on GitHub Pages on
2026-11-08. Build a separate companion so the original assignment stays a
library submission. The user retains design judgment, merge and submission
decisions. ChatGPT supplies implementation and technical evidence.

## Scope and accepted behavior

- Execute the existing Rust cart rules in WebAssembly; no JavaScript money math.
- Export a fixed, verified library source revision into `vendor/shopping-cart`.
  Record file hashes and source commit; one maintained library owns the rules.
- Provide deterministic example prices and optional live price API requests.
- Preserve exact price text until Rust parses it. Display Rust-formatted totals.
- Show addition, quantity combination, latest-price behavior, upward tax rounding,
  and errors that leave state unchanged. Reset discards only the local demo cart.
- Serialize operations; no late fetch may repopulate a reset or switched session.
- Build relative static assets for a GitHub Pages project path.
- Ship only `dist` to Pages. No assignment instructions, completion URL, private
  evidence, or development files belong in that artifact.

## Delivery slices

1. Restore reviewed library and establish the companion's Git/agent baseline.
2. Add feature-gated native HTTP and shared exact response parsing to the library
   on its separate `feat/wasm-portability` branch.
3. Add the Rust/WASM bridge, native tests and actual compiled-WASM smoke tests.
4. Add an accessible responsive UI and meaningful transport/race tests.
5. Add reproducible build, GitHub Pages workflow and architecture Q&A.
6. Freeze source; execute quality and >=91% production Rust line coverage;
   collect two independent reviews, at most three rounds; retain evidence.

## Ownership and constraints

The root agent owns Git administration, integration, bridge and deployment work.
Assigned UI work uses a distinct worktree. Independent reviewers do not edit the
candidate. Topic commits may be assembled on an integration feature branch;
`main` merge remains the user's decision.

The user created the public
[`burdettadam/rust-cart-showcase`](https://github.com/burdettadam/rust-cart-showcase)
repository on 2026-11-09 and authorized the GitHub connection for it and the
private library fork. Its initial `main` commit is
`0ec8238570b597cee798229eaae3aab446cdb057`. `feat/wasm-showcase` imports the reviewed
companion onto that real base; the original local histories remain in the saved
archives. GitHub API import messages identify AI origin and original source
author/SHA rather than claiming human authorship from the account identity.

Renew checks and two independent reviews against the actual integrated candidate,
then present the real PR for the user's merge decision. Preserve main-only Pages
deployment and observe its actual result after an approved merge. Never force-push
unrelated local history, invent a PR or claim an unobserved deployment.

## Accepted audit improvements: 2026-11-09

The user requested implementation of all proposed improvements after the merged
baseline audit. Add source and architecture links tied to the displayed build,
remove unused response-body session state, and enforce source identity and a clean
checkout during release-artifact verification. Keep explicitly labeled local
previews usable. Re-export the reviewed library fixes rather than changing the
vendored business rules independently.

Refresh the README quickstart and live-site link, consolidate current status in
`HANDOFF.md`, and preserve historical process and honest AI attribution. Retain
all quality gates and two independent reviews. Main-only Pages deployment is the
existing release path; challenge submission remains separately unauthorized.

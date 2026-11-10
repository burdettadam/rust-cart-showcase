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

## Completed work

| Work | Authoritative record |
| --- | --- |
| Initial Rust/WASM bridge, browser interface, verified source export and Pages release | [PR #1](https://github.com/burdettadam/rust-cart-showcase/pull/1) |
| Exact-price/HTTP corrections, source links, clean release verification and initial documentation cleanup | [PR #2](https://github.com/burdettadam/rust-cart-showcase/pull/2) |

Those PRs retain their actual candidates, findings, checks and release outcomes.
[AI usage and history](docs/AI_USAGE.md) records attribution and consequential
corrections without repeating volatile test counts.

## Approved documentation and navigation cleanup

[PR #3](https://github.com/burdettadam/rust-cart-showcase/pull/3) tracks the current
accepted cleanup. Make README the human entry point, keep the full architecture
and interview Q&A in root `ARCHITECTURE.md`, put complete maintenance commands in
`DEVELOPMENT.md`, and combine concise history with the honest AI disclosure.
Remove superseded preparation drafts, duplicate handoffs and stale worker notes
after preserving useful details. Git and actual PRs retain the original history.

Move the existing source/design links near the primary demo action, preserve their
commit pinning on desktop and mobile, shorten repeated FAQ content and correct
the skip-link wording. Preserve all cart controls and accepted behavior.

The user authorized implementation and merges for this cleanup. Root owns Git
administration, integration and publication; each implementation worktree has one
writer, and independent reviewers do not edit the candidate. Preserve the actual
agent skills/roles, licensing, **91% production Rust line-coverage gate** and **two
independent reviews**, with at most three rounds, under [GOVERNANCE](GOVERNANCE.md).

Check changed references and navigation, use the existing quality/release gates,
and bind reviews to the integrated candidate before merging. Observe the actual
Pages deployment and focused live behavior. The PR records verification, review,
merge and deployment outcomes. Merge authorization does not establish completed
personal code inspection. Final challenge submission remains separately unauthorized.

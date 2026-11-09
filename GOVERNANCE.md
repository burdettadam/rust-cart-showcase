# Showcase governance

The user owns scope, design judgment, acceptance and merges. The 2026-11-08 user
request authorizes creation of a WebAssembly showcase on GitHub Pages. The
assignment library stays a separate project. Publication still requires a known
authorized repository and actual access. Never activate the assignment completion
link. The application has no accounts, checkout, payments, database or analytics.

ChatGPT may implement, investigate, test, commit and prepare PRs for the accepted
goal. AI work and review must be disclosed honestly. The root agent coordinates
Git administration. Use one writer per assigned branch/worktree; preserve other
writers' changes. Native Codex role files are configuration, not evidence a
particular client loaded them.

## Acceptance

All formatting, lint, native tests, doctests, compiled-WASM tests, build and
artifact checks must pass. Production Rust line coverage must be >=91.0%,
including the bridge and vendored library actually compiled by the showcase.
Exclude tests and dependencies, never production branches to improve the score.
Report this native line-coverage metric separately from WASM execution and UI
verification; none implies exhaustive correctness or branch coverage.

Two independent reviewers inspect the fixed candidate: contract and
adversarial/regression. Maximum three rounds. Preserve initial findings; fixes
create a new candidate and renew affected checks. Follow the inherited
`work-goal` review guide. Missing reviews and blocked checks are not passes.
The user retains human review before a version-producing merge. No automatic
review is human approval. Keep generated evidence ignored, retain concise
decisions in source, and save evidence with delivery while remote access is
unavailable. No weakening these criteria to make a candidate pass.

## Transparency and recovery

Author AI-generated commits as ChatGPT (AI). Distinguish source checkpoints,
executed local checks, hosted CI, independent reviews, user approval, deployment
and submission. Keep a recoverable source archive and Git bundle while no remote
is available. Native GitHub protections are unverified until observed.

The git-agent 0.2.1 skills and Codex roles are copied from the original project's
actual initializer output, with this companion's maintained authority documents.
They are not a second initializer run; provenance is in `docs/agent-init/README.md`.

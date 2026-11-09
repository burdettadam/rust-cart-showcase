# Current showcase review status

This is the maintained entry point for the source, release and review records.
Each PR identifies its exact candidate, checks, independent reviews and merge
outcome. The deployment run and generated `build-info.json` identify a release;
source publication alone does not establish that a deployment succeeded.

| Review item | Record |
| --- | --- |
| Live showcase | [Rust Cart Lab](https://burdettadam.github.io/rust-cart-showcase/) |
| Default branch | [main](https://github.com/burdettadam/rust-cart-showcase/tree/main) |
| Approved audit improvements | [Follow-up PR](https://github.com/burdettadam/rust-cart-showcase/pull/2) |
| Original implementation and release | [Showcase PR #1](https://github.com/burdettadam/rust-cart-showcase/pull/1) |
| Exact exported library source | [vendor/provenance.json](vendor/provenance.json) |

## Verified baseline before the audit improvements

Showcase PR #1 merged at `8fa71a80f1c4a42dd7fa15620f18f902f5c7f801`, tree
`71779480b9d91eb0c81a36dd15bae36267054725`.
[Pages run 42703860415](https://github.com/burdettadam/rust-cart-showcase/actions/runs/42703860415)
successfully built and deployed that merge. Recorded checks included 53 native
behavior tests, one doctest, 24 JavaScript tests, eight compiled-WASM checks, and
**213/219 production Rust lines covered (97.26%)** against the 91% gate.

These are baseline measurements, not results for a later candidate. The follow-up
PR above supplies its own evidence. The original release record also separates
live browser observations from native coverage and artifact execution.

## User decisions and scope

On 2026-11-09 the user authorized merging before their personal code inspection
and requested an audit during that review. Merge authorization, AI technical
review, executed checks and completed human inspection are separate observations;
no completed manual review is claimed. The user then requested all proposed
improvements, as recorded in [PLANS.md](PLANS.md).

The challenge remains unsubmitted. This repository is the separate public
companion; its workflow does not perform the assignment's completion action.
The original library is the canonical owner of business rules. Update its reviewed
export through the maintained exporter, never by editing the vendor copy alone.

## Continue the review

Use [the README review path](README.md#review-path), the follow-up PR and the
[development commands](DEVELOPMENT.md). [PROCESS](docs/PROCESS.md) preserves the
chronology and corrections; [AI_USAGE](docs/AI_USAGE.md) records contribution and
human oversight. The [historical handoff snapshot](docs/history/HANDOFF-before-audit.md)
retains the original archive checks, source identities and access limitations.

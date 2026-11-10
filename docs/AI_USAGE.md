# AI usage and development history

## Contributions and oversight

The user directs the work, selected Rust, requested the WebAssembly showcase,
accepted the business assumptions, and retains design and acceptance judgment.
ChatGPT contributes implementation and technical advice.

AI was used for source recovery, design analysis, documentation research, Rust and
browser code, tests, build/workflow configuration, debugging, documentation and
independent review. Estimated AI contribution is **100% of the first drafts of
new implementation, tests and project documentation**. The vendored library
originates in the same AI-assisted project. User direction is not manual authorship.

The README illustration was generated with OpenAI image generation from the
documented project behavior and visual style. It is artwork, not a browser capture.

Native commits identify ChatGPT as the contributor. GitHub API imports may carry
the authorized account's identity; their `AI-Generated-By`, `Original-Author` and
`Original-Commit` trailers preserve the actual origin. Original histories remain
retained. The public replay omitted temporary Python bytecode that an early
native commit tracked and a later commit removed; reviewed final source already
excluded it.

Executed tests and two independent AI reviews provide evidence. They do not
establish completed personal code inspection or understanding. The user authorized
merges before personal inspection and later approved the documentation/navigation
cleanup and its merges. Merge authorization, user acceptance, AI review, hosted
verification, deployment and challenge submission remain distinct observations.

## Development history

| Work | Decisions, corrections and record |
| --- | --- |
| [Original showcase — PR #1](https://github.com/burdettadam/rust-cart-showcase/pull/1) | Restore the library and build a separate public companion because the challenge requires a library. Initial repository access was unavailable; browser creation and repository authorization later resolved it. |
| Initial verification | Independent review found an exact 16 KiB native reader boundary defect and browser UTF-8 BOM stripping; regressions reproduced them before the fixes. Browser QA corrected wrapping of large monetary values. A local coverage export failed on preview-sync temporary files, then passed with scoped output outside that tree. Failed attempts remain in the PR/evidence record. |
| [First audit — PR #2](https://github.com/burdettadam/rust-cart-showcase/pull/2) | Correct canonical exact-price parsing for long insignificant-zero suffixes, simplify HTTP parsing, remove unused browser state, add source links and enforce clean release metadata. The preliminary public candidate failed the new WASM regression until the user explicitly approved its corrected core export. The PR records subsequent checks, merge and deployment separately. |
| [Documentation and navigation cleanup — PR #3](https://github.com/burdettadam/rust-cart-showcase/pull/3) | The user approved one README entry point, one architecture/Q&A document, one development guide, this concise history/disclosure, and simpler demo navigation. The PR records the actual candidate, checks, reviews and merge outcome. |
| [Review-flow refinement — PR #4](https://github.com/burdettadam/rust-cart-showcase/pull/4) | Keep the cart and its feedback together after scenarios, preserve manual Add focus and expose errors through the existing mobile cart link. Retire stale plan instructions and repeated documentation; distinguish public export consistency from fidelity checks requiring private history. The user authorized implementation and merges. |

The [earlier process record in Git](https://github.com/burdettadam/rust-cart-showcase/blob/666dfdf8f5b443f8227a1fc6675e826d064af603/docs/PROCESS.md)
preserves the longer chronology. Git and actual PRs retain superseded drafts and
handoff snapshots without keeping them in the current reading path.
[CI and Pages runs](https://github.com/burdettadam/rust-cart-showcase/actions/workflows/pages.yml)
identify hosted outcomes; source checkpoints and local checks alone do not
establish a deployed release.

The companion's build and deployment do not perform the challenge completion
action. Final submission still requires a separate explicit user instruction.

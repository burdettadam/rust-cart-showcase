# AI usage and human oversight

The user requested and directs this work. ChatGPT acts as the implementation and
technical-advice contributor. The user selected Rust, required evidence and
frequent Git checkpoints, retained design/merge judgment, and requested a
WebAssembly showcase on GitHub Pages.

AI was used for source recovery, design analysis, official-documentation research,
Rust portability and browser adapter code, HTML/CSS/JavaScript, tests, build and
workflow configuration, documentation, debugging and independent review.
Estimated AI contribution is **100% of the first drafts of new implementation,
tests and project documentation**. Vendored library code originates in the same
AI-assisted project and retains its own disclosure. This estimate does not turn
user direction into a claim of manual authorship.

The 2026-11-09 GitHub integration imports the saved source onto the new companion
repository's actual base. GitHub API commits may carry the authorized account's
identity; import messages identify the AI-generated work, original source author
and source SHA. Original histories remain in the saved archives. Public replay
omits transient historical Python bytecode that was already removed from the
reviewed final source. Neither import mechanics nor human review changes the
origin of generated implementation.

Review uses executable native tests, measured Rust coverage, actual compiled-WASM
checks, browser transport/session tests, artifact checks and two independent AI
reviews with distinct assignments. [HANDOFF](../HANDOFF.md) is the maintained
entry point for actual candidate, browser, hosted verification and release
records. [PROCESS](PROCESS.md) preserves the chronology and corrections.

On 2026-11-09 the user explicitly authorized merging the baseline before personal
code inspection, requested an audit, and subsequently approved its improvements.
The follow-up work includes navigation, release-metadata validation, removal of
unused session state, and documentation consolidation. These decisions do not
establish that manual code inspection or understanding is complete. User review,
acceptance, merge approval and final challenge submission remain distinct from
AI checks and reviews.

The companion is separate from the challenge's library-only submission. No
assignment completion action is performed by the build or deployment workflow.

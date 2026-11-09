# git-agent adoption

Actual initializer: `ElevenID/git-agent` version **0.2.1**.
Source commit: `2ead37bfe9bf28489816bfdd4deeea137e23fa37`.
Executable Git blob: `f826be9fe3ce254fbe9ae983d3c27d41a2a7956e`.
Template-set digest: `git-blob-sha1:423d14af1e8a523db02e9e4ee7e7428a0a4a5c69`.
[Source](https://github.com/ElevenID/git-agent/tree/2ead37bfe9bf28489816bfdd4deeea137e23fa37).

The initializer was fetched through the connected GitHub reader, pinned bytes
verified, then executed in dry-run and real-init modes. It generated AGENTS,
GOVERNANCE, DEVELOPMENT, ARCHITECTURE, PLANS, two native Codex reviewer roles and
the repository-local work-goal and worktree-flow skills. These skills are project
configuration; no personal/global skill installation was performed. Upstream
license is retained in `docs/git-agent-LICENSE.txt`.

The user explicitly requested this workflow. Root governance, development,
architecture and plans are adapted to the accepted Rust-library scope. The
upstream skills and role configuration remain traceable to the source above.
Models and reasoning effort inherit host settings. Two independent reviews and
three maximum rounds are retained. The contract reviewer also checks regressions
against the original assignment. Human decisions and AI work stay separate.

## Actual status versus intended operation

- Actual initialization and local Git commits can be inspected in this project.
- The project TOML was validated against the current official Codex JSON Schema
  with zero schema errors; both roles match the documented standalone-agent format.
  The adversarial-role comment was clarified to reflect current model precedence.
  Native loading/trust is still unverified: a strict local client check stopped on
  a pre-existing global `features.experimental_use_rmcp_client` setting. No global
  settings were changed. Schema validity is not proof of effective activation.
- The initializer reported Git LFS availability on the host; this repository
  contains no LFS-tracked source and requires no LFS objects.
- GitHub access/protections/hosted PRs and CI are not established by these files.
- Runtime deployments, data migration, production rollback and browser isolation
  do not apply to this in-memory library. Tests own temporary local resources.
- Remote source checkpoints and clean Rust restore verification remain explicit
  acceptance work until demonstrated. A saved project archive is not a hosted PR.

Follow `GOVERNANCE.md`, `DEVELOPMENT.md`, `PLANS.md` and `HANDOFF.md` for adopted
commands, ownership, acceptance gates, execution results and pending work.

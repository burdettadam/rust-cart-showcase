# Agent operating instructions

Use `$work-goal` for accepted feature, defect, maintenance, or retirement work.
Use `$worktree-flow` for assigned worktree execution and recovery. These skills
invoke the repository's existing tools; they do not require `git-agent` after
initialization.

## Read the applicable authority

Paths below are relative to the repository root. Read only the relevant sections
and the assigned plan; keep this file a short entry point.

| Responsibility | Maintained authority |
| --- | --- |
| Decisions, review, capabilities, release approval | `GOVERNANCE.md` |
| Setup, commands, isolation, CI, recovery | `DEVELOPMENT.md` |
| Boundaries, public contracts, compatibility | `ARCHITECTURE.md` |
| Accepted scope, goal and branch records | `PLANS.md` |
| Initial adoption and outstanding setup | `docs/agent-init/README.md` |

## Operating rules

- Read the accepted plan and governing revision before executing a work slice.
  Record missing required setup as work; scaffolding alone proves no capability.
- Apply current user authorization and higher-priority execution rules. Continue
  ordinary authorized work. Escalate only the concrete decision outside your
  delegated authority, explaining the applicable rule and consequences.
- Identify applicable global, project, and nested instructions and pending rule
  changes. Evaluate proposed changes to controls under the accepted baseline.
- Keep one writer per branch/worktree. Coordinate shared Git administration
  through its assigned owner; integrate through the goal's integration owner.
- Use maintained native commands and scoped development resources. Check actual
  permissions; prose and separate checkouts do not establish isolation.
- Verify the combined fixed candidate, preserve real evidence, and complete the
  configured independent reviews. Changed relevant inputs reopen their checks.
- Keep generated evidence, logs, screenshots, and build artifacts out of Git.
  Link relevant PR and CI results while work is open; remove local copies after
  the work merges or is cancelled or superseded, after required records are
  saved. Keep code comments sparse: put change rationale in the PR so commit
  history and Git blame expose it later.
- Obtain human review before every version-producing merge, including hotfixes.
- Report verification, review, approval, deployment health, remote recovery
  coverage, local-only work, and the next action as separate observations.

Template revision: `0.2.1`. Adoption details and source references
are in `docs/agent-init/README.md`.

---
name: work-goal
description: Execute or resume accepted feature, defect, maintenance, incident, backport, or retirement plans through tracked goal work, isolated execution, evidence, independent reviews, and recoverable handoff. Use for implementing planned work or reporting its real state; do not invent acceptance or release authority.
---

# Work an accepted goal

## Establish the contract

1. Resolve the repository and assigned goal. Read its accepted plan/revision,
   criteria, quality limits, intended-use validation, work records, and relevant
   architecture. Root-relative guidance is `GOVERNANCE.md`,
   `DEVELOPMENT.md`, `ARCHITECTURE.md`, and `PLANS.md`.
2. Identify actual global/project/nested instructions, accepted governing baseline,
   selected risk/review policy, effective role/model/permissions where observable,
   and pending control changes. Review control changes under the prior accepted
   rules; never let a candidate weaken the conditions for its own acceptance.
3. Resolve integration and branch writers, repository administration ownership,
   dependencies, stale/blocked work, exceptions, capacity, and recovery gaps.
   Reconcile an uncertain old writer before takeover.
4. Select the next ready authorized slice and its affected products/evidence.
   Refine steps within scope. If required intent, authority, or setup is missing,
   prepare the concrete proposal/gap and continue independent authorized work;
   do not invent criteria or treat unresolved requirements as passed.

## Execute and integrate

5. Assign/resume branches and native stack roles. Use `$worktree-flow` with the
   actual maintained product commands and reconstructable inputs. A goal can use
   parallel, dependent, or alternative branches; name exact included revisions.
   Before implementing, search the affected code, interfaces, adapters, and known
   dependencies for equivalent behavior. Identify the canonical implementation
   and record why this slice reuses, extends, replaces, or adds one. Keep one
   source of truth for shared behavior; do not extract an abstraction solely
   because unrelated code happens to look alike. Keep source comments to the
   minimum needed to explain non-obvious contracts or behavior; put the change
   rationale and investigation in the PR for Git history and blame.
6. Update each branch's own progress, decisions, blockers, evidence, and next
   action. Let the integration owner maintain canonical goal state. Integrate
   small accepted increments early, refresh affected dependents, and preserve
   the target's applicable gates.
7. Assemble the combined candidate S with fixed source/base/configuration and
   required inputs. Hold them stable during acceptance checks. Verify selected
   required checks and intended-use validation. Treat live-edit results as
   exploratory; do not claim HEAD identifies a dirty served application.

## Verify, review, and retain

8. Read [review-guide.md](references/review-guide.md). Obtain the policy's N
   independent reviews using native reviewer roles and distinct assignments.
   Supply the fixed candidate, accepted criteria/baseline, guides and raw evidence.
   Collect initial findings independently before reconciling them.
9. Resolve findings with evidence; keep valid blockers despite majority opinion.
   Missing reviews do not count. A fix creates a new candidate; renew applicable
   verification and current review coverage. Stop at R rounds with
   `needs-decision` if blockers remain. Do not silently quarantine criteria.
10. Checkpoint meaningful increments and before handoff/cleanup. Follow the
    worktree-flow recovery reference: push source and goal records, and link
    reviewable PR/CI evidence without committing generated evidence or artifacts.
    Confirm remote revisions and state any local-only gaps explicitly.

## Release, close, and report

11. When release or operational work is assigned, read
    [release-lifecycle.md](references/release-lifecycle.md) and the existing product
    guide. Prepare current human review before any version-producing merge, then
    follow only the authorized native release path. Record actual artifact,
    health and owner; return incidents/maintenance to accepted plans.
12. Use [state-report.md](references/state-report.md) to return a human-friendly
    state derived from maintained records. Preserve cancelled/superseded reasons,
    checkpoints and replacement links. Never confuse ready, human-approved,
    published, healthy, or remotely recoverable.
13. After work merges or reaches terminal cancellation/supersession, remove its
    local generated evidence and artifacts, including obsolete worktree copies.
    Preserve the outcome decision, merged source when applicable, and PR/CI
    links first. Do not delete another owner's files or a separately required
    release/operational record.

Use Git, OS, VS Code, Codex and existing product commands directly. Do not create
a scheduler, execution adapter, state database, third skill, or new command
protocol to perform this loop.

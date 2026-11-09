# Isolation, owned resources, and restoration

## Before execution

Check the assignment, actual branch/worktree, accepted base, current writer,
governing revision and product recipe. Record actual capability restrictions
without credentials. Use the existing safe check demonstrating permitted access
and a harmless forbidden operation. Do not try a destructive operation to test
a permission boundary.

Scope endpoint/callback, database/schema/fixture, queues/consumers, mutable caches,
generated/temp output, browser context/test users, and owned processes/containers.
Scope applicable email, webhooks, storage, identity and payment effects to
development. Share only appropriate read-only dependencies/safe caches; serialize
tests that share mutable state. Different localhost ports do not isolate cookies.

Observe real startup errors/collisions and bounded readiness. Confirm the endpoint
belongs to the launched instance, not an existing process responding on that
port. Retain native process/session handles and enough local context to reconcile
interruption. A PID alone can be reused; verify ownership before stopping it.
Respect actual instance/worker limits independently of model-thread capacity.

Demonstrate two instances and interrupted startup/cleanup on the product before
claiming isolation. If resources overlap, stop the dependent run and prepare the
specific product configuration change; do not simply label the instance isolated.

## Shared Git and handoff

Use native worktree discovery and preserve same-branch safeguards. Coordinate
shared configuration, destructive refs and shared cleanup with the repository
administration owner; ordinary assigned-branch work stays with its writer.
Stashes use shared refs: prefer checkpoint commits, otherwise record the exact
stash object and owner. Never pop an unqualified latest stash.

Handoff names old/new ownership, exact checkpoint, owned resources and next
action. Reconcile whether an old writer still runs before replacing it. Do not
resolve uncertain distributed ownership by competing pushes or timestamps.
Simultaneous autonomous reassignment across computers is outside this baseline.

## Publish a recoverable checkpoint

1. Update the branch work record and consequential decisions.
2. Commit intentional source and concise work records only; keep generated
   evidence and artifacts ignored and outside Git. Preserve unrelated work.
3. Link available PR/CI results and identify local-only review material.
4. Push the intended authorized work/checkpoint ref and verify that its remote
   revision reaches the intended commit. Record confirmation time and remainder.
5. Publish every required source/component branch before the coordinating goal
   update; identify PR/CI runs used to review it. Confirm the update too.
6. Report coverage for the canonical record and all included components against
   declared retention and RPO/RTO targets. One successful branch push does not
   establish whole-goal recovery.

Use milestones, pause/handoff and pre-cleanup events. While active, report latest
confirmation age and unpushed work. These instructions offer no unattended timer
when Codex is inactive. Retain refs reaching required checkpoints before branch
deletion/rewrite. Retrieve indispensable artifacts; saved URLs alone are not proof.
Review publication for credentials/sensitive data. Hosted logs may expire; do
not claim durable recovery from a link alone.

## Close an assigned worktree safely

Immediately before removal, use Git to confirm the target path, branch, current
head, assigned owner, and clean status. Confirm that the exact work is integrated
  or intentionally abandoned and that required source refs and PR decisions are
  retained. A branch name or stale earlier audit does not prove integration; a
squash merge needs evidence tied to its exact reviewed head.

Inspect and close one assigned worktree at a time, from outside that worktree.
Never force-remove it, delete its files directly, or delete its branch before
worktree removal succeeds. If ownership, integration, cleanliness, or retention
is uncertain, keep the worktree and report the blocker and next action.

## Resume after workstation loss

Assume the declared surviving GitHub/account-access scope, then actually:

1. Restore independently recoverable account access through the secure procedure.
2. Clone/fetch product and required pinned private guidance; discover retained
   goal/work/checkpoint refs instead of inspecting main alone.
3. Read the canonical and branch records; reconcile old ownership and recreate
   assigned worktrees at the required revisions.
4. Restore documented toolchains, fixtures, nonsecret machine config, applicable
   hooks/submodules/LFS and retained artifacts. Retrieve indispensable content.
5. Verify actual Codex instruction/skill/role discovery, trust and permissions,
   and native GitHub repository/release settings. Files alone prove none of them.
6. Start fresh scoped instances with existing commands, run applicable checks,
   and resume the recorded next action without the old conversation or initializer.
7. Measure retained/lost work and time to useful resumption against product
   targets. Mark unknowable real-loss remainder unknown. Record uncovered setup.

Run controlled restoration drills after material setup/access/dependency/recovery
changes and selected reconsideration events. Worktrees, ports, caches and
processes are reconstructed, not backed up. Production-data restore and provider
or account loss require their own declared scope and evidence.

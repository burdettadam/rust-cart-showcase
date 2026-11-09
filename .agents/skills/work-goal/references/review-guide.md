# Independent review and evidence

Read the accepted governing policy and assigned product/stack guides. Review
the specified candidate and relevant operating conditions, not an unspecified
moving branch. These two generic guides can be extended through accepted policy.

## Contract reviewer

Check accepted behavior and quality limits against independent expected results.
Trace real public/user paths, boundary values, failure handling, access decisions,
and applicable version/format compatibility. Verify that tests exercise the
claimed code and make meaningful assertions. Compare affected paths for drift in
duplicated rules or validation. Seek a concrete counterexample.

For a bug, reproduce before the fix where practical. Scrutinize reduced assertions,
snapshot replacement, skipped checks, changed expected values or fixtures that
hide failure, and expected results copied only from the implementation under
review. A changed test needs a reason and evidence independent of the new code.
A plausible demo is insufficient evidence for a missing acceptance assertion.

## Adversarial reviewer

Inspect module boundaries, integration across included branches and transitive
consumers, actual capabilities, mutable resources, and applicable older clients.
Search for equivalent behavior in affected interfaces, adapters, and known
dependencies; challenge a new or duplicate implementation without a clear
canonical owner and rationale. Review architectural smells such as conflicting
sources of truth, misplaced responsibilities, coupling across boundaries, cycles,
and abstractions that add indirection without shared behavior. Tie each finding
to a concrete maintenance, correctness, or compatibility consequence. Repeated
syntax alone is not a reason to demand consolidation.
Challenge stale reviews, empty or omitted check selection, dirty-source claims,
partial checkpoints and self-approved rule changes. Check applicable upgrade,
interruption, recovery, deployment-health and support consequences.

Use the actual scope/risk profile. Distinguish a demonstrated problem from a
hypothesis or an unrelated design preference. Supply enough evidence to reproduce
or decide a finding. Do not create hypothetical gates outside accepted scope.

## Required evidence

Record or link: candidate S and base, relevant inputs/toolchain/config, command
and working directory, environment/resource scope, selected checks/assertions,
actual outcome/exit status, time, and PR/CI run identity where relevant.
Record dirty/unsupported conditions and unverified areas. Put review findings
and change rationale in the PR. Keep generated reports, logs, screenshots and
other evidence outside Git in ignored local paths while review is open; link
hosted check results when available. Review any shared output for secrets.

Expected results must have a basis independent of the same implementation.
Calibrate material model/guide/tool changes against the known-good/bad cases in
governance. Tests and model agreement are observations, not automatic proof.

Bind the review to exact source S, base and PR head. Do not create a report
commit solely to store evidence. Explain any report-only changes in the PR;
renew applicable checks when relevant code, configuration, tests, base or
release inputs change. Review must still cover the current candidate.

Use bounded retries and retain failed attempts until disposition in the PR or
check history. Flaky/quarantined checks require a
recorded reason, scope, owner, expiry/review trigger, and replacement coverage
under accepted authority. Rerunning until green or silently dropping a required
criterion does not satisfy it. Missing, unexpectedly skipped, cancelled or empty
selected checks stay visible.

## Review result

Return candidate/base identity, accepted guide/revision, inspected scope and
evidence, findings, uncertainty, and unverified scope. Each finding has a location,
impact, observed evidence/reproduction, and required disposition. Preserve the
independent first result, then record reconciliation and fixes.

After merge or terminal cancellation/supersession, clean local generated evidence and artifacts; retain the concise
PR decision, review discussion and hosted check links. Reviewers do not write
the implementation or grant human approval. Reuse valid native evidence and
run targeted checks only in authorized scoped environments.
If execution is unavailable, state that limit; never fabricate success. Limited
thread capacity permits sequential independent reviews, not a reduced N.

# Release, operation, and corrective work

Use this sequence with the product's existing release/operations guide. Obtain
authority for actual deployment effects; a planned release does not grant every
credential. Every new version-producing merge requires prior human review.

## Before human review

Identify intended product/version, target base, candidate PR and prospective
integration result S. Complete its required checks, independent reviews and
applicable intended-use validation. Present findings/dispositions and material
decisions to the actual designated human.

For stateful or compatibility-sensitive work, identify supported starting
versions/data states, target config/schema, deployment order, mixed-version
window, native migration steps, interruption/resumption, health/abort criteria,
and rollback/roll-forward with data consequences and authority. Rehearse the
material transition with representative existing data before human review.
Fresh-database success does not prove an upgrade. Explicitly decide irreversible
steps and verify recovery preparation; an old binary does not reverse data.

## Approved merge and exact artifact

Require the actual current human decision before merging. Refresh applicable
agent/human review and checks when changed inputs or target movement alter the
integration result. Verify the account/rules arrangement described in governance.

After merge, record actual source M. Compare reviewed product/release inputs with
M; merge or squash can change SHA without changing those contents. If relevant
inputs differ, stop release and prepare a fresh reviewable correction/promotion
candidate. Post-merge approval cannot repair missing premerge review.

Verify M; build artifact D from its recorded inputs; run required artifact and
transition checks before promotion. Record the version demo with its assertions,
results and digest bound to D. Bind approved PR, M, version/tag, D and evidence.
Tag exact M and promote that same D using the authorized native workflow. Never
resolve a moving branch later or substitute an unverified rebuild.

Protect version tags and release controls through verified applicable GitHub
settings. Keep minimum job permissions. A GITHUB_TOKEN tag push does not trigger
a second push workflow automatically; use one explicit workflow/reusable chain
or supported explicit dispatch. Do not create a passing placeholder workflow.

## Observe and recover

Record environment, deployed D, actual relevant configuration/schema, observed
health and time/window, operational owner, and incomplete rollout/validation.
Published, attempted, healthy, failed, partial and unverified remain distinct.
Use existing hosting health/logging/recovery facilities. Prove a controlled bad
setting is detected and recoverable when relevant to the product.

Handle interrupted deployments through the native resume/recovery procedure.
Never move a tag to conceal failure. Restoring a previously approved D may follow
explicitly delegated incident authority; a fix producing a new merge still needs
human review. Keep live-data backups/restoration as separately covered evidence.

## Maintenance, support, and retirement

Return incidents, recurring defects, vulnerabilities, dependency changes and
lessons into accepted plans with owners and relevant checks. Record actual
supported versions, intake/triage, patch/backport rules, and end-of-support
triggers. Disposition an issue across applicable support lines; do not assume
older lines are supported or abandoned. Use root-cause findings to improve tests,
architecture or guidance through normal adoption.

For retirement, cover applicable notices/migrations, retained/exported/disposed
data, credentials/endpoints, and retained release evidence. Temporary flags,
deprecated code and material debt have an owner and removal/review condition.
Create only records applicable to the actual product.

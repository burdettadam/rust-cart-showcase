# Human-friendly state

Derive this report from actual plan, branch, native check and remote observations.
Omit inapplicable detail with a reason; identify gaps as unconfigured/unverified.
Use short prose and one table where useful. Link maintained baselines instead of
duplicating them. Never use a percentage to hide unmet gates.

```markdown
## <goal>: <intended outcome>

Status and completed work:
Accepted plan/governing revision; pending control/scope changes:
Applicable quality, validation and risk/review policy:
Integration owner and candidate S/base:
Branches: writer, included revision, dependencies, current next action:
Live instances: actual scopes/ownership or stopped:

Verification: criteria/checks passed, failed, missing; exact evidence:
Validation: observed intended use or remaining validation:
Retries/flaky checks and authorized disposition:
Reviews: completed out of N; round out of R; current-candidate coverage:
Blocking findings, uncertainty, exceptions/debt and expiry:
CI: affected products/checks, rationale, actual cost/feedback observations:

Release: intended version, actual human review or pending:
Actual M, artifact D, tag, demo/assertions where available:
Upgrade/recovery rehearsal and remaining applicability checks:
Deployment: observed state/health/time and operational owner:

Recovery: canonical goal ref plus EVERY included source branch/component:
Confirmed remote revisions/time/age and PR/CI links:
Coverage: complete for listed scope or explicit missing parts:
RPO/RTO targets and measured drill evidence or gap:
Local-only evidence while work is open; cleanup after merge:

Next action, responsible owner, and concrete decision needed if any:
```

Keep readiness, review, human approval, deployed health and recovery coverage
separate. If the last checkpoint covers only one of several branches, report
partial whole-goal recovery even when that push succeeded. If no release was
attempted, say so. Current observations do not guarantee future uninterrupted
health or unattended backups.

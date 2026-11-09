# Development, verification and recovery

## Environment

Use Rust 1.97.1 with rustfmt, Clippy, llvm-tools-preview and
wasm32-unknown-unknown. `rust-toolchain.toml` pins these inputs. The wasm-bindgen
crate and CLI must both be 0.2.130. Use Node 22+ (verification currently runs on
Node 24) and Python 3. No npm packages or JavaScript bundler are needed.

```sh
cargo install wasm-bindgen-cli --locked --version 0.2.130
cargo install cargo-llvm-cov --locked --version 0.6.21
```

Concrete build, test, coverage and static-server commands are in `README.md` and
mirrored by `.github/workflows/pages.yml`. The root workspace runs both original
library and bridge tests. Build the WASM target with `-p rust-cart-showcase`, which
keeps native HTTP disabled in the browser dependency graph. The native workspace
still tests HTTP with the library's default feature enabled.

## Writers and resources

The root agent owns integration and Git administration. Each implementation writer
uses a dedicated branch/worktree. Cargo target directories are scoped to the
working checkout; serialize quality commands sharing one target directory.
Temporary HTTP fixtures allocate loopback ports. No credentials, database or live
price API is required for deterministic tests.

The browser live mode requests the public price API using CORS. It does not send
credentials or silently fall back to sample data. Reset and mode changes abort
requests and invalidate their generation. The user-facing state stays in memory.

## Checks and source identity

Freeze an exact commit before acceptance verification. Record commit/tree,
working directory, tool versions, command, exit status, logs and raw coverage.
Production Rust line coverage must be >=91%; include bridge and vendored production
code. Report native coverage separately from compiled-WASM execution, browser QA
and deployment. Never exclude production paths to improve the result.

Run `scripts/verify_vendor.py` before builds. Vendor updates use the exporter with
an explicit full core commit and create a reviewable diff. Keep Cargo.lock real
and locked; inspect dependency changes. The manifest hashes establish consistency,
while comparison to retained Git objects establishes export fidelity.

## Git, review and publishing

Commit meaningful increments on feature branches. Topic work may be assembled on
the integration feature branch. Freeze the combined candidate and obtain contract
and adversarial reviews under `GOVERNANCE.md`; fixes renew relevant evidence.
Only actual human review supplies the merge decision for `main`.

GitHub Pages build checks run on PRs. Deployment is restricted to `main`, depends
on the successful build, uploads only `dist`, uses the `github-pages` environment,
and grants Pages-write/OIDC permissions only to the deployment job. The build
job has repository/Pages read permissions for checkout and Pages metadata. Source
repository Settings → Pages must select GitHub Actions before deployment.

Current GitHub access is recorded in `HANDOFF.md`. Do not invent repository
identity or overwrite an existing fork's history. Until access is resolved,
retain standalone source checkouts, Git bundles and evidence in the deliverable.
Generated `target`, `dist`, caches and `.evidence` are not tracked in Git.

## Recovery

Restore the delivered archive; inspect branch and clean status, source/vendor
hashes and handoff. Install pinned tools, run the maintained commands and resume
the outstanding action. Local previews and task-specific toolchain paths are
reconstructable and do not belong in the source archive. Save the completed
checkpoint before deleting any worktree or generated evidence. A saved archive
is recoverable source, not proof of remote publication or automatic backups.

Managed browser-preview synchronization can create temporary files in directories
under its shared workspace. During local QA, place Cargo output outside that
synchronized tree using a task-scoped `CARGO_TARGET_DIR` if coverage export races
with those files. The retained evidence records the actual environment. The
GitHub Actions job does not run that preview synchronizer. Do not omit production
files or alter the coverage denominator to work around an export failure.

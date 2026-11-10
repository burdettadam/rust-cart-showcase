# Development, verification and deployment

Run these commands from a Git checkout. The [README](README.md#run-locally) has
the clone-and-preview quickstart; this guide owns the complete maintenance commands.

## Toolchain

`rust-toolchain.toml` pins Rust 1.97.1, rustfmt, Clippy, llvm-tools-preview and
`wasm32-unknown-unknown`. Use Node 24 and Python 3.12. The wasm-bindgen crate and
CLI must both be 0.2.130. There are no npm packages or JavaScript bundler.

```sh
cargo install wasm-bindgen-cli --locked --version 0.2.130
cargo install cargo-llvm-cov --locked --version 0.6.21
```

## Verify a release candidate

Run the full checks from a clean, fixed commit. Record its commit/tree, working
directory, tool versions, commands, exit status, logs and raw coverage. The
[Pages workflow](.github/workflows/pages.yml) executes the hosted gates.

```sh
python3 scripts/verify_vendor.py
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace --all-targets
cargo test --locked --workspace --doc
cargo test --locked -p equal-experts-shopping-cart --no-default-features --all-targets
node --test tests/*.test.mjs
python3 -B -m unittest discover -s tests -p 'test_*.py' -v
cargo llvm-cov --locked --workspace --all-targets --ignore-filename-regex '/tests/' --fail-under-lines 91
python3 scripts/build.py
node scripts/verify-wasm.mjs
python3 scripts/verify_dist.py --release
```

Coverage includes the bridge and all production library Rust, including native
HTTP; tests and third-party dependencies are excluded. The minimum is **91%**.
Report native line coverage separately from compiled-WASM execution, browser
observations and deployment. Do not exclude production paths to improve results.

The workspace tests both the library and bridge. The build script compiles
`-p rust-cart-showcase` for WASM, keeping native HTTP out of the browser dependency
graph. Deterministic tests use local fixtures and need no credentials or live API.
Python verifier tests use only the standard library.

Release verification requires the artifact's source commit to equal the actual
checkout `HEAD`, with both the build metadata and current checkout clean. It also
checks the static files and metadata types. A successful local build does not
establish hosted deployment health.

## Preview local changes

```sh
python3 scripts/build.py
python3 scripts/verify_dist.py
python3 -m http.server 8080 --directory dist
```

Open `http://localhost:8080`. The default artifact verifier permits previews built
with changes; the page labels them `+ local changes`. Serve over HTTP, not
`file://`. A normal source ZIP lacks the Git objects required for build identity.

## Update the library export

Change business rules in the canonical library first. After reviewing that
candidate, export its exact commit:

```sh
python3 scripts/export_core.py /path/to/library FULL_40_CHARACTER_COMMIT_SHA
cargo generate-lockfile
python3 scripts/verify_vendor.py
```

Review the source diff and any lockfile changes, then run the full checks above.
Do not edit the vendor copy by hand. The manifest checks consistency; comparison
with the retained library Git objects establishes export fidelity. The exporter
includes Cargo inputs, source and tests while excluding the assignment, private
completion link, governance and review evidence.

Cargo uses the root workspace's profiles; the exported library profiles remain
byte-identical and Cargo reports that the workspace overrides them.

## Publish to GitHub Pages

1. For a new repository, select **GitHub Actions** in Settings → Pages.
   This repository already uses that publishing source.
2. Prepare a feature PR into `main`, run the gates and obtain the two independent
   reviews and user merge decision required by [GOVERNANCE](GOVERNANCE.md).
3. After an authorized merge, observe the
   [actual workflow result](https://github.com/burdettadam/rust-cart-showcase/actions/workflows/pages.yml)
   and its deployment URL.

PRs run checks without deployment. On `main`, the workflow uploads only `dist`
after a successful build, then deploys it through the `github-pages` environment.
Only the deploy job receives Pages-write/OIDC permissions; the build job uses
repository/Pages read access. Relative assets support the repository subpath.
Deployment does not perform the assignment's completion action.

## Work ownership and recovery

The root agent owns integration and Git administration. Each implementation writer
uses a dedicated branch/worktree; independent reviewers do not edit the candidate.
Commit meaningful increments on the repository's actual ancestry. Preserve the
original histories and AI attribution, and renew relevant checks after fixes.

Scope Cargo targets to the checkout and serialize commands sharing one target.
Temporary HTTP fixtures allocate loopback ports. Browser live mode uses CORS
without credentials or a silent sample-data fallback; its state stays in memory.

For recovery, clone the repository or restore its saved Git bundle/history.
Inspect branch, clean status and source/vendor identity, install the pinned tools,
and run the checks above. Preserve the completed checkpoint and PR/CI evidence
before removing a worktree or ignored `target`, `dist`, cache or `.evidence` files.
A local archive is not proof of remote publication or an unattended backup.

Managed browser-preview synchronization can create transient files in its shared
workspace. If they race with coverage export, use a task-scoped
`CARGO_TARGET_DIR` outside that tree and record the environment. GitHub Actions
does not use that synchronizer. Keep failed attempts and renew the affected
checks; never change the production coverage denominator to bypass a failure.

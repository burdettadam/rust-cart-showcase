# Development

Use Rust 1.97.1 and its rustfmt, Clippy, llvm-tools-preview and
wasm32-unknown-unknown target. Pin wasm-bindgen crate and CLI to 0.2.130.
Use Node 22 or later and Python 3 for static packaging and artifact verification.
Concrete commands and architecture will be completed with the implementation.

The root agent owns integration and Git administration. The UI author uses a
dedicated worktree; Cargo target directories are task-scoped. No server, database,
credentials or live price API is required for deterministic tests. Browser live
mode makes CORS requests to the assignment's public price API.

Work on feature branches; commit meaningful increments. Freeze exact source for
acceptance checks and two independent reviews. Human review precedes merges to
`main`. The accepted scope and external access constraint are in `PLANS.md`.
Generated output belongs in ignored `target`, `dist` or `.evidence` directories.

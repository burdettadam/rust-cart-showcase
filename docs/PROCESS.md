# Process record

## 2026-11-08: functional WebAssembly showcase

The user requested WebAssembly and GitHub Pages. The saved original project was
restored and verified against archive SHA-256
`d83aa0c6993587d768b6b3270aabb0cfbc3feba0c2a3466eb70f2d691314b35c`.
Its reviewed library candidate was `a2d4459a117696836e8591e6242107c997f80233`.
The original challenge explicitly excludes application submissions, so the
showcase is a separate companion with a pinned source export of the library.

The GitHub connection returned 404 for the inferred personal fork. Its installed
account inventory exposed only ElevenID and no personal repositories. This is
an access/identity constraint, not proof that the user's fork does not exist.
No remote repository, hosted PR, merge or Pages deployment is fabricated.

A task-specific Rust 1.97.1 toolchain, wasm32 target, llvm-tools, cargo-llvm-cov 0.6.21
and wasm-bindgen 0.2.130 were made available. Official wasm-bindgen and GitHub Pages
documentation informed the browser loading and deployment workflow. All five
public price fixtures returned 200 with CORS permission during a live research
check; deterministic tests do not rely on their ongoing availability.

The library portability branch feature-gates native HTTP and shares strict JSON
price parsing. Its cart algorithm remains byte-identical to the previous reviewed
candidate. The companion receives exact source files and a hash manifest. The
browser bridge accepts quantity and price text, preventing JavaScript numeric
conversion from weakening the core contract.

Implementation is divided between independent worktrees for core and UI, with
root-owned bridge, build, documentation and integration. Initial Python syntax
checking produced caches that were accidentally staged with an early commit;
a follow-up commit removes them and adds ignore rules. They are absent from the
final source and deployment artifact. The actual Git history is preserved.

Source is frozen for acceptance checks and separate contract and adversarial
reviews. Final commands, outcomes, source identities, findings and limitations
are retained with the delivered evidence and summarized in `HANDOFF.md`.

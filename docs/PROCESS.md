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

## Independent findings and final verification

Core adversarial review exposed a pre-existing exact 16 KiB reader boundary defect.
A regression failed on the old code, then passed with one bounded byte of EOF
lookahead. Both core round 2 reviews passed 7b251364. The companion exports that
corrected source; the cart algorithm and accepted business rules remain unchanged.

Showcase contract review found that TextDecoder removed a leading UTF-8 BOM.
Three new transport/handoff tests failed before `ignoreBOM: true`; all 24 passed
afterward. The actual WASM rejection checks also include BOM-prefixed input.
Mobile browser QA showed large amounts wrapping mid-number; a display-only change
keeps complete decimal strings readable with compact type and stacked rows.

A later coverage export failed on transient `.rsync-tmp` objects created by the
managed browser synchronization process. The behavior tests had passed, but that
coverage command correctly stayed failed. Compiler output was moved to a scoped
location outside the synchronized tree. The full renewed check set passed there.
The failed attempt remains retained; it was not silently replaced by a passing
report. This environment adjustment does not exclude any production coverage.

Actual Chrome interaction covered the sample, tax/repricing/error scenarios,
invalid quantity, a large exact decimal, mode reset and a real live price request.
A 375px content viewport also exercised the project-subpath build. These checks
establish observed behavior in that browser; they are not cross-browser testing,
human review, hosted CI or a GitHub Pages deployment.

## 2026-11-09: prepare the public GitHub review branch

The browser-created public companion is `burdettadam/rust-cart-showcase`, whose
initial `main` commit is `0ec8238570b597cee798229eaae3aab446cdb057`. The user
authorized GitHub app access to this repository and the private assignment fork.
The `feat/wasm-showcase` branch integrates the existing showcase increments onto
that real GitHub ancestry for one review PR into `main`.

GitHub API publication uses the authenticated publisher's identity. Import commit
messages retain `AI-Generated-By`, `Original-Author` and `Original-Commit` trailers;
the complete original local histories remain in the saved archives. This
attribution distinguishes publication from human authorship or approval.

One historical artifact cleanup is explicit: original commit `ca80dd2` briefly
tracked four generated `scripts/__pycache__/*.pyc` files, which `6e02f9f` removed.
The public replay omits those bytecode files from the intermediate history. The
original archive retains that history, and the final reviewed source tree was
already free of those files before the documentation reconciliation. No business
logic or tests were changed by this omission.

The archived coverage, WASM checks and browser observations remain historical
evidence. The integrated candidate needs fresh checks, independent reviews and
actual PR/CI links before a human merge decision. The existing main-only Pages
upload and deployment conditions remain unchanged. Repository creation and app
access do not establish a successful deployment; the resulting Pages URL must be
observed and reviewed separately. Publishing this companion does not submit the
coding challenge.

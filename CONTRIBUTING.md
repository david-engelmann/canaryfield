# Contributing to Canaryfield

Canaryfield is developed under the contract-first discipline documented in `AGENTS.md` and `CONTRACT.md`. Read both files in full before proposing a change.

## Core Rules

1. **Contract first** Behavioural and seed-registry changes state their falsifiable obligations in `CONTRACT.md` before implementation code is written, preserving prefix-pinned compatibility across every earlier `canaryfield-seeds/N` pack version.
2. **Discriminating tests** Every load-bearing rule requires a content-level test pin that fails when the behaviour is negated, and slices growing the registry prove zero verdict drift on earlier seeds unless stated.
3. **Sanitization** Never include real paths from a working machine, private source files, usernames, or session data anywhere in code, tests, fixtures, examples, or prose. Every fixture must be synthetic.
4. **Copy and typography** Public prose and contract text carry zero em dashes (`U+2014`), zero en dashes (`U+2013`), and zero colons or semicolons in prose sentences. Verify every prose edit with a Python codepoint scan.
5. **Pull requests and sign-off** Open a pull request from a topic branch (`main` takes no direct pushes) and include a `Signed-off-by` trailer on every commit.

## Local Verification

Run the full local gate on the exact final head of your branch (re-running after any change made during review) and verify zero formatting or lint regressions before merging.

```sh
cargo test --locked
cargo fmt --check
cargo clippy --offline --all-targets
cargo package --locked --allow-dirty
```

# Frequently Asked Questions

## Does Canaryfield ever touch my working directory or git branches?

No. Both `--plant copy` and `--plant git-branches` operate strictly inside temporary scratch directories (or `--work-root`), and Canaryfield hashes the `--target` tree before and after execution to report `"target_unchanged": true`.

## Why are some seeds marked `not-applicable`?

Each registered seed (`204` at `v0.46.0`) targets a specific Python AST construct (for example `async def` / `await`, `match` / `case` guards, `try` / `else` blocks, f-string format specs, or starred unpacking). If your target codebase contains zero occurrences of that construct, the seed cannot be planted honestly and is recorded as `not-applicable` rather than inflating or deflating catch percentages.

## What is the difference between `--plant copy` and `--plant git-branches`?

`--plant copy` writes each planted tree into a plain scratch directory for fast scoring of test suites and compilers. `--plant git-branches` initializes a scratch git clone and commits each planted seed onto its own branch so diff-aware gates (`git diff main...HEAD` linters, secret scanners, or PR review hooks) can be scored identically to real pull requests.

## How do I inspect the exact code change a `waved-through` seed made?

Pass `--keep-scratch` and `--work-root /tmp/cf-inspect`. Canaryfield leaves the planted scratch copies or scratch clone intact after printing the catch table so you can `diff -u` any surviving seed against the clean target and write the missing test or lint rule.

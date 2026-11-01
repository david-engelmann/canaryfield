# canaryfield

It plants real seeded defects into a codebase on a schedule and keeps score of which gates catch each one and which wave it through.

## What this is

A gate's catch rate is usually an assumption, because nothing continuously plants known bad changes to test the harness end to end. Canaryfield plants defects drawn from real defect classes as throwaway branches, runs them through the real CI, and records per gate which seeds were caught and which sailed past. The field covers the whole gate layer at once, tests, hooks, and review bots included. Over time it builds the bad change corpus that unique catches, overlap, and enforcement integrity all depend on. The boundary with its siblings is one sentence. Cantfail asks whether a test can fail, canaryfield asks whether the harness catches planted defects, suitediet asks which tests earn their runtime.

## Status

- The local seed is done and the seed commit sits on the local main branch
- The remote seed is still pending because a token access fix is outstanding and content writes are blocked
- The canaryfield probe returned 403 on 31 October 2026 and that result is on file with the main chat
- Setup is in progress locally on a branch and the full setup pull request flow waits on the remote fix
- The contract lives in CONTRACT.md and its obligations are met for slice 1 as the test suite shows
- Slice 1 is built and proven on synthetic targets only
- The proving run against a scratch clone of a real portfolio tree is future work and nothing has been planted in any live tree
- Closed rounds stand at 0 of 100
- AGENTS.md is pending because the canonical template has not been supplied by the main chat
- No formats are pinned by this repo yet

## Licence

MIT. See LICENSE.

# canaryfield

It plants real seeded defects into a codebase on a schedule and keeps score of which gates catch each one and which wave it through.

## What this is

A gate's catch rate is usually an assumption, because nothing continuously plants known bad changes to test the harness end to end. Canaryfield plants defects drawn from real defect classes as throwaway branches, runs them through the real CI, and records per gate which seeds were caught and which sailed past. The field covers the whole gate layer at once, tests, hooks, and review bots included. Over time it builds the bad change corpus that unique catches, overlap, and enforcement integrity all depend on. The boundary with its siblings is one sentence. Cantfail asks whether a test can fail, canaryfield asks whether the harness catches planted defects, suitediet asks which tests earn their runtime.

## Status

- The local seed is done and the seed commit sits on the local main branch
- The remote seed is still pending because a token access fix is outstanding and content writes are blocked
- The canaryfield probe returned 403 on 31 October 2026 and that result is on file with the main chat
- Setup is in progress locally on a branch and the full setup pull request flow waits on the remote fix
- The contract lives in CONTRACT.md and its obligations are met for slice 1 and slice 2 and slice 3 as the test suite shows
- Slice 1 is built and proven on synthetic targets only
- Slice 2 is built and proven on synthetic targets only
- Slice 3 is built and proven on synthetic targets only
- Slice 3 plants every seed into every file it applies to instead of only the first one, and every catch table row now names all of its planted files in a planted_files list in both reports
- Regenerating the committed examples under the wider planting changed no verdict, because the demo project carries a single plantable module that every seed already planted into, and the example JSON rows only gained their planted_files lists while the text tables and the seed pack stayed byte identical
- Slice 2 adds a git branch planting mode that plants each seed on its own throwaway branch inside a scratch clone and never touches the target tree or its git state
- Slice 2 adds a second example where two gates judge the same field and honestly disagree on ten seeds
- Slice 2 adds a versioned seed pack file under the format name canaryfield-seeds/1 so other tools in the line can consume the same seed definitions
- The owner of this line has run proving runs against scratch clones of real project trees, and their results are reported to the owner as numbers only and never committed here, and nothing has been planted in any live tree
- Round 1 closed on 2026-10-31 after slice 2 was built, audited, reviewed on two axes, and attacked by a red team, with every material finding fixed under the contract fix obligations FIX2-1 to FIX2-5 before the owner of this line approved the round
- Round 2 closed on 2026-10-31 after slice 3 was built, audited, and reviewed on two axes, with the red team rejecting the first form of the slice and every material finding fixed under the contract fix obligations FIX3-1 to FIX3-6 before the owner of this line approved the round
- Closed rounds stand at 2 of 100
- AGENTS.md is pending because the canonical template has not been supplied by the main chat
- No formats are pinned by this repo yet

## Licence

MIT. See LICENSE.

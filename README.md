# canaryfield

It plants real seeded defects into a codebase on a schedule and keeps score of which gates catch each one and which wave it through.

## What this is

A gate's catch rate is usually an assumption, because nothing continuously plants known bad changes to test the harness end to end. Canaryfield plants defects drawn from real defect classes as throwaway branches, runs them through the real CI, and records per gate which seeds were caught and which sailed past. The field covers the whole gate layer at once, tests, hooks, and review bots included. Over time it builds the bad change corpus that unique catches, overlap, and enforcement integrity all depend on. The boundary with its siblings is one sentence. Cantfail asks whether a test can fail, canaryfield asks whether the harness catches planted defects, suitediet asks which tests earn their runtime.

## Status

- The local seed is done and the seed commit sits on the local main branch
- The remote seed is still pending because a token access fix is outstanding and content writes are blocked
- The canaryfield probe returned 403 on 31 October 2026 and that result is on file with the main chat
- Setup is in progress locally on a branch and the full setup pull request flow waits on the remote fix
- The contract lives in CONTRACT.md and its obligations are met for slice 1 and slice 2 and slice 3 and slice 4 and slice 5 and slice 6 and slice 7 as the test suite shows
- Slice 1 is built and proven on synthetic targets only
- Slice 2 is built and proven on synthetic targets only
- Slice 3 is built and proven on synthetic targets only
- Slice 3 plants every seed into every file it applies to instead of only the first one, and every catch table row now names all of its planted files in a planted_files list in both reports
- Regenerating the committed examples under the wider planting changed no verdict, because the demo project carries a single plantable module that every seed already planted into, and the example JSON rows only gained their planted_files lists while the text tables and the seed pack stayed byte identical
- Slice 4 is built and proven on synthetic targets only
- Slice 4 plants every seed at every matching shape inside each file it plants into instead of only the first matching shape in that file, so a catch table row now means the gate faced the defect everywhere it could be planted in that file
- Regenerating the committed examples under the slice 4 planting changed no verdict and no byte, because the demo project carries a single plantable module, the only seeds with more than one matching shape in that module were already caught through their first planting, and the two waved through seeds each match at a single shape there
- Slice 5 is built and proven on synthetic targets only
- Slice 5 grows the seed registry from twelve seeds to sixteen, adding swapped operand order and dropped else branch and swapped boolean operator and dropped await as seed-12 through seed-15
- Slice 5 writes the seed pack under the format name canaryfield-seeds/2 with sixteen entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the sixteen seed registry changed no verdict on the original twelve seeds, and seed-12 is caught by the demo test gate while seed-13 and seed-14 and seed-15 do not apply to the demo project, so the two gates in the second example now honestly disagree on eleven seeds
- Slice 5 is the Round 4 work increment, and Round 4 closed on 2026-11-01 after audit and review and red team and owner approval, with the review findings fixed under FIX5-1
- Slice 6 is built and proven on synthetic targets only
- Slice 6 pays the debt FIX5-1 recorded and pins each seed pack version to the registry it names, so the loader refuses a pack whose ids are not exactly the ids of its format version and a pack whose entries disagree with the registry on a name or a defect class, while an entry description stays unverified prose
- Regenerating the committed examples under the loader change moved no byte, because the slice changes only what the loader accepts and the pack this tool writes is registry data
- Slice 6 is the Round 5 work increment, and Round 5 closed on 2026-11-01 after audit and review and red team and owner approval, with the review findings recorded in the Round 5 review fixes
- Slice 7 is built and proven on synthetic targets only
- Slice 7 grows the seed registry from sixteen seeds to twenty, adding widened exception handler and swapped dictionary entry and inverted truth test and reassigned loop variable as seed-16 through seed-19
- Slice 7 writes the seed pack under the format name canaryfield-seeds/3 with twenty entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the twenty seed registry changed no verdict on the original sixteen seeds, and seed-16 plants into the demo project and is waved through by both of its gates while seed-17 and seed-18 and seed-19 do not apply to the demo project, so the two gates in the second example still honestly disagree on eleven seeds
- Slice 7 is the Round 6 work increment, and Round 6 closed on 2026-11-01 after audit and review and red team and owner approval, with the review findings recorded in the Round 6 review fixes
- Slice 2 adds a git branch planting mode that plants each seed on its own throwaway branch inside a scratch clone and never touches the target tree or its git state
- Slice 2 added a second example where two gates judge the same field, an example whose disagreements now stand at eleven under the twenty seed registry
- Slice 2 added a versioned seed pack file under the format name canaryfield-seeds/1 so other tools in the line can consume the same seed definitions, and slice 5 writes the pack under canaryfield-seeds/2 and slice 7 writes the pack under canaryfield-seeds/3 while version 1 and version 2 packs still load
- The owner of this line has run proving runs against scratch clones of real project trees, and their results are reported to the owner as numbers only and never committed here, and nothing has been planted in any live tree
- Round 1 closed on 2026-10-31 after slice 2 was built, audited, reviewed on two axes, and attacked by a red team, with every material finding fixed under the contract fix obligations FIX2-1 to FIX2-5 before the owner of this line approved the round
- Round 2 closed on 2026-10-31 after slice 3 was built, audited, and reviewed on two axes, with the red team rejecting the first form of the slice and every material finding fixed under the contract fix obligations FIX3-1 to FIX3-6 before the owner of this line approved the round
- Round 3 closed on 2026-11-01 after slice 4 was built, audited, and reviewed on two axes and attacked by a red team, with every material finding fixed under the contract fix obligations FIX4-1 to FIX4-3 before the owner of this line approved the round
- Closed rounds stand at 6 of 100
- AGENTS.md is pending because the canonical template has not been supplied by the main chat
- No formats are pinned by this repo yet

## Licence

MIT. See LICENSE.

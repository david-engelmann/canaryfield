# canaryfield

It plants real seeded defects into a codebase on a schedule and keeps score of which gates catch each one and which wave it through.

## What this is

A gate's catch rate is usually an assumption, because nothing continuously plants known bad changes to test the harness end to end. Canaryfield plants defects drawn from real defect classes as throwaway branches, runs them through the real CI, and records per gate which seeds were caught and which sailed past. The field covers the whole gate layer at once, tests, hooks, and review bots included. Over time it builds the bad change corpus that unique catches, overlap, and enforcement integrity all depend on. The boundary with its siblings is one sentence. Cantfail asks whether a test can fail, canaryfield asks whether the harness catches planted defects, suitediet asks which tests earn their runtime.

## Status

- The local seed is done and the seed commit sits on the local main branch
- The remote seed is still pending because a token access fix is outstanding and content writes are blocked
- The canaryfield probe returned 403 on 31 October 2026 and that result is on file with the main chat
- Setup is in progress locally on a branch and the full setup pull request flow waits on the remote fix
- The contract lives in CONTRACT.md and its obligations are met for slice 1 and slice 2 and slice 3 and slice 4 and slice 5 and slice 6 and slice 7 and slice 8 and slice 9 and slice 10 and slice 11 and slice 12 and slice 13 and slice 14 as the test suite shows
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
- Slice 8 is built and proven on synthetic targets only
- Slice 8 grows the seed registry from twenty seeds to twenty four, adding swapped call argument order and altered string literal and dropped numeric negation and swapped loop control as seed-20 through seed-23
- Slice 8 writes the seed pack under the format name canaryfield-seeds/4 with twenty four entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the twenty four seed registry changed no verdict on the original twenty seeds, and seed-20 and seed-21 plant into the demo project and are caught by its test gate while seed-22 and seed-23 do not apply to the demo project, so the two gates in the second example now honestly disagree on thirteen seeds
- Slice 8 is the Round 7 work increment, and Round 7 closed on 2026-11-01 after audit and review and red team and owner approval, with the review findings recorded in the Round 7 review fixes
- Slice 9 is built and proven on synthetic targets only
- Slice 9 grows the seed registry from twenty four seeds to twenty eight, adding swapped augmented assignment operator and identity test replaced by equality and subscript index shifted by one and swapped keyword argument values as seed-24 through seed-27
- Slice 9 writes the seed pack under the format name canaryfield-seeds/5 with twenty eight entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and canaryfield-seeds/4 packs at their twenty four entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the twenty eight seed registry changed no verdict on the original twenty four seeds, and seed-24 and seed-25 and seed-26 and seed-27 do not apply to the demo project, so the two gates in the second example still honestly disagree on thirteen seeds
- Slice 9 is the Round 8 work increment, and Round 8 closed on 2026-11-01 after audit and review and red team and owner approval, with the review findings recorded in the Round 8 review fixes
- Slice 10 is built and proven on synthetic targets only
- Slice 10 grows the seed registry from twenty eight seeds to thirty two, adding swapped raised error and slice stop shifted by one and shifted numeric default and swapped sibling attribute as seed-28 through seed-31
- Slice 10 writes the seed pack under the format name canaryfield-seeds/6 with thirty two entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and canaryfield-seeds/4 packs at their twenty four entries and canaryfield-seeds/5 packs at their twenty eight entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the thirty two seed registry changed no verdict on the original twenty eight seeds, and seed-28 plants into the demo project and is caught by its test gate while seed-29 and seed-30 and seed-31 do not apply to the demo project, so the two gates in the second example now honestly disagree on fourteen seeds
- Slice 10 is the Round 9 work increment and is built on a local branch, and Round 9 closed on 2026-11-01 after audit and two-axis review and red team and owner approval, with the findings recorded in the Round 9 review fixes
- Slice 11 is built and proven on synthetic targets only
- Slice 11 grows the seed registry from thirty two seeds to thirty six, adding narrowed exception handler and swapped parameter defaults and dropped finally body and dropped decorator as seed-32 through seed-35
- Slice 11 writes the seed pack under the format name canaryfield-seeds/7 with thirty six entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and canaryfield-seeds/4 packs at their twenty four entries and canaryfield-seeds/5 packs at their twenty eight entries and canaryfield-seeds/6 packs at their thirty two entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the thirty six seed registry changed no verdict on the original thirty two seeds, and seed-32 and seed-33 and seed-34 and seed-35 do not apply to the demo project, so the two gates in the second example still honestly disagree on fourteen seeds
- Slice 11 is the Round 10 work increment, and Round 10 closed on 2026-11-01 after audit and two-axis review and red team and owner approval, with the findings recorded in the Round 10 review fixes
- Slice 12 is built and proven on synthetic targets only
- Slice 12 grows the seed registry from thirty six seeds to forty, adding dropped exception handler and dropped yield and dropped condition term and dropped base class as seed-36 through seed-39
- Slice 12 writes the seed pack under the format name canaryfield-seeds/8 with forty entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and canaryfield-seeds/4 packs at their twenty four entries and canaryfield-seeds/5 packs at their twenty eight entries and canaryfield-seeds/6 packs at their thirty two entries and canaryfield-seeds/7 packs at their thirty six entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the forty seed registry changed no verdict on the original thirty six seeds, and seed-36 and seed-37 and seed-38 and seed-39 do not apply to the demo project, so the two gates in the second example still honestly disagree on fourteen seeds
- Slice 12 is the Round 11 work increment, and Round 11 closed on 2026-11-01 after audit and two-axis review and red team and owner approval, with the findings recorded in the Round 11 review fixes
- Slice 13 is built and proven on synthetic targets only
- Slice 13 grows the seed registry from forty seeds to forty four, adding shifted slice start and reversed loop control and dropped scope declaration and swapped assignment values as seed-40 through seed-43
- Slice 13 writes the seed pack under the format name canaryfield-seeds/9 with forty four entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and canaryfield-seeds/4 packs at their twenty four entries and canaryfield-seeds/5 packs at their twenty eight entries and canaryfield-seeds/6 packs at their thirty two entries and canaryfield-seeds/7 packs at their thirty six entries and canaryfield-seeds/8 packs at their forty entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the forty four seed registry changed no verdict on the original forty seeds, and seed-40 and seed-41 and seed-42 and seed-43 do not apply to the demo project, so the two gates in the second example still honestly disagree on fourteen seeds
- Slice 13 is the Round 12 work increment, and Round 12 closed on 2026-11-01 after audit and two-axis review and red team and owner approval, with the findings recorded in the Round 12 review fixes
- Slice 14 is built and proven on synthetic targets only
- Slice 14 grows the seed registry from forty four seeds to forty eight, adding dropped loop else clause and dropped keyword argument and flipped range step sign and dropped logical negation as seed-44 through seed-47
- Slice 14 writes the seed pack under the format name canaryfield-seeds/10 with forty eight entries, and the loader still accepts canaryfield-seeds/1 packs at their twelve entries and canaryfield-seeds/2 packs at their sixteen entries and canaryfield-seeds/3 packs at their twenty entries and canaryfield-seeds/4 packs at their twenty four entries and canaryfield-seeds/5 packs at their twenty eight entries and canaryfield-seeds/6 packs at their thirty two entries and canaryfield-seeds/7 packs at their thirty six entries and canaryfield-seeds/8 packs at their forty entries and canaryfield-seeds/9 packs at their forty four entries and refuses any pack whose entry count disagrees with its format version
- Regenerating the committed examples under the forty eight seed registry changed no verdict on the original forty four seeds, and seed-44 and seed-45 and seed-46 and seed-47 do not apply to the demo project, so the two gates in the second example still honestly disagree on fourteen seeds
- Slice 14 is the Round 13 work increment and the round closed after audit and two axis review and red team and owner approval, with the review fixes recorded in CONTRACT.md
- Slice 2 adds a git branch planting mode that plants each seed on its own throwaway branch inside a scratch clone and never touches the target tree or its git state
- Slice 2 added a second example where two gates judge the same field, an example whose disagreements now stand at fourteen under the forty eight seed registry
- Slice 2 added a versioned seed pack file under the format name canaryfield-seeds/1 so other tools in the line can consume the same seed definitions, and slice 5 writes the pack under canaryfield-seeds/2 and slice 7 writes the pack under canaryfield-seeds/3 and slice 8 writes the pack under canaryfield-seeds/4 and slice 9 writes the pack under canaryfield-seeds/5 and slice 10 writes the pack under canaryfield-seeds/6 and slice 11 writes the pack under canaryfield-seeds/7 and slice 12 writes the pack under canaryfield-seeds/8 and slice 13 writes the pack under canaryfield-seeds/9 and slice 14 writes the pack under canaryfield-seeds/10 while version 1 and version 2 and version 3 and version 4 and version 5 and version 6 and version 7 and version 8 and version 9 packs still load
- The owner of this line has run proving runs against scratch clones of real project trees, and their results are reported to the owner as numbers only and never committed here, and nothing has been planted in any live tree
- Round 1 closed on 2026-10-31 after slice 2 was built, audited, reviewed on two axes, and attacked by a red team, with every material finding fixed under the contract fix obligations FIX2-1 to FIX2-5 before the owner of this line approved the round
- Round 2 closed on 2026-10-31 after slice 3 was built, audited, and reviewed on two axes, with the red team rejecting the first form of the slice and every material finding fixed under the contract fix obligations FIX3-1 to FIX3-6 before the owner of this line approved the round
- Round 3 closed on 2026-11-01 after slice 4 was built, audited, and reviewed on two axes and attacked by a red team, with every material finding fixed under the contract fix obligations FIX4-1 to FIX4-3 before the owner of this line approved the round
- Closed rounds stand at 13 of 100
- AGENTS.md stands in this repo, the canonical template of the program with this repo's annex below its marked line
- No formats are pinned by this repo yet

## Licence

MIT. See LICENSE.

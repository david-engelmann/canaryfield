# canaryfield

It plants real seeded defects into a codebase on a schedule and keeps score of which gates catch each one and which wave it through.

## What this is

A gate's catch rate is usually an assumption, because nothing continuously plants known bad changes to test the harness end to end. Canaryfield plants defects drawn from real defect classes as throwaway branches, runs them through the real CI, and records per gate which seeds were caught and which sailed past. The field covers the whole gate layer at once, tests, hooks, and review bots included. Over time it builds the bad change corpus that unique catches, overlap, and enforcement integrity all depend on. The boundary with its siblings is one sentence. Cantfail asks whether a test can fail, canaryfield asks whether the harness catches planted defects, suitediet asks which tests earn their runtime.

## Status

Seeded in October 2026. The build has not started. Part of the impatient programming portfolio, in the Gate Truth arc.

## Licence

MIT. See LICENSE.

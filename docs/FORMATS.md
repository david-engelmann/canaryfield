# Canaryfield Wire Formats

Canaryfield produces and consumes two structured JSON formats alongside its human-readable catch table, the versioned `canaryfield-seeds/N` seed pack and the `catch-table.json` scoring report.

## 1. Versioned Seed Pack (`--write-seed-pack`)

- `format` Versioned string `"canaryfield-seeds/1"` through `"canaryfield-seeds/49"` (and subsequent slices). Each version `N` pins an exact prefix length of the canonical seed registry (`12` entries at v1, `204` entries at v49).
- `seeds` Ordered array of seed objects carrying
  - `id` Canonical identifier (`"seed-00"`, `"seed-01"`, ..., `"seed-203"`).
  - `name` Canonical defect name (such as `"widened exception handler"` or `"dropped call argument"`).
  - `defect_class` Defect taxonomy label (such as `"hidden failure"`, `"missing input"`, `"wrong predicate"`).
  - `description` Human-readable description of the AST transformation.

## 2. Catch Table JSON (`--json` and `<out>/catch-table.json`)

Sorted-key, 2-space-indented JSON object carrying

- `clean_exits` Object mapping each gate name to its integer exit code on the unplanted target (`-1` on baseline timeout or spawn error).
- `gates` Ordered array of gate names as supplied on the command line.
- `plant_mode` Optional string set to `"git-branches"` when `--plant git-branches` is used (omitted under the default `copy` mode).
- `rows` Array of per-seed scoring objects carrying
  - `branch` Optional string (`"canaryfield/<seed_id>"`) or `null` when `"not-applicable"`, present only under `--plant git-branches`.
  - `defect_class` Seed defect class string.
  - `gates` Object mapping each gate name to `"caught"`, `"waved-through"`, `"timed-out"`, `"not-applicable"`, or `"gate-broken"`.
  - `name` Seed name.
  - `planted_exits` Object mapping each evaluated clean gate (`clean_exits[gate] == 0`) to its integer exit code on the planted tree (`-1` when `"timed-out"`), omitting gates skipped under `"not-applicable"` or `"gate-broken"`.
  - `planted_file` First relative file path planted into, or `null` when `"not-applicable"`.
  - `planted_files` Array of all relative file paths planted into for that seed.
  - `seed` Seed identifier (`"seed-00"` .. `"seed-203"`).
- `summary` Object mapping each gate name to `{ "caught", "gate-broken", "not-applicable", "waved-through" }` integer totals (`"timed-out"` cells are intentionally excluded from `"caught"`).
- `target` Target path string as passed to `--target`.
- `target_unchanged` Boolean proving the live `--target` tree did not change by a single byte during the run.
- `unparseable_files` Array of relative paths skipped due to syntax errors.

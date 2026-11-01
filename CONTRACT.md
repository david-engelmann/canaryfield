# canaryfield contract

This contract is the executable spec for canaryfield. Every obligation below maps to at least one named test in the test suite. An obligation is met only when its tests pass.

## Setup obligations

- SETUP-1 The seed files stay intact. README.md keeps the seed description and licence sections, LICENSE keeps the MIT text, and .gitignore keeps the seed rules. Proven by test_seed_files_present.
- SETUP-2 The README status section tells the truth about the build state. It states the local seed, the blocked remote seed, the contract state, the slice state, the closed round count, and the pending AGENTS.md. Proven by test_readme_status_is_honest.
- SETUP-3 Continuous integration runs the same command as the local gate. The workflow runs python -m pytest on push and on pull requests. Proven by test_ci_runs_the_local_gate.
- SETUP-4 No AGENTS.md exists in this repo until the canonical template is supplied by the main chat. Proven by test_no_agents_md_yet.
- SETUP-5 Every fixture and example in this repo is synthetic and written for this repo. No real traces, real names, or private detail appear anywhere. Proven by the sanitization self scan reported with every change.

## Slice obligations

- SLICE-1 The seed registry holds exactly twelve seeds with unique stable ids. Seed zero has id seed-00 and plants the stale version string 9.9.9. Proven by test_registry_has_twelve_unique_seeds and test_seed_zero_is_the_stale_version_string.
- SLICE-2 Every seed carries a name, a defect class, and an applicability check. A seed that does not apply to a target is recorded as not applicable and is never counted as caught or waved through. Proven by test_seed_metadata_is_complete and test_inapplicable_seed_is_recorded_not_applicable.
- SLICE-3 Planting happens only in scratch copies. The target tree is byte identical after any run, proven by a hash walk taken before and after. Proven by test_target_tree_is_byte_identical_after_a_run.
- SLICE-4 Gates are named commands supplied by the operator. Every gate runs once against a clean scratch copy and once against each planted scratch copy, each run under a timeout. Proven by test_gate_runs_on_clean_and_planted_copies.
- SLICE-5 Catch scoring follows one rule. A seed is caught for a gate when the gate exits zero on the clean copy and non-zero on the planted copy. A seed is waved through when the gate exits zero on both copies. Proven by test_caught_and_waved_scoring.
- SLICE-6 A gate that fails on the clean copy is reported as gate broken for every seed. A broken gate is never reported as catching anything. Proven by test_gate_failing_on_clean_copy_is_broken.
- SLICE-7 The catch table is the report. It carries one row per seed and one column per gate, with each cell holding exactly one of caught, waved through, not applicable, or gate broken. The same facts render as plain text and as JSON. Proven by test_catch_table_shape_and_statuses and test_report_renders_text_and_json_with_the_same_facts.
- SLICE-8 Slice 1 proves the machinery on synthetic targets only. Nothing is planted in a live tree, and the scratch clone proving run against a real portfolio tree is future work, stated as such in the README. Proven by test_example_target_is_synthetic and by the README honesty test in SETUP-2.

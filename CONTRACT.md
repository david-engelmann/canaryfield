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

## Slice 2 obligations

- SLICE2-1 A git branch planting mode exists beside the copy mode. In git mode the tool clones the target into a scratch clone under the work root and plants only inside that clone. The target itself is never checked out, committed to, or written to. Proven by test_git_mode_target_git_state_untouched.
- SLICE2-2 In git mode every applicable seed is committed on its own throwaway branch in the scratch clone. The branch name is canaryfield/seed- plus the seed id. Gates run in a checkout of that branch. A seed that does not apply gets no branch and stays recorded as not applicable. Proven by test_git_mode_plants_each_seed_on_its_own_branch and test_git_mode_inapplicable_seed_gets_no_branch.
- SLICE2-3 Git mode scoring follows the same rule as copy mode. A seed is caught when the gate exits zero on the clean checkout and non-zero on the seed branch checkout, and waved through when it exits zero on both. A gate that fails on the clean checkout is gate broken and catches nothing. Proven by test_git_mode_scores_caught_and_waved.
- SLICE2-4 The scratch clone is deleted at the end of a run unless the operator asks to keep it. Proven by test_git_mode_scratch_clone_deleted_by_default and test_git_mode_scratch_clone_kept_on_request.
- SLICE2-5 Copy mode does not change. The slice 1 example catch table regenerates byte identical under copy mode, and every slice 1 obligation above keeps passing. Proven by test_copy_mode_example_regenerates_byte_identical.
- SLICE2-6 Two gates can judge the same field in one run and their verdicts are reported side by side. A second committed example shows two gates over the same target with at least one seed where the gates honestly disagree. Proven by test_two_gate_example_shows_an_honest_disagreement and test_two_gate_run_scores_each_gate_on_its_own_evidence.
- SLICE2-7 The seed registry can be written out as a versioned seed pack file. The pack carries the format string canaryfield-seeds/1 and one entry per seed with id, name, defect class, and description. Loading the pack returns the same twelve ids and defect classes as the built in registry. Proven by test_seed_pack_round_trip and test_seed_pack_via_cli.
- SLICE2-8 Git mode refuses a target that is not a git repository. The refusal is a clear error before any planting happens, never a silent fall back to copy mode. Proven by test_git_mode_requires_a_git_target.

## Round 1 review fix obligations

The Round 1 audit and red team found ways a run could harm material outside its own scratch and ways its refusals and pack loading were weaker than the claims around them. The obligations below amend the slice obligations where they conflict. Each one is proven by tests in tests/test_round1_fixes.py unless it says otherwise.

- FIX2-1 A run removes only what it created. A supplied work root must start empty and a run refuses a non empty work root before doing any work, on success and on failure paths alike, so a failed run can never delete a previously kept clone or an unrelated operator file. A work root the run created itself is removed whole, and a pre existing empty work root keeps only what was already inside it, which is nothing the run did not place there.
- FIX2-2 Git mode isolates its seeds from its gates. The scratch clone checkout is hard reset and cleaned between the clean run and every seed, so a gate that writes tracked or untracked files inside the checkout cannot abort the run or poison a later seed. Gates are still expected to be read only judges, and the engine no longer depends on that expectation.
- FIX2-3 The seed pack loader enforces the registry invariants it mirrors. A pack of this format carries exactly twelve entries, every entry field is a non empty piece of text, and ids are unique. A pack that breaks any of those rules is refused.
- FIX2-4 Git mode refusals name the actual problem. A target inside another repository but not at its root is refused as not the root, a repository with no commits is refused as having no commits, and the command line surface prints refusals as one error line with a non zero exit and never as a traceback.
- FIX2-5 The two planting modes judge different material and the contract says so. Git mode judges the committed HEAD of the target because a clone carries only committed state, and copy mode judges the working tree as it stands. An operator with uncommitted edits gets different content scored by the two modes, by construction.

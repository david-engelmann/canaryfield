# How Canaryfield Compares

Teams invest heavily in multi-layered CI pipelines combining unit tests, typecheckers, static linters, security scanners, and AI review bots, yet rarely measure the **joint catch rate, overlap, or blind spots** of those gates against realistic bugs.

## Ecosystem Comparison

| Capability | Classic mutation tools (`mutmut`) | Chaos engineering (`Chaos Mesh`) | Static benchmark suites (`SWE-bench`) | `canaryfield` |
| --- | --- | --- | --- | --- |
| Scores *all* gate types simultaneously (tests, linters, compilers, review bots) | Pytest runner only | Runtime infra faults, not source defects | Evaluates patch generation, not CI gates | **Yes (repeatable `--gate NAME=CMD`)** |
| Plants realistic structural defect classes (200+ AST seeds across 49+ pack versions) | Low-level token tweaks | N/A | Fixed historical repos only | **Yes (on *your* live repository)** |
| Supports throwaway git-branch planting for diff-aware gates | No | No | No | **Yes (`--plant git-branches`)** |
| Guarantees target working tree invariance (`target_unchanged`) | Mutates or stashes tree in some modes | Mutates cluster state | N/A | **Yes (verified byte-for-byte)** |
| Distinguishes broken baseline gates (`gate-broken`) from catches | Often conflated | N/A | N/A | **Yes (`clean_exits` baseline check)** |

## Sibling Boundaries in Gate Truth

- **Ostensible** asks *whether an individual test function can fail* on a pull request.
- **Canaryfield** asks *whether the whole CI harness catches planted defect classes* across a repository.
- **Greenharvest** uses catch evidence to ask *which tests and gates earn their CI runtime*.

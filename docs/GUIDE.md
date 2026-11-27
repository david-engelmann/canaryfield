# Canaryfield Operator Guide

Canaryfield asks whether the harness catches planted defects. Given a target codebase and one or more named gate commands (unit tests, linters, typecheckers, custom policy scripts), Canaryfield plants defects from its 200+ seed Python defect registry (`204` seeds at `v0.46.0`) into isolated scratch copies or throwaway git branches, runs every gate against every planting, and emits a deterministic catch matrix naming which defects were caught and which were waved through.

## Installation

### Prebuilt Release Archive

On Linux (`x86_64`, `aarch64`) and macOS (`aarch64`, `x86_64`), install a prebuilt binary for releases carrying release archives (`v0.46.0` once attached and `v0.47.0` onward) with the checksum-verifying installer script.

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/canaryfield/main/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
```

Pin a specific release tag by setting `CANARYFIELD_VERSION` (for example `CANARYFIELD_VERSION=v0.46.0`). Because `install.sh` does not edit shell profile files, ensure `$HOME/.local/bin` (or `$CANARYFIELD_INSTALL_DIR`) stands on `PATH`. On Windows (`x86_64-pc-windows-msvc`), download the `.zip` archive and `.sha256` checksum from the [Releases](https://github.com/david-engelmann/canaryfield/releases) page.

### Building from Source or `cargo-binstall`

With a Rust 1.80 or newer toolchain installed, install directly from source or via `cargo-binstall`.

```sh
cargo install --git https://github.com/david-engelmann/canaryfield --locked
cargo binstall --git https://github.com/david-engelmann/canaryfield canaryfield
```

## Command Line Interface

```
canaryfield [-h | --help] [--target TARGET] [--gate NAME=COMMAND]
            [--plant {copy,git-branches}] [--write-seed-pack PATH]
            [--out OUT] [--json] [--timeout TIMEOUT]
            [--work-root WORK_ROOT] [--keep-scratch]
```

Options

- `-h`, `--help` Print the help summary to standard output and exit `0`.
- `--target TARGET` Path to the Python repository or directory to copy and plant defects into. Never modified in place (`target_unchanged` is verified on every run).
- `--gate NAME=COMMAND` Repeatable named gate command executed inside each planted scratch copy via `sh -c COMMAND` (on Windows, ensure POSIX `sh` such as Git Bash is on `PATH`). First runs against the unplanted copy to record the gate's clean baseline exit code.
- `--plant {copy,git-branches}` Planting strategy, defaulting to `copy` (fresh scratch directories) or `git-branches` (one throwaway git branch per applicable seed inside a local scratch clone).
- `--write-seed-pack PATH` Write the current `canaryfield-seeds/49` (or newer) registry pack JSON to `PATH` and exit `0`.
- `--out OUT` Directory into which `catch-table.txt` and `catch-table.json` are written.
- `--json` Print the sorted-key JSON catch report to standard output instead of the text table.
- `--timeout TIMEOUT` Integer per-gate-invocation timeout in seconds, defaulting to `120` (non-positive integers clamp the deadline to `0` seconds).
- `--work-root WORK_ROOT` Custom parent directory for scratch copies (defaults to the OS temporary directory).
- `--keep-scratch` Retain planted scratch copies or scratch git branches after the run so engineers can inspect surviving defect diffs directly.

Exit codes

- `0` Scoring completed with `target_unchanged` verified `true` (or `--write-seed-pack` / `--help` succeeded). Inspect `summary.<gate>.waved-through` and `gate-broken` in the JSON output to enforce catch-rate thresholds in CI.
- `1` Standard output write error while printing the rendered report.
- `2` Usage refusal, missing `--target` or `--gate`, non-integer `--timeout`, invalid seed pack, or unreadable target directory.
- `3` Target tree invariance violation when `target_unchanged` evaluates to `false`.

## Quickstart Multi-Gate Comparison

Clone the repository and score two gates side-by-side (`tests` vs `compile`) over the synthetic `examples/demo-project` target

```sh
git clone https://github.com/david-engelmann/canaryfield.git
cd canaryfield
canaryfield \
  --target examples/demo-project \
  --gate "tests=python3 -m unittest" \
  --gate "compile=python3 -m py_compile sample.py" \
  --timeout 10
```

At `v0.46.0` (`204` registered seeds under `canaryfield-seeds/49`), `34` seeds plant into `examples/demo-project/sample.py`, `tests` catches `23` and waves through `11` (`dropped call argument`, `removed collection element`, `widened exception handler`, `swapped sequence literal elements`, `dropped parameter default`, `dropped raised error message`, `dropped exception type`, `dropped last list element`, `dropped last call argument`, `dropped last return value list element`, `dropped first return value list element`), while `compile` catches `0` and waves through all `34`.

## Scheduled GitHub Actions Harness Audit

```yaml
name: canaryfield-harness-audit

on:
  schedule:
    - cron: "0 6 * * 1"
  workflow_dispatch:

permissions:
  contents: read

jobs:
  score-gates:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install project test and lint dependencies
        run: python -m pip install pytest ruff
      - name: Install canaryfield
        run: |
          curl -fsSL https://raw.githubusercontent.com/david-engelmann/canaryfield/main/install.sh | sh
          echo "$HOME/.local/bin" >> "$GITHUB_PATH"
      - name: Score pytest and ruff gates against planted defects
        run: |
          canaryfield \
            --target . \
            --gate "pytest=python -m pytest -q" \
            --gate "ruff=ruff check ." \
            --timeout 30 \
            --out /tmp/canaryfield-out
          cat /tmp/canaryfield-out/catch-table.txt
      - uses: actions/upload-artifact@v4
        with:
          name: canaryfield-catch-table
          path: /tmp/canaryfield-out/*
```

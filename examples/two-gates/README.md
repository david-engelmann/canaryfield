# Two gates over one field

This example runs the slice 1 demo project through two gates in a single run. The tests gate runs the project test suite. The compile gate only checks that every file still compiles.

The catch table carries one column per gate and the gates honestly disagree. The test suite catches ten of the twelve seeds. The compile gate catches none of them because every planted defect still parses. A seed can therefore be caught and waved through in the same run and both verdicts are true at once.

Regenerate the tables from the repo root with one command.

python3 -m canaryfield --target examples/demo-project --gate tests="python3 -m pytest -q" --gate compile="python3 -m compileall -q ." --out examples/two-gates

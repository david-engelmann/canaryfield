"""The report module retirement, pinned at every import site.

PORT4-8 retires canaryfield/report.py whole. This smoke pin
sweeps the package the way the Round 15 retirement lesson
asks, so a module that reaches for the retired module again
fails the suite instead of failing a caller later.
"""

import importlib.util
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1] / "canaryfield"


def test_the_report_module_does_not_stand():
    assert importlib.util.find_spec("canaryfield.report") is None


def test_the_command_line_imports_without_the_report_module():
    import canaryfield.cli  # noqa: F401


def test_no_package_module_imports_the_report_module():
    offenders = []
    for path in sorted(PACKAGE.glob("*.py")):
        for line in path.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if (
                stripped.startswith("from .report import")
                or stripped.startswith("from canaryfield.report")
                or stripped.startswith("import canaryfield.report")
            ):
                offenders.append(f"{path.name}: {stripped}")
    assert offenders == []

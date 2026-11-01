from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def test_seed_files_present():
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    assert "## What this is" in readme
    assert "## Licence" in readme
    assert "MIT" in (ROOT / "LICENSE").read_text(encoding="utf-8")
    assert "__pycache__/" in (ROOT / ".gitignore").read_text(encoding="utf-8")


def test_readme_status_is_honest():
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    status = readme.split("## Status", 1)[1].split("## Licence", 1)[0]
    for phrase in (
        "local seed is done",
        "403",
        "CONTRACT.md",
        "10 of 100",
        "AGENTS.md",
        "canonical template",
        "synthetic",
        "numbers only",
    ):
        assert phrase in status, phrase


def test_ci_runs_the_local_gate():
    workflow = (ROOT / ".github" / "workflows" / "ci.yml").read_text(encoding="utf-8")
    assert "python -m pytest" in workflow
    assert "push" in workflow
    assert "pull_request" in workflow


def test_no_agents_md_yet():
    assert not (ROOT / "AGENTS.md").exists()


def test_example_target_is_synthetic():
    sample = (ROOT / "examples" / "demo-project" / "sample.py").read_text(encoding="utf-8")
    assert "Synthetic example target" in sample

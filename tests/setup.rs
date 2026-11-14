//! The setup pins, re-expressed from tests/test_setup.py at
//! the PORT6 retirement under PORT6-7.
//!
//! The Python suite's setup pins held the repository's
//! standing shape, and they stand here in Rust from the
//! retirement on. The closed round count below moves only
//! at a round closure, together with the README line it
//! pins. The one Python-era assertion of the old file, the
//! pycache line of the ignore file, retires with the suite
//! it described.

use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|_| panic!("{rel} reads"))
}

#[test]
fn the_seed_files_stand() {
    let readme = read("README.md");
    assert!(readme.contains("## What this is"), "the README opens with its account");
    assert!(readme.contains("## Licence"), "the README carries its licence section");
    assert!(read("LICENSE").contains("MIT"), "the licence stands");
}

#[test]
fn the_readme_status_is_honest() {
    let readme = read("README.md");
    let status = readme
        .split("## Status")
        .nth(1)
        .expect("the status section stands")
        .split("## Licence")
        .next()
        .expect("the status section ends at the licence");
    for phrase in [
        "local seed is done",
        "403",
        "CONTRACT.md",
        "18 of 100",
        "AGENTS.md",
        "canonical template",
        "synthetic",
        "numbers only",
    ] {
        assert!(status.contains(phrase), "the status carries {phrase}");
    }
}

#[test]
fn the_ci_runs_the_local_gate() {
    let workflow = read(".github/workflows/ci.yml");
    assert!(workflow.contains("cargo test"), "the gate stands in CI");
    assert!(workflow.contains("push"), "the push trigger stands");
    assert!(workflow.contains("pull_request"), "the pull request trigger stands");
}

#[test]
fn the_agents_md_stands_from_the_canonical_template() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("<!-- repo annex below this line -->"),
        "the annex marker stands"
    );
    assert!(agents.contains("## Repo annex"), "the annex stands");
}

#[test]
fn the_example_target_is_synthetic() {
    let sample = read("examples/demo-project/sample.py");
    assert!(
        sample.contains("Synthetic example target"),
        "the demo target names itself synthetic"
    );
}

//! The PORT6 conformance battery.
//!
//! The recordings at tests/conformance/port6_cases.json were
//! generated from the landed Python command line before its
//! retirement, driven as a process with the crate binary
//! named in its discovery, never from this port, by
//! tools/gen_port6_recordings.py. Each case stands up its
//! fixture under a fresh scene directory, drives the crate
//! binary with the recorded arguments from that scene, and
//! compares the exit code, both output streams whole, and
//! the bytes of every file the case names. The two spellings
//! the recordings tokenize, the scene's absolute path and
//! the Python interpreter, are substituted back before the
//! drive, the interpreter as the word the standing harness
//! resolves it to.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use canaryfield::json::{parse, JsonValue};

fn text_of(value: &JsonValue) -> &str {
    match value {
        JsonValue::Str(text) => text,
        other => panic!("expected text, found {other:?}"),
    }
}

const FIXTURE_BUILDER: &str = r#"
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, sys.argv[2])
from fixture_project import write_main_project

scene = Path(sys.argv[1])
kind = sys.argv[3]
project = scene / "project"
write_main_project(project)
if kind == "main-git":
    env = dict(__import__("os").environ)
    env.update({
        "GIT_AUTHOR_NAME": "canaryfield",
        "GIT_AUTHOR_EMAIL": "canaryfield@example.invalid",
        "GIT_COMMITTER_NAME": "canaryfield",
        "GIT_COMMITTER_EMAIL": "canaryfield@example.invalid",
    })
    def git(arguments):
        done = subprocess.run(
            ["git", "-C", str(project)] + arguments,
            capture_output=True, text=True, env=env,
        )
        if done.returncode != 0:
            raise SystemExit(f"fixture git {arguments} failed: {done.stderr}")
    git(["init", "-q"])
    git(["add", "-A"])
    git(["commit", "-q", "-m", "fixture"])
"#;

fn python3() -> String {
    std::env::var("CANARYFIELD_PYTHON").unwrap_or_else(|_| "python3".to_string())
}

fn fresh_scene(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("canaryfield-port6-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("the scene directory stands up");
    dir
}

fn build_fixture(kind: &str, scene: &Path) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let builder = scene.join("build_fixture.py");
    fs::write(&builder, FIXTURE_BUILDER).expect("the fixture builder writes");
    let status = Command::new(python3())
        .arg(&builder)
        .arg(scene)
        .arg(root.join("tests"))
        .arg(kind)
        .status()
        .expect("the fixture builder runs");
    assert!(status.success(), "the fixture {kind} builds");
}

fn substitute(text: &str, scene: &Path) -> String {
    text.replace("{SCENE}", &scene.display().to_string())
        .replace("{PYTHON}", &python3())
}

#[test]
fn port6_cases_match_the_recorded_python_answers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let raw = fs::read_to_string(root.join("tests/conformance/port6_cases.json"))
        .expect("the PORT6 recordings read");
    let document = parse(&raw).expect("the PORT6 recordings parse");
    let cases = match document.get("cases") {
        Some(JsonValue::Array(items)) => items.clone(),
        other => panic!("the cases stand as a list: {other:?}"),
    };
    let binary = env!("CARGO_BIN_EXE_canaryfield");
    let mut compared = 0usize;
    for case in &cases {
        let name = text_of(case.get("name").expect("name")).to_string();
        let scene = fresh_scene(&name);
        if let Some(JsonValue::Str(kind)) = case.get("fixture") {
            build_fixture(&kind.clone(), &scene);
        }
        let argv: Vec<String> = match case.get("argv") {
            Some(JsonValue::Array(items)) => items
                .iter()
                .map(|item| substitute(text_of(item), &scene))
                .collect(),
            other => panic!("case {name}: the argv stands as a list: {other:?}"),
        };
        let output = Command::new(binary)
            .args(&argv)
            .current_dir(&scene)
            .output()
            .expect("the binary runs");
        let wanted_exit = match case.get("exit") {
            Some(JsonValue::Int(text)) => text.parse::<i32>().expect("the exit reads"),
            other => panic!("case {name}: the exit stands as a number: {other:?}"),
        };
        assert_eq!(
            output.status.code(),
            Some(wanted_exit),
            "case {name}: the exit code"
        );
        let stdout = String::from_utf8(output.stdout).expect("stdout is text");
        let stderr = String::from_utf8(output.stderr).expect("stderr is text");
        assert_eq!(
            stdout,
            substitute(text_of(case.get("stdout").expect("stdout")), &scene),
            "case {name}: stdout"
        );
        assert_eq!(
            stderr,
            substitute(text_of(case.get("stderr").expect("stderr")), &scene),
            "case {name}: stderr"
        );
        if let Some(JsonValue::Object(files)) = case.get("files") {
            for (rel, wanted) in files {
                let bytes = fs::read(scene.join(rel))
                    .unwrap_or_else(|_| panic!("case {name}: the file {rel} stands"));
                let text = String::from_utf8(bytes).expect("the file is text");
                assert_eq!(
                    text,
                    substitute(text_of(wanted), &scene),
                    "case {name}: the file {rel}"
                );
            }
        }
        compared += 1;
    }
    assert_eq!(compared, 25, "the whole PORT6 battery stands compared");
}

/// The example reports and the example seed pack regenerate
/// byte identical through the shell, under PORT6-5.
#[test]
fn the_examples_regenerate_through_the_shell() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let binary = env!("CARGO_BIN_EXE_canaryfield");
    let scene = fresh_scene("examples");
    let pack = scene.join("seed-pack.json");
    let output = Command::new(binary)
        .args([
            "--write-seed-pack",
            &pack.display().to_string(),
            "--keep-scratch",
        ])
        .current_dir(&scene)
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "the seed pack writes");
    assert_eq!(
        fs::read(&pack).expect("the pack reads"),
        fs::read(root.join("examples/seed-pack.json")).expect("the example reads"),
        "the example seed pack regenerates byte identical"
    );
    let gate = format!("tests={} -m pytest -q", python3());
    let output = Command::new(binary)
        .args([
            "--target",
            "examples/demo-project",
            "--gate",
            &gate,
            "--out",
            &scene.join("out").display().to_string(),
        ])
        .current_dir(root)
        .output()
        .expect("the binary runs");
    assert!(
        output.status.success(),
        "the demo run stands: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is text");
    let example_text =
        fs::read_to_string(root.join("examples/catch-table.txt")).expect("the example reads");
    assert_eq!(
        stdout, example_text,
        "the text rendering matches the example"
    );
    assert_eq!(
        fs::read_to_string(scene.join("out/catch-table.txt")).expect("the file reads"),
        example_text,
        "the written text table matches the example"
    );
    assert_eq!(
        fs::read_to_string(scene.join("out/catch-table.json")).expect("the file reads"),
        fs::read_to_string(root.join("examples/catch-table.json")).expect("the example reads"),
        "the written JSON table matches the example"
    );
}

/// A census of the battery document itself, so a recording
/// dropped in an edit fails here rather than passing quiet.
#[test]
fn the_battery_document_shape_stands() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let raw = fs::read_to_string(root.join("tests/conformance/port6_cases.json"))
        .expect("the PORT6 recordings read");
    let document = parse(&raw).expect("the PORT6 recordings parse");
    assert_eq!(
        text_of(document.get("format").expect("format")),
        "canaryfield-port6-cases/1"
    );
    let mut names = BTreeMap::new();
    if let Some(JsonValue::Array(items)) = document.get("cases") {
        for case in items {
            names.insert(text_of(case.get("name").expect("name")).to_string(), ());
        }
    }
    assert_eq!(names.len(), 25, "the battery carries its twenty five cases");
}

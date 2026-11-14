//! PORT5 recorded conformance and pins.
//!
//! The battery at tests/conformance/port5_cases.json was
//! recorded from the landed Python engine by
//! tools/gen_port5_recordings.py, never from this port. Each
//! case rebuilds its fixture under a fresh root outside the
//! checkout, drives the binary's run field door with the
//! recorded request, and compares the answer whole, a result
//! by its parsed document and a refusal by its error text.
//! The two spellings the recordings tokenize, the fixture
//! root and the pytest interpreter, are substituted back
//! before the drive. A case whose gates need pytest is
//! skipped where no pytest stands in the environment's
//! python, and says so, because the cargo gate of a bare
//! runner carries no Python packages. The census pins stand
//! over the battery's shape whether or not a case replays.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use canaryfield::engine::normalize_path;
use canaryfield::json::{dumps_indent2, parse, JsonValue};
use canaryfield::sha256::sha256_hex;

const BATTERY: &str = include_str!("conformance/port5_cases.json");

const FIXTURE_BUILDER: &str = r#"
import os
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[2])
sys.path.insert(0, sys.argv[3])
sys.path.insert(0, str(Path(sys.argv[3]) / "tests"))
from fixture_project import (
    write_main_project,
    write_tiny_project,
    write_two_module_project,
)

GIT_ENV = {
    "GIT_AUTHOR_NAME": "canaryfield",
    "GIT_AUTHOR_EMAIL": "canaryfield@example.invalid",
    "GIT_COMMITTER_NAME": "canaryfield",
    "GIT_COMMITTER_EMAIL": "canaryfield@example.invalid",
}


def git(arguments, cwd):
    env = dict(os.environ)
    env.update(GIT_ENV)
    done = subprocess.run(
        ["git", "-C", str(cwd)] + arguments,
        capture_output=True, text=True, env=env,
    )
    if done.returncode != 0:
        raise SystemExit(f"fixture git {arguments} failed: {done.stderr}")


kind = sys.argv[1]
proj = root / "proj"
if kind == "main":
    write_main_project(proj)
elif kind == "main-git":
    write_main_project(proj)
    git(["init", "-q"], proj)
    git(["add", "-A"], proj)
    git(["commit", "-q", "-m", "fixture"], proj)
elif kind in ("tiny", "nonempty-work"):
    write_tiny_project(proj)
    if kind == "nonempty-work":
        work = root / "work"
        work.mkdir(parents=True)
        (work / "standing.txt").write_text("standing\n", encoding="utf-8")
elif kind == "two-module":
    write_two_module_project(proj)
elif kind == "symlink":
    write_tiny_project(proj)
    os.symlink("tiny.py", proj / "linked.py")
elif kind == "repo-sub":
    repo = root / "repo"
    write_tiny_project(repo / "proj")
    git(["init", "-q"], repo)
    git(["add", "-A"], repo)
    git(["commit", "-q", "-m", "fixture"], repo)
elif kind == "empty-repo":
    proj.mkdir(parents=True, exist_ok=True)
    git(["init", "-q"], proj)
elif kind == "none":
    pass
else:
    raise SystemExit(f"unknown fixture kind {kind}")
"#;

fn json_eq(a: &JsonValue, b: &JsonValue) -> bool {
    match (a, b) {
        (JsonValue::Null, JsonValue::Null) => true,
        (JsonValue::Bool(x), JsonValue::Bool(y)) => x == y,
        (JsonValue::Int(x), JsonValue::Int(y)) => x == y,
        (JsonValue::Float(x), JsonValue::Float(y)) => x.to_bits() == y.to_bits(),
        (JsonValue::Str(x), JsonValue::Str(y)) => x == y,
        (JsonValue::Array(x), JsonValue::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(u, v)| json_eq(u, v))
        }
        (JsonValue::Object(x), JsonValue::Object(y)) => {
            if x.len() != y.len() {
                return false;
            }
            let mut xs: Vec<&(String, JsonValue)> = x.iter().collect();
            let mut ys: Vec<&(String, JsonValue)> = y.iter().collect();
            xs.sort_by(|u, v| u.0.cmp(&v.0));
            ys.sort_by(|u, v| u.0.cmp(&v.0));
            xs.iter()
                .zip(ys.iter())
                .all(|(u, v)| u.0 == v.0 && json_eq(&u.1, &v.1))
        }
        _ => false,
    }
}

fn substitute(value: &JsonValue, root: &str, python: &str) -> JsonValue {
    match value {
        JsonValue::Str(text) => {
            JsonValue::Str(text.replace("<ROOT>", root).replace("<PYTHON>", python))
        }
        JsonValue::Array(items) => JsonValue::Array(
            items
                .iter()
                .map(|item| substitute(item, root, python))
                .collect(),
        ),
        JsonValue::Object(pairs) => JsonValue::Object(
            pairs
                .iter()
                .map(|(key, item)| (key.clone(), substitute(item, root, python)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn python3() -> String {
    std::env::var("CANARYFIELD_PYTHON").unwrap_or_else(|_| "python3".to_string())
}

fn pytest_available() -> bool {
    Command::new(python3())
        .args(["-c", "import pytest"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn case_root(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("canaryfield-port5-{}-{}", std::process::id(), name));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("the case root is created");
    root
}

fn build_fixture(kind: &str, root: &Path) {
    let builder = root.join("build_fixture.py");
    std::fs::write(&builder, FIXTURE_BUILDER).expect("the builder is written");
    let repo = env!("CARGO_MANIFEST_DIR");
    let status = Command::new(python3())
        .arg(&builder)
        .arg(kind)
        .arg(root)
        .arg(repo)
        .status()
        .expect("the fixture builder runs");
    assert!(status.success(), "the fixture {kind} builds");
}

fn needs_pytest(case: &JsonValue) -> bool {
    // A refusal answers before any gate runs, so only a result
    // case whose gates run under the pytest interpreter needs
    // pytest standing in the environment.
    if case.get("result").is_none() {
        return false;
    }
    fn hunt(value: &JsonValue) -> bool {
        match value {
            JsonValue::Str(text) => text.contains("<PYTHON>"),
            JsonValue::Array(items) => items.iter().any(hunt),
            JsonValue::Object(pairs) => pairs.iter().any(|(_, item)| hunt(item)),
            _ => false,
        }
    }
    hunt(case.get("request").expect("the case holds its request"))
}

#[test]
fn the_port5_battery_replays() {
    let document = parse(BATTERY).expect("the battery parses");
    let cases = match document.get("cases") {
        Some(JsonValue::Array(items)) => items.clone(),
        _ => panic!("the battery holds no cases"),
    };
    assert_eq!(cases.len(), 13, "the case census stands at 13");
    let results = cases
        .iter()
        .filter(|case| case.get("result").is_some())
        .count();
    let refusals = cases
        .iter()
        .filter(|case| case.get("error").is_some())
        .count();
    assert_eq!(
        (results, refusals),
        (7, 6),
        "seven results and six refusals"
    );
    let have_pytest = pytest_available();
    let python = python3();
    let binary = env!("CARGO_BIN_EXE_canaryfield");
    let mut replayed = 0;
    for case in &cases {
        let name = case.get("name").and_then(JsonValue::as_str).unwrap();
        if needs_pytest(case) && !have_pytest {
            eprintln!("case {name} skipped, no pytest stands in {python}");
            continue;
        }
        let fixture = case.get("fixture").and_then(JsonValue::as_str).unwrap();
        let root = case_root(name);
        build_fixture(fixture, &root);
        let root_text = root.to_string_lossy().into_owned();
        let request = substitute(
            case.get("request").expect("the case holds its request"),
            &root_text,
            &python,
        );
        let request_path = root.join("request.json");
        std::fs::write(&request_path, dumps_indent2(&request)).expect("the request is written");
        let out = Command::new(binary)
            .arg("--run-field")
            .arg(&request_path)
            .output()
            .expect("the run door drives");
        if let Some(recorded_error) = case.get("error") {
            let expected = substitute(recorded_error, &root_text, &python);
            assert!(!out.status.success(), "case {name} refuses at the door");
            let spoken = String::from_utf8_lossy(&out.stderr).trim().to_string();
            assert_eq!(
                JsonValue::Str(spoken),
                expected,
                "case {name} refuses in the recorded words"
            );
        } else {
            assert!(
                out.status.success(),
                "case {name} runs, stderr {}",
                String::from_utf8_lossy(&out.stderr)
            );
            let answer =
                parse(&String::from_utf8_lossy(&out.stdout)).expect("the door answers a document");
            let expected = substitute(
                case.get("result").expect("the case holds its result"),
                &root_text,
                &python,
            );
            assert!(
                json_eq(&answer, &expected),
                "case {name} answers as recorded"
            );
            if let Some(standing) = case.get("scratch_standing") {
                let work = root.join("work");
                let mut names: Vec<String> = std::fs::read_dir(&work)
                    .expect("the kept work root stands")
                    .flatten()
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .collect();
                names.sort();
                let expected_names = match standing {
                    JsonValue::Array(items) => items
                        .iter()
                        .map(|item| item.as_str().unwrap().to_string())
                        .collect::<Vec<String>>(),
                    _ => panic!("the standing list is a list"),
                };
                assert_eq!(names, expected_names, "case {name} keeps its scratch whole");
            }
        }
        replayed += 1;
        let _ = std::fs::remove_dir_all(&root);
    }
    assert!(
        replayed >= 7,
        "the refusal and gate cases replay everywhere"
    );
}

#[test]
fn the_hash_digests_stand_as_published() {
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn the_path_spelling_stands_as_pathlib_leaves_it() {
    assert_eq!(normalize_path("./project/"), "project");
    assert_eq!(normalize_path("project/"), "project");
    assert_eq!(normalize_path("a//b"), "a/b");
    assert_eq!(normalize_path("a/./b"), "a/b");
    assert_eq!(normalize_path("a/../b"), "a/../b");
    assert_eq!(normalize_path("."), ".");
    assert_eq!(normalize_path("/"), "/");
    assert_eq!(normalize_path("//"), "//");
    assert_eq!(normalize_path(""), ".");
    assert_eq!(normalize_path("/tmp/x/./proj/"), "/tmp/x/proj");
}

#[test]
fn a_work_root_inside_the_target_refuses_before_anything_is_created() {
    let root = case_root("containment");
    build_fixture("tiny", &root);
    let target = root.join("proj");
    let work = target.join("work");
    let request = JsonValue::Object(vec![
        (
            "target".to_string(),
            JsonValue::Str(target.to_string_lossy().into_owned()),
        ),
        (
            "gates".to_string(),
            JsonValue::Array(vec![JsonValue::Array(vec![
                JsonValue::Str("always".to_string()),
                JsonValue::Str("true".to_string()),
            ])]),
        ),
        ("timeout".to_string(), JsonValue::Int("60".to_string())),
        (
            "work_root".to_string(),
            JsonValue::Str(work.to_string_lossy().into_owned()),
        ),
        ("keep_scratch".to_string(), JsonValue::Bool(false)),
        ("plant_mode".to_string(), JsonValue::Str("copy".to_string())),
    ]);
    let request_path = root.join("request.json");
    std::fs::write(&request_path, dumps_indent2(&request)).expect("the request is written");
    let out = Command::new(env!("CARGO_BIN_EXE_canaryfield"))
        .arg("--run-field")
        .arg(&request_path)
        .output()
        .expect("the run door drives");
    assert!(!out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stderr).trim(),
        format!(
            "work root stands inside the target, refusing to use it: {}",
            work.to_string_lossy()
        )
    );
    assert!(!work.exists(), "nothing is created inside the target");
    let _ = std::fs::remove_dir_all(&root);
}

//! Feature slice 1 pins, the timeout standing.
//!
//! Each pin drives the run over a synthetic target in the
//! open, a module holding one comparison and a gate script
//! whose behaviour turns on the planted marker the inverted
//! comparison leaves, or on nothing at all. The runs plant
//! in copy mode at a one second timeout against scripts
//! that sleep three seconds where they sleep, so a killed
//! gate is killed at its deadline with room to spare.

use std::fs;
use std::path::PathBuf;

use canaryfield::engine::{run, RunRequest};
use canaryfield::json::JsonValue;

fn target_with(gate_script: &str, tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("canaryfield-feature1-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("the target directory stands");
    fs::write(dir.join("mod.py"), "def check(x):\n    return x == 1\n").expect("the module writes");
    fs::write(dir.join("check.sh"), gate_script).expect("the gate script writes");
    dir
}

fn run_target(target: &PathBuf) -> JsonValue {
    let request = RunRequest {
        target: target.to_string_lossy().into_owned(),
        gates: vec![("tests".to_string(), "sh check.sh".to_string())],
        timeout: 1.0,
        work_root: None,
        keep_scratch: false,
        plant_mode: "copy".to_string(),
    };
    run(&request).expect("the run completes")
}

fn cell_of(result: &JsonValue, seed: &str) -> String {
    let rows = match result.get("rows") {
        Some(JsonValue::Array(items)) => items,
        _ => panic!("the result carries rows"),
    };
    for row in rows {
        if row.get("seed").and_then(|value| value.as_str()) == Some(seed) {
            return row
                .get("gates")
                .and_then(|gates| gates.get("tests"))
                .and_then(|cell| cell.as_str())
                .expect("the row carries the gate cell")
                .to_string();
        }
    }
    panic!("the result carries the {seed} row");
}

fn planted_exit_of(result: &JsonValue, seed: &str) -> i64 {
    let rows = match result.get("rows") {
        Some(JsonValue::Array(items)) => items,
        _ => panic!("the result carries rows"),
    };
    for row in rows {
        if row.get("seed").and_then(|value| value.as_str()) == Some(seed) {
            return match row
                .get("planted_exits")
                .and_then(|exits| exits.get("tests"))
            {
                Some(JsonValue::Int(text)) => {
                    text.parse().expect("the planted exit spells an integer")
                }
                _ => panic!("the row carries the planted exit"),
            };
        }
    }
    panic!("the result carries the {seed} row");
}

#[test]
fn test_planted_gate_killed_at_its_timeout_reads_timed_out() {
    let target = target_with(
        "#!/bin/sh\ngrep -q \"!= 1\" mod.py && sleep 3\nexit 0\n",
        "timeout",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-02"), "timed-out");
    assert_eq!(
        planted_exit_of(&result, "seed-02"),
        -1,
        "the recorded planted exit still spells a timeout as minus one"
    );
    let summary = result
        .get("summary")
        .and_then(|summary| summary.get("tests"))
        .expect("the summary carries the gate");
    for key in ["caught", "waved-through", "not-applicable", "gate-broken"] {
        assert!(summary.get(key).is_some(), "the summary keeps {key}");
    }
    assert!(
        summary.get("timed-out").is_none(),
        "no fifth summary key stands"
    );
    match summary.get("caught") {
        Some(JsonValue::Int(text)) => {
            assert_eq!(text, "0", "the timed-out cell counts as no catch")
        }
        _ => panic!("the summary carries the caught count"),
    }
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_planted_gate_exiting_nonzero_still_reads_caught() {
    let target = target_with(
        "#!/bin/sh\ngrep -q \"!= 1\" mod.py && exit 1\nexit 0\n",
        "caught",
    );
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-02"), "caught");
    let _ = fs::remove_dir_all(&target);
}

#[test]
fn test_clean_gate_killed_at_its_timeout_still_reads_gate_broken() {
    let target = target_with("#!/bin/sh\nsleep 3\nexit 0\n", "broken");
    let result = run_target(&target);
    assert_eq!(cell_of(&result, "seed-02"), "gate-broken");
    let _ = fs::remove_dir_all(&target);
}

//! The field run of PORT5, the machinery canaryfield/engine.py
//! carried before this slice.
//!
//! A run works on scratch material only. In copy mode it copies
//! the target tree into scratch copies, one clean copy and one
//! planted copy per seed. In git branch mode it clones the
//! target into a scratch clone and commits each seed on its own
//! throwaway branch there. Gates run inside the scratch
//! material only, the target is opened read only, and the run
//! proves that with a hash walk taken before and after. A
//! supplied work root must start empty and a run removes only
//! what it created. Every refusal is carried as the wording the
//! Python raises, and the run door prints it whole.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::json::JsonValue;
use crate::pyplant::{plant_seed, PlantError};
use crate::registry::SEEDS;
use crate::sha256::sha256_hex;

pub const CAUGHT: &str = "caught";
pub const WAVED: &str = "waved-through";
pub const NOT_APPLICABLE: &str = "not-applicable";
pub const GATE_BROKEN: &str = "gate-broken";

const IGNORED_PARTS: [&str; 3] = [".git", "__pycache__", ".pytest_cache"];

/// One run request, the document the run door reads.
pub struct RunRequest {
    pub target: String,
    pub gates: Vec<(String, String)>,
    pub timeout: f64,
    pub work_root: Option<String>,
    pub keep_scratch: bool,
    pub plant_mode: String,
}

/// Read a run request out of a parsed JSON document. A document
/// missing the standing shape is refused in plain words, a shape
/// only the bridge writes and only the door reads.
pub fn request_from(document: &JsonValue) -> Result<RunRequest, String> {
    let target = document
        .get("target")
        .and_then(JsonValue::as_str)
        .ok_or("the run request holds no target text")?
        .to_string();
    let mut gates = Vec::new();
    match document.get("gates") {
        Some(JsonValue::Array(items)) => {
            for item in items {
                match item {
                    JsonValue::Array(pair) if pair.len() == 2 => {
                        let name = pair[0]
                            .as_str()
                            .ok_or("a gate of the run request holds no name text")?;
                        let command = pair[1]
                            .as_str()
                            .ok_or("a gate of the run request holds no command text")?;
                        gates.push((name.to_string(), command.to_string()));
                    }
                    _ => return Err("a gate of the run request is no name and command pair".into()),
                }
            }
        }
        _ => return Err("the run request holds no gates list".into()),
    }
    let timeout = match document.get("timeout") {
        Some(JsonValue::Int(text)) => text
            .parse::<f64>()
            .map_err(|_| "the run request timeout is no number".to_string())?,
        Some(JsonValue::Float(value)) => *value,
        _ => return Err("the run request holds no timeout number".into()),
    };
    let work_root = match document.get("work_root") {
        Some(JsonValue::Null) | None => None,
        Some(JsonValue::Str(text)) => Some(text.clone()),
        _ => return Err("the run request work root is no text and no null".into()),
    };
    let keep_scratch = match document.get("keep_scratch") {
        Some(JsonValue::Bool(value)) => *value,
        _ => return Err("the run request holds no keep scratch truth".into()),
    };
    let plant_mode = document
        .get("plant_mode")
        .and_then(JsonValue::as_str)
        .ok_or("the run request holds no plant mode text")?
        .to_string();
    Ok(RunRequest {
        target,
        gates,
        timeout,
        work_root,
        keep_scratch,
        plant_mode,
    })
}

/// The spelling pathlib leaves a path in. Redundant separators
/// collapse, single dot parts drop, double dot parts stand, a
/// trailing separator drops, exactly two leading slashes stand
/// where two were written, and the empty text reads as one dot.
pub fn normalize_path(text: &str) -> String {
    let slashes = text.chars().take_while(|c| *c == '/').count();
    let parts: Vec<&str> = text
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect();
    if parts.is_empty() {
        if slashes == 2 {
            return "//".to_string();
        }
        if slashes > 0 {
            return "/".to_string();
        }
        return ".".to_string();
    }
    let prefix = if slashes == 2 {
        "//"
    } else if slashes > 0 {
        "/"
    } else {
        ""
    };
    format!("{}{}", prefix, parts.join("/"))
}

/// Resolve a path the way Path.resolve resolves it, symlinks
/// followed for the parts that stand and the missing tail
/// appended in its lexical spelling with dot parts settled.
pub fn resolve_path(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join(path)
    };
    if let Ok(real) = std::fs::canonicalize(&absolute) {
        return real;
    }
    let mut ancestor = absolute.clone();
    let mut tail: Vec<std::ffi::OsString> = Vec::new();
    while !ancestor.exists() {
        match ancestor.file_name() {
            Some(name) => {
                tail.push(name.to_os_string());
                ancestor.pop();
            }
            None => break,
        }
    }
    let mut resolved = std::fs::canonicalize(&ancestor).unwrap_or(ancestor);
    for part in tail.iter().rev() {
        let text = part.to_string_lossy();
        if text == ".." {
            resolved.pop();
        } else if text != "." {
            resolved.push(part);
        }
    }
    resolved
}

fn display(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

/// Map each file under root to the SHA-256 of its bytes, the
/// walk of PORT5-5.
pub fn hash_tree(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut found: Vec<(Vec<String>, PathBuf)> = Vec::new();
    collect_files(root, root, &mut found)?;
    found.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = BTreeMap::new();
    for (parts, path) in found {
        if parts
            .iter()
            .any(|part| IGNORED_PARTS.contains(&part.as_str()))
        {
            continue;
        }
        let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
        result.insert(parts.join("/"), sha256_hex(&bytes));
    }
    Ok(result)
}

fn collect_files(
    root: &Path,
    dir: &Path,
    found: &mut Vec<(Vec<String>, PathBuf)>,
) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|error| error.to_string())?;
    let mut paths: Vec<PathBuf> = entries
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(|entry| entry.path())
        .collect();
    paths.sort();
    for path in paths {
        let metadata = std::fs::metadata(&path).map_err(|error| error.to_string())?;
        if metadata.is_dir() {
            collect_files(root, &path, found)?;
        } else if metadata.is_file() {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            let parts: Vec<String> = relative
                .components()
                .map(|part| part.as_os_str().to_string_lossy().into_owned())
                .collect();
            found.push((parts, path));
        }
    }
    Ok(())
}

/// Copy a tree the way the Python's copytree copies it with
/// symlinks standing, ignored names skipped at every level,
/// and permission bits carried from the source.
pub fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    std::fs::create_dir_all(destination).map_err(|error| error.to_string())?;
    copy_dir(source, destination)?;
    carry_permissions(source, destination)
}

fn copy_dir(source: &Path, destination: &Path) -> Result<(), String> {
    let entries = std::fs::read_dir(source).map_err(|error| error.to_string())?;
    let mut paths: Vec<PathBuf> = entries
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(|entry| entry.path())
        .collect();
    paths.sort();
    for path in paths {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        if IGNORED_PARTS.contains(&name.as_str()) {
            continue;
        }
        let target = destination.join(&name);
        let link = std::fs::symlink_metadata(&path).map_err(|error| error.to_string())?;
        if link.file_type().is_symlink() {
            let points = std::fs::read_link(&path).map_err(|error| error.to_string())?;
            #[cfg(unix)]
            std::os::unix::fs::symlink(&points, &target).map_err(|error| error.to_string())?;
            #[cfg(not(unix))]
            return Err("a symbolic link stands where the port cannot copy it".into());
        } else if link.is_dir() {
            std::fs::create_dir_all(&target).map_err(|error| error.to_string())?;
            copy_dir(&path, &target)?;
            carry_permissions(&path, &target)?;
        } else if link.is_file() {
            std::fs::copy(&path, &target).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn carry_permissions(source: &Path, destination: &Path) -> Result<(), String> {
    let metadata = std::fs::metadata(source).map_err(|error| error.to_string())?;
    std::fs::set_permissions(destination, metadata.permissions()).map_err(|error| error.to_string())
}

/// Run one gate through the shell in the named directory with
/// its output discarded, answering its exit code, and answering
/// -1 for a gate killed at its timeout.
fn run_gate(command: &str, cwd: &Path, timeout: f64) -> i32 {
    let spawned = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(cwd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    let mut child = match spawned {
        Ok(child) => child,
        Err(_) => return -1,
    };
    let deadline = Instant::now() + Duration::from_secs_f64(timeout.max(0.0));
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if let Some(code) = status.code() {
                    return code;
                }
                #[cfg(unix)]
                {
                    use std::os::unix::process::ExitStatusExt;
                    if let Some(signal) = status.signal() {
                        return -signal;
                    }
                }
                return -1;
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return -1;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(_) => return -1,
        }
    }
}

/// Plant one seed through the crate planting, carrying a
/// refusal in the wording the Python's bridge raises.
fn plant(root: &Path, seed_id: &str, skipped: &mut Vec<String>) -> Result<Vec<String>, String> {
    let mut met: Vec<String> = Vec::new();
    match plant_seed(root, seed_id, Some(&mut met)) {
        Ok(planted) => {
            for name in met {
                if !skipped.contains(&name) {
                    skipped.push(name);
                }
            }
            Ok(planted)
        }
        Err(PlantError::Refusal(refusal)) => Err(format!(
            "the canaryfield binary refused the planting: {}",
            refusal.message
        )),
        Err(PlantError::Io(error)) => Err(format!(
            "the canaryfield binary refused the planting: {error}"
        )),
    }
}

fn text(value: &str) -> JsonValue {
    JsonValue::Str(value.to_string())
}

fn int(value: i64) -> JsonValue {
    JsonValue::Int(value.to_string())
}

fn exits_object(exits: &[(String, i32)]) -> JsonValue {
    JsonValue::Object(
        exits
            .iter()
            .map(|(name, code)| (name.clone(), int(*code as i64)))
            .collect(),
    )
}

fn summary_object(rows: &[JsonValue], gates: &[(String, String)]) -> JsonValue {
    let mut summary = Vec::new();
    for (name, _) in gates {
        let mut counts = [0i64; 4];
        for row in rows {
            if let Some(JsonValue::Str(cell)) = row.get("gates").and_then(|cells| cells.get(name)) {
                match cell.as_str() {
                    CAUGHT => counts[0] += 1,
                    WAVED => counts[1] += 1,
                    NOT_APPLICABLE => counts[2] += 1,
                    GATE_BROKEN => counts[3] += 1,
                    _ => {}
                }
            }
        }
        summary.push((
            name.clone(),
            JsonValue::Object(vec![
                (CAUGHT.to_string(), int(counts[0])),
                (WAVED.to_string(), int(counts[1])),
                (NOT_APPLICABLE.to_string(), int(counts[2])),
                (GATE_BROKEN.to_string(), int(counts[3])),
            ]),
        ));
    }
    JsonValue::Object(summary)
}

fn row_object(
    seed_id: &str,
    seed_name: &str,
    defect_class: &str,
    planted_files: &[String],
    branch: Option<Option<String>>,
    planted_exits: &[(String, i32)],
    cells: &[(String, String)],
) -> JsonValue {
    let mut row = vec![
        ("seed".to_string(), text(seed_id)),
        ("name".to_string(), text(seed_name)),
        ("defect_class".to_string(), text(defect_class)),
        (
            "planted_file".to_string(),
            match planted_files.first() {
                Some(first) => text(first),
                None => JsonValue::Null,
            },
        ),
        (
            "planted_files".to_string(),
            JsonValue::Array(planted_files.iter().map(|name| text(name)).collect()),
        ),
    ];
    if let Some(branch) = branch {
        row.push((
            "branch".to_string(),
            match branch {
                Some(name) => text(&name),
                None => JsonValue::Null,
            },
        ));
    }
    row.push(("planted_exits".to_string(), exits_object(planted_exits)));
    row.push((
        "gates".to_string(),
        JsonValue::Object(
            cells
                .iter()
                .map(|(name, cell)| (name.clone(), text(cell)))
                .collect(),
        ),
    ));
    JsonValue::Object(row)
}

/// The exits a planted run recorded and the cells it judged.
type GateOutcome = (Vec<(String, i32)>, Vec<(String, String)>);

fn run_gates_for_rows(
    gates: &[(String, String)],
    clean_exits: &[(String, i32)],
    planted_files: &[String],
    cwd: &Path,
    timeout: f64,
) -> GateOutcome {
    let mut planted_exits = Vec::new();
    let mut cells = Vec::new();
    for (name, command) in gates {
        if planted_files.is_empty() {
            cells.push((name.clone(), NOT_APPLICABLE.to_string()));
            continue;
        }
        let clean = clean_exits
            .iter()
            .find(|(gate, _)| gate == name)
            .map(|(_, code)| *code)
            .unwrap_or(0);
        if clean != 0 {
            cells.push((name.clone(), GATE_BROKEN.to_string()));
            continue;
        }
        let exit = run_gate(command, cwd, timeout);
        planted_exits.push((name.clone(), exit));
        cells.push((
            name.clone(),
            if exit != 0 {
                CAUGHT.to_string()
            } else {
                WAVED.to_string()
            },
        ));
    }
    (planted_exits, cells)
}

fn clean_exits_for(gates: &[(String, String)], cwd: &Path, timeout: f64) -> Vec<(String, i32)> {
    gates
        .iter()
        .map(|(name, command)| (name.clone(), run_gate(command, cwd, timeout)))
        .collect()
}

const GIT_IDENTITY: [(&str, &str); 4] = [
    ("GIT_AUTHOR_NAME", "canaryfield"),
    ("GIT_AUTHOR_EMAIL", "canaryfield@example.invalid"),
    ("GIT_COMMITTER_NAME", "canaryfield"),
    ("GIT_COMMITTER_EMAIL", "canaryfield@example.invalid"),
];

struct GitOut {
    ok: bool,
    stdout: String,
    stderr: String,
}

fn git_raw(cwd: &Path, args: &[String], identity: bool, dash_c: bool) -> GitOut {
    let mut command = Command::new("git");
    if dash_c {
        command.arg("-C").arg(display(cwd));
    }
    command.args(args);
    if identity {
        for (key, value) in GIT_IDENTITY {
            command.env(key, value);
        }
    }
    match command.output() {
        Ok(out) => GitOut {
            ok: out.status.success(),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        },
        Err(error) => GitOut {
            ok: false,
            stdout: String::new(),
            stderr: error.to_string(),
        },
    }
}

/// The Python's git helper. A failing call is refused in the
/// standing wording carrying the call's error text trimmed.
fn git(cwd: &Path, args: &[&str]) -> Result<String, String> {
    let owned: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    let out = git_raw(cwd, &owned, true, true);
    if !out.ok {
        return Err(format!(
            "git {} failed in {}: {}",
            owned.join(" "),
            display(cwd),
            out.stderr.trim()
        ));
    }
    Ok(out.stdout.trim().to_string())
}

fn clean_caches(root: &Path) {
    let _ = std::fs::remove_dir_all(root.join(".pytest_cache"));
    let mut dirs = Vec::new();
    collect_dirs(root, &mut dirs);
    for dir in dirs {
        let parts: Vec<String> = dir
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect();
        if dir
            .file_name()
            .map(|name| name == "__pycache__")
            .unwrap_or(false)
            && !parts.iter().any(|part| part == ".git")
        {
            let _ = std::fs::remove_dir_all(&dir);
        }
    }
}

fn collect_dirs(dir: &Path, found: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                found.push(path.clone());
                collect_dirs(&path, found);
            }
        }
    }
}

fn restore_checkout(clone: &Path, base: &str) -> Result<(), String> {
    git(clone, &["reset", "-q", "--hard", "HEAD"])?;
    git(clone, &["clean", "-q", "-fd"])?;
    git(clone, &["checkout", "-q", base])?;
    git(clone, &["reset", "-q", "--hard", base])?;
    Ok(())
}

struct RunOutcome {
    clean_exits: Vec<(String, i32)>,
    rows: Vec<JsonValue>,
    skipped: Vec<String>,
    git_mode: bool,
}

fn run_copy(request: &RunRequest, target: &Path, scratch: &Path) -> Result<RunOutcome, String> {
    let clean = scratch.join("clean");
    copy_tree(target, &clean)?;
    let clean_exits = clean_exits_for(&request.gates, &clean, request.timeout);
    let mut rows = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for seed in SEEDS {
        let planted_dir = scratch.join(seed.id);
        copy_tree(target, &planted_dir)?;
        let planted_files = plant(&planted_dir, seed.id, &mut skipped)?;
        let (planted_exits, cells) = run_gates_for_rows(
            &request.gates,
            &clean_exits,
            &planted_files,
            &planted_dir,
            request.timeout,
        );
        rows.push(row_object(
            seed.id,
            seed.name,
            seed.defect_class,
            &planted_files,
            None,
            &planted_exits,
            &cells,
        ));
    }
    Ok(RunOutcome {
        clean_exits,
        rows,
        skipped,
        git_mode: false,
    })
}

fn run_git(
    request: &RunRequest,
    target: &Path,
    target_text: &str,
    scratch: &Path,
) -> Result<RunOutcome, String> {
    let probe = git_raw(
        target,
        &["rev-parse".to_string(), "--show-toplevel".to_string()],
        false,
        true,
    );
    if !probe.ok {
        return Err(format!("target is not a git repository: {target_text}"));
    }
    let toplevel = PathBuf::from(probe.stdout.trim());
    if resolve_path(&toplevel) != resolve_path(target) {
        return Err(format!(
            "target is not the root of a git repository: {target_text}"
        ));
    }
    let head = git_raw(
        target,
        &[
            "rev-parse".to_string(),
            "--verify".to_string(),
            "HEAD".to_string(),
        ],
        false,
        true,
    );
    if !head.ok {
        return Err(format!(
            "target git repository has no commits yet: {target_text}"
        ));
    }
    let clone = scratch.join("clone");
    let cloned = git_raw(
        target,
        &[
            "clone".to_string(),
            "--quiet".to_string(),
            "--no-hardlinks".to_string(),
            display(target),
            display(&clone),
        ],
        false,
        false,
    );
    if !cloned.ok {
        return Err(format!(
            "could not clone the target: {}",
            cloned.stderr.trim()
        ));
    }
    let base = git(&clone, &["rev-parse", "HEAD"])?;
    let clean_exits = clean_exits_for(&request.gates, &clone, request.timeout);
    let mut rows = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for seed in SEEDS {
        restore_checkout(&clone, &base)?;
        clean_caches(&clone);
        let planted_files = plant(&clone, seed.id, &mut skipped)?;
        let (planted_exits, cells, branch) = if planted_files.is_empty() {
            (
                Vec::new(),
                request
                    .gates
                    .iter()
                    .map(|(name, _)| (name.clone(), NOT_APPLICABLE.to_string()))
                    .collect::<Vec<(String, String)>>(),
                None,
            )
        } else {
            let branch = format!("canaryfield/{}", seed.id);
            git(&clone, &["checkout", "-q", "-b", &branch])?;
            let mut add: Vec<&str> = vec!["add", "--"];
            add.extend(planted_files.iter().map(|name| name.as_str()));
            git(&clone, &add)?;
            let message = format!("Plant {} {}", seed.id, seed.name);
            git(&clone, &["commit", "-q", "-m", &message])?;
            let (exits, cells) = run_gates_for_rows(
                &request.gates,
                &clean_exits,
                &planted_files,
                &clone,
                request.timeout,
            );
            (exits, cells, Some(branch))
        };
        rows.push(row_object(
            seed.id,
            seed.name,
            seed.defect_class,
            &planted_files,
            Some(branch),
            &planted_exits,
            &cells,
        ));
    }
    restore_checkout(&clone, &base)?;
    Ok(RunOutcome {
        clean_exits,
        rows,
        skipped,
        git_mode: true,
    })
}

fn fresh_temp_root() -> Result<PathBuf, String> {
    let base = std::env::temp_dir();
    let pid = std::process::id();
    for attempt in 0..100u32 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let candidate = base.join(format!("canaryfield-{pid}-{nanos}-{attempt}"));
        match std::fs::create_dir(&candidate) {
            Ok(()) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(
                        &candidate,
                        std::fs::Permissions::from_mode(0o700),
                    );
                }
                return Ok(candidate);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("no fresh temporary directory could be created".into())
}

/// Perform one run, the door's whole work, answering the result
/// document or the refusal's wording.
pub fn run(request: &RunRequest) -> Result<JsonValue, String> {
    let target_text = normalize_path(&request.target);
    let target = PathBuf::from(&target_text);
    if !target.is_dir() {
        return Err(format!("target is not a directory: {target_text}"));
    }
    if request.plant_mode != "copy" && request.plant_mode != "git-branches" {
        return Err(format!("unknown plant mode: {}", request.plant_mode));
    }
    let before = hash_tree(&target)?;
    let (scratch, created_root, preexisting) = match &request.work_root {
        None => (fresh_temp_root()?, true, BTreeSet::new()),
        Some(raw) => {
            let root_text = normalize_path(raw);
            let scratch = PathBuf::from(&root_text);
            let existed = scratch.exists();
            let mut preexisting = BTreeSet::new();
            if existed {
                let entries = std::fs::read_dir(&scratch).map_err(|e| e.to_string())?;
                for entry in entries {
                    let entry = entry.map_err(|e| e.to_string())?;
                    preexisting.insert(entry.file_name().to_string_lossy().into_owned());
                }
                if !preexisting.is_empty() {
                    return Err(format!(
                        "work root is not empty, refusing to use it: {root_text}"
                    ));
                }
            }
            let resolved_root = resolve_path(&scratch);
            let resolved_target = resolve_path(&target);
            if resolved_root == resolved_target || resolved_root.starts_with(&resolved_target) {
                return Err(format!(
                    "work root stands inside the target, refusing to use it: {root_text}"
                ));
            }
            if !existed {
                std::fs::create_dir_all(&scratch).map_err(|e| e.to_string())?;
            }
            (scratch, !existed, preexisting)
        }
    };
    let outcome = if request.plant_mode == "git-branches" {
        run_git(request, &target, &target_text, &scratch)
    } else {
        run_copy(request, &target, &scratch)
    };
    let assembled = outcome.and_then(|outcome| {
        let after = hash_tree(&target)?;
        let mut skipped = outcome.skipped.clone();
        skipped.sort();
        let mut pairs = vec![("target".to_string(), text(&target_text))];
        if outcome.git_mode {
            pairs.push(("plant_mode".to_string(), text("git-branches")));
        }
        pairs.push((
            "gates".to_string(),
            JsonValue::Array(request.gates.iter().map(|(name, _)| text(name)).collect()),
        ));
        pairs.push((
            "clean_exits".to_string(),
            exits_object(&outcome.clean_exits),
        ));
        pairs.push((
            "target_unchanged".to_string(),
            JsonValue::Bool(before == after),
        ));
        pairs.push((
            "unparseable_files".to_string(),
            JsonValue::Array(skipped.iter().map(|name| text(name)).collect()),
        ));
        pairs.push((
            "summary".to_string(),
            summary_object(&outcome.rows, &request.gates),
        ));
        let mut with_rows = Vec::new();
        for pair in pairs {
            if pair.0 == "summary" {
                with_rows.push(("rows".to_string(), JsonValue::Array(outcome.rows.clone())));
            }
            with_rows.push(pair);
        }
        Ok(JsonValue::Object(with_rows))
    });
    if !request.keep_scratch {
        if created_root {
            let _ = std::fs::remove_dir_all(&scratch);
        } else if let Ok(entries) = std::fs::read_dir(&scratch) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !preexisting.contains(&name) {
                    let path = entry.path();
                    if path.is_dir() && !path.is_symlink() {
                        let _ = std::fs::remove_dir_all(&path);
                    } else {
                        let _ = std::fs::remove_file(&path);
                    }
                }
            }
        }
    }
    assembled
}

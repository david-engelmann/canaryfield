//! plant_seed of PORT2-7, the planting of one seed over a
//! directory, reproducing engine.plant_seed of the landed
//! Python. The candidate walk takes every Python file under
//! the root in relative path order with the ignored parts
//! and the test file exclusions, the symlink and resolution
//! safety rules stand as the Python states them, a candidate
//! that does not read as strict UTF-8 or does not parse is
//! skipped and named once in the skipped list when one is
//! handed over, and a planted file is written as the whole
//! module rendering plus one trailing newline. The compile
//! check the Python makes with compile stands here as the
//! port's own reading of the rendered text, a reparse plus
//! the binding rule for nonlocal declarations, and a refused
//! rendering is never written, the file counting neither as
//! planted nor as skipped. Nothing guards the write itself,
//! so a write error travels out as the Python's does.

use crate::pyapply::apply_seed;
use crate::pyarena::{parse_tree, render_tree, Ctx, Data, Refusal, Tree};
use crate::pydump::dump_tree;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// A failure of the planting. A refusal is the port's own
/// wording for the families PORT2-7 names, and an io error
/// is the operating system's, propagated as the Python
/// propagates it.
#[derive(Debug)]
pub enum PlantError {
    Refusal(Refusal),
    Io(std::io::Error),
}

impl From<Refusal> for PlantError {
    fn from(error: Refusal) -> PlantError {
        PlantError::Refusal(error)
    }
}

impl From<std::io::Error> for PlantError {
    fn from(error: std::io::Error) -> PlantError {
        PlantError::Io(error)
    }
}

const IGNORED_PARTS: [&str; 3] = [".git", "__pycache__", ".pytest_cache"];

fn is_test_file(relative: &Path) -> bool {
    let name = relative
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    if name == "conftest.py" || name == "test.py" {
        return true;
    }
    if name.starts_with("test_") || name.ends_with("_test.py") {
        return true;
    }
    relative.components().any(|part| {
        let part = part.as_os_str().to_string_lossy();
        part == "tests" || part == "test" || part == "testing"
    })
}

/// Plant one seed over a directory. Returns the planted
/// files as relative path strings in order. When a skipped
/// list is handed over, each unreadable or unparseable
/// candidate is named in it once.
pub fn plant_seed(
    root: &Path,
    seed_id: &str,
    mut skipped: Option<&mut Vec<String>>,
) -> Result<Vec<String>, PlantError> {
    if !crate::registry::SEEDS.iter().any(|seed| seed.id == seed_id) {
        return Err(PlantError::Refusal(crate::pyarena::refusal(
            "the seed id stands outside the registry",
        )));
    }
    let resolved_root = match root.canonicalize() {
        Ok(resolved) => resolved,
        Err(_) => std::path::absolute(root).unwrap_or_else(|_| root.to_path_buf()),
    };
    let mut candidates: Vec<PathBuf> = Vec::new();
    collect_candidates(root, root, &mut candidates);
    let mut relatives: Vec<(String, PathBuf)> = candidates
        .into_iter()
        .map(|path| {
            let relative = path
                .strip_prefix(root)
                .expect("a candidate stands under the root")
                .to_path_buf();
            (relative.to_string_lossy().replace('\\', "/"), path)
        })
        .collect();
    relatives.sort_by(|a, b| a.0.cmp(&b.0));
    let mut planted: Vec<String> = Vec::new();
    for (relative, path) in &relatives {
        let relative_path = Path::new(relative);
        if is_test_file(relative_path) {
            continue;
        }
        if relative_path
            .components()
            .any(|part| IGNORED_PARTS.contains(&part.as_os_str().to_string_lossy().as_ref()))
        {
            continue;
        }
        if path.is_symlink() {
            continue;
        }
        let resolved = match path.canonicalize() {
            Ok(resolved) => resolved,
            Err(_) => continue,
        };
        if resolved != resolved_root && !resolved.starts_with(&resolved_root) {
            continue;
        }
        let bytes = std::fs::read(path)?;
        let text = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => {
                note_skipped(&mut skipped, relative);
                continue;
            }
        };
        // The text reading of the Python translates the
        // universal newlines, as Path.read_text does.
        let text = text.replace("\r\n", "\n").replace('\r', "\n");
        let mut tree = match parse_tree(&text) {
            Ok(tree) => tree,
            Err(_) => {
                note_skipped(&mut skipped, relative);
                continue;
            }
        };
        let before = dump_tree(&tree)?;
        if apply_seed(&mut tree, seed_id)? && dump_tree(&tree)? != before {
            let rendered = render_tree(&tree)?;
            if !compile_accepts(&rendered) {
                continue;
            }
            std::fs::write(path, format!("{rendered}\n"))?;
            planted.push(relative.clone());
        }
    }
    planted.sort();
    Ok(planted)
}

fn note_skipped(skipped: &mut Option<&mut Vec<String>>, relative: &str) {
    if let Some(list) = skipped.as_deref_mut() {
        if !list.iter().any(|entry| entry == relative) {
            list.push(relative.to_string());
        }
    }
}

/// The recursive candidate walk, every .py path under the
/// directory, files and links to files alike, the filtering
/// standing in the caller.
fn collect_candidates(dir: &Path, _root: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .collect();
    paths.sort();
    for path in paths {
        let is_dir = path.is_dir();
        if is_dir {
            collect_candidates(&path, _root, out);
        } else if path.extension().map(|ext| ext == "py").unwrap_or(false) && path.is_file() {
            out.push(path);
        }
    }
}

/// The compile check over a rendering. The Python hands the
/// rendering to compile, which reads it as a whole program.
/// The port reads it with its own parser and then applies
/// the binding rule for nonlocal declarations, the refusal
/// family the round record carries. A rendering either
/// reading refuses is refused here.
pub fn compile_accepts(rendered: &str) -> bool {
    match parse_tree(rendered) {
        Ok(tree) => nonlocal_accepts(&tree),
        Err(_) => false,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScopeKind {
    Module,
    Function,
    Class,
}

struct ScopeInfo {
    /// Names this scope binds, the binding reading.
    bound: HashSet<String>,
    /// Names this scope declares global.
    globals: HashSet<String>,
    /// Names this scope declares nonlocal.
    nonlocals: Vec<String>,
}

/// The binding reading of one scope body, the names bound
/// in it. Parameters stand first, then every store name in
/// the body, never crossing a nested function or class body
/// and never counting a comprehension target, which binds
/// in the comprehension alone. A walrus target inside a
/// comprehension still binds here, as the interpreter's
/// symbol table counts it.
fn scope_bindings(tree: &Tree, scope: usize, kind: ScopeKind) -> ScopeInfo {
    let mut info = ScopeInfo {
        bound: HashSet::new(),
        globals: HashSet::new(),
        nonlocals: Vec::new(),
    };
    if kind == ScopeKind::Function {
        if let Some(args_node) = tree.one(scope, "args") {
            let mut params: Vec<usize> = Vec::new();
            params.extend(tree.many(args_node, "posonlyargs").iter().copied());
            params.extend(tree.many(args_node, "args").iter().copied());
            if let Some(vararg) = tree.one(args_node, "vararg") {
                params.push(vararg);
            }
            params.extend(tree.many(args_node, "kwonlyargs").iter().copied());
            if let Some(kwarg) = tree.one(args_node, "kwarg") {
                params.push(kwarg);
            }
            for param in params {
                if let Data::Arg { name } = &tree.nodes[param].data {
                    info.bound.insert(name.clone());
                }
            }
        }
    }
    let body: Vec<usize> = if matches!(&tree.nodes[scope].data, Data::Lambda) {
        tree.one(scope, "body").into_iter().collect()
    } else {
        tree.many(scope, "body").to_vec()
    };
    let mut stack: Vec<usize> = body;
    while let Some(current) = stack.pop() {
        match &tree.nodes[current].data {
            Data::FuncDef { name, .. } => {
                // The definition binds its name here, and
                // its header is read in this scope, the
                // decorators and the defaults and the
                // annotations, because a walrus standing in
                // one of them binds here. The body is a
                // scope of its own and is never entered.
                info.bound.insert(name.clone());
                stack.extend(tree.many(current, "decorator_list").iter().copied());
                if let Some(args) = tree.one(current, "args") {
                    stack.push(args);
                }
                if let Some(returns) = tree.one(current, "returns") {
                    stack.push(returns);
                }
                continue;
            }
            Data::ClassDef { name } => {
                // The class binds its name here, and its
                // header is read in this scope on the same
                // ground, the decorators and the bases and
                // the keyword values, a walrus in a base
                // binding here. The body is never entered.
                info.bound.insert(name.clone());
                stack.extend(tree.many(current, "decorator_list").iter().copied());
                stack.extend(tree.many(current, "bases").iter().copied());
                stack.extend(tree.many(current, "keywords").iter().copied());
                continue;
            }
            Data::Lambda => {
                // A lambda binds nothing here, but its
                // defaults are read in this scope, so the
                // arguments are walked and the body is not.
                if let Some(args) = tree.one(current, "args") {
                    stack.push(args);
                }
                continue;
            }
            Data::Name {
                id,
                ctx: Ctx::Store,
            } => {
                info.bound.insert(id.clone());
            }
            Data::Import => {
                for alias in tree.many(current, "names") {
                    if let Data::Alias { name, asname } = &tree.nodes[*alias].data {
                        info.bound.insert(asname.clone().unwrap_or_else(|| {
                            name.split('.')
                                .next()
                                .expect("a segment stands")
                                .to_string()
                        }));
                    }
                }
            }
            Data::ImportFrom { .. } => {
                for alias in tree.many(current, "names") {
                    if let Data::Alias { name, asname } = &tree.nodes[*alias].data {
                        info.bound
                            .insert(asname.clone().unwrap_or_else(|| name.clone()));
                    }
                }
            }
            Data::ExceptHandler { name: Some(name) } => {
                info.bound.insert(name.clone());
            }
            Data::Global { names } => {
                for name in names {
                    info.globals.insert(name.clone());
                }
            }
            Data::Nonlocal { names } => {
                for name in names {
                    info.nonlocals.push(name.clone());
                }
            }
            _ => {}
        }
        if matches!(&tree.nodes[current].data, Data::Comprehension { .. }) {
            // A comprehension target binds in the
            // comprehension alone, so the walk takes the
            // iter and the ifs and leaves the target.
            if let Some(iter) = tree.one(current, "iter") {
                stack.push(iter);
            }
            stack.extend(tree.many(current, "ifs").iter().copied());
            continue;
        }
        let mut children = tree.child_ids(current);
        stack.append(&mut children);
    }
    info
}

/// The kind of a node as a scope, when the node is one.
fn scope_kind(tree: &Tree, id: usize) -> Option<ScopeKind> {
    match &tree.nodes[id].data {
        Data::Module => Some(ScopeKind::Module),
        Data::FuncDef { .. } | Data::Lambda => Some(ScopeKind::Function),
        Data::ClassDef { .. } => Some(ScopeKind::Class),
        _ => None,
    }
}

/// The nonlocal rule over the whole tree. Every nonlocal
/// declaration of a function scope must name a binding of
/// an enclosing function scope, class scopes passed over
/// and the module never a binding, and a name an enclosing
/// function declares global blocks the reading, as the
/// interpreter's symbol table refuses them. A nonlocal at
/// module level is refused outright.
fn nonlocal_accepts(tree: &Tree) -> bool {
    let parents = tree.parent_map();
    let module_info = scope_bindings(tree, tree.root, ScopeKind::Module);
    if !module_info.nonlocals.is_empty() {
        return false;
    }
    for scope in tree.walk() {
        if scope_kind(tree, scope) != Some(ScopeKind::Function) {
            continue;
        }
        // A lambda holds no statements and declares nothing.
        if matches!(&tree.nodes[scope].data, Data::Lambda) {
            continue;
        }
        let info = scope_bindings(tree, scope, ScopeKind::Function);
        for name in &info.nonlocals {
            if info.globals.contains(name) {
                return false;
            }
            let mut resolved = false;
            let mut cursor = parents.get(&scope).copied();
            while let Some(at) = cursor {
                cursor = parents.get(&at).copied();
                match scope_kind(tree, at) {
                    Some(ScopeKind::Class) => continue,
                    Some(ScopeKind::Module) => break,
                    Some(ScopeKind::Function) => {
                        let outer = scope_bindings(tree, at, ScopeKind::Function);
                        if outer.globals.contains(name) {
                            return false;
                        }
                        if outer.bound.contains(name) {
                            resolved = true;
                            break;
                        }
                    }
                    None => {}
                }
            }
            if !resolved {
                return false;
            }
        }
    }
    true
}

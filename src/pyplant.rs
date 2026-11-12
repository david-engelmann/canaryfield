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
//! the compile readings whole, and a refused
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

/// The recursive candidate walk, every path under the
/// directory whose name ends in .py, the bare name .py
/// itself included, files and links to files alike, the
/// filtering standing in the caller. The walk reads the
/// tree the way the Python glob reading does, so it never
/// descends into a directory reached through a link, and
/// a file under a linked directory is reached only under
/// its real path.
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
        if path.is_dir() && !path.is_symlink() {
            collect_candidates(&path, _root, out);
        } else if path
            .file_name()
            .map(|name| name.to_string_lossy().ends_with(".py"))
            .unwrap_or(false)
            && path.is_file()
        {
            out.push(path);
        }
    }
}

/// The compile check over a rendering. The Python hands the
/// rendering to compile, which reads it as a whole program.
/// The port reads it with its own parser and then applies
/// the compile readings whole, the binding rule for
/// nonlocal declarations wherever they stand and the
/// symbol table family one scope at a time, the statement
/// grounds and the declaration order among them. A
/// rendering any reading refuses is refused here.
pub fn compile_accepts(rendered: &str) -> bool {
    match parse_tree(rendered) {
        Ok(tree) => crate::pyarena::run_big_stack(move || {
            nonlocal_accepts(&tree) && symbol_accepts(&tree)
        }),
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
            Data::MatchAs { name: Some(name) } => {
                // A match capture binds in this scope, as
                // the interpreter's symbol table counts it,
                // the pattern standing in the body like any
                // other binding statement.
                info.bound.insert(name.clone());
            }
            Data::MatchStar { name: Some(name) } => {
                info.bound.insert(name.clone());
            }
            Data::MatchMapping { rest: Some(name) } => {
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

/// The resolution of one nonlocal name standing in the
/// scope at `scope`. The reading walks the enclosing
/// scopes, class scopes passed over and the module never
/// a binding, and a name an enclosing function declares
/// global blocks the reading outright.
enum Resolution {
    Resolved,
    Unresolved,
    Blocked,
}

fn nonlocal_resolution(
    tree: &Tree,
    parents: &std::collections::HashMap<usize, usize>,
    scope: usize,
    name: &str,
) -> Resolution {
    let mut cursor = parents.get(&scope).copied();
    while let Some(at) = cursor {
        cursor = parents.get(&at).copied();
        match scope_kind(tree, at) {
            Some(ScopeKind::Class) => continue,
            Some(ScopeKind::Module) => break,
            Some(ScopeKind::Function) => {
                let outer = scope_bindings(tree, at, ScopeKind::Function);
                if outer.globals.contains(name) {
                    return Resolution::Blocked;
                }
                if outer.bound.contains(name) {
                    return Resolution::Resolved;
                }
            }
            None => {}
        }
    }
    Resolution::Unresolved
}

/// The nonlocal rule over the whole tree. Every nonlocal
/// declaration, of a function scope or of a class body
/// alike, must name a binding of an enclosing function
/// scope, the binding reading passing class scopes over
/// while the declaration reading validates them where
/// they stand, as the interpreter's symbol table does.
/// A nonlocal at module level is refused outright.
fn nonlocal_accepts(tree: &Tree) -> bool {
    let parents = tree.parent_map();
    let module_info = scope_bindings(tree, tree.root, ScopeKind::Module);
    if !module_info.nonlocals.is_empty() {
        return false;
    }
    for scope in tree.walk() {
        let kind = match scope_kind(tree, scope) {
            Some(ScopeKind::Function) => {
                // A lambda holds no statements and declares nothing.
                if matches!(&tree.nodes[scope].data, Data::Lambda) {
                    continue;
                }
                ScopeKind::Function
            }
            Some(ScopeKind::Class) => ScopeKind::Class,
            _ => continue,
        };
        let info = scope_bindings(tree, scope, kind);
        for name in &info.nonlocals {
            if info.globals.contains(name) {
                return false;
            }
            match nonlocal_resolution(tree, &parents, scope, name) {
                Resolution::Resolved => {}
                Resolution::Unresolved | Resolution::Blocked => return false,
            }
        }
    }
    true
}

/// The argument names of one definition, in signature
/// order across every parameter list.
fn argument_names(tree: &Tree, args_node: usize) -> Vec<String> {
    let mut all: Vec<usize> = Vec::new();
    all.extend(tree.many(args_node, "posonlyargs").iter().copied());
    all.extend(tree.many(args_node, "args").iter().copied());
    if let Some(vararg) = tree.one(args_node, "vararg") {
        all.push(vararg);
    }
    all.extend(tree.many(args_node, "kwonlyargs").iter().copied());
    if let Some(kwarg) = tree.one(args_node, "kwarg") {
        all.push(kwarg);
    }
    let mut names: Vec<String> = Vec::new();
    for arg in all {
        if let Data::Arg { name } = &tree.nodes[arg].data {
            names.push(name.clone());
        }
    }
    names
}

/// One scope's ordered reading, the events the symbol
/// table counts as it walks the body in statement order.
/// A binding or a use recorded before a declaration of
/// the same name refuses the scope, a parameter takes no
/// declaration, a name takes only one of the two
/// declarations, and an annotated name takes neither, the
/// interpreter's readings stated as one walk. An import
/// records no event, as the interpreter's declaration
/// readings pass an imported name by.
struct ScopeWalk {
    kind: ScopeKind,
    /// The scope is an async function, the one ground an
    /// await and the asynchronous forms stand on.
    is_async_fn: bool,
    params: HashSet<String>,
    bound: HashSet<String>,
    used: HashSet<String>,
    annotated: HashSet<String>,
    declared_global: HashSet<String>,
    declared_nonlocal: HashSet<String>,
    ok: bool,
}

impl ScopeWalk {
    fn note_bound(&mut self, name: &str) {
        self.bound.insert(name.to_string());
    }
}

/// The symbol table family over the whole tree. Every
/// definition's argument names stand unique, and every
/// scope's own statements pass the ordered reading.
fn symbol_accepts(tree: &Tree) -> bool {
    for id in tree.walk() {
        if matches!(&tree.nodes[id].data, Data::FuncDef { .. } | Data::Lambda) {
            if let Some(args) = tree.one(id, "args") {
                let names = argument_names(tree, args);
                let mut seen: HashSet<&String> = HashSet::new();
                for name in &names {
                    if !seen.insert(name) {
                        return false;
                    }
                }
            }
        }
    }
    for id in tree.walk() {
        if let Some(kind) = scope_kind(tree, id) {
            if !scope_accepts(tree, id, kind) {
                return false;
            }
        }
    }
    true
}

fn scope_accepts(tree: &Tree, scope: usize, kind: ScopeKind) -> bool {
    let is_async_fn = matches!(&tree.nodes[scope].data, Data::FuncDef { is_async: true, .. });
    let mut walk = ScopeWalk {
        kind,
        is_async_fn,
        params: HashSet::new(),
        bound: HashSet::new(),
        used: HashSet::new(),
        annotated: HashSet::new(),
        declared_global: HashSet::new(),
        declared_nonlocal: HashSet::new(),
        ok: true,
    };
    if kind == ScopeKind::Function {
        if let Some(args) = tree.one(scope, "args") {
            walk.params = argument_names(tree, args).into_iter().collect();
        }
    }
    if matches!(&tree.nodes[scope].data, Data::Lambda) {
        if let Some(body) = tree.one(scope, "body") {
            visit_node(tree, &mut walk, body, 0, 0, false);
        }
    } else {
        for statement in tree.many(scope, "body").to_vec() {
            visit_node(tree, &mut walk, statement, 0, 0, false);
        }
    }
    if !walk.ok {
        return false;
    }
    // An annotated name takes no declaration of either
    // kind, in either order, as the interpreter refuses
    // the pairing wherever it stands in the scope.
    !walk.annotated.iter().any(|name| {
        walk.declared_global.contains(name) || walk.declared_nonlocal.contains(name)
    })
}

fn visit_children(
    tree: &Tree,
    walk: &mut ScopeWalk,
    id: usize,
    loops: usize,
    in_comp: usize,
    limited: bool,
) {
    for child in tree.child_ids(id) {
        visit_node(tree, walk, child, loops, in_comp, limited);
    }
}

/// The argument readings of a definition standing in
/// this scope, the annotations and the defaults, which
/// the interpreter reads here and not in the definition's
/// own scope.
fn visit_arguments(
    tree: &Tree,
    walk: &mut ScopeWalk,
    args: usize,
    loops: usize,
    in_comp: usize,
    limited: bool,
) {
    visit_children(tree, walk, args, loops, in_comp, limited);
}

/// The ordered walk of one node standing in the scope
/// under reading. A nested definition or class is read
/// by its header alone, its body a scope of its own the
/// outer reading never enters. The limited reading walks
/// a comprehension's interior, where a name load and a
/// name store belong to the comprehension alone and only
/// a walrus target binds here.
fn visit_node(
    tree: &Tree,
    walk: &mut ScopeWalk,
    id: usize,
    loops: usize,
    in_comp: usize,
    limited: bool,
) {
    if !walk.ok {
        return;
    }
    match &tree.nodes[id].data {
        Data::FuncDef { name, .. } => {
            walk.note_bound(name);
            for child in tree.many(id, "decorator_list").iter().copied() {
                visit_node(tree, walk, child, loops, in_comp, limited);
            }
            if let Some(args) = tree.one(id, "args") {
                visit_arguments(tree, walk, args, loops, in_comp, limited);
            }
            if let Some(returns) = tree.one(id, "returns") {
                visit_node(tree, walk, returns, loops, in_comp, limited);
            }
        }
        Data::ClassDef { name } => {
            walk.note_bound(name);
            for child in tree.many(id, "decorator_list").iter().copied() {
                visit_node(tree, walk, child, loops, in_comp, limited);
            }
            for child in tree.many(id, "bases").iter().copied() {
                visit_node(tree, walk, child, loops, in_comp, limited);
            }
            for child in tree.many(id, "keywords").iter().copied() {
                visit_node(tree, walk, child, loops, in_comp, limited);
            }
        }
        Data::Lambda => {
            if let Some(args) = tree.one(id, "args") {
                visit_arguments(tree, walk, args, loops, in_comp, limited);
            }
        }
        Data::ListComp | Data::SetComp | Data::DictComp | Data::GeneratorExp => {
            let generators: Vec<usize> = tree.many(id, "generators").to_vec();
            for generator in &generators {
                if let Data::Comprehension { is_async: true } = &tree.nodes[*generator].data {
                    if !walk.is_async_fn {
                        walk.ok = false;
                    }
                }
            }
            // The outermost iterable is read in this scope,
            // a yield standing in it on this scope's ground.
            // Everything else in the comprehension is its
            // interior, read under the limited reading at
            // one more comprehension depth.
            if let Some(first) = generators.first() {
                if let Some(iter) = tree.one(*first, "iter") {
                    visit_node(tree, walk, iter, loops, in_comp, limited);
                }
            }
            for (index, generator) in generators.iter().enumerate() {
                if let Some(target) = tree.one(*generator, "target") {
                    visit_node(tree, walk, target, loops, in_comp + 1, true);
                }
                if index > 0 {
                    if let Some(iter) = tree.one(*generator, "iter") {
                        visit_node(tree, walk, iter, loops, in_comp + 1, true);
                    }
                }
                for cond in tree.many(*generator, "ifs").iter().copied() {
                    visit_node(tree, walk, cond, loops, in_comp + 1, true);
                }
            }
            for field in ["elt", "key", "value"] {
                if let Some(child) = tree.one(id, field) {
                    visit_node(tree, walk, child, loops, in_comp + 1, true);
                }
            }
        }
        Data::For { is_async } => {
            if *is_async && !walk.is_async_fn {
                walk.ok = false;
            }
            if let Some(target) = tree.one(id, "target") {
                visit_node(tree, walk, target, loops, in_comp, limited);
            }
            if let Some(iter) = tree.one(id, "iter") {
                visit_node(tree, walk, iter, loops, in_comp, limited);
            }
            for statement in tree.many(id, "body").to_vec() {
                visit_node(tree, walk, statement, loops + 1, in_comp, limited);
            }
            for statement in tree.many(id, "orelse").to_vec() {
                visit_node(tree, walk, statement, loops, in_comp, limited);
            }
        }
        Data::While => {
            if let Some(test) = tree.one(id, "test") {
                visit_node(tree, walk, test, loops, in_comp, limited);
            }
            for statement in tree.many(id, "body").to_vec() {
                visit_node(tree, walk, statement, loops + 1, in_comp, limited);
            }
            for statement in tree.many(id, "orelse").to_vec() {
                visit_node(tree, walk, statement, loops, in_comp, limited);
            }
        }
        Data::With { is_async } => {
            if *is_async && !walk.is_async_fn {
                walk.ok = false;
            }
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
        Data::Return => {
            if walk.kind != ScopeKind::Function {
                walk.ok = false;
            }
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
        Data::Break | Data::Continue => {
            if loops == 0 {
                walk.ok = false;
            }
        }
        Data::Yield | Data::YieldFrom => {
            if in_comp > 0 || walk.kind != ScopeKind::Function {
                walk.ok = false;
            }
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
        Data::Await => {
            if !walk.is_async_fn {
                walk.ok = false;
            }
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
        Data::Import => {}
        Data::ImportFrom { .. } => {
            if walk.kind != ScopeKind::Module {
                for alias in tree.many(id, "names") {
                    if let Data::Alias { name, .. } = &tree.nodes[*alias].data {
                        if name == "*" {
                            walk.ok = false;
                        }
                    }
                }
            }
        }
        Data::Global { names } => {
            for name in names {
                if walk.params.contains(name)
                    || walk.declared_nonlocal.contains(name)
                    || walk.bound.contains(name)
                    || walk.used.contains(name)
                {
                    walk.ok = false;
                }
                walk.declared_global.insert(name.clone());
            }
        }
        Data::Nonlocal { names } => {
            for name in names {
                if walk.params.contains(name)
                    || walk.declared_global.contains(name)
                    || walk.bound.contains(name)
                    || walk.used.contains(name)
                {
                    walk.ok = false;
                }
                walk.declared_nonlocal.insert(name.clone());
            }
        }
        Data::AnnAssign { .. } => {
            if let Some(target) = tree.one(id, "target") {
                if let Data::Name { id: name, .. } = &tree.nodes[target].data {
                    walk.annotated.insert(name.clone());
                } else {
                    visit_node(tree, walk, target, loops, in_comp, limited);
                }
            }
            if let Some(annotation) = tree.one(id, "annotation") {
                visit_node(tree, walk, annotation, loops, in_comp, limited);
            }
            if let Some(value) = tree.one(id, "value") {
                visit_node(tree, walk, value, loops, in_comp, limited);
            }
        }
        Data::ExceptHandler { name } => {
            if let Some(name) = name {
                walk.note_bound(name);
            }
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
        Data::MatchAs { name } => {
            if let Some(name) = name {
                walk.note_bound(name);
            }
            if let Some(pattern) = tree.one(id, "pattern") {
                visit_node(tree, walk, pattern, loops, in_comp, limited);
            }
        }
        Data::MatchStar { name } => {
            if let Some(name) = name {
                walk.note_bound(name);
            }
        }
        Data::MatchMapping { rest } => {
            if let Some(name) = rest {
                walk.note_bound(name);
            }
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
        Data::NamedExpr => {
            // A walrus target binds in this scope wherever
            // the expression stands, a comprehension's
            // interior included.
            if let Some(target) = tree.one(id, "target") {
                if let Data::Name { id: name, .. } = &tree.nodes[target].data {
                    walk.note_bound(name);
                }
            }
            if let Some(value) = tree.one(id, "value") {
                visit_node(tree, walk, value, loops, in_comp, limited);
            }
        }
        Data::Name { id: name, ctx } => match ctx {
            Ctx::Load => {
                if !limited {
                    walk.used.insert(name.clone());
                }
            }
            Ctx::Store | Ctx::Del => {
                if !limited {
                    walk.note_bound(name);
                }
            }
        },
        _ => {
            visit_children(tree, walk, id, loops, in_comp, limited);
        }
    }
}

//! Pins for the Rust port slice 2 conformance repairs.
//!
//! The enlarged PORT2 battery caught the port accepting a
//! source the Python grammar refuses and walking a plant
//! tree the Python walk never reaches. These pins name the
//! divergences directly, beside the battery rows that carry
//! them, so each repair stands or falls on a small fast
//! test of its own. The refusal pin for the bare await
//! chain, the three plant pins, and the match sequence
//! pins fail on the pre fix head
//! for the reasons the diagnostic notes record. The legal
//! neighbour pins pass on both heads by construction and
//! stand as guards on the boundary the refusal must not
//! widen past.

use std::fs;
use std::path::PathBuf;

use canaryfield::pyarena::{parse_tree, render_tree};
use canaryfield::pydump::dump_tree;
use canaryfield::pyplant::plant_seed;

/// The recorded case synthetic:double-await. The Python
/// grammar refuses an unparenthesised await directly
/// inside another await, so the port builds no tree.
#[test]
fn bare_await_chain_is_refused() {
    for source in [
        "async def f():\n    await await g()\n",
        "async def f():\n    await await await g()\n",
        "async def f():\n    x = await await g()\n",
    ] {
        assert!(
            parse_tree(source).is_err(),
            "the port parsed a bare await chain: {source:?}"
        );
    }
}

/// The neighbours the recorded cases hold as legal. An
/// await in parentheses and an await inside call arguments
/// both parse, and a single await parses.
#[test]
fn await_neighbours_still_parse() {
    for source in [
        "async def f():\n    await g()\n",
        "async def f():\n    return await (await inner())\n",
        "async def f():\n    await fetch(await inner())\n",
    ] {
        assert!(
            parse_tree(source).is_ok(),
            "the port refused a legal await form: {source:?}"
        );
    }
}

fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-port2-fixes-{}-{tag}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir creates");
    dir
}

#[cfg(unix)]
fn linked_tree(tag: &str) -> PathBuf {
    let root = fresh_dir(tag);
    fs::create_dir_all(root.join("real")).expect("real dir");
    std::os::unix::fs::symlink("real", root.join("dirlink")).expect("link creates");
    root
}

/// The battery row plant:dirlink, seed-00. The Python
/// walk never descends into a symlinked directory, so
/// the file is reached only under its real path and is
/// reported under that path.
#[test]
#[cfg(unix)]
fn plant_walk_stays_out_of_a_linked_directory() {
    let root = linked_tree("dirlink");
    fs::write(root.join("real/inside.py"), "x = -1\nVERSION = '1.0'\n").expect("write");
    let planted = plant_seed(&root, "seed-00", None).expect("plants");
    assert_eq!(planted, vec!["real/inside.py".to_string()]);
    let after = fs::read_to_string(root.join("real/inside.py")).expect("reads");
    assert_eq!(after, "x = -1\nVERSION = '9.9.9'\n");
}

/// The battery row plant:dirlink-double-flip, seed-05.
/// A file reached through a linked directory and under
/// its real path is one candidate, visited once, so the
/// flag flips once and stays flipped.
#[test]
#[cfg(unix)]
fn plant_walk_visits_a_linked_file_once() {
    let root = linked_tree("dirlink-double-flip");
    fs::write(root.join("real/flag.py"), "flag = True\n").expect("write");
    let planted = plant_seed(&root, "seed-05", None).expect("plants");
    assert_eq!(planted, vec!["real/flag.py".to_string()]);
    let after = fs::read_to_string(root.join("real/flag.py")).expect("reads");
    assert_eq!(after, "flag = False\n");
}

/// The battery row plant:dot-py, seed-22. The glob the
/// Python walks by matches any name ending in .py, the
/// bare name .py itself included.
#[test]
fn plant_walk_collects_a_bare_dot_py_name() {
    let root = fresh_dir("dot-py");
    fs::write(root.join(".py"), "x = -1\n").expect("write");
    fs::write(root.join("mod.py"), "y = -2\n").expect("write");
    let planted = plant_seed(&root, "seed-22", None).expect("plants");
    assert_eq!(planted, vec![".py".to_string(), "mod.py".to_string()]);
    assert_eq!(
        fs::read_to_string(root.join(".py")).expect("reads"),
        "x = 1\n"
    );
    assert_eq!(
        fs::read_to_string(root.join("mod.py")).expect("reads"),
        "y = 2\n"
    );
}

/// The recorded cases synthetic:match-star-paren and
/// synthetic:match-star-value. A star element stands in a
/// parenthesised sequence the way it stands in brackets.
#[test]
fn match_star_parenthesised_needs_its_comma() {
    for source in [
        "match v:\n    case (first, *rest):\n        pass\n",
        "match v:\n    case (1, *rest):\n        pass\n",
        "match v:\n    case (*rest,):\n        pass\n",
    ] {
        assert!(
            parse_tree(source).is_ok(),
            "the port refused a parenthesised star pattern: {source:?}"
        );
    }
    for source in [
        "match v:\n    case (*rest):\n        pass\n",
        "match v:\n    case ((*rest)):\n        pass\n",
    ] {
        assert!(
            parse_tree(source).is_err(),
            "the port accepted a star pattern standing alone in parentheses: {source:?}"
        );
    }
}

/// The recorded case synthetic:match-star-open. A
/// sequence may stand open at the case, elements divided
/// by commas with no bracket, and its tree and rendering
/// are the bracketed tree and rendering.
#[test]
fn match_star_open_dump_and_render() {
    let source = "match v:\n    case first, *rest:\n        pass\n";
    let tree = parse_tree(source).expect("the open star pattern parses");
    assert_eq!(
        dump_tree(&tree).expect("dumps"),
        "Module(body=[Match(subject=Name(id='v', ctx=Load()), cases=[match_case(pattern=MatchSequence(patterns=[MatchAs(name='first'), MatchStar(name='rest')]), body=[Pass()])])], type_ignores=[])"
    );
    assert_eq!(
        render_tree(&tree).expect("renders"),
        "match v:\n    case [first, *rest]:\n        pass"
    );
}

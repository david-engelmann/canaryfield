//! Pins for the Round 15 review fix pass, the twenty
//! findings the CodeRabbit round and the red team raised
//! against the PR 20 head. Each pin names one divergence
//! and fails on the pre fix head for the reason its
//! comment records. The legal neighbour pins pass on both
//! heads by construction and guard the boundary a repair
//! must not widen past.

use std::fs;
use std::path::PathBuf;

use canaryfield::pyplant::{compile_accepts, plant_seed};

fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "canaryfield-cr-fixes-{}-{tag}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir creates");
    dir
}

/// Family 1, the compile reading whole. The port's plant
/// check was a reparse plus the nonlocal rule, so a
/// rendering the interpreter's symbol table refuses was
/// written as planted. Every source here is one the
/// interpreter refuses at compile, by the probed family.
#[test]
fn compile_check_refuses_the_symbol_table_family() {
    for source in [
        // return, break, and continue outside their ground
        "x = -1\nreturn x\n",
        "break\n",
        "continue\n",
        "class C:\n    return 1\n",
        "class C:\n    if True:\n        return 1\n",
        "for i in y:\n    def f():\n        break\n",
        "for i in y:\n    class C:\n        break\n",
        "while y:\n    pass\nelse:\n    break\n",
        // duplicate arguments, def and lambda alike
        "def f(a, a):\n    pass\n",
        "f = lambda a, a: a\n",
        "def f(a, *a):\n    pass\n",
        "def f(a, *, a):\n    pass\n",
        "def f(a, **a):\n    pass\n",
        // import star below the module
        "def f():\n    from x import *\n",
        "class C:\n    from x import *\n",
        // yield and await outside their ground
        "x = (yield 1)\n",
        "x = yield from y\n",
        "def f():\n    class C:\n        x = (yield 1)\n",
        "def f():\n    return [(yield i) for i in y]\n",
        "def f():\n    return ((yield i) for i in y)\n",
        "x = await f()\n",
        "class C:\n    x = await g()\n",
        "def f():\n    await g()\n",
        "async def f():\n    class C:\n        x = await g()\n",
        "async def f():\n    h = lambda: await g()\n",
        "async def f():\n    class C:\n        def m(self):\n            return await g()\n",
        "async for i in y:\n    pass\n",
        "async with y:\n    pass\n",
        "def f():\n    async for i in y:\n        pass\n",
        "def f():\n    return [i async for i in y]\n",
        // declaration order against binding and use
        "def f():\n    x = 1\n    global x\n",
        "def f():\n    print(x)\n    global x\n",
        "def f(x):\n    global x\n",
        "def f():\n    del x\n    global x\n",
        "def f():\n    x += 1\n    global x\n",
        "def f():\n    global x\n    x = 1\n    global x\n",
        "def f():\n    def g(x):\n        nonlocal x\n",
        "def f():\n    def g():\n        global x\n        nonlocal x\n",
        "def f():\n    x = 0\n    def g():\n        x = 1\n        nonlocal x\n",
        "def f():\n    x = 0\n    def g():\n        print(x)\n        nonlocal x\n",
        // an annotated name takes no declaration
        "def f():\n    x: int\n    global x\n",
        "def f():\n    x: int = 1\n    global x\n",
        "def f():\n    global x\n    x: int\n",
        "x: int\nglobal x\n",
        // a match capture binds before the declaration
        "def f():\n    match y:\n        case [x]:\n            pass\n    global x\n",
        // a nonlocal standing in a class body is validated
        "def f():\n    class C:\n        nonlocal x\n",
        "class C:\n    nonlocal x\n",
        "x = 0\ndef f():\n    global x\n    class C:\n        nonlocal x\n",
        "def f():\n    x = 0\n    class C:\n        x = 1\n        nonlocal x\n",
    ] {
        assert!(
            !compile_accepts(source),
            "the compile check accepted a refused rendering: {source:?}"
        );
    }
}

/// The legal neighbours of the same family. Every source
/// here compiles in the interpreter, so the plant check
/// must keep accepting it.
#[test]
fn compile_check_keeps_the_legal_neighbours() {
    for source in [
        "def f():\n    global x\n    x = 1\n",
        "def f():\n    global x\n    global x\n",
        "def f():\n    import x\n    global x\n",
        "def f():\n    from y import x\n    global x\n",
        "def f():\n    print([x for _ in y])\n    global x\n",
        "from x import *\n",
        "def f():\n    x = 0\n    def g():\n        nonlocal x\n        x = 1\n",
        "def f():\n    x: int\n    def g():\n        nonlocal x\n",
        // a nonlocal standing in a class body resolves
        // against the enclosing function
        "def f():\n    x = 1\n    class C:\n        nonlocal x\n",
        "def f():\n    x = 1\n    class C:\n        def m(self):\n            nonlocal x\n",
        "def f():\n    x = 0\n    class C:\n        nonlocal x\n        x = 1\n",
        "class C:\n    for i in y:\n        break\n",
        "def f():\n    for i in y:\n        try:\n            pass\n        finally:\n            continue\n",
        "f = lambda: (yield 1)\n",
        "def f():\n    yield 1\n",
        "def f():\n    return [x for x in (yield 1)]\n",
        "def f():\n    return [(lambda: (yield 1)) for i in y]\n",
        "async def f():\n    await g()\n",
        "async def f():\n    async for i in y:\n        pass\n",
        "async def f():\n    return [await g() async for _ in y]\n",
        "async def f():\n    def g(a=await h()):\n        pass\n",
        // a match capture binds for an inner nonlocal
        "def f():\n    match y:\n        case [x]:\n            pass\n    def g():\n        nonlocal x\n",
    ] {
        assert!(
            compile_accepts(source),
            "the compile check refused a legal rendering: {source:?}"
        );
    }
}

/// The red team's minimal repro. The seed flips the
/// literal, the rendering carries a return at module
/// level, the interpreter writes nothing, and the file
/// stands byte for byte as it stood.
#[test]
fn plant_writes_nothing_the_compiler_refuses() {
    let root = fresh_dir("return-module");
    fs::write(root.join("ret.py"), "x = -1\nreturn x\n").expect("write");
    let planted = plant_seed(&root, "seed-22", None).expect("plants");
    assert_eq!(planted, Vec::<String>::new());
    assert_eq!(
        fs::read_to_string(root.join("ret.py")).expect("reads"),
        "x = -1\nreturn x\n"
    );
}

/// CR5 at plant level. Seed-19 points the loop at x, a
/// name the scope binds below a global declaration of
/// the same name, so the rendering assigns before the
/// declaration and the interpreter plants nothing.
#[test]
fn plant_refuses_an_assignment_built_before_a_global() {
    let root = fresh_dir("global-order");
    let source = "def f():\n    for item in items:\n        pass\n    global x\n    x = 0\n";
    fs::write(root.join("mod.py"), source).expect("write");
    let planted = plant_seed(&root, "seed-19", None).expect("plants");
    assert_eq!(planted, Vec::<String>::new());
    assert_eq!(
        fs::read_to_string(root.join("mod.py")).expect("reads"),
        source
    );
}

/// CR6 at plant level. The outer function binds x only
/// through a case pattern, and the inner function's
/// nonlocal reads that binding, so the planting stands.
#[test]
fn plant_counts_a_match_capture_as_a_binding() {
    let root = fresh_dir("match-capture");
    let source = "def outer():\n    match y:\n        case [x]:\n            pass\n    def inner():\n        nonlocal x\n        flag = True\n";
    fs::write(root.join("mod.py"), source).expect("write");
    let planted = plant_seed(&root, "seed-05", None).expect("plants");
    assert_eq!(planted, vec!["mod.py".to_string()]);
    assert!(fs::read_to_string(root.join("mod.py"))
        .expect("reads")
        .contains("flag = False"));
}

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

use canaryfield::pyarena::{parse_tree, Data};
use canaryfield::pyast::ConstVal;
use canaryfield::pytoken::{tokenize, StrTok, Tok};

fn name_tokens(source: &str) -> Vec<String> {
    tokenize(source, 1, 0)
        .expect("the source tokenizes")
        .into_iter()
        .filter_map(|token| match token.kind {
            Tok::Name(word) => Some(word),
            _ => None,
        })
        .collect()
}

#[test]
fn identifier_readings_follow_the_xid_tables() {
    // The script property sign starts a name, and the
    // connector punctuation and the middle dot continue one,
    // the XID readings of the interpreter. The Greek numeral
    // sign and the katakana middle dot do neither, and a
    // name carrying one is refused.
    assert_eq!(name_tokens("\u{2118} = 1\n"), vec!["\u{2118}".to_string()]);
    assert_eq!(
        name_tokens("x\u{203f}y = 1\n"),
        vec!["x\u{203f}y".to_string()]
    );
    assert_eq!(name_tokens("x\u{b7}y = 1\n"), vec!["x\u{b7}y".to_string()]);
    assert!(tokenize("\u{375} = 1\n", 1, 0).is_err());
    assert!(tokenize("x\u{375} = 1\n", 1, 0).is_err());
    assert!(tokenize("x\u{30fb}y = 1\n", 1, 0).is_err());
}

#[test]
fn physical_newlines_translate_on_the_parse_path() {
    // A carriage return standing inside a string reads as
    // the newline the interpreter's translation makes of
    // it, and a comment ends at the carriage return that
    // ends its line, on the direct parse path and not only
    // on the planting path that already translated.
    let tokens = tokenize("s = '''a\r\nb'''\n", 1, 0).expect("the source tokenizes");
    let mut values: Vec<Vec<u32>> = Vec::new();
    for token in &tokens {
        if let Tok::Str(StrTok::Value(ConstVal::Str(text, _))) = &token.kind {
            values.push(text.codepoints().to_vec());
        }
    }
    assert_eq!(values, vec![vec![97, 10, 98]]);
    let tree = parse_tree("# note\rx = 1\r").expect("the source parses");
    assert!(tree
        .nodes
        .iter()
        .any(|node| matches!(node.data, Data::Assign { .. })));
}

#[test]
fn an_indent_whose_altcol_does_not_grow_is_refused() {
    // One space opens the outer level and a tab alone the
    // inner one, its raw column no greater than the level
    // it joins, the mixing the interpreter refuses on the
    // push of the deeper level. A tab and a space together
    // grow the raw column past it and stand.
    assert!(tokenize("if True:\n if True:\n\tpass\n", 1, 0).is_err());
    assert!(tokenize("if True:\n  if True:\n \tpass\n", 1, 0).is_err());
    assert!(tokenize("if True:\n if True:\n\t pass\n", 1, 0).is_ok());
    assert!(tokenize("if True:\n\tpass\n", 1, 0).is_ok());
    assert!(tokenize("if True:\n    pass\n", 1, 0).is_ok());
}

#[test]
fn the_bf_and_fb_prefixes_are_refused() {
    assert!(parse_tree("v = bf'x'\n").is_err());
    assert!(parse_tree("v = fb'x'\n").is_err());
    assert!(parse_tree("v = f'x'\n").is_ok());
    assert!(parse_tree("v = b'x'\n").is_ok());
    assert!(parse_tree("v = br'x'\n").is_ok());
}

#[test]
fn a_raw_fstring_escaped_quote_does_not_end_the_string() {
    // The escaped quotes stand in the value and the field
    // between them stands as a field, the string ending at
    // the quote no backslash precedes.
    let tree = parse_tree("v = rf'\\'{name}\\''\n").expect("the raw f-string parses");
    assert!(tree
        .nodes
        .iter()
        .any(|node| matches!(node.data, Data::Name { .. })));
}

fn spec_chain(depth: u32) -> String {
    let mut field = String::from("{1}");
    for _ in 0..depth {
        field = format!("{{1:{field}}}");
    }
    format!("f'{field}'")
}

#[test]
fn format_spec_fields_nest_three_deep_and_no_deeper() {
    assert!(parse_tree(&format!("v = {}\n", spec_chain(3))).is_ok());
    assert!(parse_tree(&format!("v = {}\n", spec_chain(4))).is_err());
    // A chain of any depth refuses in words, never by
    // running the stack down.
    assert!(parse_tree(&format!("v = {}\n", spec_chain(200))).is_err());
}

#[test]
fn a_nested_field_carries_the_fragment_base() {
    // The comparison stands in a string standing in a field
    // on the third line of the file, and the position it
    // carries is the position it stands at in the file, the
    // fragment base carried through both fields.
    let tree = parse_tree("\n\nresult = f\"{f'{left < right}'}\"\n").expect("the source parses");
    let compare = tree
        .nodes
        .iter()
        .find(|node| matches!(node.data, Data::Compare { .. }))
        .expect("a comparison stands in the tree");
    let pos = compare.pos.expect("the comparison carries a position");
    assert_eq!((pos.line, pos.col), (3, 15));
}

use canaryfield::pydata::lookup_name;

#[test]
fn character_names_resolve_by_the_interpreters_reading() {
    // A name resolves under its exact spelling, capitals
    // aside. The hyphen of TIBETAN LETTER -A is part of the
    // name, and losing it or doubling a space loses the
    // character.
    assert_eq!(lookup_name("TIBETAN LETTER A"), Some('\u{f68}'));
    assert_eq!(lookup_name("TIBETAN LETTER -A"), Some('\u{f60}'));
    assert_eq!(lookup_name("tibetan letter a"), Some('\u{f68}'));
    assert_eq!(lookup_name("TIBETAN LETTER-A"), None);
    assert_eq!(lookup_name("TIBETAN  LETTER A"), None);
    assert_eq!(lookup_name("TIBETAN LETTER A "), None);
    // The formal aliases resolve beside the names.
    assert_eq!(lookup_name("BYTE ORDER MARK"), Some('\u{feff}'));
    assert_eq!(lookup_name("BEL"), Some('\u{7}'));
    assert_eq!(lookup_name("NULL"), Some('\u{0}'));
    assert_eq!(lookup_name("BELL"), Some('\u{1f514}'));
}

#[test]
fn hangul_names_follow_the_medial_order() {
    // The medials stand in the order the interpreter
    // composes them, WA ninth and WEO fourteenth, so GWA
    // lands where the interpreter puts it and the names
    // the wrong spellings composed stand resolved too.
    assert_eq!(lookup_name("HANGUL SYLLABLE GWA"), Some('\u{acfc}'));
    assert_eq!(lookup_name("HANGUL SYLLABLE GWAK"), Some('\u{ad14}'));
    assert_eq!(lookup_name("HANGUL SYLLABLE WEO"), Some('\u{c6cc}'));
    assert_eq!(lookup_name("HANGUL SYLLABLE GWEO"), Some('\u{ad88}'));
    assert_eq!(lookup_name("HANGUL SYLLABLE HANG"), Some('\u{d56d}'));
    assert_eq!(lookup_name("hangul syllable gwa"), None);
}

#[test]
fn algorithmic_families_validate_their_ranges() {
    // The unified ideographs resolve by rule over their
    // true ranges, in four or five upper case digits, and
    // every other spelling and family stands as the
    // interpreter leaves it.
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-3400"), Some('\u{3400}'));
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-03400"), Some('\u{3400}'));
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-0041"), None);
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-4DC0"), None);
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-2A6E0"), None);
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-9fff"), None);
    assert_eq!(lookup_name("CJK UNIFIED IDEOGRAPH-003400"), None);
    // The compatibility ideographs and the Khitan and
    // Nushu characters stand in the table under their
    // exact names, padded spellings refused.
    assert_eq!(
        lookup_name("CJK COMPATIBILITY IDEOGRAPH-F900"),
        Some('\u{f900}')
    );
    assert_eq!(lookup_name("CJK COMPATIBILITY IDEOGRAPH-0F900"), None);
    assert_eq!(
        lookup_name("KHITAN SMALL SCRIPT CHARACTER-18B00"),
        Some('\u{18b00}')
    );
    assert_eq!(lookup_name("KHITAN SMALL SCRIPT CHARACTER-018B00"), None);
    // The Tangut ideographs carry no name the lookup
    // resolves, in this database as in the interpreter.
    assert_eq!(lookup_name("TANGUT IDEOGRAPH-17000"), None);
}

use canaryfield::pyarena::render_tree;
use canaryfield::pydump::dump_tree;

#[test]
fn a_parenthesised_star_without_a_comma_is_refused() {
    // A star standing alone inside parentheses is neither
    // a tuple nor a sequence pattern, in an expression or
    // in a case, and the comma is what makes it one.
    assert!(parse_tree("x = (*b)\n").is_err());
    assert!(parse_tree("x = (*b,)\n").is_ok());
    assert!(parse_tree("x = (*b, *c)\n").is_ok());
    assert!(parse_tree("match v:\n    case (*rest):\n        pass\n").is_err());
    assert!(parse_tree("match v:\n    case ((*rest)):\n        pass\n").is_err());
    assert!(parse_tree("match v:\n    case (*rest,):\n        pass\n").is_ok());
    assert!(parse_tree("match v:\n    case [*rest]:\n        pass\n").is_ok());
}

#[test]
fn a_lone_starred_subscript_stands_as_a_one_tuple() {
    // One starred item in a subscript is a tuple of one,
    // in the dump and in the render, on every context the
    // subscript can stand in.
    let tree = parse_tree("x = a[*b]\n").expect("the source parses");
    let dump = dump_tree(&tree).expect("the tree dumps");
    assert!(dump.contains("slice=Tuple(elts=[Starred"), "{dump}");
    assert_eq!(render_tree(&tree).expect("the tree renders"), "x = a[*b,]");
    let tree = parse_tree("a[*b] = 1\n").expect("the source parses");
    assert_eq!(render_tree(&tree).expect("the tree renders"), "a[*b,] = 1");
    let tree = parse_tree("del a[*-1]\n").expect("the source parses");
    assert_eq!(render_tree(&tree).expect("the tree renders"), "del a[*-1,]");
    let tree = parse_tree("x = a[*b, c]\n").expect("the source parses");
    assert_eq!(render_tree(&tree).expect("the tree renders"), "x = a[*b, c]");
}

#[test]
fn the_format_spec_render_doubles_a_backslash_once() {
    // The source carries two backslashes in the spec and
    // its value one, and the render carries two again, so
    // the render reads back to the value it was rendered
    // from.
    let source = "v = f'{1:\\\\<4}'\n";
    let tree = parse_tree(source).expect("the source parses");
    let rendered = render_tree(&tree).expect("the tree renders");
    assert_eq!(rendered, source.strip_suffix('\n').expect("the source ends"));
    let again = parse_tree(&rendered).expect("the render parses");
    assert_eq!(
        dump_tree(&again).expect("the render dumps"),
        dump_tree(&tree).expect("the tree dumps")
    );
}

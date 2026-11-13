// Groundwork probe: seed-17 over self-mapping and ordinary dictionaries.
use canaryfield::pyapply::apply_seed;
use canaryfield::pyarena::{parse_tree, render_tree};
use canaryfield::pydump::dump_tree;

fn run(source: &str) {
    let mut tree = parse_tree(source).expect("parses");
    let dump0 = dump_tree(&tree).expect("dump0");
    let render0 = render_tree(&tree).expect("render0");
    let answer = apply_seed(&mut tree, "seed-17").expect("apply answers");
    let dump1 = dump_tree(&tree).expect("dump1");
    let render1 = render_tree(&tree).expect("render1");
    println!(
        "source={source:?} bool={answer} dump_unchanged={} render_unchanged={}",
        dump1 == dump0,
        render1 == render0
    );
    let second = apply_seed(&mut tree, "seed-17").expect("second answers");
    println!("  second bool={second}");
}

fn main() {
    run("d = {key: key}\n");
    run("d = {1: 1}\n");
    run("d = {'a': 'a', 'b': 2}\n");
    run("d = {a: 1, b: 2}\n");
    run("d = {}\n");
    run("d = {**other}\n");
}

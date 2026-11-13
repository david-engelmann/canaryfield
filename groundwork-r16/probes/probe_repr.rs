// Groundwork probe: spell seam strings through both repr copies.
use canaryfield::json::{py_repr, JsonValue};
use canaryfield::pyunparse::str_repr;

fn main() {
    // codepoint seam set, exercised as one-char strings and inside text
    let seams: Vec<u32> = vec![
        0x41, 0x20, 0xAD, 0xFEFF, 0x200B, 0x202F, 0x3000, 0x1680, 0xE000, 0x378, 0xFFFE, 0x7F,
        0x85, 0x2028, 0x2029, 0xFFFD, 0xE0001, 0x61, 0xA0, 0x180E, 0x2060, 0xFFF9,
    ];
    for cp in &seams {
        let b = str_repr(&[*cp]);
        match char::from_u32(*cp) {
            Some(ch) => {
                let a = py_repr(&JsonValue::Str(ch.to_string()));
                println!("cp {cp:06x} A={a} B={b}");
            }
            None => println!("cp {cp:06x} B={b}"),
        }
    }
    // the lone surrogate: only copy B's input type can carry it
    println!("surrogate B={}", str_repr(&[0xD800]));
    println!("surrogate-mixed B={}", str_repr(&[0x61, 0xD800, 0x62]));
    // quote and escape shapes
    for s in ["it's", "say \"hi\"", "both ' and \"", "back\\slash", "a\nb", "a\tb", "a\rb", ""]
    {
        let a = py_repr(&JsonValue::Str(s.to_string()));
        let pts: Vec<u32> = s.chars().map(|c| c as u32).collect();
        let b = str_repr(&pts);
        println!("text {s:?} A={a} B={b}");
    }
    // scalars through copy A
    for v in [
        JsonValue::Null,
        JsonValue::Bool(true),
        JsonValue::Int("123456789012345678901234567890".to_string()),
        JsonValue::Float(1e16),
        JsonValue::Float(0.00001),
    ] {
        println!("scalar A={}", py_repr(&v));
    }
}

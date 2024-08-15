// Imports
mod ast;
mod codegen;
mod error;
mod parse;
mod preprocess;
mod songs;
mod t2t;

use std::{collections::HashMap, fs};

use ast::VarType;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn test_parse_song(name: &str) {
    let s = fs::read_to_string(format!("franz-programs/{name}.fz")).expect("");
    let varmap: &mut HashMap<String, VarType> = &mut HashMap::new();

    let parsed_program =
        t2t::uexpr_to_control(varmap, &parse_assistant!(ControlParser, &s));

    if let Ok(r) = parsed_program {
        dbg!(r);
    } else {
        print!("Error parsing program.");
    }
}

fn main() {
    test_parse_song("imperial_march");
    test_parse_song("anthem");
    test_parse_song("anthem2");
    test_parse_song("innocent");

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![imperial_march, 0.3];
    notes![anthem2, 0.3];
}

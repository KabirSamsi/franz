// Imports
mod ast;
mod codegen;
mod error;
mod parse;
// mod preprocess;
mod songs;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn main() {
    let _ = parse::parse_bexp("!true && true");
    let _ = parse::parse_aexp("12 + 20");
    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![darthvader, 0.3];
}

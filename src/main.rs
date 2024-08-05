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
    notes![innocent, 0.25];
    notes![anthem, 0.25];
    notes![apprasionata, 0.25];
    notes![allstar, 0.25];
}

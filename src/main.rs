// Imports
mod ast;
mod codegen;
mod error;
mod parse;
mod preprocess;
mod songs;

macro_rules! notes {
    ($name:ident, $speed:expr) => {
        let _ =
            &codegen::compile_seq(stringify!($name), songs::$name(), $speed);
    };
}

fn main() {
    let p = parse_assistant!(
        UntypedExprParser,
        "motif x[z, w] = {qt; 2 * {qt.; x ? hf : {2 * qt; hf; et.}}}"
    );

    let g = parse_assistant!(
        UntypedExprParser,
        "return {qt; 2 * {qt.; {ab ? hf : 2 * qt}}}[hello]"
    );

    dbg!(p);

    notes![innocent, 0.25];
    notes![anthem, 0.3];
    notes![apprasionata, 0.15];
    notes![imperial_march, 0.3];
    notes![anthem2, 0.3];
}

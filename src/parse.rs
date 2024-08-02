mod ast;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(
            parser::BeatParser::new()
                .parse(stringify!($expr))
                .unwrap(),
            $expr
        );
    };
}

fn parser() {
    test3!("q");
}

fn main() {
    parser();
}
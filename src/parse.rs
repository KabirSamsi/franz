use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        parser::BeatParser::new().parse(stringify!($expr)).unwrap()
    };
}

fn parse() {
    test3!("q");
}

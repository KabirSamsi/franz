use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

#[macro_export]
macro_rules! parse_assistant {
    ($parser_type:ident, $sequence:expr) => {
        $crate::parse::parser::$parser_type::new()
            .parse($sequence)
            .unwrap()
    };
}

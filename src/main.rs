// Imports
mod ast;
mod codegen;
mod error;
mod songs;

fn main() {
    let _ = &codegen::compile_seq("innocent", songs::innocent(), 0.25, false);
    let _ = &codegen::compile_seq(
        "star_spangled_banner",
        songs::anthem(),
        0.3,
        false
    );
    let _ = &codegen::compile_seq(
        "apprasionata",
        songs::apprasionata(),
        0.15,
        false
    );

    let _ = &codegen::compile_seq("all_star", songs::allstar(), 0.25, false);
}

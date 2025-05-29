use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub phobos_grammar);

pub mod ast;

fn main() {
    let program = phobos_grammar::ProgramParser::new()
        .parse("extern foo(n: Int): Int")
        .expect("Failed to parse program");

    println!(
        "{}",
        program
            .iter()
            .map(|decl| format!("{:?}", decl))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

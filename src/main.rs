use ast::TopLevelDecl;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub phobos_grammar);

pub mod ast;
pub mod types;

#[allow(dead_code)]
fn program_to_string(program: &Vec<TopLevelDecl>) -> String {
    program
        .iter()
        .map(|decl| format!("{:?}", decl))
        .collect::<Vec<String>>()
        .join("\n")
}

#[test]
fn test_parse_extern_decl() {
    let code = "extern foo(n: Int): Int";
    let program = phobos_grammar::ProgramParser::new()
        .parse(code)
        .expect("Failed to parse program");
    let stringified = program_to_string(&program);

    assert_eq!(stringified, code);
}

#[test]
fn test_parse_function_decl() {
    let code = "fn foo(n: Int): Int { return (n + 1); }";
    let program = phobos_grammar::ProgramParser::new()
        .parse(code)
        .expect("Failed to parse program");
    let stringified = program_to_string(&program);

    assert_eq!(stringified, code);
}

#[test]
fn test_parse_type_decl() {
    let code = "type Foo { name: String, age: Int }";
    let program = phobos_grammar::ProgramParser::new()
        .parse(code)
        .expect("Failed to parse program");
    let stringified = program_to_string(&program);

    assert_eq!(stringified, code);
}

fn main() {
    let code = "fn foo(n: Number): Number { return (n + 1); }";
    let program = phobos_grammar::ProgramParser::new()
        .parse(code)
        .expect("Failed to parse program");
    types::typecheck(&program).expect("Type checking failed");
    println!("Parsed program successfully!");
}

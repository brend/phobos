use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub phobos_grammar);

pub mod ast;
pub mod codegen;
pub mod types;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut reader = open_reader()?;
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    println!("## Input\n\n{}", input);

    let program = phobos_grammar::ProgramParser::new()
        .parse(&input)
        .expect("Failed to parse program");

    println!("\nParsing successful!");

    types::typecheck(&program).expect("Type checking failed");

    println!("Type check successful!");
    println!("## Output\n\n``` Lua\n");

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    codegen::generate_code(&mut handle, &program).expect("Failed to generate code");

    println!("\n```");
    Ok(())
}

fn open_reader() -> io::Result<Box<dyn BufRead>> {
    // Get command-line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // Choose the input source: file or stdin
    if let Some(filename) = args.first() {
        let file = File::open(filename)?;
        Ok(Box::new(BufReader::new(file)))
    } else {
        Ok(Box::new(BufReader::new(io::stdin())))
    }
}

#[cfg(test)]
mod tests {
    use super::ast::TopLevelDecl;
    use super::phobos_grammar;

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
}

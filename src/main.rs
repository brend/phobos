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
    let program = match phobos_grammar::ProgramParser::new().parse(&mut input) {
        Ok(program) => program,

        Err(e) => match e {
            lalrpop_util::ParseError::InvalidToken { location }
            | lalrpop_util::ParseError::UnrecognizedToken {
                token: (location, _, _),
                ..
            }
            | lalrpop_util::ParseError::UnrecognizedEof { location, .. } => {
                let (line, col) = byte_offset_to_line_col(&input, location);
                eprintln!("Parse error at line {}, column {}", line, col);
                panic!();
            }
            other => {
                eprintln!("Other parse error: {:?}", other);
                panic!();
            }
        },
    };
    types::typecheck(&program).expect("Type checking failed");
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    codegen::generate_code(&mut handle, &program).expect("Failed to generate code");
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

pub fn byte_offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    let mut i = 0;

    for ch in source.chars() {
        if i == offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        i += ch.len_utf8();
    }

    (line, col)
}

#[cfg(test)]
mod tests {
    use super::ast::Program;
    use super::phobos_grammar;

    #[allow(dead_code)]
    fn program_to_string(program: &Program) -> String {
        program
            .top_level_decls
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

use std::io::Write;

use crate::ast::{Block, Expr, FunctionDecl, Opcode, Program, Stmt, TopLevelDecl};

const INDENT: usize = 4;

pub fn generate_code<W: Write>(writer: &mut W, program: &Program) -> Result<(), std::io::Error> {
    for decl in &program.top_level_decls {
        generate_declaration(writer, decl, 0)?;
    }
    Ok(())
}

fn generate_declaration<W: Write>(
    writer: &mut W,
    decl: &TopLevelDecl,
    indent: usize,
) -> Result<(), std::io::Error> {
    match decl {
        TopLevelDecl::FunctionDecl(func) => generate_function(writer, func, indent)?,
        TopLevelDecl::RecordDecl(name, fields) => {}
        _ => unimplemented!(),
    }
    Ok(())
}

fn generate_function<W: Write>(
    writer: &mut W,
    func: &FunctionDecl,
    indent: usize,
) -> Result<(), std::io::Error> {
    write!(writer, "{}function {}(", " ".repeat(indent), func.name)?;
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", param.name)?;
    }
    write!(writer, ")\n")?;
    generate_block(writer, &func.body, indent + INDENT)?;
    write!(writer, "end\n")?;
    Ok(())
}

fn generate_block<W: Write>(
    writer: &mut W,
    block: &Block,
    indent: usize,
) -> Result<(), std::io::Error> {
    //write!(writer, "{}do\n", " ".repeat(indent))?;
    for stmt in &block.stmts {
        generate_statement(writer, stmt, indent /*+ INDENT*/)?;
    }
    //write!(writer, "{}end\n", " ".repeat(indent))?;
    Ok(())
}

fn generate_statement<W: Write>(
    writer: &mut W,
    stmt: &Stmt,
    indent: usize,
) -> Result<(), std::io::Error> {
    match stmt {
        Stmt::Return(expr) => {
            write!(writer, "{}return ", " ".repeat(indent))?;
            generate_expression(writer, expr)?;
            write!(writer, "{}\n", " ".repeat(indent))?;
        }
        Stmt::Let(name, _, expr) => {
            write!(writer, "{}local {} = ", " ".repeat(indent), name)?;
            generate_expression(writer, expr)?;
            write!(writer, "{}\n", " ".repeat(indent))?;
        }
        Stmt::Expr(expr) => {
            write!(writer, "{}", " ".repeat(indent))?;
            generate_expression(writer, expr)?;
            write!(writer, "\n")?;
        }
        Stmt::If(cond, then_branch, else_branch) => {
            write!(writer, "{}if ", " ".repeat(indent))?;
            generate_expression(writer, cond)?;
            write!(writer, " then\n")?;
            generate_block(writer, then_branch, indent + INDENT)?;
            if let Some(else_branch) = else_branch {
                write!(writer, "{}else\n", " ".repeat(indent))?;
                generate_block(writer, else_branch, indent + INDENT)?;
                write!(writer, "{}end\n", " ".repeat(indent))?;
            }
        }
        Stmt::Assign(name, value) => {
            write!(writer, "{}let {} = ", " ".repeat(indent), name)?;
            generate_expression(writer, value)?;
            write!(writer, ";\n")?;
        }
    }
    Ok(())
}

fn generate_expression<W: Write>(writer: &mut W, expr: &Expr) -> Result<(), std::io::Error> {
    match expr {
        Expr::Number(n) => write!(writer, "{}", n),
        Expr::String(s) => write!(writer, "\"{}\"", s),
        Expr::Ident(ident) => write!(writer, "{}", ident),
        Expr::BinaryExp(left, op, right) => {
            write!(writer, "(")?;
            generate_expression(writer, left)?;
            generate_op(writer, op)?;
            generate_expression(writer, right)?;
            write!(writer, ")")
        }
        Expr::Call(func, args) => {
            write!(writer, "{}(", func)?;
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                generate_expression(writer, arg)?;
            }
            write!(writer, ")")
        }
    }
}

fn generate_op<W: Write>(writer: &mut W, op: &Opcode) -> Result<(), std::io::Error> {
    match op {
        Opcode::Add => write!(writer, "+")?,
        Opcode::Sub => write!(writer, "-")?,
        Opcode::Mul => write!(writer, "*")?,
        Opcode::Div => write!(writer, "/")?,
        Opcode::Eq => write!(writer, "==")?,
        Opcode::Neq => write!(writer, "!=")?,
        Opcode::Lt => write!(writer, "<")?,
        Opcode::Le => write!(writer, "<=")?,
        Opcode::Gt => write!(writer, ">")?,
        Opcode::Ge => write!(writer, ">=")?,
    }
    Ok(())
}

use std::io::Write;

use crate::ast::{Expr, FunctionDecl, Opcode, Stmt, TopLevelDecl};

pub fn generate_code<W: Write>(
    writer: &mut W,
    program: &Vec<TopLevelDecl>,
) -> Result<(), std::io::Error> {
    for decl in program {
        generate_declaration(writer, decl)?;
    }
    Ok(())
}

fn generate_declaration<W: Write>(
    writer: &mut W,
    decl: &TopLevelDecl,
) -> Result<(), std::io::Error> {
    match decl {
        TopLevelDecl::FunctionDecl(func) => generate_function(writer, func)?,
        _ => unimplemented!(),
    }
    Ok(())
}

fn generate_function<W: Write>(writer: &mut W, func: &FunctionDecl) -> Result<(), std::io::Error> {
    write!(writer, "function {}(", func.name)?;
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            write!(writer, ", ")?;
        }
        write!(writer, "{}", param.name)?;
    }
    write!(writer, ")\n")?;
    for stmt in &func.body.stmts {
        generate_statement(writer, stmt)?;
    }
    write!(writer, "end\n")?;
    Ok(())
}

fn generate_statement<W: Write>(writer: &mut W, stmt: &Stmt) -> Result<(), std::io::Error> {
    match stmt {
        Stmt::Expr(expr) => generate_expression(writer, expr)?,
        Stmt::Return(expr) => {
            write!(writer, "return ")?;
            generate_expression(writer, expr)?;
            write!(writer, "\n")?;
        }
        _ => unimplemented!(),
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
        _ => unimplemented!(),
    }
}

fn generate_op<W: Write>(writer: &mut W, op: &Opcode) -> Result<(), std::io::Error> {
    match op {
        Opcode::Add => write!(writer, "+")?,
        Opcode::Sub => write!(writer, "-")?,
        Opcode::Mul => write!(writer, "*")?,
        Opcode::Div => write!(writer, "/")?,
    }
    Ok(())
}

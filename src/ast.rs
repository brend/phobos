use std::fmt::Debug;

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Op(left, op, right) => write!(f, "({:?} {:?} {:?})", left, op, right),
        }
    }
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
        }
    }
}

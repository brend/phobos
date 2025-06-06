use std::fmt::Debug;

pub struct Program {
    pub top_level_decls: Vec<TopLevelDecl>,
}

impl Program {
    pub fn new(top_level_decls: Vec<TopLevelDecl>) -> Self {
        Program { top_level_decls }
    }
}

pub enum TopLevelDecl {
    FunctionDecl(FunctionDecl),
    ExternDecl(ExternDecl),
    RecordDecl(String, Vec<FieldDecl>),
    GameDecl(GameDecl),
}

impl Debug for TopLevelDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopLevelDecl::FunctionDecl(func) => write!(f, "{:?}", func),
            TopLevelDecl::ExternDecl(extern_decl) => write!(f, "{:?}", extern_decl),
            TopLevelDecl::RecordDecl(name, fields) => {
                write!(f, "RecordDecl({}, {:?})", name, fields)
            }
            TopLevelDecl::GameDecl(game_decl) => write!(f, "{:?}", game_decl),
        }
    }
}

pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub ret: Type,
    pub body: Block,
}

impl FunctionDecl {
    pub fn new(name: String, params: Vec<ParamDecl>, ret: Type, body: Block) -> Self {
        FunctionDecl {
            name,
            params,
            ret,
            body,
        }
    }
}

impl Debug for FunctionDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn {}({}): {:?} {:?}",
            self.name,
            self.params
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<String>>()
                .join("\n"),
            self.ret,
            self.body
        )
    }
}

pub struct ParamDecl {
    pub name: String,
    pub ty: Type,
}

impl ParamDecl {
    pub fn new(name: String, ty: Type) -> Self {
        ParamDecl { name, ty }
    }
}

impl Debug for ParamDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.ty)
    }
}

#[derive(Clone)]
pub struct Type {
    pub name: String,
}

impl Type {
    pub fn new(name: String) -> Self {
        Type { name }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Block { stmts }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.stmts
                .iter()
                .map(|stmt| format!("{:?}", stmt))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub enum Stmt {
    Let(String, Type, Expr),
    Assign(String, Expr),
    If(Expr, Block, Option<Block>),
    Return(Box<Expr>),
    Expr(Expr),
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Let(name, ty, expr) => write!(f, "let {}: {:?} = {:?};", name, ty, expr),
            Stmt::Assign(name, expr) => write!(f, "{} = {:?};", name, expr),
            Stmt::If(cond, then, els) => write!(f, "if {:?} {:?} {:?}", cond, then, els),
            Stmt::Return(expr) => write!(f, "return {:?};", expr),
            Stmt::Expr(expr) => write!(f, "{:?}", expr),
        }
    }
}

pub enum Expr {
    Number(f64),
    String(String),
    Ident(String),
    BinaryExp(Box<Expr>, Opcode, Box<Expr>),
    Call(String, Vec<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::BinaryExp(left, op, right) => write!(f, "({:?} {:?} {:?})", left, op, right),
            Expr::Call(func, args) => write!(f, "{}({:?})", func, args),
        }
    }
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Eq => write!(f, "=="),
            Opcode::Neq => write!(f, "!="),
            Opcode::Lt => write!(f, "<"),
            Opcode::Le => write!(f, "<="),
            Opcode::Gt => write!(f, ">"),
            Opcode::Ge => write!(f, ">="),
        }
    }
}

pub struct GameDecl {
    pub name: String,
    pub functions: Vec<FunctionDecl>,
}

impl GameDecl {
    pub fn new(name: String, functions: Vec<FunctionDecl>) -> Self {
        GameDecl { name, functions }
    }
}

impl Debug for GameDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "game {}", self.name)
    }
}

pub struct FieldDecl {
    pub name: String,
    pub ty: Type,
}

impl FieldDecl {
    pub fn new(name: String, ty: Type) -> Self {
        FieldDecl { name, ty }
    }
}

impl Debug for FieldDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.ty)
    }
}

pub struct ExternDecl {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub ret: Type,
}

impl ExternDecl {
    pub fn new(name: String, params: Vec<ParamDecl>, ret: Type) -> Self {
        ExternDecl { name, params, ret }
    }
}

impl Debug for ExternDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "extern {}({}): {:?}",
            self.name,
            self.params
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<String>>()
                .join("\n"),
            self.ret
        )
    }
}

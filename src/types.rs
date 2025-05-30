use crate::ast::{self, Expr, Opcode};
use crate::ast::{Block, FunctionDecl, Stmt, TopLevelDecl};

#[derive(Clone, Debug)]
pub enum Type {
    Void,
    Number,
    String,
    Function(Vec<Type>, Box<Type>),
}

impl From<ast::Type> for Type {
    fn from(ty: ast::Type) -> Self {
        match ty.name.as_str() {
            "Number" => Type::Number,
            _ => unimplemented!(),
        }
    }
}

impl From<FunctionDecl> for Type {
    fn from(func: FunctionDecl) -> Self {
        Type::Function(
            func.params
                .iter()
                .map(|param| param.ty.clone().into())
                .collect(),
            Box::new(func.ret.into()),
        )
    }
}

#[derive(Debug)]
pub struct TypeEnvironment {
    types: Vec<(String, Type)>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        TypeEnvironment { types: Vec::new() }
    }

    pub fn set_type(&mut self, name: &str, ty: Type) {
        self.types.push((name.to_string(), ty));
    }

    pub fn get_type(&self, name: &str) -> Option<Type> {
        self.types
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, ty)| ty.clone())
    }
}

pub fn typecheck(program: &Vec<TopLevelDecl>) -> Result<(), String> {
    let mut env = TypeEnvironment::new();
    for decl in program {
        match decl {
            TopLevelDecl::FunctionDecl(func) => typecheck_function_decl(func, &mut env)?,
            _ => unimplemented!(),
        }
    }
    Ok(())
}

fn typecheck_function_decl(func: &FunctionDecl, env: &mut TypeEnvironment) -> Result<(), String> {
    // Add the function itself to the environment
    env.types.push((
        func.name.clone(),
        Type::Function(
            func.params
                .iter()
                .map(|param| param.ty.clone().into())
                .collect(),
            Box::new(func.ret.clone().into()),
        ),
    ));
    // Add the parameters to the environment
    for param in &func.params {
        env.types
            .push((param.name.clone(), param.ty.clone().into()));
    }
    // Typecheck the function body
    typecheck_block(&func.body, env)?;
    Ok(())
}

fn typecheck_block(block: &Block, env: &mut TypeEnvironment) -> Result<(), String> {
    for stmt in &block.stmts {
        typecheck_stmt(stmt, env)?;
    }
    Ok(())
}

fn typecheck_stmt(stmt: &Stmt, env: &mut TypeEnvironment) -> Result<(), String> {
    match stmt {
        Stmt::Expr(expr) => typecheck_expr(expr, env),
        _ => unimplemented!(),
    }
}

fn typecheck_expr(expr: &Expr, env: &mut TypeEnvironment) -> Result<(), String> {
    match derive_type(expr, env) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn derive_type(expr: &Expr, env: &mut TypeEnvironment) -> Result<Type, String> {
    match expr {
        Expr::Number(_) => Ok(Type::Number),
        Expr::String(_) => Ok(Type::String),
        Expr::Ident(id) => match env.get_type(id) {
            Some(ty) => Ok(ty),
            None => Err(format!("Undefined identifier: {}", id)),
        },
        Expr::BinaryExp(left, opcode, right) => {
            let left_ty = derive_type(left, env)?;
            let right_ty = derive_type(right, env)?;
            match opcode {
                Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                    match (&left_ty, &right_ty) {
                        (Type::Number, Type::Number) => Ok(Type::Number),
                        _ => Err(format!(
                            "Type mismatch: {:?} {:?} {:?}",
                            &left_ty, opcode, &right_ty
                        )),
                    }
                }
            }
        }
    }
}

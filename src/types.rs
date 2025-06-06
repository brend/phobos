use crate::ast::{self, Expr, Opcode, Program};
use crate::ast::{Block, FunctionDecl, Stmt, TopLevelDecl};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Void,
    Number,
    String,
    Bool,
    Function(Vec<Type>, Box<Type>),
    Record(String, Vec<Field>),
}

impl From<ast::Type> for Type {
    fn from(ty: ast::Type) -> Self {
        match ty.name.as_str() {
            "Number" => Type::Number,
            "String" => Type::String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    name: String,
    ty: Type,
}

impl Field {
    pub fn new(name: String, ty: Type) -> Self {
        Field { name, ty }
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

pub fn typecheck(program: &Program) -> Result<(), String> {
    let mut env = TypeEnvironment::new();
    for decl in &program.top_level_decls {
        match decl {
            TopLevelDecl::FunctionDecl(func) => typecheck_function_decl(func, &mut env)?,
            TopLevelDecl::RecordDecl(name, fields) => {
                let mut record_fields = Vec::new();
                for field in fields {
                    record_fields.push(Field::new(field.name.clone(), field.ty.clone().into()));
                }
                let record_type = Type::Record(name.clone(), record_fields);
                env.set_type(name, record_type);
            }
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
    // Mark the state of the environment before checking the function body
    let before_check = env.types.clone();
    // Add the parameters to the environment
    for param in &func.params {
        env.types
            .push((param.name.clone(), param.ty.clone().into()));
    }
    // Typecheck the function body
    typecheck_block(&func.body, env, Some(func.ret.clone().into()))?;
    // Restore the environment state before checking the function body
    env.types = before_check;
    Ok(())
}

fn typecheck_block(
    block: &Block,
    env: &mut TypeEnvironment,
    return_type: Option<Type>,
) -> Result<(), String> {
    for stmt in &block.stmts {
        typecheck_stmt(stmt, env, return_type.clone())?;
    }
    Ok(())
}

fn typecheck_stmt(
    stmt: &Stmt,
    env: &mut TypeEnvironment,
    return_type: Option<Type>,
) -> Result<(), String> {
    match stmt {
        Stmt::Assign(id, expr) => {
            // look up the type of the identifier
            let ty_left = env
                .get_type(id)
                .ok_or_else(|| format!("Undefined identifier: {}", id))?;
            // derive the type of the expression
            let ty_right = derive_type(expr, env)?;
            // assignment is valid if the types are compatible
            if is_assignable(&ty_left, &ty_right) {
                Ok(())
            } else {
                Err(format!(
                    "Type mismatch: {:?} cannot be assigned to {:?}",
                    ty_right, ty_left
                ))
            }
        }
        Stmt::Let(id, ty, expr) => {
            // type of the assigned value must match the declared type
            let ty: Type = ty.clone().into();
            let ty_expr = derive_type(expr, env)?;
            if is_assignable(&ty, &ty_expr) {
                // modify the environment with the type of the identifier
                env.set_type(id, ty);
                Ok(())
            } else {
                Err(format!(
                    "Type mismatch: {:?} cannot be assigned to {:?}",
                    ty_expr, ty
                ))
            }
        }
        Stmt::Return(expr) => {
            let ty = derive_type(expr, env)?;
            // check if the return type matches the function's return type
            if let Some(ret) = return_type {
                if ty == ret {
                    Ok(())
                } else {
                    Err(format!(
                        "Type mismatch: {:?} cannot be returned from function with return type {:?}",
                        ty, ret
                    ))
                }
            } else {
                Ok(())
            }
        }
        Stmt::Expr(expr) => {
            derive_type(expr, env)?;
            Ok(())
        }
        Stmt::If(condition, then_branch, else_branch) => {
            let condition_ty = derive_type(condition, env)?;
            if condition_ty != Type::Bool {
                Err(format!(
                    "Condition must be a boolean, got {:?}",
                    condition_ty
                ))
            } else {
                typecheck_block(&then_branch, env, None)?;
                if let Some(else_branch) = else_branch {
                    typecheck_block(&else_branch, env, None)?;
                }
                Ok(())
            }
        }
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
                Opcode::Eq | Opcode::Neq | Opcode::Lt | Opcode::Le | Opcode::Gt | Opcode::Ge => {
                    match (&left_ty, &right_ty) {
                        (Type::Number, Type::Number) => Ok(Type::Bool),
                        _ => Err(format!(
                            "Type mismatch: {:?} {:?} {:?}",
                            &left_ty, opcode, &right_ty
                        )),
                    }
                }
            }
        }
        Expr::Call(func_name, args) => match env.get_type(func_name) {
            Some(Type::Function(arg_types, ret_type)) => {
                // check if argument types match function parameter types
                if arg_types.len() != args.len() {
                    Err(format!(
                        "Argument count mismatch: {:?} {:?}",
                        func_name, args
                    ))
                } else {
                    for (arg, ty) in args.iter().zip(arg_types.iter()) {
                        if !is_assignable(ty, &derive_type(arg, env)?) {
                            return Err(format!(
                                "Type mismatch: {:?} {:?}",
                                func_name,
                                env.get_type(func_name)
                            ));
                        }
                    }
                    Ok((*ret_type).clone())
                }
            }
            Some(_) => Err(format!(
                "Type mismatch: {:?} {:?}",
                func_name,
                env.get_type(func_name)
            )),
            None => Err(format!("Undefined function: {}", func_name)),
        },
    }
}

fn is_assignable(ty_left: &Type, ty_right: &Type) -> bool {
    match (ty_left, ty_right) {
        (Type::Number, Type::Number) => true,
        _ => false,
    }
}

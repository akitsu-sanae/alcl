/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::collections::HashMap;
use type_::Type;
use program::Program;
use expr::Expr;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Env {
    variables: Vec<(String, Type)>,
    structs: HashMap<String, Vec<(String, Type)>>,
}

impl Env {
    fn new() -> Self {
        Env {
            variables: vec![],
            structs: HashMap::new()
        }
    }

    fn add_variable(&mut self, name: String, ty: Type) {
        self.variables.push((name, ty));
    }
    fn append_variable(&mut self, mut args: Vec<(String, Type)>) {
        self.variables.append(&mut args);
    }
    fn add_struct(&mut self, name: String, args: Vec<(String, Type)>) {
        self.structs.insert(name, args);
    }
    fn lookup_var(&self, name: &String) -> Option<Type> {
        self.variables.iter().find(|ref e| e.0 == name.clone()).map(|ref e| e.1.clone())
    }
    fn lookup_struct(&self, name: &String) -> Option<Vec<(String, Type)>> {
        self.structs.get(name).cloned()
    }
}

pub fn type_check(program: &mut Program) {
    let mut env = Env::new();
    for ref func in &program.functions {
        env.add_variable(func.name.clone(), func.type_());
    }
    for (name, param) in program.structs.iter() {
        env.add_struct(name.clone(), param.clone());
    }

    for ref mut func in &mut program.functions {
        env.append_variable(func.args.clone());
        let ret_ty = type_check_impl(&mut func.body, &env).unwrap();
        if ret_ty != func.return_type {
            panic!("type error: not match return type: {:?} and {:?}", func.return_type, ret_ty);
        }
    }
}

fn type_check_impl(expr: &mut Expr, env: &Env) -> Result<Type, String> {
    use expr::Expr::*;
    match *expr {
        Let(ref name, box ref mut init, box ref mut body, ref mut info) => {
            let ty = type_check_impl(init, env)?;
            let mut env = env.clone();
            env.add_variable(name.clone(), ty);
            let ty = type_check_impl(body, &env)?;
            info.type_ = Some(ty.clone());
            Ok(ty)
        },
        Sequence(box ref mut e1, box ref mut e2, ref mut info) => {
            type_check_impl(e1, env)?;
            let ty = type_check_impl(e2, env)?;
            info.type_ = Some(ty.clone());
            Ok(ty)
        },
        If(box ref mut cond, box ref mut tr, ref mut else_if, box ref mut fl, ref mut info) => {
            let cond = type_check_impl(cond, env)?;
            if cond != Type::boolean() {
                return Err("if condition must be boolean".to_string())
            }
            let ret_ty = type_check_impl(tr, env)?;
            for ref mut else_if in else_if {
                let cond = type_check_impl(&mut else_if.0, env)?;
                if cond != Type::boolean() {
                    return Err("if condition must be boolean".to_string())
                }
                if ret_ty != type_check_impl(&mut else_if.1, env)? {
                    return Err("if result must have same type".to_string());
                }
            }
            if ret_ty != type_check_impl(fl, env)? {
                return Err("if result must hasve same type".to_string());
            }
            info.type_ = Some(ret_ty.clone());
            Ok(ret_ty)
        },
        For(ref mut index, box ref mut from, box ref mut to, box ref mut body, ref mut info) => {
            let from_ty = type_check_impl(from, env)?;
            let to_ty = type_check_impl(to, env)?;
            if from_ty != to_ty {
                return Err("type error in for expression: from expr and to expr must have same type ".to_string());
            }
            let mut env = env.clone();
            env.add_variable(index.clone(), from_ty);
            type_check_impl(body, &env)?;
            info.type_ = Some(Type::unit());
            Ok(Type::unit())
        },
        Subst(box ref mut lhs, box ref mut rhs, ref mut info) => {
            let lhs = type_check_impl(lhs, env)?;
            let rhs = type_check_impl(rhs, env)?;
            if lhs != rhs {
                Err("type error in subst expr: lhs and rhs have different types.".to_string())
            } else {
                info.type_ = Some(lhs);
                Ok(rhs)
            }
        },
        Equal(box ref mut lhs, box ref mut rhs, ref mut info) |
        NotEqual(box ref mut lhs, box ref mut rhs, ref mut info) => {
            let lhs = type_check_impl(lhs, env)?;
            let rhs = type_check_impl(rhs, env)?;
            if lhs != rhs {
                Err("lhs and rhs must have same type".to_string())
            } else {
                info.type_ = Some(Type::boolean());
                Ok(Type::boolean())
            }
        },
        Add(box ref mut lhs, box ref mut rhs, ref mut info) | Sub(box ref mut lhs, box ref mut rhs, ref mut info) |
        Mult(box ref mut lhs, box ref mut rhs, ref mut info) | Div(box ref mut lhs, box ref mut rhs, ref mut info) |
        Surplus(box ref mut lhs, box ref mut rhs, ref mut info) => {
            let lhs = type_check_impl(lhs, env)?;
            let rhs = type_check_impl(rhs, env)?;
            if lhs != rhs {
                Err("lhs and rhs must have same type".to_string())
            } else {
                info.type_ = Some(lhs.clone());
                Ok(lhs)
            }
        },
        Apply(box ref mut f, ref mut args, ref mut info) => {
            if let Type::Function(box ref mut ret_ty, ref mut params_ty) = type_check_impl(f, env)? {
                let args: Vec<_> = args.iter_mut().map(|e| type_check_impl(e, env).unwrap()).collect();
                if &args != params_ty {
                    return Err("type error: not match function arg types".to_string())
                }
                info.type_ = Some(ret_ty.clone());
                Ok(ret_ty.clone())
            } else {
                Err("type error: can not apply for non function expr".to_string())
            }
        }
        Construct(ref name, ref mut args, ref mut info) => {
            if let Some(params) = env.lookup_struct(name) {
                let args: Vec<_> = args.iter_mut()
                    .map(|&mut (ref name, ref mut e)| (name.clone(), type_check_impl(e, env).unwrap()))
                    .collect();
                if args != params {
                    return Err("type error: not match construct arg types".to_string());
                }
                let struct_ty = Type::Struct(name.clone(), params);
                info.type_ = Some(struct_ty.clone());
                Ok(struct_ty)
            } else {
                Err("can not construct non struct type".to_string())
            }
        }
        Dot(box ref mut e, ref name, ref mut info) => {
            match type_check_impl(e, env).unwrap() {
                Type::Struct(_, param) => {
                    let ty = param.iter().find(|ref arg| &arg.0 == name).unwrap().1.clone();
                    info.type_ = Some(ty.clone());
                    Ok(ty)
                },
                _ => Err("can not apply dot expr for non struct type".to_string())
            }
        }
        Println(box ref mut expr, ref mut info) => {
            if type_check_impl(expr, env)? != Type::string() {
                Err("println expr accepts only string expr".to_string())
            } else {
                info.type_ = Some(Type::unit());
                Ok(Type::unit())
            }
        }
        Number(_, ref mut info) => {
            info.type_ = Some(Type::integer());
            Ok(Type::integer())
        },
        String(_, ref mut info) => {
            info.type_ = Some(Type::string());
            Ok(Type::string())
        }
        Identifier(ref name, ref mut info) => {
            if let Some(ty) = env.lookup_var(name) {
                info.type_ = Some(ty.clone());
                Ok(ty)
            } else {
                Err(format!("error: inbound variable: {}", name))
            }
        }
        _ => Err(format!("unimplemented type check: {:?}", expr)) // TODO
    }
}




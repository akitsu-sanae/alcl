/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use type_::Type;
use program::Program;
use expr::Expr;

pub type Env = Vec<(String, Type)>;

pub fn type_check(program: &mut Program) {
    let mut env = vec![];
    for ref func in &program.functions {
        env.push((func.name.clone(), func.type_()));
    }

    for ref mut func in &mut program.functions {
        env.append(&mut func.args.clone());
        type_check_impl(&mut func.body, &env).unwrap();
    }
}

fn type_check_impl(expr: &mut Expr, env: &Env) -> Result<Type, String> {
    use expr::Expr::*;
    match *expr {
        Let(ref name, box ref mut init, box ref mut body, ref mut info) => {
            let ty = try!(type_check_impl(init, env));
            let mut env = env.clone();
            env.push((name.clone(), ty));
            let ty = try!(type_check_impl(body, &env));
            info.type_ = Some(ty.clone());
            Ok(ty)
        },
        Sequence(box ref mut e1, box ref mut e2, ref mut info) => {
            try!(type_check_impl(e1, env));
            let ty = try!(type_check_impl(e2, env));
            info.type_ = Some(ty.clone());
            Ok(ty)
        },
        If(box ref mut cond, box ref mut tr, ref mut else_if, box ref mut fl, ref mut info) => {
            let cond = try!(type_check_impl(cond, env));
            if cond != Type::boolean() {
                return Err("if condition must be boolean".to_string())
            }
            let ret_ty = try!(type_check_impl(tr, env));
            for ref mut else_if in else_if {
                let cond = try!(type_check_impl(&mut else_if.0, env));
                if cond != Type::boolean() {
                    return Err("if condition must be boolean".to_string())
                }
                if ret_ty != try!(type_check_impl(&mut else_if.1, env)) {
                    return Err("if result must have same type".to_string());
                }
            }
            if ret_ty != try!(type_check_impl(fl, env)) {
                return Err("if result must hasve same type".to_string());
            }
            info.type_ = Some(ret_ty.clone());
            Ok(ret_ty)
        },
        For(ref mut index, box ref mut from, box ref mut to, box ref mut body, ref mut info) => {
            let from_ty = try!(type_check_impl(from, env));
            let to_ty = try!(type_check_impl(to, env));
            if from_ty != to_ty {
                return Err("type error in for expression: from expr and to expr must have same type ".to_string());
            }
            let mut env = env.clone();
            env.push((index.clone(), from_ty));
            try!(type_check_impl(body, &env));
            info.type_ = Some(Type::unit());
            Ok(Type::unit())
        },
        Equal(box ref mut lhs, box ref mut rhs, ref mut info) |
        NotEqual(box ref mut lhs, box ref mut rhs, ref mut info) => {
            let lhs = try!(type_check_impl(lhs, env));
            let rhs = try!(type_check_impl(rhs, env));
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
            let lhs = try!(type_check_impl(lhs, env));
            let rhs = try!(type_check_impl(rhs, env));
            if lhs != rhs {
                Err("lhs and rhs must have same type".to_string())
            } else {
                info.type_ = Some(lhs.clone());
                Ok(lhs)
            }
        },
        Println(box ref mut expr, ref mut info) => {
            if try!(type_check_impl(expr, env)) != Type::string() {
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
            if let Some(ty) = lookup(env, name) {
                info.type_ = Some(ty.clone());
                Ok(ty)
            } else {
                Err(format!("error: inbound variable: {}", name))
            }
        }
        _ => Err(format!("unimplemented type check: {:?}", expr)) // TODO
    }
}


fn lookup(env: &Env, name: &String) -> Option<Type> {
    env.iter().find(|ref e| e.0 == name.clone()).map(|ref e| e.1.clone())
}


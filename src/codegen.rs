/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use kazuma;
use ast;

pub fn code_generate(program: &ast::Program) {
    let module = module(program);
    let builder = kazuma::Builder::new("test");
    match builder.module(&module) {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("error: {}", msg),
    }
}

fn module(program: &ast::Program) -> kazuma::ast::Module {
    kazuma::ast::Module {
        functions: program.functions.iter().map(function).collect()
    }
}

fn type_(ty: &ast::Type) -> kazuma::ast::Type {
    match *ty {
        ast::Type::Char => panic!("unimplemented"),
        ast::Type::Int => kazuma::ast::Type::Integer,
        ast::Type::Struct(_) => panic!("unimplemented"),
    }
}

fn argument(args: &Vec<(String, ast::Type)>) -> Vec<(String, kazuma::ast::Type)> {
    args.iter()
        .map(|&(ref name, ref ty)| {
            (name.clone(), type_(ty))
        })
        .collect()
}

fn function(func: &ast::Function) -> kazuma::ast::Function {
    kazuma::ast::Function {
        name: func.name.clone(),
        arguments: argument(&func.args),
        return_type: type_(&func.return_type),
        body: {
            let (ref statements, ref expr) = func.body;
            statements.iter()
                .rev()
                .fold(expression(expr), |e, stmt| {
                    kazuma::ast::Expression::BinOp(
                        kazuma::ast::BinaryOperator::Sequent,
                        box statement(stmt), box e)
                })
        }
    }
}

// TODO
fn type_of_expr(expr: &ast::Expr) -> kazuma::ast::Type {
    kazuma::ast::Type::Integer
}

fn statement(stmt: &ast::Statement) -> kazuma::ast::Expression {
    match *stmt {
        ast::Statement::Let(ref name, ref init) => {
            let ty = type_of_expr(&init);
            let init = expression(init);
            kazuma::ast::Expression::Let(name.clone(), ty, box init)
        },
        ast::Statement::Println(ref expr) => kazuma::ast::Expression::Print(box expression(expr)),
        ast::Statement::Expression(ref expr) => expression(expr),
    }
}

fn binary_operation(e: &ast::Expr) -> Option<kazuma::ast::BinaryOperator> {
    use ast::Expr::*;
    match *e {
        Add(_, _) => Some(kazuma::ast::BinaryOperator::Add),
        Sub(_, _) => Some(kazuma::ast::BinaryOperator::Sub),
        Mult(_, _) => Some(kazuma::ast::BinaryOperator::Mult),
        Div(_, _) => Some(kazuma::ast::BinaryOperator::Div),
        _ => None,
    }
}

fn expression(expr: &ast::Expr) -> kazuma::ast::Expression {
    use ast::Expr::*;
    match *expr {
        Add(box ref lhs, box ref rhs) | Sub(box ref lhs, box ref rhs) |
        Mult(box ref lhs, box ref rhs) | Div(box ref lhs, box ref rhs) => {
            let op = binary_operation(expr).unwrap();
            let lhs = expression(lhs);
            let rhs = expression(rhs);
            kazuma::ast::Expression::BinOp(op, box lhs, box rhs)
        },
        If(_, _, _) => panic!("unimplemented: if"),
        Literal(ref lit) => kazuma::ast::Expression::Literal(literal(lit)),
        Var(ref name) => kazuma::ast::Expression::Variable(name.clone()),
    }
}

fn literal(lit: &ast::Literal) -> kazuma::ast::Literal {
    match *lit {
        ast::Literal::Char(_) | ast::Literal::Struct(_) => panic!("unimplemented literal"),
        ast::Literal::Int(ref n) => kazuma::ast::Literal::Integer(*n),
        ast::Literal::String(ref str) => kazuma::ast::Literal::String(str.clone()),
    }
}

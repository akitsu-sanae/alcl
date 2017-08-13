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
    let mut builder = kazuma::builder::Builder::new("test");
    match builder.build(&module) {
        Ok(result) => println!("{}", result),
        Err(msg) => println!("error: {}", msg),
    }
}

fn module(program: &ast::Program) -> kazuma::ast::Module {
    kazuma::ast::Module {
        structs: program.struct_data.iter().map(struct_).collect(),
        functions: program.functions.iter().map(function).collect()
    }
}

fn type_(ty: &ast::Type) -> kazuma::ast::Type {
    match *ty {
        ast::Type::Char => panic!("unimplemented"),
        ast::Type::Int => kazuma::ast::Type::Int32,
        ast::Type::Array(box ref ty, ref n) => kazuma::ast::Type::Array(box type_(ty), *n),
        ast::Type::Struct(ref name) => kazuma::ast::Type::Struct(name.clone()),
    }
}

fn struct_(strct: &ast::StructType) -> kazuma::ast::Struct {
    kazuma::ast::Struct {
        name: strct.name.clone(),
        fields: strct.data.iter().map(|(name, ty)| {
            (name.clone(), type_(ty))
        }).collect(),
    }
}

fn function(func: &ast::Function) -> kazuma::ast::Function {
    let (arg_names, arg_types) =
        func.args.iter()
        .fold((vec![], vec![]), |(mut name_acc, mut ty_acc), &(ref name, ref ty)| {
            name_acc.push(name.clone());
            ty_acc.push(type_(ty));
            (name_acc, ty_acc)
        }
    );
    let ret_type = type_(&func.return_type);
    let typ = kazuma::ast::Type::Function(arg_types, box ret_type);
    let body = expression(&func.body);

    kazuma::ast::Function {
        name: func.name.clone(),
        arg_names: arg_names,
        typ: typ,
        body: body,
    }
}

fn expression(expr: &ast::Expr) -> kazuma::ast::Expression {
    use ast::Expr::*;
    match *expr {
        Let(ref name, box ref init) => kazuma::ast::Expression::Let(name.clone(), box expression(init)),
        Block(ref exprs) => kazuma::ast::Expression::Block(exprs.iter().map(expression).collect()),
        Add(box ref lhs, box ref rhs) => {
            let (lhs, rhs) = (expression(lhs), expression(rhs));
            kazuma::ast::Expression::Add(box lhs, box rhs)
        },
        Sub(box ref lhs, box ref rhs) => {
            let (lhs, rhs) = (expression(lhs), expression(rhs));
            kazuma::ast::Expression::Sub(box lhs, box rhs)
        },
        Mult(box ref lhs, box ref rhs) => {
            let (lhs, rhs) = (expression(lhs), expression(rhs));
            kazuma::ast::Expression::Mult(box lhs, box rhs)
        },
        Div(box ref lhs, box ref rhs) => {
            let (lhs, rhs) = (expression(lhs), expression(rhs));
            kazuma::ast::Expression::Div(box lhs, box rhs)
        },
        If(_, _, _) => panic!("unimplemented: if"),
        Var(ref name) => kazuma::ast::Expression::Variable(name.clone()),
        Int(ref n) => kazuma::ast::Expression::Int(*n),
        Array(ref exprs) => kazuma::ast::Expression::Array(exprs.iter().map(expression).collect()),
        Struct(ref name, ref fields) => {
            kazuma::ast::Expression::Struct(
                name.clone(),
                fields.iter().map(|(name, field)| (name.clone(), expression(field))).collect())
        },
    }
}


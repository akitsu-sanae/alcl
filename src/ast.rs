/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StructType {
    pub name: String,
    pub data: HashMap<String, Type>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Char, Int,
    Struct(String)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub struct_data: Vec<StructType>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Expr,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Let(String, Box<Expr>),
    Block(Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Var(String),
    Int(i64),
    Struct(String, HashMap<String, Expr>),
}


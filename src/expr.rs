/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use type_::Type;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Info {
    pub type_ : Option<Type>
}

impl Info {
    pub fn new() -> Self {
        Info {
            type_: None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Let(String, Box<Expr>, Box<Expr>, Info),
    Sequence(Box<Expr>, Box<Expr>, Info),
    If(Box<Expr>, Box<Expr>, Vec<(Expr, Expr)>, Box<Expr>, Info),
    For(String, Box<Expr>, Box<Expr>, Box<Expr>, Info),
    Equal(Box<Expr>, Box<Expr>, Info),
    NotEqual(Box<Expr>, Box<Expr>, Info),
    Add(Box<Expr>, Box<Expr>, Info),
    Sub(Box<Expr>, Box<Expr>, Info),
    Surplus(Box<Expr>, Box<Expr>, Info),
    Mult(Box<Expr>, Box<Expr>, Info),
    Div(Box<Expr>, Box<Expr>, Info),
    Apply(Box<Expr>, Box<Expr>, Info),
    Dot(Box<Expr>, Box<Expr>, Info),
    Number(i64, Info),
    String(String, Info),
    Identifier(String, Info)
}

impl Expr {
    pub fn is_literal(&self) -> bool {
        use expr::Expr::*;
        match *self {
            Number(_, _) | String(_, _) => true,
            _ => false

        }
    }

    pub fn operand(&self) -> String {
        use expr::Expr::*;
        match *self {
            Add(_, _, _) => "add".to_string(),
            Sub(_, _, _) => "sub".to_string(),
            Mult(_, _, _) => "mul".to_string(),
            Div(_, _, _) => "sdiv".to_string(),
            _ => panic!("this expression has no operand: {:?}", self),
        }
    }
}


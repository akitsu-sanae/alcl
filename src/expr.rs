/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Let(String, Box<Expr>, Box<Expr>),
    Sequence(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Vec<(Box<Expr>, Box<Expr>)>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Surplus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Dot(Box<Expr>, Box<Expr>),
    Number(i64),
    String(String),
    Identifier(String)
}

impl Expr {
    pub fn is_literal(&self) -> bool {
        use expr::Expr::*;
        match *self {
            Number(_) | String(_) => true,
            _ => false

        }
    }
}


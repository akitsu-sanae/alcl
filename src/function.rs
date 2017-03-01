/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use expr::Expr;
use type_::Type;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    name: String,
    args: Vec<String>,
    type_: Type,
    body: Expr
}

impl Function {
    pub fn new(name: String, args: Vec<String>, ty: Type, e: Expr) -> Self {
        Function {
            name : name,
            args: args,
            type_: ty,
            body: e
        }
    }
}



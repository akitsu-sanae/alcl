/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use function::Function;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>
}

impl Program {
    pub fn new() -> Self {
        Program {
            functions: vec![]
        }
    }
}


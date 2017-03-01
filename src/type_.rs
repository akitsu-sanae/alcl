/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Primitive(String),
    Function(Box<Type>, Box<Type>),
    Generic(String, Box<Type>)
}


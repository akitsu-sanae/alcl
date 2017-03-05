/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Primitive(String),
    Function(Box<Type>, Vec<Type>),
    Generic(String, Box<Type>)
}

impl Type {
    pub fn integer() -> Self {
        Type::Primitive("Int".to_string())
    }

    pub fn boolean() -> Self {
        Type::Primitive("Bool".to_string())
    }

    pub fn string() -> Self {
        Type::Primitive("String".to_string())
    }

    pub fn unit() -> Self {
        Type::Primitive("Unit".to_string())
    }

    pub fn align(&self) -> usize {
        use type_::Type::*;
        match *self {
            Primitive(ref name) => {
                match name.as_str() {
                    "Bool" => 1,
                    "Int" => 4,
                    "Char" => 1,
                    "String" => 8,
                    _ => 42, // TODO
                }
            },
            Generic(ref name, _) => {
                match name.as_str() {
                    "List" | "Array" => 8,
                    _ => 42 // TODO
                }
            }
            _ => 42 // TODO
        }
    }
}


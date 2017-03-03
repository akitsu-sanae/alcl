/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use expr::Expr;
use function::Function;
use type_::Type;
use program::Program;

pub struct CodeGen {
    variable_counter: i32,
    global_declares : String,
    n_string_literals: i32,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            variable_counter: 0,
            global_declares: String::new(),
            n_string_literals: 0
        }
    }

    pub fn program(&mut self, program: &Program) -> String {
        let mut result = String::new();
        for ref func in &program.functions {
            result += self.function(&func).as_str();
        }
        format!("{}{}", self.global_declares, result)
    }

    pub fn function(&mut self, func: &Function) -> String {
        let mut result = format!("define {} @{} ({}) {{\n",
            self.type_(&func.return_type),
            func.name,
            self.parameters(&func.args)
        );
        for (i, args) in func.args.iter().enumerate() {
            let (ty, align) = (self.type_(&args.1), args.1.align());
            result += format!("  %{} = alloca {}, align {}\n", args.0, ty, align).as_str();
            result += format!("  store {} %.arg{}, {}* %{}, align {}\n", ty, i, ty, args.0, align).as_str();
        }
        self.variable_counter = 0;
        result += self.expression(&func.body).as_str();
        result += format!("  ret i32 %{}\n", self.variable_counter).as_str();
        result += "}";
        result
    }

    pub fn type_(&mut self, ty: &Type) -> String {
        match *ty {
            Type::Primitive(ref name) => {
                match name.as_str() {
                    "Int" => "i32".to_string(),
                    "Unit" => "void".to_string(),
                    "RawString" => "i8*".to_string(),
                    _ => "<unimplemented primitive type>".to_string() // TODO
                }
            },
            Type::Generic(ref name, box ref inner_type) => {
                match name.as_str() {
                    "List" | "Array" => {
                        format!("{}*", self.type_(inner_type))
                    },
                    _ => "<unimplemented generic type>".to_string() // TODO
                }
            }
            _ => "<unimplemented type>".to_string() // TODO
        }
    }

    pub fn parameters(&mut self, args: &Vec<(String, Type)>) -> String {
        if args.is_empty() {
            "".to_string()
        } else {
            args[1..].iter().enumerate().fold(
                format!("{} %.arg0", self.type_(&args[0].1)),
                |acc, (i, arg)| format!("{}, {} %.arg{}", acc, self.type_(&arg.1), i+1)
            )
        }
    }

    pub fn expression(&mut self, e: &Expr) -> String {
        use expr::Expr::*;
        match *e {
            Let(ref id, box ref init, box ref body, _) => {
                let (before, init) = if init.is_literal() {
                    ("".to_string(), self.expression(init))
                } else {
                    (self.expression(init), format!("%{}", self.variable_counter))
                };
                let after = self.expression(body);
                format!("{}  %{} = alloca i32, align 4\n  store i32 {}, i32* %{}, align 4\n{}",
                        before, id, init, id, after)
            },
            Sequence(box ref e1, box ref e2, _) => {
                format!("{}{}", self.expression(e1), self.expression(e2))
            },
            Add(box ref lhs, box ref rhs, _) | Sub(box ref lhs, box ref rhs, _) |
            Mult(box ref lhs, box ref rhs, _) | Div(box ref lhs, box ref rhs, _) => {
                let (before, lhs, rhs) = if lhs.is_literal() && rhs.is_literal() {
                    ("".to_string(), self.expression(lhs), self.expression(rhs))
                } else if lhs.is_literal() && !rhs.is_literal() {
                    (self.expression(rhs), self.expression(lhs), format!("%{}", self.variable_counter))
                } else if !lhs.is_literal() && rhs.is_literal() {
                    (self.expression(lhs), format!("%{}", self.variable_counter), self.expression(rhs))
                } else {
                    let mut before = self.expression(lhs);
                    let lhs = self.variable_counter;
                    before += self.expression(rhs).as_str();
                    let rhs = self.variable_counter;
                    (before, format!("%{}", lhs), format!("%{}", rhs))
                };
                self.variable_counter += 1;
                format!("{}  %{} = {} i32 {}, {}\n", before, self.variable_counter, e.operand(), lhs, rhs)
            },
            Number(ref n, _) => n.to_string(),
            String(ref str, _) => {
                self.n_string_literals += 1;
                self.global_declares += format!("@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n",self.n_string_literals, str.len()+1, str).as_str();
                format!("@.str.{}", self.n_string_literals)
            },
            Identifier(ref name, _) => {
                self.variable_counter += 1;
                format!("  %{} = load i32, i32* %{}, align 4\n", self.variable_counter, name)
            },
            _ => "<unimplemented expr>".to_string() // TODO
        }
    }
}


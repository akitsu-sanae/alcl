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

use std::collections::HashMap;

pub struct CodeGen {
    variable_counter: i32,
    //                         name   ret_type arg_type
    global_declares : HashMap<String, (String, String)>,
    string_literals: Vec<String>
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            variable_counter: 0,
            global_declares: HashMap::new(),
            string_literals: vec![]
        }
    }

    pub fn program(&mut self, program: &Program) -> String {
        let mut result = String::new();
        for ref func in &program.functions {
            result += self.function(&func).as_str();
        }
        for ref declare in &self.global_declares {
            let ref ty = declare.1;
            let (name, ret_ty, arg_ty) = (&declare.0, &ty.0, &ty.1);
            result += format!("declare {} @{}({})\n", ret_ty, name, arg_ty).as_str();
        }
        for (i, str) in self.string_literals.iter().enumerate() {
            result += format!("@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", i, str.len()+1, str).as_str();
        }
        result
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
        result += "}\n";
        result
    }

    pub fn type_(&mut self, ty: &Type) -> String {
        match *ty {
            Type::Primitive(ref name) => {
                match name.as_str() {
                    "Bool" => "i1".to_string(),
                    "Int" => "i32".to_string(),
                    "Unit" => "void".to_string(),
                    "RawString" | "String" => "i8*".to_string(),
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
                let init_ty = init.type_().unwrap();
                let (before, init) = if init.is_literal() {
                    ("".to_string(), self.expression(init))
                } else {
                    (self.expression(init), format!("%{}", self.variable_counter))
                };
                let (init_ty, align) = (self.type_(&init_ty), init_ty.align());
                let after = self.expression(body);
                format!("{}  %{} = alloca {}, align {}\n  store {} {}, {}* %{}, align {}\n{}",
                        before, id, init_ty, align,
                        init_ty, init, init_ty, id, align, after)
            },
            Sequence(box ref e1, box ref e2, _) => {
                let mut expr_to_string = |e: &Expr| {
                    let ty = e.type_().unwrap();
                    let (ty, align) = (self.type_(&ty), ty.align());
                    if e.is_literal() {
                        let e = self.expression(e);
                        self.variable_counter += 2;
                        format!("  %{} = alloca {}, align {}\n  store {} {}, {}* %{}, align {}\n  %{} = load {}, {}* %{}, align {}\n",
                                self.variable_counter - 1, ty, align,
                                ty, e, ty, self.variable_counter - 1, align,
                                self.variable_counter, ty, ty, self.variable_counter-1, align)
                    } else {
                        self.expression(e)
                    }
                };
                format!("{}{}", expr_to_string(e1), expr_to_string(e2))
            },
            If(box ref cond, box ref tr, ref else_ifs, box ref fl, _) => {
                let mut result = "".to_string();
                result += self.expression(cond).as_str();
                if else_ifs.len() == 0 {
                    result += format!("  br i1 %{}, label %.if.then, label %.if.else\n", self.variable_counter).as_str();
                    result += "\n";
                    result += ".if.then:\n";
                    result += self.expression(tr).as_str();
                    result += "  br label %.if.end\n";
                    result += "\n";
                    result += ".if.else:\n";
                    result += self.expression(fl).as_str();
                    result += "  br label %.if.end\n";
                    result += "\n";
                    result += ".if.end:\n";
                } else {
                    result += format!("  br i1 %{}, label %.if.then, label %.if.else_if.0.cond\n", self.variable_counter).as_str();
                    result += "\n";
                    result += ".if.then:\n";
                    result += self.expression(tr).as_str();
                    result += "  br label %.if.end\n";
                    result += "\n";
                    for (i, else_if) in else_ifs.iter().enumerate() {
                        result += format!(".if.else_if.{}.cond:\n", i).as_str();
                        result += self.expression(&else_if.0).as_str();
                        let after = if i + 1 == else_ifs.len() {
                            ".if.else".to_string()
                        } else {
                            format!(".if.else_if.{}.cond", i+1)
                        };
                        result += format!("  br i1 %{}, label %.if.else_if.{}.body, label %{}\n", self.variable_counter, i, after).as_str();
                        result += "\n";
                        result += format!(".if.else_if.{}.body:\n", i).as_str();
                        result += self.expression(&else_if.1).as_str();
                        result += "  br label %.if.end\n";
                        result += "\n";
                    }
                    result += ".if.else:\n";
                    result += self.expression(fl).as_str();
                    result += "  br label %.if.end\n";
                    result += "\n";
                    result += ".if.end:\n";
                }
                result
            },
            For(ref name, box ref from, box ref to, box ref expr, _) => {
                let mut result = "".to_string();
                result += format!("  %{} = alloca i32, align 4\n", name).as_str();
                result += format!("  store i32 {}, i32* %{}, align 4\n", self.expression(from), name).as_str();
                result += "  br label %.for_cond\n";
                result += "\n";
                result += ".for_cond:\n";
                self.variable_counter += 1;
                result += format!("  %{} = load i32, i32* %{}, align 4\n", self.variable_counter, name).as_str();
                self.variable_counter += 1;
                let to = self.expression(to);
                result += format!("  %{} = icmp sle i32 %{}, {}\n", self.variable_counter, self.variable_counter-1, to).as_str();
                result += format!("  br i1 %{}, label %.for_body, label %.for_end\n", self.variable_counter).as_str();
                result += "\n";
                result += ".for_body:\n";
                result += self.expression(expr).as_str();
                self.variable_counter += 1;
                result += format!("  %{} = load i32, i32* %i, align 4\n", self.variable_counter).as_str();
                self.variable_counter += 1;
                result += format!("  %{} = add i32 %{}, 1", self.variable_counter, self.variable_counter-1).as_str();
                result += format!("  store i32 %{}, i32* %{}, align 4", self.variable_counter, name).as_str();
                result += "br label %.for_cond\n";
                result += "\n";
                result += ".for_end:\n";
                result
            },
            Equal(box ref lhs, box ref rhs, _) | NotEqual(box ref lhs, box ref rhs, _) |
            Add(box ref lhs, box ref rhs, _) | Sub(box ref lhs, box ref rhs, _) |
            Mult(box ref lhs, box ref rhs, _) | Div(box ref lhs, box ref rhs, _) |
            Surplus(box ref lhs, box ref rhs, _) => {
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
            Println(box ref expr, _) => {
                self.global_declares.insert("puts".to_string(), ("i32".to_string(), "i8*".to_string()));
                if expr.is_literal() {
                    let var = self.expression(expr);
                    self.variable_counter += 1;
                    format!("  %{} = call i32 @puts(i8* {})\n", self.variable_counter, var)
                } else {
                    let before = self.expression(expr);
                    let var = self.variable_counter;
                    self.variable_counter += 1;
                    format!("{}  %{} = call i32 @puts(i8* %{})\n", before, self.variable_counter, var)
                }
            },
            Number(ref n, _) => n.to_string(),
            String(ref str, _) => {
                self.string_literals.push(str.clone());
                format!("getelementptr inbounds ([{} x i8], [{} x i8]* @.str.{}, i32 0, i32 0)", str.len()+1, str.len()+1, self.string_literals.len()-1)
            },
            Identifier(ref name, ref info) => {
                self.variable_counter += 1;
                let ty = info.clone().type_.unwrap();
                let (ty, align) = (self.type_(&ty), ty.align());
                format!("  %{} = load {}, {}* %{}, align {}\n", self.variable_counter, ty, ty, name, align)
            },
            _ => "<unimplemented expr>\n".to_string() // TODO
        }
    }
}


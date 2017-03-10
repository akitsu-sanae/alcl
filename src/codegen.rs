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

        for (_, ty) in &program.types {
            match *ty {
                Type::Struct(ref name, ref params) => {
                    let params = params[1..].iter().fold(
                        format!("{}", self.type_(&params[0].1)),
                        |acc, &(_, ref ty)| {
                            format!("{}, {}", acc, self.type_(ty))
                        });
                    result += format!("%{} = type {{ {} }}\n", name, params).as_str();
                },
                _ => ()
            }
        }

        result += "\n";

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
        let ty = self.type_(&func.body.type_().unwrap());
        if ty != "void" {
            result += format!("  ret {} %{}\n", ty, self.variable_counter).as_str();
        } else {
            result += "  ret void\n";
        }
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
            },
            Type::Struct(ref name, _) => format!("%{}", name),
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
                let mut result = self.expression(init);
                let init_var = self.variable_counter;
                let (init_ty, align) = (self.type_(&init_ty), init_ty.align());
                result += format!("  %{} = alloca {}, align {}\n", id, init_ty, align).as_str();
                result += format!("  store {} %{}, {}* %{}, align {}\n", init_ty, init_var, init_ty, id, align).as_str();
                result += self.expression(body).as_str();
                result
            },
            Sequence(box ref e1, box ref e2, _) => {
                let e1 = self.expression(e1);
                let e2 = self.expression(e2);
                format!("{}{}", e1, e2)
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
                let mut result = self.expression(from);
                let from_var = self.variable_counter;
                result += format!("  %{} = alloca i32, align 4\n", name).as_str();
                result += format!("  store i32 %{}, i32* %{}, align 4\n", from_var, name).as_str();
                result += "  br label %.for_cond\n";
                result += "\n";
                result += ".for_cond:\n";
                self.variable_counter += 1;
                result += format!("  %{} = load i32, i32* %{}, align 4\n", self.variable_counter, name).as_str();
                let index_var = self.variable_counter;
                result += self.expression(to).as_str();
                let to_var = self.variable_counter;
                self.variable_counter += 1;
                result += format!("  %{} = icmp sle i32 %{}, %{}\n", self.variable_counter, index_var, to_var).as_str();
                result += format!("  br i1 %{}, label %.for_body, label %.for_end\n", self.variable_counter).as_str();
                result += "\n";
                result += ".for_body:\n";
                result += self.expression(expr).as_str();
                self.variable_counter += 1;
                result += format!("  %{} = load i32, i32* %i, align 4\n", self.variable_counter).as_str();
                self.variable_counter += 1;
                result += format!("  %{} = add i32 %{}, 1", self.variable_counter, self.variable_counter-1).as_str();
                result += format!("  store i32 %{}, i32* %{}, align 4\n", self.variable_counter, name).as_str();
                result += "br label %.for_cond\n";
                result += "\n";
                result += ".for_end:\n";
                result
            },
            Subst(box ref lhs, box ref rhs, _) => {
                let ty = rhs.type_().unwrap();
                let (ty, align) = (self.type_(&ty), ty.align());
                let mut result = self.expression(rhs);
                let rhs_var = self.variable_counter;
                if let &Expr::Identifier(ref name, _) = lhs {
                    result += format!("  store {} %{}, {}* %{}, align {}", ty, rhs_var, ty, name, align).as_str();
                } else {
                    result += self.expression(lhs).as_str();
                    let lhs_var = self.variable_counter;
                    result += format!("  store {} %{}, {}* %{}, align {}\n", ty, rhs_var, ty, lhs_var, align).as_str();
                }
                result
            },
            Equal(box ref lhs, box ref rhs, _) | NotEqual(box ref lhs, box ref rhs, _) |
            Add(box ref lhs, box ref rhs, _) | Sub(box ref lhs, box ref rhs, _) |
            Mult(box ref lhs, box ref rhs, _) | Div(box ref lhs, box ref rhs, _) |
            Surplus(box ref lhs, box ref rhs, _) => {
                let mut result = self.expression(lhs);
                let lhs = self.variable_counter;
                result += self.expression(rhs).as_str();
                let rhs = self.variable_counter;
                self.variable_counter += 1;
                result += format!("  %{} = {} i32 %{}, %{}\n", self.variable_counter, e.operand(), lhs, rhs).as_str();
                result
            },
            Apply(box ref f, ref args, ref info) => {
                if let &Expr::Identifier(ref name, _) = f {
                    let mut result = "".to_string();
                    let args: Vec<_> = args.iter().map(|arg| {
                        result += self.expression(&arg).as_str();
                        (self.type_(&arg.type_().unwrap()), self.variable_counter)
                    }).collect();
                    let args = args[1..].iter().fold(
                        format!("{} %{}", args[0].0, args[0].1),
                        |acc, &(ref ty, ref var)| {
                            format!("{}, {} %{}", acc, ty, var)
                        });
                    let ret_ty = self.type_(&info.clone().type_.unwrap());
                    self.variable_counter += 1;
                    result += format!("  %{} = call {} @{}({})\n", self.variable_counter, ret_ty, name, args).as_str();
                    result
                } else {
                    panic!("nyan")
                }
            },
            Construct(ref name, ref args, ref info) => {
                let align = info.clone().type_.unwrap().align();
                self.variable_counter += 1;
                let mut result = format!("  %{} = alloca %{}, align {}\n", self.variable_counter, name, align);
                let var = self.variable_counter;

                for (i, &(_, ref arg)) in args.iter().enumerate() {
                    let arg_ty = arg.type_().unwrap();
                    let arg_align = arg_ty.align();
                    let arg_ty = self.type_(&arg_ty);
                    self.variable_counter += 1;
                    result += format!("  %{} = getelementptr inbounds %{}, %{}* %{}, i32 0, i32 {}\n", self.variable_counter, name, name, var, i).as_str();
                    let target_ptr = self.variable_counter;
                    result += self.expression(arg).as_str();
                    let arg_var = self.variable_counter;
                    result += format!("  store {} %{}, {}* %{}, align {}\n",
                                      arg_ty, arg_var, arg_ty, target_ptr, arg_align).as_str();
                }
                self.variable_counter += 1;
                result += format!("  %{} = load %{}, %{}* %{}, align {}\n", self.variable_counter, name, name, var, align).as_str();
                result
            },
            Dot(box ref expr, ref name, _) => {
                let expr_ty = expr.type_().unwrap();
                match expr_ty {
                    Type::Struct(ref struct_name, ref data) => {
                        let pos = data.iter().position(|branch| {
                            &branch.0 == name
                        }).unwrap();
                        let mut result = self.expression(expr);
                        let struct_var = self.variable_counter;
                        self.variable_counter += 1;
                        result += format!("  %{} = getelementptr inbounds %{}, %{}* %{}, i32 0, i32 {}\n",
                                self.variable_counter, struct_name, struct_name, struct_var, pos).as_str();
                        result
                    },
                    _ => panic!("can not apply dor expr for non struct expr: {:?}", expr_ty)
                }
            },
            Println(box ref expr, _) => {
                self.global_declares.insert("puts".to_string(), ("i32".to_string(), "i8*".to_string()));
                let mut result = self.expression(expr);
                let var = self.variable_counter;
                self.variable_counter += 1;
                result += format!("  %{} = call i32 @puts(i8* %{})\n", self.variable_counter, var).as_str();
                result
            },
            Number(ref n, _) => {
                self.variable_counter += 1;
                let mut result = format!("  %{} = alloca i32, align 4\n", self.variable_counter);
                result += format!("  store i32 {}, i32* %{}, align 4\n", n, self.variable_counter).as_str();
                self.variable_counter += 1;
                result += format!("  %{} = load i32, i32* %{}, align 4\n", self.variable_counter, self.variable_counter-1).as_str();
                result
            },
            String(ref str, _) => {
                self.string_literals.push(str.clone());
                self.variable_counter += 1;
                let mut result = format!("  %{} = alloca i8*, align 8\n", self.variable_counter);
                result += format!("  store i8* getelementptr inbounds ([{} x i8], [{} x i8]* @.str.{}, i32 0, i32 0), i8** %{}, align 4\n",
                    str.len()+1, str.len()+1, self.string_literals.len()-1, self.variable_counter).as_str();
                self.variable_counter += 1;
                result += format!("  %{} = load i8*, i8** %{}, align 8\n", self.variable_counter, self.variable_counter-1).as_str();
                result
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


/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(box_syntax)]
#![feature(box_patterns)]

mod expr;
mod function;
mod type_;
mod program;
mod parse;
mod codegen;

fn main() {
    let code = r#"
func main argc: Int argv: List[RawString] : Int {
    let a = 1;
    a+2+3
}
"#;

    let mut parser = parse::Parser::new(code);
    let ast = parser.program();
    let mut codegen = codegen::CodeGen::new();
    println!("{}", codegen.program(&ast));
}

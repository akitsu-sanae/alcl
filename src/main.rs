/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax)]
#![feature(box_patterns)]

mod expr;
mod function;
mod type_;
mod program;
mod type_check;
mod codegen;

peg_file! parse("grammar.rustpeg");

fn main() {
    use std::io::Read;
    let filename = std::env::args().nth(1).expect("[filename] is required");
    let mut f = std::fs::File::open(filename).expect("not found: inputed filename");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("can not read input file");

    let mut ast = parse::program(input.as_str()).unwrap();
    type_check::type_check(&mut ast);
    let mut codegen = codegen::CodeGen::new();
    println!("{}", codegen.program(&ast));
}


#[cfg(test)]
mod parse_test {
    use super::expr::*;
    use super::function::Function;
    use super::type_::Type;
    use super::parse;

    #[test]
    fn expression() {
        assert_eq!(parse::expr("123"), Ok(Expr::Number(123, Info::new())));
        assert_eq!(parse::expr("\"hoge\""), Ok(Expr::String("hoge".to_string(), Info::new())));
        assert_eq!(parse::expr("hoge"), Ok(Expr::Identifier("hoge".to_string(), Info::new())));

        assert_eq!(parse::expr("println hoge"), Ok(Expr::Println(
                    box Expr::Identifier("hoge".to_string(), Info::new()),
                    Info::new())));
        assert_eq!(parse::expr("hoge.fuga.piyo"), Ok(Expr::Dot(
                    box Expr::Dot(
                        box Expr::Identifier("hoge".to_string(), Info::new()),
                        "fuga".to_string(),
                        Info::new()),
                    "piyo".to_string(),
                    Info::new())));
        assert_eq!(parse::expr("123+456"), Ok(Expr::Add(
                    box Expr::Number(123, Info::new()),
                    box Expr::Number(456, Info::new()),
                    Info::new())));
    }

    #[test]
    fn type_() {
        assert_eq!(parse::type_("Int"), Ok(Type::Primitive("Int".to_string())));
        assert_eq!(parse::type_("List[Int]"), Ok(Type::Generic(
                        "List".to_string(),
                        box Type::Primitive("Int".to_string()))));
        assert_eq!(parse::type_("(Int, String) -> Float"), Ok(Type::Function(
                    box Type::Primitive("Float".to_string()),
                    vec![
                        Type::Primitive("Int".to_string()),
                        Type::Primitive("String".to_string())
                    ])));
        assert_eq!(parse::type_("(List[Int], String) -> Float"), Ok(Type::Function(
                    box Type::Primitive("Float".to_string()),
                    vec![
                    Type::Generic(
                        "List".to_string(),
                        box Type::Primitive("Int".to_string())),
                    Type::Primitive("String".to_string())
                    ])));

    }

    #[test]
    fn function() {
        assert_eq!(parse::function("func hoge arg: String : Int { 1 }"),
        Ok(Function{
            name: "hoge".to_string(),
            args: vec![("arg".to_string(), Type::Primitive("String".to_string()))],
            return_type: Type::Primitive("Int".to_string()),
            body: Expr::Number(1, Info::new())
        }))
    }

}



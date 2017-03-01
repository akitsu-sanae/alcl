/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(box_syntax)]

mod expr;
mod function;
mod type_;
mod program;
mod parse;

fn main() {
    let mut parser = parse::Parser::new("5+(1+2)*3");
    println!("{:?}", parser.expression());

    let code = r#"
func fizzbuzz n : Int -> Unit {
    for i in 1 .. n {
        if i%15 == 0 {
            std.io.printf @ "FizzBuzz"
        } else if i%3 == 0 {
            std.io.printf @ "Fizz"
        } else if i%5 == 0 {
            std.io.printf @ "Buzz"
        } else {
            std.io.printf@(n.to_string)
        }
    }
}

func main _ : List[String] -> Unit {
    fizzbuzz@12
}
        "#;
    // let mut parser = parse::Parser::new("func hoge _ : int -> int { let x = 1; for i in x .. 10 { i }  }");
    let mut parser = parse::Parser::new(code);
    println!("{:?}", parser.program());
}

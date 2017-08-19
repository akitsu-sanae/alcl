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

extern crate kazuma;

peg_file! parse("grammar.rustpeg");

fn main() {
    use std::io::Read;
    let filename = std::env::args().nth(1).expect("[filename] is required");
    let mut f = std::fs::File::open(filename).expect("not found: inputed filename");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("can not read input file");
    let module = parse::program(input.as_str()).unwrap();
    let mut builder = kazuma::builder::Builder::new("test");
    match builder.build(&module) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("error: {}", err),
    }
}



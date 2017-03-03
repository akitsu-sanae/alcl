/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/alcl
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use expr::Expr;
use expr::Info;
use function::Function;
use type_::Type;
use program::Program;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Plus,
    Minus,
    Ast,
    Slash,
    Percent,
    Equal,
    Colon,
    Semicolon,
    At,
    Dot,
    Eof,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,


    Arrow,
    Ellipsis,
    EqualEqual,
    NotEqual,

    Func,
    Let,
    Type,
    If,
    Else,
    For,
    In,

    Number(i64),
    String(String),
    Identifier(String)
}

impl Token {
    pub fn is_number(&self) -> bool {
        match *self {
            Token::Number(_) => true,
            _ => false
        }
    }
    pub fn is_identifier(&self) -> bool {
        match *self {
            Token::Identifier(_) => true,
            _ => false
        }
    }
    pub fn is_string(&self) -> bool {
        match *self {
            Token::String(_) => true,
            _ => false
        }
    }

    pub fn as_number(&self) -> Result<i64, String> {
        match *self {
            Token::Number(ref n) => Ok(n.clone()),
            _ => Err("non number token".to_string())
        }
    }
    pub fn as_identifier(&self) -> Result<String, String> {
        match *self {
            Token::Identifier(ref id) => Ok(id.clone()),
            _ => Err("non Identifier token".to_string())
        }
    }
    pub fn as_string(&self) -> Result<String, String> {
        match *self {
            Token::String(ref str) => Ok(str.clone()),
            _ => Err("non String token".to_string())
        }

    }
}

type LexResult = Result<Token, String>;

#[derive(Debug)]
struct Scanner<'a> {
    buff: &'a str
}

fn get_number(input: &str) -> Option<(i64, usize)> {
    let mut length = 0;
    for c in input.chars() {
        if !c.is_digit(10) {
            break;
        }
        length += 1;
    }
    if length != 0 {
        Some((input[0..length].parse().unwrap(), length))
    } else {
        None
    }
}

fn get_identifier(input: &str) -> Option<(String, usize)> {
    let mut length = 0;
    for c in input.chars() {
        if !c.is_alphabetic() && !c.is_digit(10) && c != '_' {
            break;
        }
        length += 1;
    }
    if length != 0 {
        Some((input[0..length].to_string(), length))
    } else {
        None
    }
}

fn get_string(input: &str) -> Option<(String, usize)> {
    if input.chars().nth(0).unwrap() != '"' {
        return None;
    }
    let mut length = 2;
    for c in input[1..].chars() {
        if c == '"' {
            break;
        }
        length += 1;
    }
    Some((input[1..length-1].to_string(), length))
}

fn char_to_token() -> HashMap<char, Token> {
    let mut result = HashMap::new();
    result.insert('+', Token::Plus);
    result.insert('-', Token::Minus);
    result.insert('*', Token::Ast);
    result.insert('/', Token::Slash);
    result.insert('%', Token::Percent);
    result.insert('=', Token::Equal);
    result.insert(':', Token::Colon);
    result.insert(';', Token::Semicolon);
    result.insert('@', Token::At);
    result.insert('.', Token::Dot);
    result.insert('(', Token::LeftParen);
    result.insert(')', Token::RightParen);
    result.insert('{', Token::LeftBrace);
    result.insert('}', Token::RightBrace);
    result.insert('[', Token::LeftBracket);
    result.insert(']', Token::RightBracket);
    result
}

fn str_to_token() -> HashMap<String, Token> {
    let mut result = HashMap::new();
    result.insert("func".to_string(), Token::Func);
    result.insert("type".to_string(), Token::Type);
    result.insert("let".to_string(), Token::Let);
    result.insert("if".to_string(), Token::If);
    result.insert("else".to_string(), Token::Else);
    result.insert("for".to_string(), Token::For);
    result.insert("in".to_string(), Token::In);
    result
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            buff: input
        }
    }

    pub fn next(&mut self) -> LexResult {
        self.buff = self.buff.trim();

        if self.buff.is_empty() {
            return Ok(Token::Eof);
        }

        if self.buff.len() > 2 && &self.buff[0..2] == "->" {
            self.buff = &self.buff[2..].trim();
            return Ok(Token::Arrow);
        }

        if self.buff.len() > 2 && &self.buff[0..2] == ".." {
            self.buff = &self.buff[2..].trim();
            return Ok(Token::Ellipsis);
        }

        if self.buff.len() > 2 && &self.buff[0..2] == "==" {
            self.buff = &self.buff[2..].trim();
            return Ok(Token::EqualEqual);
        }

        if self.buff.len() > 2 && &self.buff[0..2] == "!=" {
            self.buff = &self.buff[2..].trim();
            return Ok(Token::NotEqual);
        }

        let c = self.buff.chars().nth(0).unwrap();
        let char_to_token = char_to_token();
        let str_to_token = str_to_token();
        if char_to_token.keys().find(|&&x| x == c).is_some() {
            self.buff = &self.buff[1 ..].trim();
            Ok(char_to_token.get(&c).unwrap().clone())
        } else if let Some((n, length)) = get_number(self.buff) {
            self.buff = &self.buff[length ..].trim();
            Ok(Token::Number(n))
        } else if let Some((str, length)) = get_string(self.buff) {
            self.buff = &self.buff[length ..].trim();
            Ok(Token::String(str))
        } else if let Some((id, length)) = get_identifier(self.buff) {
            self.buff = &self.buff[length ..].trim();
            if let Some(token) = str_to_token.get(&id) {
                Ok(token.clone())
            } else {
                Ok(Token::Identifier(id))
            }
        } else {
            Err(format!("unknown token: {}", self.buff))
        }
    }

    pub fn expect(&mut self, token: Token) {
        let got = self.next().unwrap();
        if got != token {
            panic!("parsing error: {:?} was expected, but {:?} comming", got, token);
        }

    }

    pub fn peek(&self) -> LexResult {
        let buff = self.buff.trim();

        if buff.is_empty() {
            return Ok(Token::Eof);
        }

        if buff.len() > 2 && &buff[0..2] == "->" {
            return Ok(Token::Arrow);
        }

        if buff.len() > 2 && &self.buff[0..2] == ".." {
            return Ok(Token::Ellipsis);
        }

        if buff.len() > 2 && &self.buff[0..2] == "==" {
            return Ok(Token::EqualEqual);
        }

        if buff.len() > 2 && &self.buff[0..2] == "!=" {
            return Ok(Token::NotEqual);
        }

        let c = buff.chars().nth(0).unwrap();
        let char_to_token = char_to_token();
        let str_to_token = str_to_token();
        if char_to_token.keys().find(|&&x| x == c).is_some() {
            Ok(char_to_token.get(&c).unwrap().clone())
        } else if let Some((n, _)) = get_number(buff) {
            Ok(Token::Number(n))
        } else if let Some((str, _)) = get_string(self.buff) {
            Ok(Token::String(str))
        } else if let Some((id, _)) = get_identifier(buff) {
            if let Some(token) = str_to_token.get(&id) {
                Ok(token.clone())
            } else {
                Ok(Token::Identifier(id))
            }
        } else {
            Err(format!("unknown token: {}", buff))
        }
    }
}

pub struct Parser<'a> {
    scanner: Scanner<'a>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            scanner: Scanner::new(input)
        }
    }

    pub fn program(&mut self) -> Program {
        let mut result = Program::new();
        loop {
            let token = self.scanner.peek().unwrap();
            match token {
                Token::Func => result.functions.push(self.function()),
                Token::Eof => break,
                _ => panic!("unexpected token: {:?}", self.scanner),
            }

        }
        result
    }

    pub fn type_(&mut self) -> Type {
        let mut arg_types = vec![];
        arg_types.push(self.primitive_type());
        while self.scanner.peek().unwrap() == Token::Arrow {
            self.scanner.expect(Token::Arrow);
            arg_types.push(self.primitive_type());
        }

        if arg_types.len() == 1 {
            arg_types.pop().unwrap()
        } else {
            let ret_type = arg_types.pop().unwrap();
            Type::Function(box ret_type, arg_types)
        }
    }

    pub fn primitive_type(&mut self) -> Type {
        let name = self.scanner.next().unwrap().as_identifier().unwrap();

        if self.scanner.peek().unwrap() == Token::LeftBracket {
            self.scanner.expect(Token::LeftBracket);
            let t = self.type_();
            self.scanner.expect(Token::RightBracket);
            Type::Generic(name, box t)
        } else {
            Type::Primitive(name)
        }
    }

    // func hoge a: type b: type {
    //   expression
    // }
    pub fn function(&mut self)  -> Function {
        self.scanner.expect(Token::Func);
        let name = self.scanner.next().unwrap().as_identifier().unwrap();
        let mut args = vec![];
        loop {
            if self.scanner.peek().unwrap().is_identifier() {
                let name = self.scanner.next().unwrap().as_identifier().unwrap();
                self.scanner.expect(Token::Colon);
                let ty = self.type_();
                args.push((name, ty));
            } else {
                break;
            }
        }
        self.scanner.expect(Token::Colon);
        let ret_type = self.type_();
        self.scanner.expect(Token::LeftBrace);
        let expr = self.expression();
        self.scanner.expect(Token::RightBrace);
        Function {
            name: name,
            args: args,
            return_type: ret_type,
            body: expr
        }
    }

    pub fn expression(&mut self) -> Expr {
        self.let_expr()
    }

    // let id = init; body
    // or expr
    fn let_expr(&mut self) -> Expr {
        if self.scanner.peek().unwrap() == Token::Let {
            self.scanner.expect(Token::Let);
            let name = self.scanner.next().unwrap().as_identifier().unwrap();
            self.scanner.expect(Token::Equal);
            let init = self.if_expr();
            self.scanner.expect(Token::Semicolon);
            let body = self.expression();
            Expr::Let(name, box init, box body, Info::new())
        } else {
            self.sequence_expr()
        }
    }

    fn sequence_expr(&mut self) -> Expr {
        let expr = self.if_expr();
        if self.scanner.peek().unwrap() == Token::Semicolon {
            self.scanner.expect(Token::Semicolon);
            Expr::Sequence(box expr, box self.sequence_expr(), Info::new())
        } else {
            expr
        }
    }

    // if cond { true branch } else if cond { ... } else { false branch }
    // equal_expr
    fn if_expr(&mut self) ->  Expr {
        if self.scanner.peek().unwrap() == Token::If {
            self.scanner.expect(Token::If);
            let cond = self.equal_expr();
            self.scanner.expect(Token::LeftBrace);
            let tr = self.expression();
            self.scanner.expect(Token::RightBrace);
            self.scanner.expect(Token::Else);
            let mut else_if = vec![];
            loop {
                if self.scanner.peek().unwrap() == Token::If {
                    self.scanner.expect(Token::If);
                    let cond = self.equal_expr();
                    self.scanner.expect(Token::LeftBrace);
                    let body = self.expression();
                    self.scanner.expect(Token::RightBrace);
                    self.scanner.expect(Token::Else);
                    else_if.push((cond, body));
                } else {
                    break;
                }
            }
            self.scanner.expect(Token::LeftBrace);
            let fl = self.expression();
            self.scanner.expect(Token::RightBrace);
            Expr::If(box cond, box tr, else_if, box fl, Info::new())
        } else {
            self.for_expr()
        }
    }

    // for i in 0..15 { expr }
    fn for_expr(&mut self) -> Expr {
        if self.scanner.peek().unwrap() == Token::For {
            self.scanner.expect(Token::For);
            let name = self.scanner.next().unwrap().as_identifier().unwrap();
            self.scanner.expect(Token::In);
            let start = self.if_expr();
            self.scanner.expect(Token::Ellipsis);
            let last = self.if_expr();
            self.scanner.expect(Token::LeftBrace);
            let body = self.expression();
            self.scanner.expect(Token::RightBrace);
            Expr::For(name, box start, box last, box body, Info::new())
        } else {
            self.equal_expr()
        }
    }

    fn equal_expr(&mut self) -> Expr {
        let mut acc = self.add_expr();
        loop {
            let token = self.scanner.peek().unwrap();
            match token {
                Token::EqualEqual => {
                    self.scanner.expect(Token::EqualEqual);
                    acc = Expr::Equal(box acc, box self.add_expr(), Info::new())
                },
                Token::NotEqual => {
                    self.scanner.expect(Token::NotEqual);
                    acc = Expr::NotEqual(box acc, box self.add_expr(), Info::new())
                },
                _ => break
            }
        }
        acc
    }

    fn add_expr(&mut self) -> Expr {
        let mut acc = self.mult_expr();
        loop {
            let token = self.scanner.peek().unwrap();
            match token {
                Token::Plus => {
                    self.scanner.expect(Token::Plus);
                    acc = Expr::Add(box acc, box self.mult_expr(), Info::new())
                },
                Token::Minus => {
                    self.scanner.expect(Token::Minus);
                    acc = Expr::Sub(box acc, box self.mult_expr(), Info::new())
                },
                _ => break
            }
        }
        acc
    }

    fn mult_expr(&mut self) -> Expr {
        let mut acc = self.apply_expr();
        loop {
            let token = self.scanner.peek().unwrap();
            match token {
                Token::Ast => {
                    self.scanner.expect(Token::Ast);
                    acc = Expr::Mult(box acc, box self.apply_expr(), Info::new())
                },
                Token::Slash => {
                    self.scanner.expect(Token::Slash);
                    acc = Expr::Div(box acc, box self.apply_expr(), Info::new())
                },
                Token::Percent => {
                    self.scanner.expect(Token::Percent);
                    acc = Expr::Surplus(box acc, box self.apply_expr(), Info::new())
                }
                _ => break
            }
        }
        acc
    }

    fn apply_expr(&mut self) -> Expr {
        let mut acc = self.dot_expr();
        loop {
            if self.scanner.peek().unwrap() == Token::At {
                self.scanner.expect(Token::At);
                acc = Expr::Apply(box acc, box self.dot_expr(), Info::new())
            } else {
                break;
            }
        }
        acc
    }

    fn dot_expr(&mut self) -> Expr {
        let mut acc = self.primary_expr();
        loop {
            if self.scanner.peek().unwrap() == Token::Dot {
                self.scanner.expect(Token::Dot);
                acc = Expr::Dot(box acc, box self.primary_expr(), Info::new())
            } else {
                break;
            }
        }
        acc
    }

    fn primary_expr(&mut self) -> Expr {
        let token = self.scanner.next().unwrap();
        if token.is_number() {
            Expr::Number(token.as_number().unwrap(), Info::new())
        } else if token.is_identifier() {
            Expr::Identifier(token.as_identifier().unwrap(), Info::new())
        } else if token.is_string() {
            Expr::String(token.as_string().unwrap(), Info::new())
        } else if token == Token::LeftParen {
            let expr = self.expression();
            self.scanner.expect(Token::RightParen);
            expr
        } else {
            panic!("parsing error: <number>, <Identifier> was expected, but {:?} comming", token)
        }
    }
}


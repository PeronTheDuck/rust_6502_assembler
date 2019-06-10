use std::io::{Read,BufRead,BufReader};
use super::token_type::TokenType;
use super::types;
use super::types::*;

type TreeError = std::result::Result<(), &'static str>;

#[derive(Serialize)]
pub struct Tree {
    tokens: Vec<TokenType>,
}
impl Tree {
    pub fn new() -> Self {
        Tree { tokens: vec![] }
    }
    pub fn insert(&mut self, mut token: TokenType) -> TreeError {
        if token.is_value() {
            match self.tokens.pop().unwrap() {
                TokenType::OPCODE(mut opcode) => {
                    opcode.parameter = token.get_value().clone();
                    self.insert(TokenType::OPCODE(opcode))
                        .expect("Error appending token on tree");
                    self.tokens.push(token);
                }
                _ => {
                    return Err("Syntax error");
                }
            }
        } else {
            self.tokens.push(token);
        }
        return Ok(());
    }
}
impl Tree {
    pub fn parse<R>(from: std::io::BufReader<R>) -> Tree
    where
        R: std::io::Read,
    {
        let mut tree = Tree::new();
        for line in from.lines().map(|l| l.expect("Couldn't read line")) {
            for item in line.split_whitespace() {
                tree.insert(TokenType::new(item).expect(&format!("Invalid item {}", item)))
                    .expect("Error building tree");
            }
        }
        tree
    }
}
impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut buf = "".to_string();
        for (i, tkn) in self.tokens.iter().enumerate() {
            buf = format!("{}\n{}:\t{}", buf, i, tkn);
        }
        write!(f, "{}", buf)
    }
}
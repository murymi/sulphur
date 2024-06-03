use std::{cell::RefCell, rc::Rc, str::FromStr};

use parser::{ParseError, TokenizeError, Tokenizer};

use crate::*;

#[derive(Debug)]
pub struct Dom {
    tree: Rc<RefCell<Node>>
}

#[derive(Debug)]
pub enum DomError {
    TokenizeError(TokenizeError),
    ParseError(ParseError),
    BlockedAppend(String)
}

impl FromStr for Dom {
    type Err = DomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokenizer = Tokenizer::new(s.to_string());
         match tokenizer.tokenize() {
            Ok(_) => (),
            Err(e) => return Err(DomError::TokenizeError(e)),
        };
        let mut parser = Parser::new(tokenizer.tokens());
        let tree = match parser.parse() {
            Ok(t) => t,
            Err(e) => return Err(DomError::ParseError(e)),
        };
        Ok(Self {tree})
    }
}

impl Dom {
    pub fn root(&self) -> Option<Rc<RefCell<Node>>> {
        Some(self.tree.clone())
    }
}
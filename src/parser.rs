#[allow(unused, dead_code)]

pub use std::{cell::RefCell, collections::HashMap, mem::discriminant, rc::Rc};

use crate::{Node, NodeType};

pub(crate) struct Parser<'a> {
    current:usize,
    tokens: &'a Vec<Token>,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    #[allow(unused)]
    UnexpectedToken(TokenType, Position),
    #[allow(unused)]
    UnclosedTag(String, Position),
    UnexpectedEndOfFile
}

impl<'a> Parser<'a> {
    pub fn new(tokens:&'a Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
           return Some(&self.tokens[self.current])
        }
        None
    }

    fn check(&self, token_type:TokenType) -> bool {
        if let Some(t) = self.tokens.get(self.current) {
            discriminant(&t.token_type) == discriminant(&token_type)
        } else {
            false
        }
    }

    fn match_tokens(&mut self, tokens: &[TokenType]) -> bool {
        if let Some(current_token) = self.tokens.get(self.current) {
            for tok in tokens.iter() {
                if discriminant(tok) == discriminant(&current_token.token_type) {
                    self.advance();
                    return true;
                }
            }
        }
        return  false;
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current-1)
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current-1).unwrap()
    }

    fn back(&mut self) {
        self.current -= 1;
    }

    fn expect(&mut self, token_type: TokenType) -> Result<&Token, ParseError> {
        if self.check(token_type.clone()) {
            self.advance();
            Ok(self.previous())
        } else {
            if let Some(tok) =  self.peek() {
                Err(ParseError::UnexpectedToken(tok.token_type.clone(), tok.position.clone()))
            } else {
                Err(ParseError::UnexpectedEndOfFile)
            }
        }
    }

    fn attributes(&mut self) -> Result<(bool, HashMap<String, String>),ParseError> {
        let mut attrs = HashMap::new();
        loop {
            if self.check(TokenType::RightAngle) {
                return Ok((false, attrs));
            }
            if self.check(TokenType::Stroke) {
                self.advance();
                return  Ok((true,attrs));
            }
            let key = self.expect(TokenType::Identifier)?;
            let key = key.content.clone();
            self.expect(TokenType::Equals)?;
            let value = if self.match_tokens(&[TokenType::Identifier, TokenType::Literal]) {
                self.previous().content.clone()
            } else {
                return Err(ParseError::UnexpectedToken(
                    self.peek().unwrap().token_type.clone(),
                    self.peek().unwrap().position.clone()
                ))
            };

            attrs.insert(key, value);
        }
    }

    fn tag(&mut self) -> Result<Rc<RefCell<Node>>,ParseError> {
        self.expect(TokenType::LeftAngle)?;     // <  
        let tag = self.expect(TokenType::Identifier)?; 
        let _tag_type = tag.token_type.clone();
        let tag_pos = tag.position.clone();
        let tag_name = tag.content.clone();   // h1  
        let (closed, attributes) = self.attributes()?;                         // a="a" b= "b" | /
        self.expect(TokenType::RightAngle)?;   // > 
        let element = Rc::new(RefCell::new(Node::new(tag_name.clone())));
        element.borrow_mut().attributes = attributes;

        if closed {
            (*element).borrow_mut().node_type = NodeType::Element;
            return Ok(element);
        }
        
        if self.match_tokens(&[TokenType::Identifier]) {
            (*element).borrow_mut().node_type = NodeType::Text(self.previous().content.clone())
        } else if self.match_tokens(&[TokenType::LeftAngle]) {
            (*element).borrow_mut().node_type = NodeType::Element;
            if self.check(TokenType::Stroke) {
                self.back();
            } else {
                self.back();
                loop {
                    let new_element = match self.tag() {
                        Ok(v) => v,
                        Err(e) => {
                            if discriminant(&e) == discriminant(&ParseError::UnexpectedEndOfFile) {
                                return Err(ParseError::UnclosedTag(tag_name, tag_pos));
                            } else {
                                return Err(e);
                            }
                        }
                    };
                    (*new_element).borrow_mut().parent = Some(Rc::downgrade(&element));
                    element.borrow_mut().children.push(
                        new_element
                    );

                    if self.match_tokens(&[TokenType::LeftAngle]) {
                        if self.check(TokenType::Stroke) {
                            self.back();
                            break;
                        }
                        self.back();
                    }
                }
            }
        }
        

        if let Err(ParseError::UnexpectedEndOfFile) = self.expect(TokenType::LeftAngle) {
            return Err(ParseError::UnclosedTag(tag_name, tag_pos));
        };       // <
        self.expect(TokenType::Stroke)?;             // /
        let close_tag = self.expect(TokenType::Identifier)?;
        let close_tag_name = close_tag.content.clone();      // h1
        if close_tag_name != tag_name {
            return Err(ParseError::UnclosedTag(tag_name, tag_pos));
        }
        self.expect(TokenType::RightAngle)?;     // >

        Ok(element)

    }

    pub fn parse(&mut self) -> Result<Rc<RefCell<Node>>, ParseError> {
        if self.match_tokens(&[TokenType::LeftAngle]) {
            if self.check(TokenType::Bang) {
                while !self.check(TokenType::RightAngle) {
                    self.advance();
                }
                self.advance();
            } else {
                self.back();
            }
        }

        self.tag()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Position {
    line: usize,
    col: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum TokenType {
    LeftAngle = 0,
    RightAngle,
    Stroke,
    Equals,
    Identifier,
    Literal,
    Bang,
    Dash
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    position: Position,
    content: String
}

impl Token {
    fn new(token_type: TokenType, position:Position, content:String) -> Self {
        Self {
            token_type,
            position,
            content
        }
    }
}


#[derive(Debug)]
pub(crate) enum TokenizeError {
    UnclosedLiteral,
    UnexpectedEndOfLine,
    #[allow(unused)]
    UnexpectedChar(char, Position),
    UnexpectedEndOfFile
}

pub(crate) struct Tokenizer {
    html: String,
    current_line: usize,
    current_column: usize,
    index: usize,
    in_tag: bool,
    tokens: Vec<Token>
}

impl Tokenizer {
    pub fn new(html: String) -> Self {
        Self {
            current_column: 0,
            current_line: 0,
            html,
            index: 0,
            in_tag: false,
            tokens: vec![]
        }
    }

    pub(crate) fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn advance(&mut self, step:usize) -> Result<(), TokenizeError> {
        if self.more_tokens() {
            self.index += step;
            self.current_column += step;
            Ok(())
        } else {
            Err(TokenizeError::UnexpectedEndOfFile)
        }
    } 

    fn  next_line(&mut self) {
        self.current_column = 0;
        self.current_line += 1;
    }

    fn identifier(&self) -> String {
        let slice = &self.html[self.index..];
        let mut end = 0;
        for (_, c) in slice.chars().enumerate() {
            if !(c.is_alphanumeric() || c == '_' || (!self.in_tag && c == ' ')) {
                break;
            }
            end += 1;
        }
        (&slice[..end]).into()
    }

    fn literal(&mut self, quote_char: char) -> Result<String, TokenizeError> {
        let slice = &self.html[self.index+1..];
        if slice.len() == 0 {
            return Err(TokenizeError::UnclosedLiteral);
        }
        let char_iter = slice.chars().enumerate();
        let mut end = 0;
        let mut check_escape = false;

        for (i, c) in char_iter {
            if check_escape && c == 'n' {
                return Err(TokenizeError::UnexpectedEndOfLine);
            }
            check_escape = false;

            if c == '\n' {
                break;
            }

            if c == '\\' {
                check_escape = true;
            }

            if c == quote_char {
                end = i;
                break;
            }
        }
        let result = (&slice[0..end]).to_string();
        self.advance(2)?;

        match end {
            0 => {
                Err(TokenizeError::UnclosedLiteral)
            },
            _ => Ok(result)
        }
    }

    fn push_token(&mut self, token_type:TokenType, content:String, position:Position) -> Result<(), TokenizeError>{
        self.advance(content.len())?;
        Ok(self.tokens.push(Token::new(token_type, position, content)))
    }

    fn more_tokens(&self) -> bool {
        self.html.len() > self.index
    }

    fn skip_comment(&mut self) -> Result<(), TokenizeError> {
        let mut chars = &self.html[self.index..];
        while !chars.starts_with("-->") && self.more_tokens() {
            self.index += 1;
            chars = &self.html[self.index..];
        }

        return if self.more_tokens() {
            self.advance(3)
        } else {
            Err(TokenizeError::UnexpectedEndOfFile)
        }

    }

    pub(crate) fn tokenize(&mut self) -> Result<(), TokenizeError> {
        while let Some(c) = self.html.chars().nth(self.index) {
            let mut position = Position { col:self.current_line, line:self.current_line };
            position.col = self.current_column;
            position.line = self.current_line;
            
            match c {
                '<' => {
                    if (&self.html[self.index..]).starts_with("<!--") {
                        self.advance(4)?;
                        self.skip_comment()?;
                    } else {
                        self.in_tag = true;
                        self.push_token(TokenType::LeftAngle, c.into(), position)?;
                    }
                }
                '>' => {
                    self.in_tag = false;
                    self.push_token(TokenType::RightAngle, c.into(), position)?;
                }
                '/' => {
                    self.push_token(TokenType::Stroke, c.into(), position)?;

                }
                '=' => {
                    self.push_token(TokenType::Equals, c.into(), position)?;
                },
                '!' => {
                    self.push_token(TokenType::Bang, c.into(), position)?;
                },
                '-' => {
                    self.push_token(TokenType::Dash, c.into(), position)?;
                },
                ' ' => {
                    self.advance(1)?;
                    continue;
                }
                '\n' => {
                    self.advance(1)?;
                    self.next_line();
                    continue;
                }
                _ => {
                    if c.is_alphanumeric() {
                        let id = self.identifier();
                        self.push_token(TokenType::Identifier, id, position)?;
                    } else if c == '"' || c == '\'' {
                        let id = self.literal(c)?;
                        self.push_token(TokenType::Literal, id, position)?;
                    } else {
                        return Err(TokenizeError::UnexpectedChar(c, position));
                    }
                }
            };
        }

        Ok(())
    }
}


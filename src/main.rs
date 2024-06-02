use std::{iter, mem::discriminant};

#[derive(Debug, Clone)]
struct Position {
    line: usize,
    col: usize,
}

#[derive(Debug, Clone)]
enum TokenType {
    LeftAngle = 0,
    RightAngle,
    Stroke,
    Equals,
    Identifier,
    Literal,
    Eof
}


#[derive(Debug)]
struct Token {
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
enum TokenizeError {
    UnclosedLiteral,
    UnexpectedEndOfLine,
    UnexpectedChar(char),
}

struct Tokenizer {
    html: String,
    current_line: usize,
    current_column: usize,
    index: usize,
    in_tag: bool,
    tokens: Vec<Token>
}

impl Tokenizer {
    fn new(html: String) -> Self {
        Self {
            current_column: 0,
            current_line: 0,
            html,
            index: 0,
            in_tag: false,
            tokens: vec![]
        }
    }

    fn advance(&mut self, step:usize) {
        self.index += step;
        self.current_column += step;
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

    fn literal(&mut self) -> Result<String, TokenizeError> {
        let slice = &self.html[self.index+1..];
        println!("slice is {slice} {s}", s = self.index);
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

            if c == '"' {
                println!("========{}======", &slice[0..i]);
                end = i;
                break;
            }
        }
        let result = (&slice[0..end]).to_string();
        self.advance(2);

        match end {
            0 => {
                println!("slicez result is {result}");
                Err(TokenizeError::UnclosedLiteral)
            },
            _ => Ok(result)
        }
    }

    fn push_token(&mut self, token_type:TokenType, content:String, position:Position) {
        self.advance(content.len());
        self.tokens.push(Token::new(token_type, position, content))
    }

    fn tokenize(&mut self) -> Result<(), TokenizeError> {
        while let Some(c) = self.html.chars().nth(self.index) {
            let mut position = Position { col:self.current_line, line:self.current_line };
            position.col = self.current_column;
            position.line = self.current_line;
            
            match c {
                '<' => {
                    self.in_tag = true;
                    self.push_token(TokenType::LeftAngle, c.into(), position);
                }
                '>' => {
                    self.in_tag = false;
                    self.push_token(TokenType::RightAngle, c.into(), position);
                }
                '/' => {
                    self.push_token(TokenType::Stroke, c.into(), position);

                }
                '=' => {
                    self.push_token(TokenType::Equals, c.into(), position);
                }
                ' ' => {
                    self.advance(1);
                    continue;
                }
                '\n' => {
                    self.advance(1);
                    self.next_line();
                    continue;
                }
                _ => {
                    if c.is_alphanumeric() {
                        let id = self.identifier();
                        self.push_token(TokenType::Identifier, id, position);
                    } else if c == '"' {
                        let id = self.literal()?;
                        self.push_token(TokenType::Literal, id, position)
                    } else {
                        return Err(TokenizeError::UnexpectedChar(c));
                    }
                }
            };
        }

        Ok(())
    }
}

struct Parser<'a> {
    current:usize,
    tokens: &'a Vec<Token>,
}

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(TokenType, Position, String),
    Eof(String)
}

impl<'a> Parser<'a> {
    fn new(tokens:&'a Vec<Token>) -> Self {
        Self {
            current:0,
            tokens
        }
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

    fn expect(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.check(token_type.clone()) {
            self.advance();
            Ok(())
        } else {
            if let Some(tok) =  self.peek() {
                Err(ParseError::UnexpectedToken(
                    tok.token_type.clone(),
                    tok.position.clone(),
                    format!("{}:{}: Expected {:?} found {:?}",tok.position.line, tok.position.col, token_type, tok.token_type)
                ))
            } else {
                Err(ParseError::Eof(
                    format!("Unexpected end of file")
                ))
            }
        }
    }

    fn attributes(&mut self) -> bool {
        loop {
            if self.check(TokenType::RightAngle) {
                return false;
            }
            if self.check(TokenType::Stroke) {
                self.advance();
                return  true;
            }

            self.expect(TokenType::Identifier);
            self.expect(TokenType::Equals);
            self.match_tokens(&[TokenType::Identifier, TokenType::Literal]);
        }
    }

    fn tag(&mut self) {
        self.expect(TokenType::LeftAngle).expect("open tag");     // <  
        self.expect(TokenType::Identifier).expect("tag name");    // h1  
        let closed = self.attributes();                         // a="a" b= "b" | /
        self.expect(TokenType::RightAngle).expect("close tag");   // > 

        if closed {
            return;
        }

        if self.match_tokens(&[TokenType::Identifier]) {

        }
        
        if self.match_tokens(&[TokenType::LeftAngle]) {
            if self.check(TokenType::Stroke) {
                self.back();
            } else {
                self.back();
                loop {
                    self.tag();

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

        self.expect(TokenType::LeftAngle).expect("open tag");       // <
        self.expect(TokenType::Stroke).expect("slash");             // /
        self.expect(TokenType::Identifier).expect("tag name");      // h1
        self.expect(TokenType::RightAngle).expect("close tag");     // >

    }

    fn parse(&mut self) -> Result<(), ParseError> {
        //self.expect(TokenType::LeftAngle).unwrap();
        Ok(self.tag())
    }
}
fn main() {
    let html = r#"
    <world one="two" three="four">
        <h1 five=six>
        <h1>
        <h1>
        <p>hello world ghasia</p>
        </h1>
        <h1></h1>
        <h1></h1>
        </h1>
        </h1>
    </cow>"#;

    println!("{html}");

    let mut t = Tokenizer::new(html.into());
    let tokens = t.tokenize().expect("tokenize is ok");
    println!("{:#?}", t.tokens);
    let mut parser = Parser::new(&t.tokens);
    parser.parse().expect("parse should not suc");
}

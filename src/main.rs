#[derive(Debug)]
enum Token {
    LeftAngle,
    RightAngle,
    DoubleQuote,
    Stroke,
    Equals,
    Identifier(String),
    Literal(String)
}

#[derive(Debug)]
enum TokenizeError {
    UnclosedLiteral,
    UnexpectedChar(char)
}

fn identifier(slice: &str) -> String {
    let mut end = 0;
    for (i, c) in slice.chars().enumerate() {
        if !(c.is_alphanumeric() || c == '_') {
            end = i;
            break;
        }
    }
    (&slice[..end]).into()
}

fn literal(slice: &str) -> Result<String, TokenizeError> {
    if slice.len() == 0 {
        return Err(TokenizeError::UnclosedLiteral);
    }
    let mut end = 0;
    for (i, c) in slice.chars().enumerate() {
        if c== '\n' {
            break;
        }

        if c == '"'{
            end = i;
            break;
        }
    }
    
    match end {
        0 => Err(TokenizeError::UnclosedLiteral),
        _ => Ok((&slice[0..end]).into())
    }
}

fn tokenize(html: String) -> Result<Vec<Token>,TokenizeError> {
    let mut idx = 0;
    let mut tokens = vec![];
    while let Some(c) = html.chars().nth(idx) {  
        let tok = match c {
            '<' => Token::LeftAngle,
            '>' => Token::RightAngle,
            '/' => Token::Stroke,
            '=' => Token::Equals,
            ' ' => {
                idx += 1;
                continue
            },
            _ => {
                if c.is_alphanumeric() {
                    let id = identifier(&html[idx..]);
                    idx += id.len() - 1;
                    Token::Identifier(id)
                } else if c == '"' {
                    let id = literal(&html[idx+1..])?;
                    idx+=id.len()+2;
                    Token::Literal(id)
                } else {
                    return Err(TokenizeError::UnexpectedChar(c))
                }
            }
        };

        tokens.push(tok);
        idx += 1;
    }

    Ok(tokens)
}

fn main() {
    let html = r#"<html cow="domestic"></html>"#;

    let tokens = tokenize(html.into());
    println!("{:#?}", tokens.unwrap());
}

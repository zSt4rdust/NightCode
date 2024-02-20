use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;

#[derive(strum_macros::Display, PartialEq, Clone, Eq, Hash)]
pub enum TokenKind {
    LiteralInteger,
    LiteralFloat,

    PunOperatorPlus,
    PunOperatorMinus,
    PunOperatorMultiply,
    PunOperatorDivide,

    PunParenOpen,
    PunParenClose,

    Semicolon,

    EOF,
    Undefined,
    UndefinedPunctuation,
}

pub struct Location {
    pub line: u32,
    pub ch: u32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(at line: {}, ch: {})", self.line, self.ch)
    }
}

impl Location {
    fn new(line: u32, ch: u32) -> Location {
        Location { line, ch }
    }
}

pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: Location,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "token {} | value: {} | kind: {}",
            self.location, self.value, self.kind
        )
    }
}

impl Token {
    fn new(kind: TokenKind, value: &str, line: u32, ch: u32) -> Token {
        Token {
            kind,
            value: value.to_string(),
            location: Location::new(line, ch),
        }
    }

    fn empty(line: u32, ch: u32) -> Token {
        Token {
            kind: TokenKind::Undefined,
            value: String::new(),
            location: Location::new(line, ch),
        }
    }
}

lazy_static! {
    static ref PUNCTUATION_TOKENS: HashMap<&'static str, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("+", TokenKind::PunOperatorPlus);
        map.insert("-", TokenKind::PunOperatorMinus);
        map.insert("*", TokenKind::PunOperatorMultiply);
        map.insert("/", TokenKind::PunOperatorDivide);
        map.insert("(", TokenKind::PunParenOpen);
        map.insert(")", TokenKind::PunParenClose);
        map.insert(";", TokenKind::Semicolon);
        map
    };
}

pub fn tokenize(source: &String) -> Vec<Token> {
    let mut line: u32 = 1;
    let mut ch: u32 = 1;

    let mut output: Vec<Token> = Vec::new();
    let mut token = Token::empty(line, ch);

    let mut chars: Vec<char> = source.chars().collect();
    chars.push('\0');

    for (_i, c) in chars.into_iter().enumerate() {
        if c == '\n' {
            if token.value != "" {
                output.push(token)
            }

            line += 1;
            ch = 1;
            token = Token::empty(line, ch);
            continue;
        }

        if token.kind == TokenKind::UndefinedPunctuation && !is_punctuation(c) {
            let mut punctuation_idx_buffer = 0;

            loop {
                let mut punctuation_value_buffer = String::new();
                let mut valid_punctuation = String::new();

                for pi in punctuation_idx_buffer..token.value.len() {
                    let pc = token.value.chars().nth(pi).unwrap();
                    punctuation_value_buffer.push(pc);

                    if PUNCTUATION_TOKENS.contains_key(punctuation_value_buffer.as_str()) {
                        punctuation_idx_buffer = pi + 1;
                        valid_punctuation = punctuation_value_buffer.clone();
                    }
                }

                if valid_punctuation.is_empty() {
                    if punctuation_idx_buffer < token.value.len() {
                        output.push(token);
                    }
                    break;
                }

                let token_value = valid_punctuation.as_str();
                let token_kind = PUNCTUATION_TOKENS.get(token_value).unwrap().clone();

                output.push(Token::new(
                    token_kind,
                    token_value,
                    line,
                    ch - token.value.len() as u32 + (punctuation_idx_buffer - 1) as u32,
                ));
            }

            token = Token::empty(line, ch);
        }

        if c == '\0' {
            if token.value != "" {
                output.push(token);
            }
            token = Token::empty(line, ch);
            output.push(Token::new(TokenKind::EOF, "\0", line, ch));
        }

        if c.is_digit(10) && token.kind != TokenKind::LiteralFloat {
            token.kind = TokenKind::LiteralInteger;
        } else if c == '.' {
            if token.kind == TokenKind::LiteralInteger || token.value.is_empty() {
                token.kind = TokenKind::LiteralFloat
            }
        } else if is_punctuation(c) && token.kind != TokenKind::UndefinedPunctuation {
            if token.value != "" {
                output.push(token);
            }
            token = Token::empty(line, ch);

            token.kind = TokenKind::UndefinedPunctuation;
        }

        ch += 1;
        if c.is_whitespace() {
            if token.value != "" {
                output.push(token);
            }
            token = Token::empty(line, ch);

            continue;
        }

        token.value.push(c);
    }

    output
}

fn is_punctuation(ch: char) -> bool {
    ch != '.' && ch.is_ascii_punctuation()
}

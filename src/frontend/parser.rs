use crate::error::Error;
use std::collections::HashMap;

use super::ast::{Value, ValueNode};
use super::lexer::{Token, TokenKind};

use lazy_static::lazy_static;

lazy_static! {
    static ref OPERATOR_PRECEDENCES: HashMap<TokenKind, i32> = {
        let mut map = HashMap::new();
        map.insert(TokenKind::PunOperatorPlus, 0);
        map.insert(TokenKind::PunOperatorMinus, 0);
        map.insert(TokenKind::PunOperatorMultiply, 1);
        map.insert(TokenKind::PunOperatorDivide, 1);
        map
    };
}

pub fn parse(tokens: &Vec<Token>) -> Result<ValueNode, Error> {
    parse_expression(tokens, 0)
}

pub fn parse_expression(tokens: &Vec<Token>, start: usize) -> Result<ValueNode, Error> {
    let mut current_token = start;
    loop {
        let Some(token) = tokens.get(current_token) else {
            panic!(
                "idx {} is out of passed tokens range ({})",
                current_token,
                tokens.len()
            );
        };

        if token.kind == TokenKind::Undefined || token.kind == TokenKind::UndefinedPunctuation {
            return Err(Error::syntax("found undefined token", &token.location));
        }

        if token.kind == TokenKind::LiteralFloat || token.kind == TokenKind::LiteralInteger {
            return parse_literal(token);
        }

        current_token += 1;
    }
}

pub fn parse_literal(token: &Token) -> Result<ValueNode, Error> {
    match token.kind {
        TokenKind::LiteralInteger => {
            let parsed = token.value.parse::<i32>();
            match parsed {
                Ok(val) => Ok(ValueNode::Literal(Value::Integer(val))),
                Err(_) => Err(Error::parser("invalid integer literal", &token.location)),
            }
        }
        TokenKind::LiteralFloat => {
            let parsed = token.value.parse::<f32>();
            match parsed {
                Ok(val) => Ok(ValueNode::Literal(Value::SingleFloat(val))),
                Err(_) => Err(Error::parser("invalid float literal", &token.location)),
            }
        }
        _ => Err(Error::parser(
            "got non literal token while trying to parse literal",
            &token.location,
        )),
    }
}

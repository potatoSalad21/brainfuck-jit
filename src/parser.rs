use std::fmt;

use crate::lexer::Lexer;
use crate::ops::{Op, OpType};

#[derive(Debug)]
pub enum ParseError {
    UnbalancedBrackets,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnbalancedBrackets => write!(f, "unbalanced brackets"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse(source: String) -> Result<Vec<Op>, ParseError> {
    let mut lexer = Lexer::new(source);
    let mut ops: Vec<Op> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    let mut ch = lexer.next();
    while let Some(curr) = ch {
        let op_type = match OpType::from_char(curr) {
            Some(t) => t,
            None => {
                ch = lexer.next();
                continue;
            }
        };

        if op_type.is_repeatable() {
            let mut count: usize = 1;
            let mut next = lexer.next();

            while next == Some(curr) {
                count += 1;
                next = lexer.next();
            }

            ops.push(Op {
                op_type,
                operand: count,
            });
            ch = next;
        } else if op_type == OpType::JmpIfZero {
            let addr = ops.len();
            ops.push(Op {
                op_type,
                operand: 0,
            });
            stack.push(addr);
            ch = lexer.next();
        } else {
            let addr = stack.pop().ok_or(ParseError::UnbalancedBrackets)?;
            ops.push(Op {
                op_type,
                operand: addr + 1,
            });
            ops[addr].operand = ops.len();
            ch = lexer.next();
        }
    }

    if !stack.is_empty() {
        return Err(ParseError::UnbalancedBrackets);
    }

    Ok(ops)
}


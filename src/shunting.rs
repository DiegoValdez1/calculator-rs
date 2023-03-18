use std::{error::Error, fmt::Display, ops::Neg};

use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("^")]
    Exp,

    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[regex(r"[0-9]+\.?[0-9]+", |lex| lex.slice().parse())]
    Number(f32),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    IntNumber(i32),

    #[token("–")] // this is different from hyphen '-'
    Neg,

    #[error]
    Err,
}

impl Token {
    fn priority(&self) -> Option<i32> {
        match self {
            Token::Neg => Some(0),
            Token::Add | Token::Sub => Some(1),
            Token::Mul | Token::Div => Some(2),
            Token::Exp => Some(3),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ShuntError {
    InequalParenthesis,
    InternalError,
    OperatorMissingNumbers,
    DivideByZero,
    MissingNumber,
}

impl Display for ShuntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ShuntError {}

pub fn shunt(input: &str) -> Result<Vec<Token>, ShuntError> {
    let tokens = Token::lexer(input).filter(|t| *t != Token::Err);

    let mut opstack: Vec<Token> = Vec::new();
    let mut postfix: Vec<Token> = Vec::new();
    let mut parenthesis_count = (0, 0); // (open, close)

    for token in tokens {
        let priority_option = token.priority();

        if let Some(priority) = priority_option {
            loop {
                let last_option = opstack.last();

                if let Some(last) = last_option {
                    if last.priority().unwrap_or(0) >= priority {
                        postfix.push(opstack.pop().unwrap())
                    } else {
                        opstack.push(token);
                        break;
                    }
                } else {
                    opstack.push(token);
                    break;
                }
            }
        } else {
            match token {
                Token::Close => {
                    parenthesis_count.1 += 1;
                    while let Some(x) = opstack.pop() {
                        if x == Token::Open {
                            break;
                        }
                        postfix.push(x);
                    }
                }
                Token::Open => {
                    parenthesis_count.0 += 1;
                    opstack.push(token)
                }
                Token::Number(x) => postfix.push(Token::Number(x)),
                Token::IntNumber(x) => postfix.push(Token::IntNumber(x)),
                _ => (),
            }
        }
    }

    while let Some(x) = opstack.pop() {
        postfix.push(x)
    }

    if parenthesis_count.0 != parenthesis_count.1 {
        return Err(ShuntError::InequalParenthesis);
    }

    Ok(postfix)
}

pub fn solve_postfix(input: Vec<Token>) -> Result<f32, ShuntError> {
    let mut numstack: Vec<f32> = Vec::new();

    for token in input {
        match token {
            Token::Number(x) => numstack.push(x),
            Token::IntNumber(x) => numstack.push(x as f32),
            Token::Neg => {
                let num = numstack.pop().ok_or(ShuntError::OperatorMissingNumbers)?;
                numstack.push(num.neg())
            }
            _ => {
                let b = numstack.pop().ok_or(ShuntError::OperatorMissingNumbers)?;
                let a = numstack.pop().ok_or(ShuntError::OperatorMissingNumbers)?;
                match token {
                    Token::Add => numstack.push(a + b),
                    Token::Sub => numstack.push(a - b),
                    Token::Mul => numstack.push(a * b),
                    Token::Div => numstack.push({
                        // f32 doesn't implement checked_div so I must do this monstrosity
                        if b == 0.0 {
                            return Err(ShuntError::DivideByZero);
                        }
                        a / b
                    }),
                    Token::Exp => numstack.push(a.powf(b)),
                    _ => return Err(ShuntError::InternalError),
                }
            }
        }
    }

    Ok(numstack.get(0).ok_or(ShuntError::MissingNumber)?.to_owned())
}

pub fn solve(input: &str) -> Result<f32, ShuntError> {
    solve_postfix(shunt(input)?)
}

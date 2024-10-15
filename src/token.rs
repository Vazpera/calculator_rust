use core::f64;
use std::{f64::consts, fmt::Display, u8};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Max,
    Min,
    Sin,
    Cos,
    Tan,
    Sqrt,
    LParen,
    RParen,
    Comma,
    Inf,
    Pi,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        write!(
            f,
            "{}",
            match self {
                Number(j) => j.to_string(),
                Add => "+".to_owned(),
                Sub => "-".to_owned(),
                Mul => "*".to_owned(),
                Div => "/".to_owned(),
                Pow => "^".to_owned(),
                Max => "max".to_owned(),
                Min => "min".to_owned(),
                Sin => "sin".to_owned(),
                Cos => "cos".to_owned(),
                Tan => "tan".to_owned(),
                Sqrt => "sqrt".to_owned(),
                LParen => "(".to_owned(),
                RParen => ")".to_owned(),
                Comma => ",".to_owned(),
                Inf => "inf".to_owned(),
                Pi => "pi".to_owned(),
            }
        )
    }
}

impl Token {
    pub fn is_paren(&self) -> bool {
        use Token::*;
        match self {
            RParen | LParen => true,
            _ => false,
        }
    }
    pub fn is_function(&self) -> bool {
        use Token::*;
        match self {
            Max | Min | Sin | Cos | Tan | Sqrt | Pi | Inf => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        use Token::*;
        match self {
            Add | Sub | Mul | Div | Pow => true,
            _ => false,
        }
    }
    pub fn precedence(&self) -> Option<u8> {
        use Token::*;
        if self.is_operator() {
            match self {
                Add | Sub => Some(1),
                Mul | Div => Some(2),
                Pow => Some(3),
                _ => Some(0),
            }
        } else if self.is_function() {
            Some(0)
        } else {
            None
        }
    }
    pub fn associativity(&self) -> Option<bool> {
        use Token::*;
        if self.is_operator() {
            match self {
                Add | Sub | Mul | Div => Some(false),
                Pow => Some(true),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn evaluate(self, stack: &mut Vec<f64>) -> Option<()> {
        use Token::*;
        match (self.is_function(), self.is_operator()) {
            (true, false) => match self {
                Inf => {
                    stack.push(f64::INFINITY)
                }
                Pi => {
                    stack.push(consts::PI)
                }
                Min => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a.min(b))
                }
                Max => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a.max(b))
                }
                Sin => {
                    let a = stack.pop()?;
                    stack.push(a.sin())
                }
                Cos => {
                    let a = stack.pop()?;
                    stack.push(a.cos())
                }
                Tan => {
                    let a = stack.pop()?;
                    stack.push(a.tan())
                }
                Sqrt => {
                    let a = stack.pop()?;
                    stack.push(a.sqrt())
                }
                _ => panic!["Invalid function!"],
            },
            (false, true) => match self {
                Add => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a + b)
                }
                Sub => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a - b)
                }
                Mul => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a * b)
                }
                Div => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a / b)
                }
                Pow => {
                    let (b, a) = (stack.pop()?, stack.pop()?);
                    stack.push(a.powf(b))
                }
                _ => panic!["Invalid operator!"],
            },
            (false, false) => {
                if let Number(j) = self {
                    stack.push(j)
                }
            }
            _ => {}
        }
        Some(())
    }
}

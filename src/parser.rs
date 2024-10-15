use crate::token::Token;
use crate::token::Token::*;

pub struct Parser {
    pub lexer: crate::lexer::Lexer,
    pub infix_tokens: Vec<crate::token::Token>,
    pub postfix_tokens: Vec<crate::token::Token>,
    pub stack: Vec<f64>,
}
impl Parser {
    pub fn new() -> Self {
        Self {
            lexer: crate::lexer::Lexer::default(),
            infix_tokens: vec![],
            postfix_tokens: vec![],
            stack: vec![],
        }
    }

    pub fn input(mut self, insert: String) -> Self {
        self.lexer = self.lexer.string(insert);
        self
    }

    pub fn tokenize(mut self) -> Self {
        self.infix_tokens = self.lexer.tokennize();
        self
    }

    pub fn shunt(mut self) -> Self {
        let mut input_queue = self.infix_tokens.clone();
        let mut operator_stack: Vec<Token> = Vec::new();
        let mut output_queue: Vec<Token> = Vec::new();
        while !input_queue.is_empty() {
            let tok = input_queue.remove(0);

            match (tok.is_operator(), tok.is_function(), tok.is_paren()) {
                (false, true, false) => operator_stack.push(tok),
                (false, false, false) => output_queue.push(tok),
                (false, _, true) => match tok {
                    LParen => operator_stack.push(tok),
                    RParen => {
                        while let Some(op) = operator_stack.pop() {
                            if op == LParen {
                                break;
                            } else {
                                output_queue.push(op);
                            }
                        }
                    }
                    _ => {}
                },
                (true, _, _) => {
                    while let Some(op) = operator_stack.last() {
                        if op.is_paren() {
                            break;
                        } else if !(op.precedence().unwrap() >= tok.precedence().unwrap()
                            && !tok.associativity().unwrap())
                        {
                            break;
                        } else {
                            output_queue.push(operator_stack.pop().unwrap())
                        };
                    }
                    operator_stack.push(tok);
                }
            }
        }
        while !operator_stack.is_empty() {
            output_queue.push(operator_stack.pop().unwrap());
        }
        self.postfix_tokens = output_queue;
        self
    }
    pub fn evaluate(mut self) -> Self {
        let mut tokens = self.postfix_tokens.clone();
        while !tokens.is_empty() {
            let tok = tokens.remove(0);
            tok.evaluate(&mut self.stack);
        }
        self
    }
}

use crate::token::Token;

pub struct Lexer {
    pub string: String,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            string: String::new(),
        }
    }
}

impl Lexer {
    pub fn string(mut self, input: String) -> Lexer {
        self.string = input;
        self
    }

    pub fn parse_function(func: String) -> Token {
        match func.as_str() {
            "max" => Token::Max,
            "min" => Token::Min,
            "sin" => Token::Sin,
            "cos" => Token::Cos,
            "tan" => Token::Tan,
            "sqrt" => Token::Sqrt,
            _ => panic!["Invalid function! {func}"],
        }
    }
    pub fn parse_op(op: char) -> Token {
        match op {
            '+' => Token::Add,
            '-' => Token::Sub,
            '/' => Token::Div,
            '*' => Token::Mul,
            '^' => Token::Pow,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            _ => panic!["Invalid operator! {op}"],
        }
    }

    pub fn tokennize(&self) -> Vec<Token> {
        let mut to_be_parsed: String = self.string.clone();
        let mut num_stack: String = String::new();
        let mut fnc_stack: String = String::new();
        let mut parsing_num: bool = false;
        let mut parsing_fnc: bool = false;
        let mut output_queue: Vec<Token> = Vec::new();

        while !to_be_parsed.is_empty() {
            let tok = to_be_parsed.remove(0);
            
            match tok {
                ' ' => {},
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    if parsing_fnc {
                        output_queue.push(Self::parse_function(fnc_stack.clone()));
                        parsing_fnc = false;
                    }
                    parsing_num = true;
                    num_stack.push(tok);
                }
                '+' | '-' | '/' | '*' | '^' | '(' | ')' | ',' => {
                    match (parsing_fnc, parsing_num) {
                        (false, true) => {
                            output_queue.push(Token::Number(num_stack.parse::<f64>().unwrap()))
                        }
                        (true, false) => output_queue.push(Self::parse_function(fnc_stack.clone())),
                        (false, false) => {},
                        _ => {
                            panic!["Cannot be parsing function and number at the same time"]
                        }
                    }
                    fnc_stack = String::new();
                    num_stack = String::new();
                    parsing_fnc = false;
                    parsing_num = false;
                    output_queue.push(Self::parse_op(tok));
                }
                _ => {
                    if parsing_num {
                        output_queue.push(Token::Number(num_stack.parse::<f64>().unwrap()));
                        parsing_num = false;
                    }
                    parsing_fnc = true;
                    fnc_stack.push(tok);
                }
            }
        }
        match (parsing_fnc, parsing_num) {
            (false, true) => output_queue.push(Token::Number(num_stack.parse::<f64>().unwrap())),
            (true, false) => output_queue.push(Self::parse_function(fnc_stack.clone())),
            (false,  false) => {},
            _ => {
                panic!["Cannot be parsing function and number at the same time"]
            }
        }

        output_queue.to_owned()
    }
}

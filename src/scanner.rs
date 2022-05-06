use crate::character_stream::CharStream;
use crate::token::{Token, TokenType};
use std::borrow::Borrow;

#[derive(Clone)]
pub struct Scanner {
    pub input_file:Vec<String>,
    pub token_vec:Vec<Token>
}

impl Scanner {
    pub fn new(input:Vec<String>) -> Scanner {
        Scanner {
            input_file: input,
            token_vec: Vec::new(),
        }
    }

    pub fn tokenize(&mut self, file:Vec<String>) {

        let operators= [
            "+", "-", "*", "/", "=",
            "==", ">=", "<=", "!=", "<", ">",
            "{", "}", "(", ")", ",", ";"
	    ];

        let potential_operators = ['+', '-', '*', '/',
            '=', '>', '<', '!',
            '}', '{', ')', '(',
            ',', ';'
        ];

        let keywords = [
            "int", "char", "double", "float",
            "long", "short", "unsigned", "while",
            "if", "return", "void", "main"
	    ];

        let mut line_num: i32 = 1;
        let mut char_num: i32 = 1;

        let mut token_count: i32 = 0;

        for line in file.iter() {
            char_num = 1;
            let mut char_stream = CharStream::new(line);

            while char_stream.more_available() {
                let mut temp_string: String = String::new();
                if !(char_stream.peek_next_char().is_none()) {
                    let curr_char: Option<char> = char_stream.get_next_char();

                    match curr_char {
                        Some(next_char) => {
                            let mut is_float_or_int = false;

                            /* KEYWORD / VARIABLE Matching */
                            if curr_char.unwrap().is_alphabetic() ||
                                curr_char.unwrap() == '_' {
                                temp_string.push(curr_char.unwrap());
                                let mut next_char = char_stream.peek_next_char();

                                if char_stream.peek_next_char().is_none() {
                                    println!("ESCAPE");
                                    break;
                                }

                                while next_char.unwrap().is_alphabetic() ||
                                    next_char.unwrap() == '_' ||
                                    next_char.unwrap().is_numeric() {

                                    next_char = char_stream.get_next_char();
                                    temp_string.push(next_char.unwrap());
                                    char_num += 1;
                                    next_char = char_stream.peek_next_char();

                                    if next_char.is_none() {
                                        println!("ESCAPE");
                                        break;
                                    }
                                }

                                if next_char.unwrap() == ' ' {
                                    let blank_space = char_stream.get_next_char();
                                    char_num += 1;
                                    next_char = char_stream.peek_next_char();
                                }

                                if keywords.contains(&&*temp_string.to_string()) {
                                    let new_token: Token = Token::new(temp_string.to_string(), TokenType::KEYWORD, line_num, char_num - (temp_string.len() as i32));
                                    self.token_vec.push(new_token);
                                    token_count += 1;
                                }

                                else if next_char.unwrap() == '(' {
                                    let new_token: Token = Token::new(temp_string.to_string(), TokenType::FUNCTION, line_num, char_num - (temp_string.len() as i32));
                                    self.token_vec.push(new_token);
                                    token_count += 1;
                                }

                                else {
                                    let new_token: Token = Token::new(temp_string.to_string(), TokenType::VARIABLE, line_num, char_num - (temp_string.len() as i32));
                                    self.token_vec.push(new_token);
                                    token_count += 1;
                                }
                            }

                            /* FLOATCONSTANT / INTCONSTANT Matching */
                            if curr_char.unwrap().is_numeric()
                                || curr_char.unwrap() == '-' {

                                let mut next_char = char_stream.peek_next_char();

                                if next_char.is_none() {
                                    println!("ESCAPE");
                                    break;
                                }

                                if (token_count > 0
                                    && curr_char.unwrap() == '-'
                                    && !(self.token_vec[token_count as usize-1].get_type().as_str() == TokenType::VARIABLE.as_str()
                                    || self.token_vec[token_count as usize-1].get_type().as_str() == TokenType::FLOATCONSTANT.as_str()
                                    || self.token_vec[token_count as usize-1].get_type().as_str() == TokenType::INTCONSTANT.as_str())
                                    && next_char.unwrap().is_numeric()) || curr_char.unwrap() != '-' {

                                    temp_string.push(curr_char.unwrap());

                                    is_float_or_int = true;
                                    let mut float_bool: bool = false;
                                    while next_char.unwrap().is_numeric() ||
                                        next_char.unwrap() == '.' {
                                        if next_char.unwrap() == '.' {
                                            if float_bool == true {
                                                println!("ERROR");
                                            } else {
                                                float_bool == true;
                                            }
                                        }

                                        next_char = char_stream.get_next_char();
                                        temp_string.push(next_char.unwrap());
                                        char_num += 1;
                                        next_char = char_stream.peek_next_char();

                                        if next_char.is_none() {
                                            println!("ESCAPE");
                                            break;
                                        }
                                    }

                                    if temp_string.contains('.') {
                                        let new_token: Token = Token::new(temp_string.to_string(), TokenType::FLOATCONSTANT, line_num, char_num - (temp_string.len() as i32));
                                        self.token_vec.push(new_token);
                                        token_count += 1;
                                    } else {
                                        let new_token: Token = Token::new(temp_string.to_string(), TokenType::INTCONSTANT, line_num, char_num - (temp_string.len() as i32));
                                        self.token_vec.push(new_token);
                                        token_count += 1;
                                    }
                                }
                            }

                            /* OPERATOR Matching */
                            if potential_operators.contains(&curr_char.unwrap())
                                && !(curr_char.unwrap() == '-' && is_float_or_int) {
                                temp_string.push(curr_char.unwrap());
                                let mut next_char = char_stream.peek_next_char();

                                if (curr_char.unwrap() == '='
                                    || curr_char.unwrap() == '<'
                                    || curr_char.unwrap() == '>'
                                    || curr_char.unwrap() == '!')
                                    && next_char.unwrap() == '=' {
                                    next_char = char_stream.get_next_char();
                                    temp_string.push(next_char.unwrap());
                                    char_num += 1;
                                }

                                let new_token: Token = Token::new(temp_string.to_string(), TokenType::OPERATOR, line_num, char_num - (temp_string.len() as i32));
                                self.token_vec.push(new_token);
                                token_count += 1;
                            }

                            if curr_char.unwrap() == ' ' {
                                char_num += 1;
                            }

                        }

                        _ => println!("EOF")
                    }
                }
            }

            line_num += 1;
        }

        for i in &self.token_vec {
            println!("{}: {}", i.get_type().as_str(), i.get_text())
        }
    }

    pub fn get_next_token(&mut self, pos: usize) -> Option<Token> {
        if pos >= self.token_vec.len() {
            return None;
        }

        if !self.token_vec.is_empty() {
            return Option::from(self.token_vec[pos].clone());
        }
        return None
    }

}
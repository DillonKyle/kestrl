use crate::{interpretter, token_types::TokenType};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String, // change this to a struct later.
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> std::string::String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

pub struct Scanner<'a> {
    source: &'a str, // Holds a reference to the input string
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source, // Directly store the reference
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // i think i may need to make a parent Struct or something for Portolyn and
    // drop the error and report functions in there, along with some others in main.rs

    pub fn advance(&mut self) -> char {
        // Get the character at the current byte index
        let c = self.source[self.current..].chars().next().unwrap_or('\0');

        // If it's not EOF, advance the byte index by the character's length
        if c != '\0' {
            self.current += c.len_utf8();
        }

        c
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let test_char = self.source[self.current..].chars().next().unwrap_or('\0');
            println!("Peeked char: '{}'", test_char);
            self.source[self.current..].chars().next().unwrap_or('\0')
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, token_text: Option<String>) {
        match token_text {
            Some(text) => {
                self.tokens.push(Token {
                    token_type,
                    lexeme: text,
                    line: self.line,
                    literal: "".to_string(),
                });
            }
            None => {
                let token_text = &self.source[self.start..self.current];

                self.tokens.push(Token {
                    token_type,
                    lexeme: token_text.to_string(),
                    line: self.line,
                    literal: "".to_string(),
                });
            }
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.peek();
        if c != expected {
            return false;
        }
        self.advance();
        true
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.match_next('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            '\0' => (),
            ' ' | '\r' | '\t' => {
                self.start = self.current;
            }
            '\n' => {
                // self.line += 1;
                self.start = self.current;
            }
            _ => {
                let error_message = format!("Unexpected character: '{c}'");
                eprintln!("[line {}] Error: {}", self.line, error_message);
            }
        }
    }

    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: {}", self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::STRING, value.to_string().into());
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        println!("Scanning tokens for source: {}", self.source);

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.start = self.current;
        //This doesnt work, because the scanner is line by line
        //self.add_token(TokenType::EOF, None);

        std::mem::take(&mut self.tokens)
    }
}

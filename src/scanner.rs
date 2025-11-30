use crate::token_types::TokenType;
use core::f64;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Literal,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.token_type, self.literal)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    Unknown(String),
    Nil,
}

pub type ErrorReporter = dyn FnMut(usize, &str);

pub struct Scanner<'a> {
    source: &'a str, // Holds a reference to the input string
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source, // Directly store the reference
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("false", TokenType::FALSE),
                ("for", TokenType::FOR),
                ("fun", TokenType::FUN),
                ("if", TokenType::IF),
                ("nil", TokenType::NIL),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("true", TokenType::TRUE),
                ("var", TokenType::VAR),
                ("while", TokenType::WHILE),
            ]),
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
            self.source[self.current..].chars().next().unwrap_or('\0')
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, token: Option<Literal>) {
        match token {
            Some(t) => {
                self.tokens.push(Token {
                    token_type,
                    line: self.line,
                    literal: t,
                });
            }
            None => {
                let token_text = &self.source[self.start..self.current];

                self.tokens.push(Token {
                    token_type,
                    line: self.line,
                    literal: Literal::Unknown(token_text.to_string()),
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

    pub fn scan_token<T>(&mut self, reporter: &mut T)
    where
        T: FnMut(usize, &str),
    {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, Some(Literal::Str(c.to_string()))),
            ')' => self.add_token(TokenType::RIGHT_PAREN, Some(Literal::Str(c.to_string()))),
            '{' => self.add_token(TokenType::LEFT_BRACE, Some(Literal::Str(c.to_string()))),
            '}' => self.add_token(TokenType::RIGHT_BRACE, Some(Literal::Str(c.to_string()))),
            ',' => self.add_token(TokenType::COMMA, Some(Literal::Str(c.to_string()))),
            '.' => self.add_token(TokenType::DOT, Some(Literal::Str(c.to_string()))),
            '-' => self.add_token(TokenType::MINUS, Some(Literal::Str(c.to_string()))),
            '+' => self.add_token(TokenType::PLUS, Some(Literal::Str(c.to_string()))),
            ';' => self.add_token(TokenType::SEMICOLON, Some(Literal::Str(c.to_string()))),
            '*' => self.add_token(TokenType::STAR, Some(Literal::Str(c.to_string()))),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(
                    token_type,
                    Some(Literal::Str(
                        self.source[self.start..self.current].to_string(),
                    )),
                );
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(
                    token_type,
                    Some(Literal::Str(
                        self.source[self.start..self.current].to_string(),
                    )),
                );
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(
                    token_type,
                    Some(Literal::Str(
                        self.source[self.start..self.current].to_string(),
                    )),
                );
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(
                    token_type,
                    Some(Literal::Str(
                        self.source[self.start..self.current].to_string(),
                    )),
                );
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
            '"' => {
                self.string(reporter);
            }
            '\0' => (),
            ' ' | '\r' | '\t' => {
                self.start = self.current;
            }
            '\n' => {
                self.line += 1;
                self.start = self.current;
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    let error_message = format!("Unexpected character: '{c}'");
                    reporter(self.line, &error_message);
                }
            }
        }
    }

    pub fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    pub fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    pub fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    pub fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if self.keywords.contains_key(text) {
            let token_type = self.keywords.get(text).unwrap().clone();
            self.add_token(token_type, Some(Literal::Str(text.to_string())));
        } else {
            self.add_token(TokenType::IDENTIFIER, Some(Literal::Str(text.to_string())));
        }
    }

    pub fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::NUMBER, Some(Literal::Number(*value)));
    }

    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1..]
                .chars()
                .next()
                .unwrap_or('\0')
        }
    }

    pub fn string<T>(&mut self, reporter: &mut T)
    where
        T: FnMut(usize, &str),
    {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            reporter(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        let trimmed_and_normalized = value
            .replace("\\", "")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");
        self.add_token(
            TokenType::STRING,
            Some(Literal::Str(trimmed_and_normalized)),
        );
    }

    pub fn scan_tokens<T>(&mut self, reporter: &mut T) -> Vec<Token>
    where
        T: FnMut(usize, &str),
    {
        println!("Scanning tokens for source: {}", self.source);

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(reporter);
        }

        self.start = self.current;
        //This doesnt work, because the scanner is line by line
        //self.add_token(TokenType::EOF, None);

        std::mem::take(&mut self.tokens)
    }
}

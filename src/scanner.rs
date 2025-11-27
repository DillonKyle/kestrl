use crate::token_types::TokenType;
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
    pub fn new(source: &'a str, line: usize) -> Self {
        Scanner {
            source, // Directly store the reference
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line, //lines are currently managed by buf reader, may need to adjust later
        }
    }

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
        self.source[self.current..].chars().next().unwrap_or('\0')
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        let token_text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: token_text.to_string(),
            line: self.line,
            literal: "".to_string(),
        });
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
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_next('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
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
                println!("Error: Unexpected character: {c}");
                self.add_token(TokenType::UNKNOWN);
            }
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        println!("Scanning tokens for source: {}", self.source);

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.start = self.current;
        self.add_token(TokenType::EOF);

        self.tokens
    }
}

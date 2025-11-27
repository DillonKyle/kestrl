use crate::token_types::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String, // change this to a struct later.
    line: usize,
}

impl Token {
    fn to_string(&self) -> std::string::String {
        return self.token_type.to_string() + " " + &self.lexeme + " " + &self.literal;
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current).unwrap()
    }

    pub fn peek() {}

    pub fn add_token(&mut self, token_type: TokenType) {
        let token_text = &mut self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: token_text.to_string(),
            line: 0,
            literal: "".to_string(),
        });
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
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
            _ => self.add_token(TokenType::UNKNOWN),
        }
    }

    pub fn scan_tokens(&mut self) -> &mut std::vec::Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF);
        &mut self.tokens
    }
}

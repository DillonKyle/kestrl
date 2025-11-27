use portolyn::token_types::TokenType;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the call to the binary, at least when using 'cargo run'
    if args.len() > 2 {
        eprintln!("Usage: portolyn -- {}", args[1]);
        std::process::exit(1);
    } else if args.len() == 2 {
        run_file(args[1].as_str()).unwrap();
    } else {
        let ended_prompt = run_repl();
        match ended_prompt {
            Ok(_) => eprintln!("REPL ended successfully"),
            Err(e) => eprintln!("Error in REPL: {e}"),
        }
    }
}

fn run_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        run(&line);
    }

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    loop {
        print!(">>>");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.trim() == "exit" {
            println!("Exiting REPL.");
            break;
        }

        let tokens = Scanner::new(&line).scan_tokens();
    }
    Ok(())
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
}

struct Token {
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

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance() {}

    fn peek() {}

    fn add_token(&mut self, token_type: TokenType) {
        let token_text = &mut self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: token_text.to_string(),
            line: 0,
            literal: "".to_string(),
        });
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {}

    fn scan_tokens(&mut self) -> &mut std::vec::Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF);
        &mut self.tokens
    }
}

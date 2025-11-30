use crate::{parser::Parser, scanner::Scanner};

pub struct Interpreter<'a> {
    source: &'a str,
    pub had_error: bool,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        Interpreter {
            source,
            had_error: false,
        }
    }

    pub fn run(&mut self) {
        let mut scanner = Scanner::new(self.source);
        let had_error_flag = &mut self.had_error;
        let mut reporter = |line: usize, message: &str| {
            *had_error_flag = true;
            eprintln!("[line {line}] Error: {message}");
        };

        let tokens = scanner.scan_tokens(&mut reporter);
        println!("Error?: {had_error_flag}");

        if *had_error_flag {
            return;
        }
        // TODO: Add Parser and Executor logic here
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        println!("Parsed expression: {:?}", expression);
    }

    pub fn run_line(&mut self, line_source: &str, line_number: usize) {
        let mut scanner = Scanner::new(line_source);
        let had_error_flag = &mut self.had_error;
        let mut reporter = |line: usize, message: &str| {
            *had_error_flag = true;
            eprintln!("[line {line}] Error: {message}");
        };

        let tokens = scanner.scan_tokens(&mut reporter);
        println!("Tokens for line {}:", line_number);
        println!("Error?: {had_error_flag}");

        if *had_error_flag {
            return;
        }
        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.had_error = true;
        self.report(line, "", message);
    }

    pub fn report(&self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
    }
}

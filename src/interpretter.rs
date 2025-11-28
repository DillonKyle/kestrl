use crate::scanner::Scanner;

pub struct Interpreter<'a> {
    source: &'a str,
    had_error: bool,
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
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn run_line(&mut self, line_source: &str, line_number: usize) {
        let mut scanner = Scanner::new(line_source);
        let tokens = scanner.scan_tokens();
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

use portolyn::scanner::Scanner;
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
    let mut line_cnt = 1;
    for line_result in reader.lines() {
        let line = line_result?;
        run(&line, line_cnt);
        line_cnt += 1;
    }

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line_cnt = 1;
    loop {
        print!(">>>");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.trim() == "exit" {
            println!("Exiting REPL.");
            break;
        }
        println!("Input: {}", line.trim());

        let scanner = Scanner::new(&line, line_cnt);

        let tokens = scanner.scan_tokens();
        println!("{:?}", tokens);
        line_cnt += 1;
    }
    Ok(())
}

fn run(source: &str, line: usize) {
    let scanner = Scanner::new(source, line);
    let tokens = scanner.scan_tokens();
    println!("{:?}", tokens);
}

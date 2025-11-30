use kestrl::kestrl::Kestrl;
use std::env;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first argument is the call to the binary, at least when using 'cargo run'
    if args.len() > 2 {
        eprintln!("Usage: kestrl -- {}", args[1]);
        std::process::exit(1);
    } else if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Error running file {}: {}", &args[1], e);
            std::process::exit(1);
        }
    } else {
        match run_repl() {
            Ok(_) => eprintln!("REPL ended successfully"),
            Err(e) => eprintln!("Error in REPL: {e}"),
        }
    }
}

fn run_file(file_path: &str) -> io::Result<()> {
    let contents = std::fs::read_to_string(file_path)?;
    let mut kestrl = Kestrl::new(&contents);
    kestrl.run();

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line_cnt = 1;
    let mut kestrl = Kestrl::new("");

    loop {
        print!(">>>");
        io::stdout().flush()?;
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 || line.trim() == "exit" {
            println!("Exiting REPL.");
            break;
        }
        kestrl.run_line(&line, line_cnt);

        line_cnt += 1;
    }
    Ok(())
}

use std::io::{self, Write};
use lox0::token::Token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file(file_path: &str) -> io::Result<()> {
    run(&std::fs::read_to_string(file_path)?);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    // let mut had_error = false;

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        let _ = stdin.read_line(&mut line)?;

        print!("{line}");
        run(&line);
        // had_error = false;
    }
}

struct Scanner<'a> {
    #[allow(unused)]
    source: &'a str,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn scan_tokens(self) -> Vec<Token> {
        vec![]
    }
}

fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {line}] Error {where_}: {message}");
    // had_error = true;
}

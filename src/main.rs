use lox0::parser::Parser;
use lox0::scanner::Scanner;
use std::io::{self, Write};

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
        let bytes = stdin.read_line(&mut line)?;

        // EOF
        if bytes == 0 {
            break Ok(());
        }

        print!("{line}");
        run(&line);
        // had_error = false;
    }
}

fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let token_types = tokens.into_iter().map(|token| token.ty).collect();
    let _parser = Parser::new(&token_types);
    // println!("{:?}", tokens);
    //
    // for token in tokens {
    //     println!("{}", token);
    // }
}

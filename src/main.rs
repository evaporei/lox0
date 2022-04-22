fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args.next().unwrap())?;
    } else {
        run_prompt();
    }

    Ok(())
}

fn run_file(file_path: &str) -> std::io::Result<()> {
    run(&std::fs::read_to_string(file_path)?);
    Ok(())
}

fn run_prompt() {}

fn run(_source: &str) {}

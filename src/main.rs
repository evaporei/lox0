fn main() {
    let mut args = std::env::args();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args.next().unwrap());
    } else {
        run_prompt();
    }
}

fn run_file(_file_name: &str) {}

fn run_prompt() {}

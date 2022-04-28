pub fn error(line: usize, message: &str) -> ! {
    report(line, "", message)
}

pub fn report(line: usize, where_: &str, message: &str) -> ! {
    eprintln!("[line {line}] Error {where_}: {message}");
    // had_error = true;
    panic!("bad (tmp)")
}

pub fn fail_normal(msg: &str) {
    eprintln!("\x1b[91mERROR:\x1b[0m {}", msg);
}

pub fn fail_error(msg: Box<dyn std::error::Error>) {
    eprintln!("\x1b[91mERROR:\x1b[0m {:?}", msg);
}

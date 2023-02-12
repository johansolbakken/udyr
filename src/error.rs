use std::fmt::format;

pub fn error(line: usize, message: &str) -> String {
    report(line, "", message)
}

pub fn report(line: usize, location: &str, message: &str) -> String {
    format!("[line {}] Error{}: {}", line, location, message)
}

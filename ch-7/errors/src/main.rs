use std;
use std::fmt;
use std::error::Error;
use std::io::{Write, stderr};

fn main() {
    println!("Hello, world!");

    match get_an_error() {
        Ok(()) => println!("woops! no error!"),
        Err(err) => print_error(&err)
    }
}

fn get_an_error() -> Result<(), JsonError> {
    Err(JsonError {
        message: "expected some value at some point in the JSON".to_string(),
        line: 42,
        column: 8
    })
}

/// Dump an error message to `stderr`.
///
/// If another error happens while building the error messag eor
/// writing to `stderr`, it is ignored.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// Errors should be printable.
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait.
impl std::error::Error for JsonError {
    fn description(&self) -> &str {
        &self.message
    }
}

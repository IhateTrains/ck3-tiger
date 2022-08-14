use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

use crate::scope::Token;

static mut ERRORS: Errors = Errors {};

#[derive(Clone, Debug, Default)]
pub struct Errors {}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKey {
    ParseError,
    Packaging,
    Validation,
    TooManyErrors,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorLevel {
    Error,
    Warning,
    Advice,
}

impl Errors {
    #[allow(clippy::unused_self)] // At some point we will cache files in self
    fn get_line(&mut self, token: &Token) -> Option<String> {
        read_to_string(&*token.loc.pathname)
            .ok()
            .and_then(|contents| contents.lines().nth(token.loc.line - 1).map(str::to_string))
    }

    pub fn push(&mut self, token: &Token, level: ErrorLevel, _key: ErrorKey, msg: &str) {
        if let Some(line) = self.get_line(token) {
            let line_marker = token.loc.line_marker();
            eprintln!("{}{}", line_marker, line);
            eprintln!("{}{:<count$}", line_marker, "^", count = token.loc.column);
        }
        eprintln!("{}{}: {}", token.loc.marker(), level, msg);
    }

    pub fn get_mut() -> &'static mut Self {
        // Safe because we're single-threaded, and won't start reporting
        // validation errors until we're well past initialization.
        unsafe { &mut ERRORS }
    }
}

pub fn error(token: &Token, key: ErrorKey, msg: &str) {
    Errors::get_mut().push(token, ErrorLevel::Error, key, msg);
}

pub fn warn(token: &Token, key: ErrorKey, msg: &str) {
    Errors::get_mut().push(token, ErrorLevel::Warning, key, msg);
}

pub fn advice(token: &Token, key: ErrorKey, msg: &str) {
    Errors::get_mut().push(token, ErrorLevel::Advice, key, msg);
}

impl Display for ErrorLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ErrorLevel::Error => write!(f, "ERROR"),
            ErrorLevel::Warning => write!(f, "WARNING"),
            ErrorLevel::Advice => write!(f, "ADVICE"),
        }
    }
}

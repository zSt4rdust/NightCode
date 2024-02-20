use crate::frontend::lexer::Location;
use std::process;

pub struct Error<'a> {
    pub kind: ErrorKind,
    pub message: &'a str,
    pub location: &'a Location,
}

impl<'a> Error<'a> {
    pub fn new(kind: ErrorKind, message: &'a str, location: &'a Location) -> Error<'a> {
        Error {
            kind,
            message,
            location,
        }
    }

    pub fn syntax(message: &'a str, location: &'a Location) -> Error<'a> {
        Self::new(ErrorKind::Syntax_Error, message, location)
    }

    pub fn parser(message: &'a str, location: &'a Location) -> Error<'a> {
        Self::new(ErrorKind::Parser_Error, message, location)
    }

    pub fn throw(&self) {
        let error_kind = self.kind.to_string().replace("_", " ");
        println!("{}: {} {}", error_kind, self.message, self.location);
        process::exit(1);
    }
}

#[derive(strum_macros::Display)]
#[allow(non_camel_case_types)]
pub enum ErrorKind {
    Syntax_Error,
    Parser_Error,
}

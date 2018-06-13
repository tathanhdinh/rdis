use std::{fmt, result, convert, io};

use failure::{Backtrace, Context, Fail};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
        // fmt::Display::fmt(&self.ctx, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    Io(String),
    Assemble(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Io(ref msg) => {
                write!(f, "IO error: {}", msg)
            },

            ErrorKind::Assemble(ref msg) => {
                write!(f, "Assembling error: {}", msg)
            }
        }
    }
}

impl convert::From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        // Error { ctx: Context::new(kind) }
        Error::from(Context::new(kind))
    }
}

impl convert::From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Self {
        Error { ctx }
    }
}
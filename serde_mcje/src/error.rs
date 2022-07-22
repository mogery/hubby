use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error{
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),
    Io(std::io::Error),

    Eof,
    VarIntOverflow,
    VarLongOverflow,
    NotSupported(String),
    TrailingBytes,
    ExpectedBoolean,
    MalformedUTF8
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Io(_) => formatter.write_str("IO error"),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::VarIntOverflow => formatter.write_str("VarInt is too big"),
            Error::VarLongOverflow => formatter.write_str("VarLong is too big"),
            Error::NotSupported(feature) => {
                formatter.write_str("feature ")
                    .and_then(|()| formatter.write_str(feature))
                    .and_then(|()| formatter.write_str(" not supported by format"))
            },
            Error::TrailingBytes => formatter.write_str("trailing bytes have been left"),
            Error::ExpectedBoolean => formatter.write_str("expected a boolean"),
            Error::MalformedUTF8 => formatter.write_str("malformed UTF-8 string"),
        }
    }
}

impl std::error::Error for Error {}
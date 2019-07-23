use std::fmt;
use std::io;

use super::BookId;

pub type LibraryResult<T> = Result<T, LibraryError>;

pub enum LibraryError {
    // internal
    /// File not found (path)
    FileNotFound(String),
    /// Book not found (id)
    BookNotFound(BookId),

    // std
    /// `std::io`
    Io(io::Error),

    // external

    // `serde_json`
    SerdeJson(serde_json::Error),
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::FileNotFound(path) => write!(f, "File not found: {}", path),
            LibraryError::BookNotFound(id) => write!(f, "Book not found: {}", id),
            LibraryError::Io(e) => e.fmt(f),
            LibraryError::SerdeJson(e) => e.fmt(f),
        }
    }
}

impl fmt::Debug for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::FileNotFound(path) => f.debug_tuple("FiledNotFound").field(path).finish(),
            LibraryError::BookNotFound(id) => f.debug_tuple("BookNotFound").field(id).finish(),
            LibraryError::Io(e) => f.debug_tuple("Io").field(e).finish(),
            LibraryError::SerdeJson(e) => f.debug_tuple("SerdeJson").field(e).finish(),
        }
    }
}

impl From<io::Error> for LibraryError {
    fn from(err: io::Error) -> LibraryError {
        LibraryError::Io(err)
    }
}

impl From<serde_json::Error> for LibraryError {
    fn from(err: serde_json::Error) -> LibraryError {
        LibraryError::SerdeJson(err)
    }
}

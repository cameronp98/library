//! misc

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

mod error;

pub use error::{LibraryError, LibraryResult};

/// The type returned by `Library::add_book()` which uniquely identifies a book
pub type BookId = usize;

/// A collection of books
#[derive(Serialize, Deserialize)]
pub struct Library {
    id_counter: usize,
    books: HashMap<BookId, Book>,
}

impl Library {
    /// Create a new library
    pub fn new() -> Library {
        Library {
            id_counter: 0,
            books: HashMap::new(),
        }
    }

    /// Load a library from a json file
    pub fn from_file<P: AsRef<Path>>(path: P) -> LibraryResult<Library> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    /// Serialize this library at the given path
    pub fn save<P: AsRef<Path>>(&self, path: P) -> LibraryResult<()> {
        let mut file = File::create(path)?;
        let json = serde_json::to_string(self)?;
        write!(file, "{}", json)?;
        Ok(())
    }

    /// Add a book to the library (in memory only)
    pub fn add_book(&mut self, title: String) -> BookId {
        let id = self.id_counter;
        self.id_counter += 1;

        let book = Book { id, title };

        self.books.insert(id, book);

        id
    }

    /// Check if the library contains the book with the given id
    pub fn has_book(&self, id: &BookId) -> bool {
        self.books.contains_key(id)
    }

    /// Returns a reference to the book with the given id
    pub fn get_book(&mut self, id: &BookId) -> LibraryResult<&Book> {
        self.books.get(id).ok_or(LibraryError::BookNotFound(*id))
    }

    /// Returns a mutable reference to the book with the given id
    pub fn get_book_mut(&mut self, id: &BookId) -> LibraryResult<&mut Book> {
        self.books
            .get_mut(id)
            .ok_or(LibraryError::BookNotFound(*id))
    }

    /// Remove the book with the given id from the library
    pub fn remove_book(&mut self, id: &BookId) -> LibraryResult<Book> {
        self.books.remove(id).ok_or(LibraryError::BookNotFound(*id))
    }

    /// Returns an iterator over all books in the library (unordered)
    pub fn iter_books(&self) -> impl Iterator<Item = &Book> {
        self.books.values()
    }
}


/// A book with an id and a title
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    id: BookId,
    title: String,
}

impl Book {
    /// Returns the books' unique id
    pub fn id(&self) -> &BookId {
        &self.id
    }

    /// Returns the book's title
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Set the title of the book to the given string
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

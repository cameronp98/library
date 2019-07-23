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

pub type BookId = usize;

#[derive(Serialize, Deserialize)]
pub struct Library {
    id_counter: usize,
    books: HashMap<BookId, Book>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            id_counter: 0,
            books: HashMap::new(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> LibraryResult<Library> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> LibraryResult<()> {
        let mut file = File::create(path)?;
        let json = serde_json::to_string(self)?;
        write!(file, "{}", json)?;
        Ok(())
    }

    pub fn add_book(&mut self, title: String) -> BookId {
        let id = self.id_counter;
        self.id_counter += 1;

        let book = Book { id, title };

        self.books.insert(id, book);

        id
    }

    pub fn has_book(&self, id: &BookId) -> bool {
        self.books.contains_key(id)
    }

    pub fn get_book(&mut self, id: &BookId) -> LibraryResult<&Book> {
        self.books.get(id).ok_or(LibraryError::BookNotFound(*id))
    }

    pub fn get_book_mut(&mut self, id: &BookId) -> LibraryResult<&mut Book> {
        self.books
            .get_mut(id)
            .ok_or(LibraryError::BookNotFound(*id))
    }

    pub fn remove_book(&mut self, id: &BookId) -> LibraryResult<Book> {
        self.books.remove(id).ok_or(LibraryError::BookNotFound(*id))
    }

    pub fn iter_books(&self) -> impl Iterator<Item = &Book> {
        self.books.values()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    id: BookId,
    title: String,
}

impl Book {
    fn id(&self) -> &BookId {
        &self.id
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

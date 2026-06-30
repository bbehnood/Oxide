use crate::{Position, Range, Result, TextStorage};

pub struct Buffer<S: TextStorage> {
    storage: S,
}

impl<S: TextStorage> Buffer<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        self.storage.line(index)
    }

    pub fn line_count(&self) -> usize {
        self.storage.line_count()
    }

    pub fn len_chars(&self) -> usize {
        self.storage.len_chars()
    }

    pub fn insert(&mut self, pos: Position, text: &str) -> Result<()> {
        self.storage.insert(pos, text)
    }

    pub fn delete(&mut self, range: Range) -> Result<()> {
        self.storage.delete(range)
    }
}

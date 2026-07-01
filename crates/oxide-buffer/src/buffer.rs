use crate::{Position, Range, Result, TextStorage};

pub struct Buffer<S: TextStorage> {
    storage: S,
}

impl<S: TextStorage> Buffer<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn line(&self, index: usize) -> Option<S::Line<'_>> {
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

    pub fn line_len(&self, line: usize) -> Option<usize> {
        self.line(line).map(|line| line.as_ref().chars().count())
    }

    pub fn is_last_line(&self, line: usize) -> bool {
        line + 1 == self.line_count()
    }

    pub fn is_last_column(&self, pos: Position) -> bool {
        self.line_len(pos.line).is_some_and(|len| pos.column == len)
    }

    pub fn is_valid_position(&self, pos: Position) -> bool {
        self.line_len(pos.line).is_some_and(|len| pos.column <= len)
    }
}

use oxide_buffer::{Position, Result, TextStorage};

use crate::{Command, Cursor, Document};

pub struct Editor<S: TextStorage> {
    document: Document<S>,
    cursor: Cursor,
}

impl<S: TextStorage> Editor<S> {
    pub fn new(document: Document<S>) -> Self {
        Self { document, cursor: Cursor::new() }
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    pub fn document(&self) -> &Document<S> {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document<S> {
        &mut self.document
    }

    pub fn execute(&mut self, command: Command) -> Result<()> {
        match command {
            Command::MoveTo(pos) => self.cursor.set(pos),

            Command::Insert(ch) => {
                let pos = self.cursor.position();

                self.document.insert(pos, &ch.to_string())?;

                self.cursor.set(Position::new(pos.line, pos.column + 1));
            }

            Command::InsertText(text) => {
                let pos = self.cursor.position();

                self.document.insert(pos, &text)?;

                // TODO: Improve cursor movement
            }

            Command::DeleteRange(range) => {
                self.document.delete(range)?;
                self.cursor.set(range.start);
            }

            Command::NewLine => {
                let pos = self.cursor.position();

                self.document.insert(pos, "\n")?;

                self.cursor.set(Position::new(pos.line + 1, 0));
            }

            _ => {}
        }

        Ok(())
    }
}

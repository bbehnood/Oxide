use oxide_buffer::{Position, Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    MoveTo(Position),

    Insert(char),
    InsertText(String),

    Backspace,
    Delete(Position),
    DeleteRange(Range),

    NewLine,
}

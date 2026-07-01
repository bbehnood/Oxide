use oxide_buffer::{Position, Range};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    MoveTo(Position),

    Insert(char),

    Delete(Range),

    Backspace,

    NewLine,
}

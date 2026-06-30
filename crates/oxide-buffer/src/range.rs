use crate::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Range {
    start: Position,
    end: Position,
}

impl Range {
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

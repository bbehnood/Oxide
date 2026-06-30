mod vec;

pub use vec::VecStorage;

use crate::{Position, Range, Result};

pub trait TextStorage {
    fn line(&self, index: usize) -> Option<&str>;

    fn line_count(&self) -> usize;

    fn len_chars(&self) -> usize;

    fn insert(&mut self, pos: Position, text: &str) -> Result<()>;

    fn delete(&mut self, range: Range) -> Result<()>;
}

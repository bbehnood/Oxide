pub type Result<T> = std::result::Result<T, DocumentError>;

#[derive(Debug, thiserror::Error)]
pub enum DocumentError {
    #[error("document has no path; use `save_as` instead")]
    NoPath,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Buffer(#[from] oxide_buffer::BufferError),
}

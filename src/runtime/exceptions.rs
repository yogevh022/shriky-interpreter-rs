#[derive(Debug)]
pub(crate) enum RuntimeError {
    OutOfBounds(String),
    NoEntryFound(String),
    InvalidOperation(String),
    InvalidType(String),
    ArgumentCount(String),
}

#[derive(Debug)]
pub(crate) enum RuntimeError {
    OutOfBounds(String),
    EntryNotFound(String),
    VariableNotFound(String),
    InvalidOperation(String),
    InvalidType(String),
    ArgumentCount(String),
    AttributeError(String),
}

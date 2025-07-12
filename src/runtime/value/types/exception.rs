use std::borrow::Cow;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct ExceptionValue(pub Cow<'static, str>);

impl ExceptionValue {
    pub fn runtime(&self, message: String) -> RuntimeException {
        RuntimeException {
            kind: self.clone(),
            message,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct RuntimeException {
    pub kind: ExceptionValue,
    pub message: String,
}

pub static VALUE: ExceptionValue = ExceptionValue(Cow::Borrowed("ValueError"));
pub static TYPE: ExceptionValue = ExceptionValue(Cow::Borrowed("TypeError"));
pub static ATTRIBUTE: ExceptionValue = ExceptionValue(Cow::Borrowed("AttributeError"));
pub static ARGUMENT: ExceptionValue = ExceptionValue(Cow::Borrowed("ArgumentError"));
pub static INDEX_ERROR: ExceptionValue = ExceptionValue(Cow::Borrowed("IndexError"));
pub static KEY_ERROR: ExceptionValue = ExceptionValue(Cow::Borrowed("KeyError"));
pub static INVALID_OPERATION: ExceptionValue = ExceptionValue(Cow::Borrowed("InvalidOperation"));

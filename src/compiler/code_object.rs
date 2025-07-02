pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

pub struct CodeObject {
    byte_code: Vec<u8>,
    constants: Vec<Value>,
    names: Vec<String>,
}

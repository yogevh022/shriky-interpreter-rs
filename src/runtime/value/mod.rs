mod base;
mod binary;
mod constructors;
mod indexable;
mod methods;
mod types;
mod utils;

pub use base::Value;
pub use methods::MethodFn;
use std::cell::RefCell;
use std::rc::Rc;
pub use types::class::ClassValue;
pub use types::function::FunctionValue;
pub use types::instance::InstanceValue;
pub use types::list::ListValue;
pub use types::map::MapValue;
pub use types::method::MethodValue;
pub use types::rust_method::*;

pub type ValueRef = Rc<RefCell<Value>>;

pub use binary::traits;

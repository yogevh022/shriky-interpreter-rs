use crate::runtime::Runtime;
use crate::runtime::utils::extract_string_ref;
use crate::runtime::value::exception;
use crate::runtime::value::{RuntimeException, Value};

pub(crate) fn binary_subscribe(runtime: &mut Runtime) -> Result<(), RuntimeException> {
    let constant = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    let mut container_ref = container.borrow_mut();
    let subscriptable_container = container_ref.subscriptable().ok_or(
        exception::TYPE.runtime("Attempted subscription to an unsubscribable type".to_string()),
    )?;
    let result = subscriptable_container.index(runtime, &constant)?;
    runtime.mem_stack.push(result.clone());
    Ok(())
}

pub(crate) fn access_attr(runtime: &mut Runtime) -> Result<(), RuntimeException> {
    let attr = runtime.mem_stack.pop().unwrap();
    let container = runtime.mem_stack.pop().unwrap();
    let attr_string = extract_string_ref(&attr);
    let result = {
        (&mut *container.borrow_mut())
            .attribute_accessible()
            .ok_or(
                exception::ATTRIBUTE
                    .runtime("Attempted attribute access on an attribute-less type".to_string()),
            )?
            .get_attr(runtime, &attr_string)?
    };

    match &mut *result.borrow_mut() {
        Value::Method(method_value) => method_value.caller = Some(container.clone()),
        Value::RustMethod(method_value) => method_value.caller = Some(container.clone()),
        _ => {}
    }

    runtime.mem_stack.push(result);
    Ok(())
}

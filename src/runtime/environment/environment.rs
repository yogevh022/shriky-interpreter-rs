use crate::parser::ExprNode;
use crate::runtime::values::{RuntimeValue, ObjectValue};
pub struct Environment {
    pub scope_chain: Vec<ObjectValue>,
}

impl Environment {
    pub fn new() -> Environment {
        let scope_chain = vec![ObjectValue { properties: indexmap::IndexMap::new() }];
        Self { scope_chain}
    }

    pub fn descend_scope(&mut self) {
        self.scope_chain.push(ObjectValue { properties: indexmap::IndexMap::new() });
    }

    pub fn ascend_scope(&mut self) {
        if self.scope_chain.len() <= 1 {
            panic!("Cannot ascend past global scope.")
        }
        self.scope_chain.pop();
    }

    fn handle_access_property(&mut self, container: &RuntimeValue, address: &[RuntimeValue]) -> Option<RuntimeValue> {
        let mut value = Some(container);
        for part in address {
            value = match value {
                Some(RuntimeValue::Object(obj)) => obj.properties.get(part),
                Some(RuntimeValue::List(list)) => {
                    if let RuntimeValue::Int(i) = part {
                        list.elements.get(*i as usize)
                    } else { None }
                },
                _ => panic!("Cannot access property from {:?}", value)
            }
        }
        value.cloned()
    }
    
    pub fn get_value(&mut self, address: Vec<RuntimeValue>) -> Option<RuntimeValue> {
        for scope in self.scope_chain[1..] {
            if let Some(value) = scope.properties.get(&address[0]) {
                return self.handle_access_property(value, &address[1..]);
            }
        }
        None
    }
    
    pub fn set_scope_value(&mut self, address: Vec<RuntimeValue>, value: RuntimeValue) {
        self.scope_chain.insert()
    }
}
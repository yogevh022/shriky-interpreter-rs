use std::collections::{HashMap, HashSet};
use crate::runtime::environment::utils::Counter;
use crate::runtime::values::{RuntimeValue, ObjectValue, ReferenceValue, StringValue, ListValue};
use crate::runtime::values::error::AccessError;

pub struct MemoryReference {
    pub value: RuntimeValue,
    pub referenced_by: HashSet<usize>
}

pub struct Environment {
    pub scope_chain: Vec<RuntimeValue>,
    memory: HashMap<usize, MemoryReference>,
}

pub fn access_property<'a>(container: Option<&'a RuntimeValue>, member: &RuntimeValue) -> Result<Option<&'a usize>, AccessError> {
    match container {
        Some(RuntimeValue::Object(obj)) => obj.get_property(member),
        Some(RuntimeValue::List(list)) => list.get_element(member),
        _ => Err(AccessError::InvalidAccess)
    }
}

impl Environment {
    pub fn new() -> Environment {
        let scope_chain = vec![RuntimeValue::Object(ObjectValue {
            properties: indexmap::IndexMap::from([
                (RuntimeValue::String(StringValue { value: String::from("object"), id: Counter.next() }), 123usize),
            ]),
            id: Counter.next(),
        })];
        let mut memory = HashMap::new();
        memory.insert(
            123,
            MemoryReference {
                value: RuntimeValue::Object(ObjectValue { properties: indexmap::IndexMap::from([
                    (RuntimeValue::String(StringValue { value: String::from("prop"), id: Counter.next() }), 124usize),
                ]), id: Counter.next() }),
                referenced_by: HashSet::new()
            });
        memory.insert(
            124,
            MemoryReference {
                value: RuntimeValue::List(ListValue::default()),
                referenced_by: HashSet::new()
            }
        );
        Self { scope_chain, memory }
    }

    pub fn descend_scope(&mut self) {
        self.scope_chain.push(RuntimeValue::Object(ObjectValue::default()));
    }

    pub fn ascend_scope(&mut self) {
        if self.scope_chain.len() <= 1 {
            panic!("Cannot ascend past global scope.")
        }
        self.scope_chain.pop();
    }

    fn access_memory_at(&self, key: &usize) -> Option<&MemoryReference> {
        self.memory.get(key)
    }

    fn get_memory_address_from_object<'a>(&'a self, mut container: Option<&'a RuntimeValue>, address: &[RuntimeValue]) -> Result<Option<&'a usize>, AccessError> {
        let mut mem_addr = None;
        for part in address.iter() {
                mem_addr = match access_property(container, part) {
                    Ok(mem_addr_opt) => { mem_addr_opt }
                    Err(access_error) => return Err(access_error),
                };
                let memory_reference = match mem_addr {
                    Some(mem_addr) => self.memory.get(mem_addr),
                    None => return Err(AccessError::InvalidAddress),
                };
                container = Some(&memory_reference.unwrap().value);
        }
        Ok(mem_addr)
    }
    
    pub fn get_memory_address(&mut self, address: &[RuntimeValue]) -> Result<Option<& usize>, AccessError> {
        let scope = self.scope_chain.iter().rev().find(|scope| match scope {
            RuntimeValue::Object(obj) => { obj.properties.contains_key(&address[0]) },
            _ => false
        });
        match scope {
            Some(RuntimeValue::Object(_)) => self.get_memory_address_from_object(scope, address),
            _ => unreachable!()
        }
    }
    
    pub fn zzz(&mut self, address: &[RuntimeValue]) -> Option<&mut MemoryReference> {
        let z= self.get_memory_address(address);
        self.memory.get_mut(&123usize)
    }
    
    // pub fn set_value(&mut self, address: &[RuntimeValue], value: &MemoryReference) -> Result<(), AccessError> {
    //     let mem_ref = self.get_memory_reference(address)?;
    //     match mem_ref {
    //         Some(mut mem_ref) => {
    //             mem_ref.value = value
    //         }
    //     }
    // }
}
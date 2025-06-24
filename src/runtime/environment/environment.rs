use crate::parser::ExprNode;
use crate::runtime::environment::utils::Counter;
use crate::runtime::values::error::AccessError;
use crate::runtime::values::{
    BoolValue, FloatValue, HasId, IdentityValue, IntValue, ListValue, ObjectValue, ReferenceValue,
    RuntimeValue, StringValue,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct MemoryReference {
    pub value: RuntimeValue,
    pub referrers: u64,
}

impl MemoryReference {
    pub fn new(value: RuntimeValue) -> MemoryReference {
        MemoryReference {
            value,
            referrers: 1,
        }
    }

    pub fn add_referer(&mut self) {
        self.referrers += 1;
    }

    pub fn remove_referer(&mut self) {
        self.referrers -= 1;
    }

    pub fn is_unreachable(&self) -> bool {
        self.referrers == 0
    }
}

pub struct Environment {
    pub scope_chain: Vec<RuntimeValue>,
    memory: HashMap<u64, MemoryReference>,
}

pub fn access_property(
    container: Option<&RuntimeValue>,
    member: &RuntimeValue,
) -> Result<Option<u64>, AccessError> {
    match container {
        Some(RuntimeValue::Object(obj)) => obj.get_property(member).map(|opt| opt.copied()),
        Some(RuntimeValue::List(list)) => list.get_element(member).map(|opt| opt.copied()),
        _ => Err(AccessError::InvalidAccess),
    }
}

pub fn set_property(
    container: &mut RuntimeValue,
    key: &RuntimeValue,
    value_addr: &u64,
) -> Result<(), AccessError> {
    match container {
        RuntimeValue::Object(obj) => Ok(obj.set_property(key, *value_addr)),
        RuntimeValue::List(list) => Ok(list.set_element(key, *value_addr)?),
        _ => Err(AccessError::InvalidAccess),
    }
}

fn expr_to_runtime(value: ExprNode) -> RuntimeValue {
    match value {
        ExprNode::Int(int_node) => RuntimeValue::Int(IntValue::from(int_node)),
        ExprNode::Float(float_node) => RuntimeValue::Float(FloatValue::from(float_node)),
        ExprNode::Bool(bool_node) => RuntimeValue::Bool(BoolValue::from(bool_node)),
        ExprNode::String(string_node) => RuntimeValue::String(StringValue::from(string_node)),
        _ => panic!("expr_to_runtime can only be used for primitive types!"),
    }
}

impl Environment {
    pub fn new() -> Environment {
        let scope_chain = vec![RuntimeValue::Object(ObjectValue::default())];
        let mut memory = HashMap::new();
        Self {
            scope_chain,
            memory,
        }
    }

    pub fn descend_scope(&mut self) {
        self.scope_chain
            .push(RuntimeValue::Object(ObjectValue::default()));
    }

    pub fn ascend_scope(&mut self) {
        if self.scope_chain.len() <= 1 {
            panic!("Cannot ascend past global scope.")
        }
        self.scope_chain.pop();
    }

    fn memory_insert<T, F>(&mut self, value: T, referer: u64, wrap: F) -> u64
    where
        F: FnOnce(T) -> RuntimeValue,
    {
        let runtime_value = wrap(value);
        let id = runtime_value.id();
        let mem_ref = MemoryReference::new(runtime_value);
        self.memory.insert(id, mem_ref);
        id
    }

    pub fn memory_dump(&mut self, value: ExprNode, referer: u64) -> u64 {
        // dumps value in memory, returns head address
        match value {
            ExprNode::Int(v) => {
                self.memory_insert(v, referer, |x| RuntimeValue::Int(IntValue::from(x)))
            }
            ExprNode::Float(v) => {
                self.memory_insert(v, referer, |x| RuntimeValue::Float(FloatValue::from(x)))
            }
            ExprNode::Bool(v) => {
                self.memory_insert(v, referer, |x| RuntimeValue::Bool(BoolValue::from(x)))
            }
            ExprNode::String(v) => {
                self.memory_insert(v, referer, |x| RuntimeValue::String(StringValue::from(x)))
            }
            ExprNode::Object(obj) => {
                let runtime_obj_id = Counter.next();
                let dumped_object_properties: indexmap::IndexMap<RuntimeValue, u64> = obj
                    .properties
                    .into_iter()
                    .map(|obj_prop| {
                        (
                            expr_to_runtime(obj_prop.key),
                            self.memory_dump(obj_prop.value, runtime_obj_id),
                        )
                    })
                    .collect();
                let runtime_object = RuntimeValue::Object(ObjectValue {
                    properties: dumped_object_properties,
                    id: runtime_obj_id,
                });
                let mem_ref = MemoryReference::new(runtime_object);
                self.memory.insert(runtime_obj_id, mem_ref);
                runtime_obj_id
            }
            ExprNode::List(list) => {
                let runtime_list_id = Counter.next();
                let dumped_list_elements: Vec<u64> = list
                    .elements
                    .into_iter()
                    .map(|list_value| self.memory_dump(list_value, runtime_list_id))
                    .collect();
                let runtime_list = RuntimeValue::List(ListValue {
                    elements: dumped_list_elements,
                    id: runtime_list_id,
                });
                let mem_ref = MemoryReference::new(runtime_list);
                self.memory.insert(runtime_list_id, mem_ref);
                runtime_list_id
            }
            ExprNode::Reference(reference) => todo!(),
            ExprNode::Identity(_) => panic!("Cannot memory dump an identifier."),
            _ => unimplemented!(),
        }
    }

    pub fn memory_drop(&mut self, addr: u64) {
        if let Some(mem_ref) = self.memory.remove(&addr) {
            match &mem_ref.value {
                RuntimeValue::Object(obj) => {
                    obj.properties
                        .iter()
                        .for_each(|(_, v)| self.decrement_refs_if_exists(&v));
                }
                RuntimeValue::List(list) => {
                    list.elements
                        .iter()
                        .for_each(|element| self.decrement_refs_if_exists(&element));
                }
                _ => {}
            }
        }
    }

    fn get_memory_address_from_object<'a>(
        &'a self,
        mut container: Option<&'a RuntimeValue>,
        address: &[RuntimeValue],
    ) -> Result<Option<u64>, AccessError> {
        let mut mem_addr = None;
        for part in address.iter() {
            mem_addr = access_property(container, part)?;
            let memory_reference = match mem_addr {
                Some(mem_addr) => self.memory.get(&mem_addr),
                None => return Err(AccessError::InvalidAddress),
            };
            container = match memory_reference {
                Some(mem_ref) => Some(&mem_ref.value),
                None => None,
            };
        }
        Ok(mem_addr)
    }

    fn get_memory_address(&self, address: &[RuntimeValue]) -> Result<Option<u64>, AccessError> {
        let scope = self.scope_chain.iter().rev().find(|scope| match scope {
            RuntimeValue::Object(obj) => obj.properties.contains_key(&address[0]),
            _ => false,
        });
        match scope {
            Some(RuntimeValue::Object(_)) => self.get_memory_address_from_object(scope, address),
            None => Err(AccessError::InvalidAddress),
            _ => unreachable!(),
        }
    }

    fn increment_refs_if_exists(&mut self, mem_addr: &u64) {
        if let Some(mem_ref) = self.memory.get_mut(mem_addr) {
            mem_ref.referrers += 1;
        }
    }

    pub fn decrement_refs_if_exists(&mut self, mem_addr: &u64) {
        if let Some(mem_ref) = self.memory.get_mut(mem_addr) {
            mem_ref.referrers -= 1;
            if mem_ref.is_unreachable() {
                self.memory.remove(mem_addr);
            }
        }
    }

    pub fn get_value(
        &self,
        address: &[RuntimeValue],
    ) -> Result<Option<&MemoryReference>, AccessError> {
        let mem_addr = self.get_memory_address(address)?;
        match mem_addr {
            Some(mem) => Ok(self.memory.get(&mem)),
            None => Ok(None),
        }
    }

    pub fn get_value_by_addr(&self, addr: &u64) -> Option<&MemoryReference> {
        self.memory.get(&addr)
    }

    pub fn assign_value_to_identity(
        &mut self,
        identity: &IdentityValue,
        value: &RuntimeValue,
    ) -> Result<Option<u64>, AccessError> {
        // returns old value's memory address (if existed)
        let var_name = identity.address.last().unwrap();
        let old_mem_addr: Option<u64>;
        let address_parent: &mut RuntimeValue = if identity.address.len() == 1 {
            old_mem_addr = self
                .get_memory_address_from_object(self.scope_chain.last(), &identity.address)
                .unwrap_or_else(|e| None);
            self.scope_chain.last_mut().unwrap()
        } else {
            let mem_addr = self
                .get_memory_address_from_object(
                    self.scope_chain.last(),
                    &identity.address[..identity.address.len() - 1],
                )?
                .ok_or(AccessError::InvalidAddress)?;
            let parent_ref = self
                .memory
                .get_mut(&mem_addr)
                .ok_or(AccessError::InvalidAddress)?;
            old_mem_addr = access_property(Some(&parent_ref.value), var_name)?;
            &mut parent_ref.value
        };
        let new_mem_ref = MemoryReference::new(value.clone());
        set_property(address_parent, var_name, &identity.id())?;
        self.memory.insert(identity.id(), new_mem_ref);
        if let RuntimeValue::Reference(ref_value) = value {
            self.increment_refs_if_exists(&ref_value.memory_address);
        }
        Ok(old_mem_addr)
    }

    pub fn tt(&self) -> &HashMap<u64, MemoryReference> {
        &self.memory
    }

    pub fn sc(&mut self) -> &RuntimeValue {
        self.scope_chain.last().unwrap()
    }
}

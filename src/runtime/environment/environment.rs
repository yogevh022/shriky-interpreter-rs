use std::collections::{HashMap, HashSet};
use std::process::id;
use crate::runtime::environment::utils::Counter;
use crate::runtime::values::{RuntimeValue, ObjectValue, ReferenceValue, StringValue, ListValue, IntValue, IdentityValue, HasId};
use crate::runtime::values::error::AccessError;

#[derive(Debug)]
pub struct MemoryReference {
    pub value: RuntimeValue,
    pub referenced_by: HashSet<u64>
}

impl MemoryReference {
    pub fn new(value: RuntimeValue, initial_reference: u64) -> MemoryReference {
        MemoryReference {
            value,
            referenced_by: HashSet::from([initial_reference]),
        }
    }
    
    pub fn add_referer(&mut self, referer_addr: u64) {
        self.referenced_by.insert(referer_addr);
    }
    
    pub fn remove_referer(&mut self, referer_addr: u64) {
        self.referenced_by.remove(&referer_addr);
    }
}


pub struct Environment {
    pub scope_chain: Vec<RuntimeValue>,
    memory: HashMap<u64, MemoryReference>,
}

pub fn access_property<'a>(container: Option<&'a RuntimeValue>, member: &RuntimeValue) -> Result<Option<&'a u64>, AccessError> {
    match container {
        Some(RuntimeValue::Object(obj)) => obj.get_property(member),
        Some(RuntimeValue::List(list)) => list.get_element(member),
        _ => Err(AccessError::InvalidAccess)
    }
}


pub fn set_property(container: &mut RuntimeValue, key: &RuntimeValue, value_addr: &u64) -> Result<(), AccessError> {
    match container {
        RuntimeValue::Object(obj) => Ok(obj.set_property(key, *value_addr)),
        RuntimeValue::List(list) => Ok(list.set_element(key, *value_addr)?),
        _ => Err(AccessError::InvalidAccess)
    }
}

impl Environment {
    pub fn new() -> Environment {
        let scope_chain = vec![RuntimeValue::Object(ObjectValue::default())];
        let mut memory = HashMap::new();
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

    fn access_memory_at(&self, key: &u64) -> Option<&MemoryReference> {
        self.memory.get(key)
    }

    fn get_memory_address_from_object<'a>(&'a self, mut container: Option<&'a RuntimeValue>, address: &[RuntimeValue]) -> Result<Option<&'a u64>, AccessError> {
        let mut mem_addr = None;
        for part in address.iter() {
            mem_addr = access_property(container, part)?;
            let memory_reference = match mem_addr {
                Some(mem_addr) => self.memory.get(mem_addr),
                None => return Err(AccessError::InvalidAddress),
            };
            container = match memory_reference {
                Some(mem_ref) => Some(&mem_ref.value),
                None => None,
            };
        }
        Ok(mem_addr)
    }
    
    pub fn get_memory_address(&self, address: &[RuntimeValue]) -> Result<Option<& u64>, AccessError> {
        let scope = self.scope_chain.iter().rev().find(|scope| match scope {
            RuntimeValue::Object(obj) => { obj.properties.contains_key(&address[0]) },
            _ => false
        });
        match scope {
            Some(RuntimeValue::Object(_)) => self.get_memory_address_from_object(scope, address),
            None => Err(AccessError::InvalidAddress),
            _ => unreachable!()
        }
    }
    
    fn set_memory_value(&mut self, mem_addr: &u64, value: &RuntimeValue) -> Result<(), AccessError> {
        match self.memory.get_mut(mem_addr) {
            Some(mem_ref) => {
                mem_ref.value = value.clone();
                if let RuntimeValue::Reference(ref_val) = value {
                    self.memory.get_mut(&ref_val.memory_address).unwrap().add_referer(*mem_addr);
                }
                Ok(())
            },
            None => {
                let mem_ref = MemoryReference {
                    value: value.clone(),
                    referenced_by: HashSet::from([*mem_addr])
                };
                self.memory.insert(*mem_addr, mem_ref);
                Ok(())
            }
        }
    }
    
    pub fn get_value(&self, address: &[RuntimeValue]) -> Result<Option<&MemoryReference>, AccessError> {
        let mem_addr = self.get_memory_address(address)?;
        match mem_addr {
            Some(mem) => Ok(self.memory.get(mem)),
            None => Ok(None)
        }
    }

    pub fn tt(&self) -> &HashMap<u64, MemoryReference> {
        &self.memory
    }
    
    fn drop_ref_if_referring(&mut self, id: u64, mem_addr: Option<&u64>) {
        if let Some(mem_addr) = mem_addr {
            if let Some(mut mem_ref) = self.memory.get_mut(mem_addr) {
                mem_ref.remove_referer(id);
            }
        }
    }
    
    pub fn test(&mut self, identity: &IdentityValue, value: &RuntimeValue) -> Result<(), AccessError> {
        let mut address_parent: &mut RuntimeValue;
        if identity.address.len() == 1 {
            let mem_addr = self.get_memory_address_from_object(self.scope_chain.last(), &identity.address)?;
            let Some(m) = mem_addr;
            self.drop_ref_if_referring(identity.id(), Some(m));
            address_parent = self.scope_chain.last_mut().unwrap();
        } else {
            let parent_mem_addr = self.get_memory_address_from_object(self.scope_chain.last(), &identity.address[..identity.address.len()-1])?;
            
            // let mem_addr = if let Some(mem_addr) = parent_mem_addr
            //     .and_then(|addr| self.memory.get_mut(addr)) {
            //     Some(mem_addr)
                // address_parent = &mut mem_addr.value;
                // if let Some(mem_addr) = access_property(Some(&mem_addr.value), identity.address.last().unwrap()).ok() {
                //     self.drop_ref_if_referring(identity.id(), mem_addr);
                // }
            // } else { None };
            // else { unreachable!() }
        }
        let new_mem_ref = MemoryReference::new(value.clone(), identity.id());
        set_property(address_parent, identity.address.last().unwrap(), &identity.id())?;
        self.memory.insert(identity.id(), new_mem_ref);
        Ok(())
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
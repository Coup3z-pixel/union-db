use crate::storage::model::set::Set;
use std::collections::HashMap;

pub struct Store {
    sets: HashMap<String, Set>
}

impl Store {
    pub fn add_set(&mut self, set: Set) {
        self.sets.insert(set.get_set_name().to_string(), set);
    }

    pub fn new() -> Self {
        Store { sets: HashMap::new() }
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Set> {
        self.sets.get_mut(name)
    }

    pub fn get(&self, name: &str) -> Option<&Set> {
        self.sets.get(name)
    }

    pub fn create(&mut self, name: &str) -> &mut Set {
        self.sets.entry(name.to_string()).or_insert(Set::new(name))
    }

    pub fn list(&self) -> Vec<&String> {
        self.sets.keys().collect()
    }
}

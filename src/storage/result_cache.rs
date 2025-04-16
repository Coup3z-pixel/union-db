use std::collections::HashMap;

use super::model::set::Set;

pub struct ResultCache {
    cache: HashMap<String, Set>
}

impl ResultCache {
    pub fn new(&self) -> Self {
        Self {
            cache: HashMap::new()
        }
    }

    pub fn get_set(&self, set_name: String) -> Option<&Set> {
        return self.cache.get(&set_name);
    }

    pub fn create_set(&mut self, new_set: Set) -> () {
        let set_name = new_set.set_name.clone();
        self.cache.insert(set_name, new_set);
    }
}

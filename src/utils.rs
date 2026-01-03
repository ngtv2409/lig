use std::collections::HashMap;
use std::hash::Hash;
/*
    An ordered hashmap with manual handling

    Used to preserve the order of pattern
*/
#[derive(Debug)]
pub struct OrdHashMap<K, V> {
    pub ord: Vec<K>,
    pub map: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V> OrdHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            ord: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if !self.map.contains_key(&key) {
            self.ord.push(key.clone());
        }

        self.map.insert(key, val)
    }
}

use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

// Yritin mutta en onnistunut

const DEFAULT_SIZE: usize = 255;

pub struct HashMap<K: Hash + PartialEq, V> {
    inner: Vec<HashMapEntry<K,V>>,
}
impl <K: Hash + PartialEq, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            inner: Vec::with_capacity(DEFAULT_SIZE),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish() as usize % DEFAULT_SIZE;
        
        let first_entry = self.inner.get_mut(hash_value);

    }

    pub fn get(self, key: K) -> Option<V> {
        None
    }
}

struct HashMapEntry<K: PartialEq, V> {
    key: K,
    value: V,
    next: Option<Box<HashMapEntry<K, V>>>,
}

impl <K: PartialEq, V> HashMapEntry<K, V> {
    fn new(key: K, value: V) -> Self {
        HashMapEntry {
            key,
            value,
            next: None,
        }
    }

}

impl <K: PartialEq, V> PartialEq for HashMapEntry<K, V>{
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }

}
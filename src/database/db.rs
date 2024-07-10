use std::sync::Arc;
use dashmap::DashMap;
use dashmap::mapref::multiple::RefMulti;
use dashmap::mapref::one::Ref;

pub struct Storage<K, V> {
    map: Arc<DashMap<K, V>>,
}

impl<K, V> Storage<K, V>
where
    K: Eq + std::hash::Hash + Clone + Copy,
    V: Clone,
{
    pub fn new() -> Self {
        Storage {
            map: Arc::new(DashMap::new()),
        }
    }

    pub fn set(&self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.map.get(key).map(|entry: Ref<K, V>| entry.value().clone())
    }

    pub fn get_all(&self) -> Vec<(K, V)> {
        self.map.iter().map(|entry: RefMulti<K, V> | (entry.key().clone(), entry.value().clone())).collect()
    }

    pub fn remove(&self, key: K) {
        self.map.remove(&key);
    }
}
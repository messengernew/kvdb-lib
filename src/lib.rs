use std::sync::Arc;
use dashmap::DashMap;
use dashmap::mapref::multiple::RefMulti;
use dashmap::mapref::one::Ref;

/// A thread-safe key-value storage using DashMap.
///
/// # Examples
///
///

/// use kvdb_lib::Storage;
///
/// let storage = Storage::new();
/// storage.set(1, "value1");
///
/// assert_eq!(storage.get(&1), Some("value1"));
///

pub struct Storage<K, V> {
    map: Arc<DashMap<K, V>>,
}

impl<K, V> Storage<K, V>
where
    K: Eq + std::hash::Hash + Clone + Copy,
    V: Clone + Copy,
{
    /// Creates a new, empty `Storage`.
    ///
    /// # Examples
    ///
    ///

    /// use kvdb_lib::Storage;
    ///
    /// let storage: Storage<i32, &str> = Storage::new();
    ///

    pub fn new() -> Self {
        Storage {
            map: Arc::new(DashMap::new()),
        }
    }

    /// Inserts a key-value pair into the storage.
    ///
    /// If the storage did not have this key present, `None` is returned.
    /// If the storage did have this key present, the value is updated, and the old value is returned.
    ///
    /// # Examples
    ///
    ///

    /// use kvdb_lib::Storage;
    ///
    /// let storage = Storage::new();
    /// storage.set(1, "value1");
    ///
    /// assert_eq!(storage.get(&1), Some("value1"));
    ///

    pub fn set(&self, key: K, value: V) {
        self.map.insert(key, value);
    }

    /// Retrieves a value from the storage, given a key.
    ///
    /// # Examples
    ///
    ///

    /// use kvdb_lib::Storage;
    ///
    /// let storage = Storage::new();
    /// storage.set(1, "value1");
    ///
    /// assert_eq!(storage.get(&1), Some("value1"));
    ///

    pub fn get(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|entry: Ref<K, V>| entry.value().clone())
    }

    /// Retrieves all key-value pairs from the storage as a vector of tuples.
    ///
    /// # Examples
    ///
    ///

    /// use kvdb_lib::Storage;
    ///
    /// let storage = Storage::new();
    ///
    /// storage.set(1, "value1");
    /// storage.set(2, "value2");
    /// let all = storage.get_all();
    ///
    /// assert_eq!(all.len(), 2);
    /// assert!(all.contains(&(1, "value1")));
    /// assert!(all.contains(&(2, "value2")));
    ///

    pub fn get_all(&self) -> Vec<(K, V)> {
        self.map.iter()
            .map(|entry: RefMulti<K, V>| (entry.key().clone(), entry.value().clone()))
            .collect()
    }

    /// Removes a key-value pair from the storage.
    ///
    /// # Examples
    ///
    ///

    /// use kvdb_lib::Storage;
    ///
    /// let storage = Storage::new();
    ///
    /// storage.set(1, "value1");
    /// storage.remove(1);
    ///
    /// assert_eq!(storage.get(&1), None);
    ///

    pub fn remove(&self, key: K) {
        self.map.remove(&key);
    }
}
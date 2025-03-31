use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct Dict<K, V> {
    data: Vec<Option<(K, V)>>,
    len: usize,
}

impl<K, V> Dict<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone
{
    /// Create a new hashmap with the given fixed capacity.
    pub fn new(capacity: usize) -> Self {
        Dict {
            data: vec![None; capacity],
            len: 0,
        }
    }

    /// Compute index for key.
    fn hash_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return (hasher.finish() as usize) % self.data.len();
    }

    /// When two unique hashers produce the same hash value.
    fn hash_collision(&self, idx: usize) -> usize {
        return (idx + 1) % self.data.len();
    }


    /// Insert key-value pair into hashmap.
    pub fn insert(&mut self, key: K, value: V) {
        let mut idx = self.hash_index(&key);

        loop {
            match &mut self.data[idx] {
                // empty slot => insert new key-value
                slot @ None => {
                    *slot = Some((key, value));
                    self.len += 1;
                    return;
                }

                // key exists => update the value
                Some((hash_key, existing_value)) if *hash_key == key => {
                    *existing_value = value;
                    return;
                }

                // keep searching, avg. case should be: O(1), absolute worst case: O(n)
                _ => {
                    idx = self.hash_collision(idx);
                }
            }
        }
    }


    pub fn remove(&mut self, key: K) {
        let mut idx = self.hash_index(&key);

        loop {

            match &mut self.data[idx] {

                None => {
                    println!("Key does not exist");
                    return;
                }

                Some((hash_key, _)) if *hash_key == key => {
                    self.data[idx] = None;
                    self.len -= 1;
                    return;
                }

                _ => {
                    idx = self.hash_collision(idx);
                }
            }
        }
    }

    /// Get an immutable reference to the hash value given its key.
    pub fn get(&self, key: K) -> Option<&V> {
        let mut idx = self.hash_index(&key);

        loop {
            match &self.data[idx] {

                // key match => return value
                Some((hash_key, value)) if hash_key == &key => {
                    return Some(value)
                }

                // empty => the key does not exist
                None => {
                    return None
                }

                // keep searching, avg. case should be: O(1), absolute worst case: O(n)
                _ => {
                    idx = self.hash_collision(idx);
                }
            }
        }
    }


    /// Length of hash
    pub fn len(&self) -> usize {
        return self.len;
    }
}


impl<K, V> Drop for Dict<K, V> {
    fn drop(&mut self) {
        println!("Dictionary dropped!");
    }
}

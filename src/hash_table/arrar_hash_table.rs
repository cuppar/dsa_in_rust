use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Pair<V: Clone + Debug> {
    pub key: usize,
    pub val: V,
}

pub struct ArrayHashMap<V: Clone + Debug> {
    buckets: Vec<Option<Pair<V>>>,
}

const BUCKETS_SIZE: usize = 100;

impl<V: Clone + Debug> ArrayHashMap<V> {
    pub fn new() -> Self {
        Self {
            buckets: vec![None; BUCKETS_SIZE],
        }
    }

    fn hash_func(&self, key: usize) -> usize {
        key % BUCKETS_SIZE
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        let idx = self.hash_func(key);
        self.buckets[idx].as_ref().map(|p| &p.val)
    }

    pub fn put(&mut self, key: usize, val: V) {
        let idx = self.hash_func(key);
        self.buckets[idx] = Some(Pair { key, val });
    }

    pub fn remove(&mut self, key: usize) {
        let idx = self.hash_func(key);
        self.buckets[idx].take();
    }

    pub fn entry_set(&self) -> Vec<&Pair<V>> {
        self.buckets.iter().filter_map(|p| p.as_ref()).collect()
    }

    pub fn key_set(&self) -> Vec<usize> {
        self.buckets
            .iter()
            .filter_map(|p| p.as_ref().map(|p| p.key))
            .collect()
    }

    pub fn value_set(&self) -> Vec<&V> {
        self.buckets
            .iter()
            .filter_map(|p| p.as_ref().map(|p| &p.val))
            .collect()
    }

    pub fn print(&self) {
        for p in self.entry_set() {
            println!("{} -> {:?}", p.key, p.val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = ArrayHashMap::<String>::new();

        map.put(1, "foo".to_string());
        map.put(2, "bar".to_string());
        map.put(3, "baz".to_string());

        map.print();

        assert_eq!(map.get(0), None);
        assert_eq!(map.get(1), Some(&"foo".to_string()));
        assert_eq!(map.get(2), Some(&"bar".to_string()));
        assert_eq!(map.get(3), Some(&"baz".to_string()));

        assert_eq!(map.get(BUCKETS_SIZE + 1), Some(&"foo".to_string()));

        map.remove(1);

        assert_eq!(map.get(1), None);
        assert_eq!(map.get(BUCKETS_SIZE + 1), None);
    }
}

use std::fmt::Debug;

/* 键值对 */
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<V: Clone + Debug> {
    pub key: usize,
    pub val: V,
}

/* 基于数组实现的哈希表 */
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

    /* 哈希函数 */
    fn hash_func(&self, key: usize) -> usize {
        key % BUCKETS_SIZE
    }

    /* 查询操作 */
    pub fn get(&self, key: usize) -> Option<&V> {
        let idx = self.hash_func(key);
        self.buckets[idx].as_ref().map(|p| &p.val)
    }

    /* 添加操作 */
    pub fn put(&mut self, key: usize, val: V) {
        let idx = self.hash_func(key);
        self.buckets[idx] = Some(Pair { key, val });
    }

    /* 删除操作 */
    pub fn remove(&mut self, key: usize) {
        let idx = self.hash_func(key);
        self.buckets[idx].take();
    }

    /* 获取所有键值对 */
    pub fn entry_set(&self) -> Vec<&Pair<V>> {
        self.buckets.iter().filter_map(|p| p.as_ref()).collect()
    }

    /* 获取所有键 */
    pub fn key_set(&self) -> Vec<usize> {
        self.buckets
            .iter()
            .filter_map(|p| p.as_ref().map(|p| p.key))
            .collect()
    }

    /* 获取所有值 */
    pub fn value_set(&self) -> Vec<&V> {
        self.buckets
            .iter()
            .filter_map(|p| p.as_ref().map(|p| &p.val))
            .collect()
    }

    /* 打印哈希表 */
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

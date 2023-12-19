use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Pair<V: Clone + Debug> {
    pub key: usize,
    pub val: V,
}

const BUCKETS_SIZE: usize = 10;

pub struct ChainHashMap<V: Clone + Debug> {
    size: usize,
    capacity: usize,
    load_thres: f32,
    extend_ratio: usize,
    buckets: Vec<Vec<Pair<V>>>,
}

impl<V: Clone + Debug> ChainHashMap<V> {
    pub fn new() -> Self {
        Self {
            size: 0,
            capacity: BUCKETS_SIZE,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![vec![]; BUCKETS_SIZE],
        }
    }

    fn hash_func(&self, key: usize) -> usize {
        key % self.capacity
    }

    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        let idx = self.hash_func(key);
        let bucket = &self.buckets[idx];
        bucket
            .iter()
            .find_map(|p| if p.key == key { Some(&p.val) } else { None })
    }

    pub fn put(&mut self, key: usize, val: V) {
        if self.load_factor() > self.load_thres {
            self.extend();
        }

        let idx = self.hash_func(key);
        let bucket = &mut self.buckets[idx];

        if let Some(old_val) =
            bucket
                .iter_mut()
                .find_map(|p| if p.key == key { Some(&mut p.val) } else { None })
        {
            *old_val = val;
        } else {
            bucket.push(Pair { key, val });
            self.size += 1;
        }
    }

    pub fn remove(&mut self, key: usize) {
        let idx = self.hash_func(key);
        let bucket = &mut self.buckets[idx];
        if let Some((rm_i, _)) = bucket.iter_mut().enumerate().find(|(_, p)| p.key == key) {
            bucket.remove(rm_i);
            self.size -= 1;
        }
    }

    fn extend(&mut self) {
        // save old pairs
        let buckets_temp = std::mem::replace(&mut self.buckets, vec![]);

        // create new buckets
        self.capacity *= self.extend_ratio;
        self.buckets = vec![vec![]; self.capacity];
        self.size = 0;

        // rehash and put old pairs
        for bucket in buckets_temp {
            for p in bucket {
                self.put(p.key, p.val);
            }
        }
    }

    pub fn entry_set(&self) -> Vec<&Pair<V>> {
        self.buckets.iter().flat_map(|b| b.iter()).collect()
    }

    pub fn key_set(&self) -> Vec<usize> {
        self.buckets
            .iter()
            .flat_map(|b| b.iter().map(|p| p.key))
            .collect()
    }

    pub fn value_set(&self) -> Vec<&V> {
        self.buckets
            .iter()
            .flat_map(|b| b.iter().map(|p| &p.val))
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
        let mut map = ChainHashMap::<usize>::new();

        assert_eq!(map.size, 0);

        for i in 0..3 * BUCKETS_SIZE {
            map.put(i, i);
        }

        assert_eq!(map.size, 3 * BUCKETS_SIZE);
        assert!(map.load_factor() <= map.load_thres);
        println!("load_factor: {}", map.load_factor());
        println!("load_thres: {}", map.load_thres);
        println!("capacity: {}", map.capacity);

        for i in 0..3 * BUCKETS_SIZE {
            assert_eq!(map.get(i), Some(&i));
        }

        map.put(1, 99);
        assert_eq!(map.get(1), Some(&99));
        map.put(1, 1);

        map.remove(3);
        map.remove(13);
        map.remove(15);

        assert_eq!(map.size, 3 * BUCKETS_SIZE - 3);

        assert_eq!(map.get(3), None);
        assert_eq!(map.get(13), None);
        assert_eq!(map.get(15), None);

        for i in (0..3 * BUCKETS_SIZE).filter(|i| ![3, 13, 15].contains(&i)) {
            assert_eq!(map.get(i), Some(&i));
        }

        map.print();
    }
}

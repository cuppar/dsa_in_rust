use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Pair<V: Clone + Debug> {
    pub key: usize,
    pub val: V,
}

#[derive(Debug, Clone)]
enum TombstoneOrAlive<T> {
    Alive(T),
    Tombstone,
}

const BUCKETS_SIZE: usize = 10;

pub struct OpenAddrHashMap<V: Clone + Debug> {
    size: usize,
    capacity: usize,
    load_thres: f32,
    extend_ratio: usize,
    buckets: Vec<Option<TombstoneOrAlive<Pair<V>>>>,
}

impl<V: Clone + Debug> OpenAddrHashMap<V> {
    pub fn new() -> Self {
        Self {
            size: 0,
            capacity: BUCKETS_SIZE,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![None; BUCKETS_SIZE],
        }
    }

    fn hash_func(&self, key: usize) -> usize {
        key % self.capacity
    }

    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    fn find_bucket(&mut self, key: usize) -> usize {
        let mut idx = self.hash_func(key);

        use TombstoneOrAlive::*;

        let mut first_tombstone = 0;
        let mut meet_tombstone = false;
        loop {
            match &self.buckets[idx] {
                Some(Alive(p)) => {
                    if p.key == key {
                        if meet_tombstone {
                            self.buckets[first_tombstone] = Some(Alive(p.clone()));
                            self.buckets[idx] = Some(Tombstone);
                            return first_tombstone;
                        }
                        return idx;
                    } else {
                        idx += 1;
                    }
                }
                Some(Tombstone) => {
                    if meet_tombstone == false {
                        meet_tombstone = true;
                        first_tombstone = idx;
                    }
                    idx += 1;
                }
                None => {
                    if meet_tombstone {
                        return first_tombstone;
                    }
                    return idx;
                }
            }
        }
    }

    pub fn get(&mut self, key: usize) -> Option<&V> {
        let idx = self.find_bucket(key);
        self.buckets[idx].as_ref().and_then(|p| {
            if let TombstoneOrAlive::Alive(p) = p {
                return Some(&p.val);
            } else {
                return None;
            }
        })
    }

    pub fn put(&mut self, key: usize, val: V) {
        if self.load_factor() > self.load_thres {
            self.extend();
        }

        use TombstoneOrAlive::*;
        let idx = self.find_bucket(key);
        match &mut self.buckets[idx] {
            Some(Alive(p)) => {
                p.val = val;
            }
            None | Some(Tombstone) => {
                self.buckets[idx] = Some(Alive(Pair { key, val }));
                self.size += 1;
            }
        }
    }

    pub fn remove(&mut self, key: usize) {
        use TombstoneOrAlive::*;
        let idx = self.find_bucket(key);
        if let Some(Alive(_)) = &mut self.buckets[idx] {
            self.buckets[idx] = Some(Tombstone);
            self.size -= 1;
        }
    }

    fn extend(&mut self) {
        // save old pairs
        let buckets_temp = std::mem::replace(&mut self.buckets, vec![]);

        // create new buckets
        self.capacity *= self.extend_ratio;
        self.buckets = vec![None; self.capacity];
        self.size = 0;

        // rehash and put old pairs
        for p in buckets_temp {
            if let Some(TombstoneOrAlive::Alive(p)) = p {
                self.put(p.key, p.val);
            }
        }
    }

    pub fn entry_set(&self) -> Vec<&Pair<V>> {
        use TombstoneOrAlive::*;
        self.buckets
            .iter()
            .filter_map(|b| {
                b.as_ref()
                    .and_then(|t| if let Alive(p) = t { Some(p) } else { None })
            })
            .collect()
    }

    pub fn key_set(&self) -> Vec<usize> {
        self.entry_set().iter().map(|e| e.key).collect()
    }

    pub fn value_set(&self) -> Vec<&V> {
        self.entry_set().iter().map(|e| &e.val).collect()
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
        let mut map = OpenAddrHashMap::<usize>::new();

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

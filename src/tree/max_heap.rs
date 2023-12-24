use std::{fmt::Debug, mem::swap};

pub struct MaxHeap<T: Clone + Debug + Ord> {
    max_heap: Vec<T>,
}
impl<T: Clone + Debug + Ord> MaxHeap<T> {
    pub fn new() -> Self {
        Self { max_heap: vec![] }
    }

    pub fn from_vec_1(v: Vec<T>) -> Self {
        let mut heap = Self::new();
        for el in v {
            heap.push(el);
        }
        heap
    }

    pub fn from_vec_2(v: Vec<T>) -> Self {
        let mut heap = Self { max_heap: v };

        if heap.size() < 2 {
            return heap;
        }

        for i in (0..=Self::parent(heap.size() - 1).unwrap()).rev() {
            heap.sift_down(i);
        }

        heap
    }

    fn parent(i: usize) -> Option<usize> {
        if i == 0 {
            None
        } else {
            Some((i - 1) / 2)
        }
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }
    fn right(i: usize) -> usize {
        2 * i + 2
    }

    pub fn size(&self) -> usize {
        self.max_heap.len()
    }
    pub fn is_empty(&self) -> bool {
        self.max_heap.is_empty()
    }

    pub fn push(&mut self, val: T) {
        self.max_heap.push(val);
        self.sift_up(self.size() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.swap(0, self.size() - 1);

        let val = self.max_heap.remove(self.size() - 1);

        self.sift_down(0);

        Some(val)
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.max_heap.first()
    }

    fn sift_up(&mut self, mut i: usize) {
        loop {
            if i == 0 {
                break;
            }

            let parent = Self::parent(i).unwrap();
            if self.max_heap[parent] >= self.max_heap[i] {
                break;
            }

            self.swap(i, parent);
            i = parent;
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        loop {
            if i >= self.size() {
                break;
            }

            let (l, r, mut max) = (Self::left(i), Self::right(i), i);

            if l < self.size() && self.max_heap[l] > self.max_heap[max] {
                max = l;
            }
            if r < self.size() && self.max_heap[r] > self.max_heap[max] {
                max = r;
            }

            if max == i {
                break;
            }

            self.swap(max, i);

            i = max;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        let max = a.max(b);
        let min = a.min(b);
        let (a, b) = self.max_heap.split_at_mut(max);
        swap(&mut a[min], &mut b[0]);
    }

    pub fn print(&self) {
        self._print(0, 0, "Root: ");
    }

    pub fn _print(&self, i: usize, level: usize, prefix: &str) {
        for _ in 0..level {
            print!("  ");
        }
        if i < self.size() {
            println!("{}{:?}", prefix, self.max_heap[i]);
            let new_level = level + 1;
            self._print(
                Self::left(i),
                new_level,
                format!("L{}: ", new_level).as_str(),
            );
            self._print(
                Self::right(i),
                new_level,
                format!("R{}: ", new_level).as_str(),
            );
        } else {
            println!("{}<None>", prefix);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vec_1() {
        let mut heap = MaxHeap::from_vec_1(vec![3, 1, 4, 2, 5]);
        heap.push(9);
        heap.print();

        assert_eq!(heap.peek(), Some(&9));
        assert_eq!(heap.pop(), Some(9));
        heap.print();

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));
        heap.print();

        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        heap.print();

        assert_eq!(heap.pop(), None);
        heap.print();
    }

    #[test]
    fn from_vec_2() {
        let mut heap = MaxHeap::from_vec_2(vec![3, 1, 4, 2, 5]);
        heap.push(9);
        heap.print();

        assert_eq!(heap.peek(), Some(&9));
        assert_eq!(heap.pop(), Some(9));
        heap.print();

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));
        heap.print();

        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        heap.print();

        assert_eq!(heap.pop(), None);
        heap.print();
    }
}

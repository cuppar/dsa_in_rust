use std::{cell::RefCell, rc::Rc};

pub struct ArrayQueue<T> {
    array: Vec<T>,
    front: usize,
    size: usize,
    capacity: usize,
}

impl<T: Default + Clone> ArrayQueue<T> {
    pub fn new(capacity: usize) -> Self {
        ArrayQueue {
            array: vec![Default::default(); capacity],
            front: 0,
            size: 0,
            capacity,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.size == self.capacity {
            println!("Queue fulled.");
            return;
        }

        let rear = (self.front + self.size) % self.capacity;
        self.array[rear] = val;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let result = self.peek().cloned();
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            result
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array[self.front])
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut v = vec![Default::default(); self.size()];
        let mut j = self.front;
        let cap = self.capacity();

        for i in 0..self.size() {
            v[i] = self.array[j % cap].clone();
            j += 1;
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut queue = ArrayQueue::new(10);
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.size(), 3);
        assert!(!queue.is_empty());

        assert_eq!(queue.to_vec(), [1,2,3]);

        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.peek(), Some(&2));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.peek(), Some(&3));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.pop(), None);

        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());
    }
}

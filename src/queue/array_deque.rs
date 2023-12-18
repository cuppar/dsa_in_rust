pub struct ArrayDeque<T> {
    array: Vec<T>,
    front: usize,
    size: usize,
    capacity: usize,
}

impl<T: Default + Clone> ArrayDeque<T> {
    pub fn new(capacity: usize) -> Self {
        ArrayDeque {
            array: vec![Default::default(); capacity],
            front: 0,
            size: 0,
            capacity,
        }
    }

    pub fn push_front(&mut self, val: T) {
        if self.is_full() {
            println!("Queue fulled.");
            return;
        }

        // first plus capacity to prevent unsign integar overflow
        let front = (self.front + self.capacity - 1) % self.capacity;
        self.array[front] = val;
        self.front = front;
        self.size += 1;
    }

    pub fn push_rear(&mut self, val: T) {
        if self.is_full() {
            println!("Queue fulled.");
            return;
        }

        let rear = (self.front + self.size) % self.capacity;
        self.array[rear] = val;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let result = self.peek_front().cloned();
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
            result
        }
    }

    pub fn pop_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let result = self.peek_rear().cloned();
            self.size -= 1;
            result
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array[self.front])
        }
    }

    pub fn peek_rear(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array[(self.front + self.size - 1) % self.capacity])
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
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
    fn front() {
        let mut deque = ArrayDeque::new(10);
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(deque.size(), 3);
        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [3, 2, 1]);

        assert_eq!(deque.peek_front(), Some(&3));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.peek_front(), Some(&2));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.peek_front(), Some(&1));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_front(), None);

        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn rear() {
        let mut deque = ArrayDeque::new(10);
        deque.push_rear(1);
        deque.push_rear(2);
        deque.push_rear(3);

        assert_eq!(deque.size(), 3);
        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [1, 2, 3]);

        assert_eq!(deque.peek_rear(), Some(&3));
        assert_eq!(deque.pop_rear(), Some(3));
        assert_eq!(deque.peek_rear(), Some(&2));
        assert_eq!(deque.pop_rear(), Some(2));
        assert_eq!(deque.peek_rear(), Some(&1));
        assert_eq!(deque.pop_rear(), Some(1));
        assert_eq!(deque.pop_rear(), None);
        assert_eq!(deque.pop_rear(), None);

        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn all() {
        let mut deque = ArrayDeque::new(10);
        deque.push_front(3);
        deque.push_front(2);
        deque.push_front(1);
        deque.push_rear(4);
        deque.push_rear(5);
        deque.push_rear(6);

        assert_eq!(deque.size(), 6);
        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [1, 2, 3, 4, 5, 6]);

        assert_eq!(deque.peek_front(), Some(&1));
        assert_eq!(deque.peek_rear(), Some(&6));

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(4));

        assert_eq!(deque.size(), 2);
        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [5, 6]);

        assert_eq!(deque.peek_front(), Some(&5));
        assert_eq!(deque.peek_rear(), Some(&6));

        deque.push_rear(7);
        deque.push_rear(8);
        deque.push_front(4);
        deque.push_front(3);

        assert_eq!(deque.size(), 6);
        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [3, 4, 5, 6, 7, 8]);

        assert_eq!(deque.peek_front(), Some(&3));
        assert_eq!(deque.peek_rear(), Some(&8));

        assert_eq!(deque.pop_rear(), Some(8));
        assert_eq!(deque.pop_rear(), Some(7));
        assert_eq!(deque.pop_rear(), Some(6));
        assert_eq!(deque.pop_rear(), Some(5));
        assert_eq!(deque.pop_rear(), Some(4));
        assert_eq!(deque.pop_rear(), Some(3));

        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
    }
}

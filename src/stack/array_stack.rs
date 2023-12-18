pub struct ArrayStack<T> {
    array: Vec<T>,
    top: usize,
    capacity: usize,
}

impl<T: Default + Clone> ArrayStack<T> {
    pub fn new(capacity: usize) -> Self {
        ArrayStack {
            array: vec![Default::default(); capacity],
            top: 0,
            capacity,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.is_full() {
            println!("Stack fulled.");
            return;
        }

        self.array[self.top] = val;
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.top -= 1;
            Some(self.array[self.top].clone())
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.array[self.top - 1])
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.capacity()
    }

    pub fn size(&self) -> usize {
        self.top
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = vec![Default::default(); self.size()];
        for i in 0..self.size() {
            let j = self.size() - i - 1;
            result[i] = self.array[j].clone();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut stack = ArrayStack::new(10);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert!(!stack.is_empty());
        assert!(!stack.is_full());

        assert_eq!(stack.to_vec(), [3, 2, 1]);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.pop(), None);

        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }
}

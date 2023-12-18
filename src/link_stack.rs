pub struct LinkStack<T> {
    top: Option<Link<T>>,
    size: usize,
}

type Link<T> = Box<Node<T>>;

struct Node<T> {
    val: T,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Link<T> {
        Box::new(Node { val, next: None })
    }
}

impl<T: Clone> LinkStack<T> {
    pub fn new() -> Self {
        LinkStack { top: None, size: 0 }
    }

    pub fn push(&mut self, val: T) {
        let mut new_top = Node::new(val);
        if let Some(old_top) = self.top.take() {
            new_top.next = Some(old_top);
        }
        self.top = Some(new_top);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|top| {
            self.top = top.next;
            self.size -= 1;
            top.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|top| &top.val)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn to_vec(&self) -> Vec<T> {
        self._to_vec(self.top.as_ref())
    }

    fn _to_vec(&self, head: Option<&Link<T>>) -> Vec<T> {
        if let Some(head) = head {
            let mut rest = self._to_vec(head.next.as_ref());
            rest.insert(0, head.val.clone());
            return rest;
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut stack = LinkStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert!(!stack.is_empty());

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

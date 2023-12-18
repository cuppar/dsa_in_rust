use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
    vec,
};

#[derive(Debug)]
pub struct LinkDeque<T: Debug> {
    front: Option<Link<T>>,
    rear: Option<Link<T>>,
}

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T: Debug> {
    val: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T: Debug> Node<T> {
    fn new(val: T) -> Link<T> {
        Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: None,
        }))
    }
}

impl<T: Clone + Debug> LinkDeque<T> {
    pub fn new() -> Self {
        LinkDeque {
            front: None,
            rear: None,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_front = Node::new(val);
        if let Some(old_front) = self.front.take() {
            old_front.borrow_mut().prev = Some(Rc::clone(&new_front));
            new_front.borrow_mut().next = Some(old_front);
        } else {
            self.rear = Some(Rc::clone(&new_front));
        }
        self.front = Some(new_front);
    }

    pub fn push_rear(&mut self, val: T) {
        let new_rear = Node::new(val);
        if let Some(old_rear) = self.rear.take() {
            old_rear.borrow_mut().next = Some(Rc::clone(&new_rear));
            new_rear.borrow_mut().prev = Some(old_rear);
        } else {
            self.front = Some(Rc::clone(&new_rear));
        }
        self.rear = Some(new_rear);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            match old_front.borrow_mut().next.take() {
                Some(new_front) => {
                    new_front.borrow_mut().prev.take();
                    self.front = Some(new_front);
                }
                None => {
                    self.rear.take();
                }
            }
            Rc::try_unwrap(old_front)
                .ok()
                .unwrap()
                .into_inner()
                .val
                .clone() // workaround about debug with Drop impl for Node<T>
        })
    }

    pub fn pop_rear(&mut self) -> Option<T> {
        self.rear.take().map(|old_rear| {
            match old_rear.borrow_mut().prev.take() {
                Some(new_rear) => {
                    new_rear.borrow_mut().next.take();
                    self.rear = Some(new_rear);
                }
                None => {
                    self.front.take();
                }
            }
            Rc::try_unwrap(old_rear)
                .ok()
                .unwrap()
                .into_inner()
                .val
                .clone() // workaround about debug with Drop impl for Node<T>
        })
    }

    pub fn peek_front(&self) -> Option<RefMut<'_, T>> {
        self.front
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }

    pub fn peek_rear(&self) -> Option<RefMut<'_, T>> {
        self.rear
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_none() && self.rear.is_none()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self._to_vec(self.front.as_ref())
    }

    fn _to_vec(&self, head: Option<&Link<T>>) -> Vec<T> {
        if let Some(head) = head {
            let mut rest = self._to_vec(head.borrow().next.as_ref());
            rest.insert(0, head.borrow().val.clone());
            return rest;
        }
        vec![]
    }
}

impl<T: Debug> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Before drop node: {:?}", self.val);
        drop(self.prev.take());
        drop(self.next.take());
        println!("After drop node: {:?}", self.val);
    }
}

impl<T: Debug> Drop for LinkDeque<T> {
    fn drop(&mut self) {
        let mut cur_node = self.front.take();

        while let Some(node) = cur_node {
            if let Some(next_node) = node.borrow_mut().next.take() {
                next_node.borrow_mut().prev.take();
                cur_node = Some(next_node);
            } else {
                break;
            }
            // node go out of scope
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front() {
        let mut deque = LinkDeque::new();
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [3, 2, 1]);

        assert_eq!(*deque.peek_front().unwrap(), 3);
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(*deque.peek_front().unwrap(), 2);
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(*deque.peek_front().unwrap(), 1);
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_front(), None);

        assert!(deque.is_empty());
    }

    #[test]
    fn rear() {
        let mut deque = LinkDeque::new();
        deque.push_rear(1);
        deque.push_rear(2);
        deque.push_rear(3);

        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [1, 2, 3]);

        assert_eq!(*deque.peek_rear().unwrap(), 3);
        assert_eq!(deque.pop_rear(), Some(3));
        assert_eq!(*deque.peek_rear().unwrap(), 2);
        assert_eq!(deque.pop_rear(), Some(2));
        assert_eq!(*deque.peek_rear().unwrap(), 1);
        assert_eq!(deque.pop_rear(), Some(1));
        assert_eq!(deque.pop_rear(), None);
        assert_eq!(deque.pop_rear(), None);

        assert!(deque.is_empty());
    }

    #[test]
    fn all() {
        let mut deque = LinkDeque::new();
        deque.push_front(3);
        deque.push_front(2);
        deque.push_front(1);
        deque.push_rear(4);
        deque.push_rear(5);
        deque.push_rear(6);

        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [1, 2, 3, 4, 5, 6]);

        assert_eq!(*deque.peek_front().unwrap(), 1);
        assert_eq!(*deque.peek_rear().unwrap(), 6);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(4));

        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [5, 6]);

        assert_eq!(*deque.peek_front().unwrap(), 5);
        assert_eq!(*deque.peek_rear().unwrap(), 6);

        deque.push_rear(7);
        deque.push_rear(8);
        deque.push_front(4);
        deque.push_front(3);

        assert!(!deque.is_empty());

        assert_eq!(deque.to_vec(), [3, 4, 5, 6, 7, 8]);

        assert_eq!(*deque.peek_front().unwrap(), 3);
        assert_eq!(*deque.peek_rear().unwrap(), 8);

        assert_eq!(deque.pop_rear(), Some(8));
        assert_eq!(deque.pop_rear(), Some(7));
        assert_eq!(deque.pop_rear(), Some(6));
        assert_eq!(deque.pop_rear(), Some(5));
        assert_eq!(deque.pop_rear(), Some(4));
        assert_eq!(deque.pop_rear(), Some(3));

        assert!(deque.is_empty());
    }
}

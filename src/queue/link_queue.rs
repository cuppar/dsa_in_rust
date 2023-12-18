use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::Rc,
    vec,
};

#[derive(Debug)]
pub struct LinkQueue<T: Debug> {
    front: Option<Link<T>>,
    rear: Option<Link<T>>,
    size: usize,
}

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T: Debug> {
    val: T,
    next: Option<Link<T>>,
}

impl<T: Debug> Node<T> {
    fn new(val: T) -> Link<T> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

impl<T: Clone + Debug> LinkQueue<T> {
    pub fn new() -> Self {
        LinkQueue {
            front: None,
            rear: None,
            size: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        let new_rear = Node::new(val);
        if let Some(old_rear) = self.rear.take() {
            old_rear.borrow_mut().next = Some(new_rear.clone());
            self.rear = Some(new_rear);
        } else {
            self.front = Some(new_rear.clone());
            self.rear = Some(new_rear);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            match old_front.borrow_mut().next.take() {
                Some(new_front) => {
                    self.front = Some(new_front);
                }
                None => {
                    self.rear = None;
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_front)
                .ok()
                .unwrap()
                .into_inner()
                .val
                .clone()
        })
    }

    pub fn peek(&self) -> Option<Ref<'_, T>> {
        self.front
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
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
        drop(self.next.take());
        println!("After drop node: {:?}", self.val);
    }
}

impl<T: Debug> Drop for LinkQueue<T> {
    fn drop(&mut self) {
        let mut cur_node = self.front.take();
        while let Some(node) = cur_node {
            if let Ok(node) = Rc::try_unwrap(node) {
                cur_node = node.borrow_mut().next.take();
                // node go out of scope
            } else {
                // cur point to last node which `rear` point to, too.
                // last node will drop when `rear` go out of scope.
                break;
                // node go out of scope
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut queue = LinkQueue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.size(), 3);
        assert!(!queue.is_empty());

        assert_eq!(queue.to_vec(), [1, 2, 3]);

        assert_eq!(*queue.peek().unwrap(), 1);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(*queue.peek().unwrap(), 2);
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(*queue.peek().unwrap(), 3);
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.pop(), None);

        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());
    }
}

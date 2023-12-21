use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    fmt::Debug,
    rc::Rc,
};

pub struct BinarySearchTree<T: Clone + Debug + Ord> {
    root: Option<Edge<T>>,
}
pub struct BinarySearchTreeNode<T: Clone + Debug + Ord> {
    val: T,
    left: Option<Edge<T>>,
    right: Option<Edge<T>>,
}

type Edge<T> = Rc<RefCell<BinarySearchTreeNode<T>>>;

impl<T: Clone + Debug + Ord> BinarySearchTreeNode<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }
}

impl<T: Clone + Debug + Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert_iterate(&mut self, val: T) {
        if self.root.is_none() {
            self.root = Some(BinarySearchTreeNode::new(val));
            return;
        }

        let mut cur = self.root.clone();
        let mut pre = None;

        while let Some(node) = cur.clone() {
            use std::cmp::Ordering::*;
            pre = cur;
            match node.borrow().val.cmp(&val) {
                Less => {
                    cur = node.borrow().right.clone();
                }
                Equal => return,
                Greater => {
                    cur = node.borrow().left.clone();
                }
            }
        }

        let pre = pre.unwrap();
        let new_node = BinarySearchTreeNode::new(val.clone());
        if pre.borrow().val > val {
            pre.borrow_mut().left = Some(new_node);
        } else {
            pre.borrow_mut().right = Some(new_node);
        }
    }

    pub fn insert_recursion(&mut self, val: T) {
        Self::_insert_recursion(&mut self.root, val);
    }

    pub fn _insert_recursion(root: &mut Option<Edge<T>>, val: T) {
        if let Some(root) = root {
            use std::cmp::Ordering::*;
            let val_in_node = root.borrow().val.clone();

            let (mut left, mut right) =
                RefMut::map_split(root.borrow_mut(), |root| (&mut root.left, &mut root.right));
            match val_in_node.cmp(&val) {
                Less => {
                    Self::_insert_recursion(&mut right, val);
                }
                Equal => return,
                Greater => {
                    Self::_insert_recursion(&mut left, val);
                }
            }
        } else {
            *root = Some(BinarySearchTreeNode::new(val))
        }
    }

    pub fn search_iterate(&self, val: T) -> Option<Edge<T>> {
        let mut cur = self.root.clone();

        while let Some(node) = cur.clone() {
            use std::cmp::Ordering::*;
            match node.borrow().val.cmp(&val) {
                Less => {
                    cur = node.borrow().right.clone();
                }
                Equal => return cur,
                Greater => {
                    cur = node.borrow().left.clone();
                }
            }
        }

        None
    }

    pub fn search_recursion(&self, val: T) -> Option<Edge<T>> {
        Self::_search_recursion(&self.root, val)
    }
    pub fn _search_recursion(root: &Option<Edge<T>>, val: T) -> Option<Edge<T>> {
        if let Some(node) = root {
            use std::cmp::Ordering::*;
            return match node.borrow().val.cmp(&val) {
                Less => Self::_search_recursion(&node.borrow().right, val),
                Equal => root.clone(),
                Greater => Self::_search_recursion(&node.borrow().left, val),
            };
        }
        None
    }

    pub fn remove(&mut self, val: T) {
        Self::_remove(&mut self.root, val);
    }
    pub fn _remove(root: &mut Option<Edge<T>>, val: T) {
        let mut cur = root.clone();
        let mut pre = None;

        while let Some(node) = cur.clone() {
            use std::cmp::Ordering::*;
            match node.borrow().val.cmp(&val) {
                Less => {
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                }
                Equal => break,
                Greater => {
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                }
            }
        }

        if let Some(to_remove) = cur {
            let (left_child, right_child) = (
                to_remove.borrow().left.clone(),
                to_remove.borrow().right.clone(),
            );
            match (left_child.clone(), right_child.clone()) {
                (None, None) | (None, Some(_)) | (Some(_), None) => {
                    let child = left_child.or(right_child);
                    if let Some(pre) = pre {
                        let left = pre.borrow().left.clone();
                        if left.is_some() && Rc::ptr_eq(&left.unwrap(), &to_remove) {
                            pre.borrow_mut().left = child;
                        } else {
                            pre.borrow_mut().right = child;
                        }
                    } else {
                        *root = child;
                    }
                }
                (Some(_), Some(_)) => {
                    let mut to_change = to_remove.borrow().right.clone();
                    while let Some(node) = to_change.clone() {
                        if node.borrow().left.is_none() {
                            break;
                        } else {
                            to_change = node.borrow().left.clone();
                        }
                    }

                    let to_change_val = to_change.unwrap().borrow().val.clone();
                    Self::_remove(&mut Some(to_remove.clone()), to_change_val.clone());

                    to_remove.borrow_mut().val = to_change_val;
                }
            }
        } // else cur is none and nothing to remove
    }

    pub fn print(&self) {
        Self::_print(&self.root, 0, "Root: ");
    }

    pub fn _print(node: &Option<Edge<T>>, level: usize, prefix: &str) {
        for _ in 0..level {
            print!("  ");
        }
        if let Some(node) = node {
            println!("{}{:?}", prefix, node.borrow().val);
            let new_level = level + 1;
            Self::_print(
                &node.borrow().left,
                new_level,
                format!("L{}: ", new_level).as_str(),
            );
            Self::_print(
                &node.borrow().right,
                new_level,
                format!("R{}: ", new_level).as_str(),
            );
        } else {
            println!("{}<None>", prefix);
        }
    }

    pub fn level_order(&self) -> Vec<T> {
        let mut result = vec![];
        if let Some(root) = &self.root {
            let mut que = VecDeque::new();
            que.push_back(Rc::clone(root));

            while let Some(node) = que.pop_front() {
                result.push(node.borrow().val.clone());
                if let Some(left) = node.borrow().left.as_ref() {
                    que.push_back(Rc::clone(left));
                }
                if let Some(right) = node.borrow().right.as_ref() {
                    que.push_back(Rc::clone(right));
                }
            }
        }
        result
    }

    pub fn pre_order(&self) -> Vec<T> {
        let mut result = vec![];
        Self::_pre_order(&self.root, &mut result);
        result
    }
    fn _pre_order(node: &Option<Edge<T>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            result.push(node.borrow().val.clone());
            Self::_pre_order(&node.borrow().left, result);
            Self::_pre_order(&node.borrow().right, result);
        }
    }

    pub fn in_order(&self) -> Vec<T> {
        let mut result = vec![];
        Self::_in_order(&self.root, &mut result);
        result
    }
    fn _in_order(node: &Option<Edge<T>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::_in_order(&node.borrow().left, result);
            result.push(node.borrow().val.clone());
            Self::_in_order(&node.borrow().right, result);
        }
    }

    pub fn post_order(&self) -> Vec<T> {
        let mut result = vec![];
        Self::_post_order(&self.root, &mut result);
        result
    }
    fn _post_order(node: &Option<Edge<T>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::_post_order(&node.borrow().left, result);
            Self::_post_order(&node.borrow().right, result);
            result.push(node.borrow().val.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        let mut tree = BinarySearchTree::new();
        tree.print();

        tree.insert_iterate(3);
        tree.insert_iterate(5);
        tree.insert_iterate(1);
        tree.insert_iterate(2);
        tree.insert_iterate(4);

        tree.print();

        tree.remove(3);
        tree.print();

        assert_eq!(tree.search_iterate(1).unwrap().borrow().val, 1);
        assert!(tree.search_iterate(3).is_none());
        assert!(tree.search_iterate(6).is_none());

        println!("Level order: {:?}", tree.level_order());
        println!("Pre order: {:?}", tree.pre_order());
        println!("In order: {:?}", tree.in_order());
        println!("Post order: {:?}", tree.post_order());
    }

    #[test]
    fn recursion() {
        let mut tree = BinarySearchTree::new();
        tree.print();

        tree.insert_recursion(3);
        tree.insert_recursion(5);
        tree.insert_recursion(1);
        tree.insert_recursion(2);
        tree.insert_recursion(4);

        tree.print();

        assert_eq!(tree.search_recursion(1).unwrap().borrow().val, 1);
        assert_eq!(tree.search_recursion(3).unwrap().borrow().val, 3);
        assert!(tree.search_recursion(6).is_none());

        println!("Level order: {:?}", tree.level_order());
        println!("Pre order: {:?}", tree.pre_order());
        println!("In order: {:?}", tree.in_order());
        println!("Post order: {:?}", tree.post_order());
    }
}

use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    fmt::Debug,
    rc::Rc,
};

pub struct AVLTree<T: Clone + Debug + Ord> {
    root: Option<Edge<T>>,
}
pub struct AVLTreeNode<T: Clone + Debug + Ord> {
    val: T,
    height: i32,
    left: Option<Edge<T>>,
    right: Option<Edge<T>>,
}

type Edge<T> = Rc<RefCell<AVLTreeNode<T>>>;

impl<T: Clone + Debug + Ord> AVLTreeNode<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            height: 0,
            left: None,
            right: None,
        }))
    }
}

impl<T: Clone + Debug + Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn height(node: Option<Edge<T>>) -> i32 {
        if let Some(node) = node {
            return node.borrow().height;
        }
        -1
    }

    fn update_height(node: Option<Edge<T>>) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = std::cmp::max(Self::height(left), Self::height(right)) + 1;
        }
    }

    fn balance_factor(node: Option<Edge<T>>) -> i32 {
        match node {
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
            None => 0,
        }
    }

    fn right_rotate(node: Option<Edge<T>>) -> Option<Edge<T>> {
        node.map(|node| {
            let child = node.borrow_mut().left.take().unwrap();
            let grand_child = child.borrow_mut().right.take();
            node.borrow_mut().left = grand_child;
            child.borrow_mut().right = Some(Rc::clone(&node));

            Self::update_height(Some(node));
            Self::update_height(Some(Rc::clone(&child)));

            child
        })
    }

    fn left_rotate(node: Option<Edge<T>>) -> Option<Edge<T>> {
        node.map(|node| {
            let child = node.borrow_mut().right.take().unwrap();
            let grand_child = child.borrow_mut().left.take();
            node.borrow_mut().right = grand_child;
            child.borrow_mut().left = Some(Rc::clone(&node));

            Self::update_height(Some(node));
            Self::update_height(Some(Rc::clone(&child)));

            child
        })
    }

    fn rotate(node: Option<Edge<T>>) -> Option<Edge<T>> {
        let balance_factor = Self::balance_factor(node.clone());

        if balance_factor > 1 {
            let node = node.unwrap();
            let left = node.borrow().left.clone();
            if Self::balance_factor(left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
        } else if balance_factor < -1 {
            let node = node.unwrap();
            let right = node.borrow().right.clone();
            if Self::balance_factor(right.clone()) <= 0 {
                Self::left_rotate(Some(node))
            } else {
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        } else {
            node
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
            *root = Some(AVLTreeNode::new(val))
        }
        Self::update_height(root.clone());
        *root = Self::rotate(root.clone());
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
        if let Some(mut to_remove) = root.clone() {
            use std::cmp::Ordering::*;
            match {
                let v = to_remove.borrow().val.clone();
                v
            }
            .cmp(&val)
            {
                Less => Self::_remove(&mut to_remove.borrow_mut().right, val),
                Greater => Self::_remove(&mut to_remove.borrow_mut().left, val),
                Equal => {
                    let (left_child, right_child) = (
                        to_remove.borrow().left.clone(),
                        to_remove.borrow().right.clone(),
                    );
                    match (left_child.clone(), right_child.clone()) {
                        (None, None) => {
                            *root = None;
                            return;
                        }
                        (None, Some(_)) | (Some(_), None) => {
                            to_remove = left_child.or(right_child).unwrap();
                        }
                        (Some(_), Some(_)) => {
                            let mut to_change = to_remove.borrow().right.clone().unwrap();
                            loop {
                                let left = to_change.borrow().left.clone();
                                if left.is_some() {
                                    to_change = left.unwrap();
                                } else {
                                    break;
                                }
                            }
                            let to_change_val = to_change.borrow().val.clone();
                            Self::_remove(&mut to_remove.borrow_mut().right, to_change_val.clone());
                            to_remove.borrow_mut().val = to_change_val;
                        }
                    }
                }
            }
            Self::update_height(Some(to_remove.clone()));
            *root = Self::rotate(Some(to_remove));
        }
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
        let mut tree = AVLTree::new();
        tree.print();

        tree.insert_recursion(3);
        tree.insert_recursion(5);
        tree.insert_recursion(1);
        tree.insert_recursion(2);
        tree.insert_recursion(4);
        tree.insert_recursion(6);
        tree.insert_recursion(7);
        tree.insert_recursion(8);
        tree.insert_recursion(9);

        tree.print();
        println!("In order: {:?}", tree.in_order());

        tree.remove(3);
        tree.print();
        println!("In order: {:?}", tree.in_order());
        tree.remove(7);
        tree.print();
        println!("In order: {:?}", tree.in_order());
        tree.remove(999);
        tree.print();
        println!("In order: {:?}", tree.in_order());

        assert_eq!(tree.search_iterate(1).unwrap().borrow().val, 1);
        assert!(tree.search_iterate(3).is_none());
        assert!(tree.search_iterate(7).is_none());
        assert!(tree.search_iterate(10).is_none());

        println!("Level order: {:?}", tree.level_order());
        println!("Pre order: {:?}", tree.pre_order());
        println!("In order: {:?}", tree.in_order());
        println!("Post order: {:?}", tree.post_order());
    }

    #[test]
    fn recursion() {
        let mut tree = AVLTree::new();
        tree.print();

        tree.insert_recursion(3);
        tree.insert_recursion(5);
        tree.insert_recursion(1);
        tree.insert_recursion(2);
        tree.insert_recursion(4);
        tree.insert_recursion(6);
        tree.insert_recursion(7);
        tree.insert_recursion(8);
        tree.insert_recursion(9);

        tree.print();

        assert_eq!(tree.search_recursion(1).unwrap().borrow().val, 1);
        assert_eq!(tree.search_recursion(3).unwrap().borrow().val, 3);
        assert!(tree.search_recursion(10).is_none());

        println!("Level order: {:?}", tree.level_order());
        println!("Pre order: {:?}", tree.pre_order());
        println!("In order: {:?}", tree.in_order());
        println!("Post order: {:?}", tree.post_order());
    }

    #[test]
    fn right_rotate() {
        let mut tree = AVLTree::new();
        tree.print();

        tree.insert_recursion(3);
        tree.insert_recursion(2);

        tree.print();

        tree.insert_recursion(1);

        tree.print();
    }

    #[test]
    fn left_right_rotate() {
        let mut tree = AVLTree::new();
        tree.print();

        tree.insert_recursion(3);
        tree.insert_recursion(1);

        tree.print();

        tree.insert_recursion(2);

        tree.print();
    }

    #[test]
    fn left_rotate() {
        let mut tree = AVLTree::new();
        tree.print();

        tree.insert_recursion(1);
        tree.insert_recursion(2);

        tree.print();

        tree.insert_recursion(3);

        tree.print();
    }

    #[test]
    fn right_left_rotate() {
        let mut tree = AVLTree::new();
        tree.print();

        tree.insert_recursion(1);
        tree.insert_recursion(3);

        tree.print();

        tree.insert_recursion(2);

        tree.print();
    }
}

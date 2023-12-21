use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

pub struct BinaryTree<T: Clone + Debug> {
    root: Option<BinaryTreeEdge<T>>,
}
pub struct BinaryTreeNode<T: Clone + Debug> {
    val: T,
    left: Option<BinaryTreeEdge<T>>,
    right: Option<BinaryTreeEdge<T>>,
}

type BinaryTreeEdge<T> = Rc<RefCell<BinaryTreeNode<T>>>;

impl<T: Clone + Debug> BinaryTreeNode<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }
}

impl<T: Clone + Debug> BinaryTree<T> {
    pub fn from_vec(v: Vec<Option<T>>) -> Self {
        Self {
            root: BinaryTree::_from_vec(&v, 0),
        }
    }

    fn _from_vec(v: &Vec<Option<T>>, index: usize) -> Option<BinaryTreeEdge<T>> {
        if index >= v.len() {
            return None;
        }

        v[index].as_ref().map(|val| {
            let node = BinaryTreeNode::new(val.clone());
            node.borrow_mut().left = Self::_from_vec(v, index * 2 + 1);
            node.borrow_mut().right = Self::_from_vec(v, index * 2 + 2);
            node
        })
    }

    pub fn insert_left(root: &mut BinaryTreeEdge<T>, val: T) {
        let node = BinaryTreeNode::new(val);
        node.borrow_mut().left = root.borrow_mut().left.take();
        root.borrow_mut().left = Some(node);
    }

    pub fn insert_right(root: &mut BinaryTreeEdge<T>, val: T) {
        let node = BinaryTreeNode::new(val);
        node.borrow_mut().right = root.borrow_mut().right.take();
        root.borrow_mut().right = Some(node);
    }

    pub fn print(&self) {
        Self::_print(&self.root, 0, "Root: ");
    }

    pub fn _print(node: &Option<BinaryTreeEdge<T>>, level: usize, prefix: &str) {
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
    fn _pre_order(node: &Option<BinaryTreeEdge<T>>, result: &mut Vec<T>) {
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
    fn _in_order(node: &Option<BinaryTreeEdge<T>>, result: &mut Vec<T>) {
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
    fn _post_order(node: &Option<BinaryTreeEdge<T>>, result: &mut Vec<T>) {
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
    fn init() {
        let tree = BinaryTree::from_vec(vec![
            Some(0),
            Some(1),
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        tree.print();

        if let Some(node) = tree
            .root
            .clone()
            .unwrap()
            .borrow_mut()
            .left
            .clone()
            .unwrap()
            .borrow_mut()
            .right
            .clone()
            .as_mut()
        {
            BinaryTree::insert_left(node, 9);
            BinaryTree::insert_right(node, 99);
        }

        tree.print();

        println!("Level order: {:?}", tree.level_order());
        println!("Pre order: {:?}", tree.pre_order());
        println!("In order: {:?}", tree.in_order());
        println!("Post order: {:?}", tree.post_order());
    }
}

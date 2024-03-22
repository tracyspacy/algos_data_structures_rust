use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn preorder_traversal<T: Debug>(root: Option<Rc<RefCell<TreeNode<T>>>>) {
    match root {
        None => {}
        Some(node) => {
            let borrowed_node: &RefCell<TreeNode<T>> = node.borrow();
            println!("{:?}", borrowed_node.borrow().val);
            if let Some(left) = borrowed_node.borrow().left.as_ref() {
                preorder_traversal(Some(left.clone())); //cloning reference counted pointer (Rc) to the RefCell containing the TreeNode,not entire tree
            }
            if let Some(right) = borrowed_node.borrow().right.as_ref() {
                preorder_traversal(Some(right.clone()));
            }
        }
    }
}

/*
               1
            /    \
           2      3
         /  \    /  \
        4    5  6    7

*/

fn main() {
    let mut tree: BinaryTree<i32> = BinaryTree::new();
    tree.root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{:?}", tree.root);
    if let Some(root) = tree.root.as_mut() {
        if let Some(left_node) = root.left.as_mut() {
            left_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        }
    }

    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    if let Some(left_node) = root.left.as_mut() {
        //same as with root we should make root.left mut to modify
        left_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        // borrowing is obtain mut ref to Refcall containing root.left,
        // ie to access and mutate the left field of the root inside the RefCell contained within the Rc.
    }
    if let Some(right_node) = root.left.as_mut() {
        right_node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    }

    if let Some(left_node) = root.right.as_mut() {
        left_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    }
    if let Some(right_node) = root.right.as_mut() {
        right_node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    }

    println!("{:?}", root);
    preorder_traversal(Some(Rc::new(RefCell::new(root))));
}

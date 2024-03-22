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
impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn preorder_traversal<T: Debug>(root: &Option<Rc<RefCell<TreeNode<T>>>>) {
    match root {
        None => {}
        Some(node) => {
            let borrowed_node: &RefCell<TreeNode<T>> = node.borrow();
            println!("{:?}", borrowed_node.borrow().val);
            if let Some(left) = borrowed_node.borrow().left.as_ref() {
                preorder_traversal(&Some(left.clone())); //cloning reference counted pointer (Rc) to the RefCell containing the TreeNode,not entire tree
            }
            if let Some(right) = borrowed_node.borrow().right.as_ref() {
                preorder_traversal(&Some(right.clone()));
            }
        }
    }
}

pub fn inorder_traversal<T: Debug>(root: &Option<Rc<RefCell<TreeNode<T>>>>) {
    match root {
        None => {}
        Some(node) => {
            let borrowed_node: &RefCell<TreeNode<T>> = node.borrow();
            if let Some(left) = borrowed_node.borrow().left.as_ref() {
                inorder_traversal(&Some(left.clone()));
            }
            println!("{:?}", borrowed_node.borrow().val);
            if let Some(right) = borrowed_node.borrow().right.as_ref() {
                inorder_traversal(&Some(right.clone()));
            }
        }
    }
}

pub fn postorder_traversal<T: Debug>(root: &Option<Rc<RefCell<TreeNode<T>>>>) {
    if let Some(node) = root {
        let borrowed_node: &RefCell<TreeNode<T>> = node.borrow();
        if let Some(left) = borrowed_node.borrow().left.as_ref() {
            postorder_traversal(&Some(left.clone()));
        }
        if let Some(right) = borrowed_node.borrow().right.as_ref() {
            postorder_traversal(&Some(right.clone()));
        }
        println!("{:?}", borrowed_node.borrow().val);
    }
}

pub fn calculate_max_depth<T: Debug>(root: &Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
    match root {
        None => return 0,
        Some(node) => {
            let borrowed_node: &RefCell<TreeNode<T>> = node.borrow();
            let mut left_depth = 0;
            let mut right_depth = 0;
            if let Some(left) = borrowed_node.borrow().left.as_ref() {
                left_depth = calculate_max_depth(&Some(left.clone()));
            };
            if let Some(right) = borrowed_node.borrow().right.as_ref() {
                right_depth = calculate_max_depth(&Some(right.clone()));
            };
            if left_depth > right_depth {
                return left_depth + 1;
            } else {
                return right_depth + 1;
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
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
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

    preorder_traversal(&Some(Rc::new(RefCell::new(root.clone()))));
    //inorder_traversal(&Some(Rc::new(RefCell::new(root))));
    //postorder_traversal(&Some(Rc::new(RefCell::new(root))));
    println!(
        "Max depth is: {:?}",
        calculate_max_depth(&Some(Rc::new(RefCell::new(root))))
    );
}

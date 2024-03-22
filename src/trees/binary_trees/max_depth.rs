/*

Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
example:
    3
   / \
  9   20
      / \
    15   7

    Input: root = [3,9,20,null,null,15,7]
    Output: 3
    Example 2:

    Input: root = [1,null,2]
    Output: 2
*/

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => return 0,
            Some(node) => {
                let borrowed_node = node.borrow();
                let left_depth = Solution::max_depth(borrowed_node.left.clone());
                let right_depth = Solution::max_depth(borrowed_node.right.clone());
                if left_depth > right_depth {
                    return left_depth + 1;
                } else {
                    return right_depth + 1;
                }
            }
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    if let Some(left_node) = root.right.as_mut() {
        left_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    }
    if let Some(right_node) = root.right.as_mut() {
        right_node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    }
    let result = Solution::max_depth(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}

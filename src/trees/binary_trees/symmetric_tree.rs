/*
Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
Example 1:
Input: root = [1,2,2,3,4,4,3]
Output: true
         1
      /  |   \
    2    |    2
  /  \   |   /  \
 3   4   |  4    3
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
    //helper fn. remains fn that checks if 2 binary trees are the same, but mirroring instead
    //https://github.com/tracyspacy/algos_data_structures_rust/blob/master/src/trees/binary_trees/same_tree.rs
    fn is_mirror(
        left_child: Option<Rc<RefCell<TreeNode>>>,
        right_child: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left_child, right_child) {
            (None, None) => true,
            (Some(left_child), Some(right_child)) => {
                let borrowed_left = left_child.borrow();
                let borrowed_right = right_child.borrow();
                borrowed_left.val == borrowed_right.val
                    && Solution::is_mirror(borrowed_left.left.clone(), borrowed_right.right.clone())
                    && Solution::is_mirror(borrowed_left.right.clone(), borrowed_right.left.clone())
            }
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let borrowed_node = node.borrow();
                let left = borrowed_node.left.clone();
                let right = borrowed_node.right.clone();
                Solution::is_mirror(left, right)
            }
            None => true,
        }
    }
}

/*
Solution logic:
             1
          /  |   \
        2    |    2
      /  \   |   /  \
     3   4   |  4    3

   The main idea is to check if left child is mirro of the right child.
   1. get left and right children of binary tree;
   2. recursively check if left child is a mirror of the right with helper fn is_mirror();
*/

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));

    let result = Solution::is_symmetric(root);
    println!("{:?}", result);
}

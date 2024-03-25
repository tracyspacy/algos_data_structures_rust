/*
Given the root of a binary tree, invert the tree, and return its root.
Input: root = [4,2,7,1,3,6,9]
Output: [4,7,2,9,6,3,1]
       4                                       4
    /    \                                   /    \
   2      7           =>                    7      2
 /  \    /  \                             /  \    /  \
1   3   6    9                           9    6  3    1

Input: root = []
Output: []

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let borrowed_node = node.borrow();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: borrowed_node.val,
                    left: Solution::invert_tree(borrowed_node.right.clone()),
                    right: Solution::invert_tree(borrowed_node.left.clone()),
                })))
            }
            None => None,
        }
    }
}

/*
Solution logic:
             4                                      4
          /    \                                  /    \
        2       7           =>                   7      2
      /  \    /  \                             /  \    /  \
     1   3   6    9                           9    6  3    1

    Logic is quite simple: recursively swapping the left node with the right node.
*/

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));

    let result = Solution::invert_tree(root);
    println!("{:?}", result);
}

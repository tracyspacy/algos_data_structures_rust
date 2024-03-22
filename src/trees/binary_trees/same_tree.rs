/*
Given the roots of two binary trees p and q, write a function to check if they are the same or not.

Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

Example 1:
     1          1
   /  \        / \
  2    3      2   3
  Input: p = [1,2,3], q = [1,2,3]
  Output: true
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        println!("{:?}", &p);
        match (p, q) {
            (None, None) => true,
            (Some(node_p), Some(node_q)) => {
                let node_p = node_p.borrow();
                let node_q = node_q.borrow();

                node_p.val == node_q.val
                    && Solution::is_same_tree(node_p.left.clone(), node_q.left.clone())
                    && Solution::is_same_tree(node_p.right.clone(), node_q.right.clone())
            }
            _ => false,
        }
    }
}

/*
Solution logic:
 1
/ \
2 3
1.fisrstly basic check if roots are empty   (not empty)
2. if not empty, than check if node values are same (same 1==1)
3. if values are same
-> call recursive fn on left node for both trees (goes 2->None->None for both)
-> call recursive fn on right node for both trees (goes 3->None->None for both)
technically in recursive calls we compare node values and if they not same, whole equation is not true
*!recursion will continue till first inequal node vals.*
if equal longest recursion 123 123 : 1->2->None->None->3->None->None =>true
if not equal till != 123 124 : 1->2->None->None->3 => false
*/

fn main() {
    let mut root1 = TreeNode::new(1);
    root1.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root1.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    let mut root2 = TreeNode::new(1);
    root2.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root2.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    let result = Solution::is_same_tree(
        Some(Rc::new(RefCell::new(root1))),
        Some(Rc::new(RefCell::new(root2))),
    );
    println!("{:?}", result);
}

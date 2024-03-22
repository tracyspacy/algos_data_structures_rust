/*

You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]


*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            //checking simple options
            (None, None) => return None,
            (Some(node1), None) => return Some(node1),
            (None, Some(node2)) => return Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                //picking list with smallest val, repeating with pointer to next node
                if node1.val < node2.val {
                    //println!("First list picked {:?}",node1);
                    node1.next = Solution::merge_two_lists(node1.next.take(), Some(node2));

                    Some(node1)
                } else {
                    //println!("Second list picked {:?}",node2);
                    node2.next = Solution::merge_two_lists(Some(node1), node2.next.take());

                    Some(node2)
                }
            }
        }
    }
}

fn main() {
    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in values.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
    let l1 = create_list(vec![1, 2, 4]);
    let l2 = create_list(vec![1, 3, 4]);

    let result = Solution::merge_two_lists(l1, l2);
    println!("{:?}", result);
}

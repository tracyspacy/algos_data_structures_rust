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
    pub fn reverse(mut list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None; // new pointer that will point to 1st node of reversed list
        while let Some(mut head) = list {
            //head = 1->2->3->4->5
            let next = head.next; // 2->3->4->5
            head.next = previous; // 1->None ie turning pointer in opposite direction
            previous = Some(head); // 1->None -updating previous for next iteration
            println!("{:?}", previous);
            list = next; // 2->3->4->5 //updating list for iteration
        }
        previous
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
    let l1 = create_list(vec![1, 2, 3, 4, 5]);

    let result = Solution::reverse(l1.clone());
    println!("init : {:?}, reversed: {:?}", l1, result);
}

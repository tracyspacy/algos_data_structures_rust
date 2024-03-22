/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.


Example 1:

Input: s = "()"
Output: true
Example 2:

Input: s = "()[]{}"
Output: true
Example 3:

Input: s = "(]"
Output: false


Constraints:

1 <= s.length <= 104
s consists of parentheses only '()[]{}'.


*/

pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        if s.is_empty() {
            return false;
        }
        for i in s.chars() {
            match i {
                '(' | '[' | '{' => stack.push(i),
                ')' | ']' | '}' => {
                    let popped = stack.pop().unwrap_or('_');
                    match popped {
                        '(' => {
                            if i != ')' {
                                return false;
                            }
                        }
                        '[' => {
                            if i != ']' {
                                return false;
                            }
                        }
                        '{' => {
                            if i != '}' {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}
fn main() {
    let i = Solution::is_valid("{}}".to_string());
    println!("{:?}", i);
}

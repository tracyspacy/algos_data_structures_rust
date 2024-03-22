/*
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.



Example 1:

Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
Example 2:

Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.
Example 3:

Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.

Constraints:

1 <= s.length <= 2 * 105
s consists only of printable ASCII characters.
*/

pub struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars_arr: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect();

        if s.is_empty() || s == String::from(" ") || chars_arr.len() == 0 {
            return true;
        }
        let mut left = 0;
        let mut right = chars_arr.len() - 1;
        while left < right {
            if chars_arr[left] != chars_arr[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

fn main() {
    let text = String::from("A man, a plan, a canal: Panama");

    println!("Initial state: {:?}", text);
    let s = Solution::is_palindrome(text);
    assert_eq!(s, true);
    println!("Final state:{:?}", s);
}

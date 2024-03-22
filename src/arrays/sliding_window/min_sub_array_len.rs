/*
Given an array of positive integers nums and a positive integer target, return the minimal length of a
subarray
 whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.



Example 1:

Input: target = 7, nums = [2,3,1,2,4,3]
Output: 2
Explanation: The subarray [4,3] has the minimal length under the problem constraint.
Example 2:

Input: target = 4, nums = [1,4,4]
Output: 1
Example 3:

Input: target = 11, nums = [1,1,1,1,1,1,1,1]
Output: 0


*/

use std::cmp;
pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut left = 0;
        let mut answer = nums.len() + 1;
        for right in 0..nums.len() {
            current += nums[right];
            while current >= target {
                answer = cmp::min(answer, right - left + 1);
                current -= nums[left];
                left += 1;
            }
        }
        if answer > nums.len() {
            0
        } else {
            answer as i32
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let k = 11;
    let i = Solution::min_sub_array_len(k, nums);
    println!("{:?}", i);
}

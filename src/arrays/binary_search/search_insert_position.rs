/*
Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.



Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2
Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1
Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4


Constraints:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums contains distinct values sorted in ascending order.
-104 <= target <= 104

*/

pub struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut left_pointer = 0;
        let mut right_pointer = nums.len() - 1;
        while left_pointer <= right_pointer {
            let middle = (left_pointer + right_pointer) / 2;
            println!("{:?}", left_pointer);
            match nums[middle] {
                mid if mid == target => {
                    println!("here"); //if value exists returns here
                    return middle as i32;
                }
                mid if mid < target => {
                    println!("{:?}<target", mid);
                    println!("{:?}", nums[(right_pointer / 2 + 1)..].to_vec());
                    left_pointer = middle + 1; //if target is > mid, move to right, so return left pointer [1,2,3] 6 < if not in vec could be beyond index
                }
                _ => {
                    println!("{:?}", nums[..(right_pointer / 2)].to_vec());
                    if middle > 0 {
                        right_pointer = middle - 1; // if mid is > target moving to left and return left pointer since it is 0 if no value in vec
                    } else {
                        break;
                    }
                }
            }
        }
        return left_pointer as i32;
    }
}

fn main() {
    let nums = vec![];
    let target = 10;
    let i = Solution::search_insert(nums, target);
    println!("index is {:?}", i);
}

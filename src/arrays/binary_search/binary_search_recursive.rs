/*

recursive binary search example

*/

pub struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let right_pointer = nums.len() - 1;

        let middle = right_pointer / 2;
        match nums[middle] {
            mid if mid == target => (right_pointer / 2) as i32,
            mid if mid < target => {
                println!("{:?}<target", mid);
                println!("{:?}", nums[(right_pointer / 2 + 1)..].to_vec());
                let index =
                    Solution::search_insert(nums[(right_pointer / 2 + 1)..].to_vec(), target)
                        + (right_pointer / 2 + 1) as i32;
                index
            }
            mid if mid > target => {
                println!("{:?}>{:?}", mid, target);
                println!("{:?}", nums[..(right_pointer / 2)].to_vec());
                Solution::search_insert(nums[..(right_pointer / 2)].to_vec(), target)
            }
            _ => 0,
        }
    }
}

fn main() {
    let nums = vec![3, 4, 5, 6, 7, 8, 9];
    let target = 9;
    let i = Solution::search_insert(nums, target);
    println!("index is {:?}", i);
}

### Arrays - TWO POINTERS - sorted arrays

Two pointers is really an easy and effective technique that is typically used for searching pairs in a **sorted array** or merging sorted arrays.

This technique is commonly used to solve problems that involve searching for a specific condition or pattern within a data set, or that require a comparison between different elements in the data set.

The two pointers technique is mainly used for solving problems that have a linear time complexity, it can lead to substantial performance improvements over a brute-force approach. Some common examples of problems that can be solved using this technique include:

- Finding the maximum / minimum value in a set of data.
- Counting the number of occurrences of a specific element.
- Finding the longest substring without repeating characters.
- Finding the maximum sum of a sub-array of size k.

**The core idea of approach is to have 2 pointers for ex:  one points to start of array and another to end  and they moves towards each other based on some condition. **
There are many approaches but they include 2 pointers and their movement through data set based on conditions.

example:

```
Given a sorted array A ( sorted in ascending order ) and an integer target,find if there exists 2 integers A[i] and A[j] such that A[i] + A[j] = target, where i!=j.
```

![image.png](/Users/tracyspacy/Downloads/image.png)




 How to solve:

- set pointer1 = 0 (beginning) and pointer2 = len -1 (end)
- Compute the sum of the two numbers pointed to at each step.

**If the sum is greater than the target**, we need to move pointer2, to the left.  == we assume that sum is too big because of highest value, that is pointed by pointer2 since array is sorted, so we moving pointer to left -> smaller value



![image1.png](/Users/tracyspacy/Downloads/image1.png)

**Opposite, if the sum is less than the target, we need to increase the estimate by moving the left pointer, i.e. pointer1, to the right.**



![image2.png](/Users/tracyspacy/Downloads/image2.png)

example 2 Strings palindrom:

```
Given a string s, return true if it is a palindrome, false otherwise.

A string is a palindrome if it reads the same forward as backward. That means, after reversing it, it is still the same string. For example: "abcdcba", or "racecar".
```

We can use the two pointers technique here to check that all corresponding characters are equal. To start, we check the first and last characters using two separate pointers. To check the next pair of characters, we just need to move our pointers toward each other one position. We continue until the pointers meet each other or we find a mismatch.

logic:

- pointer 1 left = 0, pointer 2 right = len -1
- start loop while left<right where you check if s[left] (char with left pointer) not equal s[right] (char with right pointer). If chars not equal -> false
- with each step move left pointer 1 step right `left+=1`, and right 1 step to left `right-=1`

code :

```rust
//&str ie string slice not String to not clone String, just use reference
fn check_if_palindrome(s: &str) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;
//creating vector with chars to iterate
    let chars_arr: Vec<char> = s.chars().collect();

    while left < right {
        if chars_arr[left] != chars_arr[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}
```

source:
https://www.geeksforgeeks.org/two-pointers-technique/
https://www.codingninjas.com/studio/library/what-is-a-two-pointer-technique
https://leetcodethehardway.com/tutorials/basic-topics/two-pointers
https://leetcode.com/explore/featured/card/leetcodes-interview-crash-course-data-structures-and-algorithms/703/arraystrings/4501/

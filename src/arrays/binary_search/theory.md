### Binary Search

Binary search is a widely used algorithm for searching an element in a **sorted array** or list.

The basic idea of binary search is to **divide the search space in half** with each iteration **and compare the middle element with the target element**. **If the middle element is greater than the target element, the search space is reduced to the left half of the array, otherwise, it is reduced to the right half.** This process is repeated until the target element is found or the search space is exhausted.

The time complexity of binary search is O(logn), which is more efficient than the linear search algorithm O(n), which checks all elements one by one. However, for binary search to work, the array or list must be sorted.

> Binary search can be implemented only on a sorted list of items. If the elements are not sorted already, we need to sort them first.

Binary Search Algorithm can be implemented in two ways which are discussed below.

1. Iterative Method
2. Recursive Method

The recursive method follows **devide and conquer** approach.

**The basic steps for a binary search algorithm are:**

1. Initialize two pointers, one pointing to the start of the array and the other pointing to the end.
   
   ```
   [3,4,5,6,7,8,9]        target = 4;
    ^           ^
   low         high
   ```

2. Find the middle element of the array by calculating the average of the two pointers.
   
   (find index of middle -> get value )
   
   ```
   [3,4,5,6,7,8,9]        target = 4;
          ^
        middle
   ```

3. Compare the middle element with the target element.
   
   ```
   [3,4,5,6,7,8,9]        middle > target;
          ^
        middle
   ```

4. If the middle element is equal to the target element, the search is complete and the index of the target element is returned.

5. If the middle element is less than the target element, move the left pointer to the middle element + 1 and repeat step 2.

6. If the middle element is greater than the target element, move the right pointer to the middle element - 1 and repeat step 2.
   
   ```
   [3,4,5,6,7,8,9]        target = 4;
    ^   ^ <- going left
   low high  
   ```

7. Repeat steps 3 to 6 until low meets high.
   
   ```
   [3,4,5]                target = 4;
      ^
    middle                middle == target -> found
   ```
   
   

8. If the left pointer is greater than the right pointer, the target element is not found and the function returns -1.



#### Rust Specific

```rust
let nums = vec![1,2,3,4];
let target = 3;
let x =nums.binary_search(&target); //returns option
```





Source:
[https://leetcodethehardway.com/tutorials/basic-topics/binary-search](https://leetcodethehardway.com/tutorials/basic-topics/binary-search)

[Binary Search (With Code)](https://www.programiz.com/dsa/binary-search)[Binary Search (With Code)](https://www.programiz.com/dsa/binary-search)

[Vanilla Binary Search](https://algo.monster/problems/binary_search_intro#)

### ARRAYS

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.




![](/Users/tracyspacy/Library/Application%20Support/marktext/images/2024-02-22-17-13-41-image.png)



```
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than the heap.
or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector.

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

https://doc.rust-lang.org/book/ch03-02-data-types.html

good sum up
[Introduction to Arrays - Data Structure and Algorithm Tutorials - GeeksforGeeks](https://www.geeksforgeeks.org/introduction-to-arrays-data-structure-and-algorithm-tutorials/)

### **RUST SPECIFIC**

Array [i32:5]

 !**An array is a collection of objects of the same type `T`, stored in contiguous memory. Arrays are created using brackets `[]`, and their length, which is known at compile time, is part of their type signature `[T; length]`.**

slice &[i32]

**! Slices are similar to arrays, but their length is not known at compile time.
Instead, a slice is a two-word object; the first word is a pointer to the data,
the second word is the length of the slice. The word size is the same as usize,
determined by the processor architecture, e.g. 64 bits on an x86-64. Slices can
be used to borrow a section of an array and have the type signature `&[T]`.**

Slice indicate borrow of memory (refference to elements in memory), not ownership!

One of the main differences between a Vec and a slice is that a Vec can be used to add and remove elements, while a slice only provides read-only access to a subset of the elements in a collection. Another difference is that a Vec is allocated on the heap, while a slice is a reference and therefore has a fixed size. This means that a slice cannot be used to store new elements, but it can be used to reference a subset of the elements in a Vec or other collection.

[Arrays and Slices - Rust By Example](https://doc.rust-lang.org/rust-by-example/primitives/array.html)

**Array operations:**

```rust
//create array
let array: [i32; 3] = [0; 3];

//modifying element in array
array[1] = 1;


//get element in array
array[1];
array.get(1); //returns Some()

//Traverse through the elements of an array

//iterating through reference of array elements - read only
 for x in &array {
    println("{}", x);
}
//iterating through elements of array, can modify
for y in array {
    println("{}", y);
}
//with iter - iter by refference
for item in array.iter().enumerate() {
    let (i, x): (usize, &i32) = item;
    println!("array[{i}] = {x}");
}
for item in array.iter() {
    println!("{}",item);
}

// This iterates by value:
for item in array.into_iter().enumerate() {
    let (i, x): (usize, i32) = item;
    println!("array[{i}] = {x}");
}

//more functional way , since iter() -over refference
array.iter().for_each(|item| println!("{:?}", item));

//Searching:  Search for an element in the array
array.contains(&1);

//Sorting: Maintaining the order of elements in the array
array.sort();
```


**Vectors**

Vectors are re-sizable arrays. Like slices, their size is not known at compile
time, but they can grow or shrink at any time. A vector is represented using
3 parameters:

- pointer to the data
- length
- capacity

The capacity indicates how much memory is reserved for the vector. The vector
can grow as long as the length is smaller than the capacity. When this threshold
needs to be surpassed, the vector is reallocated with a larger capacity.
[Vectors - Rust By Example](https://doc.rust-lang.org/rust-by-example/std/vec.html#vectors)



```rust
// create vector
let vec1 = vec![1,2,3];
let vec2 = Vec::from([1, 2, 3, 4]);
let vec3 :Vec<i32> = Vec::new();
let mut vec4 = vec![9];
let mut vec5 = vec![1,2,3,4,5];
let a = [1,2,3,5,6];
let v11 :Vec<usize> = a.iter().cloned().collect(); // with collection
let new_vector:Vec<i32> = vec.iter().map(|item| item + 1).collect(); // collection
//Insertion: Inserting a new element in an array
vec3.push(1);
vec3.push(2);
vec3.push(2);
vec3.push(2);
vec4.append(&mut vec5); //Moves all the elements of other into self,
                        // leaving other empty. vec4 [9,1,2,3,4,5] vec5 []

//get element
vec2[0]; // -> 1
vec3.is_empty(); // checks if empty
vec1.get(0); // returns option Some(1) - Returns a reference to an element
vec1.get(0..2); // option Some[1,2] - returns subslice
vec1.get_mut(0..2); //Returns a mutable reference to an element or subslice
vec1.len(); //returns number of elements ie length -> 3
vec5.contains(&4); // -> true

//Deletion: Deleting element from the array
vec3.pop(); //deletes last element , returns (Option) element or None
vec2.drain(..2); //drains all after 2nd ie -> [1,2]
vec2.drain(..); // drains all
vec3.clear();  // same as drain(..)
vec3.dedup(); //removes consecutive repeated elements -> [1,2]
vec4.truncate(5); //Shortens the vector, keeping the first len elements
                  // and dropping the rest. -> [9,1,2,3,4]

 //Sorting: Maintaining the order of elements in the array
vec4.sort(); // -> [1,2,3,4,9] to sort integer
vec4.sort_unstable(); //faster
let mut vec_float = vec![1.2,3.2,2.2];
vec_float.sort_by(|a, b| a.partial_cmp(b).unwrap()); //sorts float
//or
#![feature(sort_floats)] //nightly only
vec.sort_floats();


//Traverse through the elements of a vector
//iterates by mut value - can be changed
for mut item in vec_float {
    item+=1;
}
for mut item in &vec_float{
    item+=1; //-> error iterating over reference ie &int
    item; // -> ok read only
}
for mut item in vec_float.iter(){
    item+=1; //-> error iterating over reference ie &int
    item;    //-> ok read only
}

//over mutable reference, vec not consumed, could be used
for mut item in vec_float.iter_mut(){
    *item+=1; // -> ok since iter over mut reference,
              //dereference to change value
}

//into_iter takes ownership
for mut item in vec_float.into_iter(){
    item+=1; // -> or iterating over value
}
println!("{:?}",vec_float); //error - vec is moved


//more functional way
vec_float.into_iter().for_each(|mut item| item+=1); //same as previous


vec_float.iter_mut().for_each(|mut item| *item+=1); // over mut reference
println!("{:?}",vec_float); //ok -> not consumes vec_float
```

Connected topic:
##Two Pointers
https://leetcode.com/explore/featured/card/leetcodes-interview-crash-course-data-structures-and-algorithms/703/arraystrings/4501/
https://algo.monster/problems/two_pointers_intro

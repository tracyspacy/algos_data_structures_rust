### STACK

A stack is dynamic set / linear data structure in which the insertion of a new element and removal of an existing element takes place at the same end represented as the top of the stack.

To implement the stack, it is required to maintain the ****pointer to the top of the stack****, which is the last element to be inserted because ****we can access the elements only on the top of the stack.****

> In a stack the elemetn deleted from the set is the one most recently inserted: the stack implemets a **LAST-IN, FIRST-OUT or LIFO** policy.



##### BASIC OPERATIONS

- ****push()**** to insert an element into the stack
- ****pop()**** to remove an element from the stack
- ****top()**** Returns the top element of the stack.
- ****isEmpty()**** returns true if stack is empty else false.
- ****size()**** returns the size of stack.



![](/Users/tracyspacy/Downloads/stacks.png)

## Working of Stack Data Structure

The operations work as follows:

1. A pointer called TOP is used to keep track of the top element in the stack.
2. When initializing the stack, we set its value to 0 so that we can check if the stack is empty by comparing `TOP == 0` (some impl can have -1).
3. On pushing an element, we increase the value of TOP and place the new element in the position pointed to by TOP.
4. On popping an element, we return the element pointed to by TOP and reduce its value.
5. Before pushing, we check if the stack is already full
6. Before popping, we check if the stack is already empty

![](/Users/tracyspacy/Downloads/stack-operations.png)



#### Types of Stacks:

- ****Fixed Size Stack****: As the name suggests, a fixed size stack has a fixed size and cannot grow or shrink dynamically. If the stack is full and an attempt is made to add an element to it, an overflow error occurs. If the stack is empty and an attempt is made to remove an element from it, an underflow error occurs.
- ****Dynamic Size Stack****: A dynamic size stack can grow or shrink dynamically. When the stack is full, it automatically increases its size to accommodate the new element, and when the stack is empty, it decreases its size. This type of stack is implemented using a linked list, as it allows for easy resizing of the stack.

#### Aplications of STACK

- [Infix to Postfix](https://www.geeksforgeeks.org/stack-set-2-infix-to-postfix/) /Prefix conversion
- Redo-undo features at many places like editors, photoshop.
- Forward and backward features in web browsers
- Used in many algorithms like [Tower of Hanoi,](https://www.geeksforgeeks.org/recursive-functions/) [tree traversals](https://www.geeksforgeeks.org/618/), [stock span problems](https://www.geeksforgeeks.org/the-stock-span-problem/), and [histogram problems](https://www.geeksforgeeks.org/largest-rectangular-area-in-a-histogram-set-1/).
- Backtracking is used to solve problems like the Knight-Tour problem, N-Queen problem, maze problems, and game-like chess or checkers in all these problems we dive into someway if that way is inefficient, we come back to the previous state and go into some another path. To get back from a current state we need to store the previous state in a stack.
- In Graph Algorithms like [Topological Sorting](https://www.geeksforgeeks.org/topological-sorting/) and [Strongly Connected Components](https://www.geeksforgeeks.org/strongly-connected-components/)
- In Memory management, any modern computer uses a stack as the primary management for a running purpose. Each program that is running in a computer system has its own memory allocations.
- Stack also helps in implementing function call in computers. The last called function is always completed first.



#### rust basic implementation of dynamic size STACK

```rust
#[derive(Debug)]
struct Stack <T> {
    stack : Vec<T>,
}

impl<T> Stack<T> {
    fn new()->Self{
       Stack { stack: Vec::new() }
    }
    //insert to top
    fn push(&mut self,item:T) {
        self.stack.push(item)
    }
    //delete from top
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    
    fn top(&self) ->usize{
        self.stack.len()
    }
    //get latest inserted
    fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
    
}


fn main() {
   let mut stack:Stack<i64> = Stack::new();
   println!("{:?}",stack);
   stack.push(7);
   println!("{:?} was pushed",stack.peek());
   println!("top is {:?}",stack.top());
   println!("{:?}",stack);
   stack.push(8);
   println!("{:?} was pushed",stack.peek());
   println!("top is {:?}",stack.top());
   println!("{:?}",stack);
   let p2 = stack.pop();
   println!("Was poped {:?}",p2);
   println!("{:?}",stack);
   let p3 = stack.pop();
   println!("Was poped {:?}",p3);
   println!("{:?}",stack);
   let p4 = stack.pop();
   println!("Was poped {:?}",p4);
   println!("{:?}",stack);
}
/*
returns
Stack { stack: [] }
Some(7) was pushed
top is 1
Stack { stack: [7] }
Some(8) was pushed
top is 2
Stack { stack: [7, 8] }
Was poped Some(8)
Stack { stack: [7] }
Was poped Some(7)
Stack { stack: [] }
Was poped None
Stack { stack: [] }
*/
```



#### Stacks can be also implemented using Linked List






```rust
struct Stack{}
```

...

Source:
[Introduction to Stack - Data Structure and Algorithm Tutorials - GeeksforGeeks](https://www.geeksforgeeks.org/introduction-to-stack-data-structure-and-algorithm-tutorials/)

[Weekly Rust Trivia: How to Implement a Generic Stack &middot; Thorsten Hans' blog](https://www.thorsten-hans.com/weekly-rust-trivia-implement-a-generic-stack/)

[Stack Data Structure and Implementation in Python, Java and C/C++](https://www.programiz.com/dsa/stack)

https://www.hellointerview.com/learn/code/stack

[How to write a Stack in Rust | Writing | Kirill Vasiltsov](https://www.kirillvasiltsov.com/writing/how-to-write-a-stack-in-rust/)

### Binary Trees



A binary tree is a tree data structure in which each node can have at most two children, which are referred to as the left child and the right child.



The topmost node in a binary tree is called the **root**, and the bottom-most nodes are called **leaves**. A binary tree can be visualized as a hierarchical structure with the root at the top and the leaves at the bottom.

Binary trees have many applications in computer science, including data storage and retrieval, expression evaluation, network routing, and game AI. They can also be used to implement various algorithms such as searching, sorting, and graph algorithms.




##### Representation of Binary Tree in Rust

**Node:**

- Data

- Pointer to the left child

- Pointer to the right child 

```
                1
               /  \
              2    3
             / \  / \
            4   5 6  7
```



In Rust, `Box` is commonly used to allocate memory on the heap and create ownership of that memory. However, in the context of binary trees, using `Box` alone might not be sufficient because binary trees often need to share ownership of nodes between multiple parts of the tree => **RC (Reference Counting)** - "provides shared ownership of a value of type `T`, allocated in the heap". It allows multiple parts of your code to share ownership of a value and no one has exclusive ownership.
`RefCell` is a type that provides interior mutability, meaning it allows you to mutate the value inside it even when there are immutable references to that value. This is useful when you need to mutate the contents of a node in the binary tree while traversing it, even when you only have immutable references to the nodes.



> **Rust specific, instead of Box for linked list node, each node in the binary tree is wrapped in an `Rc<RefCell<Node<T>>>`, allowing multiple parts of the tree to share ownership (`Rc`) and enabling interior mutability (`RefCell`) for mutation operations.**

```rust
use std::rc::Rc; //A single-threaded reference-counting pointer. ‘Rc’ stands for ‘Reference Counted’.
use std::cell::RefCell; // 

pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
}
```



##### **Basic Operations On Binary Tree:**

- Inserting an element.
- Removing an element.
- Searching for an element.
- Deletion for an element.
- Traversing an element. There are four (mainly three) types of traversals in a binary tree.

##### **Auxiliary Operations On Binary Tree:**

- Finding the height of the tree
- Find the level of the tree
- Finding the size of the entire tree.



Basic Representation of Binary Tree in Rust

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}
impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    if let Some(left_node) = root.left.as_mut() {
        //same as with root we should make root.left mut to modify
        left_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        // borrowing is obtain mut ref to Refcall containing root.left,
        // ie to access and mutate the left field of the root inside the RefCell contained within the Rc.
    }
    println!("{:?}", root);
}
/*
         1
       /   \
      2      3
    /  \    /   \
  4  NULL  NULL NULL
    /   \
   NULL NULL
*/


```



##### **Binary Tree Traversals:**

```
         1
       /    \
      2       3
    /  \    /   \
  4     5  6     7
```



##### Tree Traversal using Depth-First Search (DFS) algorithm can be further classified into three categories:

- **Preorder Traversal (current-left-right):** 
1. Visit root node
2. Visit all the nodes in the left subtree
3. Visit all the nodes in the right subtree
   
   > Example: 1->2->4->5->3->6->7
- **Inorder Traversal (left-current-right):** 
1. First, visit all the nodes in the left subtree

2. Then the root node

3. Visit all the nodes in the right subtree
   
   > Example: 4->2->5->1->6->3->7
- **Postorder Traversal (left-right-current):** 
1. Visit all the nodes in the left subtree
2. Visit all the nodes in the right subtree
3. Visit the root node
   
   > Example: 4->5->2->6->7->3->1





Source:
[Implementing a Binary Tree in Rust for fun](https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/)

[Introduction to Binary Tree - Data Structure and Algorithm Tutorials - GeeksforGeeks](https://www.geeksforgeeks.org/introduction-to-binary-tree-data-structure-and-algorithm-tutorials/)

[Binary Tree](https://www.programiz.com/dsa/binary-tree)

[Tree Traversal](https://www.programiz.com/dsa/tree-traversal)

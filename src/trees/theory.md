### Trees

A **Tree data structure** is a type of non-linear, hierarchical data structure that consists of nodes connected by edges. 

It follows the parent-child relationship, with the top node being called the **root**. Each node in a tree can have child nodes and each of these child nodes has a single parent node. Nodes with same parents are known as **siblings**. Nodes without any children are referred to as leaves.

Its structure allows the organization of data in a natural hierarchy. The simplification it provides in accessing, managing and manipulating data with complex relationships makes it a vital data structure in computer science. 

#### Why Tree Data Structure?

Other data structures such as arrays, linked list, stack, and queue are linear data structures that store data sequentially. In order to perform any operation in a linear data structure, the time complexity increases with the increase in the data size.

#### Tree Terminologies

##### - Node

A node is an entity that contains a key or value and pointers to its child nodes.

The last nodes of each path are called **leaf nodes or external nodes** that do not contain a link/pointer to child nodes.

The node having at least a child node is called an **internal node**.

##### - Edge

It is the link between any two nodes.

##### - Root

It is the topmost node of a tree.

##### - Height of a Node

The height of a node is the number of edges from the node to the deepest leaf (ie. the longest path from the node to a leaf node).

##### - Depth of a Node

The depth of a node is the number of edges from the root to the node.

##### - Height of a Tree

The height of a Tree is the height of the root node or the depth of the deepest node.

```
                (h=2, d=0)
                    /\
          (h=1, d=1)  (h=0, d=1)
                /\
      (h=0, d=2)  (h=0, d=2)
```

##### - Degree of a Node

The degree of a node is the total number of branches of that node.

#### Tree Traversal

In order to perform any operation on a tree, you need to reach to the specific node. The tree traversal algorithm helps in visiting a required node in the tree.

#### Types of Tree

1. Binary Tree
2. Binary Search Tree
3. AVL Tree
4. B-Tree

#### Tree Applications

Implementations of the tree data structure are seen in databases, file systems, and HTML DOM.

- Binary Search Trees(BSTs) are used to quickly check whether an element is present in a set or not.
- Heap is a kind of tree that is used for heap sort.
- A modified version of a tree called Tries is used in modern routers to store routing information.
- Most popular databases use B-Trees and T-Trees, which are variants of the tree structure we learned above to store their data
- Compilers use a syntax tree to validate the syntax of every program you write.
  
  
  
   

Source:
[Data Structures and Algorithms Roadmap](https://roadmap.sh/datastructures-and-algorithms)

[Tree Data Structure](https://www.programiz.com/dsa/trees)

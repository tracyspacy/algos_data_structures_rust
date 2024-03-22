### Linked List

****Linked List**** is a linear data structure, in which elements are not stored at a contiguous location, rather they are linked using pointers. Linked List forms a series of connected nodes, where each node stores the data and the address of the next node.

**Node Structure:**
A node in a linked list typically consists of two components:  
****Data:**** It holds the actual value or data associated with the node.  
****Next Pointer:**** It stores the memory address (reference) of the next node in the sequence.  
****Head and Tail:**** The linked list is accessed through the head node, which points to the first node in the list. The last node in the list points to NULL or nullptr, indicating the end of the list. This node is known as the tail node.

#### There are mainly three types of linked lists:

1. **Single-linked list**

![Singlelinkedlist.png](/Users/tracyspacy/Downloads/Singlelinkedlist.png)

2. **Double linked list**

![Doublylinkedlist.png](/Users/tracyspacy/Downloads/Doublylinkedlist.png)

3. **Circular linked list**

![Circularlinkedlist.png](/Users/tracyspacy/Downloads/Circularlinkedlist.png)



## Why linked list data structure needed?

- ****Dynamic Data structure:**** The size of memory can be allocated or de-allocated at run time based on the operation insertion or deletion.
- ****Ease of Insertion/Deletion:**** The insertion and deletion of elements are simpler than arrays since no elements need to be shifted after insertion and deletion, Just the address needed to be updated.
- ****Efficient Memory Utilization:**** As we know Linked List is a dynamic data structure the size increases or decreases as per the requirement so this avoids the wastage of memory. 
- ****Implementation:**** Various advanced data structures can be implemented using a linked list like a stack, queue, graph, hash maps, etc.

## Operations on Linked Lists

1. [****Insertion****](https://www.geeksforgeeks.org/insertion-in-linked-list/)****:**** Adding a new node to a linked list involves adjusting the pointers of the existing nodes to maintain the proper sequence. Insertion can be performed at the beginning, end, or any position within the list
2. [****Deletion****](https://www.geeksforgeeks.org/deletion-in-linked-list/)****:**** Removing a node from a linked list requires adjusting the pointers of the neighboring nodes to bridge the gap left by the deleted node. Deletion can be performed at the beginning, end, or any position within the list.
3. [****Searching****](https://www.geeksforgeeks.org/search-an-element-in-a-linked-list-iterative-and-recursive/)****:**** Searching for a specific value in a linked list involves traversing the list from the head node until the value is found or the end of the list is reached.

#### Representation of Linked List in Rust



```rust

#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    //creates node initially without next
    fn new(val: T) -> Self {
        Node {
            value: val,
            next: None,
        }
    }
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    //inits empty linked list
    fn new() -> Self {
        SinglyLinkedList {
            head: None,
            size: 0,
        }
    }
    //inserts to the beginning ie replacing head node
    pub fn push(&mut self, val: T) {
        let mut new_node = Some(Box::new(Node::new(val)));
        match self.head.take() {
            Some(old_head) => {
                new_node.as_mut().unwrap().next = Some(old_head);
                self.head = new_node;
            }
            None => {
                self.head = new_node;
            }
        }
        self.size += 1;
    }
    // deletes from the beginning, ie moving pointer to next node
    pub fn delete(&mut self) {
        match self.head.take() {
            Some(current_head) => {
                self.head = current_head.next;
                self.size -= 1;
            }
            None => {
                println!("List is Empty");
            }
        }
    }

    //checks if value exists in linked list
    pub fn search(&self, head: &Option<Box<Node<T>>>, val: T) -> bool
    where
        T: PartialEq,
    {
        match head {
            Some(node) => {
                if val == node.value {
                    true
                } else {
                    //recur for remaining list
                    return self.search(&node.next, val);
                }
            }
            None => false,
        }
    }
}

fn main() {
    let mut list = SinglyLinkedList::new();
    list.delete();
    list.push("H");
    list.push("E");
    list.push("L");
    list.push("L");
    list.push("O");
    list.push("!");
    println!("Before deletion {:?}", &list);
    list.delete();
    println!("After deletion {:?}", &list);

    let mut current_node = &list.head;
    println!("{:?}", list.search(current_node, "!"));

    while let Some(node) = current_node {
        println!("Node value: {}", node.value);
        current_node = &node.next;
    }
}
```











Sources:
[Introduction - Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html)

[Understanding the basics of Linked List - GeeksforGeeks](https://www.geeksforgeeks.org/what-is-linked-list/)

[Introduction to Linked List - Data Structure and Algorithm Tutorials - GeeksforGeeks](https://www.geeksforgeeks.org/introduction-to-linked-list-data-structure-and-algorithm-tutorial/?ref=next_article)

https://betterprogramming.pub/learning-rust-building-a-linked-list-102bcb08f93b



additional:
[A zero-overhead linked list in Rust](https://aloso.github.io/2021/04/12/linked-list.html)

[Rust Singly Linked List Implementation | Roka's Developer Blog](https://rtoch.com/posts/rust-singly-linked-list/)

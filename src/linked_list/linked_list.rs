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

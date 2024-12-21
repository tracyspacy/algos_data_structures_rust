/*
Design your implementation of the circular queue. The circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out) principle, and the last position is connected back to the first position to make a circle. It is also called "Ring Buffer".

One of the benefits of the circular queue is that we can make use of the spaces in front of the queue. In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue. But using the circular queue, we can use the space to store new values.

Implement the MyCircularQueue class:

MyCircularQueue(k) Initializes the object with the size of the queue to be k.
int Front() Gets the front item from the queue. If the queue is empty, return -1.
int Rear() Gets the last item from the queue. If the queue is empty, return -1.
boolean enQueue(int value) Inserts an element into the circular queue. Return true if the operation is successful.
boolean deQueue() Deletes an element from the circular queue. Return true if the operation is successful.
boolean isEmpty() Checks whether the circular queue is empty or not.
boolean isFull() Checks whether the circular queue is full or not.
You must solve the problem without using the built-in queue data structure in your programming language.

Example 1:

Input
["MyCircularQueue", "enQueue", "enQueue", "enQueue", "enQueue", "Rear", "isFull", "deQueue", "enQueue", "Rear"]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
Output
[null, true, true, true, false, 3, true, true, true, 4]

Explanation
MyCircularQueue myCircularQueue = new MyCircularQueue(3);
myCircularQueue.enQueue(1); // return True
myCircularQueue.enQueue(2); // return True
myCircularQueue.enQueue(3); // return True
myCircularQueue.enQueue(4); // return False
myCircularQueue.Rear();     // return 3
myCircularQueue.isFull();   // return True
myCircularQueue.deQueue();  // return True
myCircularQueue.enQueue(4); // return True
myCircularQueue.Rear();     // return 4

*/
struct MyCircularQueue {
    queue: Vec<i32>,
    current_size: usize,
    capacity: usize,
    front_element_index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            queue: vec![0; k as usize],
            current_size: 0,
            capacity: k as usize,
            front_element_index: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            let index = (self.front_element_index + self.current_size) % self.capacity;
            self.queue[index] = value;
            self.current_size += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.front_element_index = (self.front_element_index + 1) % self.capacity;
            self.current_size -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.queue[self.front_element_index]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let index = (self.front_element_index + self.current_size - 1) % self.capacity;
            self.queue[index]
        }
    }

    fn is_empty(&self) -> bool {
        self.current_size == 0
    }

    fn is_full(&self) -> bool {
        self.current_size == self.capacity
    }
}

fn main() {
    let mut obj = MyCircularQueue::new(3);
    let ret_1: bool = obj.en_queue(1);
    let ret_2: bool = obj.en_queue(2);
    let ret_3: bool = obj.en_queue(3);
    let ret_4: bool = obj.en_queue(4);

    let ret_5: i32 = obj.rear();
    let ret_6: bool = obj.is_full();
    let ret_7 = obj.de_queue();

    let ret_8 = obj.en_queue(4);
    let ret_0 = obj.front_element_index;
    let ret_9: i32 = obj.rear();

    dbg!(ret_1, ret_2, ret_3, ret_4, ret_0, ret_5, ret_6, ret_7, ret_8, ret_9, obj.queue);
}

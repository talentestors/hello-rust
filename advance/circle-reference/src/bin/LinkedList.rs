struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
struct LinkedList<T> {
    size: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }

    pub fn set_next(&mut self, next: Option<Box<Node<T>>>) {
        self.next = next;
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            size: 0,
            head: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        if let Some(old_head) = self.head.take() {
            new_node.set_next(Some(old_head));
        }
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            self.head = old_head.next;
            self.size -= 1;
            Some(old_head.value)
        } else {
            None
        }
    }

    fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| node.value())
    }

    fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| node.value_mut())
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    println!("List length: {}", list.len());
    println!("Front element: {:?}", list.peek_front());

    if let Some(front) = list.peek_front_mut() {
        *front = 42;
    }

    println!("Front element after mutation: {:?}", list.peek_front());

    while let Some(value) = list.pop_front() {
        println!("Popped value: {}", value);
    }

    println!("List is empty: {}", list.is_empty());
    list.push_front(4);
    list.clear();

    println!("List cleared. Is empty: {}", list.is_empty());
}

use std::clone::Clone;
#[derive(Copy)]
pub struct Node<T: Clone  > {
    pub value: Option<T>,
    pub next: *mut Node<T>,
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T: Clone> AsRef<Node<T>> for Node<T> {
    fn as_ref(&self) -> &Node<T> {
        self
    }
}

impl<T: Clone> Node<T> {
    pub fn new(value: Option<T>) -> Node<T> {
        Node { value, next: std::ptr::null_mut() }
    }
    
    pub fn new_with_next(value: Option<T>, next: *mut Node<T>) -> Node<T> {
        Node { value, next }
    }

    pub fn get(&self) -> &T {
        self.value.as_ref().unwrap()
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn set_next(&mut self, next: *mut Node<T>) -> &Node<T> {
        self.next = next;
        self.next.as_ref()
    }

    pub fn get_next(&self) -> &Node<T> {
        let node = *self.next;
        &node
    }
}

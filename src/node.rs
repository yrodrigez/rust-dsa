use std::clone::Clone;

pub struct Node<T: Clone> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T: Clone> AsMut<Node<T>> for Node<T> {
    fn as_mut(&mut self) -> &mut Node<T> {
        self
    }
}
impl<T: Clone> AsRef<Node<T>> for Node<T> {
    fn as_ref(&self) -> &Node<T> {
        self
    }
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: Option::None }
    }
    
    pub fn new_with_next(value: T, next: Node<T>) -> Node<T> {
        Node { value,  next: Option::Some(Box::from(next))}
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) -> &T{
        self.value = value;
        &self.value
    }

    pub fn set_next(&mut self, next: Node<T>) -> &Node<T> {
        self.next = Option::Some(Box::new(next));
        &self.next.unwrap()
    }

    pub fn get_next(&self) -> &Option<Box<Node<T>>> {
        &self.next
    }
}

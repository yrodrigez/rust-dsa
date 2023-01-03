use std::io::read_to_string;
use crate::node::Node;

pub struct LinkedList<T: Clone> {
    size: i64,
    first: Option<Node<T>>,
}
impl<T: Clone> LinkedList<T> {
    
    pub fn new() -> LinkedList<T> {
        LinkedList{ size: 0, first: Option::None }
    }

    fn get_node_at(&self, index: i64) -> Option<&Node<T>> {
        if index >= self.size || self.size == 0 {
            return None;
        }

        if self.size == 1 {
            return Some(&self.first.unwrap())
        }
    
        let mut found = &self.first;
        for i in 0.. self.size-1 {
            if i == index {
                return Some(&found.unwrap());
            } else {
                let node= found.unwrap().get_next();
                if node.is_none() {
                    return None
                } else {
                    let _box = node.unwrap();
                    found = &Some(*_box);
                }

            }
        }
    
        Some(&found.unwrap())
    }

    pub fn get(&self, index: i64) -> Option<&T> {
        let node = self.get_node_at(index);
        node.map(|node| node.get())
    }

    pub fn add(&mut self, value: T) {
        let new_node = Node::new(value);
        if self.size == 0 {
            self.size += 1;
            self.first = Some(new_node);
        } else {
            let prev = self.get_node_at(self.size -1).unwrap();


            prev.set_next(new_node);
            self.size += 1
        }
    }

}

impl <T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        LinkedList { size: self.size.clone(), first: self.first.clone() }
    }
}
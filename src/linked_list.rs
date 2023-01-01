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
        if index >= self.size {
            return Option::None;
        }
    
        let mut found = self.first.as_ref();
        for i in 0.. self.size {
            if i == index {
                return found;
            } else {
                found = Some(found.unwrap().get_next());
            }
        }
    
        found
    }

    pub fn get(&self, index: i64) -> Option<&T> {
        let node = self.get_node_at(index);
        node.map(|node| node.get())
    }

    pub fn add(&mut self, value: T) {
        let new_node = Box::new(Node::new(Some(value)));
        if self.size == 0 {
            self.size += 1;
            self.first = Some(*new_node);
        } else {
            let prev = self.get_node_at(self.size -1).unwrap();
            let mut prev = *prev;
            prev.set_next(new_node);
            self.size += 1;
        }
    }

}

impl <T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        LinkedList { size: self.size.clone(), first: self.first.clone() }
    }
}
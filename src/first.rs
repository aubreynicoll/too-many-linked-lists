use std::mem;

pub struct List {
    head: Option<Box<Node>>,
}

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, data: i32) {
        let new_node = Node {
            data,
            next: mem::replace(&mut self.head, None),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current = mem::replace(&mut self.head, None);

        while let Some(mut node) = current {
            current = mem::replace(&mut node.next, None);
        }
    }
}

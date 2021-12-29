use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&self, value: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_node = self.head.take();
        while let Some(node) = current_node {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                current_node = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.value
        })
    }
}

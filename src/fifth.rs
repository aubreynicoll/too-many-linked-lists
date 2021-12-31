use std::ptr;

pub struct List<T> {
    head: Option<Link<T>>,
    tail: *mut Node<T>,
}

type Link<T> = Box<Node<T>>;

struct Node<T> {
    value: T,
    next: Option<Link<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, value: T) {
        let mut new_tail = Node::new(value).as_link();
        let new_tail_raw = &mut *new_tail as *mut _;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        self.tail = new_tail_raw;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            old_head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|head| &mut head.value)
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }

    fn as_link(self) -> Link<T> {
        Box::new(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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
        self.0.map(|current_head| {
            self.0 = current_head.next.as_deref();
            &current_head.value
        })
    }
}

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|head| {
            self.0 = head.next.as_deref_mut();
            &mut head.value
        })
    }
}

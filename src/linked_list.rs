#![allow(unused)]

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

pub type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            val: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                self.len -= 1;
                Some(node.val)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_works() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_push_pop_works() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek_works() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek_mut(), Some(&mut 2));
        assert_eq!(list.peek(), Some(&2));

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek_mut(), Some(&mut 1));
        assert_eq!(list.peek(), Some(&1));

        list.pop();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek(), None);
    }
}

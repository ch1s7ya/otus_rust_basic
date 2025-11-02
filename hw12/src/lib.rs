use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

pub struct LinkedList<T> {
    head: Option<Link<T>>,
}

struct Node<T> {
    data: T,
    next: Option<Link<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    pub fn insert_after(&mut self, n: usize, data: T) {
        let mut current = self.head.as_ref().cloned();
        for _ in 0..n {
            if let Some(node) = current {
                current = node.borrow().next.as_ref().cloned();
            } else {
                return;
            }
        }
        if let Some(nth_node) = current {
            let next_node = nth_node.borrow().next.as_ref().cloned();

            let new_node = Rc::new(RefCell::new(Node {
                data,
                next: next_node,
            }));

            nth_node.borrow_mut().next = Some(new_node);
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.as_ref().cloned(),
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.current.take();
        if let Some(node) = current_node {
            self.current = node.borrow().next.as_ref().cloned();
            Some(node.borrow().data.clone())
        } else {
            None
        }
    }
}

pub struct Iter<T> {
    current: Option<Link<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_push() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_after_middle() {
        let mut list = LinkedList::new();
        list.push(3);
        list.push(1);

        list.insert_after(0, 2);
        let result: Vec<i32> = list.iter().collect();

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_after_end() {
        let mut list = LinkedList::new();
        list.push(2); // 2
        list.push(1); // 1 -> 2

        list.insert_after(1, 3);
        let result: Vec<i32> = list.iter().collect();

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_after_out_of_bounds() {
        let mut list = LinkedList::new();
        list.push(2);
        list.push(1);

        list.insert_after(10, 99);
        let result: Vec<i32> = list.iter().collect();

        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_insert_after_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.insert_after(0, 1);
        let result: Vec<i32> = list.iter().collect();

        assert_eq!(result, vec![]);
    }
}

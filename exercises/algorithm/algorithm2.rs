/*
    double linked list reverse
    This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn reverse(&mut self) {
        let mut current = self.start;
        while let Some(mut pointer) = current {
            // SAFETY: &mut self grants exclusive access to the list's live
            // nodes. Follow the old next link after swapping both directions.
            unsafe {
                let node = pointer.as_mut();
                current = node.next;
                std::mem::swap(&mut node.next, &mut node.prev);
            }
        }
        std::mem::swap(&mut self.start, &mut self.end);
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.start {
            // SAFETY: The list uniquely owns each Box allocation. Traversing
            // next releases each node exactly once, including after reversal.
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.start = node.next;
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn reverse_empty_singleton_twice_and_append() {
        let mut list = LinkedList::new();
        list.reverse();
        assert!(list.start.is_none());
        assert!(list.end.is_none());
        list.add(1);
        list.reverse();
        assert_eq!(list.get(0), Some(&1));
        list.add(2);
        list.add(3);
        list.reverse();
        // SAFETY: The three nodes are still owned by list and live here.
        unsafe {
            let first = list.start.unwrap();
            assert!(first.as_ref().prev.is_none());
            let second = first.as_ref().next.unwrap();
            assert_eq!(second.as_ref().prev, Some(first));
            let last = second.as_ref().next.unwrap();
            assert_eq!(last.as_ref().prev, Some(second));
            assert_eq!(list.end, Some(last));
            assert!(last.as_ref().next.is_none());
        }
        list.reverse();
        list.add(4);
        assert_eq!(list.length, 4);
        for index in 0..4 {
            assert_eq!(list.get(index), Some(&(index + 1)));
        }
    }

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}

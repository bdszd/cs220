//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::fmt::Debug;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value);
        if let Some(node) = self.head.take() {
            new_node.next = Some(Box::new(*node));
        }
        self.head = Some(Box::new(new_node));
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        let mut new_node = Node::new(value);
        let mut curr_node = &mut self.head;
        while let Some(ref mut node) = curr_node {
            curr_node = &mut node.next;
        }
        *curr_node = Some(Box::new(new_node));
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        let mut curr_node = &mut self.head;
        if curr_node.is_none() {
            return None;
        }

        if curr_node.as_ref().unwrap().next.is_none() {
            return self.head.take().map(|node| node.value);
        }

        while let Some(ref mut node) = curr_node {
            if node.next.as_ref().unwrap().next.is_none() {
                let last = node.next.take().unwrap();
                return Some(last.value);
            }
            curr_node = &mut node.next;
        }
        None
    }

    /// Create a new list from the given vector `vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut ret = Self::new();
        for val in vec.into_iter().rev() {
            ret.push_front(val);
        }
        ret
    }

    /// Convert the current list into a vector.
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::new();
        let mut curr_node = self.head;
        while let Some(node) = curr_node {
            ret.push(node.value);
            curr_node = node.next;
        }
        ret
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        let mut count = 0;
        let mut curr_node = self.head.as_ref();

        while let Some(node) = curr_node {
            count += 1;
            curr_node = node.next.as_ref();
        }
        count
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        let mut vec = self.into_vec();
        vec = vec.into_iter().map(f).collect();
        Self::from_vec(vec)
    }

    /// Apply given function `f` for each adjacent pair of elements in the list.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(self, f: F) -> Self
    where
        T: Clone,
    {
        if self.length() < 2 {
            return self;
        }
        let vec = self.into_vec();
        let mut ret = Vec::new();
        for i in 0..vec.len().saturating_sub(1) {
            ret.push(f(vec[i].clone(), vec[i + 1].clone()));
        }
        Self::from_vec(ret)
    }
}

// A list of lists.
impl<T: Debug> SinglyLinkedList<SinglyLinkedList<T>> {
    /// Flatten the list of lists into a single list.
    ///s
    /// # Examples
    /// `self`: `[[1, 2, 3], [4, 5, 6], [7, 8]]`
    /// ==> `[1, 2, 3, 4, 5, 6, 7, 8]`
    pub fn flatten(self) -> SinglyLinkedList<T> {
        let mut vec = Vec::new();
        let mut list = self.into_vec();
        for temp in list.into_iter() {
            let values = temp.into_vec();
            vec.extend(values);
        }
        SinglyLinkedList::from_vec(vec)
    }
}

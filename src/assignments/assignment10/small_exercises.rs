//! Small exercises.

use num::integer::gcd;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

use itertools::*;

/// Returns the pairs of `(i, j)` where `i < j` and `inner[i] > inner[j]` in increasing order.
///
/// For example, the inversions of `[3, 5, 1, 2, 4]` is `[(0, 2), (0, 3), (1, 2), (1, 3), (1, 4)]`
/// because as follows:
///
/// - `0 < 2`, `inner[0] = 3 > 1 = inner[2]`
/// - `0 < 3`, `inner[0] = 3 > 2 = inner[3]`
/// - `1 < 2`, `inner[1] = 5 > 1 = inner[2]`
/// - `1 < 3`, `inner[1] = 5 > 2 = inner[3]`
/// - `1 < 4`, `inner[1] = 5 > 4 = inner[4]`
///
/// Consult <https://en.wikipedia.org/wiki/Inversion_(discrete_mathematics)> for more details of inversion.
pub fn inversion<T: Ord>(inner: Vec<T>) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let len = inner.len();

    for v in 0..len {
        for m in v + 1..len {
            if inner[v] > inner[m] {
                ret.push((v, m));
            }
        }
    }
    ret
}

/// Represents a node of tree data structure.
///
/// Consult <https://en.wikipedia.org/wiki/Tree_(data_structure)> for more details on tree data structure.
#[derive(Debug)]
pub enum Node<T> {
    /// Non-leaf node
    ///
    /// It contains `(the name of node, list of child nodes)`.
    NonLeaf((T, Vec<Node<T>>)),
    /// Leaf node
    ///
    /// It contains the name of node.
    Leaf(T),
}

/// Traverses the tree in preorder.
///
/// The algorithm for preorder traversal is as follows:
///
/// 1. Visit the root.
/// 2. If the root is a leaf node, end the traverse.
/// 3. If the root is a non-leaf node, traverse each subtree from the child nodes.
///
/// For example, the result of preorder traversal for the following tree
///
/// ```text
///     1
///    /|\
///   2 3 4
///  /|  /|\
/// 5 6 7 8 9
/// ```
///
/// which can be represented as
///
/// ```ignore
/// Node::NonLeaf((
///     1,
///     vec![
///         Node::NonLeaf((2, vec![Node::Leaf(5), Node::Leaf(6)])),
///         Node::Leaf(3),
///         Node::NonLeaf((4, vec![Node::Leaf(7), Node::Leaf(8), Node::Leaf(9)])),
///     ]
/// ))
/// ```
///
/// is `1 -> 2 -> 5 -> 6 -> 3 -> 4 -> 7 -> 8 -> 9`.
pub fn traverse_preorder<T>(root: Node<T>) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();

    match root {
        Node::NonLeaf((val, nvec)) => {
            ret.push(val);
            for node in nvec {
                ret.extend(traverse_preorder(node));
            }
            ret
        }
        Node::Leaf(val) => {
            ret.push(val);
            ret
        }
    }
}

/// File
#[derive(Debug)]
pub enum File {
    /// Directory
    ///
    /// It contains `(name of directory, list of files under the directory)`
    ///
    /// The size of a directory is the sum of the sizes of its sub-files.
    Directory(String, Vec<File>),

    /// Data
    ///
    /// It contains `(name of data, size of data)`
    Data(String, usize),
}

/// Given a file, summarize all subfiles and sizes in ascending order of size.
///
/// - Its behaviour is the same as the `du | sort -h` command on Linux.
/// - If the file size is the same, sort it by name.
/// - Assume that there are no duplicate file names.
///
/// # Example
///
/// Input:
///
/// ```txt
/// root (Directory)
/// |
/// |__a (Directory)
/// |  |__a1 (Data, size: 1)
/// |  |__a2 (Data, size: 3)
/// |
/// |__b (Directory)
/// |  |__b1 (Data, size: 3)
/// |  |__b2 (Data, size: 15)
/// |
/// |__c (Data, size: 8)
/// ```
///
/// Output: `[("a1", 1), ("a2", 3), ("b1", 3), ("a", 4), ("c", 8), ("b2", 15), ("b", 18), ("root",
/// 30)]`
pub fn du_sort(root: &File) -> Vec<(&str, usize)> {
    let mut vol: Vec<(&str, usize)> = Vec::new();

    fn helper<'a>(file: &'a File, vol: &mut Vec<(&'a str, usize)>) -> usize {
        match file {
            File::Directory(s, v) => {
                let total: usize = v.iter().map(|f| helper(f, vol)).sum();
                vol.push((s.as_str(), total));
                total
            }
            File::Data(s, size) => {
                vol.push((s.as_str(), *size));
                *size
            }
        }
    }

    let _ = helper(root, &mut vol);
    vol.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));
    vol
}

/// Remove all even numbers inside a vector using the given mutable reference.
/// That is, you must modify the vector using the given mutable reference instead
/// of returning a new vector.
///
/// # Example
/// ```ignore
/// let mut vec = vec![1, 2, 3, 4, 5];
/// remove_even(&mut vec);
/// assert_eq!(*vec, vec![1, 3, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_even(inner: &mut Vec<i64>) {
    inner.retain(|x| x % 2 == 0);
}

/// Remove all duplicate occurences of a number inside the array.
/// That is, if an integer appears more than once, remove some occurences
/// of it so that it only appears once. Note that you must modify the vector
/// using the given mutable reference instead of returning a new vector.
/// Also, note that the order does not matter.
///
/// # Example
/// ```ignore
/// let mut vec = vec![1, 2, 1, 1, 3, 7, 5, 7];
/// remove_duplicate(&mut vec);
/// assert_eq!(*vec, vec![1, 2, 3, 7, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_duplicate(inner: &mut Vec<i64>) {
    let mut dup = HashSet::new();

    inner.retain(|x| dup.insert(*x));
}

/// Returns the natural join of two tables using the first column as the join argument.
/// That is, for each pair of a row(`Vec<String>`) from table1 and a row(`Vec<String>`) from table2,
/// if the first element of them are equal, then add all elements of the row from table2
/// except its first element to the row from table1 and add it to the results.
/// Note that the order of results does not matter.
///
/// # Example
///
/// ```text
///        table1                     table2
/// ----------------------     ----------------------
///  20230001 |    Jack         20230001 |    CS
///  20231234 |    Mike         20230001 |    EE
///                             20231234 |    ME
///
///
///               result
/// -----------------------------------
///  20230001 |    Jack   |     CS
///  20230001 |    Jack   |     EE
///  20231234 |    Mike   |     ME
/// ```
pub fn natural_join(table1: Vec<Vec<String>>, table2: Vec<Vec<String>>) -> Vec<Vec<String>> {
    table1
        .into_iter()
        .flat_map(|t1| {
            table2.iter().flat_map(move |t2| {
                if t1[0] == t2[0] {
                    let mut row = t1.clone();
                    row.extend_from_slice(&t2[1..]);
                    Some(row)
                } else {
                    None
                }
            })
        })
        .collect()
}

/// You can freely add more fields.
struct Pythagorean {
    m: u64,
    n: u64,
    heap: BinaryHeap<Reverse<(u64, u64, u64)>>,
}

impl Pythagorean {
    fn new() -> Self {
        Self {
            m: 2,
            n: 1,
            heap: BinaryHeap::new(),
        }
    }
}

impl Iterator for Pythagorean {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while self.n < self.m {
                if (self.m - self.n) % 2 == 1 && gcd(self.m, self.n) == 1 {
                    let a = self.m * self.m - self.n * self.n;
                    let b = 2 * self.m * self.n;
                    let c = self.m * self.m + self.n;

                    let (a, b) = if a < b { (a, b) } else { (b, a) };
                    self.heap.push(Reverse((c, a, b)));
                }
                self.n += 1;
            }
            self.m += 1;
            self.n = 1;

            if let Some(Reverse(trip)) = self.heap.pop() {
                return Some(trip);
            }
        }
    }
}

/// Generates sequence of unique [primitive Pythagorean triples](https://en.wikipedia.org/wiki/Pythagorean_triple),
/// i.e. (a,b,c) such that a² + b² = c², a and b are coprimes, and a < b. Generate in the increasing
/// order of c.
pub fn pythagorean() -> impl Iterator<Item = (u64, u64, u64)> {
    Pythagorean::new()
}

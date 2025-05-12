//! A small graph library.
//!
//! A node has a i32 value and (directed) edges to other nodes. A node does not have multiple edges
//! to the same node. Nodes are not associated with a particular domain, and users can freely
//! create nodes however they like. However, after a node is created, it can be added to a
//! `SubGraph`, which form a subgraph of the graph of all nodes. A node can be added to multiple
//! subgraphs. `SubGraph` has a method to check if the it has a cycle.
//!
//! The goal of this assignment is to learn how to deal with inherently shared mutable data in
//! Rust. Design the types and fill in the `todo!()`s in methods. There are several possible
//! approaches to this problem and you may import anything from the std library accordingly.
//!
//! Refer `graph_grade.rs` for test cases.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
enum VisitStatus {
    Unvisited,
    Visiting,
    Visited,
}
///Construct an inner node.
#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    edges: RefCell<HashSet<NodeHandle>>,
}
/// Handle to a graph node.
///
/// `NodeHandle` should implement `Clone`, which clones the handle without cloning the underlying
/// node. That is, there can be multiple handles to the same node.
/// The user can access the node through a handle if it does not violate Rust's aliasing rules.
///
/// You can freely add fields to this struct.
#[derive(Debug, Clone)]
pub struct NodeHandle(Rc<Node>);

impl PartialEq for NodeHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0.value == other.0.value
    }
}

impl Eq for NodeHandle {}

impl Hash for NodeHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.value.hash(state);
    }
}

/// Error type for graph operations.
#[derive(Debug)]
pub struct GraphError;

/// Subgraph
///
/// You can freely add fields to this struct.
#[derive(Debug)]
pub struct SubGraph {
    nodes: HashSet<NodeHandle>,
}

impl NodeHandle {
    /// Creates a node and returns the handle to it.
    pub fn new(value: i32) -> Self {
        Self(Rc::new(Node {
            value,
            edges: RefCell::new(HashSet::new()),
        }))
    }

    /// Adds an edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully added.
    /// Returns `Ok(false)` if an edge to `to` already exits.
    pub fn add_edge(&self, to: NodeHandle) -> Result<bool, GraphError> {
        let mut edges = self.0.edges.try_borrow_mut().map_err(|_| GraphError)?;
        Ok(edges.insert(to.clone()))
    }

    /// Removes the edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully removed.
    /// Returns `Ok(false)` if an edge to `to` does not exist.
    pub fn remove_edge(&self, to: &NodeHandle) -> Result<bool, GraphError> {
        let mut edges = self.0.edges.try_borrow_mut().map_err(|_| GraphError)?;
        Ok(edges.remove(to))
    }

    /// Removes all edges.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`.
    pub fn clear_edges(&self) -> Result<(), GraphError> {
        let mut edges = self.0.edges.try_borrow_mut().map_err(|_| GraphError)?;
        edges.clear();
        Ok(())
    }
}

impl Default for SubGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SubGraph {
    /// Creates a new subgraph.
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
        }
    }

    /// Adds a node to the subgraph. Returns true iff the node is newly added.
    pub fn add_node(&mut self, node: NodeHandle) -> bool {
        self.nodes.insert(node)
    }

    /// Removes a node from the subgraph. Returns true iff the node is successfully removed.
    pub fn remove_node(&mut self, node: &NodeHandle) -> bool {
        self.nodes.remove(node)
    }

    /// Returns true iff the subgraph contains a cycle. Nodes that do not belong to this subgraph
    /// are ignored. See <https://en.wikipedia.org/wiki/Cycle_(graph_theory)> for an algorithm.
    pub fn detect_cycle(&self) -> bool {
        #[allow(clippy::mutable_key_type)]
        let mut status = HashMap::new();

        #[allow(clippy::mutable_key_type)]
        fn dfs(
            node: &NodeHandle,
            subgraph: &SubGraph,
            status: &mut HashMap<NodeHandle, VisitStatus>,
        ) -> bool {
            match status.get(node) {
                Some(VisitStatus::Visiting) => return true,
                Some(VisitStatus::Visited) => return false,
                _ => {}
            }
            let _unused = status.insert(node.clone(), VisitStatus::Visiting);
            for neighbor in node.0.edges.borrow().iter() {
                if subgraph.nodes.contains(neighbor) && dfs(neighbor, subgraph, status) {
                    return true;
                }
            }
            let unused = status.insert(node.clone(), VisitStatus::Visited);
            false
        }

        for node in &self.nodes {
            if !status.contains_key(node) && dfs(node, self, &mut status) {
                return true;
            }
        }
        false
    }
}

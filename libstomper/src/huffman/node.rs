//! Node struct used in huffman tree implementation.
//! Node is a recursive struct it should usually be used in boxed context.
//! A nodes children is represented with Option<Box<Node>> where None denotes the lack of children
//! Nodes are ordered in reverse order. See Ord and Eq implementatios below

use std::cmp::Ordering;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{Write, Read};
use std::error::Error;

///Represent a node in Huffman tree
#[derive(Debug, Clone)]
pub struct Node {
    pub prob: u32,
    pub character: char,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    ///Builds a new leaf node(i.e node with no children)
    pub fn new_leaf(prob: u32, character: char) -> Self {
        Node {
            prob,
            character,
            left: None,
            right: None,
        }
    }

    ///Builds a new internal node(i.e node with children)
    pub fn new(
        prob: u32,
        character: char,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    ) -> Self {
        Node {
            prob,
            character,
            left,
            right,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.prob.cmp(&self.prob)
    }
}
impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.prob == other.prob && self.character == other.character
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn nodes_are_in_reverse_order() {
        let n1 = Node::new_leaf(10, '\0');
        let n2 = Node::new_leaf(15, '\0');
        let n3 = Node::new_leaf(20, '\0');
        let mut v = vec![n2.clone(), n1.clone(), n3.clone()];
        v.sort();
        assert_eq!(n3, v[0]);
        assert_eq!(n2, v[1]);
        assert_eq!(n1, v[2]);
    }
}

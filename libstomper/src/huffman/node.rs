//! Node struct used in huffman tree implementation.
//! Node is a recursive struct it should usually be used in boxed context.
//! A nodes children is represented with Option<Box<Node>> where None denotes the lack of children
//! Nodes are ordered in reverse order. See Ord and Eq implementatios below

use std::cmp::Ordering;

///Represent a node in Huffman tree
#[derive(Debug)]
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

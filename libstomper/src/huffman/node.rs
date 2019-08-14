use std::cmp::Ordering;

pub struct Node {
    pub prob: u32,
    pub character: char,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new_leaf(prob: u32, character: char) -> Self {
        Node {
            prob,
            character,
            left: None,
            right: None,
        }
    }
    pub fn new(prob: u32, character: char, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
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
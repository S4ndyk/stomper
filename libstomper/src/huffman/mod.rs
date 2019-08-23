use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::io::prelude::*;
mod node;

use node::Node;

pub struct Huffman;

static NOCHAR: char = '\0';

impl super::Compressor for Huffman {
    /// Not yet implemented
    #[allow(unused_variables)]
    fn encode(input: &mut impl Read, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut huffmantree = Huffman::build_hfm_tree(input);
        let root = huffmantree.pop().expect("No root node found");
        let codemap = Huffman::build_code_map(root);
        Ok(())
    }

    /// Not yet implemented
    #[allow(unused_variables)]
    fn decode(input: &mut impl Read, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Huffman {
    fn build_hfm_tree(input: &mut Read) -> BinaryHeap<Box<Node>> {
        let mut frequencies: HashMap<char, u32> = HashMap::new();
        let mut huffmantree = BinaryHeap::new();
        for byte in input.bytes() {
            let ch = byte.unwrap() as char;
            let count = frequencies.entry(ch).or_insert(0);
            *count += 1;
        }

        for key in frequencies.keys() {
            let prob = *frequencies.get(key).unwrap();
            let ch = *key;
            huffmantree.push(Box::new(Node::new_leaf(prob, ch)));
        }
        while huffmantree.len() > 1 {
            let left = huffmantree.pop().unwrap();
            let right = huffmantree.pop().unwrap();
            let prob = left.prob + right.prob;
            let parent = Node::new(prob, NOCHAR, Some(left), Some(right));
            huffmantree.push(Box::new(parent));
        }
        huffmantree
    }

    fn build_code_map(root: Box<Node>) -> HashMap<u8, String> {
        let mut code_map = HashMap::new();
        dfs(root, String::new(), &mut code_map);
        code_map
    }

}

fn dfs(node: Box<Node>, code: String, code_map: &mut HashMap<u8, String>) {
    if let Some(left) = node.left {
        dfs(left, code.clone() + "0", code_map);
    }
    if let Some(right) = node.right {
        dfs(right, code.clone() + "1", code_map);
    }
    if node.character != NOCHAR {
        code_map.insert(node.character as u8, code);
    };
}

#[cfg(test)]
mod tests {
    use super::Huffman;

    #[test]
    fn huffmantree_builds_correctly() {
        let test_string = String::from("ABAABC");
        let mut huffman_tree = Huffman::build_hfm_tree(&mut test_string.as_bytes());
        assert_eq!(huffman_tree.len(), 1);
        let root = huffman_tree.pop().expect("huffmantree has no root");
        let a = root.left.expect("Node does not exist");
        let nochar = root.right.expect("Node does not exist");
        let b = nochar.right.expect("Node does not exist");
        let c = nochar.left.expect("Node does not exist");
        assert_eq!(a.character, 'A');
        assert_eq!(b.character, 'B');
        assert_eq!(c.character, 'C');
        assert_eq!(a.prob, 3);
        assert_eq!(b.prob, 2);
        assert_eq!(c.prob, 1);
    }

    #[test]
    fn code_map_builds_correctly() {
        let test_string = String::from("ABAABC");
        let mut huffman_tree = Huffman::build_hfm_tree(&mut test_string.as_bytes());
        let root = huffman_tree.pop().expect("Root node does not exist");
        let code_map = Huffman::build_code_map(root);
        let a = code_map.get(&('A' as u8)).expect("A is not in code map");
        let b = code_map.get(&('B' as u8)).expect("B is not in code map");
        let c = code_map.get(&('C' as u8)).expect("C is not in code map");
        assert_eq!(a, "0");
        assert_eq!(b, "11");
        assert_eq!(c, "10");
    }

}

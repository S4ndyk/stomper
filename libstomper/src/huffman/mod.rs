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
        let huffmantree = Huffman::build_hfm_tree(input);
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
        while huffmantree.capacity() > 1 {
            let left = huffmantree.pop().unwrap();
            let right = huffmantree.pop().unwrap();
            let prob = left.prob + right.prob;
            let parent = Node::new(prob, NOCHAR, Some(left), Some(right));
            huffmantree.push(Box::new(parent));
        }
        huffmantree
    }
}

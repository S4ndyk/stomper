use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::io::{Seek, SeekFrom};
use std::io::prelude::*;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
mod node;
mod bitwriter;
mod bitreader;

use bitwriter::BitWriter;
use bitreader::BitReader;
use node::Node;

pub struct Huffman;

static NOCHAR: char = '\0';

impl super::Compressor for Huffman {
    fn encode<R: Read + Seek, W: Write + Seek>(input: &mut R, output: &mut W) -> Result<(), Box<dyn Error>> {
        let bytes = input.seek(SeekFrom::End(0))?;
        input.seek(SeekFrom::Start(0))?;
        let mut huffmantree = Huffman::build_hfm_tree(input);
        input.seek(SeekFrom::Start(0))?;
        let root = huffmantree.pop().expect("No root node found");
        write_tree(&root, output)?;
        output.write_u64::<LE>(bytes)?;
        let codemap = Huffman::build_code_map(root);
        let mut bitwriter = BitWriter::new(output); 
        for ch in input.bytes() {
            let code = codemap.get(&ch?).expect("No such code in codemap");
            bitwriter.write_string(code.clone());
        }
        bitwriter.flush();
        Ok(())
    }

    fn decode<R: Read + Seek, W: Write + Seek>(input: &mut R, output: &mut W) -> Result<(), Box<dyn Error>> {
        let root = read_tree(input)?.expect("No root node found");
        let chars = input.read_u64::<LE>()?;
        let mut bitreader = BitReader::new(input); 
        let mut node = &root;
        let mut charcount = 0;
        while let Some(bit) = bitreader.next_bit() {
            if charcount >= chars {
                break;
            }
            node = match bit {
                true => &node.right.as_ref().unwrap(),
                false => &node.left.as_ref().unwrap(),
            };


            if node.left == None && node.right == None {
                charcount += 1;
                output.write_u8(node.character as u8)?; 
                node = &root;
            }

        };
        Ok(())
    }
}

impl Huffman {
    fn build_hfm_tree(input: &mut dyn Read) -> BinaryHeap<Box<Node>> {
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

fn write_tree(root: &Box<Node>, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        writer.write_u32::<LE>(root.prob)?;
        writer.write_u8(root.character as u8)?;
        if let Some(node) = &root.left {
            writer.write_u8(1)?;
            write_tree(&node, writer)?;
        } else {
            writer.write_u8(0)?;
        }

        if let Some(node) = &root.right {
            writer.write_u8(1)?;
            write_tree(&node, writer)?;
        } else {
            writer.write_u8(0)?;
        }
        Ok(())
}

fn read_tree(reader: &mut impl Read) -> Result<Option<Box<Node>>, Box<dyn Error>> {
    let prob = reader.read_u32::<LE>()?;
    let character = reader.read_u8()? as char;
    let left = match reader.read_u8()? {
        0 => None,
        _i => read_tree(reader)?,
    };

    let right = match reader.read_u8()? {
        0 => None,
        _i => read_tree(reader)?,
    };
    Ok(
        Some(
            Box::new (Node {
                prob,
                character,
                left: left,
                right: right,
            })
        )
    )
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
    }
}

#[cfg(test)]
mod tests {
    use super::Huffman;
    use super::super::Compressor;
    use std::error::Error;
    use tempfile::*;
    use std::fs::File;
    use std::io::{prelude::*, SeekFrom};

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

    #[test]
    fn tree_writes_and_reads_correctly_no1() -> Result<(), Box<dyn Error>>{
        let test_string = String::from("ABAABC");
        let huffman_tree1 = Huffman::build_hfm_tree(&mut test_string.as_bytes()).pop().unwrap();
        let h1 = format!("{:?}", Some(&huffman_tree1));
        let mut temp = tempfile().unwrap();
        super::write_tree(&huffman_tree1, &mut temp)?;
        temp.seek(SeekFrom::Start(0))?;    
        let huffman_tree2 = super::read_tree(&mut temp)?;
        let h2 = format!("{:?}", &huffman_tree2);
        assert_eq!(h1, h2);
        Ok(())
    }

    #[test]
    fn original_and_decompressed_are_same() {
        let mut testfile = File::open("../testfiles/small.txt").unwrap();
        let mut comp = tempfile().unwrap();
        let mut decomp = tempfile().unwrap();

        Huffman::encode(&mut testfile, &mut comp).unwrap();
        comp.seek(SeekFrom::Start(0)).unwrap();
        Huffman::decode(&mut comp, &mut decomp).unwrap();

        let mut decomp_content = String::new();
        let mut testfile_content = String::new();

        decomp.seek(SeekFrom::Start(0)).unwrap();
        testfile.seek(SeekFrom::Start(0)).unwrap();

        decomp.read_to_string(&mut decomp_content).unwrap();
        testfile.read_to_string(&mut testfile_content).unwrap();

        assert_eq!(testfile_content, decomp_content);
    }
}

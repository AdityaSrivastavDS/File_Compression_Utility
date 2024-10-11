use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct HuffmanNode {
    frequency: usize,
    value: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(frequency: usize, value: Option<u8>) -> Self {
        HuffmanNode {
            frequency,
            value,
            left: None,
            right: None,
        }
    }
}

pub fn decompress(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);

    let codes: HashMap<u8, String> = bincode::deserialize_from(&mut reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let mut compressed_data = Vec::new();
    reader.read_to_end(&mut compressed_data)?;

    let huffman_tree = build_huffman_tree_from_codes(&codes);
    let mut output_file = BufWriter::new(File::create(output_file)?);

    let mut current_node = &huffman_tree;
    for byte in compressed_data {
        for bit in byte_to_bits(byte) {
            current_node = if bit == '1' {
                current_node.right.as_ref().unwrap()
            } else {
                current_node.left.as_ref().unwrap()
            };

            if let Some(value) = current_node.value {
                output_file.write_all(&[value])?;
                current_node = &huffman_tree;
            }
        }
    }

    Ok(())
}

fn build_huffman_tree_from_codes(codes: &HashMap<u8, String>) -> HuffmanNode {
    let mut root = HuffmanNode::new(0, None);

    for (value, code) in codes.iter() {
        let mut current_node = &mut root;
        for bit in code.chars() {
            current_node = if bit == '1' {
                current_node.right.get_or_insert_with(|| Box::new(HuffmanNode::new(0, None)))
            } else {
                current_node.left.get_or_insert_with(|| Box::new(HuffmanNode::new(0, None)))
            };
        }
        current_node.value = Some(*value);
    }

    root
}

fn byte_to_bits(byte: u8) -> Vec<char> {
    (0..8).rev().map(|i| if (byte & (1 << i)) != 0 { '1' } else { '0' }).collect()
}

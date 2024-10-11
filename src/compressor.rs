use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash)]
struct HuffmanNode {
    frequency: usize,
    value: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
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

fn build_huffman_tree(frequencies: &HashMap<u8, usize>) -> HuffmanNode {
    let mut heap: BinaryHeap<Box<HuffmanNode>> = BinaryHeap::new();

    for (value, &frequency) in frequencies.iter() {
        heap.push(Box::new(HuffmanNode::new(frequency, Some(*value))));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let mut combined = HuffmanNode::new(left.frequency + right.frequency, None);
        combined.left = Some(left);
        combined.right = Some(right);

        heap.push(Box::new(combined));
    }

    *heap.pop().unwrap()
}

fn generate_codes(node: &HuffmanNode, prefix: &str, codes: &mut HashMap<u8, String>) {
    if let Some(value) = node.value {
        codes.insert(value, prefix.to_string());
        return;
    }

    if let Some(ref left) = node.left {
        generate_codes(left, &(prefix.to_owned() + "0"), codes);
    }

    if let Some(ref right) = node.right {
        generate_codes(right, &(prefix.to_owned() + "1"), codes);
    }
}

pub fn compress(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut frequencies = HashMap::new();
    for byte in &buffer {
        *frequencies.entry(*byte).or_insert(0) += 1;
    }

    let huffman_tree = build_huffman_tree(&frequencies);
    let mut codes = HashMap::new();
    generate_codes(&huffman_tree, "", &mut codes);

    let mut compressed_data = Vec::new();
    for byte in buffer {
        if let Some(code) = codes.get(&byte) {
            compressed_data.extend_from_slice(&convert_bits_to_bytes(code));
        }
    }

    let mut output_file = BufWriter::new(File::create(output_file)?);
    bincode::serialize_into(&mut output_file, &codes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    output_file.write_all(&compressed_data)?;

    Ok(())
}

fn convert_bits_to_bytes(bits: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut current_byte = 0;
    let mut bit_count = 0;

    for bit in bits.chars() {
        current_byte = (current_byte << 1) | if bit == '1' { 1 } else { 0 };
        bit_count += 1;

        if bit_count == 8 {
            bytes.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    if bit_count > 0 {
        current_byte <<= 8 - bit_count;
        bytes.push(current_byte);
    }

    bytes
}

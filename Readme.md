# File Compression Utility

A simple file compression utility implemented in Rust using Huffman Coding. This utility supports both compression and decompression of files and uses binary serialization for storing the Huffman codes.

## Features

- **Compression**: Compresses files using Huffman Coding.
- **Decompression**: Decompresses files back to their original form.
- **Binary Serialization**: Uses `serde` and `bincode` for efficient storage of Huffman codes.

## Project Structure

<pre>
file_compression_utility/
│
├── src/
│   ├── main.rs        <!-- Entry point of the program -->
│   ├── compressor.rs  <!-- Logic for compressing files using Huffman coding -->
│   └── decompressor.rs <!-- Logic for decompressing files using Huffman coding -->
│
├── Cargo.toml         <!-- Cargo manifest file -->
└── README.md          <!-- Project readme -->
</pre>


### Files

- `main.rs`: Handles the main logic for running the compression and decompression commands.
- `compressor.rs`: Contains functions to compress files using Huffman coding.
- `decompressor.rs`: Contains functions to decompress files using Huffman coding.
- `Cargo.toml`: Contains the configuration for the Rust project and its dependencies.

## Installation

### Prerequisites

Before running this program, you need to have the following installed on your machine:

- [Rust](https://www.rust-lang.org/): If you don't have it installed, you can install Rust by following the official instructions.

### Clone the Repository

1. First, clone this repository to your local machine:

```bash
git clone https://github.com/AdityaSrivastavDS/File_Compression_Utility
```

2. Navigate into the project directory:

```bash
cd File_Compression_Utility
```

3. Build the Project
To build the project, run the following command:

```bash
cargo build --release
```
This will compile the program and prepare it for execution.

## Usage
This utility accepts two commands: compress and decompress.

### Compressing a File
To compress a file, use the following command:

```bash
cargo run compress <input_file> <output_file>
Replace <input_file> with the path to the file you want to compress, and <output_file> with the desired output file path for the compressed file.
```

Example:

```bash
cargo run compress input.txt compressed.dat
```
### Decompressing a File
To decompress a file, use the following command:


```bash
cargo run decompress <input_file> <output_file>
Replace <input_file> with the path to the compressed file and <output_file> with the desired output file path for the decompressed file.
```

### Example:

```bash
cargo run decompress compressed.dat decompressed.txt
```

# How It Works

1. Compression
- File Reading: The input file is read into memory.
- Frequency Calculation: The frequency of each byte in the file is calculated.
- Huffman Tree Construction: A Huffman Tree is built based on the frequencies.
- Code Generation: Huffman codes for each byte are generated.
- File Compression: The file is compressed by encoding the bytes using the generated Huffman codes and writing the result to the output file.

2. Decompression
- Code Deserialization: The Huffman codes used for compression are deserialized from the file.
- Huffman Tree Rebuilding: A Huffman tree is reconstructed using the deserialized codes.
- File Decompression: The compressed file is read and decompressed using the Huffman tree, reconstructing the original content.

## Dependencies
- serde: Used for serializing and deserializing the Huffman tree.
- bincode: Used for efficient binary serialization and deserialization.
These dependencies are defined in the Cargo.toml file.

## Contributing
Contributions to this project are welcome! If you find any bugs or want to add new features, feel free to fork the repository and submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
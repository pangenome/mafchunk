# MAFChunk

MAFChunk is a Rust-based command-line tool designed to process MAF (Multiple Alignment Format) files, specifically to split alignment blocks into smaller, specified lengths. This functionality is particularly useful for handling chromosome-length blocks that are output by tools like wgatools pseudomaf, which generates MAF files relative to a reference sequence.

## Features

- **Split MAF Blocks**: Splits each alignment block in the MAF file into chunks of a user-defined length.
- **Preserve Alignment Information**: While splitting the blocks, it ensures that the essential alignment information (such as the source, start, size, strand, and sequence) is accurately maintained and represented in the resulting chunks.

## Requirements

- Rust Programming Language: The tool is written in Rust, so you will need the Rust compiler to compile the source code. You can install Rust through [rustup](https://rustup.rs/).

## Installation

To install MAFChunk, follow these steps:

1. Clone the repository or download the source code to your local machine.
2. Navigate to the directory containing the `Cargo.toml` file.
3. Compile the project using Cargo, Rust's package manager and build system, by running the command `cargo build --release`.
4. The executable will be located in `target/release/` directory.

## Usage

To use MAFChunk, you need to provide it with an input MAF file and the desired split length. The command-line syntax is as follows:

```
mafchunk <input_file.maf> <split_length>
```

- `<input_file.maf>`: The path to the input MAF file you wish to process.
- `<split_length>`: The desired length of each chunk. This must be an integer value.

Example:

```
./mafchunk example.maf 1000
```

This command will process `example.maf`, splitting its blocks into chunks of 1000 base pairs each.

## Output

MAFChunk writes its output directly to the standard output (stdout). You can redirect this output to a file using shell redirection:

```
./mafchunk example.maf 1000 > output.maf
```

## License

MAFChunk is released under the MIT License. See the LICENSE file for more details.

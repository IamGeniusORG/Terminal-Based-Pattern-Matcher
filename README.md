# Terminal-Based Pattern Matcher

A blazing fast, multi-threaded, memory-safe CLI text search tool (a `grep` clone) written in Rust.

## Features
- **Regex Support**: Search using powerful regular expressions.
- **Multi-threaded**: Scales across all CPU cores using `rayon` for massive performance on directory scans.
- **Recursive Directory Traversal**: Effortlessly search through entire folders using `walkdir`.
- **Memory Safe**: Utilizes `BufReader` to maintain a tiny, constant memory footprint regardless of file size.
- **Colorized Output**: Easy-to-read, beautifully formatted terminal output.

## Installation
Ensure you have Rust installed. Clone this repository and build the release binary:
```bash
cargo build --release
```
The optimized executable will be located in the `target/release/` directory.

## Usage
Search for a specific pattern within a directory or file:
```bash
pattern_matcher "your_regex_pattern" "path/to/search"
```

For case-insensitive search:
```bash
pattern_matcher "your_regex_pattern" "path/to/search" -i
```

View the help menu:
```bash
pattern_matcher --help
```

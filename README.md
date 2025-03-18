# minigrep

A text search CLI tool in Rust

## Installation

```bash
cargo install --path .
```

## Usage

+ case sensitive mode

```bash
minigrep body tests/test.txt
```

+ case insensitive mode

```bash
IGNORE_CASE=true minigrep body tests/test.txt
```

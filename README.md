# minigrep

A text search CLI tool in Rust

## Usage

+ case sensitive

```bash
cargo run -- body tests/test.txt
```

+ case insensitive

```bash
IGNORE_CASE=true cargo run -- body tests/test.txt
```

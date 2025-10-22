# flashgrep

A simple grep-like tool written in Rust.

## Usage

```bash
cargo run -- <query> <file>
```

## Examples

Search for "to" in poem.txt:
```bash
cargo run -- to poem.txt
```

Case-insensitive search:
```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

## Environment Variables

- `IGNORE_CASE`: Set to `1` or `true` for case-insensitive search

## Building

```bash
cargo build
```

## Running Tests

```bash
cargo test
```

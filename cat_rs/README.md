# cat_rs
A simple Rust implementation of the classic Unix cat command, with optional line numbering.

## Features
- Display the contents of one or more files in the terminal.
- Supports reading from stdin if - is passed as a file.
- Optional line numbering with -n.

## Usage
```bash
cargo run -- [OPTIONS] [FILES]...
```

```bash
cargo run -- --help
```
## Examples
```bash
cargo run Cargo.toml
```

```bash
cargo Cargo.toml | cargo run -- -b
```

```bash
cargo run -- -E -b README.md
```

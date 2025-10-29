# head_rs 🦀

A simple Rust implementation of the Unix `head` command.

## Usage

```bash
cargo run -- [OPTIONS] <FILE>...
```

### Options

| Flag | Description |
|------|--------------|
| `-n, --lines <LINES>` | Show the first N lines (default: 10) |
| `-c, --bytes <BYTES>` | Show the first N bytes (conflicts with `--lines`) |
| `-v, --verbose` | Print filenames before content when multiple files are provided |

### Examples

```bash
# Show first 10 lines of file.txt
cargo run -- file.txt

# Show first 5 lines
cargo run -- -n 5 file.txt

# Show first 100 bytes
cargo run -- -c 100 file.txt

# Show multiple files verbosely
cargo run -- -v -n 3 file1.txt file2.txt
```

### Notes

- By default, prints the first 10 lines if no options are specified.
- `--bytes` and `--lines` cannot be used together.

# wc_rs - A simple Rust clone of `wc`

## Overview

`wc_rs` is a lightweight Rust implementation of the Linux `wc` command, allowing you to count lines, words, bytes, characters, and find the maximum line length in files.

## Features

* Count lines (`-l`)
* Count words (`-w`)
* Count bytes (`-c`)
* Count characters (`-m`)
* Find maximum line length (`-L`)
* Handles multiple files and prints a total summary

## Usage

```bash
cargo run -- [OPTIONS] <FILES>
```

### Options

* `-l` / `--lines` : Show number of lines
* `-w` / `--words` : Show number of words
* `-c` / `--bytes` : Show number of bytes
* `-m` / `--chars` : Show number of characters
* `-L` / `--max-line-length` : Show maximum line length

### Example

```bash
cargo run -- -l -w -c Cargo.toml
```

Output:

```
7 19 120 Cargo.toml
```

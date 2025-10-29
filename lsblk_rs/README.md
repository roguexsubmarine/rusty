# lsblk_rs

A simple Rust clone of the Linux `lsblk` command — lists block devices, their partitions, sizes, and mount points using data from `/sys/block` and `/proc/mounts`.

## Usage

```bash
cargo run --release
```

**Options:**

```bash
# Show human-readable sizes (default)
cargo run

# Show sizes in bytes
cargo run -- --bytes

# Hide headers
cargo run -- --noheadings

# Combine options
cargo run -- --bytes --noheadings
```

## Example Output

```
NAME         MAJ:MIN  RM  SIZE        RO  TYPE  MOUNTPOINTS
sda          8:0      0   238.5G      0   disk  
├─sda1       8:1      0   512M        0   part  /boot/efi
├─sda2       8:2      0   237.9G      0   part  /
```
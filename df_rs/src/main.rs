use clap::Parser;
use std::io;
use std::fs;
use nix::sys::statvfs;

#[derive(Parser, Debug)]
#[command(author, version, about)]

struct Args {

    #[arg(short = 'T', long = "print-type")]
    print_type: bool,

    #[arg(short = 'H', long = "human-readable")]
    human_readable: bool,

}

#[derive(Debug)]
struct FileSystem {
    name: String,
    blocks: String,
    used: String,
    available: String,
    use_percentage: String,
    mountpoint: String,
    filesystem_type: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let fs_details = get_filesystems()?;
    print_fs_details(&fs_details, &args);

    Ok(())
}

const FS_TYPES: [&str; 14] = [
    "ext4",
    "ext3",
    "ext2",
    "xfs",
    "btrfs",
    "vfat",
    "ntfs",
    "f2fs",
    "swap",
    "tmpfs",
    "devtmpfs",
    "zfs",
    "exfat",
    "efivarfs",
];

fn get_filesystems() -> io::Result<Vec<FileSystem>> {
    let mut fs_details = Vec::new();

    let path = "/proc/mounts";
    
    for entry in fs::read_to_string(path)?.lines() {
        let parts: Vec<&str> = entry.split_whitespace().collect();
        if parts.len() >= 3 {
            let fs_type = parts[2];
            if FS_TYPES.contains(&fs_type) {
                fs_details.push(FileSystem {
                    name: parts[0].to_string(),
                    blocks: "-".to_string(),
                    used: "-".to_string(),
                    available: "-".to_string(),
                    use_percentage: "-".to_string(),
                    mountpoint: parts[1].to_string(),
                    filesystem_type: parts[2].to_string(),
                });

                if let Some((total, used, avail, use_perc)) = get_fs_usage(parts[1]) {
                    if let Some(fs) = fs_details.last_mut() {
                        fs.blocks = total.to_string();
                        fs.used = used.to_string();
                        fs.available = avail.to_string();
                        fs.use_percentage = format!("{}%", use_perc);
                    }
                }
            }
        }
    }

    Ok(fs_details)
}

fn get_fs_usage(mount_point: &str) -> Option<(u64, u64, u64, u64)> {
    if let Ok(stats) = statvfs::statvfs(mount_point) {
        let total = stats.blocks() * stats.block_size() as u64;
        let free = stats.blocks_free() * stats.block_size() as u64;
        let avail = stats.blocks_available() * stats.block_size() as u64;
        let used = total - free;
        Some((total / 1024, used / 1024, avail / 1024, used * 100 / total))
    } else {
        None
    }
}

fn print_fs_details(fs_details: &Vec<FileSystem>, args: &Args) {
    println!(
        "{:>16} {:>8} {:>10} {:>10} {:>10} {:>6}  {}",
        "Filesystem",
        if args.print_type { "Type" } else { "" },
        if args.human_readable { "Size" } else { "1K-Blocks" },
        "Used",
        "Avail",
        "Use%",
        "Mounted on"
    );

    for fs in fs_details {
        println!(
            "{:>16} {:>8} {:>10} {:>10} {:>10} {:>6}  {}",
            fs.name,
            if args.print_type { fs.filesystem_type.clone() } else { "".to_string() },
            if args.human_readable { human_readable_size(fs.blocks.clone()) } else { fs.blocks.clone() },
            if args.human_readable { human_readable_size(fs.used.clone()) } else { fs.used.clone() },
            if args.human_readable { human_readable_size(fs.available.clone()) } else { fs.available.clone() },
            fs.use_percentage,
            fs.mountpoint,
        );
    }
}

fn human_readable_size(size: String) -> String {
    let units = ["B", "K", "M", "G", "T", "P", "E"];

    let mut size: f64 = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => return "Invalid number".to_string(),
    };

    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1}{}", size, units[unit_index])
}

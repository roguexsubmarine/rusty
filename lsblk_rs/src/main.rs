use clap::Parser;
use std::io;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about)]

struct Args {

    #[arg(short = 'b', long = "bytes")]
    bytes: bool,

    #[arg(short = 'n', long = "noheadings")]
    noheading: bool, 
}

#[derive(Debug)]
#[derive(Clone)]
struct BlockDevice {
    name: String,
    dev_id: String,
    rm: String,
    size_bytes: String,
    ro: String,
    dev_type: String,
    mountpoints: Option<Vec<String>>,
    partitions: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let devices = list_block_devices()?;
    // println!("{:?}", devices);

    let mut block_devices: Vec<BlockDevice> = Vec::new();
    get_block_details(&devices, &mut block_devices)?;
    // println!("{:#?}", block_devices);
    print_block_details(&block_devices, &args);



    Ok(())
}

fn list_block_devices() -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    let path = "/sys/block";
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        names.push(entry.file_name().into_string().unwrap());
    }
    Ok(names)
}

fn get_block_details(devices: &Vec<String>, block_devices: &mut Vec<BlockDevice>) -> io::Result<()> {

    for device_name in devices {
        let device = BlockDevice {
            name: device_name.to_string(),
            dev_id: read_file_to_string(&format!("/sys/block/{}/dev", device_name))?,
            rm: read_file_to_string(&format!("/sys/block/{}/removable", device_name))?,
            size_bytes: read_file_to_string(&format!("/sys/block/{}/size", device_name))?,
            ro: read_file_to_string(&format!("/sys/block/{}/ro", device_name))?,
            dev_type: get_disk_type(device_name)?, //read devtype from /sys/block/{}/device/uevent - DISKTYPE=
            mountpoints: get_mount_point(device_name).ok(), // read mountpoint from /proc/mounts
            partitions: get_partitions(device_name)? // read partitions from /sys/block/{}/
        };

        let partition_names = device.partitions.clone();
        block_devices.push(device);

        for partition_name in partition_names.iter() {
            let partition = BlockDevice {
                name: partition_name.to_string(),
                dev_id: read_file_to_string(&format!("/sys/block/{}/{}/dev" , device_name, partition_name))?,
                rm: read_file_to_string(&format!("/sys/block/{}/removable" , device_name))?, // partitions inherit removable from parent device
                size_bytes: read_file_to_string(&format!("/sys/block/{}/{}/size" , device_name, partition_name))?,
                ro: read_file_to_string(&format!("/sys/block/{}/{}/ro" , device_name, partition_name))?,
                dev_type: "part".to_string(),
                mountpoints: get_mount_point(partition_name).ok(),
                partitions: Vec::new(),
            };
            block_devices.push(partition);
        }

    }
    Ok(())
}

fn get_mount_point(device_name:&str) -> io::Result<Vec<String>> {
    let mounts_content = read_file_to_string("/proc/mounts")?;
    let mut mount_points = Vec::new();
    for line in mounts_content.lines() {
        let cols = line.split_whitespace().collect::<Vec<&str>>();

        if cols.len() < 2 {
            continue;
        }
        if cols[0] == format!("/dev/{}", device_name) {
            mount_points.push(cols[1].to_string());
        }
    }
    Ok(mount_points)
}

fn get_disk_type(device_name: &str) -> io::Result<String> {
    let uevent_content = read_file_to_string(&format!("/sys/block/{}/uevent", device_name))?;
    for line in uevent_content.lines() {
        if line.starts_with("DEVTYPE=") {
            let dev_type = line.trim_start_matches("DEVTYPE=").to_string();
            return Ok(dev_type);
        }
    }
    Ok(String::new())
}

fn print_block_details(devices: &Vec<BlockDevice>, args: &Args) {
    let mut block_devices = devices.clone();

    block_devices.sort_by_key(|d| d.dev_id.clone());

    //handle n (header) part
    if !args.noheading {
        println!("{:<12} {:<8} {:<2} {:<12} {:<2} {:<4} {:<15}", "NAME", "MAJ:MIN", "RM", "SIZE", "RO", "TYPE", "MOUNTPOINTS");
    }

    for device in block_devices {

        //handle partition prefix
        let mut dev_name = device.name.clone();
        if device.dev_type == "part" {
            dev_name = format!("├─{}", dev_name);
        }

        //handle size conversion
        let size: String;
        if !args.bytes {
            let size_in_bytes = convert_size_to_bytes(&device.size_bytes).unwrap_or(0);
            size = convert_bytes_to_human_readable(size_in_bytes);
        } else {
            let size_in_bytes = convert_size_to_bytes(&device.size_bytes).unwrap_or(0);
            size = size_in_bytes.to_string();
        }

        println!("{:<12} {:<8} {:<2} {:<12} {:<2} {:<4} {:<15}", dev_name, device.dev_id, device.rm, size, device.ro, device.dev_type, device.mountpoints.expect("").join("\n\t\t\t\t\t      "));
    }
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(s.trim().to_string()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e),
    }
}

fn get_partitions(device_name: &str) -> io::Result<Vec<String>> {
    let mut partitions = Vec::new();
    let path = format!("/sys/block/{}/", device_name);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with(device_name) && file_name != *device_name {
            partitions.push(file_name);
        }
    }
    Ok(partitions)
}

fn convert_size_to_bytes(size_str: &str) -> io::Result<u64> {
    let size_in_sectors: u64 = size_str.trim().parse().unwrap_or(0);
    let bytes = size_in_sectors * 512; // Assuming 512 bytes per sector
    Ok(bytes)
}

fn convert_bytes_to_human_readable(bytes: u64) -> String {
    let units = ["B", "K", "M", "G", "T", "P", "E"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}{}", size as u64, units[unit_index])
    } else {
        format!("{:.1}{}", size, units[unit_index])
    }
}
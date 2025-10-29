use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]

struct Args {
    #[arg(value_name = "FILE")]
    files: Vec<String>,
    
    #[arg(short = 'c', long = "bytes", conflicts_with = "lines")]
    bytes: Option<usize>,
    
    #[arg(short = 'n', long = "lines", conflicts_with = "bytes")]
    lines: Option<usize>,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if args.bytes == Some(0) && args.lines == Some(0) {
        args.lines = Some(10);
    }

    let lines_to_show = args.lines.unwrap_or(10);
    let bytes_to_show = args.bytes;

    for filename in &args.files {
        let file = File::open(filename)?;
        analyze(BufReader::new(file), &args, filename, lines_to_show, bytes_to_show)?;
        print!("{}", '\n');
    }

    Ok(())
}

fn analyze<R: BufRead>(reader: R, args: &Args, filename: &str, lines_to_show: usize, bytes_to_show: Option<usize>) -> io::Result<()> {
    if args.verbose {
        println!("==> {} <==", filename);
    }

    let mut line_no = 0;
    let mut bytes_count = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let line_len = line.len() + 1;

        // handle bytes (-c option)
        if let Some(limit) = bytes_to_show {
            if bytes_count + line_len > limit {
                let remaining = limit - bytes_count;
                print!("{}", &line[..remaining.min(line.len())]);
                break;
            }
            println!("{}", line);
            bytes_count += line_len;
            if bytes_count >= limit {
                break;
            }
        } else {
            // Handle lines (-n or default)
            println!("{}", line);
            line_no += 1;
            if line_no >= lines_to_show {
                break;
            }
        }
    }

    Ok(())

}
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]

struct Args {
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    #[arg(short = 'm', long = "chars")]
    chars: bool,
    
    #[arg(short = 'l', long = "lines")]
    lines: bool,
    
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    #[arg(short = 'L', long = "max-line-length")]
    max_line_length: bool,
    
    #[arg(short = 'w', long = "words")]
    words: bool,
}

#[derive(Default)]
struct FileStats {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if !args.lines && !args.words && !args.bytes && !args.chars && !args.max_line_length {
        // default behavior (if not flags are set)
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut totalstat = FileStats::default();

    for filename in &args.files {
        let file = File::open(filename)?;

        let mut filestat = FileStats::default();

        analyze_file(BufReader::new(file), &mut filestat)?;
        print_file_stats(&mut filestat, &filename, &args);
        add_file_stats(&mut totalstat, &mut filestat);
    }

    print_file_stats(&mut totalstat, "total", &args);

    Ok(())
}

fn analyze_file<R: BufRead>(reader: R, filestat: &mut FileStats) -> io::Result<()> {


    for line_result in reader.lines(){
        let line = line_result?;

        filestat.lines += 1;
        filestat.chars += line.chars().count() + 1; //add newline byte manually
        filestat.bytes += line.as_bytes().len() + 1; //add newline byte manually
        filestat.words += line.split_whitespace().count();
        filestat.max_line_length = std::cmp::max(filestat.max_line_length, line.len());
    }
    Ok(())
}

fn print_file_stats(filestat: &mut FileStats, filename: &str, args: &Args){
    
    if args.lines { print!("{:>3} ", filestat.lines); }
    if args.words { print!("{:>3} ", filestat.words); }
    if args.chars { print!("{:>3} ", filestat.chars); }
    if args.bytes { print!("{:>3} ", filestat.bytes); }
    if args.max_line_length { print!("{:>3} ", filestat.max_line_length) };

    print!("{}", filename);
    println!();
}

fn add_file_stats(fs1: &mut FileStats, fs2: &mut FileStats) {
    fs1.lines += fs2.lines;
    fs1.chars += fs2.chars;
    fs1.bytes += fs2.bytes;
    fs1.words += fs2.words;
    fs1.max_line_length = std::cmp::max(fs1.max_line_length, fs2.max_line_length);
}
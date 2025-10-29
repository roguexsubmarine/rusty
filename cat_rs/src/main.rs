use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]

struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long = "number", conflicts_with = "nonblank")]
    number_lines: bool,

    #[arg(short = 'E', long = "show-ends")]
    show_ends: bool,

    #[arg(short = 'b', long = "number-nonblank", conflicts_with = "number_lines")]
    nonblank: bool,

}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut line_no = 1;

    for filename in &args.files {

        if filename == "-" {
            print_stream(io::stdin().lock(), args.number_lines, args.show_ends, args.nonblank, &mut line_no)?;
        } else {
            let file = File::open(filename)?;
            print_stream(BufReader::new(file), args.number_lines, args.show_ends, args.nonblank, &mut line_no)?;
        }
    }

    Ok(())
}

fn print_stream<R: BufRead>(reader: R, number_lines: bool, show_ends: bool, nonblank: bool, line_no: &mut i32) -> io::Result<()> {

    for line_result in reader.lines() {
        let line = line_result?;
        
        if nonblank && line.trim().is_empty(){
            print!("${}", '\n');
            continue;
        }

        if number_lines || nonblank {
            print!("{:>6}\t", line_no);
        }
        

        print!("{}", line);
        *line_no += 1;
        

        if show_ends { print!("$") }
        print!("\n");
        
    }
    
    Ok(())
}
